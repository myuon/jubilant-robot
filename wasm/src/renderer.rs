use std::cell::Cell;

use crate::{
    event::DragAndDropEvent,
    figures::{Figure, Rectangle, TDrawingContext},
};

pub struct Renderer {
    figures: Cell<Vec<Figure>>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            figures: Cell::new(vec![]),
        }
    }

    pub fn register(&self, figure: Figure) {
        let mut figs = self.figures.take();
        figs.push(figure);
        self.figures.replace(figs);
    }

    pub fn on_mouse_dnd(&self, event: DragAndDropEvent, context: &impl TDrawingContext) {
        let rect = Rectangle::new(event.from, event.to);
        rect.draw(context);

        self.register(Figure::Rectangle(rect));
    }
}
