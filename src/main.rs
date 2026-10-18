use macroquad::prelude::*;

#[derive(Clone)]
struct Boid {
    pos: Vec2,
    vel: Vec2,
}

#[macroquad::main("Boids Simulation")]
async fn main() {
    let mut boids = Vec::new();
    let num_boids = 150;

    // Spawn them in the middle of the screen in a tight circle
    for i in 0..num_boids {
        let angle = i as f32 * 0.5;
        let x = screen_width() / 2.0 + angle.cos() * 100.0;
        let y = screen_height() / 2.0 + angle.sin() * 100.0;
        
        boids.push(Boid {
            pos: vec2(x, y),
            vel: vec2(rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0)),
        });
    }

    loop {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_pos = vec2(mouse_x, mouse_y);

        let mut new_boids = boids.clone();

        for i in 0..new_boids.len() {
            let mut sep = Vec2::ZERO;
            let mut ali = Vec2::ZERO;
            let mut coh = Vec2::ZERO;
            let mut count = 0;

            // Look at nearby boids
            for j in 0..boids.len() {
                if i != j {
                    let dist = boids[i].pos.distance(boids[j].pos);
                    if dist < 60.0 { 
                        sep += (boids[i].pos - boids[j].pos).normalize() / dist;
                        ali += boids[j].vel;
                        coh += boids[j].pos;
                        count += 1;
                    }
                }
            }

            // Apply flocking rules
            if count > 0 {
                ali /= count as f32;
                coh /= count as f32;

                new_boids[i].vel += sep * 0.05;                 // Separation (avoid)
                new_boids[i].vel += (ali - boids[i].vel) * 0.05; // Alignment (match speed)
                new_boids[i].vel += (coh - boids[i].pos) * 0.001; // Cohesion (stick together)
            }

            // Run away from the mouse!
            let mouse_dist = boids[i].pos.distance(mouse_pos);
            if mouse_dist < 150.0 {
                new_boids[i].vel += (boids[i].pos - mouse_pos).normalize() * 0.2;
            }

            // Limit speed so they don't fly away instantly
            let max_speed = 3.0;
            if new_boids[i].vel.length() > max_speed {
                new_boids[i].vel = new_boids[i].vel.normalize() * max_speed;
            }

            // Move them
            let v = new_boids[i].vel;
            new_boids[i].pos += v;

            // Bounce off walls instead of wrapping around
            if new_boids[i].pos.x < 0.0 || new_boids[i].pos.x > screen_width() { new_boids[i].vel.x *= -1.0; }
            if new_boids[i].pos.y < 0.0 || new_boids[i].pos.y > screen_height() { new_boids[i].vel.y *= -1.0; }
        }

        boids = new_boids;

        // Draw
        clear_background(BLACK);

        for boid in &boids {
            // Draw a circle for the boid
            draw_circle(boid.pos.x, boid.pos.y, 2.0, SKYBLUE);
        }

        draw_text("Boids Simulation", 10.0, 30.0, 30.0, WHITE);
        draw_text("Move mouse to scare them", 10.0, 55.0, 20.0, GRAY);

        next_frame().await
    }
}