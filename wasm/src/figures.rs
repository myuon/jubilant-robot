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
}
