use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct Projectile {
    /// The speed of our laser beam
    pub velocity: f32,
    /// The width of our laser beam
    pub width: f32,
    /// The height of our laser beam
    pub height: f32,

    pub rise: f32,

    pub run: f32,
    
}

impl Component for Projectile { 
    type Storage = DenseVecStorage<Self>;
}

  