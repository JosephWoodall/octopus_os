#![no_std]
#![no_main]

use aya_ebpf::{
    macros::{kprobe, tracepoint},
    programs::{ProbeContext, TracePointContext},
    helpers::gen::bpf_get_current_pid_tgid,
    maps::RingBuf,
};
use octopus_ebpf_common::{
    KernelEvent, EVENT_TYPE_FILE_OPEN, EVENT_TYPE_NET_SEND, EVENT_TYPE_PROC_EXEC,
};

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

#[allow(non_upper_case_globals)]
#[no_mangle]
static mut EVENTS: RingBuf = RingBuf::with_max_entries(4096, 0);

#[tracepoint]
pub fn octopus_file_open(ctx: TracePointContext) -> u32 {
    let pid_tgid = bpf_get_current_pid_tgid();
    let pid = (pid_tgid >> 32) as u32;
    
    if let Some(mut event) = unsafe { EVENTS.reserve::<KernelEvent>(0) } {
        event.event_type = EVENT_TYPE_FILE_OPEN;
        event.pid = pid;
        event.submit(0);
    }

    0
}

#[kprobe]
pub fn octopus_net_send(ctx: ProbeContext) -> u32 {
    let pid_tgid = bpf_get_current_pid_tgid();
    let pid = (pid_tgid >> 32) as u32;

    if let Some(mut event) = unsafe { EVENTS.reserve::<KernelEvent>(0) } {
        event.event_type = EVENT_TYPE_NET_SEND;
        event.pid = pid;
        event.submit(0);
    }
    0
}

#[tracepoint]
pub fn octopus_proc_exec(ctx: TracePointContext) -> u32 {
    let pid_tgid = bpf_get_current_pid_tgid();
    let pid = (pid_tgid >> 32) as u32;

    if let Some(mut event) = unsafe { EVENTS.reserve::<KernelEvent>(0) } {
        event.event_type = EVENT_TYPE_PROC_EXEC;
        event.pid = pid;
        event.submit(0);
    }
    0
}
