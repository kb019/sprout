use ratatui::style::Color;

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
        fg_dim: Color::Rgb(154, 132, 80),
        fg_muted: Color::Rgb(90, 73, 40),
        border: Color::Rgb(68, 53, 24),
        border_dim: Color::Rgb(42, 32, 13),
        selection: Color::Rgb(58, 47, 18),
        background: Color::Rgb(25, 26, 27),
        row_highlight: Color::Rgb(46, 40, 26),
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
        dot: Color::Rgb(30, 24, 10),
    };

    pub const MONO: Self = Self {
        accent: Color::Rgb(207, 233, 212),
        fg: Color::Rgb(207, 233, 212),
        fg_dim: Color::Rgb(120, 128, 120),
        fg_muted: Color::Rgb(68, 72, 68),
        border: Color::Rgb(44, 50, 44),
        border_dim: Color::Rgb(28, 31, 28),
        selection: Color::Rgb(40, 46, 40),
        background: Color::Rgb(25, 26, 27),
        row_highlight: Color::Rgb(40, 44, 40),
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
        dot: Color::Rgb(24, 26, 24),
    };

    pub const OCEAN: Self = Self {
        accent: Color::Rgb(65, 195, 235),
        fg: Color::Rgb(207, 233, 212),
        fg_dim: Color::Rgb(75, 145, 175),
        fg_muted: Color::Rgb(38, 80, 105),
        border: Color::Rgb(22, 58, 80),
        border_dim: Color::Rgb(13, 35, 50),
        selection: Color::Rgb(18, 55, 80),
        background: Color::Rgb(25, 26, 27),
        row_highlight: Color::Rgb(24, 42, 54),
        heatmap: [
            Color::Rgb(13, 35, 50),
            Color::Rgb(15, 72, 105),
            Color::Rgb(22, 115, 158),
            Color::Rgb(40, 163, 210),
            Color::Rgb(65, 195, 235),
        ],
        amber: Color::Rgb(255, 190, 60),
        danger: Color::Rgb(194, 84, 84),
        danger_bg: Color::Rgb(42, 20, 20),
        soil: Color::Rgb(107, 86, 54),
        dot: Color::Rgb(10, 24, 34),
    };

    pub const PAPER: Self = Self {
        accent: Color::Rgb(40, 155, 85),
        fg: Color::Rgb(28, 33, 30),
        fg_dim: Color::Rgb(88, 108, 95),
        fg_muted: Color::Rgb(155, 170, 160),
        border: Color::Rgb(175, 200, 185),
        border_dim: Color::Rgb(212, 226, 218),
        selection: Color::Rgb(190, 228, 205),
        background: Color::Rgb(248, 250, 248),
        row_highlight: Color::Rgb(237, 244, 240),
        heatmap: [
            Color::Rgb(212, 226, 218),
            Color::Rgb(160, 210, 178),
            Color::Rgb(95, 178, 130),
            Color::Rgb(52, 148, 92),
            Color::Rgb(40, 155, 85),
        ],
        amber: Color::Rgb(185, 120, 0),
        danger: Color::Rgb(175, 45, 45),
        danger_bg: Color::Rgb(255, 228, 228),
        soil: Color::Rgb(130, 100, 60),
        dot: Color::Rgb(226, 237, 230),
    };

    pub fn from_index(index: usize) -> Self {
        match index {
            1 => Self::AMBER,
            2 => Self::MONO,
            3 => Self::OCEAN,
            4 => Self::PAPER,
            _ => Self::SPROUT,
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::SPROUT
    }
}
