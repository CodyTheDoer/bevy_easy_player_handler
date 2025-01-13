use bevy::prelude::*;

use bevy_easy_shared_definitions::{
    DatabaseConnection, 
    ErrorTypePlayerHandler,
};

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
    Party, 
    Player, 
    PlayerAi,
    PlayerHandlerInterface, 
    PlayerLocal, PlayerType,
};

impl PlayerHandlerInterface {
    pub fn pipeline_db_and_party_add_new_synced_player_ai_local(
        &self,
        db: &Res<DatabaseConnection>,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
        username: &str,
    ) -> Result<(), ErrorTypePlayerHandler> {
        info!("Init: pipeline_db_and_party_add_new_synced_player_ai_local:");

        // Party Size Management Checks
        self.verify_if_party_size_exceeds_limit(&plugin, party)?;

        // Init a new local player and add into the party
        let player_username = String::from(username);
        let new_player = PlayerAi::new(None, Some(player_username.clone()), PlayerType::PlayerAiLocal);
        let packaged_player = Arc::new(Mutex::new(new_player));
        party.players_add_player(packaged_player)?;

        // Get the new party size
        let party_size = party.get_player_count_party()?;

        // Update the active player to the new player
        party.active_player_set(party_size as i32)?;

        info!("Init Record: [{:?}]::[{}]", party.active_player_get_player_type(), party.active_player_get_player_id()?);

        // Build the new database record referencing the new player record in the party
        self.action_insert_player_record(&db, &party.active_player_get_player_id()?.to_string(), Some(&String::from("PlayerAiLocal")), Some(&player_username), PlayerType::PlayerAiLocal)?;

        Ok(())
    }

    pub fn pipeline_db_and_party_add_new_synced_player_local(
        &self,
        db: &Res<DatabaseConnection>,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
        username: &str,
    ) -> Result<(), ErrorTypePlayerHandler> {
        info!("Init: pipeline_db_and_party_add_new_synced_player_local:");

        // Party Size Management Checks
        self.verify_if_party_size_exceeds_limit(&plugin, party)?;

        // Init a new local player and add into the party
        let player_username = String::from(username);
        let new_player = PlayerLocal::new(None, Some(player_username.clone()), PlayerType::PlayerLocal);
        let packaged_player: Arc<Mutex<PlayerLocal>> = Arc::new(Mutex::new(new_player));
        party.players_add_player(packaged_player)?;

        // Get the new party size
        let party_size = party.get_player_count_party()?;

        // Update the active player to the new player
        party.active_player_set(party_size as i32)?;

        info!("Init Record: [{:?}]::[{}]", party.active_player_get_player_type(), party.active_player_get_player_id()?);

        // Build the new database record referencing the new player record in the party
        self.action_insert_player_record(&db, &party.active_player_get_player_id()?.to_string(), Some(&String::from("PlayerLocal")), Some(&player_username), PlayerType::PlayerLocal)?;

        Ok(())
    }

    pub fn pipeline_db_and_party_add_player_from_db_to_party(
        &self,
        db: &Res<DatabaseConnection>,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
        existing_uuid: &Uuid,
    ) -> Result<(), ErrorTypePlayerHandler> {
        info!("Init: pipeline_db_and_party_add_player_to_party_from_db:");
        dotenv().ok();

        // grab the test ref uuid from .env
        let test_ref_uuid_string = match env::var("TEST_REF_PLAYER_UUID") {
            Ok(value) => {
                println!("[ dotenv ] TEST_REF_PLAYER_UUID: {}", value); 
                value
            },
            Err(VarError::NotPresent) => {
                return Err(ErrorTypePlayerHandler::VarErrorNotPresent)
            },
            Err(VarError::NotUnicode(err)) => {
                let err_string = err.into_string().unwrap();
                return Err(ErrorTypePlayerHandler::VarErrorNotUnicode(err_string))
            },
        };

        let test_ref_uuid = match Uuid::try_parse(test_ref_uuid_string.as_str()) {
            Ok(uuid) => uuid,
            Err(e) =>{
                return Err(ErrorTypePlayerHandler::UuidParsingFailed(e.to_string()))},
        };

        if test_ref_uuid == *existing_uuid {
            return Err(ErrorTypePlayerHandler::AddPlayerFromDbToPartyFailed(format!("Player: [{}] is the test reference, not a valid player", &existing_uuid)))
        }

        let party_ids = party.all_players_get_ids()?;
        for player in party_ids {
            if player == *existing_uuid {
                return Err(ErrorTypePlayerHandler::AddPlayerFromDbToPartyFailed(format!("Player: [{}] is already in the party", &existing_uuid)))
            }
        };

        // Party Size Management Checks
        self.verify_if_party_size_exceeds_limit(&plugin, party)?;

        info!("[ verify_if_party_size_exceeds_limit ]: Does not exceed limit");

        // query existing players and search for provided uuid
        let existing_players_vec = self.query_existing_players(&db)?;

        let mut player_match = false;
        let target_uuid_string_ref = &existing_uuid.to_string();
        for player in existing_players_vec {
            let player_uuid_string_ref = player.get_uuid_string();
            if player_uuid_string_ref == target_uuid_string_ref {
                // Init a new local player and add into the party
                let player_username = player.get_user_name_string();
                let new_player = PlayerLocal::new(None, Some(player_username.clone()), PlayerType::PlayerLocal);
                let packaged_player: Arc<Mutex<PlayerLocal>> = Arc::new(Mutex::new(new_player));
                party.players_add_player(packaged_player)?;
                player_match = true;
            }
        }

        if !player_match {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("pipeline_db_and_party_add_player_from_db_to_party: Failed")))
        }

        let target_idx = party.get_player_count_party()? as i32;
        party.active_player_set(target_idx)?;
        party.active_player_set_uuid(existing_uuid.to_owned())?;

        Ok(())
    }

    pub fn pipeline_db_and_party_init_test_ref_and_main_player(
        &self,
        db: &Res<DatabaseConnection>,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        info!("Init: pipeline_db_and_party_init_test_ref_and_main_player:");

        let test_ref_info = self.test_ref_info()?;
    
        let main_player_uuid = party.main_player_clone_player_id()?.to_string();
        let main_player_email = plugin.get_main_player_email()?;
        let main_player_username = plugin.get_main_player_user_name()?;
    
        // Check if there are any existing players in the database
        let count = self.query_count_existing_players(&db)?;
    
        info!("Existing Player Count: [{}]", count);
        if count == 1 { // Records: [Missing] Delete and start over
            self.action_remove_all_player_records(&db)?;
            info!("Single Player Count Detected: Records Scrubbed for fresh init");
        }
        
        let count = self.query_count_existing_players(&db)?;
    
        info!("Existing Player Count: [{}]", count);
        if count == 0 {
            // Build the test reference player in the DB
            info!("Init Record: Testing Reference");
            self.action_insert_player_record(&db, &test_ref_info[0], Some(&test_ref_info[1]), Some(&test_ref_info[2]), PlayerType::PlayerTestRef)?;    
           
            info!("Init Record: Main Player");
            self.action_insert_player_record(&db, &main_player_uuid, main_player_email, main_player_username, PlayerType::PlayerMain)?;

        } else if count > 1 { // If a player already exists in local database, sync the ecs Uuid to match locally stored profile 
            let players = self.query_existing_players(&db)?;
    
            for (idx, player) in players.into_iter().enumerate() {
                if idx == 0 {
                    let player_id = player.uuid;
                    let player_uuid = Uuid::try_parse(player_id.as_str())
                        .map_err(|e| {
                            warn!("Error: Failed to convert from string to Uuid...");
                            ErrorTypePlayerHandler::UuidParsingFailed(e.to_string())
                        })?;
                    party.player_set_player_id(0, player_uuid)?;
                }
                break;
            }
        }
        
        let count = self.query_count_existing_players(&db)?;
    
        info!("Existing Player Count: [{}]", count);
        Ok(())
    }
    
    pub fn pipeline_db_and_party_sync_main_player_uuids(
        &self,
        db: &Res<DatabaseConnection>,
        party: &mut ResMut<Party>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        info!("Init: pipeline_db_and_party_sync_main_player_uuids:");

        let party_main_player_uuid = party.main_player_clone_player_id()?;
        let database_main_player = self.query_main_player(&db)?;
        let database_main_player_uuid = match Uuid::try_parse(database_main_player.uuid.as_str()) {
            Ok(uuid) => uuid,
            Err(e) => {
                warn!("[ Error ] pipeline_db_and_party_sync_main_player_uuids -> Uuid::try_parse(database_main_player.uuid.as_str()): [{:?}]", e);
                return Err(ErrorTypePlayerHandler::UuidParsingFailed(e.to_string()));
            },
        };

        if party_main_player_uuid != database_main_player_uuid {
            party.active_player_set_uuid(database_main_player_uuid)?;
        };

        Ok(())
    }

    pub fn test_ref_info(&self) -> Result<Vec<String>, ErrorTypePlayerHandler>  {
        dotenv().ok();
        // grab the test values from .env
        let test_ref_uuid_string = match env::var("TEST_REF_PLAYER_UUID") {
            Ok(value) => {
                println!("[ dotenv ] TEST_REF_PLAYER_UUID: {}", value); 
                value
            },
            Err(VarError::NotPresent) => {
                return Err(ErrorTypePlayerHandler::VarErrorNotPresent)
            },
            Err(VarError::NotUnicode(err)) => {
                let err_string = err.into_string().unwrap();
                return Err(ErrorTypePlayerHandler::VarErrorNotUnicode(err_string))
            },
        };

        let test_ref_user_name_string = match env::var("TEST_REF_PLAYER_USERNAME") {
            Ok(value) => {
                println!("[ dotenv ] TEST_REF_PLAYER_USERNAME: {}", value); 
                value
            },
            Err(VarError::NotPresent) => {
                return Err(ErrorTypePlayerHandler::VarErrorNotPresent)
            },
            Err(VarError::NotUnicode(err)) => {
                let err_string = err.into_string().unwrap();
                return Err(ErrorTypePlayerHandler::VarErrorNotUnicode(err_string))
            },
        };
        
        let test_ref_email_string = match env::var("TEST_REF_PLAYER_EMAIL") {
            Ok(value) => {
                println!("[ dotenv ] TEST_REF_PLAYER_EMAIL: {}", value); 
                value
            },
            Err(VarError::NotPresent) => {
                return Err(ErrorTypePlayerHandler::VarErrorNotPresent)
            },
            Err(VarError::NotUnicode(err)) => {
                let err_string = err.into_string().unwrap();
                return Err(ErrorTypePlayerHandler::VarErrorNotUnicode(err_string))
            },
        };

        let info_vec = vec![
            test_ref_uuid_string,
            test_ref_user_name_string,
            test_ref_email_string,
        ];

        Ok(info_vec)
    }
}