use amethyst::renderer::{Material, Mesh};
use amethyst::assets::Handle;
use crate::components::Enemy as EnemyComponent;

#[derive(Clone)]
pub struct EnemyResource {
    pub mesh: Handle<Mesh>,
    pub material: Material,
    pub component: EnemyComponent,
    pub num_enemies: u8,
    pub time_till_next_spawn: f32,
}