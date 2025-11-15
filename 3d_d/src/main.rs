// main.rs (regenerado con correcciones solicitadas)
// NOTA: Reemplaza este archivo completo por el generado aquí.
// Si tienes otros módulos (matrix.rs, framebuffer.rs, etc.) no se modifican.

#![allow(dead_code)]

use rand::Rng;
use raylib::prelude::*;
use std::f32::consts::PI;

mod framebuffer;
mod line;
mod matrix;
mod obj;
mod shaders;
mod triangle;
mod vertex;

use framebuffer::Framebuffer;

// ==========================================================
// Estructuras principales
// ==========================================================

#[derive(Clone)]
struct Body {
    name: String,
    radius: f32,
    color: Vec3,
    orbit_radius: f32,
    orbit_speed: f32,
    angle_orbit: f32,
    self_rot_speed: f32,
    angle_rotation: f32,
    parent: Option<usize>,
}

struct Camera2D {
    focus_index: usize,
    offset_x: f32,
    zoom: f32,
    global_rotation: f32,
}

impl Camera2D {
    fn world_to_screen(&self, world_x: f32, world_y: f32, sw: i32, sh: i32) -> (i32, i32) {
        let (s, c) = self.global_rotation.sin_cos();
        let rx = world_x * c - world_y * s;
        let ry = world_x * s + world_y * c;

        let cx = sw as f32 * 0.5;
        let cy = sh as f32 * 0.5;

        let sx = cx + (rx - self.offset_x) * self.zoom;
        let sy = cy + ry * self.zoom;
        (sx as i32, sy as i32)
    }
}

// ==========================================================
// Dibujo de un círculo lleno en el framebuffer
// ==========================================================

fn draw_filled_circle(fb: &mut Framebuffer, x0: i32, y0: i32, r: i32, color: Vec3) {
    for dy in -r..=r {
        for dx in -r..=r {
            if dx * dx + dy * dy <= r * r {
                fb.set_pixel(x0 + dx, y0 + dy, color);
            }
        }
    }
}

// ==========================================================
// Cráteres (corregido)
// ==========================================================

fn draw_craters(fb: &mut Framebuffer, cx: i32, cy: i32, radius: i32) {
    let mut rng = rand::thread_rng();

    for _ in 0..6 {
        let crater_r = rng.gen_range(2..(radius / 3).max(3));

        let theta = rng.r#gen::<f32>() * 2.0 * PI;
        let dist = rng.r#gen::<f32>().sqrt() * (radius as f32 * 0.6);

        let x = cx + (theta.cos() * dist) as i32;
        let y = cy + (theta.sin() * dist) as i32;

        let crater_color = Vec3::new(0.7, 0.7, 0.65);
        draw_filled_circle(fb, x, y, crater_r, crater_color);
    }
}

// ==========================================================
// Cálculo de posiciones orbitales
// ==========================================================

fn compute_world_position(bodies: &[Body], index: usize) -> (f32, f32) {
    let b = &bodies[index];
    let (_x, _y) = (0.0, 0.0);

    if let Some(p) = b.parent {
        let (px, py) = compute_world_position(bodies, p);
        let ox = b.angle_orbit.cos() * b.orbit_radius;
        let oy = b.angle_orbit.sin() * b.orbit_radius;
        (px + ox, py + oy)
    } else {
        (0.0, 0.0)
    }
}

// ==========================================================
// Input
// ==========================================================

fn handle_input_sim(rl: &mut RaylibHandle, cam: &mut Camera2D) {
    if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        cam.global_rotation += 0.02;
    }
    if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        cam.global_rotation -= 0.02;
    }

    if rl.is_key_down(KeyboardKey::KEY_Z) {
        cam.zoom *= 1.02;
    }
    if rl.is_key_down(KeyboardKey::KEY_X) {
        cam.zoom /= 1.02;
    }
}

// ==========================================================
// Main: Simulación
// ==========================================================

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1200, 900)
        .title("Sistema Solar 2D - Rust")
        .build();

    let mut fb = Framebuffer::new(1200, 900);

    let mut bodies = vec![
        Body {
            name: "Estrella".into(),
            radius: 50.0,
            color: Vec3::new(1.0, 1.0, 0.2),
            orbit_radius: 0.0,
            orbit_speed: 0.0,
            angle_orbit: 0.0,
            self_rot_speed: 0.01,
            angle_rotation: 0.0,
            parent: None,
        },
        Body {
            name: "PlanetaA".into(),
            radius: 24.0,
            color: Vec3::new(0.3, 0.6, 1.0),
            orbit_radius: 210.0,
            orbit_speed: 0.004,
            angle_orbit: 0.0,
            self_rot_speed: 0.015,
            angle_rotation: 0.0,
            parent: Some(0),
        },
        Body {
            name: "LunaA".into(),
            radius: 8.0,
            color: Vec3::new(0.6, 0.6, 0.6),
            orbit_radius: 48.0,
            orbit_speed: 0.03,
            angle_orbit: 0.0,
            self_rot_speed: 0.02,
            angle_rotation: 0.0,
            parent: Some(1),
        },
    ];

    let mut cam = Camera2D {
        focus_index: 0,
        offset_x: 0.0,
        zoom: 1.0,
        global_rotation: 0.0,
    };

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let sw = rl.get_screen_width();
        let sh = rl.get_screen_height();

        // input
        handle_input_sim(&mut rl, &mut cam);

        // update bodies
        for b in bodies.iter_mut() {
            b.angle_orbit += b.orbit_speed;
            b.angle_rotation += b.self_rot_speed;
        }

        // render
        fb.clear(Vec3::new(0.02, 0.02, 0.07));

        for (i, body) in bodies.iter().enumerate() {
            let (wx, wy) = compute_world_position(&bodies, i);
            let (sx, sy) = cam.world_to_screen(wx, wy, sw, sh);
            let r = (body.radius * cam.zoom) as i32;

            draw_filled_circle(&mut fb, sx, sy, r, body.color);

            if body.name.contains("Planeta") {
                draw_craters(&mut fb, sx, sy, r);
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        fb.draw_to(&mut d);
    }
}
