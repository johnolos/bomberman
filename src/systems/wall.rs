use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, ReadExpect, System, WriteStorage},
    renderer::{SpriteRender},
};

use crate::{
    wall::Wall,
    wall_resource::WallResource
};

pub struct WallSystem {
    initialized: bool
}

impl WallSystem {
    pub fn default() -> Self {
        WallSystem {
            initialized: false
        }
    }

    fn create_initial_wall_boundary<'s>(
        &mut self,
        entities: &Entities<'s>,
        walls: &mut WriteStorage<'s, Wall>,
        transforms: &mut WriteStorage<'s, Transform>,
        sprite_renders: &mut WriteStorage<'s, SpriteRender>,
        wall_resource: &ReadExpect<'s, WallResource>,
    ) {
        let y_values = vec![0, 14];

        for x in 0..15 {
            for y in y_values.iter() {
                let mut transform = Transform::default();
                transform.translate_xyz(x as f32 * 128.0, *y as f32 * 128.0, 0.0);

                let sprite_render = SpriteRender {
                    sprite_sheet: wall_resource.sprite_sheet.clone(),
                    sprite_number: 0,
                };

                entities
                    .build_entity()
                    .with(sprite_render, sprite_renders)
                    .with(
                        Wall {},
                        walls
                    )
                    .with(transform, transforms)
                    .build();
            }
        }

        let x_values = vec![0, 14];

        for x in x_values.iter() {
            for y in 1..14 {
                let mut transform = Transform::default();
                transform.translate_xyz(*x as f32 * 128.0, y as f32 * 128.0, 0.0);

                let sprite_render = SpriteRender {
                    sprite_sheet: wall_resource.sprite_sheet.clone(),
                    sprite_number: 0,
                };

                entities
                    .build_entity()
                    .with(sprite_render, sprite_renders)
                    .with(
                        Wall {},
                        walls
                    )
                    .with(transform, transforms)
                    .build();
            }
        }
    }
}

impl<'s> System<'s> for WallSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Wall>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, WallResource>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut walls,
            mut transforms,
            mut sprite_renders,
            wall_resources,
        ) = data;

        // FIXME: Optimalization - Remove its rendered
        if !self.initialized {
            self.create_initial_wall_boundary(
                &entities,
                &mut walls,
                &mut transforms,
                &mut sprite_renders,
                &wall_resources
            );

            self.initialized = true;
        }

    }
}
