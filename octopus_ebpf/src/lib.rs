use aya::{
    maps::ring_buf::RingBuf,
    programs::{KProbe, TracePoint},
    Ebpf,
};
use aya_log::EbpfLogger;
use octopus_ebpf_common::KernelEvent;
use tokio::sync::mpsc;

pub struct SensorySystem {
    _handle: tokio::task::JoinHandle<()>,
}

impl SensorySystem {
    pub async fn init(tx: mpsc::Sender<KernelEvent>) -> anyhow::Result<Self> {
        // Attempt to load the BPF bytecode. 
        let mut bpf = match Ebpf::load(&[]) {
            Ok(b) => b,
            Err(e) => {
                return Err(anyhow::anyhow!("eBPF bytecode missing or invalid: {}", e));
            }
        };
        
        // Attach File Tentacle
        if let Some(program) = bpf.program_mut("octopus_file_open") {
            let tp: &mut TracePoint = program.try_into()?;
            tp.load()?;
            let _ = tp.attach("syscalls", "sys_enter_openat");
        }

        // Attach Network Tentacle
        if let Some(program) = bpf.program_mut("octopus_net_send") {
            let kp: &mut KProbe = program.try_into()?;
            kp.load()?;
            let _ = kp.attach("tcp_sendmsg", 0);
        }

        // Attach Process Tentacle
        if let Some(program) = bpf.program_mut("octopus_proc_exec") {
            let tp: &mut TracePoint = program.try_into()?;
            tp.load()?;
            let _ = tp.attach("sched", "sched_process_exec");
        }

        let handle = tokio::spawn(async move {
            let mut bpf = bpf;
            
            if let Err(e) = EbpfLogger::init(&mut bpf) {
                log::warn!("failed to initialize eBPF logger: {}", e);
            }

            if let Some(map) = bpf.map_mut("EVENTS") {
                if let Ok(mut ring_buf) = RingBuf::try_from(map) {
                    loop {
                        while let Some(item) = ring_buf.next() {
                            let event = unsafe { *(item.as_ptr() as *const KernelEvent) };
                            if tx.send(event).await.is_err() {
                                return;
                            }
                        }
                        tokio::task::yield_now().await;
                    }
                }
            }
            
            loop {
                tokio::task::yield_now().await;
            }
        });

        Ok(Self { _handle: handle })
    }
}
