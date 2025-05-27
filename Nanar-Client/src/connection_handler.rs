use std::io::{Read, Write};
mod connection_helper;
use super::ps_functions;

// TODO Still in testing (add more connection functions)
pub fn init_conn_with_server(server_addr: &str, server_port: &str, init_conn_pass: &str) -> std::io::Result<()> {

    println!("{}", init_conn_pass);

    // Times vars // Make 30000
    let time_before_heartbeat_ms: u64 = 3000;
    let duration_before_heartbeat: std::time::Duration = std::time::Duration::from_millis(time_before_heartbeat_ms);

    // Server ip and port parser
    let server_ip_u8: [u8; 4] = connection_helper::ip_to_u8_array(server_addr).unwrap();
    let server_port_u16: u16 = server_port.parse().expect("[!] Failed to parse the port number to u16");
    let server_ip_addr: std::net::Ipv4Addr = std::net::Ipv4Addr::new(
        server_ip_u8[0], server_ip_u8[1], server_ip_u8[2], server_ip_u8[3]
    );

    // Socket addr var
    let sock_addr: std::net::SocketAddr = std::net::SocketAddr::new(
         std::net::IpAddr::V4(server_ip_addr), server_port_u16);

    // Temp Buffer (TODO Change later)
    let mut buffer: [u8; 1024]   = [0; 1024];

    match std::net::TcpStream::connect_timeout(&sock_addr, duration_before_heartbeat) {
        
        Err(e) => {
            println!("[!] Error: Connection inilization timeout: {}", e);
            heartbeat(sock_addr, time_before_heartbeat_ms, "INIT_CONNECTION_FAILED")?;
        },
        Ok(mut stream) => {

            stream.set_write_timeout(Some(duration_before_heartbeat))?;
            stream.set_read_timeout(Some(duration_before_heartbeat))?;
            
            match stream.write(connection_helper::CHECK_SERVER_MSG) {
                
                Ok(_) => println!("[+] Sent {:?}", connection_helper::CHECK_SERVER_MSG),
                Err(e) => {
                    println!("[!] Error: Sending connection initilization key failed {}", e);
                    heartbeat(sock_addr, time_before_heartbeat_ms, "INIT_KEY_SENDING_FAILED")?;
                }
            }

            match stream.read(&mut buffer) {
                
                Ok(data) if data > 0 => {
                    let server_response: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..data]);
                    
                    if server_response.to_string().as_bytes() == connection_helper::SERVER_IS_UP_MSG {
                        
                        println!("Server Is UP {:?}", server_response)
                    }
                },
                Ok(_) => {
                    println!("Server closed the connection");
                },
                Err(e) => {
                    eprintln!("Read failed: {}", e);
                    heartbeat(sock_addr, time_before_heartbeat_ms, "FAILED_READ_DATA_FROM_SERVER")?;
                }
            }
            
            match stream.write(connection_helper::CLIENT_INIT_CONN_KEY_MSG) {

                Ok(_) => {
                    println!("[+] Send initlization key to the server")
                }
                Err(e) => {

                    println!("[!] Error sending the key {}", e);
                    heartbeat(sock_addr, time_before_heartbeat_ms, "COULD_NOT_SEND_KEY")?;
                }
            }

            match stream.read(&mut buffer) {
                
                Ok(data) if data > 0 => {
                    let server_response: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..data]);
                    println!("{}", server_response);
                    if server_response.to_string().as_bytes() == connection_helper::KEY_EXCHANGE_SUCCEEDED_MSG {
                        
                        println!("[+] Key exchange sucess!");
                        heartbeat(sock_addr, time_before_heartbeat_ms, "CONNECTION_SUCCEEDED")?;
                    }
                    else if server_response.to_string().as_bytes() == connection_helper::KEY_EXCHANGE_FAILED_MSG {

                        println!("[!] Key exchange failed becuase it was wrong! {}", server_response);
                        heartbeat(sock_addr, time_before_heartbeat_ms, "WRONG_KEY")?;
                    }
                },
                Ok(_) => {
                    println!("Server closed the connection");
                },
                Err(e) => {
                    
                    eprintln!("[!] Error: key exchage failed: {}", e);
                    heartbeat(sock_addr, time_before_heartbeat_ms, "KEY_EXCHANGE_FAILED_MSG")?;
                }
            }
            
            stream.shutdown(std::net::Shutdown::Both).expect("shutdown call failed");

        }
    }

    Ok(())
}

pub fn heartbeat(sock_addr: std::net::SocketAddr, time_before_heartbeat_ms: u64, call_reason: &str) -> std::io::Result<()>{

    println!("[+] Enter Heartbeat!!");

    println!("{}", time_before_heartbeat_ms);

    let suspend_flag: bool = false;
    
    // This is just for simplicity that's why there is many other options for the call_reason in other locations
    // Maybe later I will update it to be able to reconnect with the server from the point it missed the connection
    match call_reason {
        connection_helper::HEARTBEAT_NO_ACTION => connection_helper::HEARTBEAT_NO_ACTION,
        _ => connection_helper::MISCONNECTION_OR_COMMUNICATION
    };


    // Make 60000
    let hearbeat_message_timer_ms: u64 = 3000;
    let heartbeat_message_duration: std::time::Duration = std::time::Duration::from_millis(hearbeat_message_timer_ms);
    
    if call_reason != connection_helper::HEARTBEAT_NO_ACTION {
        // 1) Suspend all the child processes
        ps_functions::control_child_processes(suspend_flag)?;
        // 2) Initiate the heartbeat connection with the call reason MISCONNECTION_OR_COMMUNICATION
        connection_helper::hearbeat_connection(sock_addr, heartbeat_message_duration, call_reason);
    
    } else if call_reason == connection_helper::HEARTBEAT_NO_ACTION {
        // 1) Don't stop all the child processes
        // 2) Initiate the heartbeat connection with the call reason HEARTBEAT_NO_ACTION
        connection_helper::hearbeat_connection(sock_addr, heartbeat_message_duration, call_reason);
        // 3) 
    }

    Ok(())
}