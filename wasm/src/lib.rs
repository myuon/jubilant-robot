mod components;
mod model;
mod ui;
mod utils;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use components::Button;
use js_sys::Array;
use model::figures::TDrawingContext;
use model::figures::TFigure;
use std::cell::Cell;
use std::f64;
use std::rc::Rc;
use ui::UI;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use crate::model::figures::Rectangle;
use crate::utils::console_log;
use crate::utils::event::DragAndDropEvent;
use crate::utils::event::MouseUpEvent;

impl TDrawingContext for CanvasRenderingContext2d {
    fn stroke_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        self.stroke_rect(x, y, w, h);
    }

    fn fill_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        self.fill_rect(x, y, w, h);
    }

    fn text(&self, text: &str, x: f64, y: f64, size: i32) {
        self.set_fill_style(&JsValue::from_str("white"));
        self.set_font(format!("{}px sans-serif", size).as_str());
        self.fill_text(text, x, y).unwrap();
    }

    fn clear_rect(&self, x: f64, y: f64, w: f64, h: f64) {
        self.clear_rect(x, y, w, h)
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

    fn set_fill_color(&self, color: &str) {
        self.set_fill_style(&JsValue::from_str(color));
    }
}

#[derive(Clone)]
struct App {
    control: Rc<UI>,
    control_select_rect: Rc<Cell<Option<Rectangle>>>,
    paint: Rc<UI>,
    dnd_event: Rc<Cell<DragAndDropEvent>>,
}

impl App {
    fn new(control_canvas: Rc<UI>, paint_canvas: Rc<UI>) -> Self {
        App {
            control: control_canvas,
            control_select_rect: Rc::new(Cell::new(None)),
            paint: paint_canvas,
            dnd_event: Rc::new(Cell::new(DragAndDropEvent::default())),
        }
    }

    fn initialize(&self) {
        let paint = self.paint.clone();
        self.control.register(Button::new(
            "CLEAR".to_string(),
            Rectangle::new((0.0, 0.0), (100.0, 40.0)),
            Box::new(move || {
                paint.clear();
            }),
        ));
        self.control.render();
    }

    fn register_mousedown(&self) -> Result<(), JsValue> {
        let dnd = self.dnd_event.clone();
        self.control
            .register_on_mousedown(Box::new(move |event: web_sys::MouseEvent| {
                let point = (event.offset_x() as f64, event.offset_y() as f64);

                let mut d = dnd.get();
                d.from = point;
                d.dragging = true;
                dnd.set(d);
            }))?;

        Ok(())
    }

    fn register_mousemove(&self) -> Result<(), JsValue> {
        let dnd = self.dnd_event.clone();
        let app = self.clone();
        self.control
            .register_on_mousemove(Box::new(move |event: web_sys::MouseEvent| {
                let d = dnd.get();
                if d.dragging {
                    app.control.render();

                    let rect =
                        Rectangle::new(d.from, (event.offset_x() as f64, event.offset_y() as f64));

                    let context = app.control.get_context();
                    context.set_stroke_dashed(vec![5, 5]);
                    rect.render(context);
                    context.reset_stroke();
                }
            }))?;

        Ok(())
    }

    fn register_mouseup(&self) -> Result<(), JsValue> {
        let dnd = self.dnd_event.clone();
        let app = self.clone();
        self.control
            .register_on_mouseup(Box::new(move |event: web_sys::MouseEvent| {
                let mut d = dnd.get();
                d.to = (event.offset_x() as f64, event.offset_y() as f64);
                d.dragging = false;
                dnd.set(d);

                // DnDで矩形を登録する
                let rect = Rectangle::new(d.from, d.to);
                app.paint.register(rect);
                app.paint.render();

                app.paint.handle_mouse_up(MouseUpEvent {
                    at: (event.offset_x() as f64, event.offset_y() as f64),
                });

                app.control.render();
                app.control.handle_mouse_up(MouseUpEvent {
                    at: (event.offset_x() as f64, event.offset_y() as f64),
                });
            }))?;

        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    console_log!("Application initialized.");

    let control_canvas = Rc::new(UI::create_by_element_id("control-canvas")?);
    let paint_canvas = Rc::new(UI::create_by_element_id("paint-canvas")?);

    let app = App::new(control_canvas, paint_canvas);
    app.initialize();

    app.register_mousedown()?;
    app.register_mousemove()?;
    app.register_mouseup()?;

    Ok(())
}
