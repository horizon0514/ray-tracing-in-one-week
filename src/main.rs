use std::fs;

mod vector3;
use vector3::{Color, Vector3, Point3};

use crate::material::{Lambertian, Metal, Dielectric};
mod ray3;

mod sphere;
use sphere::Sphere;

mod hittable;

mod hittable_list;
use hittable_list::HittableList;

mod camera;
use camera::Camera;

mod util;
mod material;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 200_f32;

    // Camera, 位置在原点,朝向为负Z轴
    let camera = Camera::new(
        aspect_ratio, 
        20.0,
        Point3::new(12.0, 2.0,3.0), 
        Point3::new(0.0, 0.0, 0.0), 
        Vector3::new(0.0, 1.0, 0.0), 
        0.6,
        10.0,
        image_width
    );

    // Material
    let material_ground = Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    };
    
    // World
    let mut world = HittableList::new();
    let ground = Sphere::new(Vector3 { x: 0.0, y: -1000.0, z: 0.0 },Vector3 { x: 0.0, y: -1000.0, z: 0.0 }, 1000.0, material_ground);
    world.add(Box::new(ground));

    // generate random spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = util::random_double();
            let center = Point3 { x: a as f32 + 0.9 * util::random_double(), y: 0.2, z: b as f32 + 0.9 * util::random_double() };
            if (center - Point3 { x: 4.0, y: 0.2, z: 0.0 }).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::new(util::random_double() * util::random_double(), util::random_double() * util::random_double(), util::random_double() * util::random_double());
                    let center2 = center + Vector3 { x: util::random_double() * 0.1, y: util::random_double(), z: util::random_double() * 0.1 };
                    let sphere = Sphere::new(center,center2, 0.2, Lambertian { albedo });
                    world.add(Box::new(sphere));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::new(0.5 * (1.0 + util::random_double()), 0.5 * (1.0 + util::random_double()), 0.5 * (1.0 + util::random_double()));
                    let fuzz = 0.5 * util::random_double();
                    let sphere = Sphere::new(center,center.clone(), 0.2, Metal { albedo, fuzz });
                    world.add(Box::new(sphere));
                } else {
                    // glass
                    let sphere = Sphere::new(center, center,0.2, Dielectric { ir: 1.5 });
                    world.add(Box::new(sphere));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(Point3 { x: 0.0, y: 1.0, z: 0.0 }, Point3 { x: 0.0, y: 1.0, z: 0.0 },1.0, Dielectric { ir: 1.5 })));
    world.add(Box::new(Sphere::new(Point3 { x: -4.0, y: 1.0, z: 0.0 }, Point3 { x: -4.0, y: 1.0, z: 0.0 },1.0, Lambertian { albedo: Color::new(0.4, 0.2, 0.1) })));
    world.add(Box::new(Sphere::new(Point3 { x: 4.0, y: 1.0, z: 0.0 }, Point3 { x: 4.0, y: 1.0, z: 0.0 },1.0, Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 })));

    // Render
    let file_name = "image.ppm";
    let mut file = fs::File::create(file_name).unwrap();
    // write: Sphere<Lambertian> ppm file header
    

    camera.render(&mut file, &world, 500);
}
