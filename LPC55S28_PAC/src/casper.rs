#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Contains the offsets of AB and CD in the RAM."]
    pub ctrl0: CTRL0,
    #[doc = "0x04 - Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
    pub ctrl1: CTRL1,
    #[doc = "0x08 - Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations."]
    pub loader: LOADER,
    #[doc = "0x0c - Indicates operational status and would contain the carry bit if used."]
    pub status: STATUS,
    #[doc = "0x10 - Sets interrupts"]
    pub intenset: INTENSET,
    #[doc = "0x14 - Clears interrupts"]
    pub intenclr: INTENCLR,
    #[doc = "0x18 - Interrupt status bits (mask of INTENSET and STATUS)"]
    pub intstat: INTSTAT,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - A register"]
    pub areg: AREG,
    #[doc = "0x24 - B register"]
    pub breg: BREG,
    #[doc = "0x28 - C register"]
    pub creg: CREG,
    #[doc = "0x2c - D register"]
    pub dreg: DREG,
    #[doc = "0x30 - Result register 0"]
    pub res0: RES0,
    #[doc = "0x34 - Result register 1"]
    pub res1: RES1,
    #[doc = "0x38 - Result register 2"]
    pub res2: RES2,
    #[doc = "0x3c - Result register 3"]
    pub res3: RES3,
    _reserved15: [u8; 0x20],
    #[doc = "0x60 - Optional mask register"]
    pub mask: MASK,
    #[doc = "0x64 - Optional re-mask register"]
    pub remask: REMASK,
    _reserved17: [u8; 0x18],
    #[doc = "0x80 - Security lock register"]
    pub lock: LOCK,
}
#[doc = "CTRL0 (rw) register accessor: an alias for `Reg<CTRL0_SPEC>`"]
pub type CTRL0 = crate::Reg<ctrl0::CTRL0_SPEC>;
#[doc = "Contains the offsets of AB and CD in the RAM."]
pub mod ctrl0;
#[doc = "CTRL1 (rw) register accessor: an alias for `Reg<CTRL1_SPEC>`"]
pub type CTRL1 = crate::Reg<ctrl1::CTRL1_SPEC>;
#[doc = "Contains the opcode mode, iteration count, and result offset (in RAM) and also launches the accelerator. Note: with CP version: CTRL0 and CRTL1 can be written in one go with MCRR."]
pub mod ctrl1;
#[doc = "LOADER (rw) register accessor: an alias for `Reg<LOADER_SPEC>`"]
pub type LOADER = crate::Reg<loader::LOADER_SPEC>;
#[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations."]
pub mod loader;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Indicates operational status and would contain the carry bit if used."]
pub mod status;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Sets interrupts"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Clears interrupts"]
pub mod intenclr;
#[doc = "INTSTAT (rw) register accessor: an alias for `Reg<INTSTAT_SPEC>`"]
pub type INTSTAT = crate::Reg<intstat::INTSTAT_SPEC>;
#[doc = "Interrupt status bits (mask of INTENSET and STATUS)"]
pub mod intstat;
#[doc = "AREG (rw) register accessor: an alias for `Reg<AREG_SPEC>`"]
pub type AREG = crate::Reg<areg::AREG_SPEC>;
#[doc = "A register"]
pub mod areg;
#[doc = "BREG (rw) register accessor: an alias for `Reg<BREG_SPEC>`"]
pub type BREG = crate::Reg<breg::BREG_SPEC>;
#[doc = "B register"]
pub mod breg;
#[doc = "CREG (rw) register accessor: an alias for `Reg<CREG_SPEC>`"]
pub type CREG = crate::Reg<creg::CREG_SPEC>;
#[doc = "C register"]
pub mod creg;
#[doc = "DREG (rw) register accessor: an alias for `Reg<DREG_SPEC>`"]
pub type DREG = crate::Reg<dreg::DREG_SPEC>;
#[doc = "D register"]
pub mod dreg;
#[doc = "RES0 (rw) register accessor: an alias for `Reg<RES0_SPEC>`"]
pub type RES0 = crate::Reg<res0::RES0_SPEC>;
#[doc = "Result register 0"]
pub mod res0;
#[doc = "RES1 (rw) register accessor: an alias for `Reg<RES1_SPEC>`"]
pub type RES1 = crate::Reg<res1::RES1_SPEC>;
#[doc = "Result register 1"]
pub mod res1;
#[doc = "RES2 (rw) register accessor: an alias for `Reg<RES2_SPEC>`"]
pub type RES2 = crate::Reg<res2::RES2_SPEC>;
#[doc = "Result register 2"]
pub mod res2;
#[doc = "RES3 (rw) register accessor: an alias for `Reg<RES3_SPEC>`"]
pub type RES3 = crate::Reg<res3::RES3_SPEC>;
#[doc = "Result register 3"]
pub mod res3;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Optional mask register"]
pub mod mask;
#[doc = "REMASK (rw) register accessor: an alias for `Reg<REMASK_SPEC>`"]
pub type REMASK = crate::Reg<remask::REMASK_SPEC>;
#[doc = "Optional re-mask register"]
pub mod remask;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Security lock register"]
pub mod lock;
