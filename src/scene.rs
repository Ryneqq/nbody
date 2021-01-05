use crate::body::Body;
use crate::{Vector, Point};
use itertools::Itertools;
use std::cmp::Ordering;
use rand::{thread_rng, Rng};

trait f64Ext {
    fn cmp(&self, other: &Self) -> Ordering;
}

impl f64Ext for f64 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self < other {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}


const V: f64 = 2f64;
pub struct Scene {
    pub bodies: Vec<Body>,
}

impl Scene {
    pub fn new() -> Self {
        let bodies = (0..30).map(|_| Self::random_body()).chain(Some(Self::black_hole())).collect();

        Self { bodies }
    }

    fn random_body() -> Body {
        let mut rng = thread_rng();
        let mass = rng.gen_range(10f64.powi(4), 10f64.powi(13));
        let x = rng.gen_range(0.0, 1200.0);
        let y = rng.gen_range(0.0, 900.0);
        let position = Point::new(x,y);
        let v_x = rng.gen_range(-V, V);
        let v_y = rng.gen_range(-V, V);
        let velocity = Vector::new(v_x, v_y);

        Body::new(mass, position, velocity)
    }

    fn black_hole() -> Body {
        let mass = 10f64.powi(15);
        let position = Point::new(600f64, 450f64);
        let velocity = Vector::new(0f64, 0f64);

        Body::new(mass, position, velocity)
    }

    pub fn update(&mut self) {
        let new_bodies = self.bodies.iter()
            .map(self.update_body)
            .sorted_by(|(ma, _), (mb, _)| ma.cmp(mb))
            .map(|(_, a)| a)
            .coalesce(|a, b| {
                if a.colliding(&b) {
                    Ok(a.merge(b))
                } else {
                    Err((a, b))
                }
            })
            .collect();

        self.bodies = new_bodies;
    }

    fn update_body(&self, body: &Body) -> (&Body, Body) {
        let mut new_body = body.clone();
        self.bodies.iter()
            .filter(|b| *b != body)
            .for_each(|b| new_body.apply_gravity(&b));
        let closest = self.bodies.iter()
            .sorted_by(|a, b| body.distance(a).cmp(&body.distance(b)))
            .map(|b| body.distance(b))
            .next()
            .unwrap();
        new_body.step();

        (closest, new_body)
    }
}
