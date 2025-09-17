use nanar_server::args_handler;

fn main() {
    
    // Get the command line args
    let args: Vec<String> = std::env::args().collect();

    args_handler::arguments_handler(args);
}
