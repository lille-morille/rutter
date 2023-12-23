use lib::{
    build_context::{BuildContext, Theme},
    widget::{AxisSize, Container, CrossAxisAlign, MainAxisAlign, Row, Text, Widget},
};
use macroquad::prelude::*;
use std::default::Default;

mod lib;

#[macroquad::main("Flutter.rs")]
async fn main() {
    loop {
        let ctx = BuildContext {
            size: vec2(screen_width(), screen_height()),
            theme: Theme::default(),
            position: vec2(50., 50.),
        };

        Container {
            child: Box::new("Hello!"),
            color: Theme::default().container_color,
            width: 300.,
            height: 300.,
        }
        .build(&ctx);

        Row {
            children: children!["Hello world!", "Cool text!", "Nice!"],
            main_axis_alignment: MainAxisAlign::SpaceEvenly,
            main_axis_size: AxisSize::Max,
            cross_axis_alignment: CrossAxisAlign::End,
        }
        .build(&ctx);

        next_frame().await
    }
}
