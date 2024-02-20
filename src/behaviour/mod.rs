pub(crate) use super::*;
use crate::prelude::*;
use std::time::{Duration, Instant};

pub enum ProcessNext {
    Continue,
    End,
}

pub trait Behaviour {
    fn process(
        &mut self,
        buffer: &mut CharBuffer,
        delta: f32,
    ) -> std::result::Result<ProcessNext, Box<dyn std::error::Error>>;
}

pub struct Runner<B: Behaviour> {
    behaviour: B,
    pub buffer: CharBuffer,
    last_frame: Option<Instant>,
    spf: f32,
}

#[buildstructor::buildstructor]
impl<B: Behaviour> Runner<B> {
    #[builder]
    pub fn new(
        behaviour: B,
        dimensions: UVec2,
        fps: Option<f32>,
        character: Option<char>,
        color: Option<RgbColor>,
    ) -> Self {
        Self {
            behaviour,
            buffer: CharBuffer::new(
                dimensions,
                character.unwrap_or(' '),
                color.unwrap_or(RgbColor(255, 255, 255)),
            )
            .unwrap_or_else(|e| panic!("{e}")),
            last_frame: None,
            spf: 1.0 / fps.unwrap_or(10.0),
        }
    }
}

impl<B: Behaviour> Runner<B> {
    #[inline]
    pub fn to_string(&self) -> String {
        format!("{}", self.buffer)
    }
    pub fn run_frame(
        &mut self,
        delta: f32,
    ) -> std::result::Result<ProcessNext, Box<dyn std::error::Error>> {
        self.behaviour.process(&mut self.buffer, delta)
    }
    pub fn run(&mut self) -> std::result::Result<(), Box<dyn std::error::Error>> {
        execute!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::Purge),
            cursor::MoveTo(0,0),
            cursor::Hide,
            cursor::SavePosition,
        )?;
        for i in 0.. {
            let delta = if let Some(inst) = self.last_frame {
                let elapsed = inst.elapsed().as_secs_f32();
                let d = self.spf - elapsed;
                if d.is_sign_positive() {
                    std::thread::sleep(Duration::from_secs_f32(d));
                }
                inst.elapsed().as_secs_f32()
            } else {
                self.spf
            };
            self.last_frame = Some(Instant::now());
            if let ProcessNext::End = self.run_frame(delta)? {
                break;
            }
            execute!(
                std::io::stdout(),
                BeginSynchronizedUpdate,
                cursor::RestorePosition,
                Print(self.to_string()),
                EndSynchronizedUpdate
            )?;
        }
        Ok(())
    }
}

impl<B: Behaviour> std::fmt::Display for Runner<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buffer)
    }
}
