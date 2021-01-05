use crate::{Vector, Point, Float};

const G: Float = 6.6743015 * (1. / 100_000_000_000.); // m3 / (s2 * kg)
const TIME_DELTA: Float = 0.0000000000001; // s

#[derive(Debug, Clone, PartialEq)]
pub struct Body {
    id: usize,
    mass: Float,
    radius: Float,
    position: Point,
    velocity: Vector,
}

impl Body {
    pub fn new(id: usize, mass: Float, position: Point, velocity: Vector) -> Self {
        Body {
            id,
            mass,
            position,
            velocity,
            radius: Self::calc_radius(mass),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn mass(&self) -> Float {
        self.mass
    }

    pub fn radius(&self) -> Float {
        self.radius
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn apply(&mut self, force: Vector) {
        // println!("Fource of {}: {}", self.id, force);
        self.velocity = self.velocity + (force / self.mass) * TIME_DELTA;
        // println!("Velocity: {}", self.velocity);
    }

    pub fn apply_gravity(&mut self, other: &Self) {
        let force = self.gravity_force(other);

        self.apply(force);
    }

    pub fn gravity_force(&self, other: &Self) -> Vector {
        let distance = self.distance(other);
        // println!("distance between {} and {} is {}", self.id, other.id, distance);
        let gravity = G * ((self.mass * other.mass) / distance.powi(2));
        let shift = (other.position - self.position);
        let direction = shift.normalize();
        // let direction = shift * 0.0001;

        direction * gravity
    }

    pub fn distance(&self, other: &Self) -> Float {
        let x = self.position.x() - other.position.x();
        let y = self.position.y() - other.position.y();
        let z = self.position.z() - other.position.z();

        Float::sqrt(x.powi(2) + y.powi(2) + z.powi(2))
    }

    pub fn merge(self, other: Self) -> Self {
        let sum_mass = self.mass + other.mass;
        let position = if self.mass > other.mass { self.position } else { other.position };
        let velocity = self.mass * self.velocity + other.mass * other.velocity;
        let velocity = velocity / sum_mass;

        Body::new(self.id, sum_mass, position, velocity)
    }

    pub fn colliding(&self, other: &Self) -> bool {
        self.distance(other) < self.radius + other.radius
    }

    pub fn calc_radius(mass: Float) -> Float {
        (mass / 10_000f32).ln() / 500f32
    }

    pub fn step(&mut self) {
        self.position = self.position + self.velocity;
    }
}
