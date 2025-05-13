use std::io::{Read, Write};

// TODO Still in testing (add more connection functions)
pub fn init_conn_with_server(addr_port: &str, init_conn_pass: &str) -> std::io::Result<()> {

    let mut time_before_heartbeat: i16 = 3000;
    let mut buffer: [u8; 1024]   = [0; 1024];

    let mut stream: std::net::TcpStream = std::net::TcpStream::connect(addr_port)
        .expect("[-] Error: Could not connect to the server");

    loop {
        
        match stream.write_all(init_conn_pass.as_bytes()) {
            
            Ok(_) => println!("[+] Write successfull"),
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