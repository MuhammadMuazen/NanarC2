use std::io::Write;

pub fn init_conn_with_server(addr_port: &str, init_conn_pass: &str) -> std::io::Result<()> {

    let mut stream = std::net::TcpStream::connect(addr_port)
        .expect("[-] Error: Could not connect to the server");

    
    match stream.write_all(init_conn_pass.as_bytes()) {
        Ok(_) => println!("Write successful"),
        Err(e) => eprintln!("Write failed: {}", e),
    }


    stream.shutdown(std::net::Shutdown::Both).expect("shutdown call failed");

    Ok(())
}