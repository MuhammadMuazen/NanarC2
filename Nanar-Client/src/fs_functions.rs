pub fn get_current_dir() -> String {
    
    let current_path: String = std::env::current_dir()
        .expect("Failed to get current directory") // Panics if error
        .to_str().expect("Path is not valid UTF-8").to_string();

    current_path
}

pub fn list_current_dir()  {

    let current_dir_content: std::fs::ReadDir = std::fs::read_dir(get_current_dir()).unwrap();

    for content in current_dir_content {

        let content_name = content.unwrap().path().is_dir();
        println!("Name: {}", content.unwrap().path().display());
        
    }
}