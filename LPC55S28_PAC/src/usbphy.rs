#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB PHY Power-Down Register"]
    pub pwd: PWD,
    #[doc = "0x04 - USB PHY Power-Down Register"]
    pub pwd_set: PWD_SET,
    #[doc = "0x08 - USB PHY Power-Down Register"]
    pub pwd_clr: PWD_CLR,
    #[doc = "0x0c - USB PHY Power-Down Register"]
    pub pwd_tog: PWD_TOG,
    #[doc = "0x10 - USB PHY Transmitter Control Register"]
    pub tx: TX,
    #[doc = "0x14 - USB PHY Transmitter Control Register"]
    pub tx_set: TX_SET,
    #[doc = "0x18 - USB PHY Transmitter Control Register"]
    pub tx_clr: TX_CLR,
    #[doc = "0x1c - USB PHY Transmitter Control Register"]
    pub tx_tog: TX_TOG,
    #[doc = "0x20 - USB PHY Receiver Control Register"]
    pub rx: RX,
    #[doc = "0x24 - USB PHY Receiver Control Register"]
    pub rx_set: RX_SET,
    #[doc = "0x28 - USB PHY Receiver Control Register"]
    pub rx_clr: RX_CLR,
    #[doc = "0x2c - USB PHY Receiver Control Register"]
    pub rx_tog: RX_TOG,
    #[doc = "0x30 - USB PHY General Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x34 - USB PHY General Control Register"]
    pub ctrl_set: CTRL_SET,
    #[doc = "0x38 - USB PHY General Control Register"]
    pub ctrl_clr: CTRL_CLR,
    #[doc = "0x3c - USB PHY General Control Register"]
    pub ctrl_tog: CTRL_TOG,
    #[doc = "0x40 - USB PHY Status Register"]
    pub status: STATUS,
    _reserved17: [u8; 0x5c],
    #[doc = "0xa0 - USB PHY PLL Control/Status Register"]
    pub pll_sic: PLL_SIC,
    #[doc = "0xa4 - USB PHY PLL Control/Status Register"]
    pub pll_sic_set: PLL_SIC_SET,
    #[doc = "0xa8 - USB PHY PLL Control/Status Register"]
    pub pll_sic_clr: PLL_SIC_CLR,
    #[doc = "0xac - USB PHY PLL Control/Status Register"]
    pub pll_sic_tog: PLL_SIC_TOG,
    _reserved21: [u8; 0x10],
    #[doc = "0xc0 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect: USB1_VBUS_DETECT,
    #[doc = "0xc4 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_set: USB1_VBUS_DETECT_SET,
    #[doc = "0xc8 - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_clr: USB1_VBUS_DETECT_CLR,
    #[doc = "0xcc - USB PHY VBUS Detect Control Register"]
    pub usb1_vbus_detect_tog: USB1_VBUS_DETECT_TOG,
    _reserved25: [u8; 0x30],
    #[doc = "0x100 - USB PHY Analog Control Register"]
    pub anactrl: ANACTRL,
    #[doc = "0x104 - USB PHY Analog Control Register"]
    pub anactrl_set: ANACTRL_SET,
    #[doc = "0x108 - USB PHY Analog Control Register"]
    pub anactrl_clr: ANACTRL_CLR,
    #[doc = "0x10c - USB PHY Analog Control Register"]
    pub anactrl_tog: ANACTRL_TOG,
}
#[doc = "PWD (rw) register accessor: an alias for `Reg<PWD_SPEC>`"]
pub type PWD = crate::Reg<pwd::PWD_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd;
#[doc = "PWD_SET (rw) register accessor: an alias for `Reg<PWD_SET_SPEC>`"]
pub type PWD_SET = crate::Reg<pwd_set::PWD_SET_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_set;
#[doc = "PWD_CLR (rw) register accessor: an alias for `Reg<PWD_CLR_SPEC>`"]
pub type PWD_CLR = crate::Reg<pwd_clr::PWD_CLR_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_clr;
#[doc = "PWD_TOG (rw) register accessor: an alias for `Reg<PWD_TOG_SPEC>`"]
pub type PWD_TOG = crate::Reg<pwd_tog::PWD_TOG_SPEC>;
#[doc = "USB PHY Power-Down Register"]
pub mod pwd_tog;
#[doc = "TX (rw) register accessor: an alias for `Reg<TX_SPEC>`"]
pub type TX = crate::Reg<tx::TX_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx;
#[doc = "TX_SET (rw) register accessor: an alias for `Reg<TX_SET_SPEC>`"]
pub type TX_SET = crate::Reg<tx_set::TX_SET_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_set;
#[doc = "TX_CLR (rw) register accessor: an alias for `Reg<TX_CLR_SPEC>`"]
pub type TX_CLR = crate::Reg<tx_clr::TX_CLR_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_clr;
#[doc = "TX_TOG (rw) register accessor: an alias for `Reg<TX_TOG_SPEC>`"]
pub type TX_TOG = crate::Reg<tx_tog::TX_TOG_SPEC>;
#[doc = "USB PHY Transmitter Control Register"]
pub mod tx_tog;
#[doc = "RX (rw) register accessor: an alias for `Reg<RX_SPEC>`"]
pub type RX = crate::Reg<rx::RX_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx;
#[doc = "RX_SET (rw) register accessor: an alias for `Reg<RX_SET_SPEC>`"]
pub type RX_SET = crate::Reg<rx_set::RX_SET_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_set;
#[doc = "RX_CLR (rw) register accessor: an alias for `Reg<RX_CLR_SPEC>`"]
pub type RX_CLR = crate::Reg<rx_clr::RX_CLR_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_clr;
#[doc = "RX_TOG (rw) register accessor: an alias for `Reg<RX_TOG_SPEC>`"]
pub type RX_TOG = crate::Reg<rx_tog::RX_TOG_SPEC>;
#[doc = "USB PHY Receiver Control Register"]
pub mod rx_tog;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl;
#[doc = "CTRL_SET (rw) register accessor: an alias for `Reg<CTRL_SET_SPEC>`"]
pub type CTRL_SET = crate::Reg<ctrl_set::CTRL_SET_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_set;
#[doc = "CTRL_CLR (rw) register accessor: an alias for `Reg<CTRL_CLR_SPEC>`"]
pub type CTRL_CLR = crate::Reg<ctrl_clr::CTRL_CLR_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_clr;
#[doc = "CTRL_TOG (rw) register accessor: an alias for `Reg<CTRL_TOG_SPEC>`"]
pub type CTRL_TOG = crate::Reg<ctrl_tog::CTRL_TOG_SPEC>;
#[doc = "USB PHY General Control Register"]
pub mod ctrl_tog;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "USB PHY Status Register"]
pub mod status;
#[doc = "PLL_SIC (rw) register accessor: an alias for `Reg<PLL_SIC_SPEC>`"]
pub type PLL_SIC = crate::Reg<pll_sic::PLL_SIC_SPEC>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic;
#[doc = "PLL_SIC_SET (rw) register accessor: an alias for `Reg<PLL_SIC_SET_SPEC>`"]
pub type PLL_SIC_SET = crate::Reg<pll_sic_set::PLL_SIC_SET_SPEC>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_set;
#[doc = "PLL_SIC_CLR (rw) register accessor: an alias for `Reg<PLL_SIC_CLR_SPEC>`"]
pub type PLL_SIC_CLR = crate::Reg<pll_sic_clr::PLL_SIC_CLR_SPEC>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_clr;
#[doc = "PLL_SIC_TOG (rw) register accessor: an alias for `Reg<PLL_SIC_TOG_SPEC>`"]
pub type PLL_SIC_TOG = crate::Reg<pll_sic_tog::PLL_SIC_TOG_SPEC>;
#[doc = "USB PHY PLL Control/Status Register"]
pub mod pll_sic_tog;
#[doc = "USB1_VBUS_DETECT (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_SPEC>`"]
pub type USB1_VBUS_DETECT = crate::Reg<usb1_vbus_detect::USB1_VBUS_DETECT_SPEC>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect;
#[doc = "USB1_VBUS_DETECT_SET (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_SET_SPEC>`"]
pub type USB1_VBUS_DETECT_SET = crate::Reg<usb1_vbus_detect_set::USB1_VBUS_DETECT_SET_SPEC>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_set;
#[doc = "USB1_VBUS_DETECT_CLR (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_CLR_SPEC>`"]
pub type USB1_VBUS_DETECT_CLR = crate::Reg<usb1_vbus_detect_clr::USB1_VBUS_DETECT_CLR_SPEC>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_clr;
#[doc = "USB1_VBUS_DETECT_TOG (rw) register accessor: an alias for `Reg<USB1_VBUS_DETECT_TOG_SPEC>`"]
pub type USB1_VBUS_DETECT_TOG = crate::Reg<usb1_vbus_detect_tog::USB1_VBUS_DETECT_TOG_SPEC>;
#[doc = "USB PHY VBUS Detect Control Register"]
pub mod usb1_vbus_detect_tog;
#[doc = "ANACTRL (rw) register accessor: an alias for `Reg<ANACTRL_SPEC>`"]
pub type ANACTRL = crate::Reg<anactrl::ANACTRL_SPEC>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl;
#[doc = "ANACTRL_SET (rw) register accessor: an alias for `Reg<ANACTRL_SET_SPEC>`"]
pub type ANACTRL_SET = crate::Reg<anactrl_set::ANACTRL_SET_SPEC>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_set;
#[doc = "ANACTRL_CLR (rw) register accessor: an alias for `Reg<ANACTRL_CLR_SPEC>`"]
pub type ANACTRL_CLR = crate::Reg<anactrl_clr::ANACTRL_CLR_SPEC>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_clr;
#[doc = "ANACTRL_TOG (rw) register accessor: an alias for `Reg<ANACTRL_TOG_SPEC>`"]
pub type ANACTRL_TOG = crate::Reg<anactrl_tog::ANACTRL_TOG_SPEC>;
#[doc = "USB PHY Analog Control Register"]
pub mod anactrl_tog;
