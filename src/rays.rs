use std::cmp::min;
use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

pub struct Hit<'a> {
    pub position: Vec3,
    pub distance: f32,
    pub shape: &'a Box< dyn Hittable >
}

pub struct Material {
    pub color: Vec3
}

pub trait Hittable {
    fn distance( &self, pos: Vec3 ) -> f32;
    fn material( &self ) -> & Material;
}

struct Sphere {
    position: Vec3,
    radius: f32,
    material: Material
}

impl Hittable for Sphere {
    fn distance( &self, pos: Vec3 ) -> f32 {
        (pos - self.position).length() - self.radius
    }

    fn material( &self ) -> & Material {
        & self.material
    }
}

pub struct World {
    content: Vec<Box< dyn Hittable>>
}

impl World {
    pub fn new() -> World {
        World {
            content: vec![
                Box::new( Sphere { position: Vec3::new( 0., 0.5, 0. ), radius: 1., material: Material { color: Vec3::new( 1., 0.5, 0. ) } } ),
                Box::new( Sphere { position: Vec3::new( 0., -100., 0. ), radius: 100., material: Material { color: Vec3::new( 0., 1., 0. ) } } ),
            ]
        }
    }

    pub fn cast( &self, ray: Ray ) -> Option< Hit > {

        if self.content.len() == 0 {
            return None;
        }

        let mut t = 0.;
        for _i in 0..100 {
            let mut closest_shape = None;
            let mut min_dist = f32::MAX;
            for shape in &self.content {
                let dist = shape.distance( ray.origin + ray.direction * t );
                if dist < min_dist {
                    min_dist = dist;
                    closest_shape = Some( shape );
                }
            }

            t += min_dist;
            if min_dist < 0.000001 {
                return Some( Hit {
                    position: ray.origin + ray.direction * t,
                    distance: t,
                    shape: closest_shape.unwrap()
                } );
            }
        }

        return None;
    }
}