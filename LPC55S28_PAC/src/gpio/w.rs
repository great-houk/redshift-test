#[doc = r"Register block"]
#[repr(C)]
pub struct W {
    #[doc = "0x00..0x80 - Word pin registers for all port GPIO pins"]
    pub w_: [W_; 32],
}
#[doc = "W_ (rw) register accessor: an alias for `Reg<W__SPEC>`"]
pub type W_ = crate::Reg<w_::W__SPEC>;
#[doc = "Word pin registers for all port GPIO pins"]
pub mod w_;
