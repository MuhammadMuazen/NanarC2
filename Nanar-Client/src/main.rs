use nanar_client::*;


fn main() {
    
    let addr_port: &str = "127.0.0.1:9999";
    let init_conn_pass: &str = "what is my name\n";

    let _ = connection_handler::init_conn_with_server(addr_port, init_conn_pass);
}