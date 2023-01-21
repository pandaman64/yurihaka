#![no_std]
#![no_main]

use aya_bpf::{
    macros::tracepoint,
    programs::TracePointContext,
};
use aya_log_ebpf::info;

#[tracepoint(name="yurihaka")]
pub fn yurihaka(ctx: TracePointContext) -> u32 {
    match try_yurihaka(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_yurihaka(ctx: TracePointContext) -> Result<u32, u32> {
    info!(&ctx, "tracepoint sched_process_exec called");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
