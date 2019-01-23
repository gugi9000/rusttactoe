use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::prelude::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    right: bool,
    offset: f64,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREY: [f32; 4] = [0.1, 0.3, 0.3, 0.7];
        const BLUE: [f32; 4] = [0.1, 0.1, 1.0, 0.7];
        const RED: [f32; 4] = [1.0, 0.3, 0.3, 0.7];
        const GREEN: [f32; 4] = [0.1, 1.0, 0.3, 0.7];
        const WHITE: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

        let hline = rectangle::square(0.0, 0.0, 650.0);

        // let right = self.right;
        let rotation = self.rotation;
        let offset = self.offset;

        let (x, y) = (250.0, 50.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(WHITE, gl);

            let transform_h1 = c
                .transform
                .trans(x + offset, y)
                .rot_rad(13.30 + rotation)
                .trans(-25.0, -25.0);
            let transform_h2 = c
                .transform
                .trans(x + offset + 350.0, y)
                .rot_rad(13.40 + rotation)
                .trans(-25.0, -25.0);
            let transform_v1 = c
                .transform
                .trans(x - 200.0 + offset, y + 300.0 + offset)
                .rot_rad(5.4 + rotation)
                .trans(-25.0, -25.0);
            let transform_v2 = c
                .transform
                .trans(x - 200.0, y + 650.0 + offset)
                .rot_rad(5.30 + rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            // dbg!(rotation);
            line(RED, 10.0, hline, transform_h1, gl);
            line(GREEN, 10.0, hline, transform_h2, gl);
            line(BLUE, 10.0, hline, transform_v1, gl);
            line(GREY, 10.0, hline, transform_v2, gl)
        });
    }

    fn update(&mut self, args: UpdateArgs) {
        // Rotate 0.02 radians  offset -.04 per second

        if self.rotation > 0.04 {
            self.right = false;
        }
        if self.rotation < -0.04 {
            self.right = true;
        }

        if self.right {
            self.rotation += 0.02 * args.dt
        } else {
            self.rotation -= 0.02 * args.dt
        }
        let mut rng = rand::thread_rng();
        let y: f64 = rng.gen(); // generates a float between 0 and 1

        self.offset += y - 0.5;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("rust tac toe", [900, 900])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        right: true,
        offset: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(u);
        }
    }
}
