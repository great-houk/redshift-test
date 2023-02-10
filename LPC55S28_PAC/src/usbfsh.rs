#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)"]
    pub hcrevision: HCREVISION,
    #[doc = "0x04 - Defines the operating modes of the HC"]
    pub hccontrol: HCCONTROL,
    #[doc = "0x08 - This register is used to receive the commands from the Host Controller Driver (HCD)"]
    pub hccommandstatus: HCCOMMANDSTATUS,
    #[doc = "0x0c - Indicates the status on various events that cause hardware interrupts by setting the appropriate bits"]
    pub hcinterruptstatus: HCINTERRUPTSTATUS,
    #[doc = "0x10 - Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt"]
    pub hcinterruptenable: HCINTERRUPTENABLE,
    #[doc = "0x14 - The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt"]
    pub hcinterruptdisable: HCINTERRUPTDISABLE,
    #[doc = "0x18 - Contains the physical address of the host controller communication area"]
    pub hchcca: HCHCCA,
    #[doc = "0x1c - Contains the physical address of the current isochronous or interrupt endpoint descriptor"]
    pub hcperiodcurrented: HCPERIODCURRENTED,
    #[doc = "0x20 - Contains the physical address of the first endpoint descriptor of the control list"]
    pub hccontrolheaded: HCCONTROLHEADED,
    #[doc = "0x24 - Contains the physical address of the current endpoint descriptor of the control list"]
    pub hccontrolcurrented: HCCONTROLCURRENTED,
    #[doc = "0x28 - Contains the physical address of the first endpoint descriptor of the bulk list"]
    pub hcbulkheaded: HCBULKHEADED,
    #[doc = "0x2c - Contains the physical address of the current endpoint descriptor of the bulk list"]
    pub hcbulkcurrented: HCBULKCURRENTED,
    #[doc = "0x30 - Contains the physical address of the last transfer descriptor added to the 'Done' queue"]
    pub hcdonehead: HCDONEHEAD,
    #[doc = "0x34 - Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun"]
    pub hcfminterval: HCFMINTERVAL,
    #[doc = "0x38 - A 14-bit counter showing the bit time remaining in the current frame"]
    pub hcfmremaining: HCFMREMAINING,
    #[doc = "0x3c - Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD"]
    pub hcfmnumber: HCFMNUMBER,
    #[doc = "0x40 - Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list"]
    pub hcperiodicstart: HCPERIODICSTART,
    #[doc = "0x44 - Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF"]
    pub hclsthreshold: HCLSTHRESHOLD,
    #[doc = "0x48 - First of the two registers which describes the characteristics of the root hub"]
    pub hcrhdescriptora: HCRHDESCRIPTORA,
    #[doc = "0x4c - Second of the two registers which describes the characteristics of the Root Hub"]
    pub hcrhdescriptorb: HCRHDESCRIPTORB,
    #[doc = "0x50 - This register is divided into two parts"]
    pub hcrhstatus: HCRHSTATUS,
    #[doc = "0x54 - Controls and reports the port events on a per-port basis"]
    pub hcrhportstatus: HCRHPORTSTATUS,
    _reserved22: [u8; 0x04],
    #[doc = "0x5c - Controls the port if it is attached to the host block or the device block"]
    pub portmode: PORTMODE,
}
#[doc = "HCREVISION (r) register accessor: an alias for `Reg<HCREVISION_SPEC>`"]
pub type HCREVISION = crate::Reg<hcrevision::HCREVISION_SPEC>;
#[doc = "BCD representation of the version of the HCI specification that is implemented by the Host Controller (HC)"]
pub mod hcrevision;
#[doc = "HCCONTROL (rw) register accessor: an alias for `Reg<HCCONTROL_SPEC>`"]
pub type HCCONTROL = crate::Reg<hccontrol::HCCONTROL_SPEC>;
#[doc = "Defines the operating modes of the HC"]
pub mod hccontrol;
#[doc = "HCCOMMANDSTATUS (rw) register accessor: an alias for `Reg<HCCOMMANDSTATUS_SPEC>`"]
pub type HCCOMMANDSTATUS = crate::Reg<hccommandstatus::HCCOMMANDSTATUS_SPEC>;
#[doc = "This register is used to receive the commands from the Host Controller Driver (HCD)"]
pub mod hccommandstatus;
#[doc = "HCINTERRUPTSTATUS (rw) register accessor: an alias for `Reg<HCINTERRUPTSTATUS_SPEC>`"]
pub type HCINTERRUPTSTATUS = crate::Reg<hcinterruptstatus::HCINTERRUPTSTATUS_SPEC>;
#[doc = "Indicates the status on various events that cause hardware interrupts by setting the appropriate bits"]
pub mod hcinterruptstatus;
#[doc = "HCINTERRUPTENABLE (rw) register accessor: an alias for `Reg<HCINTERRUPTENABLE_SPEC>`"]
pub type HCINTERRUPTENABLE = crate::Reg<hcinterruptenable::HCINTERRUPTENABLE_SPEC>;
#[doc = "Controls the bits in the HcInterruptStatus register and indicates which events will generate a hardware interrupt"]
pub mod hcinterruptenable;
#[doc = "HCINTERRUPTDISABLE (rw) register accessor: an alias for `Reg<HCINTERRUPTDISABLE_SPEC>`"]
pub type HCINTERRUPTDISABLE = crate::Reg<hcinterruptdisable::HCINTERRUPTDISABLE_SPEC>;
#[doc = "The bits in this register are used to disable corresponding bits in the HCInterruptStatus register and in turn disable that event leading to hardware interrupt"]
pub mod hcinterruptdisable;
#[doc = "HCHCCA (rw) register accessor: an alias for `Reg<HCHCCA_SPEC>`"]
pub type HCHCCA = crate::Reg<hchcca::HCHCCA_SPEC>;
#[doc = "Contains the physical address of the host controller communication area"]
pub mod hchcca;
#[doc = "HCPERIODCURRENTED (rw) register accessor: an alias for `Reg<HCPERIODCURRENTED_SPEC>`"]
pub type HCPERIODCURRENTED = crate::Reg<hcperiodcurrented::HCPERIODCURRENTED_SPEC>;
#[doc = "Contains the physical address of the current isochronous or interrupt endpoint descriptor"]
pub mod hcperiodcurrented;
#[doc = "HCCONTROLHEADED (rw) register accessor: an alias for `Reg<HCCONTROLHEADED_SPEC>`"]
pub type HCCONTROLHEADED = crate::Reg<hccontrolheaded::HCCONTROLHEADED_SPEC>;
#[doc = "Contains the physical address of the first endpoint descriptor of the control list"]
pub mod hccontrolheaded;
#[doc = "HCCONTROLCURRENTED (rw) register accessor: an alias for `Reg<HCCONTROLCURRENTED_SPEC>`"]
pub type HCCONTROLCURRENTED = crate::Reg<hccontrolcurrented::HCCONTROLCURRENTED_SPEC>;
#[doc = "Contains the physical address of the current endpoint descriptor of the control list"]
pub mod hccontrolcurrented;
#[doc = "HCBULKHEADED (rw) register accessor: an alias for `Reg<HCBULKHEADED_SPEC>`"]
pub type HCBULKHEADED = crate::Reg<hcbulkheaded::HCBULKHEADED_SPEC>;
#[doc = "Contains the physical address of the first endpoint descriptor of the bulk list"]
pub mod hcbulkheaded;
#[doc = "HCBULKCURRENTED (rw) register accessor: an alias for `Reg<HCBULKCURRENTED_SPEC>`"]
pub type HCBULKCURRENTED = crate::Reg<hcbulkcurrented::HCBULKCURRENTED_SPEC>;
#[doc = "Contains the physical address of the current endpoint descriptor of the bulk list"]
pub mod hcbulkcurrented;
#[doc = "HCDONEHEAD (rw) register accessor: an alias for `Reg<HCDONEHEAD_SPEC>`"]
pub type HCDONEHEAD = crate::Reg<hcdonehead::HCDONEHEAD_SPEC>;
#[doc = "Contains the physical address of the last transfer descriptor added to the 'Done' queue"]
pub mod hcdonehead;
#[doc = "HCFMINTERVAL (rw) register accessor: an alias for `Reg<HCFMINTERVAL_SPEC>`"]
pub type HCFMINTERVAL = crate::Reg<hcfminterval::HCFMINTERVAL_SPEC>;
#[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun"]
pub mod hcfminterval;
#[doc = "HCFMREMAINING (rw) register accessor: an alias for `Reg<HCFMREMAINING_SPEC>`"]
pub type HCFMREMAINING = crate::Reg<hcfmremaining::HCFMREMAINING_SPEC>;
#[doc = "A 14-bit counter showing the bit time remaining in the current frame"]
pub mod hcfmremaining;
#[doc = "HCFMNUMBER (rw) register accessor: an alias for `Reg<HCFMNUMBER_SPEC>`"]
pub type HCFMNUMBER = crate::Reg<hcfmnumber::HCFMNUMBER_SPEC>;
#[doc = "Contains a 16-bit counter and provides the timing reference among events happening in the HC and the HCD"]
pub mod hcfmnumber;
#[doc = "HCPERIODICSTART (rw) register accessor: an alias for `Reg<HCPERIODICSTART_SPEC>`"]
pub type HCPERIODICSTART = crate::Reg<hcperiodicstart::HCPERIODICSTART_SPEC>;
#[doc = "Contains a programmable 14-bit value which determines the earliest time HC should start processing a periodic list"]
pub mod hcperiodicstart;
#[doc = "HCLSTHRESHOLD (rw) register accessor: an alias for `Reg<HCLSTHRESHOLD_SPEC>`"]
pub type HCLSTHRESHOLD = crate::Reg<hclsthreshold::HCLSTHRESHOLD_SPEC>;
#[doc = "Contains 11-bit value which is used by the HC to determine whether to commit to transfer a maximum of 8-byte LS packet before EOF"]
pub mod hclsthreshold;
#[doc = "HCRHDESCRIPTORA (rw) register accessor: an alias for `Reg<HCRHDESCRIPTORA_SPEC>`"]
pub type HCRHDESCRIPTORA = crate::Reg<hcrhdescriptora::HCRHDESCRIPTORA_SPEC>;
#[doc = "First of the two registers which describes the characteristics of the root hub"]
pub mod hcrhdescriptora;
#[doc = "HCRHDESCRIPTORB (rw) register accessor: an alias for `Reg<HCRHDESCRIPTORB_SPEC>`"]
pub type HCRHDESCRIPTORB = crate::Reg<hcrhdescriptorb::HCRHDESCRIPTORB_SPEC>;
#[doc = "Second of the two registers which describes the characteristics of the Root Hub"]
pub mod hcrhdescriptorb;
#[doc = "HCRHSTATUS (rw) register accessor: an alias for `Reg<HCRHSTATUS_SPEC>`"]
pub type HCRHSTATUS = crate::Reg<hcrhstatus::HCRHSTATUS_SPEC>;
#[doc = "This register is divided into two parts"]
pub mod hcrhstatus;
#[doc = "HCRHPORTSTATUS (rw) register accessor: an alias for `Reg<HCRHPORTSTATUS_SPEC>`"]
pub type HCRHPORTSTATUS = crate::Reg<hcrhportstatus::HCRHPORTSTATUS_SPEC>;
#[doc = "Controls and reports the port events on a per-port basis"]
pub mod hcrhportstatus;
#[doc = "PORTMODE (rw) register accessor: an alias for `Reg<PORTMODE_SPEC>`"]
pub type PORTMODE = crate::Reg<portmode::PORTMODE_SPEC>;
#[doc = "Controls the port if it is attached to the host block or the device block"]
pub mod portmode;
