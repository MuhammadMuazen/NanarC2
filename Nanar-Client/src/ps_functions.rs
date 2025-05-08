pub fn run_exec(exec_file_name: &str, args: &str) {

    std::process::Command::new(exec_file_name)
    .args(&[args])
    .status()
    .expect(format!("[-] Process Creation Error: {}", std::io::Error::last_os_error()).as_str());
}

pub fn process_list() -> String {

    let task_list: tasklist::Tasklist = tasklist::Tasklist::new().unwrap();
    let mut task_list_buffer: String = String::new();
    
    tasklist::enable_debug_priv();

    for task in task_list {

        if task.get_user().is_err() {
            task_list_buffer.push_str(
                format!(
                    "Process Id:{}\nProcess Name:{}\nProcess User: {}\nProcess Info: {}\n-------------------------\n", 
                    task.get_pid(), task.get_pname(), "[-] Error: Access is denied",
                    "[-] Error: Access is denied").as_str());
        } else {
            task_list_buffer.push_str(
                format!(
                    "Process Id: {}\nProcess Name: {}\nProcess User: {}\nProcess Info: {:?}\n-------------------------\n", 
                    task.get_pid(), task.get_pname(), task.get_user().unwrap().as_str(),
                    task.get_file_info()).as_str());
        }
    }

    task_list_buffer
}   