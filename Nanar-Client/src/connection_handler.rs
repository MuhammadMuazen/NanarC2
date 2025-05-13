use std::io::{Read, Write};
mod connection_helper;

const CHECK_SERVER_MSG: &[u8] = "CHECK_SERVER_MSG".as_bytes();
const SERVER_IS_UP_MSG: &[u8] = "SERVER_IS_UP_MSG".as_bytes();
const SERVER_IS_DOWN_MSG: &[u8] = "SERVER_IS_DOWN_MSG".as_bytes();
const CLIENT_INIT_CONN_KEY_MSG: &[u8] = "CLIENT_INIT_CONN_KEY_MSG".as_bytes();
const KEY_EXCHANGE_SUCCEEDED_MSG: &[u8] = "KEY_EXCHANGE_SUCCEEDED_MSG".as_bytes();
const KEY_EXCHANGE_FAILED_MSG: &[u8] = "KEY_EXCHANGE_FAILED_MSG".as_bytes();

// TODO Still in testing (add more connection functions)
pub fn init_conn_with_server(server_addr: &str, server_port: &str, init_conn_pass: &str) -> std::io::Result<()> {

    let mut time_before_heartbeat: u16 = 3000;
    let duration_before_heartbeat: std::time::Duration = std::time::Duration::from_millis(time_before_heartbeat.into());

    let mut buffer: [u8; 1024]   = [0; 1024];
    let server_ip_u8: [u8; 4] = connection_helper::ip_to_u8_array(server_addr).unwrap();
    let server_port_u16: u16 = server_port.parse().expect("[!] Failed to parse the port number to u16");

    let server_ip_addr: std::net::Ipv4Addr = std::net::Ipv4Addr::new(
        server_ip_u8[0], server_ip_u8[1], server_ip_u8[2], server_ip_u8[3]
    );

    let sock_addr: std::net::SocketAddr = std::net::SocketAddr::new(
         std::net::IpAddr::V4(server_ip_addr), server_port_u16);

    match std::net::TcpStream::connect_timeout(&sock_addr, duration_before_heartbeat) {
        Err(e) => {
            println!("[!] Error: Connection inilization timeout: {}", e);
            heartbeat(time_before_heartbeat);
        },
        Ok(stream) => {

            
        }
    }

    loop {
        
        match stream.write_all(init_conn_pass.as_bytes()) {
            
            Ok(_) => println!("[+] Write suc"),
            Err(e) => eprintln!("Write failed: {}", e),
        }

        match stream.read(&mut buffer) {
            Ok(n) if n > 0 => {
                let response = String::from_utf8_lossy(&buffer[..n]);
                println!("Server response: {}", response);
            }
            Ok(_) => {
                println!("Server closed the connection");
                break;
            }
            Err(e) => {
                eprintln!("Read failed: {}", e);
                break;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(1000));
        
        time_before_heartbeat -= 1000;

        if time_before_heartbeat == 0 {
            
            break;
        }
    }



    stream.shutdown(std::net::Shutdown::Both).expect("shutdown call failed");

    Ok(())
}

pub fn heartbeat(time_sec: u16) {


}