use ratatui::widgets::canvas::{Painter, Shape};

use crate::palette::Palette;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SproutPercentage {
    #[default]
    TenPercent,
    TwentyPercent,
    ThirtyPercent,
    FortyPercent,
    FiftyPercent,
    SixtyPercent,
    SeventyPercent,
    EightyPercent,
    NinetyPercent,
    HundredPercent,
}

pub type Points = Vec<(f64, f64)>;
pub struct SproutPoints {
    pub tree: Points,
    pub flower: Points,
}

impl SproutPercentage {
    pub fn data(self, sprout: &Sprout) -> SproutPoints {
        match self {
            Self::TenPercent => sprout.generate_percent_ten_points(),
            _ => sprout.generate_percent_ten_points(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Sprout {
    pub percentage: SproutPercentage,
}

impl Shape for Sprout {
    fn draw(&self, painter: &mut Painter) {
        let points = self.percentage.data(self);
        for &(x, y) in &points.tree {
            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, Palette::BRAND_GREEN);
            }
        }
        for &(x, y) in &points.flower {
            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, Palette::AMBER);
            }
        }
    }
}

impl Sprout {
    fn generate_percent_ten_points(&self) -> SproutPoints {
        let mut tree_points: Vec<(f64, f64)> = Vec::new();
        let mut flower_points: Vec<(f64, f64)> = Vec::new();

        //to the bracnhes a bit thicker, we can add a cluster of points around each branch point
        let push_cluster = |pts: &mut Vec<(f64, f64)>, x: f64, y: f64| {
            pts.push((x, y));
            pts.push((x + 0.4, y));
            pts.push((x - 0.4, y));
            pts.push((x, y + 0.4));
            pts.push((x, y - 0.4));
        };

        // trunk
        for y in -35..=-10 {
            tree_points.push((0.0, y as f64));
        }

        // lower left branch
        for i in 0..=90 {
            let t = i as f64 / 90.0;
            let x = -20.0 * t;
            let y = -20.0 + 7.5 * (1.0 - (2.0 * t - 1.0).powi(2));
            push_cluster(&mut tree_points, x, y);
        }

        // lower right branch
        for i in 0..=90 {
            let t = i as f64 / 90.0;
            let x = 20.0 * t;
            let y = -20.0 + 7.5 * (1.0 - (2.0 * t - 1.0).powi(2));
            push_cluster(&mut tree_points, x, y);
        }

        // upper stem
        for y in -10..=5 {
            tree_points.push((0.0, y as f64));
        }

        // upper right branch
        for i in 0..=90 {
            let t = i as f64 / 90.0;
            let x = 13.0 * t;
            let y = 5.0 + 4.5 * (1.0 - (2.0 * t - 1.0).powi(2));
            push_cluster(&mut tree_points, x, y);
        }

        // upper left branch
        for i in 0..=90 {
            let t = i as f64 / 90.0;
            let x = -13.0 * t;
            let y = 5.0 + 4.5 * (1.0 - (2.0 * t - 1.0).powi(2));
            push_cluster(&mut tree_points, x, y);
        }

        // top stem
        for y in 5..=20 {
            tree_points.push((0.0, y as f64));
        }

        // flower at the top
        flower_points.extend_from_slice(&[
            (-2.0, 24.0),
            (-1.2, 25.0),
            (0.0, 25.6),
            (1.2, 25.0),
            (2.0, 24.0),
            (-2.5, 23.0),
            (-1.0, 22.6),
            (1.0, 22.6),
            (2.5, 23.0),
            (-1.5, 26.0),
            (0.0, 26.4),
            (1.5, 26.0),
        ]);
        SproutPoints {
            tree: tree_points,
            flower: flower_points,
        }
    }
}
