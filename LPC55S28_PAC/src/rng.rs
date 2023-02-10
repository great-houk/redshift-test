#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register contains a random 32 bit number which is computed on demand, at each time it is read"]
    pub random_number: RANDOM_NUMBER,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - no description available"]
    pub counter_val: COUNTER_VAL,
    #[doc = "0x0c - no description available"]
    pub counter_cfg: COUNTER_CFG,
    #[doc = "0x10 - no description available"]
    pub online_test_cfg: ONLINE_TEST_CFG,
    #[doc = "0x14 - no description available"]
    pub online_test_val: ONLINE_TEST_VAL,
    _reserved5: [u8; 0x0fe4],
    #[doc = "0xffc - IP identifier"]
    pub moduleid: MODULEID,
}
#[doc = "RANDOM_NUMBER (r) register accessor: an alias for `Reg<RANDOM_NUMBER_SPEC>`"]
pub type RANDOM_NUMBER = crate::Reg<random_number::RANDOM_NUMBER_SPEC>;
#[doc = "This register contains a random 32 bit number which is computed on demand, at each time it is read"]
pub mod random_number;
#[doc = "COUNTER_VAL (rw) register accessor: an alias for `Reg<COUNTER_VAL_SPEC>`"]
pub type COUNTER_VAL = crate::Reg<counter_val::COUNTER_VAL_SPEC>;
#[doc = "no description available"]
pub mod counter_val;
#[doc = "COUNTER_CFG (rw) register accessor: an alias for `Reg<COUNTER_CFG_SPEC>`"]
pub type COUNTER_CFG = crate::Reg<counter_cfg::COUNTER_CFG_SPEC>;
#[doc = "no description available"]
pub mod counter_cfg;
#[doc = "ONLINE_TEST_CFG (rw) register accessor: an alias for `Reg<ONLINE_TEST_CFG_SPEC>`"]
pub type ONLINE_TEST_CFG = crate::Reg<online_test_cfg::ONLINE_TEST_CFG_SPEC>;
#[doc = "no description available"]
pub mod online_test_cfg;
#[doc = "ONLINE_TEST_VAL (rw) register accessor: an alias for `Reg<ONLINE_TEST_VAL_SPEC>`"]
pub type ONLINE_TEST_VAL = crate::Reg<online_test_val::ONLINE_TEST_VAL_SPEC>;
#[doc = "no description available"]
pub mod online_test_val;
#[doc = "MODULEID (r) register accessor: an alias for `Reg<MODULEID_SPEC>`"]
pub type MODULEID = crate::Reg<moduleid::MODULEID_SPEC>;
#[doc = "IP identifier"]
pub mod moduleid;
