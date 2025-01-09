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
    pub fn action_count_players_in_db(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<i32, ErrorType> {
        info!("Init: action_count_players_in_db:");
        
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
        let party_player_count_query: i32 = conn.query_row(
            "SELECT COUNT(*) AS PartyPlayerCount FROM player_table;",
            (),
            |row| row.get(0),
        )
        .map_err(|e| {
            error!("Failed to execute query to count player records: [{:?}]", e);
            ErrorType::DBQueryFailedPlayerCount
        })?;

        info!("Party Players Count: [{:?}]", party_player_count_query);

        Ok(party_player_count_query)
    }

    pub fn action_sync_party_and_db_main_player(
        &self,
        db: &Res<DatabaseConnection>,
        party: &mut ResMut<Party>,
    ) -> Result<(), ErrorType> {
        info!("Init: action_sync_party_and_db_main_player:");

        let party_main_player_uuid = party.main_player_clone_player_id();
        let database_main_player = match self.action_query_main_player(&db) {
            Ok(dbplayer) => dbplayer,
            Err(e) => {
                warn!("[ Error ] action_sync_party_and_db_main_player -> action_query_main_player: [{:?}]", e);
                return Err(ErrorType::DBQueryFailedPlayerTablePlayerMain);
            },
        };
        let database_main_player_uuid = match Uuid::try_parse(database_main_player.uuid.as_str()) {
            Ok(uuid) => uuid,
            Err(e) => {
                warn!("[ Error ] action_sync_party_and_db_main_player -> Uuid::try_parse(database_main_player.uuid.as_str()): [{:?}]", e);
                return Err(ErrorType::UuidParsingFailed);
            },
        };

        if party_main_player_uuid != database_main_player_uuid {
            party.active_player_set_uuid(database_main_player_uuid);
        };

        Ok(())
    }

    pub fn action_table_player_init(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<(), ErrorType> {
        info!("Init: action_table_player_init:");
        
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
        
        // Execute the SQL statement to create the player table
        conn.execute(
            "CREATE TABLE player_table (
                uuid TEXT PRIMARY KEY,
                email BLOB,
                user_name BLOB
            )",
            (),
        )
        .map_err(|_| ErrorType::DBActionFailedPlayerTableCreation)?; // Map any error to ErrorType and propagate it

        Ok(()) // Return success if the table is created without errors
    }

    pub fn action_insert_player_record(
        &self,
        db: &Res<DatabaseConnection>,
        main_player_uuid: String, 
        main_player_email: Option<String>, 
        main_player_username: Option<String>,
        test_player: bool,
    ) -> Result<(), ErrorType> {
        info!("Init: action_insert_player_record: Player [{}]", &main_player_uuid);
        info!("UserName: [{:?}], Email: [{:?}], TestPlayer: [{}]", &main_player_username, &main_player_email, &test_player);
        
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

        conn.execute(
            "INSERT INTO player_table (uuid, email, user_name) VALUES (?1, ?2, ?3)",
            (main_player_uuid, main_player_email, main_player_username),
        )
        .map_err(|_| if test_player {ErrorType::DBActionFailedPlayerTableInsertRecordTestRef} else {ErrorType::DBActionFailedPlayerTableInsertRecordPlayer})?;

        Ok(())
    }

    pub fn action_remove_all_player_records(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<(), ErrorType> {
        info!("Init: action_remove_all_player_records:");
        
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

        conn.execute(
            "DELETE FROM player_table",
            (),
        )
        .map_err(|_| ErrorType::DBDeleteFailedPlayerTableDropAllRecords)?;

        Ok(())
    }

    pub fn action_remove_player_record(
        &self,
        db: &Res<DatabaseConnection>,
        player_uuid: &Uuid,
    ) -> Result<(), ErrorType> {
        info!("Init: action_remove_all_player_records:");
        
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

        let player_uuid = player_uuid.to_owned();
        let player_uuid_string = String::from(player_uuid);
        let player_uuid_str = player_uuid_string.as_str();
        // setup and execute deletion of single player record from DB
        let delete_call = format!("DELETE FROM player_table WHERE uuid LIKE %{}%", player_uuid_str);
        let delete_call_sqlite = delete_call.as_str();
        conn.execute(
            delete_call_sqlite,
            (),
        )
        .map_err(|_| ErrorType::DBDeleteFailedPlayerRecordFromPlayerTable)?;

        Ok(())
    }

    pub fn action_query_main_player(
        &self,
        db: &Res<DatabaseConnection>,
    ) -> Result<DBPlayer, ErrorType> {    
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
            Err(ErrorType::DBQueryFailedPlayerTablePlayerMain)
        }
    }
}