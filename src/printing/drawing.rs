pub(crate) use super::*;

#[derive(Debug, Error)]
pub enum DrawError {
    #[error("failed to draw line {0},{1}")]
    Line(IVec2, IVec2),
    #[error("failed to draw triangle {0:?}")]
    Triangle([IVec2; 3]),
}

impl CharBuffer {
    pub fn draw_triangle(
        &mut self,
        mut verticies: [IVec2; 3],
        shading: fn(pos: IVec2, buf: &CharBuffer) -> (Option<char>, Option<RgbColor>),
    ) -> Result<(), DrawError> {
        verticies.sort_by(|a, b| a.y.cmp(&b.y));

        let top_triangle = verticies[2].y != verticies[1].y;
        let bottom_triangle = verticies[0].y != verticies[1].y;

        if !top_triangle && !bottom_triangle {
            return Ok(());
        }

        let delta = verticies[2] - verticies[0];

        // The point on the edge of the triangle opposite to the middle vertex with the same value
        // as the middle vertex. A line between this vertex and the middle vertex would be
        // horizontal and cut the triangle in half.
        let midpoint = ivec2(
            verticies[0].x + delta.x * (verticies[1].y - verticies[0].y) / delta.y,
            verticies[1].y,
        );

        if top_triangle {
            self.draw_top_triangle([midpoint, verticies[1], verticies[2]], shading)?;
        }
        if bottom_triangle {
            self.draw_bottom_triangle([verticies[0], verticies[1], midpoint], shading)?;
        }

        Ok(())
    }
    fn draw_top_triangle(
        &mut self,
        mut verticies: [IVec2; 3],
        shading: fn(pos: IVec2, buf: &CharBuffer) -> (Option<char>, Option<RgbColor>),
    ) -> Result<(), DrawError> {
        let delta = (
            verticies[2] - verticies[1],
            verticies[2] - verticies[0],
        );
        let row_edges = |y: i32| -> (i32, i32) {
            let a = verticies[1].x + delta.0.x * (y - verticies[1].y) / delta.0.y;
            let b = verticies[0].x + delta.1.x * (y - verticies[0].y) / delta.1.y;

            (std::cmp::min(a, b), std::cmp::max(a, b))
        };
        for ver in (verticies[0].y..=verticies[2].y)
            .map(|y| {
                let edges = row_edges(y);
                (edges.0..=edges.1).map(move |x| ivec2(x, y))
            })
            .flatten()
        {
            let shade = shading(ver, &self);
            if self.is_valid_point(ver) {
                self.set_char(uvec2(ver.x as u32, ver.y as u32), shade.0, shade.1)
                    .change_context_lazy(|| DrawError::Triangle(verticies))
                    .attach_printable_lazy(|| format!("Failed at point: {ver}"))?;
            }
        }
        Ok(())
    }
    fn draw_bottom_triangle(
        &mut self,
        mut verticies: [IVec2; 3],
        shading: fn(pos: IVec2, buf: &CharBuffer) -> (Option<char>, Option<RgbColor>),
    ) -> Result<(), DrawError> {
        let delta = (
            verticies[0] - verticies[1],
            verticies[0] - verticies[2],
        );
        let row_edges = |y: i32| -> (i32, i32) {
            let a = verticies[1].x + delta.0.x * (y - verticies[1].y) / delta.0.y;
            let b = verticies[2].x + delta.1.x * (y - verticies[2].y) / delta.1.y;

            (std::cmp::min(a, b), std::cmp::max(a, b))
        };
        for ver in (verticies[0].y..=verticies[2].y)
            .map(|y| {
                let edges = row_edges(y);
                (edges.0..=edges.1).map(move |x| ivec2(x, y))
            })
            .flatten()
        {
            let shade = shading(ver, &self);
            if self.is_valid_point(ver) {
                self.set_char(uvec2(ver.x as u32, ver.y as u32), shade.0, shade.1)
                    .change_context_lazy(|| DrawError::Triangle(verticies))
                    .attach_printable_lazy(|| format!("Failed at point: {ver}"))?;
            }
        }
        Ok(())
    }
    pub fn draw_line(
        &mut self,
        mut start_point: IVec2,
        mut end_point: IVec2,
        shading: fn(pos: IVec2, buf: &CharBuffer) -> (Option<char>, Option<RgbColor>),
    ) -> Result<(), DrawError> {
        if start_point.x == end_point.x {
            return self.draw_vertical_line(start_point, end_point, shading);
        } else if start_point.x > end_point.x {
            std::mem::swap(&mut start_point, &mut end_point);
        }

        let dif_vec = end_point - start_point;
        let slope = dif_vec.y as f32 / dif_vec.x as f32;

        if slope.abs() > 1.0 {
            self.draw_steep_line(start_point, end_point, slope, shading)?;
            return Ok(());
        }

        self.draw_shallow_line(start_point, end_point, slope, shading)?;

        Ok(())
    }
    fn draw_vertical_line(
        &mut self,
        mut start_point: IVec2,
        mut end_point: IVec2,
        shading: fn(pos: IVec2, buf: &CharBuffer) -> (Option<char>, Option<RgbColor>),
    ) -> Result<(), DrawError> {
        if start_point.y == end_point.y {
            if self.is_valid_point(start_point) {
                let (character, color) = shading(ivec2(start_point.x, start_point.y), &self);
                return self
                    .set_char(
                        uvec2(start_point.x as u32, start_point.y as u32),
                        character,
                        color,
                    )
                    .change_context_lazy(|| DrawError::Line(start_point, end_point));
            }
            return Ok(());
        }
        if start_point.y > end_point.y {
            std::mem::swap(&mut start_point, &mut end_point);
        }

        let x = start_point.x;

        (std::cmp::max(0, start_point.y)..=std::cmp::min(end_point.y, self.dimensions.y as i32 - 1))
            .map(|y| {
                let (character, color) = shading(ivec2(x, y), &self);
                self.set_char(uvec2(x as u32, y as u32), character, color)
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
        shading: fn(pos: IVec2, buf: &CharBuffer) -> (Option<char>, Option<RgbColor>),
    ) -> Result<(), DrawError> {
        if start_point.y > end_point.y {
            std::mem::swap(&mut start_point, &mut end_point)
        }
        (start_point.y..=end_point.y)
            .map(|y| {
                let x = ((y - start_point.y) as f32 * 1.0 / slope + start_point.x as f32) as i32;
                if !self.is_valid_point(ivec2(x, y)) {
                    return Ok(());
                }
                let (character, color) = shading(ivec2(x, y), &self);
                self.set_char(uvec2(x as u32, y as u32), character, color)
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
        start_point: IVec2,
        end_point: IVec2,
        slope: f32,
        shading: fn(pos: IVec2, buf: &CharBuffer) -> (Option<char>, Option<RgbColor>),
    ) -> Result<(), DrawError> {
        (start_point.x..=end_point.x)
            .map(|x| {
                let y = ((x - start_point.x) as f32 * slope + start_point.y as f32) as i32;
                if !self.is_valid_point(ivec2(x, y)) {
                    return Ok(());
                }
                let (character, color) = shading(ivec2(x, y), &self);
                self.set_char(uvec2(x as u32, y as u32), character, color)
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
