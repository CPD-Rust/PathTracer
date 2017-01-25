extern crate cgmath;
use self::cgmath::{Vector3, Point3};
use std::io;
use std::mem;
use rand;
use ray::{Ray,Intersection};
use bvh::BVH;
use std::f32::consts::FRAC_1_PI;
extern crate memmap;
use self::memmap::*;

use primitive::Primitive;
use primitive::sphere::Sphere;
use material::Material;

#[derive(Debug)]
pub struct Scene<T: Primitive> {
    pub bvh: BVH<T>,
    skybox: Vec<f32>,
}

impl<T: Primitive> Scene<T> {
    // creates a new default scene
    fn new(objects: Vec<T>) -> Result<Scene<T>, io::Error> {
        let skybox = try!(Scene::<T>::read_skybox());
        let scene = Scene {
            bvh: BVH::new(objects),
            skybox: skybox,
        };
        Ok(scene)
    }
    pub fn intersect_closest(&self, ray : & mut Ray) -> Option<Intersection> {
        self.bvh.intersect_closest(ray)
    }

    pub fn default_scene() -> Result<Scene<Sphere>, io::Error> {
        print!("Setting up default_scene\n");
        let mut spheres = Vec::new();
        spheres.push(Sphere::light(Point3::new(2.7,1.7,-0.5), 0.3));

        let bottom_plane = Sphere {
            position: Point3::new(0.0,-4999.0,0.0),
            radius: 4998.5,
            material: Material::Diffuse {
                speculaty: 0.,
                color: Vector3::new(0.0,1.0,1.0),
            },
        };

        //spheres.push(bottom_plane);
        spheres.push(Sphere {
            position: Point3::new(-0.8, 0.0, -2.0),
            radius: 0.3,
            material: Material::Diffuse {
                speculaty: 0.8,
                color: Vector3::new(1.0,0.2,0.2),
            },
        });

        spheres.push(Sphere {
            position: Point3::new(0.0,0.0,-2.0),
            radius: 0.3,
            material: Material::Dielectic {
                refraction_index_n1: 1.,
                refraction_index_n2: 1.3,
                color: Vector3::new(0.1,1.0,0.1),
            },
        });


        spheres.push(Sphere {
            position: Point3::new(0.8,0.0,-2.0),
            radius: 0.3,
            material: Material::Diffuse {
                speculaty: 0.8,
                color: Vector3::new(0.2, 0.2, 1.0),
            },
        });

        spheres.push(Sphere {
            position: Point3::new(-0.8,-0.8,-2.0),
            radius: 0.5,
            material: Material::Diffuse {
                speculaty: 0.,
                color: Vector3::new(1.0,1.0,1.0),
            },
        });
        spheres.push(Sphere {
            position: Point3::new(-0.0,-0.8,-2.0),
            radius: 0.5,
            material: Material::Diffuse {
                speculaty: 0.,
                color: Vector3::new(1.0,1.0,1.0),
            },
        });
        spheres.push(Sphere {
            position: Point3::new(0.8,-0.8,-2.0),
            radius: 0.5,
            material: Material::Diffuse {
                speculaty: 0.,
                color: Vector3::new(1.0,1.0,1.0),
            },
        });

        let scene = try!(Scene::new(spheres));
        Ok(scene)
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
        let mut floats = vec![0.0 as f32; bytes.len() / 2];
        println!("{}",floats.len());

        for (chunk, mut float) in bytes.chunks(4).into_iter().zip(floats.iter_mut()) {
            // we assume big endian here!
            // but intel is little endian
            *float = unsafe { mem::transmute([chunk[0],chunk[1],chunk[2],chunk[3]]) };
        }
        Ok(floats)
    }


}
