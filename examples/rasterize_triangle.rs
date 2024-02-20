#![allow(unused_imports)]
use error_stack::{Report, Result, ResultExt};
use text_3d_graphics::prelude::*;
use thiserror::Error;

struct DrawTriangle {
    pub rasterizer: Rasterizer,
    pub triangles: Vec<Triangle>,
    pub elapsed: f32,
}

impl Behaviour for DrawTriangle {
    fn process(
        &mut self,
        buffer: &mut CharBuffer,
        delta: f32,
    ) -> std::result::Result<ProcessNext, Box<dyn std::error::Error>> {
        buffer.fill('.', RgbColor(000, 000, 000));
        self.rasterizer.clear_frame();
        self.elapsed += delta;

        for triangle in &self.triangles {
            self.rasterizer.rasterize_triangle(*triangle, buffer)?;
        }

        // self.triangle.v = self.triangle.v.map(|v| v + vec3a(0.0,0.0,0.3 * delta));
        
        let rot = -15.0f32.to_radians() * delta;
        self.rasterizer.camera.rotate_y_radians(rot);

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
            vec3a(-1.0, -1.0, 2.0),
            vec3a(1.0, -1.0, 2.0),
            vec3a(1.0, 1.0, 2.0),
        ],
        color: RgbColor(255, 100, 200),
    };
    let triangle2 = Triangle {
        v: [
            vec3a(1.0, -1.0, -2.0),
            vec3a(-1.0, -1.0, -2.0),
            vec3a(1.0, 1.0, -2.0),
        ],
        color: RgbColor(111, 222, 200),
    };
    let mut my_runner = Runner::builder()
        .fps(10.0)
        .dimensions(uvec2(50, 50))
        .color(RgbColor(000, 000, 000))
        .character('.')
        .behaviour(DrawTriangle {
            rasterizer,
            triangles: vec![triangle, triangle2],
            elapsed: 0.0,
        })
        .build();
    my_runner.run().unwrap();
    Ok(())
}
