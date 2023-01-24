mod my_vector;
mod my_pendulum;

use my_pendulum::Pendulum;

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
