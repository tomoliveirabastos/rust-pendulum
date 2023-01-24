pub mod my_vector {
    pub struct MyVector {
        pub x: f32,
        pub y: f32,
    }

    impl MyVector {
        pub fn new(x: f32, y: f32) -> MyVector {
            MyVector { x, y }
        }

        pub fn add(&mut self, other: &MyVector) -> &MyVector {
            self.x += other.x;
            self.y += other.y;

            self
        }

        pub fn set(&mut self, x: f32, y: f32) -> &MyVector {
            self.x = x;
            self.y = y;

            self
        }
    }
}
