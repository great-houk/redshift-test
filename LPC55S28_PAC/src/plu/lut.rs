#[doc = r"Register block"]
#[repr(C)]
pub struct LUT {
    #[doc = "0x00..0x14 - LUTn input x MUX"]
    pub lut_inp_mux: [LUT_INP_MUX; 5],
}
#[doc = "LUT_INP_MUX (rw) register accessor: an alias for `Reg<LUT_INP_MUX_SPEC>`"]
pub type LUT_INP_MUX = crate::Reg<lut_inp_mux::LUT_INP_MUX_SPEC>;
#[doc = "LUTn input x MUX"]
pub mod lut_inp_mux;
