use crate::gametypes::*;
use serde::{Deserialize, Serialize};

#[derive(Derivative, Clone, Debug, Default, Serialize, Deserialize)]
#[derivative(PartialEq)]
pub struct NpcData {
    pub name: String,
    pub level: i32,
    pub movement_wait: i64,
    pub attack_wait: i64,
    pub spawn_wait: i64,
    pub maxhp: u32,
    pub maxsp: u32,
    pub maxmp: u32,
    pub sight: i32,
    pub follow_sight: i32,
    pub walkdistance: u32,
    pub pdamage: u32,
    pub pdefense: u32,
    pub canpassthru: bool,
    pub size: TileBox,
    pub behaviour: AIBehavior,
    pub maxdamage: u32,
    pub mindamage: u32,
    pub target_auto_switch: bool,
    pub target_attacked_switch: bool,
    pub target_auto_switch_chance: i64,
    pub target_range_dropout: bool,
    pub can_target: bool,
    pub can_move: bool,
    pub can_attack_player: bool,
    pub has_enemies: bool,
    pub can_attack: bool,
    pub spawntime: (GameTime, GameTime),
    pub range: i32,        //attack range. How far they need to be to hit their target.
    pub enemies: Vec<u64>, //list of enemies the npcs can attack of other npc's... WAR!
    pub drops: Vec<(u32, u32, u32)>, //item dropped on death, chance, amount
    pub drops_max: u16, //number of Different items that will be picked  0..=drops_max. that we can cycle thru and drop at random.
}

impl NpcData {
    pub fn is_agressive(&self) -> bool {
        self.behaviour.is_agressive()
    }

    pub fn is_reactive(&self) -> bool {
        self.behaviour.is_reactive()
    }

    pub fn is_healer(&self) -> bool {
        self.behaviour.is_healer()
    }

    pub fn is_friendly(&self) -> bool {
        self.behaviour.is_friendly()
    }
    /// load npc data from json with serdes.
    /// if ID does not exist or file nto found return None.
    pub fn load_npc(_id: u64) -> Option<NpcData> {
        None
    }
}
