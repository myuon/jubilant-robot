#[derive(Default, Clone, Copy)]
pub struct DragAndDropEvent {
    pub from: (f64, f64),
    pub to: (f64, f64),
    pub dragging: bool,
}

#[derive(Default, Clone, Copy)]
pub struct MouseUpEvent {
    pub at: (f64, f64),
}
