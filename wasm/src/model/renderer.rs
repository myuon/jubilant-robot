use std::cell::RefCell;

use crate::utils::{
    console_log,
    event::{DragAndDropEvent, MouseUpEvent},
};

use super::figures::{Figure, Rectangle, TDrawingContext};

pub struct Renderer {
    figures: RefCell<Vec<Figure>>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            figures: RefCell::new(vec![]),
        }
    }

    pub fn register(&self, figure: Figure) {
        let mut figs = self.figures.take();
        figs.push(figure);
        self.figures.replace(figs);
    }

    pub fn render(&self, ctx: &impl TDrawingContext) {
        let figs = self.figures.borrow();
        for fig in figs.iter() {
            fig.render(ctx);
        }
    }

    pub fn on_mouse_dnd(&self, event: DragAndDropEvent, context: &impl TDrawingContext) {
        let rect = Rectangle::new(event.from, event.to);
        rect.draw(context);

        self.register(Figure::Rectangle(rect));
    }

    pub fn on_mouse_up(&self, event: MouseUpEvent) {
        for fig in &*self.figures.borrow() {
            if fig.contains(event.at.0, event.at.1) {
                console_log!("{:?}", fig);
            }
        }
    }
}
