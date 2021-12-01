mod figures;
mod utils;

use figures::TDrawingContext;
use std::cell::Cell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;

impl TDrawingContext for CanvasRenderingContext2d {
    fn begin_path(&self) {
        self.begin_path()
    }

    fn stroke(&self) {
        self.stroke()
    }

    fn move_to(&self, x: f64, y: f64) {
        self.move_to(x, y)
    }

    fn line_to(&self, x: f64, y: f64) {
        self.line_to(x, y)
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Default, Clone, Copy)]
struct DragAndDropEvent {
    from: (f64, f64),
    to: (f64, f64),
}

impl DragAndDropEvent {
    fn into_rectangle(self) -> figures::Rectangle {
        figures::Rectangle::new(self.from, self.to)
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("control-layer").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = Rc::new(context);
    let dnd = Rc::new(Cell::new(DragAndDropEvent::default()));

    {
        let dnd = dnd.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let point = (event.offset_x() as f64, event.offset_y() as f64);

            let mut d = dnd.get();
            d.from = point;
            dnd.set(d);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure =
            Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {}) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let dnd = dnd.clone();
        let context = context.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut d = dnd.get();
            d.to = (event.offset_x() as f64, event.offset_y() as f64);
            dnd.set(d);

            let rect = dnd.get().into_rectangle();
            rect.draw(&*context);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
