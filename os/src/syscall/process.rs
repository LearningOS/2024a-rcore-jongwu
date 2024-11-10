//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, get_current_task},
    timer::{get_time_us, get_time},
};
use crate::task::TaskStatus::Ready;

/// time
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// 
    pub sec: usize,
    ///
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
    /// Last timestamp
    pub last: usize,
    /// if it's the first time to run
    pub is_first_run: bool,
    /// current num
    pub current: usize,
}

impl TaskInfo {
    /// init task info
    pub fn init() -> TaskInfo {
        Self {
            status: Ready,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
            last: 0,
            is_first_run: true,
            current: 0,
        }
    }
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    if let Some(task) = get_current_task() {
        unsafe {
            (*ti).status = task.status;
            (*ti).syscall_times = task.syscall_times;
            let _now = get_time();
            (*ti).time = (task.time + get_time_us() - task.last) / 1000;
//            (*ti).time = task.time / 1000;
            println!("======== task current num = {}\n", task.current);
        }
        return 0;
    } else {
        return -1;
    }
}
