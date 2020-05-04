mod renderer;
pub mod vertex;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use renderer::WglRect;

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

    let mut renderer = renderer::WglRenderer::new(&canvas)?;

    let texture = renderer.load_texture("./dwayne.png").await?;

    renderer.clear_screen([0.0, 0.0, 0.0, 1.0]);

    renderer.draw_texture(
        &texture,
        &WglRect {
            x: 0,
            y: 0,
            w: texture.w,
            h: texture.h,
        },
        &WglRect {
            x: 0,
            y: 0,
            w: texture.w / 2,
            h: texture.h / 2,
        },
    )?;

    renderer.draw_texture(
        &texture,
        &WglRect {
            x: 0,
            y: 0,
            w: texture.w,
            h: texture.h,
        },
        &WglRect {
            x: 100,
            y: 100,
            w: texture.w / 2,
            h: texture.h / 2,
        },
    )?;

    renderer.present();

    Ok(())
}
