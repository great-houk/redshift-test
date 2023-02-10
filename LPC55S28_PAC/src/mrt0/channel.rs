#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - MRT Time interval value register. This value is loaded into the TIMER register."]
    pub intval: INTVAL,
    #[doc = "0x04 - MRT Timer register. This register reads the value of the down-counter."]
    pub timer: TIMER,
    #[doc = "0x08 - MRT Control register. This register controls the MRT modes."]
    pub ctrl: CTRL,
    #[doc = "0x0c - MRT Status register."]
    pub stat: STAT,
}
#[doc = "INTVAL (rw) register accessor: an alias for `Reg<INTVAL_SPEC>`"]
pub type INTVAL = crate::Reg<intval::INTVAL_SPEC>;
#[doc = "MRT Time interval value register. This value is loaded into the TIMER register."]
pub mod intval;
#[doc = "TIMER (r) register accessor: an alias for `Reg<TIMER_SPEC>`"]
pub type TIMER = crate::Reg<timer::TIMER_SPEC>;
#[doc = "MRT Timer register. This register reads the value of the down-counter."]
pub mod timer;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "MRT Control register. This register controls the MRT modes."]
pub mod ctrl;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "MRT Status register."]
pub mod stat;
