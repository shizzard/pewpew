use bevy::prelude::*;

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct MovementBound {
    pub min: f32,
    pub max: f32,
}
impl From<(f32, f32)> for MovementBound {
    fn from(value: (f32, f32)) -> Self {
        Self {
            min: value.0,
            max: value.1,
        }
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
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

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub enum MovementDirectionX {
    #[default]
    Left,
    Right,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct MovableX {
    pub bound: MovementBound,
    pub speed: Speed,
}

impl MovableX {
    pub fn can_move_left(&self, transform: &mut Transform) -> bool {
        &transform.translation.x > &self.bound.min
    }

    pub fn can_move_right(&self, transform: &mut Transform) -> bool {
        &transform.translation.x < &self.bound.max
    }

    pub fn move_left(&self, transform: &mut Transform, timer: &Res<Time>) {
        let new_x: f32 = transform.translation.x - self.speed.0 * timer.delta_seconds();
        transform.translation.x = new_x.max(self.bound.min);
    }

    pub fn move_right(&self, transform: &mut Transform, timer: &Res<Time>) {
        let new_x: f32 = transform.translation.x + self.speed.0 * timer.delta_seconds();
        transform.translation.x = new_x.min(self.bound.max);
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct MovableY {
    pub bound: MovementBound,
    pub speed: Speed,
}

impl MovableY {
    pub fn can_move_down(&self, transform: &mut Transform) -> bool {
        &transform.translation.y > &self.bound.min
    }

    pub fn can_move_up(&self, transform: &mut Transform) -> bool {
        &transform.translation.y < &self.bound.max
    }

    pub fn move_down(&self, transform: &mut Transform, timer: &Res<Time>) {
        let new_y: f32 = transform.translation.y - self.speed.0 * timer.delta_seconds();
        transform.translation.y = new_y.max(self.bound.min);
    }

    pub fn move_up(&self, transform: &mut Transform, timer: &Res<Time>) {
        let new_y: f32 = transform.translation.y + self.speed.0 * timer.delta_seconds();
        transform.translation.y = new_y.min(self.bound.max);
    }
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component)]
pub struct EntitySize {
    pub vec: Vec2,
}
impl From<Vec2> for EntitySize {
    fn from(value: Vec2) -> Self {
        Self { vec: value }
    }
}
impl Into<Vec2> for EntitySize {
    fn into(self) -> Vec2 {
        self.vec
    }
}
impl From<(f32, f32)> for EntitySize {
    fn from(value: (f32, f32)) -> Self {
        Vec2 {
            x: value.0,
            y: value.1,
        }
        .into()
    }
}
