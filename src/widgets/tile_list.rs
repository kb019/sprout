use ratatui::style::{Style, Styled};
use ratatui::text::Text;
use ratatui::widgets::Block;

mod rendering;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TileDirection {
    #[default]
    LeftToRight,
    RightToLeft,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TileType {
    #[default]
    Unbordered,
    Bordered,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TileBorderType {
    #[default]
    Rounded,
    Sharp,
}

#[derive(Debug, Clone)]
pub struct TileItem<'a> {
    pub(crate) content: Text<'a>,
    pub(crate) style: Style,
}

impl<'a> TileItem<'a> {
    pub fn new(content: impl Into<Text<'a>>) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.style = style.into();
        self
    }

    pub fn width(&self) -> usize {
        self.content.width()
    }

    pub fn height(&self) -> usize {
        self.content.height()
    }
}

impl<'a, T: Into<Text<'a>>> From<T> for TileItem<'a> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl Styled for TileItem<'_> {
    type Item = Self;
    fn style(&self) -> Style {
        self.style
    }
    fn set_style<S: Into<Style>>(self, style: S) -> Self {
        self.style(style)
    }
}

#[derive(Debug, Clone, Default)]
pub struct TileList<'a> {
    pub(crate) items: Vec<TileItem<'a>>,
    pub(crate) style: Style,
    pub(crate) highlight_style: Style,
    pub(crate) block: Option<Block<'a>>,
    pub(crate) tile_type: TileType,
    pub(crate) tile_border_type: TileBorderType,
    pub(crate) direction: TileDirection,
}

impl<'a> TileList<'a> {
    pub fn new<T>(items: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<TileItem<'a>>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            ..Self::default()
        }
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn style(mut self, style: impl Into<Style>) -> Self {
        self.style = style.into();
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn highlight_style(mut self, style: impl Into<Style>) -> Self {
        self.highlight_style = style.into();
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn tile_type(mut self, tile_type: TileType) -> Self {
        self.tile_type = tile_type;
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn tile_border_type(mut self, tile_border_type: TileBorderType) -> Self {
        self.tile_border_type = tile_border_type;
        self
    }

    #[must_use = "method moves the value of self and returns the modified value"]
    pub fn direction(mut self, direction: TileDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Styled for TileList<'_> {
    type Item = Self;
    fn style(&self) -> Style {
        self.style
    }
    fn set_style<S: Into<Style>>(self, style: S) -> Self {
        self.style(style)
    }
}

impl<'a, Item: Into<TileItem<'a>>> FromIterator<Item> for TileList<'a> {
    fn from_iter<I: IntoIterator<Item = Item>>(iter: I) -> Self {
        Self::new(iter)
    }
}
