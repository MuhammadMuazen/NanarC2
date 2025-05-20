use nanar_client::*;


fn main() {
    /*
        Testing Server Connection
    */
<<<<<<< HEAD
    let addr_port: &str = "127.0.0.1:9999";
    let init_conn_pass: &str = "what is my name\n";

    let _ = connection_handler::init_conn_with_server(addr_port, init_conn_pass);
=======

    let _ = connection_handler::init_conn_with_server("127.0.0.1", "9999", "WHAT");
>>>>>>> 2c8d6c5950dcd6b79195e315fccfb0018e0e3597
    //////////////////////////////////////////////////
    /*
        Tesing PS Listing
    */
    //println!("{}", ps_functions::process_list());

    /*
        Tesing process kill
    */
    //println!("{}", ps_functions::process_kill(12312312));
}