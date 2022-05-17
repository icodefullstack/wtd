use wasm_bindgen::prelude::*;
// use rand::prelude::*;
use web_sys::{console};
use wasm_bindgen::JsCast;
use std::sync::Mutex;
use std::rc::Rc;
use std::any::type_name;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Rect {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}

#[derive(Deserialize)]
struct Cell {
    frame: Rect,
}
#[derive(Deserialize)]
struct Sheet {
    frames: HashMap<String, Cell>,
}


#[allow(dead_code)]
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
        let (_error_tx, _error_rx) = futures::channel::oneshot::channel::<Result<(), JsValue>>();

        let success_tx = Rc::new(Mutex::new(Some(success_tx)));
        let error_tx = Rc::clone(&success_tx);

        let img = web_sys::HtmlImageElement::new().unwrap();
        img.set_src("rhb.png");

        let type_str = format!("tipul = {:?}", type_of(&img) );


        console::log_1(&JsValue::from(&type_str));

        let callback = Closure::once(move || {
            if let Some(success_tx) = success_tx.lock().ok()
            .and_then(|mut opt| opt.take()) {
            success_tx.send(Ok(())).ok();
        }});

        let error_cb  = Closure::once(move |err| {
            if let Some(error_tx) = error_tx.lock().ok()
            .and_then(|mut opt| opt.take()) {
            error_tx.send(Err(err)).ok();
            }    
        });


        img.set_onload(Some(callback.as_ref().unchecked_ref()));
        img.set_onerror(Some(error_cb.as_ref().unchecked_ref()));


        let json = fetch_json("rhb.json").await.expect("Could not fetch rhb.json");

        let sheet: Sheet = json
        .into_serde()
        .expect("Could not convert rhb.json into a Sheet structure");

        
        success_rx.await.ok();
        let mut frame = -1;
        let interval_callback = Closure::wrap(Box::new(move || {
            frame = (frame + 1) % 8;
            let frame_name = format!("Run ({}).png", frame + 1);
            let sprite = sheet.frames.get(&frame_name).expect("Cell not found");
            context.clear_rect(0.0, 0.0, 600.0, 600.0);
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &img,
                sprite.frame.x.into(),
                sprite.frame.y.into(),
                sprite.frame.w.into(),
                sprite.frame.h.into(),
                0.0,
                0.0,
                sprite.frame.w.into(),
                sprite.frame.h.into(),
                ).ok();
        }) as Box<dyn FnMut()>);

        window.set_interval_with_callback_and_timeout_and_arguments_0(
            interval_callback.as_ref().unchecked_ref(),
            50,
        ).ok();
        interval_callback.forget();
    });
    

    Ok(())
}



fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}


#[allow(dead_code)]
async fn fetch_json(json_path: &str) -> Result<JsValue, JsValue> {


    let window = web_sys::window().unwrap();

    let resp_value = wasm_bindgen_futures::JsFuture::from(
        window.fetch_with_str(json_path)
    ).await?;

    let resp: web_sys::Response = resp_value.dyn_into()?;
    wasm_bindgen_futures::JsFuture::from(resp.json()?).await
    
    }