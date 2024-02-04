pub(crate) use super::*;

pub mod drawing;

#[derive(Clone, Getters, MutGetters, Setters)]
pub struct CharBuffer {
    #[getset(get="pub")]
    value: Vec<Vec<char>>, // Vec < Row >
    #[getset(get="pub")]
    colors: Vec<Vec<RgbColor>>,

    #[getset(get="pub")]
    dimensions: UVec2, // Rows, Columns
}

#[derive(Debug, Error)]
pub enum CharBufferError {
    #[error("Could not create CharBuffer from provided str {0}")]
    InvalidConstructorString(String),
    #[error("Out of bounds access. Attempted Position {attempt} in a {dimensions} dimensioned CharBuffer")]
    OutOfBounds { attempt: UVec2, dimensions: UVec2 },
    #[error("Failed to convert usize to u32 or vice versa")]
    UsizeConversion,
}

impl CharBuffer {
    pub fn set_char(
        &mut self,
        position: UVec2,
        char: Option<char>,
        color: Option<RgbColor>,
    ) -> Result<(), CharBufferError> {
        //! Sets character and/or color at position specified, will return error if value is out of
        //! range.
        //! The '.' char is reserved for a filled in character, so don't use it unintentionally.
        //! ```
        //! pub use text_3d_graphics::printing::CharBuffer;
        //! pub use text_3d_graphics::prelude::*;
        //! let mut cb = CharBuffer::from_str("   \n @ \n  +\n",RgbColor(255,255,255)).unrwap();
        //! cb.set_char(uvec2(2,1),Some('*'),None);
        //! assert_eq!("   \n @ \n *+\n",cb.to_string());
        //! ```
        let report = || {
            Report::new(CharBufferError::OutOfBounds {
                attempt: position,
                dimensions: self.dimensions,
            })
        };

        let x = TryInto::<usize>::try_into(position.x)
            .change_context_lazy(|| CharBufferError::UsizeConversion)
            .attach_printable_lazy(|| "u32 -> usize")?;
        let y = TryInto::<usize>::try_into(position.y)
            .change_context_lazy(|| CharBufferError::UsizeConversion)
            .attach_printable_lazy(|| "u32 -> usize")?;

        if let Some(char) = char {
            *(self
                .value
                .get_mut(y)
                .ok_or(report())?
                .get_mut(x)
                .ok_or(report())?) = char;
        }
        if let Some(color) = color {
            (*self
                .colors
                .get_mut(y)
                .ok_or(report())?
                .get_mut(x)
                .ok_or(report())?) = color
        }

        Ok(())
    }
    pub fn fill(&mut self, char: char, color: RgbColor) {
        self.value
            .iter_mut()
            .map(|v| v.iter_mut())
            .flatten()
            .zip(self.colors.iter_mut().map(|v| v.iter_mut()).flatten())
            .for_each(|(ch, co)| {
                *ch = char;
                *co = color;
            });
    }
    pub fn to_string(&self) -> String {
        let reset = format!("{}", anstyle::Reset);
        self.value
            .iter()
            .zip(self.colors.iter())
            .map(|(row, colors)| {
                let mut val = row
                    .iter()
                    .zip(colors.iter())
                    .map(|(char, color)| {
                        let style = anstyle::Style::new()
                            .fg_color(Some(anstyle::Color::Rgb(*color)))
                            .bg_color(if *char == '.' {
                                Some(anstyle::Color::Rgb(*color))
                            } else {
                                None
                            });
                        format!("{}{}{}{}", reset, style, char, char)
                    })
                    .collect::<String>();
                val.push_str(&reset);
                val.push('\n');
                val
            })
            .collect::<String>()
    }
    pub fn new(dimensions: UVec2, char: char, color: RgbColor) -> Result<Self, CharBufferError> {
        Ok(Self {
            value: vec![
                vec![
                    char;
                    TryInto::<usize>::try_into(dimensions.y)
                        .change_context_lazy(|| CharBufferError::UsizeConversion)
                        .attach_printable_lazy(|| "u32 -> usize")?
                ];
                TryInto::<usize>::try_into(dimensions.x)
                    .change_context_lazy(|| CharBufferError::UsizeConversion)
                    .attach_printable_lazy(|| "u32 -> usize")?
            ],
            colors: vec![
                vec![
                    color;
                    TryInto::<usize>::try_into(dimensions.y)
                        .change_context_lazy(|| CharBufferError::UsizeConversion)
                        .attach_printable_lazy(|| "u32 -> usize")?
                ];
                TryInto::<usize>::try_into(dimensions.x)
                    .change_context_lazy(|| CharBufferError::UsizeConversion)
                    .attach_printable_lazy(|| "u32 -> usize")?
            ],
            dimensions,
        })
    }
    pub fn get_char(&self, pos: UVec2) -> Option<(char, RgbColor)> {
        if self.is_valid_point(ivec2(pos.x as i32, pos.y as i32)) {
            return Some((
                *self.value.get(pos.y as usize).unwrap().get(pos.x as usize).unwrap(),
                *self.colors.get(pos.y as usize).unwrap().get(pos.x as usize).unwrap()
            ));
        }
        None
    }
    pub fn set_dimensions(&mut self, dimensions: UVec2, character: char, color: RgbColor) {
        self.value.resize(dimensions.x as usize, vec![character; dimensions.y as usize]);
        self.value.iter_mut().for_each(|v| v.resize(dimensions.y as usize, character));
        self.colors.resize(dimensions.x as usize, vec![color; dimensions.y as usize]);
        self.colors.iter_mut().for_each(|v| v.resize(dimensions.y as usize, color));
    }
}

impl std::fmt::Display for CharBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Debug for CharBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Charbuffer")
            .field("Dimensions", &self.dimensions)
            .field("Buffer", &self.to_string())
            .finish()
    }
}
