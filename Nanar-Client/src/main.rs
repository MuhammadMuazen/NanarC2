

use nanar_client::*;

fn main() {
    

    println!("{}>", fs_functions::get_current_dir());
    

    println!("{}", fs_functions::list_directory_contents(fs_functions::get_current_dir().as_str()))
}