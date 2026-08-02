use ratatui::widgets::canvas::{Painter, Shape};

use crate::palette::Palette;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum SproutPercentage {
    FivePercent,
    #[default]
    TenPercent,
    FifteenPercent,
    TwentyPercent,
    TwentyFivePercent,
    ThirtyPercent,
    ThirtyFivePercent,
    FortyPercent,
    FortyFivePercent,
    FiftyPercent,
    FiftyFivePercent,
    SixtyPercent,
    SixtyFivePercent,
    SeventyPercent,
    SeventyFivePercent,
    EightyPercent,
    EightyFivePercent,
    NinetyPercent,
    NinetyFivePercent,
    HundredPercent,
}

pub type Points = Vec<(f64, f64)>;
pub struct SproutPoints {
    pub tree: Points,
    pub flower: Points,
}

impl SproutPercentage {
    /// Rounds any raw value (e.g. 6 → 5, 8 → 10) to the nearest 5% step, clamped to [5, 100].
    pub fn from_value(v: u8) -> Self {
        let rounded = ((v as f64 / 5.0).round() as u8 * 5).clamp(5, 100);
        match rounded {
            5 => Self::FivePercent,
            10 => Self::TenPercent,
            15 => Self::FifteenPercent,
            20 => Self::TwentyPercent,
            25 => Self::TwentyFivePercent,
            30 => Self::ThirtyPercent,
            35 => Self::ThirtyFivePercent,
            40 => Self::FortyPercent,
            45 => Self::FortyFivePercent,
            50 => Self::FiftyPercent,
            55 => Self::FiftyFivePercent,
            60 => Self::SixtyPercent,
            65 => Self::SixtyFivePercent,
            70 => Self::SeventyPercent,
            75 => Self::SeventyFivePercent,
            80 => Self::EightyPercent,
            85 => Self::EightyFivePercent,
            90 => Self::NinetyPercent,
            95 => Self::NinetyFivePercent,
            _ => Self::HundredPercent,
        }
    }

    pub fn value(self) -> u8 {
        match self {
            Self::FivePercent => 5,
            Self::TenPercent => 10,
            Self::FifteenPercent => 15,
            Self::TwentyPercent => 20,
            Self::TwentyFivePercent => 25,
            Self::ThirtyPercent => 30,
            Self::ThirtyFivePercent => 35,
            Self::FortyPercent => 40,
            Self::FortyFivePercent => 45,
            Self::FiftyPercent => 50,
            Self::FiftyFivePercent => 55,
            Self::SixtyPercent => 60,
            Self::SixtyFivePercent => 65,
            Self::SeventyPercent => 70,
            Self::SeventyFivePercent => 75,
            Self::EightyPercent => 80,
            Self::EightyFivePercent => 85,
            Self::NinetyPercent => 90,
            Self::NinetyFivePercent => 95,
            Self::HundredPercent => 100,
        }
    }

    pub fn data(self, sprout: &Sprout) -> SproutPoints {
        sprout.generate_points(self.value())
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
    // Six growth phases mapped across 0-100%:
    //   5-20%  : trunk grows       (y = -35 → -10)
    //   20-35% : lower branches spread
    //   35-50% : upper stem grows  (y = -10 → 5)
    //   50-65% : upper branches spread
    //   65-80% : top stem grows    (y = 5 → 20)
    //   80-100%: flower blooms
    fn generate_points(&self, pct: u8) -> SproutPoints {
        let pct = pct.clamp(5, 100);
        let mut tree: Points = Vec::new();
        let mut flower: Points = Vec::new();
        let p = pct as f64;

        let push_cluster = |pts: &mut Vec<(f64, f64)>, x: f64, y: f64| {
            pts.push((x, y));
            pts.push((x + 0.4, y));
            pts.push((x - 0.4, y));
            pts.push((x, y + 0.4));
            pts.push((x, y - 0.4));
        };

        // Phase 1: trunk (5-20%) — y = -35 growing up to -10
        {
            let frac = (p / 20.0).min(1.0);
            let top = -35.0 + 25.0 * frac;
            for y in -35i32..=-10 {
                if (y as f64) <= top {
                    tree.push((0.0, y as f64));
                }
            }
        }

        // Phase 2: lower branches (20-35%) — arcs sweep left and right from trunk at y=-20
        if p > 20.0 {
            let frac = ((p - 20.0) / 15.0).min(1.0);
            let steps = (90.0 * frac) as i32;
            for i in 0..=steps {
                let t = i as f64 / 90.0;
                let y = -20.0 + 7.5 * (1.0 - (2.0 * t - 1.0).powi(2));
                push_cluster(&mut tree, -20.0 * t, y);
                push_cluster(&mut tree, 20.0 * t, y);
            }
        }

        // Phase 3: upper stem (35-50%) — y = -10 growing up to 5
        if p > 35.0 {
            let frac = ((p - 35.0) / 15.0).min(1.0);
            let top = -10.0 + 15.0 * frac;
            for y in -10i32..=5 {
                if (y as f64) <= top {
                    tree.push((0.0, y as f64));
                }
            }
        }

        // Phase 4: upper branches (50-65%) — shorter arcs sweep left and right from y=5
        if p > 50.0 {
            let frac = ((p - 50.0) / 15.0).min(1.0);
            let steps = (90.0 * frac) as i32;
            for i in 0..=steps {
                let t = i as f64 / 90.0;
                let y = 5.0 + 4.5 * (1.0 - (2.0 * t - 1.0).powi(2));
                push_cluster(&mut tree, 13.0 * t, y);
                push_cluster(&mut tree, -13.0 * t, y);
            }
        }

        // Phase 5: top stem (65-80%) — y = 5 growing up to 20
        if p > 65.0 {
            let frac = ((p - 65.0) / 15.0).min(1.0);
            let top = 5.0 + 15.0 * frac;
            for y in 5i32..=20 {
                if (y as f64) <= top {
                    tree.push((0.0, y as f64));
                }
            }
        }

        // Phase 6: flower blooms (80-100%) — petals appear one cluster at a time
        if p > 80.0 {
            let frac = ((p - 80.0) / 20.0).min(1.0);
            let all: &[(f64, f64)] = &[
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
            ];
            let count = (all.len() as f64 * frac).ceil() as usize;
            flower.extend_from_slice(&all[..count]);
        }

        SproutPoints { tree, flower }
    }
}
