use crate::app::App;
use crate::palette::Palette;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line as TextLine, Span};

pub fn render(_app: &mut App, frame: &mut Frame) {
    let vertical: Layout =
        Layout::vertical([Constraint::Length(5), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Percentage(100)]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [_area] = main.layout(&horizontal);

    draw_app_name(frame, top);
}

pub fn draw_app_name(frame: &mut Frame, area: Rect) {
    let title: TextLine<'_> = TextLine::from_iter([
        Span::from("sprout").style(Style::new().fg(Palette::BRAND_GREEN).bold()),
        Span::from(" - habit tracker").style(Style::new().fg(Palette::TEXT_SECONDARY)),
    ]);
    frame.render_widget(title.left_aligned(), area);
}
