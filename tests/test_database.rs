#[cfg(test)]
mod tests {
    use bevy_easy_player_handler::*;
    use bevy_easy_shared_definitions::ErrorTypePlayerHandler;
    use uuid::Uuid;

    const PLAYER_EMAIL: &str = "test@example.com";
    const PLAYER_USERNAME: &str = "test_user";
    // const ALT_PLAYER_EMAIL: &str = "another_test@example.com";
    // const ALT_PLAYER_USERNAME: &str = "another_user";
    // const PLAYER_TYPE_AI_LOCAL: PlayerType = PlayerType::PlayerAiLocal;
    // const PLAYER_TYPE_AI_REMOTE: PlayerType = PlayerType::PlayerAiRemote;
    // const PLAYER_TYPE_LOCAL: PlayerType = PlayerType::PlayerLocal;
    // const PLAYER_TYPE_MAIN: PlayerType = PlayerType::PlayerMain;
    // const PLAYER_TYPE_REMOTE: PlayerType = PlayerType::PlayerRemote;

    #[test]
    fn test_database_start_up_protocol() -> Result<(), ErrorTypePlayerHandler> {
        todo!();
        Ok(())
    }

    #[test]
    fn test_database_start_up_protocol_finish() -> Result<(), ErrorTypePlayerHandler> {
        todo!();
        Ok(())
    }

    #[test]
    fn test_database_verify_if_party_size_exceeds_limit() -> Result<(), ErrorTypePlayerHandler> {
        todo!();
        Ok(())
    }

    #[test]
    fn test_database_dbplayer() -> Result<(), ErrorTypePlayerHandler> {
        let new_uuid = Uuid::now_v7();
        let dbplayer = DBPlayer {
            uuid: String::from(new_uuid.clone()),
            email: String::from(PLAYER_EMAIL),
            username: String::from(PLAYER_USERNAME),
        };
        let ref_uuid = dbplayer.get_uuid_string();
        let ref_email = dbplayer.get_email_string();
        let ref_username = dbplayer.get_username_string();

        assert_eq!(ref_uuid, &String::from(new_uuid.clone()));
        assert_eq!(ref_email, &String::from(PLAYER_EMAIL));
        assert_eq!(ref_username, &String::from(PLAYER_USERNAME));
        Ok(())
    }
}