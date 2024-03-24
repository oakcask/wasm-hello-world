use crate::{error::Error, gl::{self, ColoredSliceTriangleStrip, Primitive, Shader, GL}, vec3, vec4};
use crate::math::Matrix4;
use log::error;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

pub struct App {
    gl: GL,
    cube: Primitive,
    cube_shader: Shader,
    counter: i32
}

impl App {
    pub fn init(id: &str) -> Result<App, Error> {
        let gl = GL::init(id)?;
        let context = gl.context();

        /*
        // Prepearing Off-screen Buffer
        let rbuf = context.create_renderbuffer().ok_or_else(||
            String::from("createRenderBufferFailed")
        )?;
        context.bind_renderbuffer(WebGl2RenderingContext::RENDERBUFFER, Some(&rbuf));
        let (width, height) = screen.size();
        context.renderbuffer_storage(WebGl2RenderingContext::RENDERBUFFER, WebGl2RenderingContext::DEPTH_STENCIL, width as i32, height as i32);

        let tex = context.create_texture().ok_or_else(|| String::from("createTexture failed"))?;
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&tex));
        context.tex_storage_2d(WebGl2RenderingContext::TEXTURE_2D, 1, WebGl2RenderingContext::RGBA8,
            width as i32,
            height as i32);

        // context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
        //    WebGl2RenderingContext::TEXTURE_2D,
        //    0,
        //    WebGl2RenderingContext::RGBA8 as i32,
        //    width as i32,
        //    height as i32,
        //    0,
        //    WebGl2RenderingContext::RGBA,
        //    WebGl2RenderingContext::UNSIGNED_BYTE,
        //    None
        // )?;
        // context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        // context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        // context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);
        // context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);

        let fbuf = context.create_framebuffer().ok_or_else(||
            String::from("createFramebuffer failed")
        )?;
        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(&fbuf));

        context.framebuffer_renderbuffer(
           WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::DEPTH_STENCIL_ATTACHMENT,
            WebGl2RenderingContext::RENDERBUFFER,
            Some(&rbuf),
        );
        context.framebuffer_texture_2d(
            WebGl2RenderingContext::FRAMEBUFFER,
            WebGl2RenderingContext::COLOR_ATTACHMENT0,
            WebGl2RenderingContext::TEXTURE_2D,
            Some(&tex),
            0);

        */

        let vert_shader_source = r##"#version 300 es
            in vec4 position;
            in vec4 color;
            uniform mat4 mvp;
            out vec4 vColor;
            void main() {
                gl_Position = mvp * position;
                vColor = color;
            }
            "##;
        let frag_shader_source = r##"#version 300 es
            precision highp float;
            in vec4 vColor;
            out vec4 outColor;
            void main() {
                outColor = vColor;
            }
            "##;
        let cube_shader = gl::create_shader(&gl, vert_shader_source, frag_shader_source)?;
        
        #[rustfmt::skip]
        let cube = [
            // http://www.cs.umd.edu/gvil/papers/av_ts.pdf
            //
            //    1---------2
            //   /|        /|
            //  / |       / |
            // 3---------4  |
            // |  5- - - |- 6
            // | /       | /
            // |/        |/
            // 8---------7
            //                          
            // 4 3 7 8 5 3 1 4 2 7 6 5 2 1 
            (vec3!( 0.5,  0.5,  0.5), vec4!(0.0, 1.0, 0.0, 1.0)), // 4
            (vec3!(-0.5,  0.5,  0.5), vec4!(0.0, 0.0, 1.0, 1.0)), // 3
            (vec3!( 0.5, -0.5,  0.5), vec4!(1.0, 0.0, 0.0, 1.0)), // 7
            (vec3!(-0.5, -0.5,  0.5), vec4!(0.0, 1.0, 0.0, 1.0)), // 8
            (vec3!(-0.5, -0.5, -0.5), vec4!(0.0, 0.0, 1.0, 1.0)), // 5
            (vec3!(-0.5,  0.5,  0.5), vec4!(1.0, 0.0, 0.0, 1.0)), // 3
            (vec3!(-0.5,  0.5, -0.5), vec4!(0.0, 1.0, 0.0, 1.0)), // 1
            (vec3!( 0.5,  0.5,  0.5), vec4!(0.0, 0.0, 1.0, 1.0)), // 4
            (vec3!( 0.5,  0.5, -0.5), vec4!(1.0, 0.0, 0.0, 1.0)), // 2
            (vec3!( 0.5, -0.5,  0.5), vec4!(0.0, 1.0, 0.0, 1.0)), // 7
            (vec3!( 0.5, -0.5, -0.5), vec4!(0.0, 0.0, 1.0, 1.0)), // 6
            (vec3!(-0.5, -0.5, -0.5), vec4!(1.0, 0.0, 0.0, 1.0)), // 5
            (vec3!( 0.5,  0.5, -0.5), vec4!(0.0, 1.0, 0.0, 1.0)), // 2
            (vec3!(-0.5,  0.5, -0.5), vec4!(0.0, 0.0, 1.0, 1.0)), // 1
        ];
        let cube = ColoredSliceTriangleStrip(&cube);
        let cube = gl::create_primitive(context, cube)?;

        /*
        let vert_shader_source = r##"#version 300 es
            in vec4 position;
            in vec2 textureCoord;
            out vec2 vTextureCoord;
            out vec4 vColor;
            void main() {
                gl_Position = position;
                vTextureCoord = textureCoord;
                vColor = vec4(1.0, 1.0, 1.0, 1.0);
            }
            "##;
        let frag_shader_source = r##"#version 300 es
            precision mediump float;
            in vec4 position;
            in vec2 vTextureCoord;
            in vec4 vColor;
            uniform sampler2D texture0;
            out vec4 outColor;
            void main() {
                vec4 sampleColor = texture(texture0, vTextureCoord);
                outColor = vColor * sampleColor;
            }
            "##;
        let shader = gl::create_shader(&screen, vert_shader_source, frag_shader_source)?;
        let verts = [
            (Vector3(-1.0, -1.0, 0.0), Vector2(0.0, 1.0)),
            (Vector3(-1.0, 1.0, 0.0), Vector2(0.0, 0.0)),
            (Vector3(1.0, -1.0, 0.0), Vector2(1.0, 1.0)),
            (Vector3(1.0, 1.0, 0.0), Vector2(1.0, 0.0)),
        ];
        let verts = UVMappedSliceTriangleStrip(&verts);
        let shape = gl::create_primitive(context, verts)?;
        */

        /*
        screen.clear((0.0, 0.0, 0.0, 1.0));
        shader.enable();
        context.active_texture(WebGl2RenderingContext::TEXTURE0);
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&tex));
        shader.set_uniform_sampler2d("texture0", 0);
        shader.draw(&shape);
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
        shader.disable();
        */

        Ok(App {
            gl,
            cube,
            cube_shader,
            counter: 0
        })
    }

    pub fn start(self) {
        let mut app = self;
        let clo = Closure::once_into_js(move |t: JsValue| {
            let performance_clock_time = t.as_f64().unwrap();
            if let Err(e) = app.tick(performance_clock_time) {
                error!("{:?}", e);
                return;
            }

            app.start();
        });

        Self::request_animation_frame(clo).unwrap();
    }

    fn tick(&mut self, _performance_clock_time: f64) -> Result<(), JsValue> {
        let context = self.gl.context();

        let deg = (self.counter % 360) as f32 / 180.0 * 2.0 * std::f32::consts::PI;

        let world = 
            Matrix4::yaw_rotation(deg) * Matrix4::pitch_rotation(deg)
            * Matrix4::scaling(4.0, 4.0, 4.0);

        let fov_y = 45.0/180.0 * std::f32::consts::PI;
        let pv = 
            Matrix4::perspective_fov(fov_y, self.gl.screen_aspect_ratio(), 0.1, 5000.0) *
            Matrix4::look_at(
                vec3!(0.0, 0.0, 10.0),
                vec3!(0.0, 0.0, 0.0),
                vec3!(0.0, 1.0, 0.0));

        let pv = pv * world;


        self.gl.clear((0.0, 0.0, 0.0, 1.0));
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        context.enable(WebGl2RenderingContext::CULL_FACE);
        self.cube_shader.enable();
        self.cube_shader.set_uniform_matrix4("mvp", &pv);
        self.cube_shader.draw(&self.cube);
        self.cube_shader.disable();

        context.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
        context.bind_renderbuffer(WebGl2RenderingContext::RENDERBUFFER, None);
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        self.counter += 1;
        if self.counter >= 360 {
            self.counter = 0;
        }

        Ok(())
    }

    fn request_animation_frame(f: JsValue) -> Result<(), JsValue> {
        if let Some(window) = web_sys::window() {
            window.request_animation_frame(f.as_ref().unchecked_ref())?;
            return Ok(());
        }
        
        Err(JsValue::from_str("requestAnimationFrame failed"))
    }

}
