#[doc = r"Register block"]
#[repr(C)]
pub struct OUT {
    #[doc = "0x00 - SCT output 0 set register"]
    pub out_set: OUT_SET,
    #[doc = "0x04 - SCT output 0 clear register"]
    pub out_clr: OUT_CLR,
}
#[doc = "OUT_SET (rw) register accessor: an alias for `Reg<OUT_SET_SPEC>`"]
pub type OUT_SET = crate::Reg<out_set::OUT_SET_SPEC>;
#[doc = "SCT output 0 set register"]
pub mod out_set;
#[doc = "OUT_CLR (rw) register accessor: an alias for `Reg<OUT_CLR_SPEC>`"]
pub type OUT_CLR = crate::Reg<out_clr::OUT_CLR_SPEC>;
#[doc = "SCT output 0 clear register"]
pub mod out_clr;
