mod model;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use js_sys::Array;
use model::figures::Figure;
use model::figures::TCanvas;
use model::figures::TDrawingContext;
use model::renderer::Renderer;
use std::cell::Cell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;

use crate::model::figures::Rectangle;
use crate::utils::console_log;
use crate::utils::event::DragAndDropEvent;
use crate::utils::event::MouseUpEvent;

impl TDrawingContext for CanvasRenderingContext2d {
    fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        self.clear_rect(x, y, w, h)
    }

    fn rectangle(&self, x: f64, y: f64, w: f64, h: f64) {
        self.stroke_rect(x, y, w, h);
    }

    fn set_stroke_dashed(&self, patterns: Vec<i32>) {
        let arr = Array::new();
        for p in patterns {
            arr.push(&JsValue::from_f64(p as f64));
        }
        self.set_line_dash(&arr).unwrap();
    }

    fn reset_stroke(&self) {
        self.set_line_dash(&Array::new()).unwrap();
    }
}

pub struct PaintingCanvas {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    figures: Cell<Vec<Figure>>,
}

impl PaintingCanvas {
    pub fn create_by_element_id(element_id: &str) -> Result<Self, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(element_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        Ok(PaintingCanvas {
            canvas,
            context,
            figures: Cell::new(vec![]),
        })
    }
}

impl TCanvas for PaintingCanvas {
    fn clear(&self) {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
    }

    fn register(&self, figure: Figure) {
        let mut figs = self.figures.take();
        figs.push(figure);
        self.figures.set(figs);
    }
}

#[derive(Clone)]
struct App {
    control_canvas: Rc<PaintingCanvas>,
    paint_canvas: Rc<PaintingCanvas>,
    renderer: Rc<Renderer>,
}

impl App {
    fn new(control_canvas: Rc<PaintingCanvas>, paint_canvas: Rc<PaintingCanvas>) -> Self {
        App {
            control_canvas,
            paint_canvas,
            renderer: Rc::new(Renderer::new()),
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_log!("Application initialized.");

    let control_canvas = Rc::new(PaintingCanvas::create_by_element_id("control-canvas")?);
    let paint_canvas = Rc::new(PaintingCanvas::create_by_element_id("paint-canvas")?);

    let app = App::new(control_canvas, paint_canvas);

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
        app.control_canvas
            .canvas
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let dnd = dnd.clone();
        let control_canvas = app.control_canvas.clone();
        let closure = {
            let control_canvas = control_canvas.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let d = dnd.get();
                if d.dragging {
                    control_canvas.clear();
                    let rect =
                        Rectangle::new(d.from, (event.offset_x() as f64, event.offset_y() as f64));

                    control_canvas.context.set_stroke_dashed(vec![5, 5]);
                    rect.draw(&control_canvas.context);
                    control_canvas.context.reset_stroke();
                }
            }) as Box<dyn FnMut(_)>)
        };
        app.control_canvas
            .canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let dnd = dnd.clone();
        let closure = {
            let app = app.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let mut d = dnd.get();
                d.to = (event.offset_x() as f64, event.offset_y() as f64);
                d.dragging = false;
                dnd.set(d);

                app.renderer.on_mouse_dnd(d, &app.paint_canvas.context);

                // イベントが確定したらcontrol layerは消去する
                app.control_canvas.clear();

                app.renderer.on_mouse_up(MouseUpEvent {
                    at: (event.offset_x() as f64, event.offset_y() as f64),
                })
            }) as Box<dyn FnMut(_)>)
        };
        app.control_canvas
            .canvas
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
