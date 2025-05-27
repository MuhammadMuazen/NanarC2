mod ps_helper;

pub fn control_child_processes(suspend_flag: bool) -> windows::core::Result<()>{

    let current_pid: u32 = std::process::id();
    let child_pids: Vec<u32> = ps_helper::get_child_processes(current_pid)?;

    if suspend_flag == true {
        // Suspend all children
        for pid in &child_pids {
        ps_helper::suspend_process(*pid)?;
        }
    } else if suspend_flag == false {
        // Resume all children
        for pid in &child_pids {
            ps_helper::resume_process(*pid)?;
        }
    }

    Ok(())
}

pub fn run_exec(exec_file_name: &str, args: &str) {

    std::process::Command::new(exec_file_name)
    .args(&[args])
    .spawn()
    .expect(format!("[-] Process Creation Error: {}", std::io::Error::last_os_error()).as_str());
}

pub fn process_list() -> String {

    let task_list: tasklist::Tasklist = tasklist::Tasklist::new().unwrap();
    let mut task_list_buffer: String = String::new();

    for task in task_list {

        if task.get_user().is_err() {

            task_list_buffer.push_str(
                format!(
                    "Process Id --> {}\nProcess Name: {}\nProcess User --> {}\nExecutable Path --> {}\n\
                    Arch --> Is_WoW: {}, Process_Arch: {}, Native_Arch: {}\n\
                    Memory Usage --> {:.3} MB\nParent Process ID: {}\nProcess SID: {}\
                    \n---------------------------------------------------------------------------\n", 
                    task.get_pid(), 
                    task.get_pname(), 
                    task.get_user().unwrap_or("[-] Error: Access is denied".to_string()),
                    task.get_path().unwrap_or("[-] Error: Access is denied".to_string()).as_str(), 
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).0,
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).1,
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).2,
                    task.get_memory_info().get_total_memory_usage() as f32 / (1024f32 * 1024f32),
                    task.get_parrent().unwrap_or(0),
                    task.get_sid().unwrap_or("[-] Error: Access is denied".to_string()))
                    .as_str());
        } else {
            task_list_buffer.push_str(
                format!(
                    "Process Id --> {}\nProcess Name --> {}\nProcess User --> {}\nExecutable Path --> {}\n\
                    Arch --> Is_WoW: {}, Process_Arch: {}, Native_Arch: {}\n\
                    Memory Usage --> {:.3} MB\nParent Process ID: {}\nProcess SID: {}\
                    \n---------------------------------------------------------------------------\n", 
                    task.get_pid(), 
                    task.get_pname(), 
                    task.get_user().unwrap().as_str(),
                    task.get_path().unwrap().as_str(), 
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).0,
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).1,
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).2,
                    task.get_memory_info().get_total_memory_usage() as f32 / (1024f32 * 1024f32),
                    task.get_parrent().unwrap_or(0),
                    task.get_sid().unwrap_or("[-] Unable to get the process SID!".to_string()))
                    .as_str());
        }
    }

    task_list_buffer
}   

pub fn process_kill(process_id: u32) -> String {
    
    let task_list: tasklist::Tasklist = match tasklist::Tasklist::new() {
        
        Ok(list) => list,
        Err(e) => return format!("[-] Error: Failed to get task list: {}", e),
    };

    for task in task_list {
        
        if task.get_pid() == process_id {
            
            return match task.kill() {
                
                Ok(_) => format!("[+] Process {} killed successfully!", process_id),
                Err(e) => format!("[-] Error: Failed to kill process {}: {}", process_id, e),
            };
        }
    }

    format!("[-] Error: No process found with ID: {}", process_id)
}