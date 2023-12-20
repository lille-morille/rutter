use std::default::Default;
use macroquad::prelude::*;
use crate::lib::build_context::{BuildContext, Theme};
use crate::lib::widget::*;

mod lib;

#[macroquad::main("Flutter.rs")]
async fn main() {
  loop {
    let ctx = BuildContext {
      size: Vec2 {
        x: screen_width(),
        y: screen_height(),
      },
      theme: Theme::default(),
      position: Vec2 {
        x: 50.,
        y: 50.,
      },
    };

    Row {
      children: vec![
        Box::new("Hello world!"),
        Box::new("Cool text!"),
        Box::new("Nice!"),
      ],
      main_axis_alignment: MainAxisAlign::SpaceEvenly,
      main_axis_size: AxisSize::Max,
      cross_axis_alignment: CrossAxisAlign::End,
    }.build(&ctx);

    next_frame().await
  }
}
