use std::io::Write;

pub fn init_conn_with_server(addr_port: &str, init_conn_pass: &str) -> std::io::Result<()> {

    let mut time_before_heartbeat: i16 = 3000;

    let mut stream: std::net::TcpStream = std::net::TcpStream::connect(addr_port)
        .expect("[-] Error: Could not connect to the server");

    loop {
        match stream.write_all(init_conn_pass.as_bytes()) {
            Ok(_) => println!("Write successful"),
            Err(e) => eprintln!("Write failed: {}", e),

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