use speedy2d::{Graphics2D, color::Color};

use crate::my_vector::my_vector::MyVector;

pub struct Pendulum {
    pub origin: MyVector,
    pub position: MyVector,
    pub angle: f32,
    pub angular_velocity: f32,
    pub angular_acceleration: f32,
    pub r: f32,
    pub m: f32,
    pub g: f32,
}

impl Pendulum {
    pub fn new(x: f32, y: f32, r: f32) -> Pendulum {
        Pendulum {
            origin: MyVector::new(x, y),
            angle: 1.0,
            position: MyVector::new(0.0, 0.0),
            angular_velocity: 0.0,
            angular_acceleration: 0.0,
            r: r,
            m: 1.0,
            g: 1.5,
        }
    }

    pub fn update(&mut self) {
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;
        self.angular_velocity += self.angular_acceleration;

        self.angle += self.angular_velocity;

        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        self.position.add(&self.origin);
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::RED,
        );

        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::RED);
    }
}
