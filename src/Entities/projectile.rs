use amethyst::ecs::prelude::{World, Entities, Entity, LazyUpdate, ReadExpect};
use amethyst::core::transform::{Transform};
use amethyst::core::nalgebra::Vector3;

use super::png_mesh_and_mat;
use crate::config::GAME_CONFIGURATION;
use crate::components::Projectile as ProjectileComponent;
use crate::resources::ProjectileResource;

pub fn initialise_projectile_resource(world: &mut World) -> ProjectileResource {
    let (mesh, material) = png_mesh_and_mat("projectile.png", [20.0,20.0], world);

    let projectile_resource = ProjectileResource {
        mesh,
        material,
        component: ProjectileComponent {
            velocity: GAME_CONFIGURATION.projectile_velocity,
            width: 20.0,
            height: 20.0,
        },
    };
    world.add_resource(projectile_resource.clone());
    projectile_resource
}

pub fn fire_projectile(
    entities: &Entities,
    projectile_resource: &ReadExpect<ProjectileResource>,
    fire_position: Vector3<f32>,
    lazy_update: &ReadExpect<LazyUpdate>,
    rise: f32,
    run: f32)
{
    let projectile_entity:Entity = entities.create();
    let local_transform = {
        let mut local_transform = Transform::default();
        local_transform.set_position(fire_position);
        // the fire position actually represents the middle of our projectile. Adjust accordingly.
        let p = local_transform.translation()[0];
        local_transform.set_x(p - projectile_resource.component.width / 2.0);
        local_transform
    };
    lazy_update.insert(projectile_entity, projectile_resource.material.clone());
    lazy_update.insert(projectile_entity, projectile_resource.mesh.clone());
    lazy_update.insert(projectile_entity, projectile_resource.component.clone());
    lazy_update.insert(projectile_entity, local_transform);
}