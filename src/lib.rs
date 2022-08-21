use wasm_bindgen::prelude::*;
mod utils;
mod vec2d;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Rules {
    speed: f32,
    scale: f32,
    radius: f32,
    separation: f32,
    alignment: f32,
    cohesion: f32,
    wall_separation: f32,
    mouse_interaction: f32,
}

#[wasm_bindgen]
impl Rules {
    pub fn new(
        speed: f32,
        scale: f32,
        radius: f32,
        separation: f32,
        alignment: f32,
        cohesion: f32,
        wall_separation: f32,
        mouse_interaction: f32,
    ) -> Rules {
        Rules {
            speed,
            scale,
            radius,
            separation,
            alignment,
            cohesion,
            wall_separation,
            mouse_interaction,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Boid {
    coords: vec2d::Vec2d,
    rules: Rules,
    rotation: f32,
}

impl Boid {
    // TODO Move to round-radius-detection
    fn find_near(&self, boids: &Vec<Boid>) -> Vec<Boid> {
        let mut results: Vec<Boid> = Vec::new();

        for boid in boids {
            let is_close_x = boid.coords.x < (self.coords.x + self.rules.radius)
                && boid.coords.x > (self.coords.x - self.rules.radius);
            let is_close_y = boid.coords.y < (self.coords.y + self.rules.radius)
                && boid.coords.y > (self.coords.y - self.rules.radius);
            let is_self = boid.rotation == self.rotation
                && boid.coords.x == self.coords.x
                && boid.coords.y == self.coords.y;

            if is_close_x && is_close_y && !is_self {
                results.push(*boid);
            }
        }

        results
    }

    fn separation(&self, near: &Vec<Boid>) -> vec2d::Vec2d {
        let mut vec = vec2d::Vec2d::new(0., 0.);

        for boid in near {
            vec = self.coords - boid.coords;
            let len = vec.len();
            vec = vec / len;
            vec = vec * (1.0 / len);
        }

        vec * self.rules.separation
    }

    fn alignment(&self, near: &Vec<Boid>) -> vec2d::Vec2d {
        let mut vec = vec2d::Vec2d::new(0., 0.);

        let len = near.len();
        if len != 0 {
            for boid in near {
                vec += vec2d::Vec2d::from_rotation(boid.rotation) * self.rules.speed;
            }
            vec /= len as f32;
        }

        vec.normalize() * self.rules.alignment
    }

    fn cohesion(&self, near: &Vec<Boid>) -> vec2d::Vec2d {
        let mut vec = vec2d::Vec2d::new(0., 0.);

        let len = near.len();
        if len != 0 {
            for boid in near {
                vec += boid.coords;
            }
            vec /= len as f32;

            vec -= self.coords;
        }

        vec.normalize() * self.rules.cohesion
    }

    fn separate_from_walls(&self, width: f32, height: f32) -> vec2d::Vec2d {
        let mut vec = vec2d::Vec2d::new(0., 0.);

        let wall_radius = self.rules.radius;
        if self.coords.x < wall_radius {
            vec.x = wall_radius - self.coords.x;
        } else if self.coords.x + wall_radius > width {
            vec.x = width - self.coords.x - wall_radius;
        }

        if self.coords.y < wall_radius {
            vec.y = wall_radius - self.coords.y;
        } else if self.coords.y + wall_radius > height {
            vec.y = height - self.coords.y - wall_radius;
        }

        vec / wall_radius * self.rules.wall_separation
    }

    fn mouse_interaction(&self, mx: f32, my: f32) -> vec2d::Vec2d {
        if mx != 0.0 && my != 0.0 {
            let vec = vec2d::Vec2d::new(mx, my) - self.coords;
            let rad = self.rules.radius * 2.0;

            if vec.x < rad && vec.x > -rad && vec.y < rad && vec.y > -rad {
                return vec * self.rules.mouse_interaction;
            }
        }

        return vec2d::Vec2d::new(0., 0.);
    }

    fn teleport_in_bounds(&mut self, width: f32, height: f32) {
        if self.coords.x > width {
            self.coords.x = 1.0;
        } else if self.coords.x < 0.0 {
            self.coords.x = width;
        }

        if self.coords.y > height {
            self.coords.y = 1.0;
        } else if self.coords.y < 0.0 {
            self.coords.y = height;
        }
    }

    fn flock(&mut self, boids: &Vec<Boid>, width: f32, height: f32, mx: f32, my: f32) {
        let near = self.find_near(boids);

        let vec = vec2d::Vec2d::from_rotation(self.rotation)
            + self.separation(&near)
            + self.alignment(&near)
            + self.cohesion(&near)
            + self.separate_from_walls(width, height)
            + self.mouse_interaction(mx, my);

        self.coords += vec * self.rules.speed;
        self.rotation = vec.rotation();

        self.teleport_in_bounds(width, height);
    }
}

#[wasm_bindgen]
impl Boid {
    pub fn x(&mut self) -> u16 {
        self.coords.x as u16
    }

    pub fn y(&mut self) -> u16 {
        self.coords.y as u16
    }

    pub fn rotation(&mut self) -> f32 {
        self.rotation
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u16,
    height: u16,
    boids: Vec<Boid>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u16, height: u16) -> Universe {
        Universe {
            width,
            height,
            boids: Vec::new(),
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn tick(&mut self, mx: u16, my: u16) {
        let boids_copy = self.boids.clone();
        for boid in &mut self.boids {
            boid.flock(
                &boids_copy,
                self.width as f32,
                self.height as f32,
                mx as f32,
                my as f32,
            );
        }
    }

    pub fn create_boid(&mut self, x: u16, y: u16, rotation: f32, rules: Rules) {
        let boid = Boid {
            coords: vec2d::Vec2d::new(x as f32, y as f32),
            rotation,
            rules,
        };
        self.boids.push(boid);
    }

    pub fn get_boid(&self, index: usize) -> Boid {
        self.boids[index]
    }
}
