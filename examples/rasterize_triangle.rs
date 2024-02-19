#![allow(unused_imports)]
use error_stack::{Report, Result, ResultExt};
use text_3d_graphics::prelude::*;
use thiserror::Error;

struct DrawTriangle {
    pub rasterizer: Rasterizer,
    pub triangle: Triangle,
}

impl Behaviour for DrawTriangle {
    fn process(
        &mut self,
        buffer: &mut CharBuffer,
        delta: f32,
    ) -> std::result::Result<ProcessNext, Box<dyn std::error::Error>> {
        buffer.fill(' ', RgbColor(255, 255, 255));
        self.rasterizer.clear_frame();
        self.rasterizer.rasterize_triangle(self.triangle, buffer)?;

        self.triangle.v = self.triangle.v.map(|v| v + vec3a(0.0,0.0,0.1 * delta));
        self.rasterizer.camera.rotate_z_radians(10.0f32.to_radians() * delta);

        Ok(ProcessNext::Continue)
    }
}

#[derive(Debug, Error)]
#[error("there was a error encountered in the main fn")]
struct MainError;

fn main() -> Result<(), MainError> {
    let rasterizer = Rasterizer {
        camera: Camera::builder()
            .position(vec3a(0.0, 0.0, 0.0))
            .fov_y_radians(90.0f32.to_radians())
            .aspect_ratio(1.0)
            .build(),
        ..Rasterizer::default()
    };
    let triangle = Triangle {
        v: [
            vec3a(-1.0, -1.0, 1.0),
            vec3a(1.0, -1.0, 1.0),
            vec3a(1.0, 1.0, 1.0),
        ],
        color: RgbColor(255, 100, 000),
    };
    let mut my_runner = Runner::builder()
        .fps(10.0)
        .dimensions(uvec2(30, 20))
        .color(RgbColor(255, 0, 255))
        .character('.')
        .behaviour(DrawTriangle {
            rasterizer,
            triangle,
        })
        .build();
    my_runner.run().unwrap();
    Ok(())
}
