use crate::{level::attach_to_level, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, attach_to_level::<Player>)
            .add_systems(Update, keyboard_input);
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
                blocks_movement: BlocksMovement,
                viewshed: Viewshed::new(8),
                sprite: text_sprite.build('@', Color::YELLOW),
            },
        }
    }
}

fn keyboard_input(
    query: Query<Entity, With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut move_action: EventWriter<MoveEvent>,
) {
    let player_id = query.single();

    if keys.just_pressed(KeyCode::Q) {
        move_action.send(MoveEvent(player_id, NorthWest));
    }
    if keys.just_pressed(KeyCode::W) {
        move_action.send(MoveEvent(player_id, North));
    }
    if keys.just_pressed(KeyCode::E) {
        move_action.send(MoveEvent(player_id, NorthEast));
    }
    if keys.just_pressed(KeyCode::A) {
        move_action.send(MoveEvent(player_id, SouthWest));
    }
    if keys.just_pressed(KeyCode::S) {
        move_action.send(MoveEvent(player_id, South));
    }
    if keys.just_pressed(KeyCode::D) {
        move_action.send(MoveEvent(player_id, SouthEast));
    }
}
