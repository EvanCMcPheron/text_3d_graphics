pub(crate) use super::*;

pub struct CharBuffer {
    value: Vec<String>, // Vec < Row >
    dimensions: UVec2,  // Rows, Columns
}

#[derive(Debug, Error)]
pub enum CharBufferError {
    #[error("Could not create CharBuffer from provided str {0}")]
    InvalidConstructorString(String),
    #[error("Out of bounds access. Attempted Position {attempt} in a {dimensions} dimensioned CharBuffer")]
    OutOfBounds { attempt: UVec2, dimensions: UVec2 },
}

impl CharBuffer {
    pub fn from_str(string: &str) -> Result<Self, CharBufferError> {
        let value: Vec<_> = string.split('\n').map(|s| s.to_owned()).collect();

        let unique_row_lengths = value.iter().map(|v| v.len()).dedup().count();

        if unique_row_lengths != 1 {
            return Err(Report::new(CharBufferError::InvalidConstructorString(
                string.to_owned(),
            )))
            .attach_printable_lazy(|| {
                format!(
                    "There were {unique_row_lengths} unique row lengths when there should be one"
                )
            });
        }

        let dimensions = uvec2(
            value
                .len()
                .try_into()
                .expect("Couldn't convert usize to u32"),
            value
                .first()
                .ok_or(Report::new(CharBufferError::InvalidConstructorString(
                    string.to_owned(),
                )))
                .attach_printable_lazy(|| "the string had no rows, it's probably empty")?
                .len()
                .try_into()
                .expect("Couldn't convert usize to u32"),
        );

        Ok(Self { value, dimensions })
    }
    pub fn set_char(&mut self, position: UVec2, char: char) -> Result<(), CharBufferError> {
        if position.x >= self.dimensions.x || position.y >= self.dimensions.y {
            return Err(Report::new(CharBufferError::OutOfBounds {
                attempt: position,
                dimensions: self.dimensions,
            }));
        }

        let x = TryInto::<usize>::try_into(position.x).expect("Couldn't convert u32 to usize");
        let y = TryInto::<usize>::try_into(position.y).expect("Couldn't convert u32 to usize");

        self.value
            .get_mut(y)
            .unwrap()
            .replace_range(x..(x + 1), &char.to_string());
        Ok(())
    }
    pub fn fill(&mut self, char: char) {
        for row in self.value.iter_mut() {
            *row = String::from_iter((0..row.len()).map(|_| char))
        }
    }
    pub fn to_string(&self) -> String {
        self.value.iter().map(|row| format!("{}\n", row)).collect()
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
