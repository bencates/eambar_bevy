use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackOrMoveEvent>()
            .add_event::<TargetEvent>()
            .add_systems(
                Update,
                (keyboard_input, attack_or_move, set_target)
                    .chain()
                    .in_set(PlanTurn)
                    .run_if(player_has_initiative),
            );
    }
}

#[derive(Component)]
pub struct Player;

impl Player {
    const Z_INDEX: f32 = 10.;
}

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    target: Target,
    character: CharacterBundle,
}

impl PlayerBundle {
    pub fn new(text_sprite: &TextSprite) -> PlayerBundle {
        PlayerBundle {
            marker: Player,
            target: Target::default(),
            character: CharacterBundle {
                marker: Character::Player,
                name: Name::new("Player"),
                health: Health::new(30),
                initiative: Initiative::new(6),
                melee_damage: MeleeDamage(5),
                blocks_movement: BlocksMovement,
                viewshed: Viewshed::new(8),
                sprite: text_sprite.build('@', Color::YELLOW, Player::Z_INDEX),
            },
        }
    }
}

#[derive(Event)]
struct AttackOrMoveEvent(CompassDirection);

#[derive(Event)]
enum TargetEvent {
    Next,
    Prev,
    Set(Entity),
}

fn player_has_initiative(query: Query<(&Player, &HasInitiative)>) -> bool {
    !query.is_empty()
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut attack_or_move_events: EventWriter<AttackOrMoveEvent>,
    mut target_events: EventWriter<TargetEvent>,
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

    if keys.just_pressed(KeyCode::Tab) {
        if keys.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
            target_events.send(TargetEvent::Prev);
        } else {
            target_events.send(TargetEvent::Next);
        }
    }
}

#[allow(clippy::type_complexity)]
fn attack_or_move(
    mut attack_or_move_events: EventReader<AttackOrMoveEvent>,
    player_query: Query<(Entity, &Position), With<Player>>,
    targets: Query<(Entity, &Position), (With<Character>, Without<Player>)>,
    mut target_events: EventWriter<TargetEvent>,
    mut melee_events: EventWriter<MeleeEvent>,
    mut move_events: EventWriter<MoveEvent>,
) {
    let (player_id, &player_pos) = player_query.single();

    for &AttackOrMoveEvent(dir) in attack_or_move_events.iter() {
        let new_pos = player_pos + dir;

        match targets.iter().find(|(_, &pos)| pos == new_pos) {
            Some((target_id, _)) => {
                target_events.send(TargetEvent::Set(target_id));
                melee_events.send(MeleeEvent(player_id, target_id))
            }
            None => move_events.send(MoveEvent(player_id, new_pos)),
        }
    }
}

#[allow(clippy::type_complexity)]
fn set_target(
    mut player_query: Query<(&Viewshed, &mut Target), With<Player>>,
    targets_query: Query<(Entity, &Position), (With<Character>, Without<Player>)>,
    mut target_events: EventReader<TargetEvent>,
) {
    let (vs, mut target) = player_query.single_mut();

    let mut targets: Vec<_> = targets_query
        .iter()
        .filter_map(|(id, pos)| vs.includes(pos).then_some(id))
        .collect();

    targets.sort_unstable();

    for event in target_events.iter() {
        let new_target_id = match event {
            TargetEvent::Prev => target
                .id()
                .and_then(|curr| {
                    targets
                        .windows(2)
                        .find_map(|t| (t[1] == curr).then_some(t[0]))
                })
                .or_else(|| targets.last().copied()),

            TargetEvent::Next => target
                .id()
                .and_then(|curr| {
                    targets
                        .windows(2)
                        .find_map(|t| (t[0] == curr).then_some(t[1]))
                })
                .or_else(|| targets.first().copied()),

            TargetEvent::Set(target_id) => targets.iter().find(|&id| id == target_id).copied(),
        };

        debug!("targeting {new_target_id:?}");

        match new_target_id {
            Some(id) => target.set(id),
            None => target.clear(),
        }
    }
}
