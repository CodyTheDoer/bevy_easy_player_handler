use bevy::prelude::*;

use bevy_easy_shared_definitions::DatabaseConnection;

use rusqlite::Result;
use uuid::Uuid;

use crate::{
    PlayerHandlerDatabaseCommands,
    DBPlayer,
    ErrorType,
    Party,
};

impl PlayerHandlerDatabaseCommands {
    pub fn query_existing_players(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<Vec<DBPlayer>, ErrorType> {    
        // Get and Lock the mutex to access the database connection
        let conn = db.get_connection();
        let conn = conn.lock();
        let conn = match conn {
            Ok(conn) => conn,
            Err(_) => {
                error!("Database connection lock poisoned.");
                return Err(ErrorType::DatabaseLockPoisoned);
            }
        };
    
        let mut stmt = conn
            .prepare("SELECT uuid, email, user_name FROM player_table")
            .map_err(|_| ErrorType::DBQueryFailedExistingPlayers)?; 
        
        let player_iter = stmt
            .query_map([], |row| {
                Ok(DBPlayer {
                    uuid: row.get(0)?,
                    email: row.get(1)?,
                    user_name: row.get(2)?,
                })
            })
            .map_err(|_| ErrorType::DBQueryMappingFailedExistingPlayers)?;
        
        let mut players: Vec<DBPlayer> = Vec::new(); 
        for player in player_iter {
            players.push(player.unwrap());
        }
        Ok(players)
    }
    
    pub fn query_test_ref_and_main_player_exists(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<bool, ErrorType> {
        info!("Init: query_test_ref_and_main_player_exists:");
        
        let count = match self.action_count_players_in_db(&db) {
            Ok(count) => count,
            Err(_) => {
                error!("Database connection lock poisoned.");
                return Err(ErrorType::DBQueryFailedPlayerCount);
            }
        };
    
        // count checks for at least the testing reference record and the main player
        let results = if count < 2 {
            false
        } else {
            true
        };
    
        Ok(results)
    }
    
    pub fn query_table_player_exists(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<bool, ErrorType> {
        info!("Init: query_table_player_exists:");
        
        // Get and Lock the mutex to access the database connection
        let conn = db.get_connection();
        let conn = conn.lock();
        let conn = match conn {
            Ok(conn) => conn,
            Err(_) => {
                error!("Database connection lock poisoned.");
                return Err(ErrorType::DatabaseLockPoisoned);
            }
        };
        
        // Execute the SQL statement to verify if the player table exists
        let does_exist : bool = conn.query_row(
            "SELECT EXISTS (
                SELECT 1 
                FROM sqlite_master 
                WHERE type = 'table' AND name = 'player_table'
            )",
            (),
            |row| row.get::<_, i32>(0),
        )
        .map_err(|_| {
            error!("Failed to execute query to check table existence.");
            ErrorType::DBQueryFailedVerifyPlayerTableExists
        })?
        == 1;
    
        info!("Player Table Exists: [{}]", does_exist );
    
        Ok(does_exist)
    }
    
    pub fn query_party_and_db_main_player_synced(
        &self,
        db: &Res<DatabaseConnection>,
        party: &mut ResMut<Party>,
    ) -> Result<bool, ErrorType> {
        info!("Init: query_party_and_db_main_player_synced:");
    
        let mut result_synced = false;
    
        let party_main_player_uuid = party.main_player_clone_player_id();
        let database_main_player = match self.action_query_main_player(&db) {
            Ok(dbplayer) => dbplayer,
            Err(e) => {
                warn!("[ Error ] query_party_and_db_main_player_synced -> action_query_main_player: [{:?}]", e);
                return Err(ErrorType::DBQueryFailedPlayerTablePlayerMain);
            },
        };
        let database_main_player_uuid = match Uuid::try_parse(database_main_player.uuid.as_str()) {
            Ok(uuid) => uuid,
            Err(e) => {
                warn!("[ Error ] query_party_and_db_main_player_synced -> Uuid::try_parse(database_main_player.uuid.as_str()): [{:?}]", e);
                return Err(ErrorType::UuidParsingFailed);
            },
        };
    
        if party_main_player_uuid == database_main_player_uuid {
            result_synced = true;
        };
    
        Ok(result_synced)
    }
    
    pub fn query_count_existing_players(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<i32, ErrorType> {
        info!("Init: query_count_existing_players:");
        
        // Get and Lock the mutex to access the database connection
        let conn = db.get_connection();
        let conn = conn.lock();
        let conn = match conn {
            Ok(conn) => conn,
            Err(_) => {
                error!("Database connection lock poisoned.");
                return Err(ErrorType::DatabaseLockPoisoned);
            }
        };
        
        // Check if there are any existing players in the database
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM player_table", [], |row| row.get(0))
            .unwrap_or(0);
    
        Ok(count)
    }
}