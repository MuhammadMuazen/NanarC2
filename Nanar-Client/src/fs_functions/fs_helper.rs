pub fn system_time_to_readable(st: std::time::SystemTime) -> String {
    let datetime: chrono::DateTime<chrono::Local> = st.into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}


pub fn get_dir_size(path: &std::path::Path) -> Result<u64, std::io::Error> {
    
    let mut total: u64 = 0;
    
    if path.is_dir() {
        
        for entry in std::fs::read_dir(path)? {
            
            let entry: std::fs::DirEntry = entry?;
            let path: std::path::PathBuf = entry.path();
            
            if path.is_dir() {
                total += get_dir_size(&path)?;
            } else {
                total += entry.metadata()?.len();
            }
        }
    }
    
    Ok(total)
}

// permission information
pub fn get_permissions(entry: &std::fs::DirEntry, metadata: &std::fs::Metadata) -> String {
    
    let mut perm_string: String = String::with_capacity(10);
    
    // File type
    perm_string.push(if metadata.is_dir() { 'd' } else { '-' });
    
    // Owner permissions
    perm_string.push(if is_readable(metadata) { 'r' } else { '-' });
    perm_string.push(if is_writable(metadata) { 'w' } else { '-' });
    perm_string.push(if is_executable(entry) { 'x' } else { '-' });
    
    // Group permissions (same as owner on Windows)
    perm_string.push_str(&perm_string[1..4].to_string().as_str());
    
    // Others permissions (same as owner on Windows)
    perm_string.push_str(&perm_string[1..4].to_string().as_str());
    
    perm_string
}

pub fn is_readable(_metadata: &std::fs::Metadata) -> bool {

    true
}

pub fn is_writable(metadata: &std::fs::Metadata) -> bool {
    #[cfg(windows)] {
        !metadata.permissions().readonly()
    }
    #[cfg(unix)] {
        metadata.permissions().mode() & 0o200 != 0
    }
}

pub fn is_executable(entry: &std::fs::DirEntry) -> bool {
    
    #[cfg(windows)] {
        // Use the entry's path to check extension
        if let Some(ext) = entry.path().extension() {
            let ext: String = ext.to_string_lossy().to_lowercase();
            ext == "exe" || ext == "bat" || ext == "cmd" || ext == "com" || ext == "msi"
        } else {
            false
        }
    }
}
