use amethyst::renderer::{Material, Mesh};
use amethyst::assets::Handle;
use crate::components::Projectile as ProjectileComponent;

#[derive(Clone)]
pub struct ProjectileResource {
    pub mesh: Handle<Mesh>,
    pub material: Material,
    pub component: ProjectileComponent,
}