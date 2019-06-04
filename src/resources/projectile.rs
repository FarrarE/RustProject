use crate::components::Projectile as ProjectileComponent;
use amethyst::assets::Handle;
use amethyst::renderer::{Material, Mesh};

#[derive(Clone)]
pub struct ProjectileResource {
    pub mesh: Handle<Mesh>,
    pub material: Material,
    pub component: ProjectileComponent,
}
