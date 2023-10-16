use crate::prelude::*;
use pathfinding::prelude::astar;

#[allow(clippy::type_complexity)]
pub(super) fn plan_turn(
    query: Query<(Entity, &Viewshed, &Position), (With<HasInitiative>, Without<Player>)>,
    player_query: Query<&Position, With<Player>>,
    blockers_query: Query<&Position, With<BlocksMovement>>,
    mut move_action: EventWriter<MoveEvent>,
    mut turns: EventWriter<SpendTurnEvent>,
) {
    let player_pos = player_query.single();

    if let Ok((entity, vs, pos)) = query.get_single() {
        if vs.includes(player_pos) {
            let blocked: HashSet<_> = blockers_query.iter().copied().collect();

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
                move_action.send(MoveEvent(entity, path[1]));
            }
        }
        turns.send(SpendTurnEvent(entity));
    }
}