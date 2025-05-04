use nanar_client::*;

fn main() {
    

    println!("{}>", fs_functions::get_current_dir());
    println!("{}", fs_functions::list_directory_contents(fs_functions::get_current_dir().as_str()));
    
    let set_path: Result<(), std::io::Error> = fs_functions::set_current_path("C:\\");

    if set_path.is_ok() {
        println!("{}>", fs_functions::get_current_dir());
        //println!("{}", fs_functions::list_directory_contents(fs_functions::get_current_dir().as_str()))
    } else if set_path.is_err() {
        println!("Path does not exist")
    }

    /*
        Testing List Directory
    */
    //println!("{}", fs_functions::list_directory_contents(fs_functions::get_current_dir().as_str()));

    //println!("{}", fs_functions::read_file_content("D:\\personal\\projects\\NanarC2\\Nanar-Client\\src\\main.rs"));

    /*
        Testing writing to a file
    */
    // let write_file: Result<(), std::io::Error> = fs_functions::write_to_file(
    //     "C:\\Users\\muhammad\\Desktop\\new.txt", "something");
    
    // if write_file.is_ok() {
    //     println!("Success Writing");
    // } else if write_file.is_err() {
    //     println!("Error Writing")
    // }

    /*
        Testing Create Directory
    */
    // let cd = fs_functions::create_dir("C:\\Users\\muhammad\\Desktop\\hello\\");

    // if cd.is_ok() {
    //     println!("Fine");
    // } else if cd.is_err() {
    //     println!("Error");
    // }

    /*
        Testing Remove file and Directory
    */
    // let rf = fs_functions::remove_file(r"C:\Users\muhammad\Desktop\new.py");
    // if rf.is_ok() {
    //     println!("Fine");
    // } else if rf.is_err() {
    //     println!("Error");
    // }

    // let rd = fs_functions::remove_dir(r"C:\Users\muhammad\Desktop\new");
    // if rd.is_ok() {
    //     println!("Fine");
    // } else if rd.is_err() {
    //     println!("Error");
    // }

    /*
        Testing copy file and dir
    */

    // let cp = fs_functions::copy_file_dir(
    //     r"D:\personal\projects\NanarC2\Nanar-Client\src\main.rs", 
    //     r"C:\Users\muhammad\Desktop\main.rs");

    // if cp.is_ok() {
    //     println!("Fine");
    // } else if cp.is_err() {
    //     println!("{}", std::io::Error::last_os_error());
    // }

    
}