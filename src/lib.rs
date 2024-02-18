#![allow(dead_code, unused_imports)]

pub mod behaviour;
pub mod printing;
pub mod rasterizing;
pub mod resources;

pub mod prelude {
    pub use {
        super::{
            behaviour::{Behaviour, ProcessNext, Runner},
            printing::CharBuffer,
            resources::{SimpleMesh, Triangle},
            rasterizing::{Camera,Rasterizer},
        },
        anstyle::{Color, RgbColor, Style},
        glam::{
            f32::{mat4, quat, vec2, vec3, vec3a, vec4, Affine3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4},
            i32::{ivec2, IVec2},
            u32::{uvec2, UVec2},
        },
    };
}

pub(crate) use {
    anstyle::{Color, RgbColor, Style},
    crossterm::{
        cursor, execute,
        style::Print,
        terminal::{BeginSynchronizedUpdate, EndSynchronizedUpdate},
    },
    error_stack::{Context, Report, Result, ResultExt},
    getset::{CopyGetters, Getters, MutGetters, Setters},
    glam::{
        f32::{mat4, quat, vec2, vec3, vec3a, vec4, Affine3A, Mat4, Quat, Vec2, Vec3, Vec3A, Vec4},
        i32::{ivec2, IVec2},
        u32::{uvec2, UVec2},
    },
    itertools::Itertools,
    std::sync::Arc,
    std::rc::Rc,
    thiserror::Error,
    buildstructor::buildstructor,
};
