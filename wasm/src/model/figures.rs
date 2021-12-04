use std::cell::Cell;

// figuresの列からcontextの命令列への変換は最適化入れられそう
pub trait TDrawingContext {
    fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64);
    fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64);
    fn text(&self, text: &str, x: f64, y: f64, size: i32);

    fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64);
    fn set_stroke_dashed(&self, patterns: Vec<i32>);
    fn reset_stroke(&self);
    fn set_fill_color(&self, color: &str);
}

pub trait TFigure: std::fmt::Debug {
    fn contains(&self, x: f64, y: f64) -> bool;
    fn render(&self, ctx: &dyn TDrawingContext);
    fn click(&self) {}
    fn move_to(&self, x: f64, y: f64);
}

#[derive(Debug, Clone, Default)]
pub struct RectangleStyleOptions {
    pub fill: Option<bool>,
    pub fill_color: Option<String>,
}

#[derive(Debug)]
pub struct Rectangle {
    pub from: Cell<(f64, f64)>,
    pub size: (f64, f64),
    pub style_options: RectangleStyleOptions,
}

impl Rectangle {
    pub fn new(from: (f64, f64), size: (f64, f64)) -> Rectangle {
        Rectangle {
            from: Cell::new(from),
            size,
            style_options: Default::default(),
        }
    }

    pub fn to(&self) -> (f64, f64) {
        let from = self.from.get();
        (from.0 + self.size.0, from.1 + self.size.1)
    }
}

impl TFigure for Rectangle {
    fn contains(&self, x: f64, y: f64) -> bool {
        let (x1, y1) = self.from.get();
        let (x2, y2) = self.to();
        x1 <= x && x <= x2 && y1 <= y && y <= y2
    }

    fn render(&self, context: &dyn TDrawingContext) {
        if let Some(c) = &self.style_options.fill_color {
            context.set_fill_color(c);
        }

        if self.style_options.fill.unwrap_or(false) {
            context.fill_rect(
                self.from.get().0,
                self.from.get().1,
                self.size.0,
                self.size.1,
            );
        } else {
            context.stroke_rect(
                self.from.get().0,
                self.from.get().1,
                self.size.0,
                self.size.1,
            );
        }
    }

    fn move_to(&self, x: f64, y: f64) {
        self.from.set((x, y));
    }
}
