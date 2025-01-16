use crate::{
    Player,
    PlayerAi,
    PlayerLocal,
    PlayerMain,
    PlayerRemote,
    PlayerType,
};
use bevy_easy_shared_definitions::ErrorTypePlayerHandler;

use std::sync::Arc;
use std::sync::Mutex;

use uuid::Uuid;

 //  -> Result<(), ErrorTypePlayerHandler> 
impl Player for PlayerAi {
    fn new(player_email: Option<String>, player_username: Option<String>, player_type: PlayerType) -> Self {
        PlayerAi {
            player_email: player_email.map(|email| email),
            player_id: Uuid::now_v7(),
            player_type: player_type,
            player_username: player_username.map(|username| username), 
        }
    }

    fn clone_with_new_id(&self) -> Result<Arc<Mutex<dyn Player + Send>>, ErrorTypePlayerHandler> {
        let mut clone: PlayerAi = self.clone();
        clone.player_id = Uuid::now_v7();
        if clone.player_id == self.player_id {
            return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                format!("PlayerAi::clone_with_new_id() Error: New Uuid didn't integrate properly")
            ))}
        let secured_clone: Arc<Mutex<PlayerAi>> = Arc::new(Mutex::new(clone));
        Ok(secured_clone)
    }

    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_email {
            Some(player_email) => Ok(player_email),
            None => Err(ErrorTypePlayerHandler::PlayerAiCallFailed(format!("PlayerAi::get_player_email() Error: Missing Player Email"))),
        }
    }

    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler> {
        match &self.player_id {
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
            None => Err(ErrorTypePlayerHandler::PlayerAiCallFailed(format!("PlayerAi::get_player_username() Error: Missing Player User Name"))),
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
                    format!("PlayerAi::set_player_email() Error: New Email didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
            format!("PlayerAi::set_player_email() Error: New Email matches existing email")
        ));
    }

    fn set_player_id(&mut self, new_id: Uuid) -> Result<(), ErrorTypePlayerHandler> {
        let id = new_id.to_owned();
        let player_id = self.get_player_id()?;
        if player_id != &id {
            self.player_id = id;
            let player_id = self.get_player_id()?;
            if player_id != &new_id {
                return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
                    format!("PlayerAi::set_player_id() Error: New id didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
            format!("PlayerAi::set_player_id() Error: New id matches existing id")
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
                    format!("PlayerAi::set_player_username() Error: New username didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PlayerAiCallFailed(
            format!("PlayerAi::set_player_username() Error: New username matches existing username")
        ));
    }
}

// --------------------------------------- //

impl Player for PlayerLocal {
    fn new(player_email: Option<String>, player_username: Option<String>, player_type: PlayerType) -> Self {
        PlayerLocal {
            player_email: player_email.map(|email| email),
            player_id: Uuid::now_v7(),
            player_type: player_type,
            player_username: player_username.map(|username| username), 
        }
    }

    fn clone_with_new_id(&self) -> Result<Arc<Mutex<dyn Player + Send>>, ErrorTypePlayerHandler> {
        let mut clone: PlayerLocal = self.clone();
        clone.player_id = Uuid::now_v7();
        if clone.player_id == self.player_id {
            return Err(ErrorTypePlayerHandler::PlayerLocalCallFailed(
                format!("PlayerLocal::clone_with_new_id() Error: New Uuid didn't integrate properly")
            ))}
        let secured_clone: Arc<Mutex<PlayerLocal>> = Arc::new(Mutex::new(clone));
        Ok(secured_clone)
    }

    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_email {
            Some(player_email) => Ok(player_email),
            None => Err(ErrorTypePlayerHandler::PlayerLocalCallFailed(format!("PlayerLocal::get_player_email() Error: Missing Player Email"))),
        }
    }

    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler> {
        match &self.player_id {
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
            self.player_id = id;
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
    fn new(player_email: Option<String>, player_username: Option<String>, player_type: PlayerType) -> Self {
        PlayerMain {
            player_email: player_email.map(|email| email),
            player_id: Uuid::now_v7(),
            player_type: player_type,
            player_username: player_username.map(|username| username), 
        }
    }

    fn clone_with_new_id(&self) -> Result<Arc<Mutex<dyn Player + Send>>, ErrorTypePlayerHandler> {
        let mut clone: PlayerMain = self.clone();
        clone.player_id = Uuid::now_v7();
        if clone.player_id == self.player_id {
            return Err(ErrorTypePlayerHandler::PartyActionFailed(
                format!("PlayerMain::clone_with_new_id() Error: New Uuid didn't integrate properly")
            ))}
        let secured_clone: Arc<Mutex<PlayerMain>> = Arc::new(Mutex::new(clone));
        Ok(secured_clone)
    }

    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_email {
            Some(player_email) => Ok(player_email),
            None => Err(ErrorTypePlayerHandler::PartyActionFailed(format!("PlayerMain::get_player_email() Error: Missing Player Email"))),
        }
    }

    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler> {
        match &self.player_id {
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
            None => Err(ErrorTypePlayerHandler::PartyActionFailed(format!("PlayerMain::get_player_username() Error: Missing Player User Name"))),
        }
    }

    fn set_player_email(&mut self, new_email: &str) -> Result<(), ErrorTypePlayerHandler> {
        let email = new_email.to_string();
        let player_email = self.get_player_email()?;
        if player_email != &email {
            self.player_email = Some(email);
            let player_email = self.get_player_email()?;
            if player_email != new_email {
                return Err(ErrorTypePlayerHandler::PartyActionFailed(
                    format!("PlayerMain::set_player_email() Error: New Email didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PartyActionFailed(
            format!("PlayerMain::set_player_email() Error: New Email matches existing email")
        ));
    }

    fn set_player_id(&mut self, new_id: Uuid) -> Result<(), ErrorTypePlayerHandler> {
        let id = new_id.to_owned();
        let player_id = self.get_player_id()?;
        if player_id != &id {
            self.player_id = id;
            let player_id = self.get_player_id()?;
            if player_id != &new_id {
                return Err(ErrorTypePlayerHandler::PartyActionFailed(
                    format!("PlayerMain::set_player_id() Error: New id didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PartyActionFailed(
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
                return Err(ErrorTypePlayerHandler::PartyActionFailed(
                    format!("PlayerMain::set_player_username() Error: New username didn't integrate properly")
                ));
            }
            return Ok(());
        }
        return Err(ErrorTypePlayerHandler::PartyActionFailed(
            format!("PlayerMain::set_player_username() Error: New username matches existing username")
        ));
    }
}

// --------------------------------------- //

impl Player for PlayerRemote {
    fn new(player_email: Option<String>, player_username: Option<String>, player_type: PlayerType) -> Self {
        PlayerRemote {
            player_email: player_email.map(|email| email),
            player_id: Uuid::now_v7(),
            player_type: player_type,
            player_username: player_username.map(|username| username), 
        }
    }

    fn clone_with_new_id(&self) -> Result<Arc<Mutex<dyn Player + Send>>, ErrorTypePlayerHandler> {
        let mut clone: PlayerRemote = self.clone();
        clone.player_id = Uuid::now_v7();
        if clone.player_id == self.player_id {
            return Err(ErrorTypePlayerHandler::PlayerRemoteCallFailed(
                format!("PlayerRemote::clone_with_new_id() Error: New Uuid didn't integrate properly")
            ))}
        let secured_clone: Arc<Mutex<PlayerRemote>> = Arc::new(Mutex::new(clone));
        Ok(secured_clone)
    }

    fn get_player_email(&self) -> Result<&String, ErrorTypePlayerHandler> {
        match &self.player_email {
            Some(player_email) => Ok(player_email),
            None => Err(ErrorTypePlayerHandler::PlayerRemoteCallFailed(format!("PlayerRemote::get_player_email() Error: Missing Player Email"))),
        }
    }

    fn get_player_id(&self) -> Result<&Uuid, ErrorTypePlayerHandler> {
        match &self.player_id {
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
            self.player_id = id;
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