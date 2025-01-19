use bevy::prelude::*;

use bevy_easy_shared_definitions::ErrorTypePlayerHandler;

use std::collections::HashMap;
// use std::sync::Arc;
// use std::sync::Mutex;
// use std::sync::MutexGuard;

use uuid::Uuid;

use crate::{
    // BevyEasyPlayerHandlerPlugin,
    BevyEasyPlayerHandlerPlugin, Party, PlayerComponent, PlayerType
};

macro_rules! player_query_get_player_lock {
    ($player_query:expr, $target_uuid:expr) => {{
        println!("MACRO: player_query_get_player_lock:");
        println!("MACRO Step: 1 [ player_query_get_player_lock ]");
        let mut player_match: Option<&PlayerComponent> = None;
        println!("MACRO Step: 2 [ player_query_get_player_lock ]");
        for player in $player_query.iter() {
            println!("MACRO Step: 3 [ player_query_get_player_lock ]");
            let player_lock = match player.player.lock() {
                Ok(player) => player,
                Err(e) => return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e))),
            };
            println!("MACRO Step: 4 [ player_query_get_player_lock ]");
            let player_id = player_lock.get_player_id().unwrap();
            println!("player_id: [ {} ]", &player_id);
            println!("MACRO Step: 5 [ player_query_get_player_lock ]");
            if $target_uuid == player_id {
                println!("MACRO Step: 6 [ player_query_get_player_lock ]");
                player_match = Some(player);
            }
        }
        println!("MACRO Step: 7 [ player_query_get_player_lock ]");
        player_match

    }};
}

impl Party {
    pub fn new() -> Self {
        let active_player: usize = 1;
        let player_map: HashMap<usize, Uuid> = HashMap::new();
        Party {
            active_player,
            player_map,
        } 
    }

    pub fn clone_player_map(
        &self
    ) -> Result<HashMap<usize, Uuid>, ErrorTypePlayerHandler> {
        let result = self.player_map.clone(); //
        Ok(result)
    }

    pub fn get_player_map_active_player_uuid(
        &self
    ) -> Result<&Uuid, ErrorTypePlayerHandler> {
        println!("Init: get_player_map_active_player_uuid:");
        println!("Step: 1 [ get_player_map_active_player_uuid ]");
        let result = self.player_map.get(&self.active_player); //
        println!("Step: 2 [ get_player_map_active_player_uuid ]");
        if result.is_none() {
            println!("Error: 1 [ get_player_map_active_player_uuid ] -> result.is_none()");
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("active_player_get_uuid")))
        }
        println!("Step: 3 [ get_player_map_active_player_uuid ]");
        Ok(result.unwrap())
    }

    pub fn clone_player(
        &self,
        target_uuid: &Uuid,
        player_query: &Query<&PlayerComponent>,
    ) -> Result<PlayerComponent, ErrorTypePlayerHandler> {
        let mut player_match: Vec<&PlayerComponent> = 
            player_query
                .iter().filter(
                    |player|{
                        let player_mutex = player.to_owned().player.lock().unwrap();
                        let player_id = player_mutex.get_player_id().unwrap().clone();
                        drop(player_mutex);
                        return target_uuid == &player_id;
                    }
                )
                .collect();
            
        let player = player_match.remove(0);
        drop(player_match);
        let player_component: PlayerComponent = PlayerComponent{ player: player.player.clone() };
        Ok(player_component)
    }

    pub fn get_active_player_index(&self) -> Result<usize, ErrorTypePlayerHandler> {
        let active_player = self.active_player;
        Ok(active_player)
    }

    pub fn set_active_player_index(
        &mut self, 
        target: usize, 
        player_query: &Query<&PlayerComponent>
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: set_active_player_index:");
        println!("Old: [ {} ], New: [ {} ], MapMax: [ {} ], QueryMax: [ {} ]", 
            self.active_player, 
            target, 
            self.player_map.len(), 
            {
                let mut count = 0; 
                for _ in player_query.iter() { 
                    count += 1
                }; 
                count
            }
        );
        println!("Step 1 [ set_active_player_index ]");
        self.active_player = target;
        println!("Updated: [ {} ]", self.active_player);
        println!("Complete [ set_active_player_index ]");
        Ok(())
    }

    pub fn clone_active_player_player_type(
        &self,
        player_query: &Query<&PlayerComponent>,
    ) -> Result<PlayerType, ErrorTypePlayerHandler> {
        println!("Init: clone_active_player_player_type:");
        println!("Step: 1 [ clone_active_player_player_type ]");
        let target_uuid = self.get_player_map_active_player_uuid()?;
        println!("Step: 2 [ clone_active_player_player_type ]");
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        println!("Step: 3 [ clone_active_player_player_type ]");
        if player_component.is_none() {
            println!("Error: 1 [ clone_active_player_player_type ]");
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        println!("Step: 4 [ clone_active_player_player_type ]");
        let player_component = player_component.unwrap();
        println!("Step: 5 [ clone_active_player_player_type ]");
        let player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                println!("Error: 2 [ clone_active_player_player_type ]");
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            }
        };
        println!("Step: 6 [ clone_active_player_player_type ]");
        let player_type = player_mutex.get_player_type()?.clone();
        println!("Step: 7 [ clone_active_player_player_type ]");
        drop(player_mutex);
        println!("Step: 8 [ clone_active_player_player_type ]");
        Ok(player_type.to_owned())
    }

    pub fn clone_active_player_player_email(
        &self,
        player_query: &Query<&PlayerComponent>,
    ) -> Result<String, ErrorTypePlayerHandler> {
        println!("Init: clone_active_player_player_email:");
        println!("Step: 1 [ clone_active_player_player_email ]");
        let target_uuid = self.get_player_map_active_player_uuid()?;
        println!("Step: 2 [ clone_active_player_player_email ]");
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        println!("Step: 3 [ clone_active_player_player_email ]");
        if player_component.is_none() {
            println!("Error: 1 [ clone_active_player_player_email ]");
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        println!("Step: 4 [ clone_active_player_player_email ]");
        let player_component = player_component.unwrap();
        println!("Step: 5 [ clone_active_player_player_email ]");
        let player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                println!("Error: 2 [ clone_active_player_player_email ]");
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            },
        };
        println!("Step: 6 [ clone_active_player_player_email ]");
        let player_email = player_mutex.get_player_email()?.clone();
        println!("Step: 7 [ clone_active_player_player_email ]");
        drop(player_mutex);
        println!("Step: 8 [ clone_active_player_player_email ]");
        Ok(player_email.to_owned())
    }

    pub fn clone_active_player_uuid(
        &self,
        player_query: &Query<&PlayerComponent>,
    ) -> Result<Uuid, ErrorTypePlayerHandler> {
        println!("Init: clone_active_player_uuid:");
        println!("Step: 1 [ clone_active_player_uuid ]");
        let target_uuid = self.get_player_map_active_player_uuid()?;
        println!("Step: 2 [ clone_active_player_uuid ]");
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        println!("Step: 3 [ clone_active_player_uuid ]");
        if player_component.is_none() {
            println!("Error: 1 [ clone_active_player_uuid ]");
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        println!("Step: 4 [ clone_active_player_uuid ]");
        let player_component = player_component.unwrap();
        println!("Step: 5 [ clone_active_player_uuid ]");
        let player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                println!("Error: 2 [ clone_active_player_player_type ]");
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            }        };
        println!("Step: 6 [ clone_active_player_uuid ]");
        let player_id = player_mutex.get_player_id()?.clone();
        println!("Step: 7 [ clone_active_player_uuid ]");
        drop(player_mutex);
        println!("Step: 8 [ clone_active_player_uuid ]");
        Ok(player_id)
    }

    pub fn clone_active_player_player_username(
        &self, 
        player_query: &Query<&PlayerComponent>,
    ) -> Result<String, ErrorTypePlayerHandler> {
        println!("Init: clone_active_player_player_username:");
        println!("Step: 1 [ clone_active_player_player_username ]");
        let target_uuid = self.get_player_map_active_player_uuid()?;
        println!("Step: 2 [ clone_active_player_player_username ]");
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        println!("Step: 3 [ clone_active_player_player_username ]");
        if player_component.is_none() {
            println!("Error: 1 [ clone_active_player_player_username ]");
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        println!("Step: 4 [ clone_active_player_player_username ]");
        let player_component = player_component.unwrap();
        println!("Step: 5 [ clone_active_player_player_username ]");
        let player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                println!("Error: 2 [ clone_active_player_player_type ]");
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            }
        };
        println!("Step: 5 [ clone_active_player_player_username ]");
        let player_username = player_mutex.get_player_username()?.to_owned();
        println!("Step: 6 [ clone_active_player_player_username ]");
        drop(player_mutex);
        println!("Step: 7 [ clone_active_player_player_username ]");
        Ok(player_username.to_owned())
    }

    pub fn set_active_player_email(
        &mut self, 
        player_query: &Query<&PlayerComponent>, 
        player_email: &str,
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: set_active_player_email:");
        println!("Step: 1 [ set_active_player_email ]");
        let target_uuid = self.get_player_map_active_player_uuid()?;
        println!("Step: 2 [ set_active_player_email ]");
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        println!("Step: 3 [ set_active_player_email ]");
        if player_component.is_none() {
            println!("Error: 1 [ set_active_player_email ]");
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        println!("Step: 4 [ set_active_player_email ]");
        let player_component = player_component.unwrap();
        println!("Step: 5 [ set_active_player_email ]");
        let mut player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                println!("Error: 2 [ clone_active_player_player_type ]");
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            }
        };
        println!("Step: 6 [ set_active_player_email ]");
        player_mutex.set_player_email(player_email)?;
        println!("Step: 7 [ set_active_player_email ]");
        drop(player_mutex);
        println!("Step: 8 [ set_active_player_email ]");
        Ok(())
    }

    pub fn set_active_player_username(
        &mut self, 
        player_query: &Query<&PlayerComponent>, 
        player_username: &str,
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: set_active_player_username:");
        println!("Step: 1 [ set_active_player_username ]");
        let target_uuid = self.get_player_map_active_player_uuid()?;
        println!("Step: 2 [ set_active_player_username ]");
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        println!("Step: 3 [ set_active_player_username ]");
        if player_component.is_none() {
            println!("Error: 1 [ set_active_player_username ]");
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        println!("Step: 4 [ set_active_player_username ]");
        let player_component = player_component.unwrap();
        println!("Step: 5 [ set_active_player_username ]");
        let mut player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                println!("Error: 2 [ clone_active_player_player_type ]");
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            }
        };
        println!("Step: 6 [ set_active_player_username ]");
        player_mutex.set_player_username(player_username)?;
        println!("Step: 7 [ set_active_player_username ]");
        drop(player_mutex);
        println!("Step: 8 [ set_active_player_username ]");
        Ok(())
    }

    pub fn set_active_player_uuid_player_map_and_component(
        &mut self, 
        player_query: &Query<&PlayerComponent>, 
        new_uuid: Uuid
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: set_active_player_uuid_player_map_and_component:");
        println!("Step: 1 [ set_active_player_uuid_player_map_and_component ]");
        let active_player_uuid = *self.get_player_map_active_player_uuid()?;
        println!("Step: 2 [ set_active_player_uuid_player_map_and_component ]");
        if active_player_uuid == new_uuid {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("active_player_set_uuid failed: new id matches existing")));
        }
        println!("Step: 3 [ set_active_player_uuid_player_map_and_component ]");
        self.set_active_player_uuid_player_component(player_query, &new_uuid)?;
        println!("Step: 4 [ set_active_player_uuid_player_map_and_component ]");
        self.set_active_player_uuid_player_map(&new_uuid)?;
        println!("Step: 5 [ set_active_player_uuid_player_map_and_component ]");
        let updated_active_player_uuid = self.get_player_map_active_player_uuid()?;
        println!("Step: 6 [ set_active_player_uuid_player_map_and_component ]");
        if active_player_uuid == *updated_active_player_uuid {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("active_player_set_uuid failed: id update failed, id matches original")));
        }
        println!("Step: 7 [ set_active_player_uuid_player_map_and_component ]");
        Ok(())
    }

    pub fn init_main_player_uuid_player_map(
        &mut self, 
        player_query: &Query<&PlayerComponent>, 
        new_uuid: Uuid
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: init_main_player_uuid_player_map:");
        println!("Step: 1 [ init_main_player_uuid_player_map ]");
        let active_player = match player_query.single().player.lock() {
            Ok(uuid) => uuid,
            Err(e) => return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e))),
        };
        println!("Step: 2 [ init_main_player_uuid_player_map ]");
        let active_player_uuid = active_player.get_player_id()?.clone();
        println!("Step: 3 [ init_main_player_uuid_player_map ]");
        drop(active_player);
        println!("Step: 4 [ init_main_player_uuid_player_map ]");
        if &active_player_uuid == &new_uuid {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("init_main_player_uuid_player_map failed: new id matches existing")));
        }
        println!("Step: 5 [ init_main_player_uuid_player_map ]");
        self.set_active_player_uuid_player_component(player_query, &new_uuid)?;
        println!("Step: 6 [ init_main_player_uuid_player_map ]");
        self.set_active_player_uuid_player_map(&new_uuid)?;
        println!("Step: 7 [ init_main_player_uuid_player_map ]");
        let updated_active_player_uuid = self.get_player_map_active_player_uuid()?;
        println!("Step: 8 [ init_main_player_uuid_player_map ]");
        if &active_player_uuid == updated_active_player_uuid {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("active_player_set_uuid failed: id update failed, id matches original")));
        }
        println!("Step: 9 [ init_main_player_uuid_player_map ]");
        Ok(())
    }

    pub fn set_active_player_uuid_player_component(
        &mut self, 
        player_query: &Query<&PlayerComponent>, 
        new_uuid: &Uuid,
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: set_active_player_uuid_player_component:");
        println!("Step: 1 [ set_active_player_uuid_player_component ]");
        let target_uuid = self.get_player_map_active_player_uuid()?;
        println!("Step: 2 [ set_active_player_uuid_player_component ]");
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        println!("Step: 3 [ set_active_player_uuid_player_component ]");
        if player_component.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        println!("Step: 4 [ set_active_player_uuid_player_component ]");
        let player_component = player_component.unwrap();
        println!("Step: 5 [ set_active_player_uuid_player_component ]");
        let mut player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e))),
        };
        println!("Step: 6 [ set_active_player_uuid_player_component ]");
        player_mutex.set_player_id(*new_uuid)?;
        drop(player_mutex);
        println!("Step: 7 [ set_active_player_uuid_player_component ]");
        Ok(())
    }

    pub fn set_active_player_uuid_player_map(
        &mut self, 
        new_uuid: &Uuid,
    ) -> Result<(), ErrorTypePlayerHandler> {
        let target_index = self.active_player;
        let player_map = &mut self.player_map;
        player_map.entry(target_index).and_modify(
            |uuid| 
            {*uuid = *new_uuid}
        );
        Ok(())
    }

    pub fn clone_main_player_uuid(
        &self, 
        player_query: &Query<&PlayerComponent>, 
    ) -> Result<Uuid, ErrorTypePlayerHandler> {
        println!("Init: clone_main_player_uuid");
        println!("Step: 1 [ clone_main_player_uuid ]");
        let mut return_id: Option<Uuid> = None;
            println!("Step: 2 [ clone_main_player_uuid ]");
        for player in player_query.iter() {
            println!("Step: 3 [ clone_main_player_uuid ]");
            let player_container = &player.player;
            println!("Step: 4 [ clone_main_player_uuid ]");
            let player_lock = match player_container.lock() {
                Ok(uuid) => uuid,
                Err(e) => return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e))),
            };
            println!("Step: 5 [ clone_main_player_uuid ]");
            let player_type = player_lock.get_player_type()?;
            println!("Step: 6 [ clone_main_player_uuid ]");
            if player_type == &PlayerType::PlayerMain {
                println!("Step: 7 [ clone_main_player_uuid ]");
                let player_uuid = player_lock.get_player_id()?;
                println!("Step: 8 [ clone_main_player_uuid ]");
                return_id = Some(*player_uuid);
            }
        }
        println!("Step: 9 [ clone_main_player_uuid ]");
        if return_id.is_none() {
            println!("Step: 10 [ clone_main_player_uuid ]");
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_main_player_uuid failed: return_id is None")))
        }
        println!("Step: 10 [ clone_main_player_uuid ]");
        let return_value = return_id.unwrap();
        println!("Step: 11 [ clone_main_player_uuid ]");
        Ok(return_value)

        // return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("Could not find PlayerMain in queried PlayerComponents")))
        // println!("Step: 13 [ clone_main_player_uuid ]");
    }

    pub fn get_all_players_ids(
        &self, 
        player_query: &Query<&PlayerComponent>, 
    ) -> Result<Vec<Uuid>, ErrorTypePlayerHandler> {
        let mut id_storage: Vec<Uuid> = Vec::new();
        for player in player_query.iter() {
            let player_mutex = player.player.lock().unwrap();
            let player_id = player_mutex.get_player_id()?.clone();
            drop(player_mutex);
            id_storage.push(player_id);
        }
        Ok(id_storage)
    }

    pub fn get_all_players_ids_and_types(
        &self, 
        player_query: &Query<&PlayerComponent>, 
    ) -> Result<Vec<(Uuid, PlayerType)>, ErrorTypePlayerHandler> {
        println!("Init: get_all_players_ids_and_types");
        println!("Step: 1 [ get_all_players_ids_and_types ]");
        let mut id_type_storage: Vec<(Uuid, PlayerType)> = Vec::new();
        println!("Step: 2 [ get_all_players_ids_and_types ]");
        for player in player_query.iter() {
            println!("Step: 3 [ get_all_players_ids_and_types ]");
            let player_mutex = player.player.lock().unwrap();
            println!("Step: 4 [ get_all_players_ids_and_types ]");
            let player_id = player_mutex.get_player_id()?.clone();
            println!("Step: 5 [ get_all_players_ids_and_types ]");
            let player_type = player_mutex.get_player_type()?.clone();
            println!("Step: 6 [ get_all_players_ids_and_types ]");
            drop(player_mutex);
            println!("Step: 7 [ get_all_players_ids_and_types ]");
            id_type_storage.push((player_id, player_type));
        }
        println!("Step: 8 [ get_all_players_ids_and_types ]");
        Ok(id_type_storage)
    }

    pub fn get_player_count_party(
        &self, 
        player_query: &Query<&PlayerComponent>, 
    ) -> Result<usize, ErrorTypePlayerHandler> {
        let count_ai_all = self.get_player_count_ai_total(&player_query)?;
        let count_local = self.get_player_count_local(&player_query)?;
        let count_main = self.get_player_count_main(&player_query)?; // should always be 1
        let count_remote = self.get_player_count_remote(&player_query)?;
        Ok(count_ai_all + count_local + count_main + count_remote)
    }

    pub fn get_player_count_main(
        &self, 
        player_query: &Query<&PlayerComponent>, 
    ) -> Result<usize, ErrorTypePlayerHandler> {
        let mut count: usize = 0;
        for player in player_query.iter() {
            let player_mutex = player.player.lock().unwrap();
            let player_type = player_mutex.get_player_type()?;
            match player_type {
                &PlayerType::PlayerMain => count += 1,
                _ => {},
            };
            drop(player_mutex);
        };
        Ok(count)
    }

    pub fn get_player_count_ai_total(
        &self, 
        player_query: &Query<&PlayerComponent>, 
    ) -> Result<usize, ErrorTypePlayerHandler> {
        let count_ai = self.get_player_count_ai_local(&player_query)?;
        let count_remote = self.get_player_count_ai_remote(&player_query)?;
        Ok(count_ai + count_remote)
    }

    pub fn get_player_count_ai_local(
        &self, 
        player_query: &Query<&PlayerComponent>, 
    ) -> Result<usize, ErrorTypePlayerHandler> {
        let mut count: usize = 0;
        for player in player_query.iter() {
            let player_mutex = player.player.lock().unwrap();
            let player_type = player_mutex.get_player_type()?;
            match player_type {
                &PlayerType::PlayerAiLocal => count += 1,
                _ => {},
            };
            drop(player_mutex);
        };
        Ok(count)
    }

    pub fn get_player_count_ai_remote(
        &self, 
        player_query: &Query<&PlayerComponent>, 
    ) -> Result<usize, ErrorTypePlayerHandler> {
        let mut count: usize = 0;
        for player in player_query.iter() {
            let player_mutex = player.player.lock().unwrap();
            let player_type = player_mutex.get_player_type()?;
            match player_type {
                &PlayerType::PlayerAiRemote => count += 1,
                _ => {},
            };
            drop(player_mutex);
        };
        Ok(count)
    }

    pub fn get_player_count_local(
        &self, 
        player_query: &Query<&PlayerComponent>, 
    ) -> Result<usize, ErrorTypePlayerHandler> {
        let mut count: usize = 0;
        for player in player_query.iter() {
            let player_mutex = player.player.lock().unwrap();
            let player_type = player_mutex.get_player_type()?;
            match player_type {
                &PlayerType::PlayerLocal => count += 1,
                _ => {},
            };
            drop(player_mutex);
        };
        Ok(count)
    }

    pub fn get_player_count_remote(
        &self, 
        player_query: &Query<&PlayerComponent>, 
    ) -> Result<usize, ErrorTypePlayerHandler> {
        let mut count: usize = 0;
        for player in player_query.iter() {
            let player_mutex = player.player.lock().unwrap();
            let player_type = player_mutex.get_player_type()?;
            match player_type {
                &PlayerType::PlayerRemote => count += 1,
                _ => {},
            };
            drop(player_mutex);
        };
        Ok(count)
    }

    pub fn verify_player_exists_player_map_and_component(
        &self, 
        player_query: &Query<&PlayerComponent>, 
        target_id: &Uuid
    ) -> Result<(bool, bool), ErrorTypePlayerHandler> {
        let exists_player_map = match self.verify_player_exists_player_map(target_id) {
            Ok(bool) => bool,
            Err(e) => return Err(e),
        };
        let exists_player_component = match self.verify_player_exists_player_component(player_query, target_id) {
            Ok(bool) => bool,
            Err(e) => return Err(e),
        };
        Ok((exists_player_map, exists_player_component))
    }

    pub fn verify_player_exists_player_map(
        &self, 
        target_id: &Uuid,
    ) -> Result<bool, ErrorTypePlayerHandler> {
        let mut exists = false;
        let player_map = &self.player_map;
        for player in player_map {
            if target_id == player.1 {
                exists = true;
            }
        }        
        Ok(exists)
    }

    pub fn verify_player_exists_player_component(
        &self, 
        player_query: &Query<&PlayerComponent>, 
        target_id: &Uuid
    ) -> Result<bool, ErrorTypePlayerHandler> {
        let mut exists = false;
        for player in player_query.iter() {
            let player_mutex = player.player.lock().unwrap();
            let player_id = player_mutex.get_player_id()?;
            if target_id == player_id {
                exists = true;
            }
            drop(player_mutex);
        }
        Ok(exists)
    }

    pub fn get_party_local_ai_uuids_vec(
        &self,
        player_query: &Query<&PlayerComponent>, 
    ) -> Result<Vec<Uuid>, ErrorTypePlayerHandler> {
        let mut ai_index: Vec<Uuid> = Vec::new();
        for player in player_query.iter() {
            let player_mutex = player.player.lock().unwrap();
            let player_type = player_mutex.get_player_type()?;
            match player_type {
                &PlayerType::PlayerAiLocal => {
                    let player_id = player_mutex.get_player_id()?.to_owned();
                    ai_index.push(player_id);
                },
                _ => {},
            };
            drop(player_mutex);
        }
        Ok(ai_index)
    }
    
    pub fn remove_player_ai(
        &self,
        commands: &mut Commands,
        player_query: &Query<&PlayerComponent>, 
        entity_player_query: &Query<(Entity, &PlayerComponent)>, 
    ) -> Result<(), ErrorTypePlayerHandler> {
        // Check for Ai Uuid
        let ai_vec = self.get_party_local_ai_uuids_vec(player_query)?;
        if ai_vec.len() == 0 {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("remove_player_ai: Failed... PlayerAi vec is empty...")));
        }
        let target = ai_vec[0];

        // Find the first ai player
        for (entity, player) in entity_player_query.iter() {
            let player_mutex = player.player.lock().unwrap();
            let player_id = player_mutex.get_player_id()?;
            if &target == player_id {
                  commands.entity(entity).despawn_recursive();
                  return Ok(());
            };
            drop(player_mutex);
        }
        return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("remove_player_ai: Failed... Target does not exist...")));
    }
    
    pub fn remove_player(
        &self,
        commands: &mut Commands,
        entity_player_query: &Query<(Entity, &PlayerComponent)>,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        target_player: &Uuid,
    ) -> Result<(), ErrorTypePlayerHandler> {
        let main_player = plugin.get_main_player_uuid()?.unwrap();
        if main_player == target_player {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("Unable to remove player... Main Player is local host...")))
        }
        for (entity, player) in entity_player_query {
            let player_mutex = player.player.lock().unwrap();
            let player_id = player_mutex.get_player_id()?;
            let mut despawn = false;
            if player_id == target_player {
                despawn = true;
                commands.entity(entity).despawn_recursive();
            }
            drop(player_mutex);
            if despawn {
                return Ok(());
            }
        }
        return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("remove_player: Failed... Target does not exist...")));
    }

    pub fn remove_player_from_player_map(
        &mut self,
        target_player: &Uuid,
    ) -> Result<(), ErrorTypePlayerHandler> {
        // Step 1: Find the key to remove using an immutable borrow
        let target = self
            .player_map
            .iter()
            .find(|(_, player_uuid)| *player_uuid == target_player)
            .map(|(key, _)| *key);

        if target.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(
                "remove_player_from_player_map failed... Target does not exist...".to_string(),
            ));
        }

        // Step 2: Remove the key using a mutable borrow
        let target = target.unwrap();
        self.player_map.remove(&target);

        Ok(())
    }

    pub fn reorder_players(
        &mut self, 
        old_index: usize, 
        new_index: usize,
        player_query: &Query<&PlayerComponent>
    ) -> Result<(), ErrorTypePlayerHandler> {
        if old_index == new_index {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("reorder_players failed... new == old, No change...")))
        }
        let old_uuid = self
            .player_map
            .iter()
            .find(|(index, _)| *index == &old_index)
            .map(|(_, uuid)| *uuid);
        if old_uuid.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("old_uuid.is_none()")));
        };
        let old_uuid = old_uuid.unwrap();
        
        let new_uuid = self
            .player_map
            .iter()
            .find(|(index, _)| *index == &new_index)
            .map(|(_, uuid)| *uuid);
        if new_uuid.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("old_uuid.is_none()")));
        };
        let new_uuid = new_uuid.unwrap();
        
        self.set_active_player_index(old_index, player_query)?;
        self.set_active_player_uuid_player_map(&new_uuid)?;
        self.set_active_player_index(new_index, player_query)?;
        self.set_active_player_uuid_player_map(&old_uuid)?;

        Ok(())
    }
}