extern crate byteorder;
extern crate itertools;

use cgmath::{Vector3, Point3};
use std::fs::File;
use std::io;
use std::io::Cursor;
use std::io::prelude::*;
use std::mem;
use std::slice;

use self::byteorder::{BigEndian, ReadBytesExt};
use self::itertools::Itertools;

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

struct Sphere {
    position: Point3<f32>,
    radius: f32,
    material: Material,
}

impl Sphere {
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

    pub fn default_scene() -> Result<Scene, io::Error> {
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


        let k = try!(Scene::read_skybox());

        Ok(scene)
    }

    fn add(&mut self, sphere: Sphere) {
        self.spheres.push(sphere)
    }

    fn read_skybox() -> Result<Vec<f32>, io::Error> {
        let mut f = try!(File::open("./assets/sky_15.raw"));

        let amount = 2500 * 1250 * 3 * mem::size_of::<f32>();

        let mut bytes = Vec::with_capacity(amount);

        try!(f.read_exact(&mut bytes));

        let floats = bytes.chunks(4).map(|chunk| {
            let k : [u8;4] = [chunk[0],chunk[1],chunk[2],chunk[3]];
            let float : f32 = unsafe { mem::transmute(k) };
            float
        }).collect();
        Ok(floats)
    }

}
