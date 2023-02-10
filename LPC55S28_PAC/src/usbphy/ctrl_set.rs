#[doc = "Register `CTRL_SET` reader"]
pub struct R(crate::R<CTRL_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_SET` writer"]
pub struct W(crate::W<CTRL_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRL_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENHOSTDISCONDETECT` reader - For host mode, enables high-speed disconnect detector"]
pub type ENHOSTDISCONDETECT_R = crate::BitReader<bool>;
#[doc = "Field `ENHOSTDISCONDETECT` writer - For host mode, enables high-speed disconnect detector"]
pub type ENHOSTDISCONDETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENIRQHOSTDISCON` reader - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
pub type ENIRQHOSTDISCON_R = crate::BitReader<bool>;
#[doc = "Field `ENIRQHOSTDISCON` writer - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
pub type ENIRQHOSTDISCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `HOSTDISCONDETECT_IRQ` reader - Indicates that the device has disconnected in High-Speed mode"]
pub type HOSTDISCONDETECT_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `HOSTDISCONDETECT_IRQ` writer - Indicates that the device has disconnected in High-Speed mode"]
pub type HOSTDISCONDETECT_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENDEVPLUGINDET` reader - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
pub type ENDEVPLUGINDET_R = crate::BitReader<ENDEVPLUGINDET_A>;
#[doc = "Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDEVPLUGINDET_A {
    #[doc = "0: Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    VALUE0 = 0,
    #[doc = "1: Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    VALUE1 = 1,
}
impl From<ENDEVPLUGINDET_A> for bool {
    #[inline(always)]
    fn from(variant: ENDEVPLUGINDET_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDEVPLUGINDET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDEVPLUGINDET_A {
        match self.bits {
            false => ENDEVPLUGINDET_A::VALUE0,
            true => ENDEVPLUGINDET_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == ENDEVPLUGINDET_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == ENDEVPLUGINDET_A::VALUE1
    }
}
#[doc = "Field `ENDEVPLUGINDET` writer - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
pub type ENDEVPLUGINDET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SET_SPEC, ENDEVPLUGINDET_A, O>;
impl<'a, const O: u8> ENDEVPLUGINDET_W<'a, O> {
    #[doc = "Disables 200kohm pullup resistors on USB_DP and USB_DM pins (Default)"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(ENDEVPLUGINDET_A::VALUE0)
    }
    #[doc = "Enables 200kohm pullup resistors on USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(ENDEVPLUGINDET_A::VALUE1)
    }
}
#[doc = "Field `DEVPLUGIN_POLARITY` reader - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
pub type DEVPLUGIN_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `DEVPLUGIN_POLARITY` writer - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
pub type DEVPLUGIN_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `RESUMEIRQSTICKY` reader - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
pub type RESUMEIRQSTICKY_R = crate::BitReader<bool>;
#[doc = "Field `RESUMEIRQSTICKY` writer - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
pub type RESUMEIRQSTICKY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENIRQRESUMEDETECT` reader - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
pub type ENIRQRESUMEDETECT_R = crate::BitReader<bool>;
#[doc = "Field `ENIRQRESUMEDETECT` writer - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
pub type ENIRQRESUMEDETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `RESUME_IRQ` reader - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
pub type RESUME_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `RESUME_IRQ` writer - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
pub type RESUME_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `DEVPLUGIN_IRQ` reader - Indicates that the device is connected"]
pub type DEVPLUGIN_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `DEVPLUGIN_IRQ` writer - Indicates that the device is connected"]
pub type DEVPLUGIN_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENUTMILEVEL2` reader - Enables UTMI+ Level 2 operation for the USB HS PHY"]
pub type ENUTMILEVEL2_R = crate::BitReader<bool>;
#[doc = "Field `ENUTMILEVEL2` writer - Enables UTMI+ Level 2 operation for the USB HS PHY"]
pub type ENUTMILEVEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENUTMILEVEL3` reader - Enables UTMI+ Level 3 operation for the USB HS PHY"]
pub type ENUTMILEVEL3_R = crate::BitReader<bool>;
#[doc = "Field `ENUTMILEVEL3` writer - Enables UTMI+ Level 3 operation for the USB HS PHY"]
pub type ENUTMILEVEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENIRQWAKEUP` reader - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
pub type ENIRQWAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `ENIRQWAKEUP` writer - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
pub type ENIRQWAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `WAKEUP_IRQ` reader - Wake-up IRQ: Indicates that there is a wak-eup event"]
pub type WAKEUP_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP_IRQ` writer - Wake-up IRQ: Indicates that there is a wak-eup event"]
pub type WAKEUP_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `AUTORESUME_EN` reader - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
pub type AUTORESUME_EN_R = crate::BitReader<bool>;
#[doc = "Field `AUTORESUME_EN` writer - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
pub type AUTORESUME_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENAUTOCLR_CLKGATE` reader - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
pub type ENAUTOCLR_CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `ENAUTOCLR_CLKGATE` writer - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
pub type ENAUTOCLR_CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENAUTOCLR_PHY_PWD` reader - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
pub type ENAUTOCLR_PHY_PWD_R = crate::BitReader<bool>;
#[doc = "Field `ENAUTOCLR_PHY_PWD` writer - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
pub type ENAUTOCLR_PHY_PWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENDPDMCHG_WKUP` reader - Enable DP DM change wake-up: Not for customer use"]
pub type ENDPDMCHG_WKUP_R = crate::BitReader<bool>;
#[doc = "Field `ENDPDMCHG_WKUP` writer - Enable DP DM change wake-up: Not for customer use"]
pub type ENDPDMCHG_WKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENVBUSCHG_WKUP` reader - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
pub type ENVBUSCHG_WKUP_R = crate::BitReader<bool>;
#[doc = "Field `ENVBUSCHG_WKUP` writer - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
pub type ENVBUSCHG_WKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENAUTOCLR_USBCLKGATE` reader - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub type ENAUTOCLR_USBCLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `ENAUTOCLR_USBCLKGATE` writer - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub type ENAUTOCLR_USBCLKGATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENAUTOSET_USBCLKS` reader - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub type ENAUTOSET_USBCLKS_R = crate::BitReader<bool>;
#[doc = "Field `ENAUTOSET_USBCLKS` writer - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
pub type ENAUTOSET_USBCLKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `HOST_FORCE_LS_SE0` reader - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
pub type HOST_FORCE_LS_SE0_R = crate::BitReader<bool>;
#[doc = "Field `HOST_FORCE_LS_SE0` writer - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
pub type HOST_FORCE_LS_SE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `UTMI_SUSPENDM` reader - Used by the PHY to indicate a powered-down state"]
pub type UTMI_SUSPENDM_R = crate::BitReader<bool>;
#[doc = "Field `CLKGATE` reader - Gate UTMI Clocks"]
pub type CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `CLKGATE` writer - Gate UTMI Clocks"]
pub type CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `SFTRST` reader - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
pub type SFTRST_R = crate::BitReader<bool>;
#[doc = "Field `SFTRST` writer - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
pub type SFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub fn enhostdiscondetect(&self) -> ENHOSTDISCONDETECT_R {
        ENHOSTDISCONDETECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub fn enirqhostdiscon(&self) -> ENIRQHOSTDISCON_R {
        ENIRQHOSTDISCON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&self) -> HOSTDISCONDETECT_IRQ_R {
        HOSTDISCONDETECT_IRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    pub fn endevplugindet(&self) -> ENDEVPLUGINDET_R {
        ENDEVPLUGINDET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub fn devplugin_polarity(&self) -> DEVPLUGIN_POLARITY_R {
        DEVPLUGIN_POLARITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub fn resumeirqsticky(&self) -> RESUMEIRQSTICKY_R {
        RESUMEIRQSTICKY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub fn enirqresumedetect(&self) -> ENIRQRESUMEDETECT_R {
        ENIRQRESUMEDETECT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub fn resume_irq(&self) -> RESUME_IRQ_R {
        RESUME_IRQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    pub fn devplugin_irq(&self) -> DEVPLUGIN_IRQ_R {
        DEVPLUGIN_IRQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel2(&self) -> ENUTMILEVEL2_R {
        ENUTMILEVEL2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    pub fn enutmilevel3(&self) -> ENUTMILEVEL3_R {
        ENUTMILEVEL3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    pub fn enirqwakeup(&self) -> ENIRQWAKEUP_R {
        ENIRQWAKEUP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[inline(always)]
    pub fn wakeup_irq(&self) -> WAKEUP_IRQ_R {
        WAKEUP_IRQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    pub fn autoresume_en(&self) -> AUTORESUME_EN_R {
        AUTORESUME_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_clkgate(&self) -> ENAUTOCLR_CLKGATE_R {
        ENAUTOCLR_CLKGATE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&self) -> ENAUTOCLR_PHY_PWD_R {
        ENAUTOCLR_PHY_PWD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable DP DM change wake-up: Not for customer use"]
    #[inline(always)]
    pub fn endpdmchg_wkup(&self) -> ENDPDMCHG_WKUP_R {
        ENDPDMCHG_WKUP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[inline(always)]
    pub fn envbuschg_wkup(&self) -> ENVBUSCHG_WKUP_R {
        ENVBUSCHG_WKUP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub fn enautoclr_usbclkgate(&self) -> ENAUTOCLR_USBCLKGATE_R {
        ENAUTOCLR_USBCLKGATE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    pub fn enautoset_usbclks(&self) -> ENAUTOSET_USBCLKS_R {
        ENAUTOSET_USBCLKS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&self) -> HOST_FORCE_LS_SE0_R {
        HOST_FORCE_LS_SE0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub fn utmi_suspendm(&self) -> UTMI_SUSPENDM_R {
        UTMI_SUSPENDM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    #[must_use]
    pub fn enhostdiscondetect(&mut self) -> ENHOSTDISCONDETECT_W<1> {
        ENHOSTDISCONDETECT_W::new(self)
    }
    #[doc = "Bit 2 - Enable IRQ for Host disconnect: Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    #[must_use]
    pub fn enirqhostdiscon(&mut self) -> ENIRQHOSTDISCON_W<2> {
        ENIRQHOSTDISCON_W::new(self)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in High-Speed mode"]
    #[inline(always)]
    #[must_use]
    pub fn hostdiscondetect_irq(&mut self) -> HOSTDISCONDETECT_IRQ_W<3> {
        HOSTDISCONDETECT_IRQ_W::new(self)
    }
    #[doc = "Bit 4 - Enables non-standard resistive plugged-in detection This bit field controls connection of nominal 200kohm resistors to both the USB_DP and USB_DM pins as one method of detecting when a USB cable is attached in device mode"]
    #[inline(always)]
    #[must_use]
    pub fn endevplugindet(&mut self) -> ENDEVPLUGINDET_W<4> {
        ENDEVPLUGINDET_W::new(self)
    }
    #[doc = "Bit 5 - Device plugin polarity: For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    #[must_use]
    pub fn devplugin_polarity(&mut self) -> DEVPLUGIN_POLARITY_W<5> {
        DEVPLUGIN_POLARITY_W::new(self)
    }
    #[doc = "Bit 8 - Resume IRQ: Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    #[must_use]
    pub fn resumeirqsticky(&mut self) -> RESUMEIRQSTICKY_W<8> {
        RESUMEIRQSTICKY_W::new(self)
    }
    #[doc = "Bit 9 - Enable IRQ Resume detect: Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    #[must_use]
    pub fn enirqresumedetect(&mut self) -> ENIRQRESUMEDETECT_W<9> {
        ENIRQRESUMEDETECT_W::new(self)
    }
    #[doc = "Bit 10 - Resume IRQ: Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    #[must_use]
    pub fn resume_irq(&mut self) -> RESUME_IRQ_W<10> {
        RESUME_IRQ_W::new(self)
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    #[must_use]
    pub fn devplugin_irq(&mut self) -> DEVPLUGIN_IRQ_W<12> {
        DEVPLUGIN_IRQ_W::new(self)
    }
    #[doc = "Bit 14 - Enables UTMI+ Level 2 operation for the USB HS PHY"]
    #[inline(always)]
    #[must_use]
    pub fn enutmilevel2(&mut self) -> ENUTMILEVEL2_W<14> {
        ENUTMILEVEL2_W::new(self)
    }
    #[doc = "Bit 15 - Enables UTMI+ Level 3 operation for the USB HS PHY"]
    #[inline(always)]
    #[must_use]
    pub fn enutmilevel3(&mut self) -> ENUTMILEVEL3_W<15> {
        ENUTMILEVEL3_W::new(self)
    }
    #[doc = "Bit 16 - Enable wake-up IRQ: Enables interrupt for the wake-up events."]
    #[inline(always)]
    #[must_use]
    pub fn enirqwakeup(&mut self) -> ENIRQWAKEUP_W<16> {
        ENIRQWAKEUP_W::new(self)
    }
    #[doc = "Bit 17 - Wake-up IRQ: Indicates that there is a wak-eup event"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_irq(&mut self) -> WAKEUP_IRQ_W<17> {
        WAKEUP_IRQ_W::new(self)
    }
    #[doc = "Bit 18 - Enable the auto resume feature, when set, HW will use 32KHz clock to send Resume to respond to the device remote wakeup(for host mode only)"]
    #[inline(always)]
    #[must_use]
    pub fn autoresume_en(&mut self) -> AUTORESUME_EN_W<18> {
        AUTORESUME_EN_W::new(self)
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn enautoclr_clkgate(&mut self) -> ENAUTOCLR_CLKGATE_W<19> {
        ENAUTOCLR_CLKGATE_W::new(self)
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn enautoclr_phy_pwd(&mut self) -> ENAUTOCLR_PHY_PWD_W<20> {
        ENAUTOCLR_PHY_PWD_W::new(self)
    }
    #[doc = "Bit 21 - Enable DP DM change wake-up: Not for customer use"]
    #[inline(always)]
    #[must_use]
    pub fn endpdmchg_wkup(&mut self) -> ENDPDMCHG_WKUP_W<21> {
        ENDPDMCHG_WKUP_W::new(self)
    }
    #[doc = "Bit 23 - Enable VBUS change wake-up: Enables the feature to wake-up USB if VBUS is toggled when USB is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn envbuschg_wkup(&mut self) -> ENVBUSCHG_WKUP_W<23> {
        ENVBUSCHG_WKUP_W::new(self)
    }
    #[doc = "Bit 25 - Enable auto-clear USB Clock gate: Enables the feature to auto-clear the USB0_CLKGATE/USB1_CLKGATE register bit in HW_DIGCTL_CTRL if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn enautoclr_usbclkgate(&mut self) -> ENAUTOCLR_USBCLKGATE_W<25> {
        ENAUTOCLR_USBCLKGATE_W::new(self)
    }
    #[doc = "Bit 26 - Enable auto-set of USB clocks: Enables the feature to auto-clear the EN_USB_CLKS register bits in HW_CLKCTRL_PLL1CTRL0/HW_CLKCTRL_P LL1CTRL1 if there is wake-up event on USB0/USB1 while USB0/USB1 is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn enautoset_usbclks(&mut self) -> ENAUTOSET_USBCLKS_W<26> {
        ENAUTOSET_USBCLKS_W::new(self)
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with low-speed timing"]
    #[inline(always)]
    #[must_use]
    pub fn host_force_ls_se0(&mut self) -> HOST_FORCE_LS_SE0_W<28> {
        HOST_FORCE_LS_SE0_W::new(self)
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate(&mut self) -> CLKGATE_W<30> {
        CLKGATE_W::new(self)
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the PWD, TX, RX, and CTRL registers"]
    #[inline(always)]
    #[must_use]
    pub fn sftrst(&mut self) -> SFTRST_W<31> {
        SFTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_set](index.html) module"]
pub struct CTRL_SET_SPEC;
impl crate::RegisterSpec for CTRL_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_set::R](R) reader structure"]
impl crate::Readable for CTRL_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_set::W](W) writer structure"]
impl crate::Writable for CTRL_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_SET to value 0xc000_0000"]
impl crate::Resettable for CTRL_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000_0000;
}
