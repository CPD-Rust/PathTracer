extern crate cgmath;
extern crate rand;
extern crate sdl2;
use self::cgmath::{Vector3, Point3};
use self::cgmath::InnerSpace;
use super::ray::Ray;
use std::f32;
use std::collections::HashSet;
use self::sdl2::keyboard::Keycode;
use self::rand::Closed01;

use scene::Scene;

#[derive(Debug)]
pub struct Camera {
    origin: Point3<f32>,
    target: Point3<f32>,
    focal_distance: f32,
    direction: Vector3<f32>,

    // screen plane
    p1: Point3<f32>,
    p2: Point3<f32>,
    p3: Point3<f32>,

    up: Vector3<f32>,
    right: Vector3<f32>,
    width: usize,
    height: usize,
    lens_size: f32,
    scene: Scene,
}

impl Camera {
    pub fn new(width: usize, height: usize, scene: Scene) -> Camera {
        let mut camera = Camera {
            width: width,
            height: height,
            lens_size: 0.04,
            origin: Point3::new(-0.94, -0.037, -3.342),
            target: Point3::new(-0.418, -0.026, -2.435),
            direction: Vector3::new(0.0,0.0,0.0),
            focal_distance: 0.0,
            p1: Point3::new(0.0,0.0,0.0),
            p2: Point3::new(0.0,0.0,0.0),
            p3: Point3::new(0.0,0.0,0.0),
            right: Vector3::new(0.0,0.0,0.0),
            up: Vector3::new(0.0,0.0,0.0),
            scene: scene
        };
        camera.update();
        camera
    }

    pub fn handle_input(&mut self, key_presses : &HashSet<Keycode>) -> bool  {
        self.target = self.origin + self.direction;

        let changed = if key_presses.contains(&Keycode::A) {
            self.origin = self.origin + (-0.1 * self.right);
            self.target = self.target + (-0.1 * self.right);
            true
        } else { false };
        let changed = if key_presses.contains(&Keycode::D) {
            self.origin =  self.origin + (0.1 * self.right);
            self.target =  self.target + (0.1 * self.right);
            true
        } else { changed };
        let changed = if key_presses.contains(&Keycode::W) {
            self.origin = self.origin + (0.1 * self.direction);
            true
        } else { changed };
        let changed = if key_presses.contains(&Keycode::S) {
            self.origin = self.origin + (-0.1 * self.direction);
            true
        } else { changed };
        let changed = if key_presses.contains(&Keycode::R) {
            self.origin = self.origin + (0.1 * self.up);
            self.target =  self.target + (0.1 * self.up);
            true
        } else { changed };
        let changed = if key_presses.contains(&Keycode::F) {
            self.origin = self.origin + (-0.1 * self.up);
            self.target = self.target + (-0.1 * self.up);
            true
        } else { changed };
        let changed = if key_presses.contains(&Keycode::Up) {
            self.target = self.target + (-0.1 * self.up);
            true
        } else { changed };
        let changed = if key_presses.contains(&Keycode::Down) {
            self.target = self.target + (0.1 * self.up);
            true
        } else { changed };
        let changed = if key_presses.contains(&Keycode::Left) {
            self.target = self.target + (-0.1 * self.right);
            true
        } else { changed };
        let changed = if key_presses.contains(&Keycode::Right) {
            self.target = self.target + (0.1 * self.right);
            true
        } else { changed };
        if changed {
            self.update();
            true
        } else {
            false
        }

    }

    fn update(&mut self) {
        self.direction = (self.target-self.origin).normalize();
        let unit_y = Vector3::new(0.0,1.0,0.0);
        self.right = unit_y.cross(self.direction);
        self.up = self.direction.cross(self.right);

        let mut ray = Ray::new(self.origin, self.direction, f32::INFINITY);
        let _intersection = self.scene.intersect(& mut ray);

        let aspect_ratio = (self.width as f32) / (self.height as f32);

        self.focal_distance = f32::min(20.0, ray.distance);

        let c = self.origin + self.focal_distance * self.direction;

        self.p1 = c  + (-0.5 * self.focal_distance * aspect_ratio * self.right) + (0.5 * self.focal_distance * self.up);
        self.p2 = c + (0.5 * self.focal_distance * aspect_ratio * self.right) + (0.5 * self.focal_distance * self.up);
        self.p3 = c + (-0.5 * self.focal_distance * aspect_ratio * self.right) + (-0.5 * self.focal_distance * self.up);

    }

    /// sample a ray by shooting it through the scene
    pub fn sample(&self, ray : & mut Ray, depth: u32) -> Vector3<f32> {
        let intersection = self.scene.intersect(ray);
        let sample = match intersection {
            None => {
                self.scene.sample_skybox(ray.direction)
            },
            Some(ref intersection) => {
                self.scene.sample_skybox(ray.direction)
            }
        };
        sample
    }

    /// generates a nice Ray (TODO better integer type)
    pub fn generate(&self, x: usize, y: usize) -> Ray {
        // NOTE: we do not have to keep track of a
        // pool of random number generators, each
        // thread in rust has its own random
        // number generator by default :)
        let Closed01(r0) = rand::random::<Closed01<f32>>();
        let Closed01(r1) = rand::random::<Closed01<f32>>();
        let Closed01(r2) = rand::random::<Closed01<f32>>();
        let r2 = r2 - 0.5;
        let Closed01(r3) = rand::random::<Closed01<f32>>();
        let r3 = r3 - 0.5;

        // calculate sub-pixel ray target position on screen plane
        let u = ((x as f32) + r0) / (self.width as f32);
        let v = ((y as f32) + r1) / (self.height as f32);
        let target = self.p1 + u * (self.p2 - self.p1) + v * (self.p3 - self.p1);
        let origin = self.origin + self.lens_size * (r2 * self.right + r3 * self.up);
        let direction = (target-origin).normalize();

        // hmm all directions are the same. that seems to be a bug =)

        Ray::new(origin, direction, f32::INFINITY)

    }
}
