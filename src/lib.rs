use wasm_bindgen::prelude::*;
use rand::prelude::*;
use web_sys::{console, HtmlImageElement};
use wasm_bindgen::JsCast;
use std::sync::Mutex;
use std::rc::Rc;

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



    wasm_bindgen_futures::spawn_local(async move{
        let (success_tx, success_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();
        let (error_tx, error_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();

        let success_tx = Rc::new(Mutex::new(Some(success_tx)));
        let error_tx = Rc::clone(&success_tx);

        let img = web_sys::HtmlImageElement::new().unwrap();
        img.set_src("Idle (1).png");

        let callback = Closure::once(move || {
            if let Some(success_tx) = success_tx.lock().ok()
            .and_then(|mut opt| opt.take()) {
            success_tx.send(Ok(()));
        }});

        let error_cb  = Closure::once(move |err| {
            if let Some(error_tx) = error_tx.lock().ok()
            .and_then(|mut opt| opt.take()) {
            error_tx.send(Err(err));
            }    
        });


        img.set_onload(Some(callback.as_ref().unchecked_ref()));
        img.set_onerror(Some(error_cb.as_ref().unchecked_ref()));
        success_rx.await;
        context.draw_image_with_html_image_element(&img, 0.0, 0.0);
        
    });

    Ok(())
}