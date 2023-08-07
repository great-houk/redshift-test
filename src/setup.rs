use rtt_target::{self as rtt_t, rprintln, rtt_init_default};

#[cortex_m_rt::entry]
fn entry() -> ! {
    // Saftey Delay, so it's possible to halt before user code runs
    cortex_m::asm::delay(10_000_000);

    let rtt = rtt_init_default!();
    rtt_t::set_print_channel(rtt.up.0);
    rprintln!("Start up finished!");
    crate::run();
}
