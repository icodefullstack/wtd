use wasm_bindgen::prelude::*;
use rand::prelude::*;
use web_sys::console;
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

        draw_triangle(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)]);

        draw_triangle(&context, [(300.0, 0.0), (150.0, 300.0), (450.0, 300.0)]);
        draw_triangle(&context, [(150.0, 300.0), (0.0, 600.0), (300.0, 600.0)]);
        draw_triangle(&context, [(450.0, 300.0), (300.0, 600.0), (600.0, 600.0)]);
    Ok(())
}


fn draw_triangle(
    context: &web_sys::CanvasRenderingContext2d,
    points: [(f64, f64); 3]
) {
    let mut rng = thread_rng();
    let color:(i32, i32, i32) = (rng.gen_range(0..255),rng.gen_range(0..255),rng.gen_range(0..255),);

    let color_str = format!("rgb({}, {}, {})", color.0, color.1, color.2);
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));

    console::log_1(&JsValue::from_str(&color_str));

        let [top, left, right] = points;

        context.move_to(top.0, top.1);
        context.begin_path();
        context.line_to(left.0, left.1);
        context.line_to(right.0, right.1);
        context.line_to(top.0, top.1);
        context.close_path();
        context.stroke();
        context.fill();

}