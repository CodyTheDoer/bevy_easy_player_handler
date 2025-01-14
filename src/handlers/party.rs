use bevy::prelude::*;

use bevy_easy_shared_definitions::ErrorTypePlayerHandler;

use std::sync::Arc;
use std::sync::Mutex;

use uuid::Uuid;

use crate::BevyEasyPlayerHandlerPlugin;
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
        let players: Arc<Mutex<Vec<Arc<Mutex<dyn Player + Send>>>>> = Arc::new(Mutex::new(vec![Arc::new(Mutex::new(PlayerLocal::new(Some(main_player_email.to_owned()), Some(main_player_user_name.to_owned()), PlayerType::PlayerLocal)))]));
        Party {
            active_player,
            ai_vec,
            party_size,
            players,
        } 
    }

    pub fn active_player_get_clone(&self) -> Result<Arc<Mutex<dyn Player + Send>>, ErrorTypePlayerHandler> {
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        
        let index = self.active_player as usize;
        if index == 0 || index > players.len() {
            return Err(ErrorTypePlayerHandler::IndexOutOfBounds(format!(
                "active_player={} is out of bounds (len={})",
                self.active_player,
                players.len()
            )));
        }

        Ok(players[index - 1].clone())
    }

    pub fn active_player_get_index(&self) -> Result<i32, ErrorTypePlayerHandler> {
        let active_player = self.active_player;
        Ok(active_player)
    }

    pub fn active_player_get_player_id(&self) -> Result<Uuid, ErrorTypePlayerHandler> {
        let active_player_index = self.active_player_get_index()?;
        let adj_active_player_index = active_player_index - 1;
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        let player_arc = &players[adj_active_player_index as usize]; 
        let player = player_arc.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?; // Lock the player mutex to get a mutable reference to the player
        let player_id = player.get_player_id()?.to_owned();
        Ok(player_id)
    }

    pub fn active_player_get_player_type(&self) -> Result<PlayerType, ErrorTypePlayerHandler> {
        let active_player_index = self.active_player; // Get the active player index
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        let player_arc = &players[(active_player_index - 1) as usize]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let player = player_arc.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?; // Lock the player mutex to get a mutable reference to the player
        let player_type = player.get_player_type()?.to_owned();
        Ok(player_type)
    }

    pub fn active_player_set(&mut self, target: i32) -> Result<(), ErrorTypePlayerHandler> {
        self.active_player = target;
        Ok(())
    }

    pub fn active_player_set_email(&mut self, player_email: &str) -> Result<(), ErrorTypePlayerHandler> {
        let active_player_index = self.active_player; // Get the active player index
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        let player_arc = &players[(active_player_index - 1) as usize]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?;// Lock the player mutex to get a mutable reference to the player
        player.set_player_email(player_email)?;
        Ok(())
    }

    pub fn active_player_set_username(&mut self, player_username: &str) -> Result<(), ErrorTypePlayerHandler> {
        let active_player_index = self.active_player; // Get the active player index
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        let player_arc = &players[(active_player_index - 1) as usize]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?; // Lock the player mutex to get a mutable reference to the player
        player.set_player_user_name(player_username)?;
        Ok(())
    }

    pub fn active_player_set_uuid(&mut self, player_id: Uuid) -> Result<(), ErrorTypePlayerHandler> {
        let active_player_index = self.active_player; // Get the active player index
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        let player_arc = &players[(active_player_index - 1) as usize]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?; // Lock the player mutex to get a mutable reference to the player
        player.set_player_id(player_id)?;
        Ok(())
    }

    pub fn all_players_get_ids(&self) -> Result<Vec<Uuid>, ErrorTypePlayerHandler> {
        let mut id_storage: Vec<Uuid> = Vec::new();
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        for player in players.iter() {
            let player = player.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?; // Lock the player mutex to get a mutable reference to the player
            let id = player.get_player_id()?;
            id_storage.push(*id);
        }
        Ok(id_storage)
    }

    pub fn all_players_get_ids_and_types(&self) -> Result<Vec<(Uuid, PlayerType)>, ErrorTypePlayerHandler> {
        let mut id_type_storage: Vec<(Uuid, PlayerType)> = Vec::new();
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        for player in players.iter() {
            let target_player = player.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?; // Lock the player mutex to get a mutable reference to the player
            let id = target_player.get_player_id()?.to_owned();
            let player_type = target_player.get_player_type()?.to_owned();
            let id_type = (id, player_type);
            id_type_storage.push(id_type);
        }
        let id_type_storage = id_type_storage.clone();
        Ok(id_type_storage)
    }

    pub fn get_player_count_ai_total(&self) -> Result<usize, ErrorTypePlayerHandler> {
        let mut count: usize = 0;
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        for player in players.iter() {
            let player_type = player.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?;
            let player_ref = player_type.get_player_type()?;
            if player_ref == &PlayerType::PlayerAiLocal || player_ref == &PlayerType::PlayerAiRemote {
                count += 1;
            }
        }
        Ok(count)
    }

    pub fn get_player_count_ai_local(&self) -> Result<usize, ErrorTypePlayerHandler> {
        let mut count: usize = 0;
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        for player in players.iter() {
            let player_type = player.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?;
            let player_ref = player_type.get_player_type()?;
            if player_ref == &PlayerType::PlayerAiLocal {
                count += 1;
            }
        }
        Ok(count)
    }

    pub fn get_player_count_ai_remote(&self) -> Result<usize, ErrorTypePlayerHandler> {
        let mut count: usize = 0;
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        for player in players.iter() {
            let player_type = player.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?;
            let player_ref = player_type.get_player_type()?;
            if player_ref == &PlayerType::PlayerAiRemote {
                count += 1;
            }
        }
        Ok(count)
    }

    pub fn get_player_count_local(&self) -> Result<usize, ErrorTypePlayerHandler> {
        let mut count: usize = 0;
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        for player in players.iter() {
            let player_type = player.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?;
            let player_ref = player_type.get_player_type()?;
            if player_ref == &PlayerType::PlayerLocal {
                count += 1;
            }
        }
        Ok(count)
    }

    pub fn get_player_count_remote(&self) -> Result<usize, ErrorTypePlayerHandler> {
        let mut count: usize = 0;
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        for player in players.iter() {
            let player_type = player.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?;
            let player_ref = player_type.get_player_type()?;
            if player_ref == &PlayerType::PlayerRemote {
                count += 1;
            }
        }
        Ok(count)
    }

    pub fn get_player_count_party(&self) -> Result<usize, ErrorTypePlayerHandler> {
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        let count: usize = players.len();
        Ok(count)
    }

    pub fn get_party_ai_index_vec(&self) -> Result<Vec<usize>, ErrorTypePlayerHandler>   {
        let mut ai_index: Vec<usize> = Vec::new();
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        for (index, player) in players.iter().enumerate() {
            let player_type = player.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?;
            let player_ref = player_type.get_player_type()?;
            if player_ref == &PlayerType::PlayerAiLocal || player_ref == &PlayerType::PlayerAiRemote {
                ai_index.push(index);
            };
        }
        Ok(ai_index)
    }

    pub fn has_player_with_id(&self, target_id: Uuid) -> Result<bool, ErrorTypePlayerHandler> {
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        for player in players.iter() {
            let unwrapped_player = player.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?;
            let player_id = unwrapped_player.get_player_id()?;
            if player_id == &target_id {
                return Ok(true)
            }
        }
        Ok(false)
    }

    pub fn main_player_clone_player_id(&self) -> Result<Uuid, ErrorTypePlayerHandler>  {
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        let player_arc = &players[0]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let player = player_arc.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?; // Lock the player mutex to get a mutable reference to the player
        let player_id = player.get_player_id()?;
        Ok(player_id.clone())
    }

    pub fn party_size(&self) -> Result<usize, ErrorTypePlayerHandler> {        
        // First, lock the players mutex to get access to the Vec
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;

        // Grab the size of the party
        let party_size = &players.len();
        Ok(*party_size)
    }

    pub fn player_set_player_id(&mut self, player_idx: usize, new_id: Uuid) -> Result<(), ErrorTypePlayerHandler> {
        let players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        let player_arc = &players[player_idx]; // adjusted for 1 indexing // Get the active player (Arc<Mutex<Player>>)
        let mut player = player_arc.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?; // Lock the player mutex to get a mutable reference to the player
        player.set_player_id(new_id)?;

        Ok(())
    }

    pub fn players_add_player(&self, player: Arc<Mutex<dyn Player + Send>>) -> Result<(), ErrorTypePlayerHandler> {
        let mut players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        let party_size = self.party_size as usize;
        if players.len() < party_size {
            players.push(player);
        } else {
            warn!("Error: Party full!");
        }

        Ok(())
    }

    pub fn players_add_player_at_index(&self, idx: usize, player: Arc<Mutex<dyn Player + Send>>) -> Result<(), ErrorTypePlayerHandler> {
        let mut players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        let party_size = self.party_size as usize;
        if players.len() < party_size {
            players.insert(idx, player);
        } else {
            warn!("Error: Party full!");
        }

        Ok(())
    }
    
    pub fn players_remove_ai_local(&self) -> Result<(), ErrorTypePlayerHandler> {
        let mut players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
    
        // Iterate through players and find the index of the first occurrence of "PlayerAi".
        if let Some(index) = players.iter().position(|player| {
            let player = player.lock().unwrap();
            let player_type = player.get_player_type().unwrap();
            player_type == &PlayerType::PlayerAiLocal
        }) {
            // Remove the player at the found index
            players.remove(index);
        }

        Ok(())
    }
    
    pub fn players_remove_ai_remote(&self) -> Result<(), ErrorTypePlayerHandler> {
        let mut players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
    
        // Iterate through players and find the index of the first occurrence of "PlayerAi".
        if let Some(index) = players.iter().position(|player| {
            let player = player.lock().unwrap();
            let player_type = player.get_player_type().unwrap();
            player_type == &PlayerType::PlayerAiRemote
        }) {
            // Remove the player at the found index
            players.remove(index);
        }

        Ok(())
    }
    
    pub fn players_remove_last_player(&self) -> Result<(), ErrorTypePlayerHandler> {
        let mut players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        
        // Only pop if we have more than one player
        if players.len() > 1 {
            players.pop();
        }

        Ok(())
    }
    
    pub fn players_remove_local_player(&self) -> Result<(), ErrorTypePlayerHandler> {
        let mut players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;
        
        // Only pop if we have more than one player
        if players.len() > 1 {
            let mut rev_index_opt = None;
            for (idx, player) in players.iter().rev().enumerate() {
                let target = player.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayer")))?;
                let player_type = target.get_player_type().unwrap();
                if player_type == &PlayerType::PlayerLocal {
                    rev_index_opt = Some(idx);
                    break;
                }
            }
            if let Some(rev_index) = rev_index_opt {
                    // Convert the reversed index to the original index
                    let original_index = players.len() - 1 - rev_index;
        
                    // Remove the player at the original index
                    players.remove(original_index);

                    return Ok(())
            };
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("Failed: Reverse Index is None")))
        }
        warn!("Party Action: Failed to remove player only main remains");
        Ok(())
    }

    pub fn players_remove_player(&mut self, player_id: &Uuid, plugin: &ResMut<BevyEasyPlayerHandlerPlugin>) -> Result<(), ErrorTypePlayerHandler> {
        let main_player_id = plugin.get_main_player_uuid()?;
        let main_player_id = main_player_id.unwrap();

        if main_player_id == player_id {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("players_remove_player: Failure to remove [{}] Can't remove local host from party", &player_id)));
        }

        let count: usize = self.get_player_count_party()?;
        let count_i32 = count as i32;
        let player_idx = self.active_player_get_index()?;

        // Adjust index in preperation for the resulting indexing overflow limit.
        if player_idx == count_i32 {
            self.active_player_set(player_idx - 1)?;
        }

        let mut players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;

        // Proceed only if we have more than one player in the vector
        if players.len() > 1 {
            players.retain(|player| {
                let target = player.lock().unwrap();
                let target_id = target.get_player_id().unwrap(); 
                target_id != player_id
            });
        };

        Ok(())
    }

    pub fn reorder_players(&mut self, old_index: usize, new_index: usize) -> Result<(), ErrorTypePlayerHandler> {
        let mut players = self.players.lock().map_err(|_| ErrorTypePlayerHandler::LockFailed(format!("ArcMutexPlayersVec")))?;

        
        if old_index >= players.len() || new_index >= players.len() {
            let msg = format!(
                "Index out of range: old={} new={} len={}",
                old_index, new_index, players.len()
            );
            return Err(ErrorTypePlayerHandler::IndexOutOfBounds(msg));
        }
        
        if old_index != new_index {
            let player = players.remove(old_index);
            players.insert(new_index, player);
        }
    
        Ok(())
    }
    
    pub fn update_ai_index_vec(&mut self) -> Result<(), ErrorTypePlayerHandler> {
        self.ai_vec = Some(self.get_party_ai_index_vec()?);
        Ok(())
    }
}