#![allow(dead_code, unused_imports)]

pub mod behaviour;
pub mod printing;
pub mod rendering;
pub mod resources;

pub(crate) use {
    crossterm::{
        cursor, execute,
        terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate},
    },
    error_stack::{Context, Report, Result, ResultExt},
    glam::{
        f32::{quat, vec2, vec3a, Quat, Vec2, Vec3A},
        i32::{ivec2, IVec2},
        u32::{uvec2, UVec2},
    },
    thiserror::Error,
    itertools::Itertools,
};
