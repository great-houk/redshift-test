#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Interrupt Mode register"]
    pub isel: ISEL,
    #[doc = "0x04 - Pin interrupt level or rising edge interrupt enable register"]
    pub ienr: IENR,
    #[doc = "0x08 - Pin interrupt level or rising edge interrupt set register"]
    pub sienr: SIENR,
    #[doc = "0x0c - Pin interrupt level (rising edge interrupt) clear register"]
    pub cienr: CIENR,
    #[doc = "0x10 - Pin interrupt active level or falling edge interrupt enable register"]
    pub ienf: IENF,
    #[doc = "0x14 - Pin interrupt active level or falling edge interrupt set register"]
    pub sienf: SIENF,
    #[doc = "0x18 - Pin interrupt active level or falling edge interrupt clear register"]
    pub cienf: CIENF,
    #[doc = "0x1c - Pin interrupt rising edge register"]
    pub rise: RISE,
    #[doc = "0x20 - Pin interrupt falling edge register"]
    pub fall: FALL,
    #[doc = "0x24 - Pin interrupt status register"]
    pub ist: IST,
    #[doc = "0x28 - Pattern match interrupt control register"]
    pub pmctrl: PMCTRL,
    #[doc = "0x2c - Pattern match interrupt bit-slice source register"]
    pub pmsrc: PMSRC,
    #[doc = "0x30 - Pattern match interrupt bit slice configuration register"]
    pub pmcfg: PMCFG,
}
#[doc = "ISEL (rw) register accessor: an alias for `Reg<ISEL_SPEC>`"]
pub type ISEL = crate::Reg<isel::ISEL_SPEC>;
#[doc = "Pin Interrupt Mode register"]
pub mod isel;
#[doc = "IENR (rw) register accessor: an alias for `Reg<IENR_SPEC>`"]
pub type IENR = crate::Reg<ienr::IENR_SPEC>;
#[doc = "Pin interrupt level or rising edge interrupt enable register"]
pub mod ienr;
#[doc = "SIENR (w) register accessor: an alias for `Reg<SIENR_SPEC>`"]
pub type SIENR = crate::Reg<sienr::SIENR_SPEC>;
#[doc = "Pin interrupt level or rising edge interrupt set register"]
pub mod sienr;
#[doc = "CIENR (w) register accessor: an alias for `Reg<CIENR_SPEC>`"]
pub type CIENR = crate::Reg<cienr::CIENR_SPEC>;
#[doc = "Pin interrupt level (rising edge interrupt) clear register"]
pub mod cienr;
#[doc = "IENF (rw) register accessor: an alias for `Reg<IENF_SPEC>`"]
pub type IENF = crate::Reg<ienf::IENF_SPEC>;
#[doc = "Pin interrupt active level or falling edge interrupt enable register"]
pub mod ienf;
#[doc = "SIENF (w) register accessor: an alias for `Reg<SIENF_SPEC>`"]
pub type SIENF = crate::Reg<sienf::SIENF_SPEC>;
#[doc = "Pin interrupt active level or falling edge interrupt set register"]
pub mod sienf;
#[doc = "CIENF (w) register accessor: an alias for `Reg<CIENF_SPEC>`"]
pub type CIENF = crate::Reg<cienf::CIENF_SPEC>;
#[doc = "Pin interrupt active level or falling edge interrupt clear register"]
pub mod cienf;
#[doc = "RISE (rw) register accessor: an alias for `Reg<RISE_SPEC>`"]
pub type RISE = crate::Reg<rise::RISE_SPEC>;
#[doc = "Pin interrupt rising edge register"]
pub mod rise;
#[doc = "FALL (rw) register accessor: an alias for `Reg<FALL_SPEC>`"]
pub type FALL = crate::Reg<fall::FALL_SPEC>;
#[doc = "Pin interrupt falling edge register"]
pub mod fall;
#[doc = "IST (rw) register accessor: an alias for `Reg<IST_SPEC>`"]
pub type IST = crate::Reg<ist::IST_SPEC>;
#[doc = "Pin interrupt status register"]
pub mod ist;
#[doc = "PMCTRL (rw) register accessor: an alias for `Reg<PMCTRL_SPEC>`"]
pub type PMCTRL = crate::Reg<pmctrl::PMCTRL_SPEC>;
#[doc = "Pattern match interrupt control register"]
pub mod pmctrl;
#[doc = "PMSRC (rw) register accessor: an alias for `Reg<PMSRC_SPEC>`"]
pub type PMSRC = crate::Reg<pmsrc::PMSRC_SPEC>;
#[doc = "Pattern match interrupt bit-slice source register"]
pub mod pmsrc;
#[doc = "PMCFG (rw) register accessor: an alias for `Reg<PMCFG_SPEC>`"]
pub type PMCFG = crate::Reg<pmcfg::PMCFG_SPEC>;
#[doc = "Pattern match interrupt bit slice configuration register"]
pub mod pmcfg;
