#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xd0],
    #[doc = "0xd0 - Security Attribution Unit Control Register"]
    pub ctrl: CTRL,
    #[doc = "0xd4 - Security Attribution Unit Type Register"]
    pub type_: TYPE,
    #[doc = "0xd8 - Security Attribution Unit Region Number Register"]
    pub rnr: RNR,
    #[doc = "0xdc - Security Attribution Unit Region Base Address Register"]
    pub rbar: RBAR,
    #[doc = "0xe0 - Security Attribution Unit Region Limit Address Register"]
    pub rlar: RLAR,
    #[doc = "0xe4 - Secure Fault Status Register"]
    pub sfsr: SFSR,
    #[doc = "0xe8 - Secure Fault Address Register"]
    pub sfar: SFAR,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Security Attribution Unit Control Register"]
pub mod ctrl;
#[doc = "TYPE (rw) register accessor: an alias for `Reg<TYPE_SPEC>`"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "Security Attribution Unit Type Register"]
pub mod type_;
#[doc = "RNR (rw) register accessor: an alias for `Reg<RNR_SPEC>`"]
pub type RNR = crate::Reg<rnr::RNR_SPEC>;
#[doc = "Security Attribution Unit Region Number Register"]
pub mod rnr;
#[doc = "RBAR (rw) register accessor: an alias for `Reg<RBAR_SPEC>`"]
pub type RBAR = crate::Reg<rbar::RBAR_SPEC>;
#[doc = "Security Attribution Unit Region Base Address Register"]
pub mod rbar;
#[doc = "RLAR (rw) register accessor: an alias for `Reg<RLAR_SPEC>`"]
pub type RLAR = crate::Reg<rlar::RLAR_SPEC>;
#[doc = "Security Attribution Unit Region Limit Address Register"]
pub mod rlar;
#[doc = "SFSR (rw) register accessor: an alias for `Reg<SFSR_SPEC>`"]
pub type SFSR = crate::Reg<sfsr::SFSR_SPEC>;
#[doc = "Secure Fault Status Register"]
pub mod sfsr;
#[doc = "SFAR (rw) register accessor: an alias for `Reg<SFAR_SPEC>`"]
pub type SFAR = crate::Reg<sfar::SFAR_SPEC>;
#[doc = "Secure Fault Address Register"]
pub mod sfar;
