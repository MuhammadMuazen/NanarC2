/*
    This file holds the command CommandMessage response message form struct
*/

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CommandMessage {

    pub client_id: String,
    pub command_msg: String
}

impl CommandMessage {

    pub fn new_empty() -> Self {
        CommandMessage {
            client_id: String::new(), 
            command_msg: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct  CommandResponse {

    pub client_id: String,
    pub command_resp_msg: String
}

impl CommandResponse {

    pub fn new(client_id: String, command_resp_msg: String) -> Self {

        CommandResponse {
            client_id,
            command_resp_msg,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("Failed to serialize CommandResponse")
    }
}

pub fn parse_str_msg_to_command_msg(server_command: &str) -> Result<CommandMessage, serde_json::Error> {
    
    let result: Result<CommandMessage, serde_json::Error> = serde_json::from_str(server_command);
    
    if let Err(e) = &result {
        
        println!("[-] Could not parse the message into CommandMessage Struct the command form is wrong: {}", e);
    }
    
    result
}

