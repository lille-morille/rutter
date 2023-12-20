use macroquad::prelude::*;
use crate::lib::build_context::BuildContext;

/// Base trait for all widgets.
///
/// Widgets are the building blocks of the UI, and are responsible for
/// drawing themselves to the screen.
pub trait Widget {
  fn build(&self, ctx: &BuildContext);
}

/// Widget for rendering text on the scree
pub struct Text {
  value: String,
  font_size: Option<u16>,
  color: Option<Color>,
  rotation: f32,
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
    // draw_text_ex(
    //   &self.value.clone(),
    //   ctx.position.x,
    //   ctx.position.y,
    //   TextParams {
    //     font_size: self.font_size.unwrap_or(ctx.theme.font_size),
    //     color: self.color.unwrap_or(ctx.theme.text_color),
    //     rotation: self.rotation,
    //     ..Default::default()
    //   },
    // );
    draw_text(&self.value, ctx.position.x, ctx.position.y, self.font_size.unwrap_or(ctx.theme
        .font_size) as f32, self.color.unwrap_or(ctx.theme.text_color));
  }
}

impl Default for Text {
  fn default() -> Self {
    Text {
      value: "".to_string(),
      font_size: None,
      color: None,
      rotation: 0.0,
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
    let child_count = self.children.iter().count();

    for (idx, child) in self.children.iter().enumerate() {
      child.build(&ctx.child_context(Vec2 {
        x: max_x / child_count as f32,
        y: max_y,
      }, Vec2 {
        x: ctx.position.x + max_x / idx as f32,
        y: ctx.position.y,
      }))
    }
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
