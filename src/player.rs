use crate::{level::attach_to_level, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackOrMoveEvent>()
            .add_systems(PostStartup, attach_to_level::<Player>)
            .add_systems(
                Update,
                (keyboard_input, attack_or_move)
                    .chain()
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
                health: Health::new(30),
                initiative: Initiative::new(6),
                melee_damage: MeleeDamage(5),
                blocks_movement: BlocksMovement,
                viewshed: Viewshed::new(8),
                sprite: text_sprite.build('@', Color::YELLOW),
            },
        }
    }
}

#[derive(Event)]
struct AttackOrMoveEvent(CompassDirection);

fn player_has_initiative(query: Query<(&Player, &HasInitiative)>) -> bool {
    !query.is_empty()
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut attack_or_move_events: EventWriter<AttackOrMoveEvent>,
) {
    if keys.just_pressed(KeyCode::Q) {
        attack_or_move_events.send(AttackOrMoveEvent(NorthWest));
    }
    if keys.just_pressed(KeyCode::W) {
        attack_or_move_events.send(AttackOrMoveEvent(North));
    }
    if keys.just_pressed(KeyCode::E) {
        attack_or_move_events.send(AttackOrMoveEvent(NorthEast));
    }
    if keys.just_pressed(KeyCode::A) {
        attack_or_move_events.send(AttackOrMoveEvent(SouthWest));
    }
    if keys.just_pressed(KeyCode::S) {
        attack_or_move_events.send(AttackOrMoveEvent(South));
    }
    if keys.just_pressed(KeyCode::D) {
        attack_or_move_events.send(AttackOrMoveEvent(SouthEast));
    }
}

#[allow(clippy::type_complexity)]
fn attack_or_move(
    mut attack_or_move_events: EventReader<AttackOrMoveEvent>,
    player_query: Query<(Entity, &Position), With<Player>>,
    targets: Query<(Entity, &Position), (With<Character>, Without<Player>)>,
    mut melee_events: EventWriter<MeleeEvent>,
    mut move_events: EventWriter<MoveEvent>,
) {
    let (player_id, &player_pos) = player_query.single();

    for &AttackOrMoveEvent(dir) in attack_or_move_events.iter() {
        let new_pos = player_pos + dir;

        match targets.iter().find(|(_, &pos)| pos == new_pos) {
            Some((target_id, _)) => melee_events.send(MeleeEvent(player_id, target_id)),
            None => move_events.send(MoveEvent(player_id, new_pos)),
        }
    }
}
