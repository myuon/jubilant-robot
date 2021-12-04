// figuresの列からcontextの命令列への変換は最適化入れられそう
pub trait TDrawingContext {
    fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64);
    fn rectangle(&self, x: f64, y: f64, w: f64, h: f64);
    fn set_stroke_dashed(&self, patterns: Vec<i32>);
    fn reset_stroke(&self);
}

pub trait TCanvas {
    fn clear(&self);
}

pub trait TFigure: std::fmt::Debug {
    fn contains(&self, x: f64, y: f64) -> bool;
    fn render(&self, context: &dyn TDrawingContext);
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
}

impl TFigure for Rectangle {
    fn contains(&self, x: f64, y: f64) -> bool {
        self.from.0 <= x && x <= self.to.0 && self.from.1 <= y && y <= self.to.1
    }

    fn render(&self, context: &dyn TDrawingContext) {
        context.rectangle(
            self.from.0,
            self.from.1,
            self.to.0 - self.from.0,
            self.to.1 - self.from.1,
        );
    }
}
