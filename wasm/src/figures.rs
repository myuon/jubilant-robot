pub trait TDrawingContext {
    fn begin_path(&self);
    fn stroke(&self);
    fn move_to(&self, x: f64, y: f64);
    fn line_to(&self, x: f64, y: f64);
    fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64);
}

pub trait TCanvas {
    fn clear(&self);
}

#[derive(Clone)]
pub struct Rectangle {
    pub from: (f64, f64),
    pub to: (f64, f64),
}

impl Rectangle {
    pub fn new(from: (f64, f64), to: (f64, f64)) -> Rectangle {
        Rectangle { from, to }
    }

    pub fn contains(&self, x: f64, y: f64) -> bool {
        self.from.0 <= x && x <= self.to.0 && self.from.1 <= y && y <= self.to.1
    }

    pub fn draw(&self, ctx: &impl TDrawingContext) {
        ctx.begin_path();
        ctx.move_to(self.from.0, self.from.1);
        ctx.line_to(self.to.0, self.from.1);
        ctx.line_to(self.to.0, self.to.1);
        ctx.line_to(self.from.0, self.to.1);
        ctx.line_to(self.from.0, self.from.1);
        ctx.stroke();
    }

    pub fn clear(&self, ctx: &impl TDrawingContext) {
        ctx.clear_rect(
            self.from.0,
            self.from.1,
            self.to.0 - self.from.0,
            self.to.1 - self.from.1,
        );
    }
}
