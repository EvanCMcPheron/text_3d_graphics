#![allow(unused_imports)]
use error_stack::{
    Report,
    ResultExt,
    Result
};
use text_3d_graphics::prelude::*;
use thiserror::Error;

struct RotatingLine {
    pub theta: f32,
    pub len: f32,
    pub ang_vel: f32,
}

impl Behaviour for RotatingLine {
    fn process(
        &mut self,
        buffer: &mut CharBuffer,
        delta: f32,
    ) -> std::result::Result<ProcessNext, Box<dyn std::error::Error>> {
        self.theta += self.ang_vel * delta;

        let dims = buffer.dimensions().clone();
        let center = vec2(dims.x as f32 / 2.0, dims.y as f32 / 2.0);
        let terminal = Vec2::from_angle(self.theta) * self.len + center;

        buffer.fill(' ', RgbColor(255, 255, 255));
        buffer.draw_line(
            ivec2(dims.x as i32 - 1, dims.y as i32 - 1), 
            ivec2(0, dims.y as i32 - 1), 
            |_, _| (Some('.'), Some(RgbColor(255, 0, 0)))
        )?;
        buffer.draw_line(
            ivec2(0, dims.y as i32 - 1), 
            ivec2(0, 0), 
            |_, _| (Some('.'), Some(RgbColor(255, 0, 0)))
        )?;
        buffer.draw_line(
            ivec2(dims.x as i32 - 1, 0), 
            ivec2(dims.x as i32 - 1, dims.y as i32 - 1), 
            |_, _| (Some('.'), Some(RgbColor(255, 0, 0)))
        )?;
        buffer.draw_line(
            ivec2(0, 0), 
            ivec2(dims.x as i32 - 1, 0), 
            |_, _| (Some('.'), Some(RgbColor(255, 0, 0)))
        )?;
        buffer.draw_line(
            ivec2(center.x as i32, center.y as i32),
            ivec2(terminal.x as i32, terminal.y as i32),
            |_, _| (Some('.'), Some(RgbColor(255, 255, 255))),
        )?;

        Ok(ProcessNext::Continue)
    }
}

#[derive(Debug, Error)]
#[error("Error in Main fn")]
struct MainError;

fn main() -> Result<(), MainError> {
    let mut runner = Runner::builder()
        .behaviour(RotatingLine{
            theta: 0.0,
            len: 45.0,
            ang_vel: 1.5,
        })
        .dimensions(uvec2(100, 100))
        .build();
    runner.run();
    Ok(())
}
