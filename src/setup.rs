use rtt_target::{self as rtt_t, rprintln, rtt_init_default};
use LPC55S28_PAC::SYSCON;

#[cortex_m_rt::entry]
fn entry() -> ! {
    let rtt = rtt_init_default!();
    rtt_t::set_print_channel(rtt.up.0);
    rprintln!("Start up finished!");
    crate::run();
}

pub fn setup_main_clock_96mhz(syscon: &SYSCON) {
    let a = syscon.mainclksela.read().sel().variant();
    rprintln!("{:?}", a);
    syscon.mainclksela.write(|w| w.sel().enum_0x3());
}
