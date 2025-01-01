//! App management syscalls
use crate::batch::run_next_app;
use crate::recevent::*;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    write_event(24);//os/src/syscall/process.rs->process::sys_exit
    run_next_app()
}
