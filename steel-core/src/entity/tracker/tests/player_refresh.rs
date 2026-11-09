use super::*;
use crate::test_support::{TestPlayerBuilder, fresh_test_world};

#[test]
fn player_refresh_visits_tracked_entities_and_entities_in_view() {
    init_vanilla_registry();
    let world_fixture = fresh_test_world("tracker_player_refresh");
    let player = TestPlayerBuilder::new(Arc::clone(&world_fixture.world), "Viewer", 10).build();
    let get_player = |player_id: i32| (player_id == player.id()).then(|| Arc::clone(&player));
    let tracker = EntityTracker::new();
    let far_pos = DVec3::new(1000.0, 64.0, 0.0);
    let far_view = PlayerChunkView::new(ChunkPos::from_entity_pos(far_pos), 2);

    player.base().set_position_local(DVec3::new(0.0, 64.0, 0.0));
    let near: SharedEntity = PairingTestEntity::new(1, Vec::new());
    let far: SharedEntity = PairingTestEntity::new(2, Vec::new());
    far.base().set_position_local(far_pos);
    tracker.add(&near, |_| vec![player.id()], get_player);
    tracker.add(&far, |_| vec![player.id()], get_player);
    assert_eq!(tracker.tracking_player_ids(near.id()), [player.id()]);
    assert!(tracker.tracking_player_ids(far.id()).is_empty());

    player.base().set_position_local(far_pos);
    tracker.update_player(&player, &far_view, |_| true);
    assert!(
        tracker.tracking_player_ids(near.id()).is_empty(),
        "a tracked entity outside the new view despawns"
    );
    assert_eq!(
        tracker.tracking_player_ids(far.id()),
        [player.id()],
        "an entity in a sent chunk of the new view spawns"
    );

    near.base().set_position_local(far_pos);
    tracker.on_entity_section_change(
        near.id(),
        ChunkPos::from_entity_pos(DVec3::ZERO),
        far_view.center,
        |_| Vec::new(),
        get_player,
    );
    tracker.update_player(&player, &far_view, |_| true);
    assert_eq!(
        tracker.tracking_player_ids(near.id()),
        [player.id()],
        "a refresh finds an entity in the chunk it moved to"
    );

    tracker.on_player_leave(&player);
    assert!(tracker.tracking_player_ids(near.id()).is_empty());
    assert!(tracker.tracking_player_ids(far.id()).is_empty());
}
