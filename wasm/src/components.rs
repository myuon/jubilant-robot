use std::cell::Cell;

use crate::model::figures::{Rectangle, TFigure};
use derivative::*;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Button {
    pub text: String,
    pub rect: Rectangle,
    #[derivative(Debug = "ignore")]
    pub on_click: Box<dyn Fn()>,
}

impl Button {
    pub fn new(text: String, mut rect: Rectangle, on_click: Box<dyn Fn()>) -> Self {
        rect.style_options.fill = Some(true);
        rect.style_options.fill_color = Some("#333".to_string());

        Button {
            text,
            rect,
            on_click,
        }
    }
}

impl TFigure for Button {
    fn contains(&self, x: f64, y: f64) -> bool {
        self.rect.contains(x, y)
    }

    fn render(&self, ctx: &dyn crate::model::figures::TDrawingContext) {
        self.rect.render(ctx);

        let (x, y) = self.rect.from;
        let (_w, h) = self.rect.size();

        ctx.text(&self.text, x, y + h * 0.5, (h * 0.5) as i32);
    }

    fn click(&self) {
        (self.on_click)();
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ToggleStateButton {
    pub rect: Rectangle,
    labels: Vec<String>,
    index: Cell<usize>,
    #[derivative(Debug = "ignore")]
    on_click: Box<dyn Fn(String)>,
}

impl ToggleStateButton {
    pub fn new(mut rect: Rectangle, labels: Vec<String>, on_click: Box<dyn Fn(String)>) -> Self {
        assert!(labels.len() > 0);

        rect.style_options.fill = Some(true);
        rect.style_options.fill_color = Some("#333".to_string());

        ToggleStateButton {
            rect,
            labels,
            index: Cell::new(0),
            on_click,
        }
    }
}

impl TFigure for ToggleStateButton {
    fn contains(&self, x: f64, y: f64) -> bool {
        self.rect.contains(x, y)
    }

    fn render(&self, ctx: &dyn crate::model::figures::TDrawingContext) {
        self.rect.render(ctx);

        let (x, y) = self.rect.from;
        let (_w, h) = self.rect.size();

        ctx.text(
            &self.labels[self.index.get()],
            x,
            y + h * 0.5,
            (h * 0.5) as i32,
        );
    }

    fn click(&self) {
        self.index.set((self.index.get() + 1) % self.labels.len());
        (self.on_click)(self.labels[self.index.get()].clone());
    }
}
