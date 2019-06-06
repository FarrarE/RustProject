use amethyst::{
    // --snip--
    ecs::prelude::{Component, DenseVecStorage, Entity},
};
 
#[derive(Default)]
pub struct HeadsUpDisplay{
    pub score: i32,
}

pub struct ScoreText {
    pub score: Entity,
}