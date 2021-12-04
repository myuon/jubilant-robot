use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::{
    model::{figures::TFigure, renderer::Renderer},
    utils::event::MouseUpEvent,
};

pub struct UI {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    renderer: Renderer,
}

impl UI {
    pub fn create_by_element_id(element_id: &str) -> Result<Self, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(element_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

        Ok(UI {
            canvas,
            context,
            renderer: Renderer::new(),
        })
    }

    pub fn get_context(&self) -> &CanvasRenderingContext2d {
        &self.context
    }

    fn clear(&self) {
        self.context.clear_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );
    }

    pub fn render(&self) {
        self.clear();
        self.renderer.render(&self.context);
    }

    pub fn register(&self, figure: impl TFigure + 'static) {
        self.renderer.register(figure);
    }

    pub fn handle_mouse_up(&self, event: MouseUpEvent) {
        self.renderer.handle_mouse_up(event);
    }

    pub fn register_on_mousedown(
        &self,
        closure: Box<dyn FnMut(MouseEvent)>,
    ) -> Result<(), JsValue> {
        let closure = Closure::wrap(closure);
        self.canvas
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }

    pub fn register_on_mousemove(
        &self,
        closure: Box<dyn FnMut(MouseEvent)>,
    ) -> Result<(), JsValue> {
        let closure = Closure::wrap(closure);
        self.canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }

    pub fn register_on_mouseup(&self, closure: Box<dyn FnMut(MouseEvent)>) -> Result<(), JsValue> {
        let closure = Closure::wrap(closure);
        self.canvas
            .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();

        Ok(())
    }
}
