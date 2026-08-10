use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::symbols;
use ratatui::widgets::Widget;

pub use self::bar::Bar;
pub use self::bar_group::BarGroup;

mod bar;
mod bar_group;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BarChart<'a> {
    bar_width: u16,
    bar_gap: u16,
    value_style: Style,
    label_style: Style,
    data: Vec<BarGroup<'a>>,
}

impl Default for BarChart<'_> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            bar_width: 1,
            bar_gap: 1,
            value_style: Style::default(),
            label_style: Style::default(),
        }
    }
}

impl<'a> BarChart<'a> {
    pub fn new<T: Into<Vec<Bar<'a>>>>(bars: T) -> Self {
        Self {
            data: Self::non_empty_groups(vec![BarGroup::new(bars.into())]),
            ..Default::default()
        }
    }

    pub fn vertical(bars: impl Into<Vec<Bar<'a>>>) -> Self {
        Self::new(bars)
    }

    fn non_empty_groups(groups: Vec<BarGroup<'a>>) -> Vec<BarGroup<'a>> {
        groups
            .into_iter()
            .filter(|group| !group.bars.is_empty())
            .collect()
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn bar_width(mut self, width: u16) -> Self {
        self.bar_width = width;
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub const fn bar_gap(mut self, gap: u16) -> Self {
        self.bar_gap = gap;
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn value_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.value_style = style.into();
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn label_style<S: Into<Style>>(mut self, style: S) -> Self {
        self.label_style = style.into();
        self
    }
}

#[derive(Clone, Copy)]
struct LabelInfo {
    group_label_visible: bool,
    bar_label_visible: bool,
    height: u16,
}

impl BarChart<'_> {
    fn group_ticks(&self, available_space: u16, bar_max_length: u16) -> Vec<Vec<u64>> {
        let max: u64 = self.maximum_data_value();
        self.data
            .iter()
            .scan(available_space, |space, group| {
                if *space == 0 {
                    return None;
                }
                let n_bars = group.bars.len() as u16;
                let group_width = n_bars * self.bar_width + n_bars.saturating_sub(1) * self.bar_gap;

                let n_bars = if *space > group_width {
                    *space = space.saturating_sub(group_width + self.bar_gap);
                    Some(n_bars)
                } else {
                    let max_bars = (*space + self.bar_gap) / (self.bar_width + self.bar_gap);
                    if max_bars > 0 {
                        *space = 0;
                        Some(max_bars)
                    } else {
                        None
                    }
                };

                n_bars.map(|n| {
                    group
                        .bars
                        .iter()
                        .take(n as usize)
                        .map(|bar| Self::scale_ticks(bar.value, max, bar_max_length))
                        .collect()
                })
            })
            .collect()
    }

    fn scale_ticks(value: u64, max: u64, max_length: u16) -> u64 {
        let max_ticks = u128::from(max_length) * 8;
        let ticks = u128::from(value) * max_ticks / u128::from(max);
        ticks.min(max_ticks) as u64
    }

    fn label_info(&self, available_height: u16) -> LabelInfo {
        if available_height == 0 {
            return LabelInfo {
                group_label_visible: false,
                bar_label_visible: false,
                height: 0,
            };
        }

        let bar_label_visible = self
            .data
            .iter()
            .any(|e| e.bars.iter().any(|e| e.label.is_some()));

        if available_height == 1 && bar_label_visible {
            return LabelInfo {
                group_label_visible: false,
                bar_label_visible: true,
                height: 1,
            };
        }

        let group_label_visible = self.data.iter().any(|e| e.label.is_some());
        LabelInfo {
            group_label_visible,
            bar_label_visible,
            height: u16::from(group_label_visible) + u16::from(bar_label_visible),
        }
    }

    fn render_vertical(&self, buf: &mut Buffer, area: Rect) {
        let label_info = self.label_info(area.height.saturating_sub(1));
        let bars_area = Rect {
            height: area.height.saturating_sub(label_info.height),
            ..area
        };
        let group_ticks = self.group_ticks(bars_area.width, bars_area.height.saturating_sub(3));
        self.render_vertical_bars(bars_area, buf, &group_ticks);
        self.render_labels_and_values(area, buf, label_info, &group_ticks);
    }

    fn render_vertical_bars(&self, area: Rect, buf: &mut Buffer, group_ticks: &[Vec<u64>]) {
        let bar_set = symbols::bar::NINE_LEVELS;
        let mut bar_x = area.left();
        for (ticks_vec, group) in group_ticks.iter().zip(&self.data) {
            for (ticks, bar) in ticks_vec.iter().zip(&group.bars) {
                let mut ticks = *ticks;
                for j in (0..area.height).rev() {
                    let symbol = match ticks {
                        0 => bar_set.empty,
                        1 => bar_set.one_eighth,
                        2 => bar_set.one_quarter,
                        3 => bar_set.three_eighths,
                        4 => bar_set.half,
                        5 => bar_set.five_eighths,
                        6 => bar_set.three_quarters,
                        7 => bar_set.seven_eighths,
                        _ => bar_set.full,
                    };
                    for x in 0..self.bar_width {
                        buf[(bar_x + x, area.top() + j)]
                            .set_symbol(symbol)
                            .set_style(bar.style);
                    }
                    ticks = ticks.saturating_sub(8);
                }
                bar_x += self.bar_gap + self.bar_width;
            }
        }
    }

    fn maximum_data_value(&self) -> u64 {
        self.data
            .iter()
            .map(|group| group.max().unwrap_or_default())
            .max()
            .unwrap_or_default()
            .max(1)
    }

    fn render_labels_and_values(
        &self,
        area: Rect,
        buf: &mut Buffer,
        label_info: LabelInfo,
        group_ticks: &[Vec<u64>],
    ) {
        let mut bar_x = area.left();
        let bar_y = area.bottom() - label_info.height - 1;
        for (group, ticks_vec) in self.data.iter().zip(group_ticks) {
            if group.bars.is_empty() {
                continue;
            }
            if label_info.group_label_visible {
                let label_max_width =
                    ticks_vec.len() as u16 * (self.bar_width + self.bar_gap) - self.bar_gap;
                let group_area = Rect {
                    x: bar_x,
                    y: area.bottom() - 1,
                    width: label_max_width,
                    height: 1,
                };
                group.render_label(buf, group_area, self.label_style);
            }
            for (bar, ticks) in group.bars.iter().zip(ticks_vec) {
                if label_info.bar_label_visible {
                    bar.render_label(buf, self.bar_width, bar_x, bar_y + 1, self.label_style);
                }
                bar.render_value(buf, self.bar_width, bar_x, bar_y, self.value_style, *ticks);
                bar_x += self.bar_gap + self.bar_width;
            }
        }
    }
}

impl Widget for BarChart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Widget::render(&self, area, buf);
    }
}

impl Widget for &BarChart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.is_empty() || self.data.is_empty() || self.bar_width == 0 {
            return;
        }
        self.render_vertical(buf, area);
    }
}

//TODO : Add tests for the top value rendering behaviour, i.e the value should appear at top instead of bottom
#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::*;

    #[test]
    fn default() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 3));
        let widget = BarChart::default();
        widget.render(buffer.area, &mut buffer);
        assert_eq!(buffer, Buffer::with_lines(["          "; 3]));
    }

    #[test]
    fn constructors_ignore_empty_groups() {
        assert!(BarChart::new(Vec::<Bar>::new()).data.is_empty());
    }

    #[test]
    fn handles_zero_width() {
        let chart = BarChart::new([Bar::with_label("A", 1)])
            .bar_width(0)
            .bar_gap(0);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 0, 10));
        chart.render(buffer.area, &mut buffer);
        assert_eq!(buffer, Buffer::empty(Rect::new(0, 0, 0, 10)));
    }

    #[test]
    fn test_barchart_new() {
        let bars = [Bar::with_label("Red", 1), Bar::with_label("Green", 2)];
        let chart = BarChart::new(bars.clone());
        assert_eq!(chart.data.len(), 1);
        assert_eq!(chart.data[0].bars, bars);
    }

    #[test]
    fn bar_width_builder() {
        let chart = BarChart::default().bar_width(5);
        assert_eq!(chart.bar_width, 5);
    }

    #[test]
    fn bar_gap_builder() {
        let chart = BarChart::default().bar_gap(3);
        assert_eq!(chart.bar_gap, 3);
    }
}
