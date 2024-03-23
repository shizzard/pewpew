use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MovementXBound {
    pub left: f32,
    pub right: f32,
}
impl From<(f32, f32)> for MovementXBound {
    fn from(value: (f32, f32)) -> Self {
        Self {
            left: value.0,
            right: value.1,
        }
    }
}

#[derive(Component, Debug)]
pub struct Speed(f32);
impl From<f32> for Speed {
    fn from(value: f32) -> Self {
        Self(value)
    }
}
impl From<Speed> for f32 {
    fn from(val: Speed) -> Self {
        val.0
    }
}

#[derive(Component, Debug)]
pub struct MovableX {
    pub bound: MovementXBound,
    pub speed: Speed,
}

impl MovableX {
    pub fn move_left(&self, transform: &mut Transform, timer: &Res<Time>) {
        let new_x: f32 = transform.translation.x - self.speed.0 * timer.delta_seconds();
        transform.translation.x = new_x.max(self.bound.left);
    }

    pub fn move_right(&self, transform: &mut Transform, timer: &Res<Time>) {
        let new_x: f32 = transform.translation.x + self.speed.0 * timer.delta_seconds();
        transform.translation.x = new_x.min(self.bound.right);
    }
}
