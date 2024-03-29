use crate::{error::Error, gl::{ColoredSliceTriangleStrip, FrameBuffer, Primitive, Shader, Sprite, SpriteBatch, GL}, rect, vec3, vec4};
use crate::math::Matrix4;
use log::error;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

pub struct App {
    gl: GL,
    cube: Primitive,
    cube_shader: Shader,
    sprite: Sprite,
    frame_buffer: FrameBuffer,
    counter: i32
}

impl App {
    pub fn init(id: &str) -> Result<App, Error> {
        let gl = GL::init(id)?;

        // Prepearing Off-screen Buffer
        let frame_buffer = FrameBuffer::new(&gl, gl.screen_size())?;

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
        let cube_shader = Shader::new(&gl, vert_shader_source, frag_shader_source)?;
        
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
        let cube = Primitive::new(&gl, cube)?;
        cube_shader.enable_vertex_attribute(&cube);

        let sprite = Sprite::new(&gl, gl.screen_size())?;

        Ok(App {
            gl,
            cube,
            cube_shader,
            sprite,
            frame_buffer,
            counter: 0
        })
    }

    pub fn start(self) {
        let app = self;
        let clo = Closure::once_into_js(move |t: JsValue| {
            let performance_clock_time = t.as_f64().unwrap();
            match app.tick(performance_clock_time) {
                Ok(app) => app.start(),
                Err(e) => {
                    error!("{:?}", e);
                }
            }
        });

        Self::request_animation_frame(clo).unwrap();
    }

    fn tick(self, _performance_clock_time: f64) -> Result<Self, JsValue> {
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

        self.gl.bind_framebuffer(&self.frame_buffer);
        self.gl.clear((1.0, 0.0, 1.0, 0.0));
        self.gl.clear_depth(0.0);
        self.gl.clear_stencil(1);

        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        context.enable(WebGl2RenderingContext::CULL_FACE);
        self.cube_shader.enable();
        self.cube_shader.set_uniform_model_view_perspective(&pv);
        self.cube_shader.draw(&self.cube);
        self.cube_shader.disable();
        context.finish();

        self.gl.bind_framebuffer(self.gl.screen());
        self.gl.clear((0.0, 0.0, 0.0, 1.0));
        self.gl.clear_depth(0.0);
        self.gl.clear_stencil(1);

        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        context.enable(WebGl2RenderingContext::CULL_FACE);
        context.enable(WebGl2RenderingContext::BLEND);
        context.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

        self.cube_shader.enable();
        self.cube_shader.set_uniform_model_view_perspective(&pv);
        self.cube_shader.draw(&self.cube);
        self.cube_shader.disable();

        let mut batch = SpriteBatch::new();
        batch.add(self.frame_buffer.texture(), vec4!(0.0, 0.0, 1.0, 1.0), rect!(0, 0, 256, 256));

        context.disable(WebGl2RenderingContext::DEPTH_TEST);
        context.enable(WebGl2RenderingContext::CULL_FACE);
        context.enable(WebGl2RenderingContext::BLEND);
        context.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        
        self.sprite.draw(batch);

        context.finish();

        let mut new_counter = self.counter + 1;
        if new_counter >= 360 {
            new_counter = 0;
        }

        Ok(Self {
            counter: new_counter,
            ..self
        })
    }

    fn request_animation_frame(f: JsValue) -> Result<(), JsValue> {
        if let Some(window) = web_sys::window() {
            window.request_animation_frame(f.as_ref().unchecked_ref())?;
            return Ok(());
        }
        
        Err(JsValue::from_str("requestAnimationFrame failed"))
    }

}
