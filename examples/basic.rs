use error_stack::Result;
use error_stack::ResultExt;
use text_3d_graphics::prelude::*;
use text_3d_graphics::printing::CharBuffer;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Error in Main fn")]
struct MainError;

fn main() -> Result<(), MainError> {
    let mut cb = CharBuffer::new(uvec2(5, 5), '+', RgbColor(255, 255, 255))
        .change_context_lazy(|| MainError)?;
    cb.set_char(uvec2(0, 2), Some('@'), Some(RgbColor(255, 100, 000))).change_context_lazy(|| MainError)?;
    cb.set_char(uvec2(3, 1), Some('p'), Some(RgbColor(255, 255, 000))).change_context_lazy(|| MainError)?;
    cb.set_char(uvec2(4, 3), Some('O'), Some(RgbColor(255, 100, 222))).change_context_lazy(|| MainError)?;
    cb.set_char(uvec2(3, 3), Some('.'), Some(RgbColor(200, 000, 150))).change_context_lazy(|| MainError)?;
    println!("{cb}");
    Ok(())
}
