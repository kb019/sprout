use ratatui::style::Color;

pub trait ComponentTheme: Sized {
    fn from_palette(palette: &Palette) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub accent: Color,
    pub fg: Color,
    pub fg_dim: Color,
    pub fg_muted: Color,
    pub border: Color,
    pub border_dim: Color,
    pub selection: Color,
    pub background: Color,
    pub row_highlight: Color,
    pub heatmap: [Color; 5],
    /// Fixed fire/streak indicator, stays amber regardless of accent theme.
    pub amber: Color,
    pub danger: Color,
    pub danger_bg: Color,
    pub soil: Color,
    pub dot: Color,
}

impl Palette {
    pub const SPROUT: Self = Self {
        accent: Color::Rgb(75, 227, 116),
        fg: Color::Rgb(207, 233, 212),
        fg_dim: Color::Rgb(111, 139, 120),
        fg_muted: Color::Rgb(63, 85, 72),
        border: Color::Rgb(36, 53, 42),
        border_dim: Color::Rgb(23, 35, 27),
        selection: Color::Rgb(22, 53, 33),
        background: Color::Rgb(25, 26, 27),
        row_highlight: Color::Rgb(38, 46, 41),
        heatmap: [
            Color::Rgb(23, 35, 27),
            Color::Rgb(14, 68, 41),
            Color::Rgb(28, 107, 61),
            Color::Rgb(47, 161, 85),
            Color::Rgb(75, 227, 116),
        ],
        amber: Color::Rgb(255, 190, 60),
        danger: Color::Rgb(194, 84, 84),
        danger_bg: Color::Rgb(42, 20, 20),
        soil: Color::Rgb(107, 86, 54),
        dot: Color::Rgb(21, 32, 25),
    };

    pub const AMBER: Self = Self {
        accent: Color::Rgb(227, 179, 65),
        fg: Color::Rgb(207, 233, 212),
        fg_dim: Color::Rgb(111, 139, 120),
        fg_muted: Color::Rgb(63, 85, 72),
        border: Color::Rgb(36, 53, 42),
        border_dim: Color::Rgb(23, 35, 27),
        selection: Color::Rgb(58, 47, 18),
        background: Color::Rgb(25, 26, 27),
        row_highlight: Color::Rgb(38, 46, 41),
        heatmap: [
            Color::Rgb(36, 29, 13),
            Color::Rgb(92, 66, 19),
            Color::Rgb(143, 111, 30),
            Color::Rgb(201, 154, 46),
            Color::Rgb(227, 179, 65),
        ],
        amber: Color::Rgb(255, 190, 60),
        danger: Color::Rgb(194, 84, 84),
        danger_bg: Color::Rgb(42, 20, 20),
        soil: Color::Rgb(107, 86, 54),
        dot: Color::Rgb(21, 32, 25),
    };

    pub const MONO: Self = Self {
        accent: Color::Rgb(207, 233, 212),
        fg: Color::Rgb(207, 233, 212),
        fg_dim: Color::Rgb(111, 139, 120),
        fg_muted: Color::Rgb(63, 85, 72),
        border: Color::Rgb(36, 53, 42),
        border_dim: Color::Rgb(23, 35, 27),
        selection: Color::Rgb(38, 48, 42),
        background: Color::Rgb(25, 26, 27),
        row_highlight: Color::Rgb(38, 46, 41),
        heatmap: [
            Color::Rgb(27, 31, 28),
            Color::Rgb(51, 59, 54),
            Color::Rgb(95, 107, 98),
            Color::Rgb(154, 168, 156),
            Color::Rgb(207, 233, 212),
        ],
        amber: Color::Rgb(255, 190, 60),
        danger: Color::Rgb(194, 84, 84),
        danger_bg: Color::Rgb(42, 20, 20),
        soil: Color::Rgb(107, 86, 54),
        dot: Color::Rgb(21, 32, 25),
    };

    pub fn from_index(index: usize) -> Self {
        match index {
            1 => Self::AMBER,
            2 => Self::MONO,
            _ => Self::SPROUT,
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::SPROUT
    }
}
