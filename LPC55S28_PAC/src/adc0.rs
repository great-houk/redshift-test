#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - ADC Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x14 - ADC Status Register"]
    pub stat: STAT,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ie: IE,
    #[doc = "0x1c - DMA Enable Register"]
    pub de: DE,
    #[doc = "0x20 - ADC Configuration Register"]
    pub cfg: CFG,
    #[doc = "0x24 - ADC Pause Register"]
    pub pause: PAUSE,
    _reserved8: [u8; 0x0c],
    #[doc = "0x34 - Software Trigger Register"]
    pub swtrig: SWTRIG,
    #[doc = "0x38 - Trigger Status Register"]
    pub tstat: TSTAT,
    _reserved10: [u8; 0x04],
    #[doc = "0x40 - ADC Offset Trim Register"]
    pub ofstrim: OFSTRIM,
    _reserved11: [u8; 0x5c],
    #[doc = "0xa0..0xe0 - Trigger Control Register"]
    pub tctrl: [TCTRL; 16],
    #[doc = "0xe0..0xe8 - FIFO Control Register"]
    pub fctrl: [FCTRL; 2],
    _reserved13: [u8; 0x08],
    #[doc = "0xf0..0xf8 - Gain Calibration Control"]
    pub gcc: [GCC; 2],
    #[doc = "0xf8..0x100 - Gain Calculation Result"]
    pub gcr: [GCR; 2],
    #[doc = "0x100 - ADC Command Low Buffer Register"]
    pub cmdl1: CMDL1,
    #[doc = "0x104 - ADC Command High Buffer Register"]
    pub cmdh1: CMDH1,
    #[doc = "0x108 - ADC Command Low Buffer Register"]
    pub cmdl2: CMDL2,
    #[doc = "0x10c - ADC Command High Buffer Register"]
    pub cmdh2: CMDH2,
    #[doc = "0x110 - ADC Command Low Buffer Register"]
    pub cmdl3: CMDL3,
    #[doc = "0x114 - ADC Command High Buffer Register"]
    pub cmdh3: CMDH3,
    #[doc = "0x118 - ADC Command Low Buffer Register"]
    pub cmdl4: CMDL4,
    #[doc = "0x11c - ADC Command High Buffer Register"]
    pub cmdh4: CMDH4,
    #[doc = "0x120 - ADC Command Low Buffer Register"]
    pub cmdl5: CMDL5,
    #[doc = "0x124 - ADC Command High Buffer Register"]
    pub cmdh5: CMDH5,
    #[doc = "0x128 - ADC Command Low Buffer Register"]
    pub cmdl6: CMDL6,
    #[doc = "0x12c - ADC Command High Buffer Register"]
    pub cmdh6: CMDH6,
    #[doc = "0x130 - ADC Command Low Buffer Register"]
    pub cmdl7: CMDL7,
    #[doc = "0x134 - ADC Command High Buffer Register"]
    pub cmdh7: CMDH7,
    #[doc = "0x138 - ADC Command Low Buffer Register"]
    pub cmdl8: CMDL8,
    #[doc = "0x13c - ADC Command High Buffer Register"]
    pub cmdh8: CMDH8,
    #[doc = "0x140 - ADC Command Low Buffer Register"]
    pub cmdl9: CMDL9,
    #[doc = "0x144 - ADC Command High Buffer Register"]
    pub cmdh9: CMDH9,
    #[doc = "0x148 - ADC Command Low Buffer Register"]
    pub cmdl10: CMDL10,
    #[doc = "0x14c - ADC Command High Buffer Register"]
    pub cmdh10: CMDH10,
    #[doc = "0x150 - ADC Command Low Buffer Register"]
    pub cmdl11: CMDL11,
    #[doc = "0x154 - ADC Command High Buffer Register"]
    pub cmdh11: CMDH11,
    #[doc = "0x158 - ADC Command Low Buffer Register"]
    pub cmdl12: CMDL12,
    #[doc = "0x15c - ADC Command High Buffer Register"]
    pub cmdh12: CMDH12,
    #[doc = "0x160 - ADC Command Low Buffer Register"]
    pub cmdl13: CMDL13,
    #[doc = "0x164 - ADC Command High Buffer Register"]
    pub cmdh13: CMDH13,
    #[doc = "0x168 - ADC Command Low Buffer Register"]
    pub cmdl14: CMDL14,
    #[doc = "0x16c - ADC Command High Buffer Register"]
    pub cmdh14: CMDH14,
    #[doc = "0x170 - ADC Command Low Buffer Register"]
    pub cmdl15: CMDL15,
    #[doc = "0x174 - ADC Command High Buffer Register"]
    pub cmdh15: CMDH15,
    _reserved45: [u8; 0x88],
    #[doc = "0x200..0x210 - Compare Value Register"]
    pub cv: [CV; 4],
    _reserved46: [u8; 0xf0],
    #[doc = "0x300..0x308 - ADC Data Result FIFO Register"]
    pub resfifo: [RESFIFO; 2],
    _reserved47: [u8; 0xf8],
    #[doc = "0x400..0x484 - Calibration General A-Side Registers"]
    pub cal_gar: [CAL_GAR; 33],
    _reserved48: [u8; 0x7c],
    #[doc = "0x500..0x584 - Calibration General B-Side Registers"]
    pub cal_gbr: [CAL_GBR; 33],
    _reserved49: [u8; 0x0a78],
    #[doc = "0xffc - ADC Test Register"]
    pub tst: TST,
}
impl RegisterBlock {
    #[doc = "0x200 - Compare Value Register"]
    #[inline(always)]
    pub fn cv1(&self) -> &CV {
        &self.cv[0]
    }
    #[doc = "0x204 - Compare Value Register"]
    #[inline(always)]
    pub fn cv2(&self) -> &CV {
        &self.cv[1]
    }
    #[doc = "0x208 - Compare Value Register"]
    #[inline(always)]
    pub fn cv3(&self) -> &CV {
        &self.cv[2]
    }
    #[doc = "0x20c - Compare Value Register"]
    #[inline(always)]
    pub fn cv4(&self) -> &CV {
        &self.cv[3]
    }
}
#[doc = "VERID (r) register accessor: an alias for `Reg<VERID_SPEC>`"]
pub type VERID = crate::Reg<verid::VERID_SPEC>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM (r) register accessor: an alias for `Reg<PARAM_SPEC>`"]
pub type PARAM = crate::Reg<param::PARAM_SPEC>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "ADC Control Register"]
pub mod ctrl;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "ADC Status Register"]
pub mod stat;
#[doc = "IE (rw) register accessor: an alias for `Reg<IE_SPEC>`"]
pub type IE = crate::Reg<ie::IE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ie;
#[doc = "DE (rw) register accessor: an alias for `Reg<DE_SPEC>`"]
pub type DE = crate::Reg<de::DE_SPEC>;
#[doc = "DMA Enable Register"]
pub mod de;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "ADC Configuration Register"]
pub mod cfg;
#[doc = "PAUSE (rw) register accessor: an alias for `Reg<PAUSE_SPEC>`"]
pub type PAUSE = crate::Reg<pause::PAUSE_SPEC>;
#[doc = "ADC Pause Register"]
pub mod pause;
#[doc = "SWTRIG (rw) register accessor: an alias for `Reg<SWTRIG_SPEC>`"]
pub type SWTRIG = crate::Reg<swtrig::SWTRIG_SPEC>;
#[doc = "Software Trigger Register"]
pub mod swtrig;
#[doc = "TSTAT (rw) register accessor: an alias for `Reg<TSTAT_SPEC>`"]
pub type TSTAT = crate::Reg<tstat::TSTAT_SPEC>;
#[doc = "Trigger Status Register"]
pub mod tstat;
#[doc = "OFSTRIM (rw) register accessor: an alias for `Reg<OFSTRIM_SPEC>`"]
pub type OFSTRIM = crate::Reg<ofstrim::OFSTRIM_SPEC>;
#[doc = "ADC Offset Trim Register"]
pub mod ofstrim;
#[doc = "TCTRL (rw) register accessor: an alias for `Reg<TCTRL_SPEC>`"]
pub type TCTRL = crate::Reg<tctrl::TCTRL_SPEC>;
#[doc = "Trigger Control Register"]
pub mod tctrl;
#[doc = "FCTRL (rw) register accessor: an alias for `Reg<FCTRL_SPEC>`"]
pub type FCTRL = crate::Reg<fctrl::FCTRL_SPEC>;
#[doc = "FIFO Control Register"]
pub mod fctrl;
#[doc = "GCC (r) register accessor: an alias for `Reg<GCC_SPEC>`"]
pub type GCC = crate::Reg<gcc::GCC_SPEC>;
#[doc = "Gain Calibration Control"]
pub mod gcc;
#[doc = "GCR (rw) register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Gain Calculation Result"]
pub mod gcr;
#[doc = "CMDL1 (rw) register accessor: an alias for `Reg<CMDL1_SPEC>`"]
pub type CMDL1 = crate::Reg<cmdl1::CMDL1_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl1;
#[doc = "CMDH1 (rw) register accessor: an alias for `Reg<CMDH1_SPEC>`"]
pub type CMDH1 = crate::Reg<cmdh1::CMDH1_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh1;
#[doc = "CMDL2 (rw) register accessor: an alias for `Reg<CMDL2_SPEC>`"]
pub type CMDL2 = crate::Reg<cmdl2::CMDL2_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl2;
#[doc = "CMDH2 (rw) register accessor: an alias for `Reg<CMDH2_SPEC>`"]
pub type CMDH2 = crate::Reg<cmdh2::CMDH2_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh2;
#[doc = "CMDL3 (rw) register accessor: an alias for `Reg<CMDL3_SPEC>`"]
pub type CMDL3 = crate::Reg<cmdl3::CMDL3_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl3;
#[doc = "CMDH3 (rw) register accessor: an alias for `Reg<CMDH3_SPEC>`"]
pub type CMDH3 = crate::Reg<cmdh3::CMDH3_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh3;
#[doc = "CMDL4 (rw) register accessor: an alias for `Reg<CMDL4_SPEC>`"]
pub type CMDL4 = crate::Reg<cmdl4::CMDL4_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl4;
#[doc = "CMDH4 (rw) register accessor: an alias for `Reg<CMDH4_SPEC>`"]
pub type CMDH4 = crate::Reg<cmdh4::CMDH4_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh4;
#[doc = "CMDL5 (rw) register accessor: an alias for `Reg<CMDL5_SPEC>`"]
pub type CMDL5 = crate::Reg<cmdl5::CMDL5_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl5;
#[doc = "CMDH5 (rw) register accessor: an alias for `Reg<CMDH5_SPEC>`"]
pub type CMDH5 = crate::Reg<cmdh5::CMDH5_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh5;
#[doc = "CMDL6 (rw) register accessor: an alias for `Reg<CMDL6_SPEC>`"]
pub type CMDL6 = crate::Reg<cmdl6::CMDL6_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl6;
#[doc = "CMDH6 (rw) register accessor: an alias for `Reg<CMDH6_SPEC>`"]
pub type CMDH6 = crate::Reg<cmdh6::CMDH6_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh6;
#[doc = "CMDL7 (rw) register accessor: an alias for `Reg<CMDL7_SPEC>`"]
pub type CMDL7 = crate::Reg<cmdl7::CMDL7_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl7;
#[doc = "CMDH7 (rw) register accessor: an alias for `Reg<CMDH7_SPEC>`"]
pub type CMDH7 = crate::Reg<cmdh7::CMDH7_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh7;
#[doc = "CMDL8 (rw) register accessor: an alias for `Reg<CMDL8_SPEC>`"]
pub type CMDL8 = crate::Reg<cmdl8::CMDL8_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl8;
#[doc = "CMDH8 (rw) register accessor: an alias for `Reg<CMDH8_SPEC>`"]
pub type CMDH8 = crate::Reg<cmdh8::CMDH8_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh8;
#[doc = "CMDL9 (rw) register accessor: an alias for `Reg<CMDL9_SPEC>`"]
pub type CMDL9 = crate::Reg<cmdl9::CMDL9_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl9;
#[doc = "CMDH9 (rw) register accessor: an alias for `Reg<CMDH9_SPEC>`"]
pub type CMDH9 = crate::Reg<cmdh9::CMDH9_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh9;
#[doc = "CMDL10 (rw) register accessor: an alias for `Reg<CMDL10_SPEC>`"]
pub type CMDL10 = crate::Reg<cmdl10::CMDL10_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl10;
#[doc = "CMDH10 (rw) register accessor: an alias for `Reg<CMDH10_SPEC>`"]
pub type CMDH10 = crate::Reg<cmdh10::CMDH10_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh10;
#[doc = "CMDL11 (rw) register accessor: an alias for `Reg<CMDL11_SPEC>`"]
pub type CMDL11 = crate::Reg<cmdl11::CMDL11_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl11;
#[doc = "CMDH11 (rw) register accessor: an alias for `Reg<CMDH11_SPEC>`"]
pub type CMDH11 = crate::Reg<cmdh11::CMDH11_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh11;
#[doc = "CMDL12 (rw) register accessor: an alias for `Reg<CMDL12_SPEC>`"]
pub type CMDL12 = crate::Reg<cmdl12::CMDL12_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl12;
#[doc = "CMDH12 (rw) register accessor: an alias for `Reg<CMDH12_SPEC>`"]
pub type CMDH12 = crate::Reg<cmdh12::CMDH12_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh12;
#[doc = "CMDL13 (rw) register accessor: an alias for `Reg<CMDL13_SPEC>`"]
pub type CMDL13 = crate::Reg<cmdl13::CMDL13_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl13;
#[doc = "CMDH13 (rw) register accessor: an alias for `Reg<CMDH13_SPEC>`"]
pub type CMDH13 = crate::Reg<cmdh13::CMDH13_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh13;
#[doc = "CMDL14 (rw) register accessor: an alias for `Reg<CMDL14_SPEC>`"]
pub type CMDL14 = crate::Reg<cmdl14::CMDL14_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl14;
#[doc = "CMDH14 (rw) register accessor: an alias for `Reg<CMDH14_SPEC>`"]
pub type CMDH14 = crate::Reg<cmdh14::CMDH14_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh14;
#[doc = "CMDL15 (rw) register accessor: an alias for `Reg<CMDL15_SPEC>`"]
pub type CMDL15 = crate::Reg<cmdl15::CMDL15_SPEC>;
#[doc = "ADC Command Low Buffer Register"]
pub mod cmdl15;
#[doc = "CMDH15 (rw) register accessor: an alias for `Reg<CMDH15_SPEC>`"]
pub type CMDH15 = crate::Reg<cmdh15::CMDH15_SPEC>;
#[doc = "ADC Command High Buffer Register"]
pub mod cmdh15;
#[doc = "CV (rw) register accessor: an alias for `Reg<CV_SPEC>`"]
pub type CV = crate::Reg<cv::CV_SPEC>;
#[doc = "Compare Value Register"]
pub mod cv;
#[doc = "RESFIFO (r) register accessor: an alias for `Reg<RESFIFO_SPEC>`"]
pub type RESFIFO = crate::Reg<resfifo::RESFIFO_SPEC>;
#[doc = "ADC Data Result FIFO Register"]
pub mod resfifo;
#[doc = "CAL_GAR (rw) register accessor: an alias for `Reg<CAL_GAR_SPEC>`"]
pub type CAL_GAR = crate::Reg<cal_gar::CAL_GAR_SPEC>;
#[doc = "Calibration General A-Side Registers"]
pub mod cal_gar;
#[doc = "CAL_GBR (rw) register accessor: an alias for `Reg<CAL_GBR_SPEC>`"]
pub type CAL_GBR = crate::Reg<cal_gbr::CAL_GBR_SPEC>;
#[doc = "Calibration General B-Side Registers"]
pub mod cal_gbr;
#[doc = "TST (rw) register accessor: an alias for `Reg<TST_SPEC>`"]
pub type TST = crate::Reg<tst::TST_SPEC>;
#[doc = "ADC Test Register"]
pub mod tst;
