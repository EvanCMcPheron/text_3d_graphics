#![allow(unused_imports)]
use error_stack::{Report, Result, ResultExt};
use text_3d_graphics::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("there was a error encountered in the main fn")]
struct MainError;

fn main() -> Result<(), MainError> {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut raster = Rasterizer {
        camera: Camera::builder()
            .aspect_ratio(1.0)
            .fov_y_radians((90.0f32).to_radians())
            .position(vec3(0.0, 0.0, 0.0))
            .build(),
        ..Rasterizer::default()
    };
    for i in -10..=10 {
        let point = vec3a(i as f32 / 10.0, 1.0, 2.0);
        println!("{point}->{}",raster.project_point(point));
    }
    Ok(())
}
