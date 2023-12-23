use crate::lib::widget::Widget;
use uuid::Uuid;

struct Element {
    key: Uuid,
    widget: dyn Widget,
}

impl Element {
    fn new(widget: &dyn Widget) -> Self {
        Element {
            key: Uuid::new_v4(),
            widget,
        }
    }
}
