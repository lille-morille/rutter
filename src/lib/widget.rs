use glam::Vec2;


/// A context that is passed to the `build` method each widget.
/// Contains contextual information about the application's state,
/// and the widget's position and size.
pub struct BuildContext {
  size: Vec2,
  position: Vec2,
}


/// Base trait for all widgets.
///
/// Widgets are the building blocks of the UI, and are responsible for
/// drawing themselves to the screen.
pub trait Widget {
 fn build(&self, ctx: &BuildContext);
}
