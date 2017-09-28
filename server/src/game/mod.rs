use uuid::Uuid;
use lib::ParseFrom;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Item {
    // TODO(jordan): macro
    Potato  { id: Uuid },
    Berry   { id: Uuid },
    TreeSap { id: Uuid },
}

impl ParseFrom<i32> for Item {
    fn parse (i: i32) -> Result<Self, String> {
        match i {
            0 => Ok(Potato::new()),
            1 => Ok(Berry::new()),
            2 => Ok(TreeSap::new()),
            _ => Err(format!("Item.parse<i32> failure: unrecognized Item {}", i)),
        }
    }
}

// TODO(jordan): macro
#[allow(non_snake_case)]
mod Potato {
    use super::{Item,Uuid};
    pub fn new () -> Item {
        Item::Potato { id: Uuid::new_v4() }
    }
}

// TODO(jordan): macro
#[allow(non_snake_case)]
mod Berry {
    use super::{Item,Uuid};
    pub fn new () -> Item {
        Item::Berry { id: Uuid::new_v4() }
    }
}

// TODO(jordan): macro
#[allow(non_snake_case)]
mod TreeSap {
    use super::{Item,Uuid};
    pub fn new () -> Item {
        Item::TreeSap { id: Uuid::new_v4() }
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
    },
}

impl Player {
    #[allow(unused_variables)]
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
        new_player_inventory.insert(Potato::new());

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
        new_player_inventory.insert(Potato::new());

        Player::RegisteredPlayer {
            id: id,
            name: name,
            state: PlayerState {
                inventory: new_player_inventory
            }
        }
    }
}
