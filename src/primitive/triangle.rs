extern crate cgmath;
use self::cgmath::{Vector3, Point3, InnerSpace, EuclideanSpace};
use std::f32;
use rand;
use rand::Closed01;

use super::Primitive;
use super::aabb::AABB;

use ray::{Ray,Intersection};
use material::{Material, LIGHT_COLOR};

#[derive(Debug)]
pub struct Triangle {
    pub position0: Point3<f32>,
    pub position1: Point3<f32>,
    pub position2: Point3<f32>,
    pub normal0: Vector3<f32>,
    pub normal1: Vector3<f32>,
    pub normal2: Vector3<f32>,
    pub material: Material,
}

impl Triangle {
    pub fn light(p0 : Point3<f32>, p1 : Point3<f32>, p2 : Point3<f32>, n0 : Vector3<f32>, n1 : Vector3<f32>, n2 : Vector3<f32>) -> Triangle {
        Triangle {
            position0: p0,
            position1: p1,
            position2: p2,
            normal0: n0,
            normal1: n1,
            normal2: n2,
            material: Material::Emissive {
                color: LIGHT_COLOR,
            }
        }
    }
}

impl Primitive for Triangle {
    fn intersect(&self, ray: &mut Ray) -> Option<Intersection> {
        let edge1 = self.position1 - self.position0;
        let edge2 = self.position2 - self.position0;
        let h = ray.direction.cross(edge2);
        let a = edge1.dot(h);

        if a > -f32::EPSILON && a < f32::EPSILON {
            return None
        }

        let f = 1.0 / a;
        let s = ray.origin - self.position0;
        let u = f * s.dot(h);

        if u < 0.0 || u > 1.0 {
            return None
        }

        let q = s.cross(edge1);
        let v = f * ray.direction.dot(q);

        if v < 0.0 || u + v > 1.0 {
            return None
        }

        // at this stage we can compute t to find out where
        // the intersection point is on the ray
        let t = f * edge2.dot(q);
        if t < 0.0 {
            return None // the intersection is behind the ray's origin
        }
        ray.distance = t;
        Some(Intersection{
            normal: ((1. - u - v) * self.normal0 + u * self.normal1 + v * self.normal2).normalize(),
            inside: a < 0.,
            material: &self.material,
        })
    }
    fn centre(&self) -> Point3<f32> {
        (self.position0 + self.position1.to_vec() + self.position2.to_vec()) * (1.0 / 3.0)
    }
    fn bounds(&self) -> AABB {
        AABB {min : Point3 {x : self.position0.x.min(self.position1.x).min(self.position2.x),
                            y : self.position0.y.min(self.position1.y).min(self.position2.y),
                            z : self.position0.z.min(self.position1.z).min(self.position2.z) },
              max : Point3 {x : self.position0.x.max(self.position1.x).max(self.position2.x),
                            y : self.position0.y.max(self.position1.y).max(self.position2.y),
                            z : self.position0.z.max(self.position1.z).max(self.position2.z) }}
    }
    fn is_light(&self) -> Option<Vector3<f32>> {
        match self.material {
            Material::Emissive { color } => Some(color),
            _ => None,
        }
    }
    fn random_point(&self) -> (Point3<f32>, f32) {
        let Closed01(u) = rand::random::<Closed01<f32>>();
        let Closed01(v) = rand::random::<Closed01<f32>>();
        let mut edge1 = self.position1 - self.position0;
        let mut edge2 = self.position2 - self.position0;
        let point = self.position0 + u * edge1 + v* edge2;
        let len1 = edge1.magnitude();
        let len2 = edge2.magnitude();
        edge1 /= len1;
        edge2 /= len2;
        let area = 0.5 * (1.0 - edge1.dot(edge2)) * len1 * len2;
        (point, area)
    }
}

#[test]
fn intersections_triangle() {
    let triangle = Triangle::light(Point3::new(1.0, 1.0, 2.0),Point3::new(1.0, -1.0, 2.0),Point3::new(-1.0, 0.0, 2.0),Vector3::new(0.0, 0.0, -1.0),Vector3::new(0.0, 0.0, -1.0),Vector3::new(0.0, 0.0, -1.0));

    // Intersects forwards
    let mut r1 = Ray::new(Point3::new(0.0,0.0,0.0), Vector3::new(0.0,0.0,1.0), f32::INFINITY);
    assert!(triangle.intersect(&mut r1).is_some());

    // Doesn't intersect backwards.
    let mut r1 = Ray::new(Point3::new(0.0,0.0,0.0), Vector3::new(0.0,0.0,-1.0), f32::INFINITY);
    assert!(!triangle.intersect(&mut r1).is_some());

    // Barely intersects top.
    let mut r1 = Ray::new(Point3::new(1.0,1.0,0.0), Vector3::new(0.0,0.0,1.0), f32::INFINITY);
    assert!(triangle.intersect(&mut r1).is_some());

    // Doesn't intersect on ray origin, is parrallel to triangle.
    let mut r1 = Ray::new(Point3::new(0.0,0.0,2.0), Vector3::new(0.0,1.0,0.0), f32::INFINITY);
    assert!(!triangle.intersect(&mut r1).is_some());

    // Intersects on ray origin.
    let mut r1 = Ray::new(Point3::new(0.0,0.0,2.0), Vector3::new(0.0,0.0,-1.0), f32::INFINITY);
    assert!(triangle.intersect(&mut r1).is_some());

    // Intersects on ray origin.
    let mut r1 = Ray::new(Point3::new(0.0,0.0,2.0), Vector3::new(0.0,0.0,1.0), f32::INFINITY);
    assert!(triangle.intersect(&mut r1).is_some());

    // Doesn't intersect ray in front of triangle.
    let mut r1 = Ray::new(Point3::new(0.0,0.0,2.5), Vector3::new(0.0,0.0,1.0), f32::INFINITY);
    assert!(!triangle.intersect(&mut r1).is_some());

    // Intersects triangle from other side.
    let mut r1 = Ray::new(Point3::new(0.0,0.0,2.5), Vector3::new(0.0,0.0,-1.0), f32::INFINITY);
    assert!(triangle.intersect(&mut r1));
}
