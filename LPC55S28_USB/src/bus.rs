use crate::USB;
use usb_device::{
    bus::{PollResult, UsbBus as UB},
    endpoint::{EndpointAddress, EndpointType},
    UsbDirection,
};
use LPC55S28_PAC as pac;

pub struct FullSpeedUsbBus {
    peripheral: USB,
}

impl FullSpeedUsbBus {
    pub fn new(
        usb: pac::USBHSD,
        phy: pac::USBPHY,
        anactrl: &pac::ANACTRL,
        iocon: &pac::IOCON,
        syscon: &pac::SYSCON,
        pmc: &pac::PMC,
    ) -> Self {
        // Steps
        // • USB HS PHY:
        //     – Power on and initialize the USB HS PHY. See Section 45.3 “Basic configuration”.

        Self::setup_phy(&phy, anactrl, pmc);

        // • Pins: Configure the USB1 pins in the IOCON register block. See Table 815.
        // • Clocks:
        //     – Configure the CPU clock to a minimum frequency of 60 MHz, or higher.
        //     – Set the USB1_DEV and USB1_RAM bits in the AHBCLKCTRL2 register to enable
        //       USB device configuration and operation. See Section 4.5.19.
        // • Enable device control of the USB1 port:
        //     – Set USB1_HOST in AHBCLKCTRL2, to allow accesses to the port mode register. See Section 4.5.19.
        //     – Set DEV_ENABLE in the port mode register. See Section 43.5.19 for more details.
        //     – To save power, clear USB1_HOST in SYSAHBCLKCTRL2.
        // • Reset:
        //     – Reset the USB1 device and RAM control by toggling the USB1_DEV_RST and USB1_RAM_RST bits in PRESETCTRL2. See Section 4.5.9 for more details.
        // • Interrupts:
        //     – The USB1 has two interrupt slot assignments, one for the main interrupt, USB1_IRQ (USB1), and the other for USB1_NEEDCLK. See Table 8 “Connection of interrupt sources to the NVIC” Clear pending interrupts before enabling them.
        // • Configure the USB1 wake-up signal (see Section 44.7.6) if necessary.

        // Done

        Self {
            peripheral: USB { dev: usb, phy },
        }
    }

    fn setup_phy(phy: &pac::USBPHY, anactrl: &pac::ANACTRL, pmc: &pac::PMC) {
        // • Clocks:
        //     – The XO32M crystal oscillator and must be powered up and configured to one of the
        //       supported USB1_PHY reference clock frequencies of 16 MHz, 19.2 MHz, 20 MHz,
        //       24 MHz or 32 MHz. Set the ENABLE_USB_HS_CLK_OUT bit of the
        //       XO32M_CTRL register to enable the XO32M clock output to the USB1_PHY. See
        //       Section 11.5.5.
        //     – Enable the 32k_osc clock to provide the 32 kHz clock to the USB1_PHY.
        //     – Set USB1_PHY in AHBCLKCTRL2, to enable clock to the USB1_PHY’s APB
        //       register interface.

        // • Power: Clear the following bits to power up the USB1_PHY: See the PDRONCFG0
        //   register for more details in Section 13.4.9.
        //     – PDEN_USB1_PHY to power up the USB1_PHY.
        //     – PDEN_LDOUSBHS to power up the USB1_PHY LDO.
        // • Interrupt:
        //     – If desired, enable the USB1_PHY interrupt. See: Table 8 “Connection of interrupt
        //       sources to the NVIC”. Clear pending interrupts before enabling them.
        // • Reset:
        //     – Toggle the USB1_PHY_RST bit in PRESETCTRL2 to reset the PHY’s APB
        //       registers.
        // • Interrupt:
        //     – Enable the USB1_PHY interrupt. See: Table 8 “Connection of interrupt sources to
        //       the NVIC”. Clear pending interrupts before enabling them.
        // • Initial configuration: The following pseudo code gives an example of initializing the
        //   PHY control registers:
        //
        // USB1_PHY_CTRL_CLR = SFTRST;
        //
        // // Set the PLL_DIV_SEL field, USB1_PHY_PLL_SIC[24:22], to DIV_VAL
        // // DIV_VAL should be set based on input frequency from XO32M.
        // USB1_PHY_PLL_SIC = (USB1_PHY_PLL_SIC & ~(0x7 << 22)) | (DIV_VAL << 22);
        // USB1_PHY_PLL_SIC_SET = PLL_REG_EN;
        // USB1_PHY_PLL_SIC_CLR = PLL_BYPASS;
        //
        // // add code to wait more than 15 us here
        //
        // USB1_PHY_PLL_SIC_SET = PLL_POWER;
        // USB1_PHY_PLL_SIC_SET = PLL_EN_USB_CLKS;
        //
        // // enable auto power down of PHY PLL during suspend
        // USB1_PHY_PLL_SIC_SET = PLL_MISC2_CONTROL0;
        // USB1_PHY_CTRL_CLR = CLKGATE;
        // USB1_PHY_PWD = 0x0;
        // USB1_PHY_CTRL_SET = ENUTMILEVEL3;
        // USB1_PHY_CTRL_SET = ENUTMILEVEL2;
        // USB1_PHY_CTRL_SET = ENAUTOCLR_CLKGATE;
        //
        // // enable using 32kHz clock for sending host resume
        // USB1_PHY_CTRL_SET = AUTORESUME_EN;
        // USB1_PHY_CTRL_SET = ENAUTOCLR_PHY_PWD;
        // USB1_PHY_CTRL_SET = ENHOSTDISCONDETECT;
    }
}

impl UB for FullSpeedUsbBus {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> usb_device::Result<EndpointAddress> {
        todo!()
    }

    fn enable(&mut self) {
        todo!()
    }

    fn reset(&self) {
        todo!()
    }

    fn set_device_address(&self, addr: u8) {
        todo!()
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> usb_device::Result<usize> {
        todo!()
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> usb_device::Result<usize> {
        todo!()
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        todo!()
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        todo!()
    }

    fn suspend(&self) {
        todo!()
    }

    fn resume(&self) {
        todo!()
    }

    fn poll(&self) -> PollResult {
        todo!()
    }

    fn force_reset(&self) -> usb_device::Result<()> {
        Err(usb_device::UsbError::Unsupported)
    }

    const QUIRK_SET_ADDRESS_BEFORE_STATUS: bool = false;
}
