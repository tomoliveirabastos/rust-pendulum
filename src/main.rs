mod my_vector;

use my_vector::my_vector::Vector;

use rand::distributions::{Distribution, Uniform};
use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    let mut vec: Vec<Pendulum> = Vec::new();
    let mut rng = rand::thread_rng();

    for _i in 0..5 {
        let die: Uniform<f32> = Uniform::from(1.0..400.0);

        let n = die.sample(&mut rng);

        let my_pendulum = Pendulum::new(300.0, 0.0, n);
        vec.push(my_pendulum);
    }

    let window = Window::new_centered("Title", (640, 480)).unwrap();

    let my_handler = MyWindowHandler { p: vec };

    window.run_loop(my_handler);
}
struct MyWindowHandler {
    p: Vec<Pendulum>,
}

impl WindowHandler for MyWindowHandler {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::BLUE);

        for p in self.p.iter_mut() {
            p.update();

            p.draw(graphics);
        }

        helper.request_redraw();
    }
}

struct Pendulum {
    origin: Vector,
    position: Vector,
    angle: f32,
    angular_velocity: f32,
    angular_acceleration: f32,
    r: f32,
    m: f32,
    g: f32,
}

impl Pendulum {
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            origin: Vector::new(x, y),
            angle: 1.0,
            position: Vector::new(0.0, 0.0),
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            r: r,
            m: 1.0,
            g: 1.5,
        }
    }

    fn update(&mut self) {
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;
        self.angular_velocity += self.angular_acceleration;

        self.angle += self.angular_velocity;

        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        self.position.add(&self.origin);
    }

    fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED);
    }
}


