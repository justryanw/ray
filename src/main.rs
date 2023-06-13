use std::f32::consts::PI;

use glam::{UVec2, Vec2, Vec3};
use image::{Rgb, RgbImage};
use rand::random;

#[derive(Clone)]
struct Material {
    colour: Vec3,
    emission_colour: Vec3,
    emission_strength: f32,
}

struct Sphere {
    position: Vec3,
    radius: f32,
    material: Material,
}

#[derive(Clone)]
struct Ray {
    position: Vec3,
    direction: Vec3,
}

struct HitInfo {
    distance: f32,
    hit_position: Vec3,
    normal: Vec3,
    material: Material,
}

const SPHERES: [Sphere; 4] = [
    Sphere {
        position: Vec3::new(0.0, 5.0, -15.0),
        radius: 5.0,
        material: Material {
            colour: Vec3::new(0.8, 0.2, 0.2),
            emission_colour: Vec3::ZERO,
            emission_strength: 0.0
        },
    },
    Sphere {
        position: Vec3::new(0.0, -5.0, -15.0),
        radius: 5.0,
        material: Material {
            colour: Vec3::new(0.2, 0.2, 0.8),
            emission_colour: Vec3::ZERO,
            emission_strength: 0.0
        },
    },
    Sphere {
        position: Vec3::new(25.0, 0.0, -15.0),
        radius: 20.0,
        material: Material {
            colour: Vec3::new(0.2, 0.8, 0.2),
            emission_colour: Vec3::ZERO,
            emission_strength: 0.0
        },
    },
    Sphere {
        position: Vec3::new(-50.0, 0.0, -12.0),
        radius: 20.0,
        material: Material {
            colour: Vec3::ZERO,
            emission_colour: Vec3::ONE,
            emission_strength: 10.0
        },
    },
];

const MAX_BOUNCE: u32 = 20;
const RAYS: u32 = 50;


fn main() {
    // let resolution = UVec2::new(3840, 2160);
    let resolution = UVec2::new(1920, 1080);
    // let resolution = UVec2::new(1280, 720);
    // let resolution = UVec2::new(400, 300);
    // let resolution = UVec2::new(100, 100);

    let aspec_ratio = resolution.x as f32 / resolution.y as f32;

    let camera_position = Vec3::new(0.0, 0.0, 1.0);

    let mut image = RgbImage::new(resolution.x, resolution.y);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let screen_position = UVec2::new(x, resolution.y - y);
        let normal_position = (screen_position.as_vec2() + 0.5) / resolution.as_vec2();
        let centerd_position = normal_position - 0.5;
        let aspect_position = centerd_position * Vec2::new(aspec_ratio, 1.0);

        let ray = Ray {
            position: camera_position,
            direction: aspect_position.extend(0.0) - camera_position,
        };

        let mut total_light = Vec3::ZERO;

        for _ in 0..RAYS {
           total_light += trace(&ray);
        }
    
        let colour = total_light / RAYS as f32;

        let rgb = colour * 255.0;
        *pixel = Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
        
        let percent = (x as f32 + y as f32 * resolution.x as f32) / (resolution.x as f32 * resolution.y as f32) * 100.0;
        println!("Percent: {:.1}%", percent);
    }

    image.save("image.png").unwrap();
}

fn trace(ray: &Ray) -> Vec3 {
    let mut ray = ray.clone();
    let mut incomming_light = Vec3::ZERO;
    let mut ray_colour = Vec3::ONE;

    for _ in 0..MAX_BOUNCE {
        if let Some(hit_info) = ray_collision(&ray) {
            ray.position = hit_info.hit_position;
            ray.direction = random_hemisphere_direction(hit_info.normal);

            let emitted_light = hit_info.material.emission_colour * hit_info.material.emission_strength;
            incomming_light += emitted_light * ray_colour;

            ray_colour *= hit_info.material.colour;
        } else {
            break;
        };
    }

    incomming_light
}

fn ray_collision(ray: &Ray) -> Option<HitInfo> {
    SPHERES
        .into_iter()
        .map(|sphere| ray_sphere(&ray, &sphere))
        .fold(None, |acc: Option<HitInfo>, optional_hit| {
            let Some(closest_hit) = &acc else { return optional_hit };
            if let Some(hit_info) = &optional_hit {
                if hit_info.distance < closest_hit.distance {
                    return optional_hit;
                }
            }
            acc
        })
}

fn ray_sphere(ray: &Ray, sphere: &Sphere) -> Option<HitInfo> {
    let offset_ray_origin = ray.position - sphere.position;

    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * offset_ray_origin.dot(ray.direction);
    let c = offset_ray_origin.dot(offset_ray_origin) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        let distance = (-b - discriminant.sqrt()) / (2.0 * a);

        if distance >= 0.0 {
            let hit_position = ray.position + ray.direction * distance;
            return Some(HitInfo {
                distance,
                hit_position,
                normal: (hit_position - sphere.position).normalize(),
                material: sphere.material.clone(),
            });
        }
    }

    None
}

fn random_normal_distribution() -> f32 {
    let theta: f32 = 2.0 * PI * random::<f32>();
    let rho = (-2.0 * random::<f32>().log2()).sqrt();
    rho * theta.cos()
}

fn random_direction() -> Vec3 {
    Vec3::new(
        random_normal_distribution(),
        random_normal_distribution(),
        random_normal_distribution(),
    )
    .normalize()
}

fn random_hemisphere_direction(normal: Vec3) -> Vec3 {
    let dir = random_direction();
    dir * normal.dot(dir).signum()
}
