use nannou::prelude::*;

const DAMPING: f32 = 0.95;
const DELTA_TIME: f32 = 1.0 / 60.0;
const GRAVITY: (f32, f32) = (0.0, -9.8);
const SCALE: f32 = 100.0;

#[derive(Clone, Copy)]
pub struct Object {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub mass: f32,
}

impl Object {
    pub fn new(position: Vec2, velocity: Vec2, acceleration: Vec2, mass: f32) -> Self {
        Self {
            position,
            velocity,
            acceleration,
            mass,
        }
    }

    pub fn update(&mut self) {
        let gravity_force = vec2(GRAVITY.0, GRAVITY.1) * self.mass * SCALE;
        self.apply_force(gravity_force);

        self.velocity += self.acceleration * DELTA_TIME;
        self.velocity *= DAMPING;
        self.position += self.velocity * DELTA_TIME;

        self.acceleration = Vec2::ZERO;
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force / self.mass;
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse().xy(self.position).color(RED);
    }
}

pub struct Spring {
    k: f32,
    rest_length: f32,
    anchor: Vec2,
}

impl Spring {
    pub fn new(k: f32, rest_length: f32, anchor: Vec2) -> Self {
        Self {
            k,
            rest_length,
            anchor,
        }
    }

    pub fn apply_force(&mut self, object: &mut Object) {
        let displacement = object.position - self.anchor;
        let displacement_length = displacement.length();
        let stretch = displacement_length - self.rest_length;
        let direction = displacement.normalize();
        let force = -self.k * stretch * direction;

        object.apply_force(force);
    }
}
