use crate::lib::build_context::BuildContext;
use macroquad::prelude::*;

pub mod layout;

/// Base trait for all widgets.
///
/// Widgets are the building blocks of the UI, and are responsible for
/// drawing themselves to the screen.
pub trait Widget {
    fn build(&self, ctx: &BuildContext);
}

pub trait ConstrainedBox {
    fn get_min() -> Vec2 {
        vec2(f32::MIN, f32::MIN)
    }

    fn get_max() -> Vec2 {
        vec2(f32::MAX, f32::MAX)
    }
}

/// Widget for rendering text on the scree
pub struct Text {
    value: String,
    font_size: Option<u16>,
    color: Option<Color>,
    rotation: f32,
    hard_wrap: bool,
}

impl Widget for &str {
    fn build(&self, ctx: &BuildContext) {
        draw_text_ex(
            self,
            ctx.position.x,
            ctx.position.y,
            TextParams {
                font_size: ctx.theme.font_size,
                color: ctx.theme.text_color,
                rotation: 0.,
                ..Default::default()
            },
        );
    }
}

impl Widget for Text {
    fn build(&self, ctx: &BuildContext) {
        let font_size = self.font_size.unwrap_or(ctx.theme.font_size);
        let text_width = measure_text(&self.value, None, font_size, 1.0);

        assert!(
            !self.hard_wrap || text_width.width <= ctx.size.x,
            "Text overflowed by \
        {} pixels. Specify `hard_wrap = true` if you wish to wrap \
        the text.",
            text_width.width - ctx.size.x
        );
        assert!(!self.hard_wrap || text_width.height <= ctx.size.y);

        draw_text_ex(
            &self.value,
            ctx.position.x,
            ctx.position.y,
            TextParams {
                font_size,
                color: self.color.unwrap_or(ctx.theme.text_color),
                rotation: self.rotation,
                ..Default::default()
            },
        );
        draw_text(
            &self.value,
            ctx.position.x,
            ctx.position.y,
            font_size as f32,
            self.color.unwrap_or(ctx.theme.text_color),
        );
    }
}

impl Default for Text {
    fn default() -> Self {
        Text {
            value: "".to_string(),
            font_size: None,
            color: None,
            rotation: 0.0,
            hard_wrap: true,
        }
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Text {
            value: value.to_string(),
            ..Default::default()
        }
    }
}

/// Widget for rendering a list of children in a row
pub struct Row {
    pub children: Vec<Box<dyn Widget>>,
    pub main_axis_alignment: MainAxisAlign,
    pub main_axis_size: AxisSize,
    pub cross_axis_alignment: CrossAxisAlign,
}

impl Widget for Row {
    fn build(&self, ctx: &BuildContext) {
        let max_x = ctx.size.x;
        let max_y = ctx.size.y;
        let child_count = self.children.len();

        for (idx, child) in self.children.iter().enumerate() {
            child.build(&ctx.child_context(
                vec2(max_x / child_count as f32, max_y),
                vec2(ctx.position.x + max_x / idx as f32, ctx.position.y),
            ))
        }
    }
}

pub struct Container {
    pub child: Box<dyn Widget>,
    pub color: Color,
    pub width: f32,
    pub height: f32,
}

impl ConstrainedBox for Container {
    fn get_min() -> Vec2 {
        vec2(100., 100.)
    }
}

impl Widget for Container {
    fn build(&self, ctx: &BuildContext) {
        draw_rectangle(
            ctx.position.x,
            ctx.position.y,
            Self::get_min().x,
            Self::get_min().y,
            ctx.theme.container_color,
        )
    }
}

/// Describes how children should be placed along the main axis of a container
pub enum MainAxisAlign {
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

/// Describes if the container should expand to fill the available space,
/// or be sized to fit its children
pub enum AxisSize {
    Min,
    Max,
}

/// Describes how children should be placed along the cross axis of a container
pub enum CrossAxisAlign {
    Start,
    End,
    Center,
    Stretch,
}
