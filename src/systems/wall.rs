use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, ReadExpect, System, WriteStorage},
    renderer::{SpriteRender},
};

use crate::{
    wall::{Wall, WALL_HEIGHT, WALL_WIDTH},
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
        let y_values = vec![0, 8];

        for x in 0..15 {
            for y in y_values.iter() {
                let mut transform = Transform::default();
                transform.translate_xyz(x as f32 * 128.0 + (WALL_WIDTH * 0.5), *y as f32 * 128.0 +(WALL_HEIGHT * 0.5), -0.1);

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
            for y in 1..8 {
                let mut transform = Transform::default();
                transform.translate_xyz(*x as f32 * 128.0 + (WALL_WIDTH * 0.5), y as f32 * 128.0 + (WALL_HEIGHT * 0.5), -0.1);

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

        let coordinates = vec![
            (2,2),
            (2,4),
            (2,6),
            (4,2),
            (4,4),
            (4,6),
            (6,2),
            (6,4),
            (6,6),
            (8,2),
            (8,4),
            (8,6),
            (10,2),
            (10,4),
            (10,6),
            (12,2),
            (12,4),
            (12,6)
        ];

        for (x, y) in coordinates.iter() {
            let mut transform = Transform::default();
            transform.translate_xyz(*x as f32 * 128.0 + (WALL_WIDTH * 0.5), *y as f32 * 128.0 + (WALL_HEIGHT * 0.5), -0.1);

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
