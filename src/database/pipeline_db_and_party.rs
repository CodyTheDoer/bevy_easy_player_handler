use bevy::prelude::*;

use bevy_easy_shared_definitions::DatabaseConnection;

use std::{
    env,
    env::VarError,
    sync::{
        Arc,
        Mutex,
    },
};

use dotenv::dotenv;
use rusqlite::Result;
use uuid::Uuid;

use crate::{
    BevyEasyPlayerHandlerPlugin, 
    DBPlayer, 
    ErrorType, 
    Party, 
    Player, 
    PlayerAi,
    PlayerHandlerDatabaseCommands, 
    PlayerLocal, PlayerType,
};

impl PlayerHandlerDatabaseCommands {
    pub fn pipeline_db_and_party_add_new_synced_player_ai_local(
        &self,
        db: &Res<DatabaseConnection>,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
        username: &str,
    ) -> Result<(), ErrorType> {
        info!("Init: pipeline_db_and_party_add_new_synced_player_ai_local:");

        // Party Size Management Checks
        match self.verify_if_party_size_exceeds_limit(&plugin, party) {
            Ok(()) => {},
            Err(e) => warn!("Error: pipeline_db_and_party_add_new_synced_player_ai_local -> verify_if_party_size_exceeds_limit [{:?}]", e),
        }

        // Init a new local player and add into the party
        let player_username = String::from(username);
        let new_player = PlayerAi::new(None, Some(player_username.clone()), PlayerType::PlayerAiLocal);
        let packaged_player = Arc::new(Mutex::new(new_player));
        party.players_add_player(packaged_player);

        // Get the new party size
        let party_size = party.get_player_count_party();

        // Update the active player to the new player
        party.active_player_set(party_size as i32);

        info!("Init Record: [{:?}]::[{}]", party.active_player_get_player_type(), party.active_player_get_player_id());

        // Build the new database record referencing the new player record in the party
        match self.action_insert_player_record(&db, party.active_player_get_player_id().to_string(), Some(String::from("PlayerAiLocal")), Some(player_username), PlayerType::PlayerAiLocal) {
            Ok(()) => {},
            Err(e) => {
                match e {
                    ErrorType::DatabaseLockPoisoned => warn!("Error: Failed to access Database lock poisoned..."),
                    ErrorType::DBActionFailedPlayerTableInsertRecordPlayerAiLocal => warn!("Error: Failed to insert new main player into player table..."),
                    _ => {warn!("Error: pipeline_db_and_party_add_new_synced_player_ai_local -> action_insert_player_record [{:?}]", e)},
                }
            }
        }

        Ok(())
    }

    pub fn pipeline_db_and_party_add_new_synced_player_local(
        &self,
        db: &Res<DatabaseConnection>,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
        username: &str,
    ) -> Result<(), ErrorType> {
        info!("Init: pipeline_db_and_party_add_new_synced_player_local:");

        // Party Size Management Checks
        match self.verify_if_party_size_exceeds_limit(&plugin, party) {
            Ok(()) => {},
            Err(e) => warn!("Error: pipeline_db_and_party_add_new_synced_player_local -> verify_if_party_size_exceeds_limit [{:?}]", e),
        }

        // Init a new local player and add into the party
        let player_username = String::from(username);
        let new_player = PlayerLocal::new(None, Some(player_username.clone()), PlayerType::PlayerLocal);
        let packaged_player: Arc<Mutex<PlayerLocal>> = Arc::new(Mutex::new(new_player));
        party.players_add_player(packaged_player);

        // Get the new party size
        let party_size = party.get_player_count_party();

        // Update the active player to the new player
        party.active_player_set(party_size as i32);

        info!("Init Record: [{:?}]::[{}]", party.active_player_get_player_type(), party.active_player_get_player_id());

        // Build the new database record referencing the new player record in the party
        match self.action_insert_player_record(&db, party.active_player_get_player_id().to_string(), Some(String::from("PlayerLocal")), Some(player_username), PlayerType::PlayerLocal) {
            Ok(()) => {},
            Err(e) => {
                match e {
                    ErrorType::DatabaseLockPoisoned => warn!("Error: Failed to access Database lock poisoned..."),
                    ErrorType::DBActionFailedPlayerTableInsertRecordPlayerLocal => warn!("Error: Failed to insert new main player into player table..."),
                    _ => {warn!("Error: pipeline_db_and_party_add_new_synced_player_local -> action_insert_player_record [{:?}]", e)},
                }
            }
        }

        Ok(())
    }

    pub fn pipeline_db_and_party_add_player_from_db_to_party(
        &self,
        db: &Res<DatabaseConnection>,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
        existing_uuid: &Uuid,
    ) -> Result<(), ErrorType> {
        info!("Init: pipeline_db_and_party_add_player_to_party_from_db:");
        dotenv().ok();
        let test_ref_uuid_string = match env::var("TEST_REF_PLAYER_UUID") {
            Ok(value) => {
                println!("[ dotenv ] TEST_REF_PLAYER_UUID: {}", value); 
                value
            },
            Err(VarError::NotPresent) => {
                warn!("Error: TEST_REF_PLAYER_UUID access failed, not present...");  
                Uuid::now_v7().to_string()
            },
            Err(VarError::NotUnicode(err)) => {
                warn!("Error: TEST_REF_PLAYER_UUID is not valid Unicode: {:?}", err); 
                Uuid::now_v7().to_string()
            },
        };

        let test_ref_uuid = match Uuid::try_parse(test_ref_uuid_string.as_str()) {
            Ok(uuid) => uuid,
            Err(e) =>{
                warn!("Error: pipeline_db_and_party_add_player_from_db_to_party -> Uuid::try_parse(test_ref_uuid_string.as_str()); {:?}", e); 
                return Err(ErrorType::UuidParsingFailed)},
        };

        if test_ref_uuid == *existing_uuid {
            return Err(ErrorType::AddPlayerFromDbToPartyFailedPlayerTestReference);
        }

        let party_ids = party.all_players_get_ids();
        for player in party_ids {
            if player == *existing_uuid {
                return Err(ErrorType::AddPlayerFromDbToPartyFailedPlayerAlreadyInParty)
            }
        };

        // Party Size Management Checks
        match self.verify_if_party_size_exceeds_limit(&plugin, party) {
            Ok(()) => {},
            Err(e) => warn!("Error: pipeline_db_and_party_add_player_to_party_from_db -> verify_if_party_size_exceeds_limit [{:?}]", e),
        }

        info!("[ verify_if_party_size_exceeds_limit ]: Does not exceed limit");

        // query existing players and search for provided uuid
        let existing_players_vec = match self.query_existing_players(&db) {
            Ok(players_vec) => players_vec,
            Err(e) => {
                warn!("Error: pipeline_db_and_party_add_player_from_db_to_party -> query_existing_players [{:?}]", e);
                let players_vec: Vec<DBPlayer> = Vec::new();
                players_vec
            },
        };

        let mut player_match = false;
        let target_uuid_string_ref = &existing_uuid.to_string();
        for player in existing_players_vec {
            let player_uuid_string_ref = player.get_uuid_string();
            if player_uuid_string_ref == target_uuid_string_ref {
                // Init a new local player and add into the party
                let player_username = player.get_user_name_string();
                let new_player = PlayerLocal::new(None, Some(player_username.clone()), PlayerType::PlayerLocal);
                let packaged_player: Arc<Mutex<PlayerLocal>> = Arc::new(Mutex::new(new_player));
                party.players_add_player(packaged_player);
                player_match = true;
            }
        }

        if !player_match {
            return Err(ErrorType::MatchTargetUuidToExistingPlayerInDatabaseNoPlayerAddedToPartyFailed)
        }

        let target_idx = party.get_player_count_party() as i32;
        party.active_player_set(target_idx);
        party.active_player_set_uuid(existing_uuid.to_owned());

        Ok(())
    }

    pub fn pipeline_db_and_party_init_test_ref_and_main_player(
        &self,
        db: &Res<DatabaseConnection>,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
    ) -> Result<(), ErrorType> {
        info!("Init: pipeline_db_and_party_init_test_ref_and_main_player:");
        dotenv().ok();
        let test_ref_uuid_string = match env::var("TEST_REF_PLAYER_UUID") {
            Ok(value) => {
                println!("[ dotenv ] TEST_REF_PLAYER_UUID: {}", value); 
                value
            },
            Err(VarError::NotPresent) => {
                warn!("Error: TEST_REF_PLAYER_UUID access failed, not present...");  
                Uuid::now_v7().to_string()
            },
            Err(VarError::NotUnicode(err)) => {
                warn!("Error: TEST_REF_PLAYER_UUID is not valid Unicode: {:?}", err); 
                Uuid::now_v7().to_string()
            },
        };
        let test_ref_user_name_string = match env::var("TEST_REF_PLAYER_USERNAME") {
            Ok(value) => {
                println!("[ dotenv ] TEST_REF_PLAYER_USERNAME: {}", value); 
                value
            },
            Err(VarError::NotPresent) => {
                warn!("Error: TEST_REF_PLAYER_USERNAME access failed, not present...");  
                String::from("TEST_REF_PLAYER_USERNAME: Access Failed")
            },
            Err(VarError::NotUnicode(err)) => {
                warn!("Error: TEST_REF_PLAYER_USERNAME is not valid Unicode: {:?}", err); 
                String::from("TEST_REF_PLAYER_USERNAME: Not Valid Unicode")
            },
        };
        let test_ref_email_string = match env::var("TEST_REF_PLAYER_EMAIL") {
            Ok(value) => {
                println!("[ dotenv ] TEST_REF_PLAYER_EMAIL: {}", value); 
                value
            },
            Err(VarError::NotPresent) => {
                warn!("Error: TEST_REF_PLAYER_EMAIL access failed, not present...");  
                String::from("TEST_REF_PLAYER_EMAIL: Access Failed")
            },
            Err(VarError::NotUnicode(err)) => {
                warn!("Error: TEST_REF_PLAYER_EMAIL is not valid Unicode: {:?}", err); 
                String::from("TEST_REF_PLAYER_EMAIL: Not Valid Unicode")
            },
        };
    
        let main_player_uuid = party.main_player_clone_player_id().to_string();
        let main_player_email = plugin.main_player_email.clone();
        let main_player_username = plugin
            .main_player_user_name
            .clone()
            .or_else(|| plugin.main_player_uuid.map(|uuid| uuid.to_string()));
    
        // Check if there are any existing players in the database
        let count = self.query_count_existing_players(&db)
            .map_err(|e| {
                match e {
                    ErrorType::DatabaseLockPoisoned => warn!("Error: Failed to access Database lock poisoned..."),
                    _ => {warn!("Unexpected Error: init_test_ref_and_main_player -> query_count_existing_players")},
                }
                e
            })?;
    
        info!("Existing Player Count: [{}]", count);
        if count == 1 { // Records: [Missing] Delete and start over
            self.action_remove_all_player_records(&db)
            .map_err(|e| {
                match e {
                    ErrorType::DatabaseLockPoisoned => warn!("Error: Failed to access Database lock poisoned..."),
                    ErrorType::DBDeleteFailedPlayerTableDropAllRecords => warn!("Error: Failed to delete players records from player table..."),
                    _ => {warn!("Unexpected Error: init_test_ref_and_main_player -> action_remove_all_player_records")},
                }
                e
            })?;
            info!("Single Player Count Detected: Records Scrubbed for fresh init");
        }
        
        let count = self.query_count_existing_players(&db)
            .map_err(|e| {
                match e {
                    ErrorType::DatabaseLockPoisoned => warn!("Error: Failed to access Database lock poisoned..."),
                    _ => {warn!("Unexpected Error: init_test_ref_and_main_player -> query_count_existing_players")},
                }
                e
            })?;
    
        info!("Existing Player Count: [{}]", count);
        if count == 0 {
            // Build the test reference player in the DB
            info!("Init Record: Testing Reference");
            self.action_insert_player_record(&db, test_ref_uuid_string, Some(test_ref_email_string), Some(test_ref_user_name_string), PlayerType::PlayerTestRef)
                .map_err(|e| {
                    match e {
                        ErrorType::DatabaseLockPoisoned => warn!("Error: Failed to access Database lock poisoned..."),
                        ErrorType::DBActionFailedPlayerTableInsertRecordPlayerTestRef => warn!("Error: Failed to insert new player into player table..."),
                        _ => {warn!("Unexpected Error: init_test_ref_and_main_player -> action_insert_player_record")},
                    }
                    e
                })?;            
            info!("Init Record: Main Player");
            self.action_insert_player_record(&db, main_player_uuid, main_player_email, main_player_username, PlayerType::PlayerMain)
                .map_err(|e| {
                    match e {
                        ErrorType::DatabaseLockPoisoned => warn!("Error: Failed to access Database lock poisoned..."),
                        ErrorType::DBActionFailedPlayerTableInsertRecordPlayerMain => warn!("Error: Failed to insert new main player into player table..."),
                        _ => {warn!("Unexpected Error: init_test_ref_and_main_player -> action_insert_player_record")},
                    }
                    e
                })?;

        } else if count > 1 { // If a player already exists in local database, sync the ecs Uuid to match locally stored profile 
            let players: Vec<DBPlayer> = self.query_existing_players(&db)
                .map_err(|e| {
                    match e {
                        ErrorType::DatabaseLockPoisoned => warn!("Error: Failed to access Database lock poisoned..."),
                        ErrorType::DBQueryFailedExistingPlayers => warn!("Error: Failed to query player table..."),
                        ErrorType::DBQueryMappingFailedExistingPlayers => warn!("Error: Failed to map player table query..."),
                        _ => {warn!("Unexpected Error: init_test_ref_and_main_player -> query_existing_players:")},
                    }
                    e
                })?;
    
            for (idx, player) in players.into_iter().enumerate() {
                if idx == 0 {
                    let player_id = player.uuid;
                    let player_uuid = Uuid::try_parse(player_id.as_str())
                        .map_err(|_| {
                            warn!("Error: Failed to convert from string to Uuid...");
                            ErrorType::UuidParsingFailed
                        })?;
                    party.player_set_player_id(0, player_uuid);
                }
                break;
            }
        }
        
        let count = self.query_count_existing_players(&db)
        .map_err(|e| {
            match e {
                ErrorType::DatabaseLockPoisoned => warn!("Error: Failed to access Database lock poisoned..."),
                _ => {warn!("Unexpected Error: init_test_ref_and_main_player -> query_count_existing_players")},
            }
            e
        })?;
    
        info!("Existing Player Count: [{}]", count);
        Ok(())
    }
    
    pub fn pipeline_db_and_party_sync_main_player_uuids(
        &self,
        db: &Res<DatabaseConnection>,
        party: &mut ResMut<Party>,
    ) -> Result<(), ErrorType> {
        info!("Init: pipeline_db_and_party_sync_main_player_uuids:");

        let party_main_player_uuid = party.main_player_clone_player_id();
        let database_main_player = match self.action_query_main_player(&db) {
            Ok(dbplayer) => dbplayer,
            Err(e) => {
                warn!("[ Error ] pipeline_db_and_party_sync_main_player_uuids -> action_query_main_player: [{:?}]", e);
                return Err(ErrorType::DBQueryFailedPlayerTablePlayerMain);
            },
        };
        let database_main_player_uuid = match Uuid::try_parse(database_main_player.uuid.as_str()) {
            Ok(uuid) => uuid,
            Err(e) => {
                warn!("[ Error ] pipeline_db_and_party_sync_main_player_uuids -> Uuid::try_parse(database_main_player.uuid.as_str()): [{:?}]", e);
                return Err(ErrorType::UuidParsingFailed);
            },
        };

        if party_main_player_uuid != database_main_player_uuid {
            party.active_player_set_uuid(database_main_player_uuid);
        };

        Ok(())
    }
}