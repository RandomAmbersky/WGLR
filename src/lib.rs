mod renderer;
pub mod vertex;

use renderer::WglRect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or(JsValue::from_str("No window detected."))?;

    let document = window
        .document()
        .ok_or(JsValue::from_str("No document in window."))?;

    let canvas = document
        .get_element_by_id("canvas")
        .ok_or(JsValue::from_str("No canvas in document."))?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    canvas.set_width(800);

    canvas.set_height(600);

    let mut renderer = renderer::WglRenderer::new(&canvas, (800, 600))?;

    let render_target = renderer.create_render_target(128, 128)?;

    let texture = renderer.load_texture("./dwayne.png").await?;

    renderer.set_render_target(&render_target)?;

    renderer.clear_screen([0.0, 0.0, 0.0, 1.0]);

    renderer.draw_texture(
        &texture,
        &WglRect::new(0, 0, texture.w, texture.h),
        &WglRect::new(0, 0, texture.w, texture.h),
    )?;

    renderer.draw_texture(
        &texture,
        &WglRect::new(0, 0, texture.w, texture.h),
        &WglRect::new(0, 0, texture.w, texture.h),
    )?;

    renderer.set_render_target(None)?;

    renderer.draw_texture(
        &render_target,
        &WglRect::new(0, 0, render_target.w, render_target.h),
        &WglRect::new(0, 0, 800, 600),
    )?;

    renderer.draw_texture(
        &texture,
        &WglRect::new(0, 0, texture.w, texture.h),
        &WglRect::new(0, 0, texture.w, texture.h),
    )?;

    renderer.present();

    Ok(())
}
