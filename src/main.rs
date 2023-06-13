use glam::{UVec2, Vec2, Vec3};
use image::{Rgb, RgbImage};

#[derive(Clone)]
struct Material {
    colour: Vec3,
}

struct Sphere {
    position: Vec3,
    radius: f32,
    material: Material,
}

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

const SPHERES: [Sphere; 2] = [
    Sphere {
        position: Vec3::new(0.0, 0.0, -15.0),
        radius: 5.0,
        material: Material {
            colour: Vec3::new(0.8, 0.2, 0.2),
        },
    },
    Sphere {
        position: Vec3::new(-10.0, 0.0, -15.0),
        radius: 3.0,
        material: Material {
            colour: Vec3::new(0.2, 0.2, 0.8),
        },
    },
];

fn main() {
    // let resolution = UVec2::new(3840, 2160);
    // let resolution = UVec2::new(1920, 1080);
    let resolution = UVec2::new(1280, 720);
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

        let hit = ray_collision(ray);

        let colour = match hit {
            Some(hit_info) => {
                // hit_info.normal * 0.5 + 0.5
                hit_info.material.colour
            }
            None => Vec3::ZERO,
        };

        let rgb = colour * 255.0;
        *pixel = Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
    }

    image.save("image.png").unwrap();
}

fn ray_collision(ray: Ray) -> Option<HitInfo> {
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

// fn random_normal_distribution() -> f32 {

// }

// fn random_direction() -> Vec3 {

// }
