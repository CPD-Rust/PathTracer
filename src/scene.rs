extern crate cgmath;
use self::cgmath::{Vector3, Point3, InnerSpace};
use std::fs::File;
use std::io;
use std::io::Cursor;
use std::io::prelude::*;
use std::mem;
use std::slice;
use ray::Ray;
use std::f32::consts::FRAC_1_PI;
extern crate memmap;
use self::memmap::*;


#[derive(Debug)]
pub enum Material {
    CheckerBoard,
    Realistic {
        refl: f32,
        refr: f32,
        emissive: bool,
        diffuse: Vector3<f32>,
    }
}


const LIGHT_SIZE: f32 = 0.3;
const LIGHT_SCALE: f32 = 1.0;

const LIGHT_COLOR: Vector3<f32> =
    Vector3 {
        x: 8.5 * LIGHT_SCALE,
        y: 8.5 * LIGHT_SCALE,
        z: 7.0 * LIGHT_SCALE,
    };

#[derive(Debug)]
struct Sphere {
    position: Point3<f32>,
    radius: f32,
    material: Material,
}

impl Sphere {

    fn intersect(&self, ray : & mut Ray) {
        let distance = self.position - ray.origin;
        let tca = distance.dot(ray.direction);

        if tca  < 0.0 {
            return
        }

        let d2 = distance.dot(distance) - tca*tca;

        if d2 > self.radius {
            return
        }

        let thc = (self.radius - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;

        if t0 > 0.0 {
            if t0 > ray.distance {
                return
            }
            ray.normal = (ray.origin + ray.direction - self.position).normalize();
        }

    }

    fn light(position: Point3<f32>, radius: f32) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            material: Material::Realistic {
                refl: 0.0,
                refr: 0.0,
                emissive: true,
                diffuse: LIGHT_COLOR,
            }
        }
    }
}

#[derive(Debug)]
pub struct Scene {
    spheres: Vec<Sphere>,
    skybox: Vec<f32>,
}

impl Scene {
    // creates a new default scene
    fn new() -> Result<Scene, io::Error> {
        let mut spheres = Vec::new();
        let skybox = try!(Scene::read_skybox());
        let mut scene = Scene {
            spheres: spheres,
            skybox: skybox,
        };
        Ok(scene)
    }

    pub fn intersect(&self, ray : & mut Ray) {
        for sphere in &self.spheres {
            sphere.intersect(ray);
        }
    }

    pub fn default_scene() -> Result<Scene, io::Error> {
        print!("Setting up default_scene\n");
        let mut scene = try!(Scene::new());

        scene.add(Sphere::light(Point3::new(2.7,1.7,-0.5), 0.3));

        let bottomPlane = Sphere {
            position: Point3::new(0.0,-4999.0,0.0),
            radius: 4998.5,
            material: Material::CheckerBoard,
        };

        let backPlane = Sphere {
            position: Point3::new(0.0,0.0,-5000.0),
            radius: 4998.5,
            material: Material::Realistic {
                diffuse: Vector3::new(1.0,1.0,1.0),
                refl: 0.0,
                refr: 0.0,
                emissive: false,
            },
        };

        scene.add(bottomPlane);
        scene.add(backPlane);
        scene.add(Sphere {
            position: Point3::new(-0.8, 0.0, -2.0),
            radius: 0.3 * 0.3,
            material: Material::Realistic {
                diffuse: Vector3::new(1.0,0.2,0.2),
                refl: 0.8,
                refr: 0.0,
                emissive: false,
            },
        });

        scene.add(Sphere {
            position: Point3::new(0.0,0.0,-2.0),
            radius: 0.3 * 0.3,
            material: Material::Realistic {
                diffuse: Vector3::new(0.9,1.0,0.9),
                refl: 0.0,
                refr: 1.0,
                emissive: false,
            },
        });

        scene.add(Sphere {
            position: Point3::new(0.8,0.0,-2.0),
            radius: 0.3 * 0.3,
            material: Material::Realistic {
                diffuse: Vector3::new(0.2, 0.2, 1.0),
                refl: 0.8,
                refr: 0.0,
                emissive: false,
            },
        });

        scene.add(Sphere {
            position: Point3::new(-0.8,-0.8,-2.0),
            radius: 0.5 * 0.5,
            material: Material::Realistic {
                diffuse: Vector3::new(1.0, 1.0, 1.0),
                refl: 0.0,
                refr: 0.0,
                emissive: false,
            },
        });
        scene.add(Sphere {
            position: Point3::new(-0.0,-0.8,-2.0),
            radius: 0.5 * 0.5,
            material: Material::Realistic {
                diffuse: Vector3::new(1.0, 1.0, 1.0),
                refl: 0.0,
                refr: 0.0,
                emissive: false,
            },
        });
        scene.add(Sphere {
            position: Point3::new(0.8,-0.8,-2.0),
            radius: 0.5 * 0.5,
            material: Material::Realistic {
                diffuse: Vector3::new(1.0, 1.0, 1.0),
                refl: 0.0,
                refr: 0.0,
                emissive: false,
            },
        });


        Ok(scene)
    }

    fn add(&mut self, sphere: Sphere) {
        self.spheres.push(sphere)
    }

    pub fn sample_skybox(&self, direction: Vector3<f32>) -> Vector3<f32> {
        let u = (2500.0 * 0.5 * (1.0 + direction.x.atan2(-direction.z) * FRAC_1_PI)) as usize;
        let v = (1250.0 * (direction.y.acos() * FRAC_1_PI)) as usize;
        let idx = u + v * 2500;
        Vector3::new(self.skybox[idx*3+0],
                     self.skybox[idx*3+1],
                     self.skybox[idx*3+2])
    }

    fn read_skybox() -> Result<Vec<f32>, io::Error> {
        let file = try!(Mmap::open_path("./assets/sky_15.raw", Protection::Read));
        let bytes: &[u8] = unsafe { file.as_slice() };
        let mut floats = vec![0.0 as f32; bytes.len() / 4];
        for (mut chunk, mut float) in bytes.chunks(4).into_iter().zip(floats.iter_mut()) {
            *float = unsafe { mem::transmute([chunk[0],chunk[1],chunk[2],chunk[3]]) };
        }
        Ok(floats)
    }

}
