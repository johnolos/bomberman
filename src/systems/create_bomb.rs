use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, Read, System, WriteStorage, Entities, ReadExpect},
    renderer::{SpriteRender},
    input::InputHandler
};

use crate::{
  bomb::{Bomb, INITIAL_BOMB_TIME, INITIAL_BLAST_RADIUS},
  bomb_resource::BombResource,
  core::PlayerNumber,
  player::Player
};

struct NewBomb {
  owner: PlayerNumber,
  bomb_time_multiplier: f32,
  blast_radius_multiplier: i8,
  position: (f32, f32)
}

#[derive(PartialEq)]
enum KeyAction {
    KeyPressed,
    KeyReleased
}

pub struct CreateBombSystem{
    key_actions: Vec<KeyAction>
}

impl CreateBombSystem {
  pub fn default() -> Self {
    CreateBombSystem {
      key_actions: vec![KeyAction::KeyReleased, KeyAction::KeyReleased]
    }
  }

  fn create_bomb<'s>(
    &mut self,
    entities: &Entities<'s>,
    bombs: &mut WriteStorage<'s, Bomb>,
    transforms: &mut WriteStorage<'s, Transform>,
    sprite_renders: &mut WriteStorage<'s, SpriteRender>,
    bomb_resource: &ReadExpect<'s, BombResource>,
    bomb: NewBomb
  ) {
    let mut transform = Transform::default();
    let (pos_x, pos_y) = bomb.position;

    transform.translate_xyz(pos_x, pos_y, 0.0);

    let sprite_render = SpriteRender {
      sprite_sheet: bomb_resource.sprite_sheet.clone(),
      sprite_number: 0
    };

    entities
      .build_entity()
      .with(sprite_render, sprite_renders)
      .with(
        Bomb {
          owner: bomb.owner,
          velocity: [0.0f32, 0.0f32],
          time_left: INITIAL_BOMB_TIME * bomb.bomb_time_multiplier,
          blast_radius: INITIAL_BLAST_RADIUS * bomb.blast_radius_multiplier
        },
        bombs
      )
      .with(transform, transforms)
      .build();
  }
}

impl<'s> System<'s> for CreateBombSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Bomb>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, BombResource>,
        Read<'s, InputHandler<String, String>>,
    );

    fn run(&mut self, data: Self::SystemData) {
      let (
        entities,
        mut players,
        mut bombs,
        mut transforms,
        mut sprite_renders,
        bomb_resources,
        input
      ) = data;

      let mut new_bombs: Vec<NewBomb> = Vec::new();

      // FIXME: Optimalization - Horrible logic in terms of handling one bomb per press
      for (player, transform) in (&mut players, &transforms).join() {
        let create_bomb_key_pressed = match player.player_number {
          PlayerNumber::PlayerOne => input.action_is_down("left_player_bomb"),
          PlayerNumber::PlayerTwo => input.action_is_down("right_player_bomb"),
        };

        if let Some(true) = create_bomb_key_pressed {

          let key_action = match player.player_number {
            PlayerNumber::PlayerOne => &self.key_actions[0],
            PlayerNumber::PlayerTwo => &self.key_actions[1]
          };

          if *key_action == KeyAction::KeyReleased {
            if player.active_bombs < player.allowed_bombs {
              new_bombs.push(NewBomb {
                owner: player.player_number.clone(),
                bomb_time_multiplier: player.bomb_time_multiplier,
                blast_radius_multiplier: player.blast_radius_multiplier,
                position: (transform.translation().x, transform.translation().y)
              });

              match player.player_number {
                PlayerNumber::PlayerOne => self.key_actions[0] = KeyAction::KeyPressed,
                PlayerNumber::PlayerTwo => self.key_actions[1] = KeyAction::KeyPressed
              }
            }
          }
        } else {
          match player.player_number {
            PlayerNumber::PlayerOne => self.key_actions[0] = KeyAction::KeyReleased,
            PlayerNumber::PlayerTwo => self.key_actions[1] = KeyAction::KeyReleased
          }
        };
      }

      for new_bomb in new_bombs {
        self.create_bomb(
          &entities,
          &mut bombs,
          &mut transforms,
          &mut sprite_renders,
          &bomb_resources,
          new_bomb
        );
      }
    }
}
