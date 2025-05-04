mod fs_helper;

pub fn get_current_dir() -> String {
    
    let current_path: String = std::env::current_dir()
        .expect("Failed to get current directory") // Panics if error
        .to_str().expect("Path is not valid UTF-8").to_string();

    current_path
}

pub fn set_current_path(wanted_path: &str) -> std::io::Result<()> {

    let requested_path: &std::path::Path = std::path::Path::new(wanted_path);

    std::env::set_current_dir(&requested_path)?;

    Ok(())
} 

pub fn list_directory_contents(dir_path: &str) -> String {
    let mut dir_contents = String::new();
    dir_contents.push_str("Perm\t\tModified\t\tSize\t\tName\n");
    dir_contents.push_str("----------------------------------------------------------------------\n");
    
    if let Ok(entries) = std::fs::read_dir(std::path::Path::new(dir_path)) {
        
        for entry in entries.flatten() {
            
            if let Ok(metadata) = entry.metadata() {
                
                let file_name: String = entry.file_name().to_string_lossy().into_owned();
                
                // Get directory size
                let size: u64 = if metadata.is_dir() {
                    fs_helper::get_dir_size(&entry.path()).unwrap_or(0)
                } else {
                    metadata.len()
                };
                
                // Format size in appropriate units
                let size_str: String = match size {
                    s if s < 1024 => format!("{} B", s),
                    s if s < 1024 * 1024 => format!("{:.2} KB", s as f64 / 1024.0),
                    s if s < 1024 * 1024 * 1024 => format!("{:.2} MB", s as f64 / (1024.0 * 1024.0)),
                    _ => format!("{:.2} GB", size as f64 / (1024.0 * 1024.0 * 1024.0)),
                };
                
                let modified: String = fs_helper::system_time_to_readable(
                    metadata.modified().unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                );
                
                // Get permissions
                let perms: String = fs_helper::get_permissions(&entry, &metadata);
                
                dir_contents.push_str(&format!(
                    "{}\t{}\t{}\t\t{}\n", perms, modified, size_str, file_name));
            }
        }
    }
    
    dir_contents
}

// creart, read, write, copy, move file
// read file
pub fn read_file_content(file_path: &str) -> String {

    let file_content: String = std::fs::read_to_string(file_path).expect("File Content");

    file_content
}

// write file
pub fn write_to_file(file_path: &str, content_to_write: &str) -> std::io::Result<()> {

    if !std::path::Path::new(file_path).exists() {
        
        let mut new_file: Result<std::fs::File, std::io::Error> = std::fs::File::create(file_path);
        new_file.write_all(content_to_write)?;
    }

    Ok(())
}