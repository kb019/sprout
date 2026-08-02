use ratatui::style::Color;

pub struct Palette;

impl Palette {
    pub const BACKGROUND: Color = Color::Rgb(10, 15, 12);
    pub const BORDER: Color = Color::Rgb(36, 53, 42);
    pub const BORDER_DIM: Color = Color::Rgb(23, 35, 27);
    pub const TEXT_PRIMARY: Color = Color::Rgb(207, 233, 212);
    pub const TEXT_SECONDARY: Color = Color::Rgb(111, 139, 120);
    pub const TEXT_MUTED: Color = Color::Rgb(63, 85, 72);
    pub const HEATMAP_0: Color = Color::Rgb(23, 35, 27);
    pub const HEATMAP_1: Color = Color::Rgb(14, 68, 41);
    pub const HEATMAP_2: Color = Color::Rgb(28, 107, 61);
    pub const HEATMAP_3: Color = Color::Rgb(47, 161, 85);
    pub const HEATMAP_4: Color = Color::Rgb(75, 227, 116);
    pub const BRAND_GREEN: Color = Color::Rgb(75, 227, 116);
    pub const AMBER: Color = Color::Rgb(255, 190, 60);
    pub const SELECTION: Color = Color::Rgb(22, 53, 33);
    pub const DANGER: Color = Color::Rgb(194, 84, 84);
    pub const SOIL: Color = Color::Rgb(107, 86, 54);
    pub const DOT: Color = Color::Black;

    pub fn heatmap(level: u8) -> Color {
        match level {
            0 => Self::HEATMAP_0,
            1 => Self::HEATMAP_1,
            2 => Self::HEATMAP_2,
            3 => Self::HEATMAP_3,
            _ => Self::HEATMAP_4,
        }
    }
}
