use crate::{containers::Storage, gameloop::*, gametypes::*, maps::*, players::*, sql::*};
use chrono::Duration;
use rand::{thread_rng, Rng};
use std::cmp::{max, min};
use unwrap_helpers::{unwrap_continue, ToOption};

pub fn update_maps(world: &Storage) -> Result<()> {
    let tick = *world.gettick.borrow();
    let mut rng = thread_rng();
    let mut spawnable = Vec::new();
    let mut len = world.npcs.borrow().len();

    for (position, map_data) in &world.map_data {
        // Only Spawn is a player is on or near a the map.
        if map_data.borrow().players_on_map() {
            //get this so we can Add to it each time without needing to borrow() npcs again.

            let mut count = 0;

            //Spawn NPC's if the max npc's per world is not yet reached.
            if len < MAX_WORLD_NPCS {
                let map = world
                    .bases
                    .map
                    .get(position)
                    .ok_or(AscendingError::MapNotFound(*position))?;

                for (id, (max_npcs, zone_npcs)) in map.zones.iter().enumerate() {
                    let data = map_data.borrow();
                    //We want to only allow this many npcs per map to spawn at a time.
                    if count + 1 >= NPCS_SPAWNCAP {
                        break;
                    }

                    if !map.zonespawns[id].is_empty() && data.zones[id] < *max_npcs {
                        // Set the Max allowed to spawn by either spawn cap or npc spawn limit.
                        let max_spawnable =
                            min((*max_npcs - data.zones[id]) as usize, NPCS_SPAWNCAP);

                        //Lets Do this for each npc;
                        for npc in zone_npcs {
                            let npc_id = unwrap_continue!(*npc);
                            let game_time = world.time.borrow();
                            let (from, to) = world
                                .bases
                                .npc
                                .get(npc_id as usize)
                                .ok_or(AscendingError::NpcNotFound(npc_id))?
                                .spawntime;

                            //Give them a percentage chance to actually spawn
                            //or see if we can spawn them yet within the time frame.
                            if rng.gen_range(0..2) > 0 || !game_time.in_range(from, to) {
                                continue;
                            }

                            //Lets only allow spawning of a set amount each time. keep from over burdening the system.
                            if count + 1 >= max_spawnable || len + 1 >= MAX_WORLD_NPCS {
                                break;
                            }

                            let mut loop_count = 0;

                            //Only try to find a spot so many times randomly.
                            while loop_count < 10 {
                                let pos_id = rng.gen_range(0..map.zonespawns[id].len());
                                let (x, y) = map.zonespawns[id][pos_id];
                                let spawn = Position::new(x as i32, y as i32, *position);

                                loop_count += 1;

                                //Check if the tile is blocked or not.
                                if !data.is_blocked_tile(spawn) {
                                    //Set NPC as spawnable and to do further checks later.
                                    //Doing this to make the code more readable.
                                    spawnable.push((spawn, id, npc_id));
                                    count = count.saturating_add(1);
                                    len = len.saturating_add(1);
                                    break;
                                }
                            }
                        }
                    }
                }

                let mut data = map_data.borrow_mut();
                //Lets Spawn the npcs here;
                for (spawn, zone_id, npc_id) in spawnable.drain(..) {

                    //data.add_entity_to_grid()
                    //data.add_npc(global_npc_id);
                }
            }
        }
    }

    Ok(())
}
