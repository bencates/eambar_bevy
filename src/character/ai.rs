use crate::prelude::*;
use pathfinding::prelude::astar;

#[allow(clippy::type_complexity)]
pub(super) fn plan_turn(
    query: Query<(Entity, &Viewshed, &Position), (With<HasInitiative>, Without<Player>)>,
    player_query: Query<(Entity, &Position), With<Player>>,
    blockers_query: Query<&Position, With<BlocksMovement>>,
    mut melee_action: EventWriter<MeleeEvent>,
    mut move_action: EventWriter<MoveEvent>,
    mut turns: EventWriter<SpendTurnEvent>,
) {
    let (player_id, player_pos) = player_query.single();

    if let Ok((entity, vs, pos)) = query.get_single() {
        if vs.includes(player_pos) {
            if pos.distance(player_pos) == 1 {
                return melee_action.send(MeleeEvent(entity, player_id));
            }

            let blocked: HashSet<_> = blockers_query.iter().collect();

            if let Some((path, _)) = astar(
                pos,
                |pos| {
                    pos.neighbors()
                        .into_iter()
                        .filter(|n| n == player_pos || !blocked.contains(n))
                        .map(|n| (n, 1))
                },
                |pos| pos.distance(player_pos),
                |pos| pos == player_pos,
            ) {
                return move_action.send(MoveEvent(entity, path[1]));
            }
        }

        turns.send(SpendTurnEvent(entity));
    }
}
