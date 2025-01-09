use bevy::prelude::*;

use bevy_easy_shared_definitions::DatabaseConnection;

use dotenv::dotenv;
use std::{
    env,
    env::VarError,
};
use rusqlite::Result;
use uuid::Uuid;

use crate::{
    BevyEasyPlayerHandlerPlugin,
    PlayerHandlerDatabaseCommands,
    DBPlayer,
    ErrorType,
    Party,
};

impl PlayerHandlerDatabaseCommands {
    pub fn get() -> Self {
        PlayerHandlerDatabaseCommands {}
    }

    pub fn pipeline_init_test_ref_and_main_player(
        &self,
        db: &Res<DatabaseConnection>,
        plugin: Res<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
    ) -> Result<(), ErrorType> {
        info!("Init: pipeline_init_test_ref_and_main_player:");
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
            self.action_insert_player_record(&db, test_ref_uuid_string, Some(test_ref_email_string), Some(test_ref_user_name_string), true)
                .map_err(|e| {
                    match e {
                        ErrorType::DatabaseLockPoisoned => warn!("Error: Failed to access Database lock poisoned..."),
                        ErrorType::DBActionFailedPlayerTableInsertRecordTestRef => warn!("Error: Failed to insert new player into player table..."),
                        _ => {warn!("Unexpected Error: init_test_ref_and_main_player -> action_insert_player_record")},
                    }
                    e
                })?;
            
            info!("Init Record: Main Player");
            self.action_insert_player_record(&db, main_player_uuid, main_player_email, main_player_username, false)
                .map_err(|e| {
                    match e {
                        ErrorType::DatabaseLockPoisoned => warn!("Error: Failed to access Database lock poisoned..."),
                        ErrorType::DBActionFailedPlayerTableInsertRecordPlayer => warn!("Error: Failed to insert new main player into player table..."),
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
}

impl DBPlayer {
    pub fn get_uuid_string(&self) -> &String {
        &self.uuid
    }
    pub fn get_email_string(&self) -> &String {
        &self.email
    }
    pub fn get_user_name_string(&self) -> &String {
        &self.user_name
    }
}