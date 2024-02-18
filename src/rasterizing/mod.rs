pub(crate) use super::*;
use crate::prelude::*;

#[derive(Debug, Error)]
pub enum RasterizationError {
    #[error("unidentified rasterization error")]
    Misc,
}

#[derive(Debug, Clone)]
pub struct Rasterizer {
    pub camera: Camera,
    pub world_tensor: Mat4,
}

#[derive(Debug, Clone)]
pub struct Camera {
    view_tensor: Option<Mat4>,
    perspective_tesnor: Option<Mat4>,
    up_dir: Vec3,
    look_dir: Vec3,
    position: Vec3,
    fov_y_radians: f32,
    aspect_ratio: f32,
    z_near: f32,
}

impl Rasterizer {
    pub fn project_point(&mut self, rhs: Vec3A) -> Vec3 {
        self.camera
            .project_point(self.world_tensor.transform_point3a(rhs))
    }
}

impl Camera {
    pub fn project_point(&mut self, rhs: Vec3A) -> Vec3 {
        if let None = self.perspective_tesnor {
            self.perspective_tesnor = Some(Mat4::perspective_infinite_rh(
                self.fov_y_radians,
                self.aspect_ratio,
                self.z_near,
            ));
        }
        if let None = self.view_tensor {
            self.view_tensor =
                Some(Mat4::look_to_rh(self.position, self.look_dir, self.up_dir));
        }

        self.perspective_tesnor
            .unwrap()
            .project_point3(self.view_tensor.unwrap().transform_point3a(rhs).into())
    }
}

impl std::default::Default for Rasterizer {
    fn default() -> Self {
        Self {
            camera: Camera::builder()
                .position(Vec3::ZERO)
                .fov_y_radians((90.0f32).to_degrees())
                .aspect_ratio(1.0)
                .build(),
            world_tensor: Mat4::IDENTITY,
        }
    }
}

#[buildstructor]
impl Camera {
    #[builder(visibility = "pub")]
    pub fn new(
        up_dir: Option<Vec3>,
        look_dir: Option<Vec3>,
        position: Vec3,
        fov_y_radians: f32,
        aspect_ratio: f32,
        z_near: Option<f32>,
    ) -> Self {
        Self {
            view_tensor: None,
            perspective_tesnor: None,
            up_dir: up_dir.unwrap_or(vec3(0.0, 1.0, 0.0)),
            look_dir: look_dir.unwrap_or(vec3(0.0, 0.0, 1.0)),
            position,
            fov_y_radians,
            aspect_ratio,
            z_near: z_near.unwrap_or(1.0),
        }
    }
    fn generate_view(&mut self) {
        self.view_tensor = Some(Mat4::look_to_rh(self.position, self.look_dir, self.up_dir));
    }
    fn generate_perspective(&mut self) {
        self.perspective_tesnor = Some(Mat4::perspective_infinite_rh(
            self.fov_y_radians,
            self.aspect_ratio,
            self.z_near,
        ));
    }
    pub fn up_dir(&self) -> &Vec3 {
        &self.up_dir
    }
    pub fn up_dir_mut(&mut self) -> &Vec3 {
        self.view_tensor = None;
        &mut self.up_dir
    }
    pub fn look_dir(&self) -> &Vec3 {
        &self.look_dir
    }
    pub fn look_dir_mut(&mut self) -> &Vec3 {
        self.view_tensor = None;
        &mut self.look_dir
    }
    pub fn position(&self) -> &Vec3 {
        &self.position
    }
    pub fn position_mut(&mut self) -> &mut Vec3 {
        self.view_tensor = None;
        &mut self.position
    }
    pub fn fov_y_radians(&self) -> &f32 {
        &self.fov_y_radians
    }
    pub fn fov_y_radians_mut(&mut self) -> &mut f32 {
        self.perspective_tesnor = None;
        &mut self.fov_y_radians
    }
    pub fn aspect_ratio(&self) -> &f32 {
        &self.aspect_ratio
    }
    pub fn aspect_ratio_mut(&mut self) -> &mut f32 {
        self.perspective_tesnor = None;
        &mut self.aspect_ratio
    }
    pub fn z_near(&self) -> &f32 {
        &self.z_near
    }
    pub fn z_near_mut(&mut self) -> &mut f32 {
        self.perspective_tesnor = None;
        &mut self.z_near
    }
    pub fn rotate_self(&mut self, rotate: Quat) {
        self.look_dir = rotate * self.look_dir;
        self.up_dir = rotate * self.up_dir;
    }
}
