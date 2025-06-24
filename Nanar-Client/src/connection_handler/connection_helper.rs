use std::io::{Read, Write};
use crate::messages;
use aes_gcm::{
    aead::{Aead, generic_array::GenericArray}, 
    Aes256Gcm, 
    KeyInit
};

// Encryption function for the commmand communication
pub fn aes_gcm_encrypt(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> (Vec<u8>, Vec<u8>) {
    
    let key: &GenericArray<u8, _> = GenericArray::from_slice(key);
    
    let cipher: aes_gcm::AesGcm<aes_gcm::aes::Aes256, 
        aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UInt<
        aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UTerm, 
        aes_gcm::aead::consts::B1>, aes_gcm::aead::consts::B1>, aes_gcm::aead::consts::B0>, 
        aes_gcm::aead::consts::B0>> = Aes256Gcm::new(key);
    
    let nonce: &GenericArray<u8, aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UInt<
        aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UTerm, 
        aes_gcm::aead::consts::B1>, aes_gcm::aead::consts::B1>, aes_gcm::aead::consts::B0>, 
        aes_gcm::aead::consts::B0>> = GenericArray::from_slice(nonce);
    
    let ciphertext: Vec<u8> = cipher.encrypt(&nonce, plaintext).unwrap();
    
    (ciphertext, nonce.to_vec())
}

// Decryption function for the commmand communication
pub fn aes_gcm_decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    
    let key: &GenericArray<u8, _> = GenericArray::from_slice(key);
    
    let nonce: &GenericArray<u8, aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UInt<
        aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UTerm, 
        aes_gcm::aead::consts::B1>, aes_gcm::aead::consts::B1>, aes_gcm::aead::consts::B0>, 
        aes_gcm::aead::consts::B0>> = GenericArray::from_slice(nonce);
    
    let cipher: aes_gcm::AesGcm<aes_gcm::aes::Aes256, aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UInt<
        aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UInt<aes_gcm::aes::cipher::typenum::UTerm, 
        aes_gcm::aead::consts::B1>, aes_gcm::aead::consts::B1>, aes_gcm::aead::consts::B0>, 
        aes_gcm::aead::consts::B0>> = Aes256Gcm::new(key);
    
    cipher.decrypt(&nonce, ciphertext).unwrap()
}

// Convert ip string to u8 array of 4 elements to be used in sock addr
pub fn ip_to_u8_array(ip_str: &str) -> Option<[u8; 4]> {
    
    let (a, rest) = ip_str.split_once('.')?;
    let (b, rest) = rest.split_once('.')?;
    let (c, d) = rest.split_once('.')?;
    
    Some([
        a.parse().ok()?,
        b.parse().ok()?,
        c.parse().ok()?,
        d.parse().ok()?,
    ])
}

// This function is used to managed the connection of the heartbeat
pub fn hearbeat_connection(sock_addr: std::net::SocketAddr, heartbeat_message_duration: std::time::Duration, call_reason: &str) {

    // Buffer to store the server messages in
    let mut buffer: [u8; 1024]   = [0; 1024];

    loop {
        match std::net::TcpStream::connect(&sock_addr) {
            Err(e) => {
                println!("[-] Error: Could not initilize the heartbeat connection: {}", e);
                println!("[!] Retrying Hearbeat connection");
                std::thread::sleep(heartbeat_message_duration);
            },
            Ok(mut stream) => {
                println!("[!] Entering heartbeat message communication status!");

                // If called beacuse of failing in the connection or the communication
                if call_reason == messages::MISCONNECTION_OR_MISCOMMUNICATION {

                    // Now init the heartbeat connection
                    match stream.write(messages::HEARTBEAT_RETRY_INIT_CONNECTION_MSG) {
                        Ok(_) => println!("[+] Sent the message: {:?}", messages::HEARTBEAT_RETRY_INIT_CONNECTION_MSG),
                        Err(e) => println!("[-] Error could not send the HEARTBEAT_RETRY_CONNECTION_MSG: {}", e)
                    };

                    match stream.read(&mut buffer) {
                        
                        Ok(data) => if data > 0 {
                            let server_response: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..data]);
                            if server_response.to_string().as_bytes() == messages::HEARTBEAT_SUCCESS_RESPONSE_MSG {

                                println!("[+] Heartbeat connection succeeded...Retrying to initiate the connection again");

                            } else {
                                
                                println!("[-] Error: got message {} which is not the corrent message...Retrying!!!", 
                                server_response.to_string());
                            }
                        },
                        Err(e) => {
                            println!("[-] Error: Could not read the response to the message HEARTBEAT_RETRY_CONNECTION_MSG: {}", e);
                        }
                    };
                    
                } else if call_reason == messages::HEARTBEAT_NO_ACTION {

                    match stream.write(messages::HEARTBEAT_NO_ACTION_MSG) {
                        Ok(_) => println!("[+] Sent the message  HEARTBEAT_NO_ACTION_MSG"),
                        Err(e) => {
                            println!("[-] Error: Could not send the HEARTBEAT_NO_ACTION_MSG: {}", e);
                        }
                    };

                    match stream.read(&mut buffer) {

                        Ok(data) if data > 0 => {
                            let server_response: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..data]);
                            if server_response.to_string().as_bytes() == messages::HEARTBEAT_NO_ACTION_RESPONSE_MSG {
                                println!("[+] Hearbeart message exchange succeeded!!")
                            }
                        },
                        Ok(_) => {
                            println!("[-] Could not get the data correctly from the server!!!");
                        },
                        Err(e) => {
                            println!("[-] Error: Did not get the response to the HEARTBEAT_NO_ACTION_MSG: {}", e)
                        }

                    }
                }
            }
        }
    }
}
