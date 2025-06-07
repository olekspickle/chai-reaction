use super::*;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Opts {
    pub inner: WidgetContent,
    // layout
    pub border_radius: f32,
    pub border_color: Color,
    pub bg_color: Color,
    pub color: Color,
    pub node: Node,
}

#[allow(dead_code)]
impl Opts {
    pub fn new(c: impl Into<WidgetContent>) -> Self {
        // a bit of a hack IMO - it's weird that text node is not the width of the text by default
        // let min_width = Px(text.len() as f32 * FONT_SIZE / 1.2);
        Self {
            inner: c.into(),
            node: Node {
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                justify_items: JustifyItems::Center,
                justify_content: JustifyContent::Center,
                border: UiRect::all(Px(2.0)),
                padding: UiRect::horizontal(Vw(3.0)),
                ..Default::default()
            },
            color: WHITEISH,
            bg_color: TRANSPARENT,
            border_color: WHITEISH,
            border_radius: BORDER_RADIUS,
        }
    }

    pub fn sprite(mut self, s: Sprite) -> Self {
        self.inner = WidgetContent::Sprite(s);
        self
    }
    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        match self.inner {
            WidgetContent::Text(ref mut t) => {
                t.text = text.into();
            }
            _ => self.inner = WidgetContent::Text(text.into().into()),
        }
        self
    }
    pub fn font(mut self, font: TextFont) -> Self {
        if let WidgetContent::Text(ref mut t) = self.inner {
            t.font = font;
        }
        self
    }
    pub fn font_size(mut self, s: f32) -> Self {
        if let WidgetContent::Text(ref mut t) = self.inner {
            t.font.font_size = s;
        }
        self
    }
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }
    pub fn node(mut self, new: Node) -> Self {
        self.node = new;
        self
    }
    pub fn width(mut self, w: Val) -> Self {
        self.node.width = w;
        self
    }
    pub fn height(mut self, h: Val) -> Self {
        self.node.height = h;
        self
    }
    pub fn row_gap(mut self, g: Val) -> Self {
        self.node.row_gap = g;
        self
    }
    pub fn margin(mut self, m: UiRect) -> Self {
        self.node.margin = m;
        self
    }
    pub fn padding(mut self, p: UiRect) -> Self {
        self.node.padding = p;
        self
    }
    pub fn into_sprite_bundle(self) -> impl Bundle {
        match &self.inner {
            WidgetContent::Sprite(c) => SpriteWidgetBundle(c.clone()),
            _ => unreachable!("Spawning sprite bundle on non sprite content"),
        }
    }
    pub fn into_text_bundle(self) -> impl Bundle {
        match &self.inner {
            WidgetContent::Text(c) => TextWidgetBundle {
                font: c.font.clone(),
                layout: c.layout,
                text: Text(c.text.to_string()),
                color: TextColor(self.color),
                background_color: self.bg_color.into(),
            },
            _ => unreachable!("Spawning text bundle on non text content"),
        }
    }
}

#[derive(Bundle)]
pub struct SpriteWidgetBundle(Sprite);

#[derive(Bundle)]
pub struct TextWidgetBundle {
    pub background_color: BackgroundColor,
    pub text: Text,
    pub color: TextColor,
    pub font: TextFont,
    pub layout: TextLayout,
}

#[derive(Debug, Clone)]
pub struct TextContent {
    pub layout: TextLayout,
    pub text: Cow<'static, str>,
    pub font: TextFont,
}

impl From<Cow<'static, str>> for TextContent {
    fn from(text: Cow<'static, str>) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}
impl Default for TextContent {
    fn default() -> Self {
        Self {
            text: "".into(),
            layout: TextLayout::new_with_justify(JustifyText::Center),
            font: TextFont::from_font_size(FONT_SIZE),
        }
    }
}

#[derive(Debug, Clone, Component)]
pub enum WidgetContent {
    Sprite(Sprite),
    Text(TextContent),
}

// To be able to provide just "my-label" or Sprite{..} as an argument for UI widgets
impl<T: Into<WidgetContent>> From<T> for Opts {
    fn from(value: T) -> Self {
        Opts::new(value.into())
    }
}

impl From<Sprite> for WidgetContent {
    fn from(value: Sprite) -> Self {
        Self::Sprite(value)
    }
}
impl From<&'static str> for WidgetContent {
    fn from(value: &'static str) -> Self {
        Self::Text(TextContent {
            text: value.into(),
            ..Default::default()
        })
    }
}
impl From<Cow<'static, str>> for WidgetContent {
    fn from(text: Cow<'static, str>) -> Self {
        Self::Text(TextContent {
            text,
            ..Default::default()
        })
    }
}
impl From<String> for WidgetContent {
    fn from(value: String) -> Self {
        Self::Text(TextContent {
            text: value.into(),
            ..Default::default()
        })
    }
}
