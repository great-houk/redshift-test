#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - no description available"]
    pub channel: [CHANNEL; 4],
    _reserved1: [u8; 0xb0],
    #[doc = "0xf0 - Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature."]
    pub modcfg: MODCFG,
    #[doc = "0xf4 - Idle channel register. This register returns the number of the first idle channel."]
    pub idle_ch: IDLE_CH,
    #[doc = "0xf8 - Global interrupt flag register"]
    pub irq_flag: IRQ_FLAG,
}
#[doc = "no description available"]
pub use self::channel::CHANNEL;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod channel;
#[doc = "MODCFG (rw) register accessor: an alias for `Reg<MODCFG_SPEC>`"]
pub type MODCFG = crate::Reg<modcfg::MODCFG_SPEC>;
#[doc = "Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature."]
pub mod modcfg;
#[doc = "IDLE_CH (r) register accessor: an alias for `Reg<IDLE_CH_SPEC>`"]
pub type IDLE_CH = crate::Reg<idle_ch::IDLE_CH_SPEC>;
#[doc = "Idle channel register. This register returns the number of the first idle channel."]
pub mod idle_ch;
#[doc = "IRQ_FLAG (rw) register accessor: an alias for `Reg<IRQ_FLAG_SPEC>`"]
pub type IRQ_FLAG = crate::Reg<irq_flag::IRQ_FLAG_SPEC>;
#[doc = "Global interrupt flag register"]
pub mod irq_flag;
