use crate::{
    Player,
    PlayerAiLocal,
    PlayerAiRemote,
    PlayerLocal,
    PlayerMain,
    PlayerRemote,
    PlayerType,
};
use bevy_easy_shared_definitions::ErrorTypePlayerHandler;

use uuid::Uuid;

impl Player for PlayerAiLocal {
    fn new(player_email: Option<String>, player_username: Option<String>, player_uuid: Option<Uuid>, player_type: PlayerType) -> Self {
        PlayerAiLocal {
            player_email: player_email.map(|email| email),
            player_type: player_type,
            player_username: player_username.map(|username| username), 
            player_uuid: player_uuid.unwrap_or_else(|| Uuid::now_v7()),
        }
    }

    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_email {
            Some(player_email) => Ok(player_email),
            None => Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                format!("PlayerAiLocal::get_player_email() Error: Missing Player Email")
            )),
        }
    }

    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler> {
        match &self.player_uuid {
            uuid => Ok(uuid),
        }
    }

    fn get_player_type(&self) -> Result<&PlayerType, ErrorTypePlayerHandler> {
        match &self.player_type {
            player_type => Ok(player_type),
        }
    }

    fn get_player_username(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_username {
            Some(player_username) => Ok(player_username),
            None => Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                format!("PlayerAiLocal::get_player_username() Error: Missing Player User Name")
            )),
        }
    }

    fn set_player_email(&mut self, new_email: &str) -> Result<(), ErrorTypePlayerHandler> {
        let email = new_email.to_string();
        let player_email = self.get_player_email()?;
        if player_email != &email {
            self.player_email = Some(email);
            let player_email = self.get_player_email()?;
            if player_email != new_email {
                return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                    format!("PlayerAiLocal::set_player_email() Error: New Email didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
            format!("PlayerAiLocal::set_player_email() Error: New Email matches existing email")
        ));
    }

    fn set_player_id(&mut self, new_id: Uuid) -> Result<(), ErrorTypePlayerHandler> {
        let id = new_id.to_owned();
        let player_id = self.get_player_id()?;
        if player_id != &id {
            self.player_uuid = id;
            let player_id = self.get_player_id()?;
            if player_id != &new_id {
                return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                    format!("PlayerAiLocal::set_player_id() Error: New id didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
            format!("PlayerAiLocal::set_player_id() Error: New id matches existing id")
        ));
    }

    fn set_player_username(&mut self, new_username: &str) -> Result<(), ErrorTypePlayerHandler> {
        let username = new_username.to_owned();
        let player_username = self.get_player_username()?;
        if player_username != &username {
            self.player_username = Some(username);
            let player_username = self.get_player_username()?;
            if player_username != &new_username {
                return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                    format!("PlayerAiLocal::set_player_username() Error: New username didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
            format!("PlayerAiLocal::set_player_username() Error: New username matches existing username")
        ));
    }
}

// --------------------------------------- //

impl Player for PlayerAiRemote {
    fn new(player_email: Option<String>, player_username: Option<String>, player_uuid: Option<Uuid>, player_type: PlayerType) -> Self {
        PlayerAiRemote {
            player_email: player_email.map(|email| email),
            player_type: player_type,
            player_username: player_username.map(|username| username), 
            player_uuid: player_uuid.unwrap_or_else(|| Uuid::now_v7()),
        }
    }

    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_email {
            Some(player_email) => Ok(player_email),
            None => Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                format!("PlayerAiRemote::get_player_email() Error: Missing Player Email")
            )),
        }
    }

    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler> {
        match &self.player_uuid {
            uuid => Ok(uuid),
        }
    }

    fn get_player_type(&self) -> Result<&PlayerType, ErrorTypePlayerHandler> {
        match &self.player_type {
            player_type => Ok(player_type),
        }
    }

    fn get_player_username(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_username {
            Some(player_username) => Ok(player_username),
            None => Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                format!("PlayerAiRemote::get_player_username() Error: Missing Player User Name")
            )),
        }
    }

    fn set_player_email(&mut self, new_email: &str) -> Result<(), ErrorTypePlayerHandler> {
        let email = new_email.to_string();
        let player_email = self.get_player_email()?;
        if player_email != &email {
            self.player_email = Some(email);
            let player_email = self.get_player_email()?;
            if player_email != new_email {
                return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                    format!("PlayerAiRemote::set_player_email() Error: New Email didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
            format!("PlayerAiRemote::set_player_email() Error: New Email matches existing email")
        ));
    }

    fn set_player_id(&mut self, new_id: Uuid) -> Result<(), ErrorTypePlayerHandler> {
        let id = new_id.to_owned();
        let player_id = self.get_player_id()?;
        if player_id != &id {
            self.player_uuid = id;
            let player_id = self.get_player_id()?;
            if player_id != &new_id {
                return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                    format!("PlayerAiRemote::set_player_id() Error: New id didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
            format!("PlayerAiRemote::set_player_id() Error: New id matches existing id")
        ));
    }

    fn set_player_username(&mut self, new_username: &str) -> Result<(), ErrorTypePlayerHandler> {
        let username = new_username.to_owned();
        let player_username = self.get_player_username()?;
        if player_username != &username {
            self.player_username = Some(username);
            let player_username = self.get_player_username()?;
            if player_username != &new_username {
                return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                    format!("PlayerAiRemote::set_player_username() Error: New username didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
            format!("PlayerAiRemote::set_player_username() Error: New username matches existing username")
        ));
    }
}

// --------------------------------------- //

impl Player for PlayerLocal {
    fn new(player_email: Option<String>, player_username: Option<String>, player_uuid: Option<Uuid>, player_type: PlayerType) -> Self {
        PlayerLocal {
            player_email: player_email.map(|email| email),
            player_type: player_type,
            player_username: player_username.map(|username| username), 
            player_uuid: player_uuid.unwrap_or_else(|| Uuid::now_v7()),
        }
    }

    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_email {
            Some(player_email) => Ok(player_email),
            None => Err(ErrorTypePlayerHandler::PlayerLocalCallFailed(format!("PlayerLocal::get_player_email() Error: Missing Player Email"))),
        }
    }

    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler> {
        match &self.player_uuid {
            uuid => Ok(uuid),
        }
    }

    fn get_player_type(&self) -> Result<&PlayerType, ErrorTypePlayerHandler> {
        match &self.player_type {
            player_type => Ok(player_type),
        }
    }

    fn get_player_username(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_username {
            Some(player_username) => Ok(player_username),
            None => Err(ErrorTypePlayerHandler::PlayerLocalCallFailed(format!("PlayerLocal::get_player_username() Error: Missing Player User Name"))),
        }
    }

    fn set_player_email(&mut self, new_email: &str) -> Result<(), ErrorTypePlayerHandler> {
        let email = new_email.to_string();
        let player_email = self.get_player_email()?;
        if player_email != &email {
            self.player_email = Some(email);
            let player_email = self.get_player_email()?;
            if player_email != new_email {
                return Err(ErrorTypePlayerHandler::PlayerLocalCallFailed(
                    format!("PlayerLocal::set_player_email() Error: New Email didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerLocalCallFailed(
            format!("PlayerLocal::set_player_email() Error: New Email matches existing email")
        ));
    }

    fn set_player_id(&mut self, new_id: Uuid) -> Result<(), ErrorTypePlayerHandler> {
        let id = new_id.to_owned();
        let player_id = self.get_player_id()?;
        if player_id != &id {
            self.player_uuid = id;
            let player_id = self.get_player_id()?;
            if player_id != &new_id {
                return Err(ErrorTypePlayerHandler::PlayerLocalCallFailed(
                    format!("PlayerLocal::set_player_id() Error: New id didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerLocalCallFailed(
            format!("PlayerLocal::set_player_id() Error: New id matches existing id")
        ));
    }

    fn set_player_username(&mut self, new_username: &str) -> Result<(), ErrorTypePlayerHandler> {
        let username = new_username.to_owned();
        let player_username = self.get_player_username()?;
        if player_username != &username {
            self.player_username = Some(username);
            let player_username = self.get_player_username()?;
            if player_username != &new_username {
                return Err(ErrorTypePlayerHandler::PlayerLocalCallFailed(
                    format!("PlayerLocal::set_player_username() Error: New username didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerLocalCallFailed(
            format!("PlayerLocal::set_player_username() Error: New username matches existing username")
        ));
    }
}

// --------------------------------------- //

impl Player for PlayerMain {
    fn new(player_email: Option<String>, player_username: Option<String>, player_uuid: Option<Uuid>, player_type: PlayerType) -> Self {
        PlayerMain {
            player_email: player_email.map(|email| email),
            player_type: player_type,
            player_username: player_username.map(|username| username), 
            player_uuid: player_uuid.unwrap_or_else(|| Uuid::now_v7()),
        }
    }

    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_email {
            Some(player_email) => Ok(player_email),
            None => Err(ErrorTypePlayerHandler::PlayerMainCallFailed(format!("PlayerMain::get_player_email() Error: Missing Player Email"))),
        }
    }

    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler> {
        match &self.player_uuid {
            uuid => Ok(uuid),
        }
    }

    fn get_player_type(&self) -> Result<&PlayerType, ErrorTypePlayerHandler> {
        match &self.player_type {
            player_type => Ok(player_type),
        }
    }

    fn get_player_username(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_username {
            Some(player_username) => Ok(player_username),
            None => Err(ErrorTypePlayerHandler::PlayerMainCallFailed(format!("PlayerMain::get_player_username() Error: Missing Player User Name"))),
        }
    }

    fn set_player_email(&mut self, new_email: &str) -> Result<(), ErrorTypePlayerHandler> {
        let email = new_email.to_string();
        let player_email = self.get_player_email()?;
        if player_email != &email {
            self.player_email = Some(email);
            let player_email = self.get_player_email()?;
            if player_email != new_email {
                return Err(ErrorTypePlayerHandler::PlayerMainCallFailed(
                    format!("PlayerMain::set_player_email() Error: New Email didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerMainCallFailed(
            format!("PlayerMain::set_player_email() Error: New Email matches existing email")
        ));
    }

    fn set_player_id(&mut self, new_id: Uuid) -> Result<(), ErrorTypePlayerHandler> {
        let id = new_id.to_owned();
        let player_id = self.get_player_id()?;
        if player_id != &id {
            self.player_uuid = id;
            let player_id = self.get_player_id()?;
            if player_id != &new_id {
                return Err(ErrorTypePlayerHandler::PlayerMainCallFailed(
                    format!("PlayerMain::set_player_id() Error: New id didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerMainCallFailed(
            format!("PlayerMain::set_player_id() Error: New id matches existing id")
        ));
    }

    fn set_player_username(&mut self, new_username: &str) -> Result<(), ErrorTypePlayerHandler> {
        let username = new_username.to_owned();
        let player_username = self.get_player_username()?;
        if player_username != &username {
            self.player_username = Some(username);
            let player_username = self.get_player_username()?;
            if player_username != &new_username {
                return Err(ErrorTypePlayerHandler::PlayerMainCallFailed(
                    format!("PlayerMain::set_player_username() Error: New username didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerMainCallFailed(
            format!("PlayerMain::set_player_username() Error: New username matches existing username")
        ));
    }
}

// --------------------------------------- //

impl Player for PlayerRemote {
    fn new(player_email: Option<String>, player_username: Option<String>, player_uuid: Option<Uuid>, player_type: PlayerType) -> Self {
        PlayerRemote {
            player_email: player_email.map(|email| email),
            player_type: player_type,
            player_username: player_username.map(|username| username), 
            player_uuid: player_uuid.unwrap_or_else(|| Uuid::now_v7()),
        }
    }

    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_email {
            Some(player_email) => Ok(player_email),
            None => Err(ErrorTypePlayerHandler::PlayerRemoteCallFailed(format!("PlayerRemote::get_player_email() Error: Missing Player Email"))),
        }
    }

    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler> {
        match &self.player_uuid {
            uuid => Ok(uuid),
        }
    }

    fn get_player_type(&self) -> Result<&PlayerType, ErrorTypePlayerHandler> {
        match &self.player_type {
            player_type => Ok(player_type),
        }
    }

    fn get_player_username(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_username {
            Some(player_username) => Ok(player_username),
            None => Err(ErrorTypePlayerHandler::PlayerRemoteCallFailed(format!("PlayerRemote::get_player_username() Error: Missing Player User Name"))),
        }
    }

    fn set_player_email(&mut self, new_email: &str) -> Result<(), ErrorTypePlayerHandler> {
        let email = new_email.to_string();
        let player_email = self.get_player_email()?;
        if player_email != &email {
            self.player_email = Some(email);
            let player_email = self.get_player_email()?;
            if player_email != new_email {
                return Err(ErrorTypePlayerHandler::PlayerRemoteCallFailed(
                    format!("PlayerRemote::set_player_email() Error: New Email didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerRemoteCallFailed(
            format!("PlayerRemote::set_player_email() Error: New Email matches existing email")
        ));
    }

    fn set_player_id(&mut self, new_id: Uuid) -> Result<(), ErrorTypePlayerHandler> {
        let id = new_id.to_owned();
        let player_id = self.get_player_id()?;
        if player_id != &id {
            self.player_uuid = id;
            let player_id = self.get_player_id()?;
            if player_id != &new_id {
                return Err(ErrorTypePlayerHandler::PlayerRemoteCallFailed(
                    format!("PlayerRemote::set_player_id() Error: New id didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerRemoteCallFailed(
            format!("PlayerRemote::set_player_id() Error: New id matches existing id")
        ));
    }

    fn set_player_username(&mut self, new_username: &str) -> Result<(), ErrorTypePlayerHandler> {
        let username = new_username.to_owned();
        let player_username = self.get_player_username()?;
        if player_username != &username {
            self.player_username = Some(username);
            let player_username = self.get_player_username()?;
            if player_username != &new_username {
                return Err(ErrorTypePlayerHandler::PlayerRemoteCallFailed(
                    format!("PlayerRemote::set_player_username() Error: New username didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerRemoteCallFailed(
            format!("PlayerRemote::set_player_username() Error: New username matches existing username")
        ));
    }
}
