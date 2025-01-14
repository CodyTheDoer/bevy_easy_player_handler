use bevy::prelude::*;

use bevy_easy_shared_definitions::ErrorTypePlayerHandler;

use crate::{
    BevyEasyPlayerHandlerPlugin, 
    DatabaseConnection,
    DBPlayer, 
    Party, 
    PlayerHandlerInterface, 
};
use rusqlite::Result;

impl PlayerHandlerInterface {
    pub fn get() -> Self {
        PlayerHandlerInterface {}
    }

    pub fn start_up_protocol(
        // mut commands: Commands,
        db: Res<DatabaseConnection>,
        phi: ResMut<PlayerHandlerInterface>,
        mut plugin: ResMut<BevyEasyPlayerHandlerPlugin>,
        mut party: ResMut<Party>,
    ) {
        info!("BevyEasyPlayerHandler: [ Start Up Protocol ]");
    
        let player_table_exists = phi.query_table_player_exists(&db).unwrap();
    
        info!("Result [query_table_player_exists]: [{}]", player_table_exists);
    
        if !player_table_exists {
            phi.action_table_player_init(&db).unwrap();
        }
    
        // ----- [ Vertify database test ref and main player exists ] ----- //
    
        let players_test_ref_and_owner_exists = phi.query_test_ref_and_main_player_exists(&db).unwrap();
        
        info!("Result [query_test_ref_and_main_player_exists]: [{}]", players_test_ref_and_owner_exists);
    
        if !players_test_ref_and_owner_exists {
            phi.pipeline_db_and_party_init_test_ref_and_main_player(&db, &plugin, &mut party).unwrap();
        }
    
        // ----- [ Sync party and database main players uuid ] ----- //
    
        let party_and_database_main_player_synced = phi.query_party_and_db_main_player_synced(&db, &mut party).unwrap();
        
        info!("Result [query_party_and_db_main_player_synced]: [{}]", party_and_database_main_player_synced);
    
        if !party_and_database_main_player_synced {
            match phi.pipeline_db_and_party_sync_main_player_uuids(&db, &mut party, &mut plugin){
                Ok(sync) => sync,
                Err(e) => {
                    warn!("startup_protocol -> pipeline_db_and_party_sync_main_player_uuids [ Failed ] Error: {:?}", e);
                }, 
            };
        }
    }

    // --- Internal Helper Functions --- //

    pub fn verify_if_party_size_exceeds_limit( 
        &self,
        plugin: &ResMut<BevyEasyPlayerHandlerPlugin>,
        party: &mut ResMut<Party>,
    ) -> Result<(), ErrorTypePlayerHandler> {
        info!("Init: verify_if_party_size_exceeds_limit:");

        // Party Size Management Checks
        let party_size_limit = plugin.party_size.unwrap() as usize;
        let party_size = party.get_player_count_party()?;

        if party_size == party_size_limit {
            return Err(ErrorTypePlayerHandler::PartySizeAtSetLimit)
        }
        if party_size > party_size_limit {
            return Err(ErrorTypePlayerHandler::PartySizeGreaterThanSetLimit)
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