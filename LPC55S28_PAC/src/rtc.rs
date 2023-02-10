#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - RTC match register"]
    pub match_: MATCH,
    #[doc = "0x08 - RTC counter register"]
    pub count: COUNT,
    #[doc = "0x0c - High-resolution/wake-up timer control register"]
    pub wake: WAKE,
    #[doc = "0x10 - Sub-second counter register"]
    pub subsec: SUBSEC,
    _reserved5: [u8; 0x2c],
    #[doc = "0x40..0x60 - General Purpose register"]
    pub gpreg: [GPREG; 8],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "RTC control register"]
pub mod ctrl;
#[doc = "MATCH (rw) register accessor: an alias for `Reg<MATCH_SPEC>`"]
pub type MATCH = crate::Reg<match_::MATCH_SPEC>;
#[doc = "RTC match register"]
pub mod match_;
#[doc = "COUNT (rw) register accessor: an alias for `Reg<COUNT_SPEC>`"]
pub type COUNT = crate::Reg<count::COUNT_SPEC>;
#[doc = "RTC counter register"]
pub mod count;
#[doc = "WAKE (rw) register accessor: an alias for `Reg<WAKE_SPEC>`"]
pub type WAKE = crate::Reg<wake::WAKE_SPEC>;
#[doc = "High-resolution/wake-up timer control register"]
pub mod wake;
#[doc = "SUBSEC (rw) register accessor: an alias for `Reg<SUBSEC_SPEC>`"]
pub type SUBSEC = crate::Reg<subsec::SUBSEC_SPEC>;
#[doc = "Sub-second counter register"]
pub mod subsec;
#[doc = "GPREG (rw) register accessor: an alias for `Reg<GPREG_SPEC>`"]
pub type GPREG = crate::Reg<gpreg::GPREG_SPEC>;
#[doc = "General Purpose register"]
pub mod gpreg;
