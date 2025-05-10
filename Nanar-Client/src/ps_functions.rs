pub fn run_exec(exec_file_name: &str, args: &str) {

    std::process::Command::new(exec_file_name)
    .args(&[args])
    .status()
    .expect(format!("[-] Process Creation Error: {}", std::io::Error::last_os_error()).as_str());
}

pub fn process_list() -> String {

    let task_list: tasklist::Tasklist = tasklist::Tasklist::new().unwrap();
    let mut task_list_buffer: String = String::new();

    for task in task_list {

        if task.get_user().is_err() {
            task_list_buffer.push_str(
                format!(
                    "Process Id --> {}\nProcess Name: {}\nProcess User --> {}\nProcess Executable File --> {}\n\
                    Arch --> Is_WoW: {}, Process_Arch: {}, Native_Arch: {}\n\
                    Memory Usage --> {} MB\n\
                    \n---------------------------------------------------------------------------\n", 
                    task.get_pid(), 
                    task.get_pname(), 
                    task.get_user().unwrap_or("[-] Error: Access is denied".to_string()),
                    task.get_path().unwrap_or("[-] Error: Access is denied".to_string()).as_str(), 
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).0,
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).1,
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).2,
                    task.get_memory_info().get_total_memory_usage() / (1024 * 1024))
                    .as_str());
        } else {
            task_list_buffer.push_str(
                format!(
                    "Process Id --> {}\nProcess Name --> {}\nProcess User --> {}\nProcess Executable File --> {}\n\
                    Arch --> Is_WoW: {}, Process_Arch: {}, Native_Arch: {}\n\
                    Memory Usage --> {:.3} MB\n\
                    \n---------------------------------------------------------------------------\n", 
                    task.get_pid(), 
                    task.get_pname(), 
                    task.get_user().unwrap().as_str(),
                    task.get_path().unwrap().as_str(), 
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).0,
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).1,
                    task.get_architecture_info().unwrap_or((false, "UNKNOWN", "UNKNOWN")).2,
                    task.get_memory_info().get_total_memory_usage() as f32 / (1024f32 * 1024f32))
                    .as_str());
        }
    }

    task_list_buffer
}   