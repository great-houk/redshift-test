#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Power Enable register"]
    pub pwren: PWREN,
    #[doc = "0x08 - Clock Divider register"]
    pub clkdiv: CLKDIV,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Clock Enable register"]
    pub clkena: CLKENA,
    #[doc = "0x14 - Time-out register"]
    pub tmout: TMOUT,
    #[doc = "0x18 - Card Type register"]
    pub ctype: CTYPE,
    #[doc = "0x1c - Block Size register"]
    pub blksiz: BLKSIZ,
    #[doc = "0x20 - Byte Count register"]
    pub bytcnt: BYTCNT,
    #[doc = "0x24 - Interrupt Mask register"]
    pub intmask: INTMASK,
    #[doc = "0x28 - Command Argument register"]
    pub cmdarg: CMDARG,
    #[doc = "0x2c - Command register"]
    pub cmd: CMD,
    #[doc = "0x30..0x40 - Response register"]
    pub resp: [RESP; 4],
    #[doc = "0x40 - Masked Interrupt Status register"]
    pub mintsts: MINTSTS,
    #[doc = "0x44 - Raw Interrupt Status register"]
    pub rintsts: RINTSTS,
    #[doc = "0x48 - Status register"]
    pub status: STATUS,
    #[doc = "0x4c - FIFO Threshold Watermark register"]
    pub fifoth: FIFOTH,
    #[doc = "0x50 - Card Detect register"]
    pub cdetect: CDETECT,
    #[doc = "0x54 - Write Protect register"]
    pub wrtprt: WRTPRT,
    _reserved18: [u8; 0x04],
    #[doc = "0x5c - Transferred CIU Card Byte Count register"]
    pub tcbcnt: TCBCNT,
    #[doc = "0x60 - Transferred Host to BIU-FIFO Byte Count register"]
    pub tbbcnt: TBBCNT,
    #[doc = "0x64 - Debounce Count register"]
    pub debnce: DEBNCE,
    _reserved21: [u8; 0x10],
    #[doc = "0x78 - Hardware Reset"]
    pub rst_n: RST_N,
    _reserved22: [u8; 0x04],
    #[doc = "0x80 - Bus Mode register"]
    pub bmod: BMOD,
    #[doc = "0x84 - Poll Demand register"]
    pub pldmnd: PLDMND,
    #[doc = "0x88 - Descriptor List Base Address register"]
    pub dbaddr: DBADDR,
    #[doc = "0x8c - Internal DMAC Status register"]
    pub idsts: IDSTS,
    #[doc = "0x90 - Internal DMAC Interrupt Enable register"]
    pub idinten: IDINTEN,
    #[doc = "0x94 - Current Host Descriptor Address register"]
    pub dscaddr: DSCADDR,
    #[doc = "0x98 - Current Buffer Descriptor Address register"]
    pub bufaddr: BUFADDR,
    _reserved29: [u8; 0x64],
    #[doc = "0x100 - Card Threshold Control"]
    pub cardthrctl: CARDTHRCTL,
    #[doc = "0x104 - Power control"]
    pub backendpwr: BACKENDPWR,
    _reserved31: [u8; 0xf8],
    #[doc = "0x200..0x300 - SDIF FIFO"]
    pub fifo: [FIFO; 64],
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "PWREN (rw) register accessor: an alias for `Reg<PWREN_SPEC>`"]
pub type PWREN = crate::Reg<pwren::PWREN_SPEC>;
#[doc = "Power Enable register"]
pub mod pwren;
#[doc = "CLKDIV (rw) register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "Clock Divider register"]
pub mod clkdiv;
#[doc = "CLKENA (rw) register accessor: an alias for `Reg<CLKENA_SPEC>`"]
pub type CLKENA = crate::Reg<clkena::CLKENA_SPEC>;
#[doc = "Clock Enable register"]
pub mod clkena;
#[doc = "TMOUT (rw) register accessor: an alias for `Reg<TMOUT_SPEC>`"]
pub type TMOUT = crate::Reg<tmout::TMOUT_SPEC>;
#[doc = "Time-out register"]
pub mod tmout;
#[doc = "CTYPE (rw) register accessor: an alias for `Reg<CTYPE_SPEC>`"]
pub type CTYPE = crate::Reg<ctype::CTYPE_SPEC>;
#[doc = "Card Type register"]
pub mod ctype;
#[doc = "BLKSIZ (rw) register accessor: an alias for `Reg<BLKSIZ_SPEC>`"]
pub type BLKSIZ = crate::Reg<blksiz::BLKSIZ_SPEC>;
#[doc = "Block Size register"]
pub mod blksiz;
#[doc = "BYTCNT (rw) register accessor: an alias for `Reg<BYTCNT_SPEC>`"]
pub type BYTCNT = crate::Reg<bytcnt::BYTCNT_SPEC>;
#[doc = "Byte Count register"]
pub mod bytcnt;
#[doc = "INTMASK (rw) register accessor: an alias for `Reg<INTMASK_SPEC>`"]
pub type INTMASK = crate::Reg<intmask::INTMASK_SPEC>;
#[doc = "Interrupt Mask register"]
pub mod intmask;
#[doc = "CMDARG (rw) register accessor: an alias for `Reg<CMDARG_SPEC>`"]
pub type CMDARG = crate::Reg<cmdarg::CMDARG_SPEC>;
#[doc = "Command Argument register"]
pub mod cmdarg;
#[doc = "CMD (rw) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command register"]
pub mod cmd;
#[doc = "RESP (rw) register accessor: an alias for `Reg<RESP_SPEC>`"]
pub type RESP = crate::Reg<resp::RESP_SPEC>;
#[doc = "Response register"]
pub mod resp;
#[doc = "MINTSTS (rw) register accessor: an alias for `Reg<MINTSTS_SPEC>`"]
pub type MINTSTS = crate::Reg<mintsts::MINTSTS_SPEC>;
#[doc = "Masked Interrupt Status register"]
pub mod mintsts;
#[doc = "RINTSTS (rw) register accessor: an alias for `Reg<RINTSTS_SPEC>`"]
pub type RINTSTS = crate::Reg<rintsts::RINTSTS_SPEC>;
#[doc = "Raw Interrupt Status register"]
pub mod rintsts;
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status register"]
pub mod status;
#[doc = "FIFOTH (rw) register accessor: an alias for `Reg<FIFOTH_SPEC>`"]
pub type FIFOTH = crate::Reg<fifoth::FIFOTH_SPEC>;
#[doc = "FIFO Threshold Watermark register"]
pub mod fifoth;
#[doc = "CDETECT (rw) register accessor: an alias for `Reg<CDETECT_SPEC>`"]
pub type CDETECT = crate::Reg<cdetect::CDETECT_SPEC>;
#[doc = "Card Detect register"]
pub mod cdetect;
#[doc = "WRTPRT (rw) register accessor: an alias for `Reg<WRTPRT_SPEC>`"]
pub type WRTPRT = crate::Reg<wrtprt::WRTPRT_SPEC>;
#[doc = "Write Protect register"]
pub mod wrtprt;
#[doc = "TCBCNT (rw) register accessor: an alias for `Reg<TCBCNT_SPEC>`"]
pub type TCBCNT = crate::Reg<tcbcnt::TCBCNT_SPEC>;
#[doc = "Transferred CIU Card Byte Count register"]
pub mod tcbcnt;
#[doc = "TBBCNT (rw) register accessor: an alias for `Reg<TBBCNT_SPEC>`"]
pub type TBBCNT = crate::Reg<tbbcnt::TBBCNT_SPEC>;
#[doc = "Transferred Host to BIU-FIFO Byte Count register"]
pub mod tbbcnt;
#[doc = "DEBNCE (rw) register accessor: an alias for `Reg<DEBNCE_SPEC>`"]
pub type DEBNCE = crate::Reg<debnce::DEBNCE_SPEC>;
#[doc = "Debounce Count register"]
pub mod debnce;
#[doc = "RST_N (rw) register accessor: an alias for `Reg<RST_N_SPEC>`"]
pub type RST_N = crate::Reg<rst_n::RST_N_SPEC>;
#[doc = "Hardware Reset"]
pub mod rst_n;
#[doc = "BMOD (rw) register accessor: an alias for `Reg<BMOD_SPEC>`"]
pub type BMOD = crate::Reg<bmod::BMOD_SPEC>;
#[doc = "Bus Mode register"]
pub mod bmod;
#[doc = "PLDMND (rw) register accessor: an alias for `Reg<PLDMND_SPEC>`"]
pub type PLDMND = crate::Reg<pldmnd::PLDMND_SPEC>;
#[doc = "Poll Demand register"]
pub mod pldmnd;
#[doc = "DBADDR (rw) register accessor: an alias for `Reg<DBADDR_SPEC>`"]
pub type DBADDR = crate::Reg<dbaddr::DBADDR_SPEC>;
#[doc = "Descriptor List Base Address register"]
pub mod dbaddr;
#[doc = "IDSTS (rw) register accessor: an alias for `Reg<IDSTS_SPEC>`"]
pub type IDSTS = crate::Reg<idsts::IDSTS_SPEC>;
#[doc = "Internal DMAC Status register"]
pub mod idsts;
#[doc = "IDINTEN (rw) register accessor: an alias for `Reg<IDINTEN_SPEC>`"]
pub type IDINTEN = crate::Reg<idinten::IDINTEN_SPEC>;
#[doc = "Internal DMAC Interrupt Enable register"]
pub mod idinten;
#[doc = "DSCADDR (rw) register accessor: an alias for `Reg<DSCADDR_SPEC>`"]
pub type DSCADDR = crate::Reg<dscaddr::DSCADDR_SPEC>;
#[doc = "Current Host Descriptor Address register"]
pub mod dscaddr;
#[doc = "BUFADDR (rw) register accessor: an alias for `Reg<BUFADDR_SPEC>`"]
pub type BUFADDR = crate::Reg<bufaddr::BUFADDR_SPEC>;
#[doc = "Current Buffer Descriptor Address register"]
pub mod bufaddr;
#[doc = "CARDTHRCTL (rw) register accessor: an alias for `Reg<CARDTHRCTL_SPEC>`"]
pub type CARDTHRCTL = crate::Reg<cardthrctl::CARDTHRCTL_SPEC>;
#[doc = "Card Threshold Control"]
pub mod cardthrctl;
#[doc = "BACKENDPWR (rw) register accessor: an alias for `Reg<BACKENDPWR_SPEC>`"]
pub type BACKENDPWR = crate::Reg<backendpwr::BACKENDPWR_SPEC>;
#[doc = "Power control"]
pub mod backendpwr;
#[doc = "FIFO (rw) register accessor: an alias for `Reg<FIFO_SPEC>`"]
pub type FIFO = crate::Reg<fifo::FIFO_SPEC>;
#[doc = "SDIF FIFO"]
pub mod fifo;
