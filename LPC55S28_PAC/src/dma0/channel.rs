#[doc = r"Register block"]
#[repr(C)]
pub struct CHANNEL {
    #[doc = "0x00 - Configuration register for DMA channel ."]
    pub cfg: CFG,
    #[doc = "0x04 - Control and status register for DMA channel ."]
    pub ctlstat: CTLSTAT,
    #[doc = "0x08 - Transfer configuration register for DMA channel ."]
    pub xfercfg: XFERCFG,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration register for DMA channel ."]
pub mod cfg;
#[doc = "CTLSTAT (r) register accessor: an alias for `Reg<CTLSTAT_SPEC>`"]
pub type CTLSTAT = crate::Reg<ctlstat::CTLSTAT_SPEC>;
#[doc = "Control and status register for DMA channel ."]
pub mod ctlstat;
#[doc = "XFERCFG (rw) register accessor: an alias for `Reg<XFERCFG_SPEC>`"]
pub type XFERCFG = crate::Reg<xfercfg::XFERCFG_SPEC>;
#[doc = "Transfer configuration register for DMA channel ."]
pub mod xfercfg;
