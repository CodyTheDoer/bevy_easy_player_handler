use bevy::prelude::*;

use bevy_easy_shared_definitions::{
    DatabaseConnection,
    ErrorTypePlayerHandler,
};

use rusqlite::Result;
use uuid::Uuid;

use crate::{
    DBPlayer,
    Party,
    PlayerComponent,
    PlayerHandlerInterface,
};

impl PlayerHandlerInterface {    
    pub fn query_db_count_existing_players(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<i32, ErrorTypePlayerHandler> {
        info!("Init: query_count_existing_players:");
        
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
        
        // Check if there are any existing players in the database
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM player_table", [], |row| row.get(0))
            .unwrap_or(0);
    
        Ok(count)
    }

    pub fn query_db_existing_players(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<Vec<DBPlayer>, ErrorTypePlayerHandler> {    
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
    
        let mut stmt = conn
            .prepare("SELECT uuid, email, username FROM player_table")
            .map_err(|_| ErrorTypePlayerHandler::DBQueryFailed(format!("query_existing_players: Failed to get existing players...")))?; 
        
        let player_iter = stmt
            .query_map([], |row| {
                Ok(DBPlayer {
                    uuid: row.get(0)?,
                    email: row.get(1)?,
                    username: row.get(2)?,
                })
            })
            .map_err(|_| ErrorTypePlayerHandler::DBQueryMappingFailed(format!("query_existing_players: Failed to map existing players...")))?;
        
        let mut players: Vec<DBPlayer> = Vec::new(); 
        for player in player_iter {
            players.push(player.unwrap());
        }
        Ok(players)
    }

    pub fn query_db_main_player(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<DBPlayer, ErrorTypePlayerHandler> {    
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

        let mut stmt = conn
            .prepare("SELECT uuid, email, username FROM player_table")
            .map_err(|_| ErrorTypePlayerHandler::DBQueryFailed(format!("query_main_player: Failed to get existing players...")))?; 
        
        let player_iter = stmt
            .query_map([], |row| {
                Ok(DBPlayer {
                    uuid: row.get(0)?,
                    email: row.get(1)?,
                    username: row.get(2)?,
                })
            })
            .map_err(|_| ErrorTypePlayerHandler::DBQueryMappingFailed(format!("query_main_player: Failed to map existing players...")))?;
        
        let mut main_player_container: Vec<DBPlayer> = Vec::new(); 
        let mut idx: usize = 0; 
        for player in player_iter {
            if idx == 1 { // idx of 1 bypasses the test ref player created and returns the main.
                main_player_container.push(player.unwrap());
            }
            idx += 1;
        }
        if let Some(main_player) = main_player_container.pop() {
            Ok(main_player)
        } else {
            Err(ErrorTypePlayerHandler::DBQueryFailed(format!("Failed: to get main Player")))
        }
    }
    
    pub fn query_party_and_db_main_player_synced(
        &self,
        db: &Res<DatabaseConnection>,
        party: &mut ResMut<Party>,
        player_query: &Query<&PlayerComponent>,
    ) -> Result<bool, ErrorTypePlayerHandler> {
        info!("Init: query_party_and_db_main_player_synced:");

        println!("Step: 1 [ query_party_and_db_main_player_synced ]");
        let mut result_synced = false;
        println!("Step: 2 [ query_party_and_db_main_player_synced ]");
        let party_size = party.get_player_count_party(player_query)?;
        println!("party size: {}", party_size);
        println!("Step: 3 [ query_party_and_db_main_player_synced ]");
        if party_size > 0 {    
            println!("Step: 4 [ query_party_and_db_main_player_synced ]");
            let database_main_player = self.query_db_main_player(&db)?;
            println!("Step: 5 [ query_party_and_db_main_player_synced ]");
            let party_main_player_uuid = party.clone_main_player_uuid(player_query)?;
            println!("Step: 6 [ query_party_and_db_main_player_synced ]");
            let database_main_player_uuid = match Uuid::try_parse(database_main_player.uuid.as_str()) {
                Ok(uuid) => uuid,
                Err(e) => {
                    warn!("[ Error ] query_party_and_db_main_player_synced -> Uuid::try_parse(database_main_player.uuid.as_str()): [{:?}]", e);
                    return Err(ErrorTypePlayerHandler::UuidParsingFailed(e.to_string()));
                },
            };
            println!("Step: 7 [ query_party_and_db_main_player_synced ]");
            if party_main_player_uuid == database_main_player_uuid {
                println!("Step: 8 [ query_party_and_db_main_player_synced ]");
                result_synced = true;
            };
        }    
        println!("Step: 9 [ query_party_and_db_main_player_synced ]");
        Ok(result_synced)
    }
    
    pub fn query_db_table_player_exists(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<bool, ErrorTypePlayerHandler> {
        info!("Init: query_table_player_exists:");
        
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
            ErrorTypePlayerHandler::DBQueryFailed(format!("Failed: to verify 'player_table' exists"))
        })?
        == 1;
    
        info!("Player Table Exists: [{}]", does_exist );
    
        Ok(does_exist)
    }
    
    pub fn query_db_test_ref_and_main_player_exists(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<bool, ErrorTypePlayerHandler> {
        info!("Init: query_test_ref_and_main_player_exists:");
        
        let count = self.action_count_players_in_db(&db)?;
        
        // count checks for at least the testing reference record and the main player
        let results = if count < 2 {
            false
        } else {
            true
        };
    
        Ok(results)
    }
}