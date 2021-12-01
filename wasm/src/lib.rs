mod figures;
mod utils;

use figures::Rectangle;
use figures::TDrawingContext;
use std::cell::Cell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

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

    fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        self.clear_rect(x, y, w, h)
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
    dragging: bool,
}

impl DragAndDropEvent {
    fn into_rectangle(self) -> figures::Rectangle {
        figures::Rectangle::new(self.from, self.to)
    }
}

fn get_canvas_context(
    element_id: &str,
) -> Result<(HtmlCanvasElement, CanvasRenderingContext2d), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(element_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    Ok((canvas, context))
}

fn clear_canvas(canvas: &HtmlCanvasElement, context: &CanvasRenderingContext2d) {
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let (canvas, control_context) = get_canvas_context("control-layer")?;
    let canvas = Rc::new(canvas);
    let control_context = Rc::new(control_context);

    let (_, paint_context) = get_canvas_context("paint-layer")?;
    let paint_context = Rc::new(paint_context);

    let dnd = Rc::new(Cell::new(DragAndDropEvent::default()));

    {
        let dnd = dnd.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let point = (event.offset_x() as f64, event.offset_y() as f64);

            let mut d = dnd.get();
            d.from = point;
            d.dragging = true;
            dnd.set(d);
        }) as Box<dyn FnMut(_)>);
        canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let dnd = dnd.clone();
        let control_context = control_context.clone();
        let closure = {
            let canvas = canvas.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let d = dnd.get();
                if d.dragging {
                    clear_canvas(&canvas, &control_context);
                    let rect =
                        Rectangle::new(d.from, (event.offset_x() as f64, event.offset_y() as f64));

                    rect.draw(&*control_context);
                }
            }) as Box<dyn FnMut(_)>)
        };
        canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let dnd = dnd.clone();
        let paint_context = paint_context.clone();
        let closure = {
            let canvas = canvas.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let mut d = dnd.get();
                d.to = (event.offset_x() as f64, event.offset_y() as f64);
                d.dragging = false;
                dnd.set(d);

                let rect = dnd.get().into_rectangle();
                rect.draw(&*paint_context);

                // イベントが確定したらcontrol layerは消去する
                clear_canvas(&canvas, &control_context);
            }) as Box<dyn FnMut(_)>)
        };
        canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
