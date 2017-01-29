extern crate cgmath;
use self::cgmath::Vector3;

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Diffuse {
        speculaty: f32,
        color: Vector3<f32>,
    },
    Dielectric {
        refraction_index_n1: f32,
        refraction_index_n2: f32,
        color: Vector3<f32>,
    },
    Emissive {
        color: Vector3<f32>,
    }
}

const LIGHT_SCALE: f32 = 0.2;

pub const LIGHT_COLOR: Vector3<f32> =
    Vector3 {
        x: 8.5 * LIGHT_SCALE,
        y: 8.5 * LIGHT_SCALE,
        z: 7.0 * LIGHT_SCALE,
    };
