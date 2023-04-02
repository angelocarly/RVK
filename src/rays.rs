use glam::{Vec3, Vec2, Vec2Swizzles};
use crate::camera;

pub struct Hit<'a> {
    pub position: Vec3,
    pub distance: f32,
    pub normal: Vec3,
    pub shape: &'a Box< dyn Hittable >
}

pub struct Material {
    pub color: Vec3,
    pub reflective: bool
}

pub trait Hittable {
    fn distance( &self, pos: Vec3 ) -> f32;
    fn material( &self ) -> & Material;
    fn calc_normal(&self, pos: Vec3 ) -> Vec3 {
        let h = 0.0001;
        let k = Vec2::new( 1.,-1. );
        ( k.xyy() * self.distance( pos + k.xyy() * h )
        + k.yyx() * self.distance( pos + k.yyx() * h )
        + k.yxy() * self.distance( pos + k.yxy() * h )
        + k.xxx() * self.distance( pos + k.xxx() * h ) ).normalize()
    }
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

struct Wall {
    position: Vec3,
    size: Vec3,
    material: Material
}

impl Hittable for Wall {
    fn distance( &self, pos: Vec3 ) -> f32 {
        // https://iquilezles.org/articles/distfunctions/
        let q = (pos - self.position).abs() - self.size;
        q.max( Vec3::ZERO ).length() + q.max_element().min( 0. )
    }

    fn material( &self ) -> & Material {
        & self.material
    }
}

pub struct World {
    content: Vec<Box< dyn Hittable>>
}

fn reflect( a: Vec3, n: Vec3 ) ->Vec3 {
    let reverse_a = -a;
    reverse_a - 2. * reverse_a.dot( n ) * n
}

impl World {
    pub fn new() -> World {
        World {
            content: vec![
                Box::new( Sphere { position: Vec3::new( 0., -1.5, 0. ), radius: 1.5, material: Material { color: Vec3::new( 1., 0., 0. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( -3.0, 1.5, 0. ), radius: 1.5, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( 3.0, 1.5, 0. ), radius: 1.5, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( 0., 2.5, -6. ), radius: 1.0, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: false } } ),
                // Box::new( Sphere { position: Vec3::new( 0., -4., 0. ), radius: 2., material: Material { color: Vec3::new( 1.0, 0.5, 0. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( 0., -100000. - 2., 0. ), radius: 100000., material: Material { color: Vec3::new( 245. / 255., 243. / 255., 193. / 255. ), reflective: false } } ),
                Box::new( Wall { position: Vec3::new( 0., 1.5, 0. ), size: Vec3::new( 1., 1., 1. ), material: Material { color: Vec3::new( 14. / 255., 162. / 255., 147. / 255. ), reflective: true } } )
            ]
        }
    }

    pub fn cast( &self, ray: camera::Ray ) -> Option< Hit > {

        if self.content.len() == 0 {
            return None;
        }

        let mut t = 0.;
        for _i in 0..300 {
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
            let EPSILON = 0.01;
            if min_dist < EPSILON {

                let shape = closest_shape.unwrap();
                if shape.material().reflective {
                    // We hit something reflective

                    if ray.reflect_count > 4 {
                        return None;
                    }

                    // Reflect around the normal
                    let position = ray.origin + ray.direction * t;
                    let normal = shape.calc_normal( position );
                    let direction = reflect( -ray.direction, normal ).normalize();
                    return self.cast( camera::Ray { origin: position + direction * EPSILON * 2., direction, reflect_count: ray.reflect_count + 1 } );
                }

                return Some( Hit {
                    position: ray.origin + ray.direction * t,
                    distance: t,
                    normal: shape.calc_normal( ray.origin + ray.direction * t ),
                    shape: closest_shape.unwrap()
                } );
            }
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    #[test]
    fn reflect_x_axis() {
        let r = Vec3::new( 1., 0., 0. );
        let n = Vec3::new( -1., 0., 0. );
        let reflected = super::reflect( -r, n );
        assert_eq!( reflected, Vec3::new( -1., 0., 0. ) );
    }

    #[test]
    fn reflect_y_axis() {
        let r = Vec3::new( 0., 1., 0. );
        let n = Vec3::new( 0., -1., 0. );
        let reflected = super::reflect( -r, n );
        assert_eq!( reflected, Vec3::new( 0., -1., 0. ) );
    }

    #[test]
    fn reflect_z_axis() {
        let r = Vec3::new( 0., 0., 1. );
        let n = Vec3::new( 0., 0., -1. );
        let reflected = super::reflect( -r, n );
        assert_eq!( reflected, Vec3::new( 0., 0., -1. ) );
    }

    #[test]
    fn reflect_diagonal() {
        let ray = Vec3::new( 0., 1., 1. ).normalize();
        let n = Vec3::new( 0., 0., -1. );
        let result = Vec3::new( 0., 1., -1. ).normalize();
        let reflected = super::reflect( -ray, n );
        assert_eq!( reflected, result );
    }
}