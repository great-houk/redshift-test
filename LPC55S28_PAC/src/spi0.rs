#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0400],
    #[doc = "0x400 - SPI Configuration register"]
    pub cfg: CFG,
    #[doc = "0x404 - SPI Delay register"]
    pub dly: DLY,
    #[doc = "0x408 - SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
    pub stat: STAT,
    #[doc = "0x40c - SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
    pub intenset: INTENSET,
    #[doc = "0x410 - SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 0x10],
    #[doc = "0x424 - SPI clock Divider"]
    pub div: DIV,
    #[doc = "0x428 - SPI Interrupt Status"]
    pub intstat: INTSTAT,
    _reserved7: [u8; 0x09d4],
    #[doc = "0xe00 - FIFO configuration and enable register."]
    pub fifocfg: FIFOCFG,
    #[doc = "0xe04 - FIFO status register."]
    pub fifostat: FIFOSTAT,
    #[doc = "0xe08 - FIFO trigger settings for interrupt and DMA request."]
    pub fifotrig: FIFOTRIG,
    _reserved10: [u8; 0x04],
    #[doc = "0xe10 - FIFO interrupt enable set (enable) and read register."]
    pub fifointenset: FIFOINTENSET,
    #[doc = "0xe14 - FIFO interrupt enable clear (disable) and read register."]
    pub fifointenclr: FIFOINTENCLR,
    #[doc = "0xe18 - FIFO interrupt status register."]
    pub fifointstat: FIFOINTSTAT,
    _reserved13: [u8; 0x04],
    #[doc = "0xe20 - FIFO write data."]
    pub fifowr: FIFOWR,
    _reserved14: [u8; 0x0c],
    #[doc = "0xe30 - FIFO read data."]
    pub fiford: FIFORD,
    _reserved15: [u8; 0x0c],
    #[doc = "0xe40 - FIFO data read with no FIFO pop."]
    pub fifordnopop: FIFORDNOPOP,
    _reserved16: [u8; 0x01b8],
    #[doc = "0xffc - Peripheral identification register."]
    pub id: ID,
}
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "SPI Configuration register"]
pub mod cfg;
#[doc = "DLY (rw) register accessor: an alias for `Reg<DLY_SPEC>`"]
pub type DLY = crate::Reg<dly::DLY_SPEC>;
#[doc = "SPI Delay register"]
pub mod dly;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "SPI Status. Some status flags can be cleared by writing a 1 to that bit position."]
pub mod stat;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "SPI Interrupt Enable read and Set. A complete value may be read from this register. Writing a 1 to any implemented bit position causes that bit to be set."]
pub mod intenset;
#[doc = "INTENCLR (w) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "SPI Interrupt Enable Clear. Writing a 1 to any implemented bit position causes the corresponding bit in INTENSET to be cleared."]
pub mod intenclr;
#[doc = "DIV (rw) register accessor: an alias for `Reg<DIV_SPEC>`"]
pub type DIV = crate::Reg<div::DIV_SPEC>;
#[doc = "SPI clock Divider"]
pub mod div;
#[doc = "INTSTAT (r) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "SPI Interrupt Status"]
pub mod intstat;
#[doc = "FIFOCFG (rw) register accessor: an alias for `Reg<FIFOCFG_SPEC>`"]
pub type FIFOCFG = crate::Reg<fifocfg::FIFOCFG_SPEC>;
#[doc = "FIFO configuration and enable register."]
pub mod fifocfg;
#[doc = "FIFOSTAT (rw) register accessor: an alias for `Reg<FIFOSTAT_SPEC>`"]
pub type FIFOSTAT = crate::Reg<fifostat::FIFOSTAT_SPEC>;
#[doc = "FIFO status register."]
pub mod fifostat;
#[doc = "FIFOTRIG (rw) register accessor: an alias for `Reg<FIFOTRIG_SPEC>`"]
pub type FIFOTRIG = crate::Reg<fifotrig::FIFOTRIG_SPEC>;
#[doc = "FIFO trigger settings for interrupt and DMA request."]
pub mod fifotrig;
#[doc = "FIFOINTENSET (rw) register accessor: an alias for `Reg<FIFOINTENSET_SPEC>`"]
pub type FIFOINTENSET = crate::Reg<fifointenset::FIFOINTENSET_SPEC>;
#[doc = "FIFO interrupt enable set (enable) and read register."]
pub mod fifointenset;
#[doc = "FIFOINTENCLR (rw) register accessor: an alias for `Reg<FIFOINTENCLR_SPEC>`"]
pub type FIFOINTENCLR = crate::Reg<fifointenclr::FIFOINTENCLR_SPEC>;
#[doc = "FIFO interrupt enable clear (disable) and read register."]
pub mod fifointenclr;
#[doc = "FIFOINTSTAT (r) register accessor: an alias for `Reg<FIFOINTSTAT_SPEC>`"]
pub type FIFOINTSTAT = crate::Reg<fifointstat::FIFOINTSTAT_SPEC>;
#[doc = "FIFO interrupt status register."]
pub mod fifointstat;
#[doc = "FIFOWR (rw) register accessor: an alias for `Reg<FIFOWR_SPEC>`"]
pub type FIFOWR = crate::Reg<fifowr::FIFOWR_SPEC>;
#[doc = "FIFO write data."]
pub mod fifowr;
#[doc = "FIFORD (r) register accessor: an alias for `Reg<FIFORD_SPEC>`"]
pub type FIFORD = crate::Reg<fiford::FIFORD_SPEC>;
#[doc = "FIFO read data."]
pub mod fiford;
#[doc = "FIFORDNOPOP (r) register accessor: an alias for `Reg<FIFORDNOPOP_SPEC>`"]
pub type FIFORDNOPOP = crate::Reg<fifordnopop::FIFORDNOPOP_SPEC>;
#[doc = "FIFO data read with no FIFO pop."]
pub mod fifordnopop;
#[doc = "ID (r) register accessor: an alias for `Reg<ID_SPEC>`"]
pub type ID = crate::Reg<id::ID_SPEC>;
#[doc = "Peripheral identification register."]
pub mod id;
