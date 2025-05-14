use nanar_client::*;


fn main() {
    /*
        Testing Server Connection
    */

    let _ = connection_handler::init_conn_with_server("127.0.0.1", "9999", "WHAT");
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