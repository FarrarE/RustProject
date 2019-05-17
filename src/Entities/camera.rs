use amethyst::prelude::Builder;
use amethyst::ecs::prelude::{Entity, World};
use amethyst::core::transform::GlobalTransform;
use amethyst::renderer::{Camera, Projection};
use amethyst::core::nalgebra::{Matrix4, Vector3};

//Create a camera entity
fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}