#![allow(unused_imports)]
pub(crate) use super::*;
use crate::prelude::*;
pub(crate) use tobj::{load_obj, Material, Mesh};

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error("failed to load obj file")]
    ObjLoadError,
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub v: [Vec3A; 3],
    pub color: RgbColor,
}

#[derive(Debug, Clone)]
pub struct SimpleMesh {
    pub triangles: Rc<[Triangle]>,
}

impl Triangle {
    pub fn normal(&self) -> Vec3A {
        (self.v[2] - self.v[0])
            .cross(self.v[1] - self.v[0])
            .normalize()
    }
}

impl From<(Mesh, &[Material])> for SimpleMesh {
    fn from(value: (Mesh, &[Material])) -> Self {
        Self {
            triangles: Rc::from_iter((0..value.0.indices.len() / 3).map(|i| i * 3).map(|i| {
                let get_vertex = |triangle_index: usize| {
                    let v1_i = value.0.indices[triangle_index] as usize;
                    vec3a(
                        value.0.positions[v1_i],
                        value.0.positions[v1_i + 1],
                        value.0.positions[v1_i + 2],
                    )
                };
                Triangle {
                    color: match value.0.material_id {
                        Some(mat_id) => {
                            let c = value.1[mat_id].diffuse.unwrap_or([1.0, 1.0, 1.0]);
                            RgbColor(
                                (255.0 * c[0]) as u8,
                                (255.0 * c[1]) as u8,
                                (255.0 * c[2]) as u8,
                            )
                        }
                        None => RgbColor(255, 255, 255),
                    },
                    v: [get_vertex(i), get_vertex(i + 1), get_vertex(i + 2)],
                }
            })),
        }
    }
}

impl From<Mesh> for SimpleMesh {
    fn from(value: Mesh) -> Self {
        Self {
            triangles: Rc::from_iter((0..value.indices.len() / 3).map(|i| i * 3).map(|i| {
                let get_vertex = |triangle_index: usize| {
                    let v1_i = value.indices[triangle_index] as usize;
                    vec3a(
                        value.positions[v1_i],
                        value.positions[v1_i + 1],
                        value.positions[v1_i + 2],
                    )
                };
                Triangle {
                    color: RgbColor(255, 255, 255),
                    v: [get_vertex(i), get_vertex(i + 1), get_vertex(i + 2)],
                }
            })),
        }
    }
}
