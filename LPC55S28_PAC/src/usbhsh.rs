#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
    pub caplength_chipid: CAPLENGTH_CHIPID,
    #[doc = "0x04 - Host Controller Structural Parameters"]
    pub hcsparams: HCSPARAMS,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Frame Length Adjustment"]
    pub fladj_frindex: FLADJ_FRINDEX,
    #[doc = "0x10 - Memory base address where ATL PTD0 is stored"]
    pub atlptd: ATLPTD,
    #[doc = "0x14 - Memory base address where ISO PTD0 is stored"]
    pub isoptd: ISOPTD,
    #[doc = "0x18 - Memory base address where INT PTD0 is stored"]
    pub intptd: INTPTD,
    #[doc = "0x1c - Memory base address that indicates the start of the data payload buffers"]
    pub datapayload: DATAPAYLOAD,
    #[doc = "0x20 - USB Command register"]
    pub usbcmd: USBCMD,
    #[doc = "0x24 - USB Interrupt Status register"]
    pub usbsts: USBSTS,
    #[doc = "0x28 - USB Interrupt Enable register"]
    pub usbintr: USBINTR,
    #[doc = "0x2c - Port Status and Control register"]
    pub portsc1: PORTSC1,
    #[doc = "0x30 - Done map for each ATL PTD"]
    pub atlptdd: ATLPTDD,
    #[doc = "0x34 - Skip map for each ATL PTD"]
    pub atlptds: ATLPTDS,
    #[doc = "0x38 - Done map for each ISO PTD"]
    pub isoptdd: ISOPTDD,
    #[doc = "0x3c - Skip map for each ISO PTD"]
    pub isoptds: ISOPTDS,
    #[doc = "0x40 - Done map for each INT PTD"]
    pub intptdd: INTPTDD,
    #[doc = "0x44 - Skip map for each INT PTD"]
    pub intptds: INTPTDS,
    #[doc = "0x48 - Marks the last PTD in the list for ISO, INT and ATL"]
    pub lastptd: LASTPTD,
    _reserved18: [u8; 0x04],
    #[doc = "0x50 - Controls the port if it is attached to the host block or the device block"]
    pub portmode: PORTMODE,
}
#[doc = "CAPLENGTH_CHIPID (r) register accessor: an alias for `Reg<CAPLENGTH_CHIPID_SPEC>`"]
pub type CAPLENGTH_CHIPID = crate::Reg<caplength_chipid::CAPLENGTH_CHIPID_SPEC>;
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block"]
pub mod caplength_chipid;
#[doc = "HCSPARAMS (r) register accessor: an alias for `Reg<HCSPARAMS_SPEC>`"]
pub type HCSPARAMS = crate::Reg<hcsparams::HCSPARAMS_SPEC>;
#[doc = "Host Controller Structural Parameters"]
pub mod hcsparams;
#[doc = "FLADJ_FRINDEX (rw) register accessor: an alias for `Reg<FLADJ_FRINDEX_SPEC>`"]
pub type FLADJ_FRINDEX = crate::Reg<fladj_frindex::FLADJ_FRINDEX_SPEC>;
#[doc = "Frame Length Adjustment"]
pub mod fladj_frindex;
#[doc = "ATLPTD (rw) register accessor: an alias for `Reg<ATLPTD_SPEC>`"]
pub type ATLPTD = crate::Reg<atlptd::ATLPTD_SPEC>;
#[doc = "Memory base address where ATL PTD0 is stored"]
pub mod atlptd;
#[doc = "ISOPTD (rw) register accessor: an alias for `Reg<ISOPTD_SPEC>`"]
pub type ISOPTD = crate::Reg<isoptd::ISOPTD_SPEC>;
#[doc = "Memory base address where ISO PTD0 is stored"]
pub mod isoptd;
#[doc = "INTPTD (rw) register accessor: an alias for `Reg<INTPTD_SPEC>`"]
pub type INTPTD = crate::Reg<intptd::INTPTD_SPEC>;
#[doc = "Memory base address where INT PTD0 is stored"]
pub mod intptd;
#[doc = "DATAPAYLOAD (rw) register accessor: an alias for `Reg<DATAPAYLOAD_SPEC>`"]
pub type DATAPAYLOAD = crate::Reg<datapayload::DATAPAYLOAD_SPEC>;
#[doc = "Memory base address that indicates the start of the data payload buffers"]
pub mod datapayload;
#[doc = "USBCMD (rw) register accessor: an alias for `Reg<USBCMD_SPEC>`"]
pub type USBCMD = crate::Reg<usbcmd::USBCMD_SPEC>;
#[doc = "USB Command register"]
pub mod usbcmd;
#[doc = "USBSTS (rw) register accessor: an alias for `Reg<USBSTS_SPEC>`"]
pub type USBSTS = crate::Reg<usbsts::USBSTS_SPEC>;
#[doc = "USB Interrupt Status register"]
pub mod usbsts;
#[doc = "USBINTR (rw) register accessor: an alias for `Reg<USBINTR_SPEC>`"]
pub type USBINTR = crate::Reg<usbintr::USBINTR_SPEC>;
#[doc = "USB Interrupt Enable register"]
pub mod usbintr;
#[doc = "PORTSC1 (rw) register accessor: an alias for `Reg<PORTSC1_SPEC>`"]
pub type PORTSC1 = crate::Reg<portsc1::PORTSC1_SPEC>;
#[doc = "Port Status and Control register"]
pub mod portsc1;
#[doc = "ATLPTDD (rw) register accessor: an alias for `Reg<ATLPTDD_SPEC>`"]
pub type ATLPTDD = crate::Reg<atlptdd::ATLPTDD_SPEC>;
#[doc = "Done map for each ATL PTD"]
pub mod atlptdd;
#[doc = "ATLPTDS (rw) register accessor: an alias for `Reg<ATLPTDS_SPEC>`"]
pub type ATLPTDS = crate::Reg<atlptds::ATLPTDS_SPEC>;
#[doc = "Skip map for each ATL PTD"]
pub mod atlptds;
#[doc = "ISOPTDD (rw) register accessor: an alias for `Reg<ISOPTDD_SPEC>`"]
pub type ISOPTDD = crate::Reg<isoptdd::ISOPTDD_SPEC>;
#[doc = "Done map for each ISO PTD"]
pub mod isoptdd;
#[doc = "ISOPTDS (rw) register accessor: an alias for `Reg<ISOPTDS_SPEC>`"]
pub type ISOPTDS = crate::Reg<isoptds::ISOPTDS_SPEC>;
#[doc = "Skip map for each ISO PTD"]
pub mod isoptds;
#[doc = "INTPTDD (rw) register accessor: an alias for `Reg<INTPTDD_SPEC>`"]
pub type INTPTDD = crate::Reg<intptdd::INTPTDD_SPEC>;
#[doc = "Done map for each INT PTD"]
pub mod intptdd;
#[doc = "INTPTDS (rw) register accessor: an alias for `Reg<INTPTDS_SPEC>`"]
pub type INTPTDS = crate::Reg<intptds::INTPTDS_SPEC>;
#[doc = "Skip map for each INT PTD"]
pub mod intptds;
#[doc = "LASTPTD (rw) register accessor: an alias for `Reg<LASTPTD_SPEC>`"]
pub type LASTPTD = crate::Reg<lastptd::LASTPTD_SPEC>;
#[doc = "Marks the last PTD in the list for ISO, INT and ATL"]
pub mod lastptd;
#[doc = "PORTMODE (rw) register accessor: an alias for `Reg<PORTMODE_SPEC>`"]
pub type PORTMODE = crate::Reg<portmode::PORTMODE_SPEC>;
#[doc = "Controls the port if it is attached to the host block or the device block"]
pub mod portmode;
