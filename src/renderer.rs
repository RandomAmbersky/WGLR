use crate::vertex::{AsF32Slice, Vertex2D};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    HtmlCanvasElement, HtmlImageElement, WebGlProgram, WebGlRenderingContext, WebGlShader,
    WebGlTexture,
};

type GlContext = WebGlRenderingContext;

#[repr(C)]
pub struct WglRect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub struct WglRenderer {
    context: GlContext,
    program: WebGlProgram,
    buffer: [Vertex2D; 4],
    indices: [i16; 6],
}

impl WglRenderer {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self, JsValue> {
        let context = canvas
            .get_context("webgl")?
            .ok_or("")?
            .dyn_into::<GlContext>()?;

        context.enable(GlContext::BLEND);

        context.blend_func(GlContext::SRC_ALPHA, GlContext::ONE_MINUS_SRC_ALPHA);

        let vert_shader = Self::compile_shader(
            &context,
            GlContext::VERTEX_SHADER,
            include_str!("./shaders/vert.glsl"),
        )?;

        let frag_shader = Self::compile_shader(
            &context,
            GlContext::FRAGMENT_SHADER,
            include_str!("./shaders/frag.glsl"),
        )?;

        let program = Self::link_program(&context, &vert_shader, &frag_shader)?;

        let buffer = [
            //top-left
            Vertex2D {
                pos: [0.0, 1.0],
                tex_coords: [0.0, 1.0],
            },
            //bottom-left
            Vertex2D {
                pos: [0.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            //top-right
            Vertex2D {
                pos: [1.0, 1.0],
                tex_coords: [1.0, 1.0],
            },
            //bottom-right
            Vertex2D {
                pos: [1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
        ];

        let indices: [i16; 6] = [0, 1, 2, 2, 1, 3];

        context.use_program(Some(&program));

        Ok(Self {
            context,
            program,
            buffer,
            indices,
        })
    }

    pub fn present(&self) {
        self.context.flush();
    }

    pub fn clear_screen(&self, color: [f32; 4]) {
        self.context
            .clear_color(color[0], color[1], color[2], color[3]);
        self.context.clear(GlContext::COLOR_BUFFER_BIT);
    }

    pub fn draw_texture(
        &mut self,
        texture: &WglTexture,
        src_rect: &WglRect,
        dest_rect: &WglRect,
    ) -> Result<(), String> {
        let left = src_rect.x as f32 / texture.w as f32;

        if left > 1.0 {
            return Ok(());
        }

        let top = src_rect.y as f32 / texture.h as f32;

        if top > 1.0 {
            return Ok(());
        }

        let right = src_rect.w as f32 / texture.w as f32;

        let bottom = src_rect.h as f32 / texture.h as f32;

        unsafe {
            //invert the texture coordinates because we have to lmao
            self.buffer.get_unchecked_mut(0).tex_coords = [left, bottom];
            self.buffer.get_unchecked_mut(1).tex_coords = [left, top];
            self.buffer.get_unchecked_mut(2).tex_coords = [right, bottom];
            self.buffer.get_unchecked_mut(3).tex_coords = [right, top];
        }

        let dest_rect_x_offset = self.context.get_uniform_location(&self.program, "destRect.x");
        let dest_rect_y_offset = self.context.get_uniform_location(&self.program, "destRect.y");
        let dest_rect_w_offset = self.context.get_uniform_location(&self.program, "destRect.w");
        let dest_rect_h_offset = self.context.get_uniform_location(&self.program, "destRect.h");
        self.context.uniform1f(dest_rect_x_offset.as_ref(), dest_rect.x as f32);
        self.context.uniform1f(dest_rect_y_offset.as_ref(), dest_rect.y as f32);
        self.context.uniform1f(dest_rect_w_offset.as_ref(), dest_rect.w as f32);
        self.context.uniform1f(dest_rect_h_offset.as_ref(), dest_rect.h as f32);

        let texture_dimensions_offset = self.context.get_uniform_location(&self.program, "textureDimensions");
        self.context.uniform2fv_with_f32_array(
            texture_dimensions_offset.as_ref(),
            &[texture.w as f32, texture.h as f32],
        );

        let projection_offset = self.context.get_uniform_location(&self.program, "projection");
        let projection: cgmath::Matrix4<f32> =
            cgmath::ortho(0.0, 800.0, 600.0, 0.0, 0.0, 100.0).into();
        let projection: &[f32; 16] = projection.as_ref();
        self.context.uniform_matrix4fv_with_f32_array(
            projection_offset.as_ref(),
            false,
            projection.as_ref(),
        );

        let vertex_buffer = self.context.create_buffer().ok_or("Failed to create buffer")?;

        self.context.bind_buffer(GlContext::ARRAY_BUFFER, Some(&vertex_buffer));

        unsafe {
            let vertex_array = js_sys::Float32Array::view(self.buffer.as_f32_slice());

            self.context.buffer_data_with_array_buffer_view(
                GlContext::ARRAY_BUFFER,
                &vertex_array,
                GlContext::STATIC_DRAW,
            );
        }

        self.context.vertex_attrib_pointer_with_i32(0, 2, GlContext::FLOAT, false, 16, 0);
        self.context.vertex_attrib_pointer_with_i32(1, 2, GlContext::FLOAT, false, 16, 8);

        self.context.enable_vertex_attrib_array(0);
        self.context.enable_vertex_attrib_array(1);

        let index_buffer = self.context.create_buffer().ok_or("Failed to create buffer")?;

        self.context.bind_buffer(GlContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

        unsafe {
            let index_array = js_sys::Int16Array::view(&self.indices);

            self.context.buffer_data_with_array_buffer_view(
                GlContext::ELEMENT_ARRAY_BUFFER,
                &index_array,
                GlContext::STATIC_DRAW,
            );
        }

        self.context.bind_texture(GlContext::TEXTURE_2D, Some(&texture.texture_data));
        self.context.draw_elements_with_i32(GlContext::TRIANGLES, 6, GlContext::UNSIGNED_SHORT, 0);
        self.context.bind_texture(GlContext::TEXTURE_2D, None);

        Ok(())
    }

    pub async fn load_texture(&mut self, path: &str) -> Result<WglTexture, JsValue> {
        let image = HtmlImageElement::new()?;

        let texture = self
            .context
            .create_texture()
            .ok_or_else(|| JsValue::from_str("Unable to create texture."))?;

        let promise = js_sys::Promise::new(&mut |resolve, reject| {
            image.set_src(path);
            image.set_onload(Some(&resolve));
            image.set_onerror(Some(&reject));
        });

        wasm_bindgen_futures::JsFuture::from(promise).await?;

        self.context
            .bind_texture(GlContext::TEXTURE_2D, Some(&texture));

        self.context.tex_parameteri(
            GlContext::TEXTURE_2D,
            GlContext::TEXTURE_WRAP_S,
            GlContext::CLAMP_TO_EDGE as i32,
        );

        self.context.tex_parameteri(
            GlContext::TEXTURE_2D,
            GlContext::TEXTURE_WRAP_T,
            GlContext::CLAMP_TO_EDGE as i32,
        );

        self.context.tex_parameteri(
            GlContext::TEXTURE_2D,
            GlContext::TEXTURE_MIN_FILTER,
            GlContext::LINEAR as i32,
        );

        self.context.tex_parameteri(
            GlContext::TEXTURE_2D,
            GlContext::TEXTURE_MAG_FILTER,
            GlContext::LINEAR as i32,
        );

        let target = GlContext::TEXTURE_2D;
        let level = 0;
        let internal_format = GlContext::RGBA;
        let src_format = GlContext::RGBA;
        let src_type = GlContext::UNSIGNED_BYTE;

        self.context.tex_image_2d_with_u32_and_u32_and_image(
            target,
            level,
            internal_format as i32,
            src_format,
            src_type,
            &image,
        )?;

        self.context.bind_texture(GlContext::TEXTURE_2D, None);

        let texture = WglTexture {
            texture_data: texture,
            w: image.width() as i32,
            h: image.height() as i32,
        };

        Ok(texture)
    }

    fn compile_shader(
        context: &GlContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, GlContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader.")))
        }
    }

    fn link_program(
        context: &GlContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<WebGlProgram, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create Shader object."))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, GlContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(program)
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }
}

pub struct WglTexture {
    pub texture_data: WebGlTexture,
    pub w: i32,
    pub h: i32,
}
