use crate::{level::attach_to_level, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, attach_to_level::<Player>)
            .add_systems(
                Update,
                keyboard_input
                    .in_set(PlanTurn)
                    .run_if(player_has_initiative),
            );
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    character: CharacterBundle,
}

impl PlayerBundle {
    pub fn new(text_sprite: &TextSprite) -> PlayerBundle {
        PlayerBundle {
            marker: Player,
            character: CharacterBundle {
                marker: Character::Player,
                name: Name("Player".to_string()),
                initiative: Initiative::new(6),
                blocks_movement: BlocksMovement,
                viewshed: Viewshed::new(8),
                sprite: text_sprite.build('@', Color::YELLOW),
            },
        }
    }
}

fn player_has_initiative(query: Query<(&Player, &HasInitiative)>) -> bool {
    !query.is_empty()
}

fn keyboard_input(
    query: Query<(Entity, &Position), With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut move_action: EventWriter<MoveEvent>,
) {
    let (player_id, player_pos) = query.single();

    if keys.just_pressed(KeyCode::Q) {
        move_action.send(MoveEvent(player_id, *player_pos + NorthWest));
    }
    if keys.just_pressed(KeyCode::W) {
        move_action.send(MoveEvent(player_id, *player_pos + North));
    }
    if keys.just_pressed(KeyCode::E) {
        move_action.send(MoveEvent(player_id, *player_pos + NorthEast));
    }
    if keys.just_pressed(KeyCode::A) {
        move_action.send(MoveEvent(player_id, *player_pos + SouthWest));
    }
    if keys.just_pressed(KeyCode::S) {
        move_action.send(MoveEvent(player_id, *player_pos + South));
    }
    if keys.just_pressed(KeyCode::D) {
        move_action.send(MoveEvent(player_id, *player_pos + SouthEast));
    }
}
