#![no_std]
#![no_main]
use aya_bpf::{
    cty::{c_int, c_void},
    helpers::bpf_get_current_task_btf,
    macros::fentry,
    programs::FEntryContext,
};
use aya_log_ebpf::{error, info};

#[allow(improper_ctypes)]
extern "C" {
    // working with c_void here prevents having to define structures
    // in Rust, which might be seen as an uncessary (as we won't use
    // structs to access fields) burden.
    fn task_struct_pid(task: *const c_void) -> c_int;
    fn task_struct_tgid(task: *const c_void) -> c_int;
    fn task_struct_comm(task: *const c_void) -> *const u8;
    fn task_struct_start_time(task: *const c_void) -> u64;
    fn task_struct_start_boottime(task: *const c_void) -> u64;
    fn task_struct_cred_uid(task: *const c_void) -> i32;
    fn task_struct_cred(task: *const c_void) -> *const c_void;

    fn cred_uid(cred: *const c_void) -> i32;
}

#[fentry(name = "schedule")]
pub fn schedule(ctx: FEntryContext) -> i32 {
    match unsafe { try_schedule(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_schedule(ctx: FEntryContext) -> Result<i32, i32> {
    let task = bpf_get_current_task_btf() as *const c_void;
    let comm = core::slice::from_raw_parts(task_struct_comm(task), 16);

    info!(
        &ctx,
        "start_time={} start_boottime={} uid={} task_struct->pid={} task_struct->tgid={} comm={}",
        task_struct_start_time(task),
        task_struct_start_boottime(task),
        task_struct_cred_uid(task),
        task_struct_pid(task),
        task_struct_tgid(task),
        core::str::from_utf8_unchecked(comm),
    );

    // we have made a shim example where we access task uid directly from task_struct
    // or by first getting access to the cred *cred member. Those two values must
    // be equal.
    if task_struct_cred_uid(task) != cred_uid(task_struct_cred(task)) {
        error!(&ctx, "uids are not equal, this should not happen")
    }
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
