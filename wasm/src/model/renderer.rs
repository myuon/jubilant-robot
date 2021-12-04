use std::cell::{Ref, RefCell};

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

    pub fn find_selected(&self, x: f64, y: f64) -> Option<usize> {
        self.figures
            .borrow()
            .iter()
            .enumerate()
            .find_map(|(i, fig)| if fig.contains(x, y) { Some(i) } else { None })
    }

    pub fn borrow(&self, i: usize) -> Ref<'_, Vec<Box<dyn TFigure>>> {
        self.figures.borrow()
    }

    pub fn set_at(&self, i: usize, fig: impl TFigure + 'static) {
        self.figures.borrow_mut()[i] = Box::new(fig);
    }
}
