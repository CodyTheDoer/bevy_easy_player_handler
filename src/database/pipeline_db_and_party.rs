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
    PlayerAiLocal,
    PlayerAiRemote, 
    PlayerComponent, 
    PlayerHandlerInterface, 
    PlayerLocal, 
    PlayerMain,
    PlayerType,
    PlayerRemote,
};

impl PlayerHandlerInterface {
    pub fn pipeline_db_and_party_add_new_synced_player_ai_local(
        &self,
        commands: &mut Commands,
        db: &Res<DatabaseConnection>,
        party: &mut ResMut<Party>,
        player_query: &Query<&PlayerComponent>,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        username: &str,
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: pipeline_db_and_party_add_new_synced_player_ai_local:");

        // Party Size Management Checks
        self.verify_if_party_size_exceeds_limit(&plugin, party, player_query)?;

        // Init a new local player and add into the party
        let player_username = String::from(username);
        let new_player = PlayerAiLocal::new(
            None, 
            Some(player_username.clone()), 
            None,
            PlayerType::PlayerAiLocal,
        );
        let packaged_player = Arc::new(Mutex::new(new_player));

        // party.players_add_player(packaged_player)?;
        commands.spawn(PlayerComponent{
            player: packaged_player,
        });


        // Get the new party size
        let party_size = party.get_player_count_party(player_query)?;

        // Update the active player to the new player
        party.set_active_player_index(party_size, player_query)?;

        println!("Init Record: [{:?}]::[{}]", party.clone_active_player_player_type(player_query)?, party.clone_active_player_uuid(player_query)?);

        // // Build the new database record referencing the new player record in the party
        // self.action_insert_player_record(&db, &party.clone_active_player_uuid(player_query)?, Some(&String::from("PlayerAiLocal")), Some(&player_username), PlayerType::PlayerAiLocal)?;

        Ok(())
    }

    pub fn pipeline_db_and_party_add_new_synced_player_local(
        &self,
        commands: &mut Commands,
        db: &Res<DatabaseConnection>,
        party: &mut ResMut<Party>,
        player_query: &Query<&PlayerComponent>,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        username: &str,
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: pipeline_db_and_party_add_new_synced_player_local:");

        println!("Step 1 [ pipeline_db_and_party_add_new_synced_player_local ]");
        // Party Size Management Checks
        self.verify_if_party_size_exceeds_limit(plugin, party, player_query)?;

        println!("Step 2 [ pipeline_db_and_party_add_new_synced_player_local ]");
        // Init a new local player and add into the party
        let player_username = String::from(username);
        println!("Step 3 [ pipeline_db_and_party_add_new_synced_player_local ]");
        let synced_uuid = Uuid::now_v7();
        println!("Step 4 [ pipeline_db_and_party_add_new_synced_player_local ]");
        let new_player = PlayerLocal::new(
            None, 
            Some(player_username.clone()), 
            Some(synced_uuid),
            PlayerType::PlayerLocal,
        );
        println!("Step 5 [ pipeline_db_and_party_add_new_synced_player_local ]");
        let packaged_player: Arc<Mutex<PlayerLocal>> = Arc::new(Mutex::new(new_player));
        
        println!("Step 6 [ pipeline_db_and_party_add_new_synced_player_local ]");
        commands.spawn(PlayerComponent{
            player: packaged_player,
        });

        println!("Step 7 [ pipeline_db_and_party_add_new_synced_player_local ]");
        // Get the new party size
        let party_size = party.get_player_count_party(player_query)?;

        println!("Step 8 [ pipeline_db_and_party_add_new_synced_player_local ]");
        party.set_active_player_index(party_size, player_query)?;

        println!("Step 10 [ pipeline_db_and_party_add_new_synced_player_local ]");
        let player_type = party.clone_active_player_player_type(player_query)?;
        println!("Init Record: [{:?}]::[{}]", player_type, synced_uuid);

        // println!("Step 11 [ pipeline_db_and_party_add_new_synced_player_local ]");
        // // Build the new database record referencing the new player record in the party
        // self.action_insert_player_record(&db, &synced_uuid, Some(&String::from("PlayerLocal")), Some(&player_username), PlayerType::PlayerLocal)?;

        println!("Success [ pipeline_db_and_party_add_new_synced_player_local ]");
        Ok(())
    }

    pub fn pipeline_db_and_party_action_remove_player(
        &self,
        commands: &mut Commands,
        db: &Res<DatabaseConnection>,
        entity_player_query: &Query<(Entity, &PlayerComponent)>,
        party: &mut ResMut<Party>,
        player_query: &Query<&PlayerComponent>,
        player_uuid: &Uuid,
        plugin: &mut ResMut<BevyEasyPlayerHandlerPlugin>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: pipeline_db_and_party_action_remove_player:");
        println!("active_player_get_index: Before: pipeline_db_and_party_action_remove_player [{}]", party.get_active_player_index()?);

        // grab the test ref player ref uuid 
        let test_ref: (Uuid, String, String) = self.test_ref_info()?;

        if test_ref.0 == *player_uuid {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("[ pipeline_db_and_party_action_remove_player: {} ] Failed: target is the test reference player, and can be not removed", &player_uuid)))
        }  
        // grab the main player ref uuid 
        let player_uuids = party.get_all_players_ids(player_query)?;

        if player_uuids[0] == *player_uuid {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("[ pipeline_db_and_party_action_remove_player: {} ] Failed: target is the main player, and can be not removed", &player_uuid)))
        }      
        
        let mut remove_player = false;
        for uuid in player_uuids.iter() {
            if player_uuid == uuid {
                remove_player = true;
            }
        }
        println!("pipeline_db_and_party_action_remove_player: remove_player = [{}]", &remove_player);

        if remove_player {
            party.remove_player(commands, entity_player_query, plugin, player_uuid)?;
        }
        
        // Get and Lock the mutex to access the database connection
        let conn = db.get_connection();
        let conn = conn.lock();
        let conn = match conn {
            Ok(conn) => conn,
            Err(_) => {
                error!("Database connection lock poisoned.");
                return Err(ErrorTypePlayerHandler::DatabaseLockPoisoned);
            }
        };

        let player_uuid = player_uuid.to_owned();
        let player_uuid_string = String::from(player_uuid);
        let player_uuid_str = player_uuid_string.as_str();
        // setup and execute deletion of single player record from DB
        conn.execute(
            "DELETE FROM player_table WHERE uuid LIKE ?",
            [format!("%{}%", player_uuid_str)],
        )
        .map_err(|e| ErrorTypePlayerHandler::DBActionFailed(format!("action_remove_player_record failed Error: [{}]", e)))?;

        println!("active_player_get_index: After: pipeline_db_and_party_action_remove_player [{}]", party.get_active_player_index()?);
        Ok(())
    }

    pub fn pipeline_db_and_party_add_player_from_db_to_party(
        &self,
        commands: &mut Commands,
        db: &Res<DatabaseConnection>,
        existing_uuid: &Uuid,
        party: &mut ResMut<Party>,
        player_query: &Query<&PlayerComponent>,
        plugin: &mut ResMut<BevyEasyPlayerHandlerPlugin>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: pipeline_db_and_party_add_player_to_party_from_db");
        dotenv().ok();

        println!("Step 1 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        // grab the test ref uuid from .env
        let test_ref_uuid_string = match env::var("TEST_REF_PLAYER_UUID") {
            Ok(value) => value,
            Err(VarError::NotPresent) => {
                println!("Error 1 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                return Err(ErrorTypePlayerHandler::VarErrorNotPresent)
            },
            Err(VarError::NotUnicode(err)) => {
                println!("Error 2 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                let err_string = err.into_string().unwrap();
                return Err(ErrorTypePlayerHandler::VarErrorNotUnicode(err_string))
            },
        };

        println!("Step 2 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        let test_ref_uuid = match Uuid::try_parse(test_ref_uuid_string.as_str()) {
            Ok(uuid) => uuid,
            Err(e) =>{
                println!("Error 3 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                return Err(ErrorTypePlayerHandler::UuidParsingFailed(e.to_string()))},
        };

        println!("Step 3 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        if test_ref_uuid == *existing_uuid {
            println!("Error 4: [ pipeline_db_and_party_add_player_to_party_from_db ]");
            return Err(ErrorTypePlayerHandler::AddPlayerFromDbToPartyFailed(format!("Player: [{}] is the test reference, not a valid player", &existing_uuid)))
        }

        println!("Step 4 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        let party_ids = party.get_all_players_ids(player_query)?;
        for player in party_ids {
            if player == *existing_uuid {
                println!("Error 5: [ pipeline_db_and_party_add_player_to_party_from_db ]");
                return Err(ErrorTypePlayerHandler::AddPlayerFromDbToPartyFailed(format!("Player: [{}] is already in the party", &existing_uuid)))
            }
        };

        println!("Step 5 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        // Party Size Management Checks
        self.verify_if_party_size_exceeds_limit(plugin, party, player_query)?;

        println!("[ verify_if_party_size_exceeds_limit ]: Success...");

        println!("Step 6 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        // query existing players and search for provided uuid
        let existing_players_vec = self.query_db_existing_players(&db)?;

        println!("Step 7 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        let mut player_match = false;
        println!("Step 8 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        let target_uuid_string_ref = &existing_uuid.to_string();
        println!("target_uuid_string_ref: [ {} ]", target_uuid_string_ref);
        println!("Step 9 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        for player in existing_players_vec {
            println!("Step 10 [ pipeline_db_and_party_add_player_to_party_from_db ]");
            let player_uuid_string_ref = player.get_uuid_string();
            println!("player_uuid_string_ref: [ {} ]", player_uuid_string_ref);
            println!("Step 11 [ pipeline_db_and_party_add_player_to_party_from_db ]");
            if player_uuid_string_ref == target_uuid_string_ref {
                // Init a new local player and add into the party
                println!("Step 12 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                let player_username = player.get_username_string();
                println!("player_username: [ {} ]", player_username);
                println!("Step 13 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                let packaged_player: Arc<Mutex<dyn Player + Send>>  = match player_username.as_str() {
                    "PlayerAiLocal" => {
                        println!("Step 13: Match [ PlayerAi: Local ] [ pipeline_db_and_party_add_player_to_party_from_db ]");
                        let new_player = PlayerAiLocal::new(
                            None, 
                            Some(player_username.clone()), 
                            Some(*existing_uuid),
                            PlayerType::PlayerAiLocal,
                        );
                        let packaged_player: Arc<Mutex<dyn Player + Send>> = Arc::new(Mutex::new(new_player));
                        packaged_player
                    },
                    "PlayerLocal" => {
                        println!("Step 13: Match [ PlayerLocal ] [ pipeline_db_and_party_add_player_to_party_from_db ]");
                        let new_player = PlayerLocal::new(
                            None, 
                            Some(player_username.clone()), 
                            Some(*existing_uuid),
                            PlayerType::PlayerLocal,
                        );
                        let packaged_player: Arc<Mutex<dyn Player + Send>> = Arc::new(Mutex::new(new_player));
                        packaged_player
                    },
                    &_ => {
                        println!("Step 13: Match [ PlayerLocal ] [ pipeline_db_and_party_add_player_to_party_from_db ]");
                        let new_player = PlayerRemote::new(
                            None, 
                            Some(player_username.clone()), 
                            Some(*existing_uuid),
                            PlayerType::PlayerRemote,
                        );
                        let packaged_player: Arc<Mutex<dyn Player + Send>> = Arc::new(Mutex::new(new_player));
                        packaged_player
                    },
                };
                println!("Step 14 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                commands.spawn(PlayerComponent{
                    player: packaged_player,
                });
                println!("Step 15 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                player_match = true;
            }
        }

        println!("Step 16 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        if !player_match {
            println!("Error 6 [ pipeline_db_and_party_add_player_to_party_from_db ]");
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("pipeline_db_and_party_add_player_from_db_to_party: Failed")))
        }

        println!("Success [ pipeline_db_and_party_add_player_to_party_from_db ]");
        Ok(())
    }

    pub fn pipeline_db_and_party_add_main_player_from_db_to_party(
        &self,
        commands: &mut Commands,
        db: &Res<DatabaseConnection>,
        existing_uuid: &Uuid,
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: pipeline_db_and_party_add_player_to_party_from_db");
        dotenv().ok();

        println!("Step 1 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        // grab the test ref uuid from .env
        let test_ref_uuid_string = match env::var("TEST_REF_PLAYER_UUID") {
            Ok(value) => value,
            Err(VarError::NotPresent) => {
                println!("Error 1 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                return Err(ErrorTypePlayerHandler::VarErrorNotPresent)
            },
            Err(VarError::NotUnicode(err)) => {
                println!("Error 2 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                let err_string = err.into_string().unwrap();
                return Err(ErrorTypePlayerHandler::VarErrorNotUnicode(err_string))
            },
        };

        println!("Step 2 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        let test_ref_uuid = match Uuid::try_parse(test_ref_uuid_string.as_str()) {
            Ok(uuid) => uuid,
            Err(e) =>{
                println!("Error 3 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                return Err(ErrorTypePlayerHandler::UuidParsingFailed(e.to_string()))},
        };

        println!("Step 3 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        if test_ref_uuid == *existing_uuid {
            println!("Error 4: [ pipeline_db_and_party_add_player_to_party_from_db ]");
            return Err(ErrorTypePlayerHandler::AddPlayerFromDbToPartyFailed(format!("Player: [{}] is the test reference, not a valid player", &existing_uuid)))
        }

        println!("Step 6 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        // query existing players and search for provided uuid
        let existing_players_vec = self.query_db_existing_players(&db)?;

        println!("Step 7 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        let mut player_match = false;
        println!("Step 8 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        let target_uuid_string_ref = &existing_uuid.to_string();
        println!("target_uuid_string_ref: [ {} ]", target_uuid_string_ref);
        println!("Step 9 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        for player in existing_players_vec {
            println!("Step 10 [ pipeline_db_and_party_add_player_to_party_from_db ]");
            let player_uuid_string_ref = player.get_uuid_string();
            println!("player_uuid_string_ref: [ {} ]", player_uuid_string_ref);
            println!("Step 11 [ pipeline_db_and_party_add_player_to_party_from_db ]");
            if player_uuid_string_ref == target_uuid_string_ref {
                // Init a new local player and add into the party
                println!("Step 12 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                let player_username = player.get_username_string();
                println!("player_username: [ {} ]", player_username);
                println!("Step 13 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                let packaged_player: Arc<Mutex<dyn Player + Send>> = Arc::new(Mutex::new(PlayerMain::new(
                    None, 
                    Some(player_username.clone()), 
                    Some(*existing_uuid),
                    PlayerType::PlayerMain,
                )));
                println!("Step 14 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                commands.spawn(PlayerComponent{
                    player: packaged_player,
                });
                println!("Step 15 [ pipeline_db_and_party_add_player_to_party_from_db ]");
                player_match = true;
            }
        }

        println!("Step 16 [ pipeline_db_and_party_add_player_to_party_from_db ]");
        if !player_match {
            println!("Error 6 [ pipeline_db_and_party_add_player_to_party_from_db ]");
            return Err(ErrorTypePlayerHandler::PartyActionFailed(format!("pipeline_db_and_party_add_player_from_db_to_party: Failed")))
        }

        println!("Success [ pipeline_db_and_party_add_player_to_party_from_db ]");
        Ok(())
    }

    pub fn pipeline_db_and_party_remove_all_build_test_ref_and_init_new_main_player(
        &self,
        db: &Res<DatabaseConnection>,
        mut commands: &mut Commands,
        entity_player_query: &Query<(Entity, &PlayerComponent)>, 
        party: &mut ResMut<Party>,
        player_query: &Query<&PlayerComponent>,
        plugin: &mut ResMut<BevyEasyPlayerHandlerPlugin>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: pipeline_db_and_party_remove_all_build_test_ref_and_init_main_player:");
    
        let count = self.query_db_count_existing_players(&db)?;    
        println!("Existing Player Count: [{}]", count);

        let test_ref_info = self.test_ref_info()?;

        self.action_remove_all_player_records(&db)?;
        party.player_map_and_component_remove_all_players(&mut commands, entity_player_query, player_query, plugin)?;
    
        // Build the test reference player in the DB
        println!("Init DB Record: Testing Reference");
        self.action_insert_player_record(&db, &test_ref_info.0, Some(&test_ref_info.1), Some(&test_ref_info.2), PlayerType::PlayerTestRef)?;    
    
        let main_player_uuid = Uuid::now_v7();

        // Build the main player
        println!("Update Plugin Target: Main Player");
        plugin.set_main_player_uuid(&main_player_uuid)?;
        let main_player_email = plugin.get_main_player_email()?;
        let main_player_email = main_player_email.expect("main_player_email unwrap failed ");
        let main_player_username = plugin.get_main_player_username()?;
        let main_player_username = main_player_username.expect("main_player_username unwrap failed ");

        println!("Init DB Record: Main Player");
        self.action_insert_player_record(&db, &main_player_uuid, Some(main_player_email), Some(main_player_username), PlayerType::PlayerMain)?;

        println!("Build DB Record: Main Player");
        self.pipeline_db_and_party_add_main_player_from_db_to_party(&mut commands, &db, &main_player_uuid)?;
        // self.pipeline_db_and_party_add_player_from_db_to_party(&mut commands, &db, &main_player_uuid, party, player_query, plugin)?;
        Ok(())
    }

    pub fn pipeline_db_and_party_startup_test_ref_and_init_main_player(
        &self,
        db: &Res<DatabaseConnection>,
        mut commands: &mut Commands,
        entity_player_query: &Query<(Entity, &PlayerComponent)>, 
        party: &mut ResMut<Party>,
        player_query: &Query<&PlayerComponent>,
        plugin: &mut ResMut<BevyEasyPlayerHandlerPlugin>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: pipeline_db_and_party_remove_all_build_test_ref_and_init_main_player:");
    
        let count = self.query_db_count_existing_players(&db)?;    
        println!("Existing Player Count: [{}]", count);

        if count <= 2 { // No non-recoverable/rebuildable records detected
            let test_ref_info = self.test_ref_info()?;

            self.action_remove_all_player_records(&db)?;
            party.player_map_and_component_remove_all_players(&mut commands, entity_player_query, player_query, plugin)?;
        
            // Build the test reference player in the DB
            println!("Init DB Record: Testing Reference");
            self.action_insert_player_record(&db, &test_ref_info.0, Some(&test_ref_info.1), Some(&test_ref_info.2), PlayerType::PlayerTestRef)?;
        
            let main_player_uuid = Uuid::now_v7();
            let main_player_email = plugin.get_main_player_email()?;
            let main_player_username = plugin.get_main_player_username()?;
            
            // Build the main player
            println!("Init DB Record: Main Player");
            self.action_insert_player_record(&db, &main_player_uuid, main_player_email, main_player_username, PlayerType::PlayerMain)?;

            self.pipeline_db_and_party_add_main_player_from_db_to_party(&mut commands, &db, &main_player_uuid)?;
        } 
        else if count > 2 { // If a player already exists in local database, sync the ecs Uuid to match locally stored profile 
            let players = self.query_db_existing_players(&db)?;
            for (idx, player) in players.into_iter().enumerate() {
                if idx == 0 {
                    let player_id = player.uuid;
                    let player_uuid = Uuid::try_parse(player_id.as_str())
                        .map_err(|e| {
                            println!("Error 2 [ pipeline_db_and_party_init_test_ref_and_main_player ]");
                            warn!("Error: Failed to convert from string to Uuid...");
                            ErrorTypePlayerHandler::UuidParsingFailed(e.to_string())
                        })?;
                    // party.player_set_player_id(0, player_uuid)?;
                    // party.set_active_player_uuid_player_map_and_component(player_query, player_uuid)?;
                    party.init_main_player_uuid_player_map(player_query, player_uuid)?;
                }
                break;
            }
        }
        Ok(())
    }
    
    pub fn pipeline_db_and_party_sync_main_player_uuids(
        &self,
        db: &Res<DatabaseConnection>,
        party: &mut ResMut<Party>,
        player_query: &Query<&PlayerComponent>,
        mut plugin: ResMut<BevyEasyPlayerHandlerPlugin>
    ) -> Result<(), ErrorTypePlayerHandler> {
        println!("Init: pipeline_db_and_party_sync_main_player_uuids:");
        println!("Step: 1 [ pipeline_db_and_party_sync_main_player_uuids ]");
        // let party_main_player_uuid = party.clone_main_player_uuid(player_query)?;
        let party_main_player_uuid = plugin.get_main_player_uuid()?;
        println!("Step: 2 [ pipeline_db_and_party_sync_main_player_uuids ]");
        if party_main_player_uuid.is_none() {
            return Err(ErrorTypePlayerHandler::PluginDataRetreivalFailed(format!("plugin.get_main_player_uuid()?; is None")))
        }
        println!("Step: 3 [ pipeline_db_and_party_sync_main_player_uuids ]");
        let party_main_player_uuid = party_main_player_uuid.unwrap();
        println!("Step: 4 [ pipeline_db_and_party_sync_main_player_uuids ]");
        let database_main_player = self.query_db_main_player(&db)?;
        println!("Step: 5 [ pipeline_db_and_party_sync_main_player_uuids ]");
        let database_main_player_uuid = match Uuid::try_parse(database_main_player.uuid.as_str()) {
            Ok(uuid) => uuid,
            Err(e) => {
                warn!("[ Error ] pipeline_db_and_party_sync_main_player_uuids -> Uuid::try_parse(database_main_player.uuid.as_str()): [{:?}]", e);
                return Err(ErrorTypePlayerHandler::UuidParsingFailed(e.to_string()));
            },
        };
        println!("Step: 6 [ pipeline_db_and_party_sync_main_player_uuids ]");
        if party_main_player_uuid != &database_main_player_uuid {
            println!("Step: 7 [ pipeline_db_and_party_sync_main_player_uuids ]");
            plugin.set_main_player_uuid(&database_main_player_uuid)?;
            // party.init_main_player_uuid_player_map(player_query, database_main_player_uuid)?;
            println!("Step: 8 [ pipeline_db_and_party_sync_main_player_uuids ]");
            party.set_active_player_uuid_player_map_and_component(player_query, database_main_player_uuid)?;
        };
        Ok(())
    }

    pub fn test_ref_info(&self) -> Result<(Uuid, String, String), ErrorTypePlayerHandler>  {
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

        let test_ref_username_string = match env::var("TEST_REF_PLAYER_USERNAME") {
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

        let test_ref_uuid = match Uuid::try_parse(test_ref_uuid_string.as_str()){
            Ok(value) => value,
            Err(_) => return Err(ErrorTypePlayerHandler::UuidParsingFailed(format!("match Uuid::try_parse(test_ref_uuid_string.as_str())"))),
        };

        let test_ref_info = (
            test_ref_uuid,
            test_ref_username_string,
            test_ref_email_string,
        );

        Ok(test_ref_info)
    }
}