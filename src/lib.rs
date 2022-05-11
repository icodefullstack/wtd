use wasm_bindgen::prelude::*;
use rand::prelude::*;
use web_sys::{console, HtmlImageElement};
use wasm_bindgen::JsCast;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let img = web_sys::HtmlImageElement::new().unwrap();
    img.set_src("Idle (1).png");
    context.draw_image_with_html_image_element(&img, 0.0, 0.0);

    Ok(())
}