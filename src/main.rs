use macroquad::prelude::*;

const PI: f32 = 3.1415926536;

fn pendulum_draw(height: f32, width: f32) -> [f32; 4] {
    let l = height / 5.0;
    let w = width / 100.0;
    let x0 = width / 2.0;
    let y0 = height / 2.0;

    [l, w, x0, y0]
}

#[macroquad::main("Simple Pendulum")]
async fn main() {
    let mut theta: f32 = 1.4;
    let grav = 1.0;
    let dt = 0.1;
    let mass = 1.2;
    let angular_velocity = 0.1;

    loop {
        //Reset theta
        if theta > 2.0 * PI {
            theta = 0.0;
        }

        clear_background(LIGHTGRAY);

        let par = pendulum_draw(screen_height(), screen_width());

        //Pivot
        let w = par[1];
        let x0 = par[2];
        let y0 = par[3];

        let l = par[0];

        //Energy calculation
        //KE = 1/2 * mass * v^2 * length^2
        let kinetic_energy = 0.5 * mass * (angular_velocity * angular_velocity) * (l * l);

        let potential_energy = mass * grav * l * (1.0 - theta.cos());

        let total_energy = kinetic_energy + potential_energy;
        //Angular velocity calculation
        let angular_accel = grav * (theta.sin() / l);
        let angular_velocity = ((2.0 * (total_energy - mass) * angular_accel * l * theta.sin())
            / (mass * l * l))
            .sqrt();

        println!("{}", angular_velocity);
        let new_angle = theta + angular_velocity * dt;
        theta = new_angle;

        draw_circle(x0, y0, w, BLACK);

        //Inextensible string
        let x1 = x0 + l * (theta.cos() as f32);
        let y1 = y0 + l * (theta.sin() as f32);

        draw_line(x0, y0, x1, y1, w / 2.0, BLACK);

        //Mass
        draw_circle(x1, y1, w, RED);

        let fps = get_fps().to_string();
        let b = format!("FPS: {}", fps);
        draw_text(&b, 50.0, 50.0, 20.0, BLACK);

        let angle = theta.to_string();
        let rad = format!("Angle: {}", angle);
        draw_text(&rad, 50.0, 70.0, 20.0, BLACK);

        let energy = total_energy.to_string();
        let text = format!("Total Energy: {}", energy);
        draw_text(&text, 50.0, 90.0, 20.0, BLACK);

        next_frame().await
    }
}
