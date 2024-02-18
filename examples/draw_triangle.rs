#![allow(unused_imports)]
use error_stack::{Report, Result, ResultExt};
use text_3d_graphics::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("there was a error encountered in the main fn")]
struct MainError;

fn main() -> Result<(), MainError> {
    let mut cb = CharBuffer::new(uvec2(20, 20), ' ', RgbColor(0, 0, 0)).unwrap();
    cb.draw_triangle([ivec2(5, 2), ivec2(14, 10), ivec2(10, 15)], |p, _| {
        (
            Some('.'),
            Some(RgbColor(200, (p.y * 15) as u8, (p.x * 14) as u8)),
        )
    })
    .change_context_lazy(|| MainError)?;
    println!("{cb}");
    Ok(())
}
