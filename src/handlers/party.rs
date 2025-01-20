use bevy::prelude::*;

use bevy_easy_shared_definitions::ErrorTypePlayerHandler;

use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    BevyEasyPlayerHandlerPlugin, 
    Party, 
    PlayerComponent,
    PlayerType,
};

macro_rules! player_query_get_player_lock {
    ($player_query:expr, $target_uuid:expr) => {{
        let mut player_match: Option<&PlayerComponent> = None;
        for player in $player_query.iter() {
            let player_lock = match player.player.lock() {
                Ok(player) => player,
                Err(e) => return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e))),
            };
            let player_id = player_lock.get_player_id().unwrap();
            if $target_uuid == player_id {
                player_match = Some(player);
            }
        }
        player_match

    }};
}

impl Party {
    pub fn new() -> Self {
        let active_player: usize = 1;
        let main_player_uuid: Option<Uuid> = None;
        let player_map: HashMap<usize, Uuid> = HashMap::new();
        Party {
            active_player,
            main_player_uuid,
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
        let result = self.player_map.get(&self.active_player); //
        if result.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("active_player_get_uuid")))
        }
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
    ) -> Result<(), ErrorTypePlayerHandler> {
        self.active_player = target;
        Ok(())
    }

    pub fn clone_active_player_player_type(
        &self,
        player_query: &Query<&PlayerComponent>,
    ) -> Result<PlayerType, ErrorTypePlayerHandler> {
        let target_uuid = self.get_player_map_active_player_uuid()?;
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        if player_component.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        let player_component = player_component.unwrap();
        let player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            }
        };
        let player_type = player_mutex.get_player_type()?.clone();
        drop(player_mutex);
        Ok(player_type.to_owned())
    }

    pub fn clone_active_player_player_email(
        &self,
        player_query: &Query<&PlayerComponent>,
    ) -> Result<String, ErrorTypePlayerHandler> {
        let target_uuid = self.get_player_map_active_player_uuid()?;
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        if player_component.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        let player_component = player_component.unwrap();
        let player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            },
        };
        let player_email = player_mutex.get_player_email()?.clone();
        drop(player_mutex);
        Ok(player_email.to_owned())
    }

    pub fn clone_active_player_uuid(
        &self,
        player_query: &Query<&PlayerComponent>,
    ) -> Result<Uuid, ErrorTypePlayerHandler> {
        let target_uuid = self.get_player_map_active_player_uuid()?;
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        if player_component.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        let player_component = player_component.unwrap();
        let player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            }        };
        let player_id = player_mutex.get_player_id()?.clone();
        drop(player_mutex);
        Ok(player_id)
    }

    pub fn clone_active_player_player_username(
        &self, 
        player_query: &Query<&PlayerComponent>,
    ) -> Result<String, ErrorTypePlayerHandler> {
        let target_uuid = self.get_player_map_active_player_uuid()?;
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        if player_component.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        let player_component = player_component.unwrap();
        let player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            }
        };
        let player_username = player_mutex.get_player_username()?.to_owned();
        drop(player_mutex);
        Ok(player_username.to_owned())
    }

    pub fn set_active_player_email(
        &mut self, 
        player_query: &Query<&PlayerComponent>, 
        player_email: &str,
    ) -> Result<(), ErrorTypePlayerHandler> {
        let target_uuid = self.get_player_map_active_player_uuid()?;
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        if player_component.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        let player_component = player_component.unwrap();
        let mut player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            }
        };
        player_mutex.set_player_email(player_email)?;
        drop(player_mutex);
        Ok(())
    }

    pub fn set_active_player_username(
        &mut self, 
        player_query: &Query<&PlayerComponent>, 
        player_username: &str,
    ) -> Result<(), ErrorTypePlayerHandler> {
        let target_uuid = self.get_player_map_active_player_uuid()?;
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        if player_component.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        let player_component = player_component.unwrap();
        let mut player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => {
                return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e)));
            }
        };
        player_mutex.set_player_username(player_username)?;
        drop(player_mutex);
        Ok(())
    }

    pub fn set_active_player_uuid_player_map_and_component(
        &mut self, 
        player_query: &Query<&PlayerComponent>, 
        new_uuid: Uuid
    ) -> Result<(), ErrorTypePlayerHandler> {
        let active_player_uuid = *self.get_player_map_active_player_uuid()?;
        if active_player_uuid == new_uuid {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("active_player_set_uuid failed: new id matches existing")));
        }
        self.set_active_player_uuid_player_component(player_query, &new_uuid)?;
        self.set_active_player_uuid_player_map(&new_uuid)?;
        let updated_active_player_uuid = self.get_player_map_active_player_uuid()?;
        if active_player_uuid == *updated_active_player_uuid {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("active_player_set_uuid failed: id update failed, id matches original")));
        }
        Ok(())
    }

    pub fn init_main_player_uuid_player_map(
        &mut self, 
        player_query: &Query<&PlayerComponent>, 
        new_uuid: Uuid
    ) -> Result<(), ErrorTypePlayerHandler> {
        let active_player = match player_query.single().player.lock() {
            Ok(uuid) => uuid,
            Err(e) => return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e))),
        };
        let active_player_uuid = active_player.get_player_id()?.clone();
        drop(active_player);
        if &active_player_uuid == &new_uuid {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("init_main_player_uuid_player_map failed: new id matches existing")));
        }
        self.set_active_player_uuid_player_component(player_query, &new_uuid)?;
        self.set_active_player_uuid_player_map(&new_uuid)?;
        let updated_active_player_uuid = self.get_player_map_active_player_uuid()?;
        if &active_player_uuid == updated_active_player_uuid {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("active_player_set_uuid failed: id update failed, id matches original")));
        }
        Ok(())
    }

    pub fn set_active_player_uuid_player_component(
        &mut self, 
        player_query: &Query<&PlayerComponent>, 
        new_uuid: &Uuid,
    ) -> Result<(), ErrorTypePlayerHandler> {
        let target_uuid = self.get_player_map_active_player_uuid()?;
        let player_component = player_query_get_player_lock!(player_query, target_uuid);
        if player_component.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_active_player_player_type -> player_component.is_none()")))
        }
        let player_component = player_component.unwrap();
        let mut player_mutex = match player_component.player.lock(){
            Ok(player) => player,
            Err(e) => return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e))),
        };
        player_mutex.set_player_id(*new_uuid)?;
        drop(player_mutex);
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
        let mut return_id: Option<Uuid> = None;
        for player in player_query.iter() {
            let player_container = &player.player;
            let player_lock = match player_container.lock() {
                Ok(uuid) => uuid,
                Err(e) => return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("{}", e))),
            };
            let player_type = player_lock.get_player_type()?;
            if player_type == &PlayerType::PlayerMain {
                let player_uuid = player_lock.get_player_id()?;
                return_id = Some(*player_uuid);
            }
        }
        if return_id.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("clone_main_player_uuid failed: return_id is None")))
        }
        let return_value = return_id.unwrap();
        Ok(return_value)
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
        let mut id_type_storage: Vec<(Uuid, PlayerType)> = Vec::new();
        for player in player_query.iter() {
            let player_mutex = player.player.lock().unwrap();
            let player_id = player_mutex.get_player_id()?.clone();
            let player_type = player_mutex.get_player_type()?.clone();
            drop(player_mutex);
            id_type_storage.push((player_id, player_type));
        }
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
        let exists_player_map = match self.verify_player_exists_player_map_uuid(target_id) {
            Ok(bool) => bool,
            Err(e) => return Err(e),
        };
        let exists_player_component = match self.verify_player_exists_player_component(player_query, target_id) {
            Ok(bool) => bool,
            Err(e) => return Err(e),
        };
        Ok((exists_player_map, exists_player_component))
    }

    pub fn verify_player_exists_player_map_uuid(
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

    pub fn verify_player_exists_player_map_index(
        &self, 
        target_idx: usize,
    ) -> Result<bool, ErrorTypePlayerHandler> {
        let mut exists = false;
        let player_map = &self.player_map;
        for player in player_map {
            if &target_idx == player.0 {
                exists = true;
                return Ok(exists)
            }
        }
        return Ok(exists)
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
        &mut self,
        commands: &mut Commands,
        entity_player_query: &Query<(Entity, &PlayerComponent)>,
        plugin: &mut ResMut<BevyEasyPlayerHandlerPlugin>,
        target_player: &Uuid,
    ) -> Result<(), ErrorTypePlayerHandler> {
        let main_player = plugin.get_main_player_uuid()?.unwrap();
        if main_player == target_player {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("Unable to remove player... Main Player is local host...")))
        }
        let mut despawned = false;
        for (entity, player) in entity_player_query {
            let player_mutex = player.player.lock().unwrap();
            let player_id = player_mutex.get_player_id()?;
            if player_id == target_player {
                despawned = true;
                commands.entity(entity).despawn_recursive();
            }
            drop(player_mutex);
        }
        if despawned {
            self.player_map_remove_player(plugin, target_player)?;
            return Ok(());
        } else {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("remove_player: Failed... Target does not exist...")));
        }
    }

    pub fn player_map_remove_player(
        &mut self,
        plugin: &mut ResMut<BevyEasyPlayerHandlerPlugin>,
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

        self.player_map_check_for_players_and_collapse_missing(plugin)?;

        Ok(())
    }

    pub fn get_main_player_uuid(
        &self,
    ) -> Result<Uuid, ErrorTypePlayerHandler> {
        let result = self.main_player_uuid;
        if result.is_none() {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("get_main_player_uuid failed... if result.is_none()")))
        }
        Ok(result.unwrap())
    }

    pub fn player_map_and_component_remove_all_players_besides_main(
        &mut self,
        commands: &mut Commands,
        entity_player_query: &Query<(Entity, &PlayerComponent)>, 
        plugin: &mut ResMut<BevyEasyPlayerHandlerPlugin>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        self.set_active_player_index(1)?;
        let main_player_id = self.get_main_player_uuid()?;
        // let main_player_id = main_player_id.expect("main_player_id unwrap failed");
        for (entity, player) in entity_player_query.iter() {
            let player_mutex = match player.player.lock() {
                Ok(mutex) => mutex,
                Err(e) => return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("player_map_and_component_remove_all_players failed: [{:?}]", e)))
            };
            let player_id = player_mutex.get_player_id()?;
            if player_id != &main_player_id {
                commands.entity(entity).despawn_recursive();    
                self.player_map_remove_player(plugin, player_id)?;            
            }
            drop(player_mutex);
        }
        Ok(())
    }

    pub fn player_map_and_component_remove_all_players(
        &mut self,
        commands: &mut Commands,
        entity_player_query: &Query<(Entity, &PlayerComponent)>, 
        plugin: &mut ResMut<BevyEasyPlayerHandlerPlugin>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        self.set_active_player_index(1)?;
        for (entity, player) in entity_player_query.iter() {
            let player_mutex = match player.player.lock() {
                Ok(mutex) => mutex,
                Err(e) => return Err(ErrorTypePlayerHandler::PoisonErrorBox(format!("player_map_and_component_remove_all_players failed: [{:?}]", e)))
            };
            let player_id = player_mutex.get_player_id()?;
            self.player_map_remove_player(plugin, player_id)?;
            commands.entity(entity).despawn_recursive();                
            drop(player_mutex);
        }
        Ok(())
    }

    pub fn player_map_check_for_players_and_collapse_missing(
        &mut self,
        plugin: &mut ResMut<BevyEasyPlayerHandlerPlugin>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        let party_limit = plugin.get_party_size_limit()?;
        if party_limit.is_none() {
            return Err(ErrorTypePlayerHandler::PluginDataRetreivalFailed(format!("plugin.get_party_size_limit()? is None...")))
        }
        let party_limit: usize = *party_limit.unwrap();
        let mut party_idx: usize = 0;

        // Build the reorder reference vec
        let mut reorder_list: Vec<(usize, bool)> = Vec::new(); 
        loop {
            party_idx += 1;
            reorder_list.push((party_idx, false));
            if party_idx == party_limit {
                break;
            }
            else {
                continue;
            }
        }

        // iterate through the reorder reference vec and identify player_map entries that need to be shifted
        loop {
            // Update the reorder reference vec with real values
            for n in 0..party_limit {
                if self.verify_player_exists_player_map_index(n + 1)? { //indexed logic
                    reorder_list[n].1 = true;
                } else {
                    reorder_list[n].1 = false;
                }
            }
            // create containers for first false and next existing
            let mut first_false: Option<usize> = None;
            let mut next_existing: Option<usize> = None;
            
            // Identify first false and next existing
            for entry in reorder_list.iter() {
                match entry.1 {
                    false => {
                        first_false = Some(entry.0);
                    },
                    true => {
                        if first_false.is_some() {
                            next_existing = Some(entry.0);
                            break;
                        }
                    },
                }
            }
            if first_false.is_none() {
                return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("player_map_check_for_players_and_collapse_missing failed: [ Party full, no index gap to collapse ]")))
            };
            if next_existing.is_none() {
                break;
            };
            let target = next_existing.unwrap();
            let target_uuid = self.player_map.get(&target);
            if target_uuid.is_none() {
                return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("player_map_check_for_players_and_collapse_missing -> extracted_uuid.is_none()")))
            };
            let extracted_uuid = target_uuid.unwrap();
            self.player_map.insert(first_false.unwrap(), *extracted_uuid);
            self.player_map.remove(&next_existing.unwrap());
        }
        Ok(())
    }

    pub fn player_map_swap_players(
        &mut self, 
        old_index: usize, 
        new_index: usize,
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
        
        self.set_active_player_index(old_index)?;
        self.set_active_player_uuid_player_map(&new_uuid)?;
        self.set_active_player_index(new_index)?;
        self.set_active_player_uuid_player_map(&old_uuid)?;

        Ok(())
    }
}