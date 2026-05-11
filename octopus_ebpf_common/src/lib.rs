#![no_std]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct KernelEvent {
    pub event_type: u32,
    pub pid: u32,
    pub uid: u32,
    pub command: [u8; 16],
    pub payload: [u8; 64],
}

pub const EVENT_TYPE_FILE_OPEN: u32 = 1;
pub const EVENT_TYPE_NET_SEND: u32 = 2;
pub const EVENT_TYPE_PROC_EXEC: u32 = 3;

#[cfg(feature = "user")]
unsafe impl aya::Pod for KernelEvent {}
