use bevy::prelude::*;

pub fn min_max(a: f32, b: f32) -> (f32, f32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

pub fn manhattan_distance(origin: Vec3, dest: Vec3) -> (f32, f32) {
    (dest.x - origin.x, dest.z - origin.z)
}

pub fn min_max_iter(a: f32, b: f32) -> MinMaxResult {
    let (min, max) = min_max(a, b);
    MinMaxResult { min, max }
}

pub struct MinMaxResult {
    min: f32,
    max: f32,
}

impl Iterator for MinMaxResult {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.min < self.max {
            let a = self.min;
            self.min += 1.0;
            Some(a)
        } else {
            None
        }
    }
}
