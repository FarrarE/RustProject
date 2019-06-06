use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, Entity},
};
 
#[derive(Default)]
pub struct UI{
    pub score: i32,
}

pub struct ScoreText {
    pub score: Entity,
}