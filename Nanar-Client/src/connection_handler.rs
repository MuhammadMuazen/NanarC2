use std::io::{Read, Write};
mod connection_helper;

const CHECK_SERVER_MSG: &[u8] = "CHECK_SERVER_MSG".as_bytes();
const SERVER_IS_UP_MSG: &[u8] = "SERVER_IS_UP_MSG".as_bytes();
const SERVER_IS_DOWN_MSG: &[u8] = "SERVER_IS_DOWN_MSG".as_bytes();
const CLIENT_INIT_CONN_KEY_MSG: &[u8] = "CLIENT_INIT_CONN_KEY_MS".as_bytes();
const KEY_EXCHANGE_SUCCEEDED_MSG: &[u8] = "KEY_EXCHANGE_SUCCEEDED_MSG".as_bytes();
const KEY_EXCHANGE_FAILED_MSG: &[u8] = "KEY_EXCHANGE_FAILED_MSG".as_bytes();

// TODO Still in testing (add more connection functions)
pub fn init_conn_with_server(server_addr: &str, server_port: &str, init_conn_pass: &str) -> std::io::Result<()> {

    // Times vars
    let time_before_heartbeat: u64 = 3000;
    let duration_before_heartbeat: std::time::Duration = std::time::Duration::from_millis(time_before_heartbeat);

    // Server ip and port
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
            heartbeat(time_before_heartbeat, "INIT_CONNECTION_FAILED");
        },
        Ok(mut stream) => {

            stream.set_write_timeout(Some(duration_before_heartbeat))?;
            stream.set_read_timeout(Some(duration_before_heartbeat))?;
            
            match stream.write(CHECK_SERVER_MSG) {
                
                Ok(_) => println!("[+] Sent {:?}", CHECK_SERVER_MSG),
                Err(e) => {
                    println!("[!] Error: Sending connection initilization key failed {}", e);
                    heartbeat(time_before_heartbeat, "INIT_KEY_SENDING_FAILED");
                }
            }

            match stream.read(&mut buffer) {
                
                Ok(data) if data > 0 => {
                    let server_response: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..data]);
                    
                    if server_response.to_string().as_bytes() == SERVER_IS_UP_MSG {
                        
                        println!("Server Is UP {:?}", server_response)
                    }
                },
                Ok(_) => {
                    println!("Server closed the connection");
                },
                Err(e) => {
                    eprintln!("Read failed: {}", e);
                    heartbeat(time_before_heartbeat, "FAILED_READ_DATA_FROM_SERVER");
                }
            }
            
            match stream.write(CLIENT_INIT_CONN_KEY_MSG) {

                Ok(_) => {
                    println!("[+] Send initlization key to the server")
                }
                Err(e) => {

                    println!("[!] Error sending the key {}", e);
                    heartbeat(time_before_heartbeat, "COULD_NOT_SEND_KEY");
                }
            }

            match stream.read(&mut buffer) {
                
                Ok(data) if data > 0 => {
                    let server_response: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&buffer[..data]);
                    
                    if server_response.to_string().as_bytes() == KEY_EXCHANGE_SUCCEEDED_MSG {
                        
                        println!("[+] Key exchange sucess!");
                    }
                    else if server_response.to_string().as_bytes() == KEY_EXCHANGE_FAILED_MSG {

                        println!("[!] Key exchange failed becuase it was wrong! {}", server_response);
                        heartbeat(time_before_heartbeat, "WRONG_KEY");
                    }
                },
                Ok(_) => {
                    println!("Server closed the connection");
                },
                Err(e) => {
                    eprintln!("[!] Error: key exchage failed failed: {}", e);
                    heartbeat(time_before_heartbeat, "KEY_EXCHANGE_FAILED_MSG");;
                }
            }
            
            stream.shutdown(std::net::Shutdown::Both).expect("shutdown call failed");

        }
    }

    Ok(())
}

pub fn heartbeat(time_sec: u64, reason: &str) {


}