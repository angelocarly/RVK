use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub reflect_count: u32
}

pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_plane: f32
}

impl Camera {
    pub fn new( position: Vec3, direction: Vec3, up: Vec3, fov: f32, aspect_ratio: f32, near_plane: f32 ) -> Camera {
        let right = direction.cross( up ).normalize();
        let up = right.cross( direction ).normalize();
        Camera { position, direction, up, right, fov, aspect_ratio, near_plane }
    }

    // Get a ray from the camera to the pixel at (x [0-1], y [0-1]) in the image.
    pub fn get_ray( &self, x: f32, y: f32 ) -> Ray {

        // Position on the plane
        let x = f32::tan( self.fov / 2. ) * self.near_plane * ( x - 0.5 ) * self.aspect_ratio;
        let y = f32::tan( self.fov / 2. ) * self.near_plane * ( -y + 0.5 );

        // Position in world space
        let pix_pos = self.position + self.near_plane * self.direction + self.right * x + self.up * y;

        let direction = (pix_pos - self.position).normalize();
        Ray { origin: self.position, direction, reflect_count: 0 }
    }
}
