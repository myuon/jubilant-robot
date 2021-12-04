use crate::model::figures::{Rectangle, TFigure};

#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,
    pub rect: Rectangle,
}

impl Button {
    pub fn new(text: String, mut rect: Rectangle) -> Self {
        rect.style_options.fill = Some(true);

        Button { text, rect }
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
}
