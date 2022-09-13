use crate::{
    containers::{HashSet, Storage},
    gameloop::*,
    gametypes::MapPosition,
    maps::*,
    players::*,
};

/* Information Packet Data Portion Worse case is 1420 bytes
* This means you can fit based on Quantity + 4 byte token header  + 4 bytes for count
* Item Size of 17 bytes can send up to 82 per packet.
* Npc Size 80 bytes can send up to 16 per packet.
* player Size 226 bytes can send up to 5 per packet.
*/

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DataTask {
    ownerid: usize,
    mapid: MapPosition,
    currentids: Vec<u64>,
}

impl DataTask {
    pub fn new(ownerid: usize, mapid: MapPosition) -> DataTask {
        DataTask {
            ownerid,
            mapid,
            currentids: Vec::with_capacity(32),
        }
    }
}

//types to buffer load when loading a map.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataTasks {
    NpcMove(DataTask),
    PlayerMove(DataTask),
    ItemUnload(DataTask),
    ItemLoad(DataTask),
    NpcAttack(),
    PlayerAttack(),
}

pub fn init_data_lists(world: &Storage, user: &mut Player, oldmap: MapPosition) {
    /*let mut new_players = HashSet::<u64>::with_capacity_and_hasher(32, Default::default());
    let mut new_npcs = HashSet::<u64>::with_capacity_and_hasher(32, Default::default());
    let mut new_items = HashSet::<u64>::with_capacity_and_hasher(32, Default::default());

    //create the data tasks to be ran against.
    let mut task_player = DataTask::new(user.e.get_id(), user.e.pos.map);
    let mut task_npc = DataTask::new(user.e.get_id(), user.e.pos.map);
    let mut task_item = DataTask::new(user.e.get_id(), user.e.pos.map);

    //get the old map npcs, resource, players and items so we can send remove requests.
    for m in get_surrounding(oldmap, true) {
        if let Some(map) = world.map_data.get(&m) {
            for id in &map.borrow().players {
                old_players.0.push(*id as u64);
                old_players.1.insert(*id as u64);
            }

            for id in &map.borrow().npcs {
                old_npcs.0.push(*id as u64);
                old_npcs.1.insert(*id as u64);
            }

            for id in &map.borrow().itemids {
                old_items.0.push(*id as u64);
                old_items.1.insert(*id as u64);
            }
        }
    }

    if let Some(map) = world.map_data.get(&user.e.pos.map) {
        //Only get the New id's not in Old for the Vec we use the old data to deturmine what use to exist.
        //this gets them for the main map the rest we will cycle thru.
        for id in &map.borrow().players {
            if !old_players.1.contains(&(*id as u64)) {
                task_player.currentids.push(*id as u64);
            }

            new_players.insert(*id as u64);
        }

        for id in &map.borrow().npcs {
            if !old_npcs.1.contains(&(*id as u64)) {
                task_npc.currentids.push(*id as u64);
            }

            new_npcs.insert(*id as u64);
        }

        for id in &map.borrow().itemids {
            if !old_items.1.contains(&(*id as u64)) {
                task_item.currentids.push(*id as u64);
            }

            new_items.insert(*id as u64);
        }

        //we have to do this so the first maps Items Always appear first in the Vec.
        for m in get_surrounding(user.e.pos.map, true) {
            if m != user.e.pos.map {
                if let Some(map) = world.map_data.get(&m) {
                    for id in &map.borrow().players {
                        if !old_players.1.contains(&(*id as u64)) {
                            task_player.currentids.push(*id as u64);
                        }
                        new_players.insert(*id as u64);
                    }
                    for id in &map.borrow().npcs {
                        if !old_npcs.1.contains(&(*id as u64)) {
                            task_npc.currentids.push(*id as u64);
                        }
                        new_npcs.insert(*id as u64);
                    }
                    for id in &map.borrow().itemids {
                        if !old_items.1.contains(&(*id as u64)) {
                            task_item.currentids.push(*id as u64);
                        }
                        new_items.insert(*id as u64);
                    }
                }
            }
        }
    }

    let _ = send_data_remove_list(
        world,
        user.e.get_id(),
        &old_players
            .0
            .iter()
            .copied()
            .filter(|id| !new_players.contains(id))
            .collect::<Vec<u64>>(),
        1,
    );

    let _ = send_data_remove_list(
        world,
        user.e.get_id(),
        &old_npcs
            .0
            .iter()
            .copied()
            .filter(|id| !new_npcs.contains(id))
            .collect::<Vec<u64>>(),
        0,
    );
    let _ = send_data_remove_list(
        world,
        user.e.get_id(),
        &old_items
            .0
            .iter()
            .copied()
            .filter(|id| !new_items.contains(id))
            .collect::<Vec<u64>>(),
        3,
    );

    user.datatasks.push(
        world
            .map_switch_tasks
            .borrow_mut()
            .insert(MapSwitchTasks::Player(task_player)),
    );
    user.datatasks.push(
        world
            .map_switch_tasks
            .borrow_mut()
            .insert(MapSwitchTasks::Npc(task_npc)),
    );
    user.datatasks.push(
        world
            .map_switch_tasks
            .borrow_mut()
            .insert(MapSwitchTasks::Items(task_item)),
    );*/
}
