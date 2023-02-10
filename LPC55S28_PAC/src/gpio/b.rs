#[doc = r"Register block"]
#[repr(C)]
pub struct B {
    #[doc = "0x00..0x20 - Byte pin registers for all port GPIO pins"]
    pub b_: [B_; 32],
}
#[doc = "B_ (rw) register accessor: an alias for `Reg<B__SPEC>`"]
pub type B_ = crate::Reg<b_::B__SPEC>;
#[doc = "Byte pin registers for all port GPIO pins"]
pub mod b_;
