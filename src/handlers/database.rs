use bevy::prelude::*;

use rusqlite::Result;

use crate::{
    BevyEasyPlayerHandlerPlugin, 
    DatabaseConnection,
    DBPlayer, 
    ErrorType, 
    Party, 
    PlayerHandlerDatabaseCommands, 
};

impl PlayerHandlerDatabaseCommands {
    pub fn get() -> Self {
        PlayerHandlerDatabaseCommands {}
    }

    pub fn start_up_protocol(
        db: Res<DatabaseConnection>,
        dbi: Res<PlayerHandlerDatabaseCommands>,
        plugin: ResMut<BevyEasyPlayerHandlerPlugin>,
        mut party: ResMut<Party>,
    ) {
        info!("BevyEasyPlayerHandler: [ Start Up Protocol ]");
    
        // ----- [ Vertify database table "player_table" exists ] ----- //
    
        let player_table_exists = match dbi.query_table_player_exists(&db) {
            Ok(exists) => {
                exists
            },
            Err(e) => {
                warn!("Error: start_up_protocol -> query_table_player_exists: {:?}", e);
                false // Assume the table does not exist on unexpected errors
            }
        };
    
        info!("Result [query_table_player_exists]: [{}]", player_table_exists);
    
        if !player_table_exists {
            match dbi.action_table_player_init(&db) {
                Ok(_) => info!("Table: 'player_table' created successfully!"),
                Err(ErrorType::DBActionFailedPlayerTableCreation) => warn!("Error: Failed to create 'player_table'..."),
                Err(e) => warn!("Error: start_up_protocol -> action_table_player_init: {:?}", e)
            };
        }
    
        // ----- [ Vertify database test ref and main player exists ] ----- //
    
        let players_test_ref_and_owner_exists = match dbi.query_test_ref_and_main_player_exists(&db) {
            Ok(does_exist) => {
                does_exist
            },
            Err(e) => {
                warn!("Error: start_up_protocol -> query_test_ref_and_main_player_exists: {:?}", e);
                false // Assume the test ref and main players do not exist on unexpected errors
            }
        };
    
        info!("Result [query_test_ref_and_main_player_exists]: [{}]", players_test_ref_and_owner_exists);
    
        if !players_test_ref_and_owner_exists {
            match dbi.pipeline_db_and_party_init_test_ref_and_main_player(&db, &plugin, &mut party) {
                Ok(_) => info!("Database: Records [ test_ref ] & [ main_player ] created successfully!"),
                Err(ErrorType::DBActionFailedPlayerCreation) => warn!("Error: Failed to create 'player'..."),
                Err(e) => warn!("Error: start_up_protocol -> init_test_ref_and_main_player: {:?}", e)
            }
        }
    
        // ----- [ Sync party and database main players uuid ] ----- //
    
        let party_and_database_main_player_synced = match dbi.query_party_and_db_main_player_synced(&db, &mut party) {
            Ok(synced) => {
                synced
            },
            Err(e) => {
                warn!("Error: start_up_protocol -> query_party_and_db_main_player_synced: {:?}", e);
                false // Assume the table does exist on unexpected errors
            }
        };
        
        info!("Result [query_party_and_db_main_player_synced]: [{}]", party_and_database_main_player_synced);
    
        if !party_and_database_main_player_synced {
            match dbi.pipeline_db_and_party_sync_main_player_uuids(&db, &mut party) {
                Ok(_) => info!("Party: Database and Party [ main_player ] synced successfully!"),
                Err(ErrorType::DBQueryFailedPlayerTablePlayerMain) => warn!("Error: Failed to query main player..."),
                Err(ErrorType::UuidParsingFailed) => warn!("Error: Failed to parse uuid..."),
                Err(e) => warn!("Error: start_up_protocol -> pipeline_db_and_party_sync_main_player_uuids: {:?}", e)
            }
        }
    }

    // --- Internal Helper Functions --- //

    pub fn verify_if_party_size_exceeds_limit( 
        &self,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
    ) -> Result<(), ErrorType> {
        info!("Init: verify_if_party_size_exceeds_limit:");

        // Party Size Management Checks
        let party_size_limit = plugin.party_size.unwrap() as usize;
        let party_size = party.get_player_count_party();

        if party_size == party_size_limit {
            return Err(ErrorType::PartySizeAtSetLimit)
        }
        if party_size > party_size_limit {
            return Err(ErrorType::PartySizeGreaterThanSetLimit)
        }
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