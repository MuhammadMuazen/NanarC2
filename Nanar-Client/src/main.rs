use nanar_client::*;

fn main() {
    

    println!("{}>", fs_functions::get_current_dir());
    
    let set_path: Result<(), std::io::Error> = fs_functions::set_current_path("E:\\");

    if set_path.is_ok() {
        println!("{}>", fs_functions::get_current_dir());
        println!("{}", fs_functions::list_directory_contents(fs_functions::get_current_dir().as_str()))
    } else if set_path.is_err() {
        println!("Path does not exist")
    }

    //println!("{}", fs_functions::list_directory_contents(fs_functions::get_current_dir().as_str()));

    //println!("{}", fs_functions::read_file_content("D:\\personal\\projects\\NanarC2\\Nanar-Client\\src\\main.rs"));

    // let write_file: Result<(), std::io::Error> = fs_functions::write_to_file(
    //     "C:\\Users\\muhammad\\Desktop\\new.txt", "something");
    
    // if write_file.is_ok() {
    //     println!("Success Writing");
    // } else if write_file.is_err() {
    //     println!("Error Writing")
    // }

    
}