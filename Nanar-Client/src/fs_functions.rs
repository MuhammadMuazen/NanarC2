use std::io::Write;
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
    let mut dir_contents: String = String::new();
    dir_contents.push_str("Perm\t\tModified\t\tSize\t\tName\n");
    dir_contents.push_str("----------------------------------------------------------------------\n");
    
    if let Ok(entries) = std::fs::read_dir(std::path::Path::new(dir_path)) {
        
        for entry in entries.flatten() {
            
            if let Ok(metadata) = entry.metadata() {
                
                let file_name: String = entry.file_name().to_string_lossy().into_owned();
                
                // Get directory size
                let size: u64 = if metadata.is_dir() {
                    0
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
pub fn read_file_content(file_path_str: &str) -> String {

    let file_content: String = std::fs::read_to_string(file_path_str).expect("File Content");

    file_content
}

// write file
pub fn write_to_file(file_path_str: &str, content: &str) -> std::io::Result<()> {
    
    let mut file: std::fs::File = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path_str)?;
    
    file.write_all(content.as_bytes())?;
    
    Ok(())
}

// Remvoe file
pub fn remove_file(file_path_str: &str) -> std::io::Result<()> {

    std::fs::remove_file(file_path_str)?;

    Ok(())
}

// Creart directory
pub fn create_dir(dir_path_str: &str) -> std::io::Result<()> {
    
    std::fs::create_dir(dir_path_str)?;

    Ok(())
}

// Remove directory
pub fn remove_dir(dir_path_str: &str) -> std::io::Result<()> {

    std::fs::remove_dir(dir_path_str)?;

    Ok(())
}

// copy file or dir
pub fn copy_file_dir(source_path_str: &str, destination_path_str: &str) -> std::io::Result<()> {

    let source_path: &std::path::Path= std::path::Path::new(&source_path_str);
    let destination_path: &std::path::Path = std::path::Path::new(&destination_path_str);
    
    if source_path.is_dir() {
        
        // Create destination directory if it doesn't exist
        std::fs::create_dir_all(destination_path)?;
        
        // Recursively copy directory contents
        for entry in std::fs::read_dir(source_path)? {
            
            let entry: std::fs::DirEntry = entry?;
            let entry_path: std::path::PathBuf = entry.path();
            let dest_path: std::path::PathBuf = destination_path.join(entry.file_name());
            
            if entry_path.is_dir() {
                copy_file_dir(
                    entry_path.to_str().unwrap(), dest_path.to_str().unwrap())?;
            } else {
                std::fs::copy(entry_path, dest_path)?;
            }
        }
    } else {

        // If source is a file, ensure parent directory exists
        if let Some(parent) = destination_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::copy(source_path, destination_path)?;
    }

    Ok(())
}

// move file or dir
pub fn move_file_dir(source_path_str: &str, destination_path_str: &str) -> std::io::Result<()> {
    
    let source_path: &std::path::Path = std::path::Path::new(source_path_str);
    let destination_path: &std::path::Path = std::path::Path::new(destination_path_str);

    // First rename
    match std::fs::rename(source_path, destination_path) {
        Ok(_) => return Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::CrossesDevices => {}
        Err(e) => return Err(e),
    }

    // Copy then delete (for cross-filesystem moves)
    copy_file_dir(source_path_str, destination_path_str)?;

    // Remove the original
    if source_path.is_dir() {
        std::fs::remove_dir_all(source_path)?;
    } else {
        std::fs::remove_file(source_path)?;
    }

    Ok(())
}