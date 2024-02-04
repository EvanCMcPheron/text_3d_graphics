use error_stack::Result;
use error_stack::ResultExt;
use text_3d_graphics::prelude::*;
use text_3d_graphics::printing::CharBuffer;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Error in Main fn")]
struct MainError;

fn main() -> Result<(), MainError> {
    let mut cb = CharBuffer::new(uvec2(15, 15), ' ', RgbColor(255, 255, 255))
        .change_context_lazy(|| MainError)?;
    cb.draw_line(ivec2(0, 0), ivec2(0, 15), '.', RgbColor(100, 150, 250))
        .change_context_lazy(|| MainError)?;
    cb.draw_line(ivec2(14, 15), ivec2(14, 0), '.', RgbColor(100, 150, 250))
        .change_context_lazy(|| MainError)?;
    cb.draw_line(ivec2(18, 19), ivec2(-1, -2), '.', RgbColor(200, 100, 50))
        .change_context_lazy(|| MainError)?;
    cb.draw_line(ivec2(13, 12), ivec2(13, 12), '.', RgbColor(200, 200, 250))
        .change_context_lazy(|| MainError)?;
    println!("{cb}");
    Ok(())
}
