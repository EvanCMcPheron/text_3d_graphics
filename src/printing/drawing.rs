pub(crate) use super::*;

#[derive(Debug, Error)]
pub enum DrawError {
    #[error("failed to draw line {0},{1}")]
    Line(IVec2, IVec2),
}

impl CharBuffer {
    #[inline]
    pub fn is_valid_point(&self, point: IVec2) -> bool {
        (!point.x.is_negative() && !point.y.is_negative())
            && (point.x < self.dimensions.x as i32 && point.y < self.dimensions.y as i32)
    }
    pub fn draw_line(
        &mut self,
        mut start_point: IVec2,
        mut end_point: IVec2,
        character: char,
        color: RgbColor,
    ) -> Result<(), DrawError> {
        if start_point.x == end_point.x {
            return self.draw_vertical_line(start_point, end_point, character, color);
        } else if start_point.x > end_point.x {
            std::mem::swap(&mut start_point, &mut end_point);
        }

        let dif_vec = end_point - start_point;
        let slope = dif_vec.y as f32 / dif_vec.x as f32;

        if slope.abs() > 1.0 {
            self.draw_steep_line(start_point, end_point, slope, character, color)?;
            return Ok(());
        }

        self.draw_shallow_line(start_point, end_point, slope, character, color)?;

        Ok(())
    }
    fn draw_vertical_line(
        &mut self,
        mut start_point: IVec2,
        mut end_point: IVec2,
        character: char,
        color: RgbColor,
    ) -> Result<(), DrawError> {
        if start_point.y > end_point.y {
            std::mem::swap(&mut start_point, &mut end_point);
        }

        let x = start_point.x as u32;

        (std::cmp::max(0, start_point.y)..=std::cmp::min(end_point.y, self.dimensions.y as i32 - 1))
            .map(|v| v as u32)
            .map(|y| {
                self.set_char(uvec2(x, y), Some(character), Some(color))
                    .change_context_lazy(|| DrawError::Line(start_point, end_point))
                    .attach_printable_lazy(|| format!("Failed to print char at {x},{y}"))
            })
            .fold(Ok(()), |accum, r| {
                if r.is_err() && !accum.is_err() {
                    r
                } else {
                    accum
                }
            })
    }
    fn draw_steep_line(
        &mut self,
        mut start_point: IVec2,
        mut end_point: IVec2,
        slope: f32,
        character: char,
        color: RgbColor,
    ) -> Result<(), DrawError> {
        if start_point.y > end_point.y {
            std::mem::swap(&mut start_point, &mut end_point)
        }
        (start_point.y..=end_point.y)
            .map(|y| {
                let x = ((y - start_point.y) as f32 * 1.0/slope + start_point.x as f32) as i32;
                if !self.is_valid_point(ivec2(x, y)) {
                    return Ok(());
                }
                self.set_char(uvec2(x as u32, y as u32), Some(character), Some(color))
                    .change_context_lazy(|| DrawError::Line(start_point, end_point))
                    .attach_printable_lazy(|| format!("failed at point {x},{y}"))
            })
            .fold(Ok(()), |accum, r| {
                if r.is_err() && accum.is_ok() {
                    r
                } else {
                    accum
                }
            })
    }
    fn draw_shallow_line(
        &mut self,
        mut start_point: IVec2,
        mut end_point: IVec2,
        slope: f32,
        character: char,
        color: RgbColor,
    ) -> Result<(), DrawError> {
        (start_point.x..=end_point.x)
            .map(|x| {
                let y = ((x - start_point.x) as f32 * slope + start_point.y as f32) as i32;
                if !self.is_valid_point(ivec2(x, y)) {
                    return Ok(());
                }
                self.set_char(uvec2(x as u32, y as u32), Some(character), Some(color))
                    .change_context_lazy(|| DrawError::Line(start_point, end_point))
                    .attach_printable_lazy(|| format!("failed at point {x},{y}"))
            })
            .fold(Ok(()), |accum, r| {
                if r.is_err() && accum.is_ok() {
                    r
                } else {
                    accum
                }
            })
    }
}

