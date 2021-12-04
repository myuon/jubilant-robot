// figuresの列からcontextの命令列への変換は最適化入れられそう
pub trait TDrawingContext {
    fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64);
    fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64);
    fn text(&self, text: &str, x: f64, y: f64, size: i32);

    fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64);
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

#[derive(Debug, Clone, Default)]
pub struct RectangleStyleOptions {
    pub fill: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub from: (f64, f64),
    pub to: (f64, f64),
    pub style_options: RectangleStyleOptions,
}

impl Rectangle {
    pub fn new(from: (f64, f64), to: (f64, f64)) -> Rectangle {
        Rectangle {
            from,
            to,
            style_options: Default::default(),
        }
    }

    pub fn size(&self) -> (f64, f64) {
        (self.to.0 - self.from.0, self.to.1 - self.from.1)
    }
}

impl TFigure for Rectangle {
    fn contains(&self, x: f64, y: f64) -> bool {
        self.from.0 <= x && x <= self.to.0 && self.from.1 <= y && y <= self.to.1
    }

    fn render(&self, context: &dyn TDrawingContext) {
        if self.style_options.fill.unwrap_or(false) {
            context.fill_rect(self.from.0, self.from.1, self.size().0, self.size().1);
        } else {
            context.stroke_rect(self.from.0, self.from.1, self.size().0, self.size().1);
        }
    }
}
