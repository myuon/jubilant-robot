// figuresの列からcontextの命令列への変換は最適化入れられそう
pub trait TDrawingContext {
    fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64);
    fn rectangle(&self, x: f64, y: f64, w: f64, h: f64);
    fn set_stroke_dashed(&self, patterns: Vec<i32>);
    fn reset_stroke(&self);
}

pub trait TCanvas {
    fn clear(&self);
    fn register(&self, figure: Figure);
}

#[derive(Debug, Clone)]
pub enum Figure {
    Rectangle(Rectangle),
}

impl Figure {
    pub fn contains(&self, x: f64, y: f64) -> bool {
        match self {
            Figure::Rectangle(rect) => rect.contains(x, y),
        }
    }
}

#[derive(Debug, Clone)]
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
        ctx.rectangle(
            self.from.0,
            self.from.1,
            self.to.0 - self.from.0,
            self.to.1 - self.from.1,
        );
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
