use std::f32::consts::PI;

const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 40;
const THETA_SPACING: f32 = 0.07;
const PHI_SPACING: f32 = 0.02;

const R1: f32 = 1.0;
const R2: f32 = 2.0;
const K2: f32 = 5.0;

const K1: f32 = (SCREEN_WIDTH as f32) * K2 * 3.0 / (8.0 * (R1 + R2));

pub fn print_frame(output: [[char; SCREEN_WIDTH]; SCREEN_HEIGHT]) {
    println!("\x1b[H");
    for row in &output {
        for &ch in row {
            print!("{}", ch);
        }
        println!();
    }
}

pub fn render_frame(a: f32, b: f32) {
    let cos_a = a.cos();
    let sin_a = a.sin();
    let cos_b = b.cos();
    let sin_b = b.sin();

    let mut output = [[' '; SCREEN_WIDTH]; SCREEN_HEIGHT];
    let mut zbuffer = [[0.0; SCREEN_WIDTH]; SCREEN_HEIGHT];

    for theta in (0..(2.0 * PI / THETA_SPACING) as usize).map(|x| x as f32 * THETA_SPACING) {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();

        for phi in (0..(2.0 * PI / PHI_SPACING) as usize).map(|x| x as f32 * PHI_SPACING) {
            let cos_phi = phi.cos();
            let sin_phi = phi.sin();

            let circle_x = R2 + R1 * cos_theta;
            let circle_y = R1 * sin_theta;

            let x = circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi) - circle_y * cos_a * sin_b;
            let y = circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi) + circle_y * cos_a * cos_b;
            let z = K2 + cos_a * circle_x * sin_phi + circle_y * sin_a;
            let ooz = 1.0 / z;

            let xp = (SCREEN_WIDTH as f32 / 2.0 + K1 * ooz * x) as usize;
            let yp = (SCREEN_HEIGHT as f32 / 2.0 - K1 * ooz * y) as usize;

            if xp < SCREEN_WIDTH && yp < SCREEN_HEIGHT {
                let l = cos_phi * cos_theta * sin_b - cos_a * cos_theta * sin_phi - sin_a * sin_theta +
                        cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);
                if l > 0.0 {
                    if ooz > zbuffer[yp][xp] {
                        zbuffer[yp][xp] = ooz;
                        let luminance_index = (l * 8.0) as usize;
                        if luminance_index < 12 {
                            output[yp][xp] = ['.', ',', '-', '~', ':', ';', '=', '*', '!', '#', '$', '@'][luminance_index];
                        }
                    }
                }
            }
        }
    }

    print_frame(output);
}
