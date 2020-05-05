mod renderer;
pub mod vertex;

use renderer::WglRect;
use wasm_bindgen::prelude::*;

const VIEWPORT_WIDTH: i32 = 512;
const VIEWPORT_HEIGHT: i32 = 512;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    let mut renderer =
        renderer::WglRenderer2d::new("canvas", (VIEWPORT_WIDTH as f32, VIEWPORT_HEIGHT as f32))?;

    let render_target = renderer.create_render_target(VIEWPORT_WIDTH, VIEWPORT_HEIGHT)?;

    let texture = renderer.load_texture("./dwayne.png").await?;

    renderer.set_render_target(&render_target)?;

    renderer.clear_render_target([0.0, 0.0, 0.0, 1.0]);

    renderer.draw_texture(
        &texture,
        &WglRect::new(0, 0, texture.w, texture.h),
        &WglRect::new(100, 120, texture.w, texture.h),
    )?;

    renderer.set_render_target(None)?;

    renderer.clear_render_target([0.0, 1.0, 0.0, 1.0]);

    renderer.draw_texture(
        &render_target,
        &WglRect::new(0, 0, render_target.w, render_target.h),
        &WglRect::new(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
    )?;

    renderer.draw_texture(
        &texture,
        &WglRect::new(0, 0, texture.w, texture.h),
        &WglRect::new(20, 10, texture.w, texture.h),
    )?;

    renderer.present();

    Ok(())
}
