#![allow(unused_imports)]
use error_stack::{Report, Result, ResultExt};
use text_3d_graphics::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("there was a error encountered in the main fn")]
struct MainError;

fn main() -> Result<(), MainError> {
    let mut cb = CharBuffer::new(uvec2(50, 50), ' ', RgbColor(0, 0, 0)).unwrap();
    cb.draw_triangle([ivec2(5, 2), ivec2(44, 30), ivec2(20, 45)], |p, _| {
        (
            Some('.'),
            Some(RgbColor(200, (p.y * 5) as u8, (p.x * 5) as u8)),
        )
    })
    .change_context_lazy(|| MainError)?;
    println!("{cb}");
    Ok(())
}
