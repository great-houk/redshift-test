#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0ff8],
    #[doc = "0xff8 - Peripheral Select and Flexcomm ID register."]
    pub pselid: PSELID,
    #[doc = "0xffc - Peripheral identification register."]
    pub pid: PID,
}
#[doc = "PSELID (rw) register accessor: an alias for `Reg<PSELID_SPEC>`"]
pub type PSELID = crate::Reg<pselid::PSELID_SPEC>;
#[doc = "Peripheral Select and Flexcomm ID register."]
pub mod pselid;
#[doc = "PID (r) register accessor: an alias for `Reg<PID_SPEC>`"]
pub type PID = crate::Reg<pid::PID_SPEC>;
#[doc = "Peripheral identification register."]
pub mod pid;
