use glam::{Vec3, Vec2, Vec2Swizzles, Mat4, Vec4, Vec4Swizzles};
use crate::camera;

pub struct Hit<'a> {
    pub position: Vec3,
    pub distance: f32,
    pub normal: Vec3,
    pub shape: &'a Box< dyn Hittable >,
    pub bounces: u32,
    pub cum_length: f32,
    pub weight: f32
}

pub struct Miss {
    pub position: Vec3,
    pub bounces: u32,
    pub cum_length: f32,
    weight: f32
}

pub enum CastResult<'a> {
    Hit( Hit<'a> ),
    Miss( Miss )
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
    rotation: Mat4,
    size: Vec3,
    material: Material
}

impl Hittable for Wall {
    fn distance( &self, pos: Vec3 ) -> f32 {
        // https://iquilezles.org/articles/distfunctions/
        let dp = pos - self.position;
        let rp = (self.rotation * Vec4::new( dp.x, dp.y, dp.z, 1. )).xyz();
        let q = rp.abs() - self.size;
        q.max( Vec3::ZERO ).length() + q.max_element().min( 0. )
    }

    fn material( &self ) -> & Material {
        & self.material
    }
}

// struct Triangle {
//     a: Vec3,
//     b: Vec3,
//     c: Vec3,
//     material: Material
// }
//
// fn dot2( v: Vec3 ) -> f32 {
//     v.dot(v)
// }
//
// impl Hittable for Triangle {
//     fn distance( &self, pos: Vec3 ) -> f32 {
//         let ba = self.b - self.a; let pa = pos - self.a;
//         let cb = self.c - self.b; let pb = pos - self.b;
//         let ac = self.a - self.c; let pc = pos - self.c;
//         let nor = ba.cross(ac);
//
//         let s = f32::signum(ba.cross(nor).dot(pa)) +
//                 f32::signum(cb.cross(nor).dot(pb)) +
//                 f32::signum(ac.cross(nor).dot(pc)) < 2.0;
//
//         return f32::sqrt(
//             ( f32::signum(ba.cross(nor).dot(pa)) +
//                 f32::signum(cb.cross(nor).dot(pb)) +
//                 f32::signum(ac.cross(nor).dot(pc)) < 2.0 ) ?
//             f32::min( f32::min(
//                 dot2(ba*clamp(dot(ba,pa)/dot2(ba),0.0,1.0)-pa),
//                 dot2(cb*clamp(dot(cb,pb)/dot2(cb),0.0,1.0)-pb) ),
//                  dot2(ac*clamp(dot(ac,pc)/dot2(ac),0.0,1.0)-pc) )
//             :
//                 nor.dor(pa)*nor.dot(pa)/dot2(nor) );
//     }
//
//     fn material( &self ) -> & Material {
//         & self.material
//     }
// }

pub struct World {
    content: Vec<Box< dyn Hittable>>
}

fn reflect( a: Vec3, n: Vec3 ) -> Vec3 {
    let reverse_a = -a;
    reverse_a - 2. * reverse_a.dot( n ) * n
}

impl World {
    pub fn new() -> World {
        World {
            content: vec![
                Box::new( Wall { position: Vec3::new( 0., -10., 0. ), rotation: Mat4::IDENTITY, size: Vec3::new( 100., 0.1, 100. ), material: Material { color: Vec3::new( 245. / 255., 243. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Wall { position: Vec3::new( 0., 10., 0. ), rotation: Mat4::IDENTITY, size: Vec3::new( 100., 0.1, 100. ), material: Material { color: Vec3::new( 245. / 255., 243. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Wall { position: Vec3::new( 0., 0., 10. ), rotation: Mat4::IDENTITY, size: Vec3::new( 100., 100., 0.1 ), material: Material { color: Vec3::new( 245. / 255., 243. / 255., 193. / 255. ), reflective: true } } ),
                // Box::new( Wall { position: Vec3::new( 0., 0., -10. ), rotation:Mat4::look_at_rh( Vec3::new( 0.5, 0.5, 0.5).normalize(), Vec3::ZERO, Vec3::new(0.,1.,0.) ), size: Vec3::new( 100., 100., 0.1 ), material: Material { color: Vec3::new( 245. / 255., 243. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Wall { position: Vec3::new( 0., 0., -10. ), rotation: Mat4::IDENTITY, size: Vec3::new( 100., 100., 0.1 ), material: Material { color: Vec3::new( 245. / 255., 243. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Wall { position: Vec3::new( -10.0, 0., 0. ), rotation: Mat4::IDENTITY, size: Vec3::new( 0.1, 100., 100. ), material: Material { color: Vec3::new( 245. / 255., 243. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Wall { position: Vec3::new( 10., 0., 0. ), rotation: Mat4::IDENTITY, size: Vec3::new( 0.1, 100., 100. ), material: Material { color: Vec3::new( 245. / 255., 243. / 255., 193. / 255. ), reflective: true } } ),
                // Box::new( Wall { position: Vec3::new( 10., 0., 0. ), rotation: Mat4::IDENTITY, size: Vec3::new( 1.5, 1.5, 1.5 ), material: Material { color: Vec3::new( 245. / 255., 243. / 255., 193. / 255. ), reflective: true } } ),
                // Box::new( Wall { position: Vec3::new( 3.4, 0., 0. ), rotation: Mat4::IDENTITY, size: Vec3::new( 0.1, 100., 100. ), material: Material { color: Vec3::new( 245. / 255., 243. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( 0., 0., 0. ), radius: 1.0, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: true } } ),
                // Box::new( Wall { position: Vec3::new( 0., 0., 0. ), rotation: Mat4::IDENTITY, size: Vec3::new( 0.7, 0.7, 0.7 ), material: Material { color: Vec3::new( 245. / 255., 243. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( -10., -10., -10. ), radius: 5.0, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( 10., -10., 10. ), radius: 5.0, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( -10., 10., 10. ), radius: 5.0, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( 10., 10., -10. ), radius: 5.0, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: true } } ),

                Box::new( Sphere { position: Vec3::new( 10., 10., 10. ), radius: 5.0, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( -10., 10., -10. ), radius: 5.0, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( 10., -10., -10. ), radius: 5.0, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: true } } ),
                Box::new( Sphere { position: Vec3::new( -10., -10., 10. ), radius: 5.0, material: Material { color: Vec3::new( 39. / 255., 225. / 255., 193. / 255. ), reflective: true } } ),
            ]
        }
    }

    pub fn cast( &self, ray: camera::Ray, max_distance: f32 ) -> Option< CastResult > {

        if self.content.len() == 0 {
            return None;
        }

        let mut t = 0.;
        for _i in 0..500 {
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
            if ray.cum_length + t > max_distance {
                return Some( CastResult::Hit( Hit {
                    position: ray.origin + ray.direction * t,
                    distance: t,
                    normal: Vec3::ZERO,
                    shape: closest_shape.unwrap(),
                    bounces: ray.reflect_count,
                    cum_length: ray.cum_length + t,
                    weight: ray.weigth
                } ) );
            }

            let EPSILON = 0.0001;
            if min_dist < EPSILON {

                let shape = closest_shape.unwrap();
                if shape.material().reflective {
                    // We hit something reflective

                    // Gotta save the stack somehow
                    if ray.reflect_count > 500 {
                        return None;
                    }

                    // Reflect around the normal
                    let position = ray.origin + ray.direction * t;
                    let normal = shape.calc_normal( position );
                    let direction = reflect( -ray.direction, normal ).normalize();
                    return self.cast(
                        camera::Ray {
                            origin: position + direction * EPSILON * 2.,
                            direction,
                            reflect_count: ray.reflect_count + 1,
                            cum_length: ray.cum_length + t,
                            weigth: ray.weigth + 1. // f32::sin( t )
                        },
                        max_distance
                    );
                }

                return Some( CastResult::Hit( Hit {
                    position: ray.origin + ray.direction * t,
                    distance: t,
                    normal: shape.calc_normal( ray.origin + ray.direction * t ),
                    shape: closest_shape.unwrap(),
                    bounces: ray.reflect_count,
                    cum_length: ray.cum_length + t,
                    weight: ray.weigth
                } ) );
            }
        }

        return Some( CastResult::Miss( Miss {
            bounces: ray.reflect_count,
            cum_length: ray.cum_length + t,
            position: ray.origin + ray.direction * t,
            weight: ray.weigth
        } ) );
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