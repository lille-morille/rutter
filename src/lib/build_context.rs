use macroquad::prelude::*;

/// A context that is passed to the `build` method each widget.
/// Contains contextual information about the application's state,
/// and the widget's position and size.
#[derive(Clone, Debug)]
pub struct BuildContext {
    pub size: Vec2,
    pub position: Vec2,
    pub theme: Theme,
}

impl BuildContext {
    pub fn child_context(&self, size: Vec2, position: Vec2) -> BuildContext {
        assert!(
            size.x <= self.size.x,
            "Child width must be less than parent width"
        );
        assert!(
            size.y <= self.size.y,
            "Child height must be less than parent height"
        );

        BuildContext {
            theme: self.theme.clone(),
            position,
            size,
            ..self.clone()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub background_color: Color,
    pub container_color: Color,
    pub text_color: Color,
    pub font_size: u16,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            background_color: color_u8!(50, 50, 50, 255),
            container_color: color_u8!(100, 100, 100, 255),
            text_color: color_u8!(255, 255, 255, 255),
            font_size: 24,
        }
    }
}
