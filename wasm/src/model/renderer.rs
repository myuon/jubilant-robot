use std::cell::RefCell;

use crate::utils::event::MouseUpEvent;

use super::figures::{TDrawingContext, TFigure};

pub struct Renderer {
    figures: RefCell<Vec<Box<dyn TFigure>>>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            figures: RefCell::new(vec![]),
        }
    }

    pub fn register(&self, figure: impl TFigure + 'static) {
        let mut figs = self.figures.take();
        figs.push(Box::new(figure));
        self.figures.replace(figs);
    }

    pub fn unregister_all(&self) {
        self.figures.take();
    }

    pub fn render(&self, ctx: &impl TDrawingContext) {
        let figs = self.figures.borrow();
        for fig in figs.iter() {
            fig.render(ctx);
        }
    }

    pub fn handle_mouse_up(&self, event: MouseUpEvent) {
        for fig in &*self.figures.borrow() {
            if fig.contains(event.at.0, event.at.1) {
                fig.click();
            }
        }
    }
}
