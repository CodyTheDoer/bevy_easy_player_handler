use bevy::prelude::*;

use std::sync::Arc;
use std::sync::Mutex;

use uuid::Uuid;

use crate::{
    Party,
    Player,
    PlayerLocal,
    PlayerType,
};

impl Party {
    pub fn new(main_player_email: &String, main_player_user_name: &String, party_size: i32) -> Self {
        let active_player: i32 = 1;
        let ai_vec: Option<Vec<usize>> = None;
        let party_size: i32 = party_size;
        let players: Arc<Mutex<Vec<Arc<Mutex<dyn Player + Send>>>>> = Arc::new(Mutex::new(vec![Arc::new(Mutex::new(PlayerLocal::new(Some(main_player_email.to_owned()), Some(main_player_user_name.to_owned()))))]));
        Party {
            active_player,
            ai_vec,
            party_size,
            players,
        } 
    }

    pub fn active_player_get_clone(&self) -> Arc<Mutex<dyn Player + Send>> {
        let active_player_index = self.active_player; // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = players_lock[active_player_index as usize - 1].clone(); // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        player_arc
    }

    pub fn active_player_get_index(&self) -> i32 {
        let active_player = self.active_player;
        active_player
    }

    pub fn active_player_get_player_id(&self) -> Uuid {
        let active_player_index = self.active_player; // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        let player_id = player.get_player_id().to_owned();
        player_id
    }

    pub fn active_player_get_player_type(&self) -> PlayerType {
        let active_player_index = self.active_player; // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        let player_type = player.get_player_type().to_owned();
        player_type
    }

    pub fn active_player_set(&mut self, target: i32) {
        self.active_player = target;
    }

    pub fn active_player_set_email(&mut self, player_email: &str) {
        let active_player_index = self.active_player; // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.set_player_email(player_email);
    }

    pub fn active_player_set_username(&mut self, player_username: &str) {
        let active_player_index = self.active_player; // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.set_player_user_name(player_username);
    }

    pub fn active_player_set_uuid(&mut self, player_id: Uuid) {
        let active_player_index = self.active_player; // Get the active player index
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[active_player_index as usize - 1]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.set_player_id(player_id);
    }

    pub fn all_players_get_ids(&self) -> Vec<Uuid> {
        let mut id_storage: Vec<Uuid> = Vec::new();
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        for player in 0..players_lock.len() {
            let player_arc = &players_lock[player];
            let player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
            let id = player.get_player_id();
            id_storage.push(*id);
        }
        id_storage
    }

    pub fn all_players_get_ids_and_types(&self) -> Vec<(Uuid, PlayerType)> {
        let mut id_type_storage: Vec<(Uuid, PlayerType)> = Vec::new();
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        for player in players_lock.iter() {
            let target_player = player.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
            let id = target_player.get_player_id().to_owned();
            let player_type = target_player.get_player_type().to_owned();
            let id_type = (id, player_type);
            id_type_storage.push(id_type);
        }
        let id_type_storage = id_type_storage.clone();
        id_type_storage
    }

    pub fn get_player_count_ai(&self) -> usize {
        let mut count: usize = 0;
        let players_lock = self.players.lock().unwrap();
        for player in players_lock.iter() {
            let player_type = player.lock().unwrap();
            let player_ref = player_type.get_player_type();
            if player_ref == &PlayerType::PlayerAi {
                count += 1;
            }
        }
        count
    }

    pub fn get_player_count_local(&self) -> usize {
        let mut count: usize = 0;
        let players_lock = self.players.lock().unwrap();
        for player in players_lock.iter() {
            let player_type = player.lock().unwrap();
            let player_ref = player_type.get_player_type();
            if player_ref == &PlayerType::PlayerLocal {
                count += 1;
            }
        }
        count
    }

    pub fn get_player_count_remote(&self) -> usize {
        let mut count: usize = 0;
        let players_lock = self.players.lock().unwrap();
        for player in players_lock.iter() {
            let player_type = player.lock().unwrap();
            let player_ref = player_type.get_player_type();
            if player_ref == &PlayerType::PlayerRemote {
                count += 1;
            }
        }
        count
    }

    pub fn get_player_count_party(&self) -> usize {
        let players_lock = self.players.lock().unwrap();
        let count: usize = players_lock.len();
        count
    }

    pub fn get_party_ai_index_vec(&self) -> Vec<usize> {
        let mut ai_index: Vec<usize> = Vec::new();
        let players_lock = self.players.lock().unwrap();
        for (index, player) in players_lock.iter().enumerate() {
            let player_type = player.lock().unwrap();
            let player_ref = player_type.get_player_type();
            if player_ref == &PlayerType::PlayerAi {
                ai_index.push(index);
            };
        }
        ai_index
    }

    pub fn main_player_clone_player_id(&self) -> Uuid {
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[0]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        let player_id = player.get_player_id();
        player_id.clone()
    }

    pub fn party_size(&self) -> usize {        
        // First, lock the players mutex to get access to the Vec
        let players_lock = self.players.lock().unwrap();

        // Grab the size of the party
        let party_size = &players_lock.len();
        *party_size 
    }

    pub fn player_set_player_id(&mut self, player_idx: usize, new_id: Uuid) {
        let players_lock = self.players.lock().unwrap(); // First, lock the players mutex to get access to the Vec
        let player_arc = &players_lock[player_idx]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().unwrap(); // Lock the player mutex to get a mutable reference to the player
        player.set_player_id(new_id);
    }

    pub fn players_add_player(&self, player: Arc<Mutex<dyn Player + Send>>) {
        let mut players_lock = self.players.lock().unwrap();
        let party_size = self.party_size as usize;
        if players_lock.len() < party_size {
            players_lock.push(player);
        } else {
            warn!("Error: Party full!");
        }
    }
    
    pub fn players_remove_ai(&self) {
        let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
    
        // Iterate through players and find the index of the first occurrence of "PlayerAi".
        if let Some(index) = players_lock.iter().position(|player| {
            let player_lock = player.lock().unwrap();
            player_lock.get_player_type() == &PlayerType::PlayerAi
        }) {
            // Remove the player at the found index
            players_lock.remove(index);
        }
    }
    
    pub fn players_remove_last_player(&self) {
        let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
        // Only pop if we have more than one player
        if players_lock.len() > 1 {
            players_lock.pop();
        }
    }
    
    pub fn players_remove_local_player(&self) {
        let mut players_lock = self.players.lock().unwrap(); // Acquire the lock to get mutable access
        // Only pop if we have more than one player
        if players_lock.len() > 1 {
            if let Some(rev_index) = players_lock.iter().rev().position(|player| {
                let player_lock = player.lock().unwrap();
                player_lock.get_player_type() == &PlayerType::PlayerLocal 
            }) {
                // Convert the reversed index to the original index
                let original_index = players_lock.len() - 1 - rev_index;
    
                // Remove the player at the original index
                players_lock.remove(original_index);
            }
        }
    }

    pub fn players_remove_player(&self, player_id: Uuid) {
        let mut players_lock = self.players.lock().unwrap();
        
        // Proceed only if we have more than one player in the vector
        if players_lock.len() > 1 {
            players_lock.retain(|player| {
                let player_lock = player.lock().unwrap();
                player_lock.get_player_id() != &player_id
            });
        }
    }

    pub fn update_ai_index_vec(&mut self) {
        self.ai_vec = Some(self.get_party_ai_index_vec());
    }
}