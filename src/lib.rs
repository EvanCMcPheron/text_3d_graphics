#![allow(dead_code, unused_imports)]

pub mod behaviour;
pub mod printing;
pub mod rendering;
pub mod resources;

pub mod prelude {
    pub use {
        super::{
            behaviour::{Behaviour, ProcessNext, Runner},
            printing::CharBuffer,
        },
        anstyle::{Color, RgbColor, Style},
        glam::{
            f32::{quat, vec2, vec3a, Quat, Vec2, Vec3A},
            i32::{ivec2, IVec2},
            u32::{uvec2, UVec2},
        },
    };
}

pub(crate) use {
    anstyle::{Color, RgbColor, Style},
    crossterm::{
        cursor, execute,style::Print,
        terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate},
    },
    error_stack::{Context, Report, Result, ResultExt},
    getset::{CopyGetters, Getters, MutGetters, Setters},
    glam::{
        f32::{quat, vec2, vec3a, Quat, Vec2, Vec3A},
        i32::{ivec2, IVec2},
        u32::{uvec2, UVec2},
    },
    itertools::Itertools,
    thiserror::Error,
};
