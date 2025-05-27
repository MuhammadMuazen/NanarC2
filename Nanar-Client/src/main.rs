use nanar_client::*;


fn main() {
    /*
        Testing Server Connection
    */

    let _ = connection_handler::init_conn_with_server("127.0.0.1", "9999", "WHAT");
    
}