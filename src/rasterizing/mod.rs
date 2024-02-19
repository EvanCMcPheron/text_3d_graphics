pub(crate) use super::*;
use crate::prelude::*;

#[derive(Debug, Error)]
pub enum RasterizationError {
    #[error("failed to convert a u32 to a usize")]
    U32ToUsize,
    #[error("unidentified rasterization error")]
    Misc,
}

#[derive(Debug, Clone)]
pub struct DepthBuffer {
    data: Vec<Vec<f32>>,
    dimensions: UVec2,
}

#[derive(Debug, Clone)]
pub struct Rasterizer {
    pub camera: Camera,
    pub world_tensor: Mat4,
    pub light_dir: Vec3,
    pub universal_lighting: f32,
    pub depth_buffer: Option<DepthBuffer>,
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

impl DepthBuffer {
    pub fn new(dimensions: UVec2) -> Result<Self, RasterizationError> {
        Ok(Self {
            dimensions,
            data: vec![
                vec![
                    -f32::INFINITY;
                    TryInto::<usize>::try_into(dimensions.x)
                        .change_context_lazy(|| RasterizationError::U32ToUsize)?
                ];
                TryInto::<usize>::try_into(dimensions.y)
                    .change_context_lazy(|| RasterizationError::U32ToUsize)?
            ],
        })
    }
    pub fn clear(&mut self) {
        self.data
            .iter_mut()
            .for_each(|v| v.iter_mut().for_each(|p| *p = -f32::INFINITY));
    }
    pub fn get_value(&self, position: UVec2) -> Option<f32> {
        self.data
            .get(
                TryInto::<usize>::try_into(position.y)
                    .change_context_lazy(|| RasterizationError::U32ToUsize)
                    .unwrap_or_else(|e| panic!("{}", e)),
            )
            .map(|v: &Vec<f32>| {
                v.get(
                    TryInto::<usize>::try_into(position.x)
                        .change_context_lazy(|| RasterizationError::U32ToUsize)
                        .unwrap_or_else(|e| panic!("{}", e)),
                )
            })
            .flatten()
            .copied()
    }
    pub fn set_value(&mut self, position: UVec2, value: f32) {
        let r = self
            .data
            .get_mut(
                TryInto::<usize>::try_into(position.y)
                    .change_context_lazy(|| RasterizationError::U32ToUsize)
                    .unwrap_or_else(|e| panic!("{}", e)),
            )
            .map(|v: &mut Vec<f32>| {
                v.get_mut(
                    TryInto::<usize>::try_into(position.x)
                        .change_context_lazy(|| RasterizationError::U32ToUsize)
                        .unwrap_or_else(|e| panic!("{}", e)),
                )
            })
            .flatten();
        if let Some(p) = r {
            *p = value;
        }
    }
}

impl Rasterizer {
    pub fn clear_frame(&mut self) {
        if let Some(buf) = self.depth_buffer.as_mut() {
            buf.clear();
        }
    }
    pub fn rasterize_triangle(
        &mut self,
        triangle: Triangle,
        char_buffer: &mut CharBuffer,
    ) -> Result<(), RasterizationError> {
        if let None = self.depth_buffer {
            self.depth_buffer = Some(DepthBuffer::new(*char_buffer.dimensions())?);
        }

        // check if triangle needs to be rendered
        // Uses the dot product of the look dirction of the camera and the normal vector of the
        // triangle to determine if the triangle is oriented towards the camera. The vectors
        // should be pointing generally in opposite directions if that is the case, so the dot
        // product should be negative.
        let normal = triangle.normal();

        if self.camera.look_dir.dot(normal.into()) >= 0.0 {
            return Ok(());
        }

        // calculate color after lighting
        let mult = f32::max(normal.dot(self.light_dir.into()), 0.0)
            * (1.0 - self.universal_lighting)
            + self.universal_lighting;
        let color = RgbColor(
            (triangle.color.0 as f32 * mult) as u8,
            (triangle.color.1 as f32 * mult) as u8,
            (triangle.color.2 as f32 * mult) as u8,
        );

        // Project the points
        let projected_verticies: [Vec3; 3] = triangle.v.map(|p| {
            (self.project_point(p) + vec3(1.0, 1.0, 0.0))
                * vec3(
                    (char_buffer.dimensions().x >> 1) as f32,
                    (char_buffer.dimensions().y >> 1) as f32,
                    0.0,
                )
        });

        // get fn for point on screen -> calculated z value based on projected coords
        
        
        // create shader including depth buffer check using closure defined above
        // draw triangle

        Ok(())
    }
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
            self.view_tensor = Some(Mat4::look_to_rh(self.position, self.look_dir, self.up_dir));
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
            light_dir: vec3(1.0, 1.0, -0.1),
            universal_lighting: 0.6,
            depth_buffer: None,
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
