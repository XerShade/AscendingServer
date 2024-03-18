use crate::{containers::Storage, gametypes::*, maps::*, npcs::*, players::Account, tasks::*};
use chrono::Duration;
use hecs::World;

pub fn npc_warp(
    world: &mut World,
    storage: &Storage,
    entity: &Entity,
    new_pos: &Position,
) {
    if world.get_or_panic::<Position>(entity).map != new_pos.map {
        let old_pos = npc_switch_maps(world, storage, entity, *new_pos);
        let _ = DataTaskToken::NpcWarp(old_pos.map)
            .add_task(storage, &WarpPacket::new(*entity, *new_pos));
        let _ = DataTaskToken::NpcWarp(new_pos.map)
            .add_task(storage, &WarpPacket::new(*entity, *new_pos));
        let _ = DataTaskToken::NpcSpawn(new_pos.map)
            .add_task(storage, &NpcSpawnPacket::new(world, entity));
    } else {
        npc_swap_pos(world, storage, entity, *new_pos);
        let _ = DataTaskToken::NpcWarp(new_pos.map)
            .add_task(storage, &WarpPacket::new(*entity, *new_pos));
    }
}

pub fn npc_movement(world: &mut World, storage: &Storage, entity: &Entity, _base: &NpcData) {
    //AI Timer is used to Reset the Moves every so offten to recalculate them for possible changes.
    if world.get_or_panic::<NpcAITimer>(entity).0 < *storage.gettick.borrow()
        && world.get_or_panic::<NpcMoving>(entity).0
    {
        world
            .get::<&mut NpcMoves>(entity.0)
            .expect("Could not find NpcMoves")
            .0
            .clear();
        world
            .get::<&mut NpcMoving>(entity.0)
            .expect("Could not find NpcMoves")
            .0 = false;
    }

    if !world.get_or_panic::<NpcMoving>(entity).0
        && world.get::<&NpcMoves>(entity.0).unwrap().0.is_empty()
    {
        if let Some(movepos) = world.get_or_panic::<NpcMovePos>(entity).0 {
            //Move pos overrides targeting pos movement.
            if let Some(path) = a_star_path(
                storage,
                world.get_or_panic::<Position>(entity),
                world.get_or_panic::<Dir>(entity).0,
                movepos,
            ) {
                npc_set_move_path(world, entity, path);
            }
        } else if world.get_or_panic::<Target>(entity).targettype != EntityType::None
            && storage
                .maps
                .get(&world.get_or_panic::<Position>(entity).map)
                .map(|map| map.borrow().players_on_map())
                .unwrap_or(false)
        {
            if let Some(path) = a_star_path(
                storage,
                world.get_or_panic::<Position>(entity),
                world.get_or_panic::<Dir>(entity).0,
                world.get_or_panic::<Target>(entity).targetpos,
            ) {
                npc_set_move_path(world, entity, path);
            }
        } else {
            //no special movement lets give them some if we can;
            if storage
                .maps
                .get(&world.get_or_panic::<Position>(entity).map)
                .map(|map| map.borrow().players_on_map())
                .unwrap_or(false)
            {
                npc_set_move_path(
                    world,
                    entity,
                    npc_rand_movement(
                        storage,
                        world.get_or_panic::<Position>(entity),
                        world.get_or_panic::<Dir>(entity).0,
                    ),
                );
            }
        }

        world
            .get::<&mut NpcAITimer>(entity.0)
            .expect("Could not find NpcAITimer")
            .0 = *storage.gettick.borrow() + Duration::try_milliseconds(2500).unwrap_or_default();
    }

    if world.get_or_panic::<NpcMoving>(entity).0 {
        let next = match world
            .get::<&mut NpcMoves>(entity.0)
            .expect("Could not find NpcMoves")
            .0
            .pop()
        {
            Some(v) => v,
            None => return,
        };

        if map_path_blocked(
            storage,
            world.get_or_panic::<Position>(entity),
            next.0,
            next.1,
        ) {
            world
                .get::<&mut NpcMoves>(entity.0)
                .expect("Could not find NpcMoves")
                .0
                .push(next);

            return;
        }

        if world.get_or_panic::<Position>(entity) == next.0 {
            set_npc_dir(world, storage, entity, next.1);
        } else {
            if world.get_or_panic::<NpcMovePos>(entity).0.is_none() {
                //do any movepos to position first
                if !storage
                    .maps
                    .get(&world.get_or_panic::<Position>(entity).map)
                    .map(|map| map.borrow().players_on_map())
                    .unwrap_or(false)
                {
                    npc_clear_move_path(world, entity);
                    return;
                }

                match world.get_or_panic::<Target>(entity).targettype {
                    EntityType::Player(i, accid) => {
                        if world.contains(i.0) {
                            if world.get_or_panic::<DeathType>(&i).is_alive()
                                && world.get::<&Account>(i.0).unwrap().id == accid
                            {
                                if world.get_or_panic::<Position>(&i) == next.0 {
                                    npc_clear_move_path(world, entity);
                                    set_npc_dir(world, storage, entity, next.1);
                                    return;
                                }
                            } else {
                                npc_clear_move_path(world, entity);
                            }
                        } else {
                            npc_clear_move_path(world, entity);
                        }
                    }
                    EntityType::Npc(i) => {
                        if world.contains(i.0) {
                            if world.get_or_panic::<DeathType>(&i).is_alive() {
                                if world.get_or_panic::<Position>(&i) == next.0 {
                                    npc_clear_move_path(world, entity);
                                    set_npc_dir(world, storage, entity, next.1);
                                    return;
                                }
                            } else {
                                npc_clear_move_path(world, entity);
                            }
                        } else {
                            npc_clear_move_path(world, entity);
                        }
                    }
                    _ => {}
                };
            } else if Some(next.0) == world.get_or_panic::<NpcMovePos>(entity).0 {
                world
                    .get::<&mut NpcMovePos>(entity.0)
                    .expect("Could not find NpcMovePos")
                    .0 = None;

                npc_clear_move_path(world, entity);
            }

            world
                .get::<&mut Dir>(entity.0)
                .expect("Could not find Dir")
                .0 = next.1;

            let old_map = world.get_or_panic::<Position>(entity).map;
            if next.0.map != old_map {
                npc_switch_maps(world, storage, entity, next.0);
                //Send this Twice one to the old map and one to the new. Just in case people in outermaps did not get it yet.
                let _ = DataTaskToken::NpcMove(old_map).add_task(
                    storage,
                    &MovePacket::new(*entity, next.0, false, true, next.1),
                );
                let _ = DataTaskToken::NpcMove(next.0.map).add_task(
                    storage,
                    &MovePacket::new(*entity, next.0, false, true, next.1),
                );
            } else {
                npc_swap_pos(world, storage, entity, next.0);
                let _ = DataTaskToken::NpcMove(next.0.map).add_task(
                    storage,
                    &MovePacket::new(*entity, next.0, false, false, next.1),
                );
            }
        }
    }
}
