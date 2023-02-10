#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x40 - no description available"]
    pub b: [B; 2],
    _reserved1: [u8; 0x0fc0],
    #[doc = "0x1000..0x1100 - no description available"]
    pub w: [W; 2],
    _reserved2: [u8; 0x0f00],
    #[doc = "0x2000..0x2008 - Direction registers for all port GPIO pins"]
    pub dir: [DIR; 2],
    _reserved3: [u8; 0x78],
    #[doc = "0x2080..0x2088 - Mask register for all port GPIO pins"]
    pub mask: [MASK; 2],
    _reserved4: [u8; 0x78],
    #[doc = "0x2100..0x2108 - Port pin register for all port GPIO pins"]
    pub pin: [PIN; 2],
    _reserved5: [u8; 0x78],
    #[doc = "0x2180..0x2188 - Masked port register for all port GPIO pins"]
    pub mpin: [MPIN; 2],
    _reserved6: [u8; 0x78],
    #[doc = "0x2200..0x2208 - Write: Set register for port. Read: output bits for port"]
    pub set: [SET; 2],
    _reserved7: [u8; 0x78],
    #[doc = "0x2280..0x2288 - Clear port for all port GPIO pins"]
    pub clr: [CLR; 2],
    _reserved8: [u8; 0x78],
    #[doc = "0x2300..0x2308 - Toggle port for all port GPIO pins"]
    pub not: [NOT; 2],
    _reserved9: [u8; 0x78],
    #[doc = "0x2380..0x2388 - Set pin direction bits for port"]
    pub dirset: [DIRSET; 2],
    _reserved10: [u8; 0x78],
    #[doc = "0x2400..0x2408 - Clear pin direction bits for port"]
    pub dirclr: [DIRCLR; 2],
    _reserved11: [u8; 0x78],
    #[doc = "0x2480..0x2488 - Toggle pin direction bits for port"]
    pub dirnot: [DIRNOT; 2],
}
#[doc = "no description available"]
pub use self::b::B;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod b;
#[doc = "no description available"]
pub use self::w::W;
#[doc = r"Cluster"]
#[doc = "no description available"]
pub mod w;
#[doc = "DIR (rw) register accessor: an alias for `Reg<DIR_SPEC>`"]
pub type DIR = crate::Reg<dir::DIR_SPEC>;
#[doc = "Direction registers for all port GPIO pins"]
pub mod dir;
#[doc = "MASK (rw) register accessor: an alias for `Reg<MASK_SPEC>`"]
pub type MASK = crate::Reg<mask::MASK_SPEC>;
#[doc = "Mask register for all port GPIO pins"]
pub mod mask;
#[doc = "PIN (rw) register accessor: an alias for `Reg<PIN_SPEC>`"]
pub type PIN = crate::Reg<pin::PIN_SPEC>;
#[doc = "Port pin register for all port GPIO pins"]
pub mod pin;
#[doc = "MPIN (rw) register accessor: an alias for `Reg<MPIN_SPEC>`"]
pub type MPIN = crate::Reg<mpin::MPIN_SPEC>;
#[doc = "Masked port register for all port GPIO pins"]
pub mod mpin;
#[doc = "SET (rw) register accessor: an alias for `Reg<SET_SPEC>`"]
pub type SET = crate::Reg<set::SET_SPEC>;
#[doc = "Write: Set register for port. Read: output bits for port"]
pub mod set;
#[doc = "CLR (w) register accessor: an alias for `Reg<CLR_SPEC>`"]
pub type CLR = crate::Reg<clr::CLR_SPEC>;
#[doc = "Clear port for all port GPIO pins"]
pub mod clr;
#[doc = "NOT (w) register accessor: an alias for `Reg<NOT_SPEC>`"]
pub type NOT = crate::Reg<not::NOT_SPEC>;
#[doc = "Toggle port for all port GPIO pins"]
pub mod not;
#[doc = "DIRSET (w) register accessor: an alias for `Reg<DIRSET_SPEC>`"]
pub type DIRSET = crate::Reg<dirset::DIRSET_SPEC>;
#[doc = "Set pin direction bits for port"]
pub mod dirset;
#[doc = "DIRCLR (w) register accessor: an alias for `Reg<DIRCLR_SPEC>`"]
pub type DIRCLR = crate::Reg<dirclr::DIRCLR_SPEC>;
#[doc = "Clear pin direction bits for port"]
pub mod dirclr;
#[doc = "DIRNOT (w) register accessor: an alias for `Reg<DIRNOT_SPEC>`"]
pub type DIRNOT = crate::Reg<dirnot::DIRNOT_SPEC>;
#[doc = "Toggle pin direction bits for port"]
pub mod dirnot;
