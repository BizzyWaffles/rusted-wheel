use lib::ParseFrom;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Item {
    Potato,
    Berry,
    TreeSap,
}

impl ParseFrom<i32> for Item {
    fn parse (i: i32) -> Result<Self, String> {
        match i {
            0 => Ok(Item::Potato),
            1 => Ok(Item::Berry),
            2 => Ok(Item::TreeSap),
            _ => Err(format!("Item.parse<i32> failure: unrecognized Item {}", i)),
        }
    }
}

#[derive(Debug,Clone)]
#[allow(non_camel_case_types)]
pub enum Action {
    addItemToInventory(Item),
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub inventory: HashSet<Item>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Player {
    AnonymousPlayer {
        state : PlayerState,
    },
    RegisteredPlayer {
        id    : i32,
        name  : String,
        state : PlayerState,
    }
}

impl Player {
    pub fn state (&mut self) -> &mut PlayerState {
        match self {
            &mut Player::AnonymousPlayer { ref mut state } => state,
            &mut Player::RegisteredPlayer { ref mut id, ref mut name, ref mut state } => state,
        }
    }
}

#[allow(non_snake_case)]
pub mod AnonymousPlayer {
    use super::*;
    pub fn new () -> Player {
        let mut new_player_inventory = HashSet::new();
        new_player_inventory.insert(Item::Potato);

        Player::AnonymousPlayer {
            state: PlayerState {
                inventory: new_player_inventory
            }
        }
    }
}

#[allow(non_snake_case,dead_code)]
pub mod RegisteredPlayer {
    use super::*;
    pub fn new (id: i32, name: String) -> Player {
        let mut new_player_inventory = HashSet::new();
        new_player_inventory.insert(Item::Potato);

        Player::RegisteredPlayer {
            id: id,
            name: name,
            state: PlayerState {
                inventory: new_player_inventory
            }
        }
    }
}
