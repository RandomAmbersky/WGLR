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

const VIEWPORT_WIDTH: i32 = 512;
const VIEWPORT_HEIGHT: i32 = 512;

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

    canvas.set_width(VIEWPORT_WIDTH as u32);

    canvas.set_height(VIEWPORT_HEIGHT as u32);

    let mut renderer = renderer::WglRenderer2d::new(&canvas, (VIEWPORT_WIDTH as f32, VIEWPORT_HEIGHT as f32))?;

    let render_target = renderer.create_render_target(VIEWPORT_WIDTH, VIEWPORT_HEIGHT)?;

    let texture = renderer.load_texture("./dwayne.png").await?;

    renderer.set_render_target(&render_target)?;

    renderer.clear_render_target([0.0, 0.0, 0.0, 1.0]);

    renderer.draw_texture(
        &texture,
        &WglRect::new(0, 0, texture.w, texture.h),
        &WglRect::new(0, 0, texture.w, texture.h),
    )?;

    log(&format!("{} {}", texture.w, texture.h));

    renderer.set_render_target(None)?;

    renderer.clear_render_target([0.0, 1.0, 0.0, 1.0]);

    renderer.draw_texture(
        &render_target,
        &WglRect::new(0, 0, render_target.w, render_target.h),
        &WglRect::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
    )?;

    renderer.present();

    Ok(())
}
