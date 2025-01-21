#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use bevy_easy_player_handler::*;
    use bevy_easy_shared_definitions::ErrorTypePlayerHandler;
    use std::{collections::HashMap,
        sync::{Arc, Mutex},
    };
    use uuid::Uuid;

    const PLAYER_EMAIL: &str = "test@example.com";
    const PLAYER_USERNAME: &str = "test_user";
    const ALT_PLAYER_EMAIL: &str = "another_test@example.com";
    const ALT_PLAYER_USERNAME: &str = "another_user";
    // const PLAYER_TYPE_AI_LOCAL: PlayerType = PlayerType::PlayerAiLocal;
    // const PLAYER_TYPE_AI_REMOTE: PlayerType = PlayerType::PlayerAiRemote;
    // const PLAYER_TYPE_LOCAL: PlayerType = PlayerType::PlayerLocal;
    // const PLAYER_TYPE_MAIN: PlayerType = PlayerType::PlayerMain;
    // const PLAYER_TYPE_REMOTE: PlayerType = PlayerType::PlayerRemote;

    #[test]
    fn test_party_new() -> Result<(), ErrorTypePlayerHandler> {
        let party: Party = Party::new(); 

        let active_player: usize = party.get_active_player_index()?;
        let main_player_uuid: Option<Uuid> = party.get_main_player_uuid()?;
        let player_map: HashMap<usize, Uuid> = party.get_player_map_clone()?;

        let ref_active_player: usize = 1;
        let ref_player_map: HashMap<usize, Uuid> = HashMap::new();

        assert_eq!(active_player, ref_active_player);
        assert_eq!(main_player_uuid, None);
        assert_eq!(player_map, ref_player_map);

        Ok(())
    }

    #[test]
    fn test_party_get_player_map_clone() -> Result<(), ErrorTypePlayerHandler> {
        let mut party: Party = Party::new();

        let player_uuid: Uuid = Uuid::now_v7();

        let mut reference_map: HashMap<usize, Uuid> = HashMap::new();
        reference_map.insert(1, player_uuid.clone());

        party.player_map.insert(1, player_uuid.clone());
        let clone: HashMap<usize, Uuid> = party.get_player_map_clone()?;
        
        assert_eq!(reference_map, clone);
        Ok(())
    }

    #[test]
    fn test_party_player_map() -> Result<(), ErrorTypePlayerHandler> {
        let mut party: Party = Party::new(); 
        
        let party_size: usize = party.player_map.len();
        let party_size_plus_one: usize = party_size + 1;

        let uuid: Option<Uuid> = Some(Uuid::now_v7());
        let uuid_unwrapped: Uuid = uuid.unwrap();

        party.player_map.insert(party_size_plus_one, uuid_unwrapped);
        
        let active_player_id = party.get_player_map_active_player_uuid()?;
        let active_player_id_unwrapped = active_player_id.unwrap().clone();
        assert_eq!(uuid_unwrapped, active_player_id_unwrapped);

        Ok(())
    }

    #[test]
    fn test_party_clone_player() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        // // Build a world to test in...
        // let mut world: World = World::new();
        // let mut party: Party = Party::new(); 

        // world.insert_resource(party);

        // // Build the player
        // let email = Some(PLAYER_EMAIL.to_string());
        // let username = Some(PLAYER_USERNAME.to_string());
        // let uuid = Some(Uuid::now_v7());

        // let player = PlayerLocal::new(email.clone(), username.clone(), uuid.clone(), PlayerType::PlayerLocal);
        // let player_container: Arc<Mutex<dyn Player + Send>> = Arc::new(Mutex::new(player));
        // let player_component = PlayerComponent {player: player_container};
        // let player_component_clone = player_component.clone();

        // world.spawn(player_component);
        
        // // Add a Party instance as a resource
        // let party = Party::new();
        // world.insert_resource(party);

        // // Retrieve a mutable reference to the Party resource
        // let mut party = world.get_resource_mut::<Party>().expect("Party resource not found");
    
        // // Prepare the QueryState
        // let mut player_query = world.query::<&PlayerComponent>();
    
        // // Test the `clone_player` function
        // let result = party.clone_player(&uuid.unwrap(), &player_query);
    
        // // Assert that the function worked as expected
        // assert!(result.is_ok(), "clone_player should return Ok");
        // let cloned_player = result.unwrap();
    
        // // Verify the cloned player's data matches the original
        // let original_player = player_component_clone.player.lock().unwrap();
        // let cloned_player_data = cloned_player.player.lock().unwrap();
    
        // assert_eq!(
        //     original_player.get_player_id().unwrap(),
        //     cloned_player_data.get_player_id().unwrap(),
        //     "The cloned player's UUID should match the original"
        // );
    
        Ok(())
    }

    #[test]
    fn test_party_set_main_player_uuid() -> Result<(), ErrorTypePlayerHandler> {
        let mut party: Party = Party::new(); 
        let uuid: Uuid = Uuid::now_v7();

        // --- party set_main_player --- //
        party.set_main_player_uuid(&uuid)?; 

        let main_player_uuid: Option<Uuid> = party.get_main_player_uuid()?;
        assert_eq!(main_player_uuid, Some(uuid));
        Ok(())
    }

    #[test]
    fn test_party_set_active_player_index() -> Result<(), ErrorTypePlayerHandler> {
        let mut party: Party = Party::new(); 
        let new_index: usize = 6;

        // --- party set_active_player_index --- //
        party.set_active_player_index(new_index)?;

        let player_index: usize = party.get_active_player_index()?;
        assert_eq!(player_index, new_index);
        Ok(())
    }

    #[test]
    fn test_party_set_active_player_username() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        // let mut party: Party = Party::new(); 
        // let new_username = ALT_PLAYER_USERNAME;

        // // --- party set_main_player --- //
        // party.set_active_player_username(player_query, new_username)?;

        // let main_player_uuid: Option<Uuid> = party.get_main_player_uuid()?;
        // assert_eq!(main_player_uuid, Some(uuid));
        Ok(())
    }

    #[test]
    fn test_party_clone_active_player_player_type() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_clone_active_player_player_email() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_clone_active_player_uuid() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_clone_active_player_player_username() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_set_active_player_email() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_set_active_player_uuid_player_map_and_component() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_init_main_player_uuid_player_map() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_set_active_player_uuid_player_component() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_set_active_player_uuid_player_map() -> Result<(), ErrorTypePlayerHandler> {
        let mut party: Party = Party::new(); 
        let old_uuid = Uuid::new_v4();
        let new_uuid = Uuid::now_v7();
        party.player_map.insert(1, old_uuid);
        party.set_active_player_uuid_player_map(&new_uuid)?;
        assert_eq!(party.player_map.get(&1), Some(&new_uuid));
        Ok(())
    }

    #[test]
    fn test_party_clone_main_player_uuid() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_get_all_players_ids() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_get_all_players_ids_and_types() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_get_player_count_party() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_get_player_count_main() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_get_player_count_ai_total() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_get_player_count_ai_local() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_get_player_count_ai_remote() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_get_player_count_local() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_get_player_count_remote() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_verify_player_exists_player_map_and_component() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_verify_player_exists_player_map_uuid() -> Result<(), ErrorTypePlayerHandler> {
        // pub fn verify_player_exists_player_map_uuid(
        //     &self, 
        //     target_id: &Uuid,
        // ) -> Result<bool, ErrorTypePlayerHandler> {
        //     let mut exists = false;
        //     let player_map = &self.player_map;
        //     for player in player_map {
        //         if target_id == player.1 {
        //             exists = true;
        //         }
        //     }        
        //     Ok(exists)
        // }
        let mut party: Party = Party::new(); 
        let old_uuid = Uuid::new_v4();
        let new_uuid = Uuid::now_v7();
        party.player_map.insert(1, old_uuid.clone());
        let reference_exists: bool = party.verify_player_exists_player_map_uuid(&old_uuid)?;
        let reference_missing: bool = party.verify_player_exists_player_map_uuid(&new_uuid)?;
        assert_eq!(reference_exists, true);
        assert_ne!(reference_missing, true);
        Ok(())
    }

    #[test]
    fn test_party_verify_player_exists_player_map_index() -> Result<(), ErrorTypePlayerHandler> {
        let mut party: Party = Party::new(); 
        let player_uuid = Uuid::new_v4();
        party.player_map.insert(1, player_uuid);
        let reference_exists: bool = party.verify_player_exists_player_map_index(1)?;
        let reference_missing: bool = party.verify_player_exists_player_map_index(2)?;
        assert_eq!(reference_exists, true);
        assert_ne!(reference_missing, true);
        Ok(())
    }

    #[test]
    fn test_party_verify_player_exists_player_component() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_get_party_local_ai_uuids_vec() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_remove_player_ai() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_remove_player() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_player_map_remove_player() -> Result<(), ErrorTypePlayerHandler> {
        // pub fn player_map_remove_player(
        //     &mut self,
        //     plugin: &mut ResMut<BevyEasyPlayerHandlerPlugin>,
        //     target_player: &Uuid,
        // ) -> Result<(), ErrorTypePlayerHandler> {
        //     // Step 1: Find the key to remove using an immutable borrow
        //     let target = self
        //         .player_map
        //         .iter()
        //         .find(|(_, player_uuid)| *player_uuid == target_player)
        //         .map(|(key, _)| *key);
    
        //     if target.is_none() {
        //         return Err(ErrorTypePlayerHandler::PartyActionFailed(
        //             "remove_player_from_player_map failed... Target does not exist...".to_string(),
        //         ));
        //     }
    
        //     // Step 2: Remove the key using a mutable borrow
        //     let target = target.unwrap();
        //     self.player_map.remove(&target);
    
        //     self.player_map_check_for_players_and_collapse_missing(plugin)?;
    
        //     Ok(())
        // }

        todo!("Plugin Logic");
        let mut party: Party = Party::new(); 
        let player_uuid = Uuid::new_v4();
        party.player_map.insert(1, player_uuid);
        let reference_exists: bool = party.verify_player_exists_player_map_index(1)?;
        // party.player_map_remove_player(plugin, target_player);
        let reference_missing: bool = party.verify_player_exists_player_map_index(1)?;
        Ok(())
    }
    
    #[test]
    fn test_party_player_map_and_component_remove_all_players_besides_main() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_player_map_and_component_remove_all_players() -> Result<(), ErrorTypePlayerHandler> {
        todo!("Query");
        Ok(())
    }

    #[test]
    fn test_party_player_map_check_for_players_and_collapse_missing() -> Result<(), ErrorTypePlayerHandler> {
        // pub fn player_map_check_for_players_and_collapse_missing(
        //     &mut self,
        //     plugin: &mut ResMut<BevyEasyPlayerHandlerPlugin>,
        // ) -> Result<(), ErrorTypePlayerHandler> {
        //     let party_limit = plugin.get_party_size_limit()?;
        //     if party_limit.is_none() {
        //         return Err(ErrorTypePlayerHandler::PluginDataRetreivalFailed(format!("plugin.get_party_size_limit()? is None...")))
        //     }
        //     let party_limit: usize = *party_limit.unwrap();
        //     let mut party_idx: usize = 0;
    
        //     // Build the reorder reference vec
        //     let mut reorder_list: Vec<(usize, bool)> = Vec::new(); 
        //     loop {
        //         party_idx += 1;
        //         reorder_list.push((party_idx, false));
        //         if party_idx == party_limit {
        //             break;
        //         }
        //         else {
        //             continue;
        //         }
        //     }
    
        //     // iterate through the reorder reference vec and identify player_map entries that need to be shifted
        //     loop {
        //         // Update the reorder reference vec with real values
        //         for n in 0..party_limit {
        //             if self.verify_player_exists_player_map_index(n + 1)? { //indexed logic
        //                 reorder_list[n].1 = true;
        //             } else {
        //                 reorder_list[n].1 = false;
        //             }
        //         }
        //         // create containers for first false and next existing
        //         let mut first_false: Option<usize> = None;
        //         let mut next_existing: Option<usize> = None;
                
        //         // Identify first false and next existing
        //         for entry in reorder_list.iter() {
        //             match entry.1 {
        //                 false => {
        //                     first_false = Some(entry.0);
        //                 },
        //                 true => {
        //                     if first_false.is_some() {
        //                         next_existing = Some(entry.0);
        //                         break;
        //                     }
        //                 },
        //             }
        //         }
        //         if first_false.is_none() {
        //             return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("player_map_check_for_players_and_collapse_missing failed: [ Party full, no index gap to collapse ]")))
        //         };
        //         if next_existing.is_none() {
        //             break;
        //         };
        //         let target = next_existing.unwrap();
        //         let target_uuid = self.player_map.get(&target);
        //         if target_uuid.is_none() {
        //             return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("player_map_check_for_players_and_collapse_missing -> extracted_uuid.is_none()")))
        //         };
        //         let extracted_uuid = target_uuid.unwrap();
        //         self.player_map.insert(first_false.unwrap(), *extracted_uuid);
        //         self.player_map.remove(&next_existing.unwrap());
        //     }
        //     Ok(())
        // }
        todo!("Plugin Logic");
        Ok(())
    }

    #[test]
    fn test_party_player_map_swap_players() -> Result<(), ErrorTypePlayerHandler> {
        // pub fn player_map_swap_players(
        //     &mut self, 
        //     old_index: usize, 
        //     new_index: usize,
        // ) -> Result<(), ErrorTypePlayerHandler> {
        //     if old_index == new_index {
        //         return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("reorder_players failed... new == old, No change...")))
        //     }
        //     let old_uuid = self
        //         .player_map
        //         .iter()
        //         .find(|(index, _)| *index == &old_index)
        //         .map(|(_, uuid)| *uuid);
        //     if old_uuid.is_none() {
        //         return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("old_uuid.is_none()")));
        //     };
        //     let old_uuid = old_uuid.unwrap();
            
        //     let new_uuid = self
        //         .player_map
        //         .iter()
        //         .find(|(index, _)| *index == &new_index)
        //         .map(|(_, uuid)| *uuid);
        //     if new_uuid.is_none() {
        //         return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("old_uuid.is_none()")));
        //     };
        //     let new_uuid = new_uuid.unwrap();
            
        //     self.set_active_player_index(old_index)?;
        //     self.set_active_player_uuid_player_map(&new_uuid)?;
        //     self.set_active_player_index(new_index)?;
        //     self.set_active_player_uuid_player_map(&old_uuid)?;
    
        //     Ok(())
        // }
        // Build the party and add some players
        let mut party: Party = Party::new(); 
        let original_player_1_uuid: Uuid = Uuid::new_v4();
        let original_player_2_uuid: Uuid = Uuid::now_v7();
        party.player_map.insert(1, original_player_1_uuid.clone());
        party.player_map.insert(2, original_player_2_uuid.clone());

        // Swap the players info and verify it maps correctly.
        party.player_map_swap_players(1, 2)?;
        let updated_player_1_uuid = party.player_map.get(&1).unwrap();
        let updated_player_2_uuid = party.player_map.get(&2).unwrap();

        // Establish the uuid's have been updated and do not match the originals
        assert_ne!(&original_player_1_uuid, updated_player_1_uuid);
        assert_ne!(&original_player_2_uuid, updated_player_2_uuid);

        // Establish the uuid's match the expected updated values
        assert_eq!(updated_player_1_uuid, &original_player_2_uuid);
        assert_eq!(updated_player_2_uuid, &original_player_1_uuid);
        Ok(())
    }
}