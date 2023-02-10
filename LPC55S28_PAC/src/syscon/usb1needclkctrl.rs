#[doc = "Register `USB1NEEDCLKCTRL` reader"]
pub struct R(crate::R<USB1NEEDCLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1NEEDCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1NEEDCLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1NEEDCLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1NEEDCLKCTRL` writer"]
pub struct W(crate::W<USB1NEEDCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1NEEDCLKCTRL_SPEC>;
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
impl From<crate::W<USB1NEEDCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1NEEDCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AP_HS_DEV_NEEDCLK` reader - USB1 Device need_clock signal control:"]
pub type AP_HS_DEV_NEEDCLK_R = crate::BitReader<AP_HS_DEV_NEEDCLK_A>;
#[doc = "USB1 Device need_clock signal control:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AP_HS_DEV_NEEDCLK_A {
    #[doc = "0: HOST_NEEDCLK is under hardware control."]
    HW_CTRL = 0,
    #[doc = "1: HOST_NEEDCLK is forced high."]
    FORCED = 1,
}
impl From<AP_HS_DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_HS_DEV_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl AP_HS_DEV_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_HS_DEV_NEEDCLK_A {
        match self.bits {
            false => AP_HS_DEV_NEEDCLK_A::HW_CTRL,
            true => AP_HS_DEV_NEEDCLK_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == AP_HS_DEV_NEEDCLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == AP_HS_DEV_NEEDCLK_A::FORCED
    }
}
#[doc = "Field `AP_HS_DEV_NEEDCLK` writer - USB1 Device need_clock signal control:"]
pub type AP_HS_DEV_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1NEEDCLKCTRL_SPEC, AP_HS_DEV_NEEDCLK_A, O>;
impl<'a, const O: u8> AP_HS_DEV_NEEDCLK_W<'a, O> {
    #[doc = "HOST_NEEDCLK is under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_HS_DEV_NEEDCLK_A::HW_CTRL)
    }
    #[doc = "HOST_NEEDCLK is forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_HS_DEV_NEEDCLK_A::FORCED)
    }
}
#[doc = "Field `POL_HS_DEV_NEEDCLK` reader - USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
pub type POL_HS_DEV_NEEDCLK_R = crate::BitReader<POL_HS_DEV_NEEDCLK_A>;
#[doc = "USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_HS_DEV_NEEDCLK_A {
    #[doc = "0: Falling edge of DEV_NEEDCLK triggers wake-up."]
    FALLING = 0,
    #[doc = "1: Rising edge of DEV_NEEDCLK triggers wake-up."]
    RISING = 1,
}
impl From<POL_HS_DEV_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_HS_DEV_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_HS_DEV_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_HS_DEV_NEEDCLK_A {
        match self.bits {
            false => POL_HS_DEV_NEEDCLK_A::FALLING,
            true => POL_HS_DEV_NEEDCLK_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == POL_HS_DEV_NEEDCLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == POL_HS_DEV_NEEDCLK_A::RISING
    }
}
#[doc = "Field `POL_HS_DEV_NEEDCLK` writer - USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
pub type POL_HS_DEV_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1NEEDCLKCTRL_SPEC, POL_HS_DEV_NEEDCLK_A, O>;
impl<'a, const O: u8> POL_HS_DEV_NEEDCLK_W<'a, O> {
    #[doc = "Falling edge of DEV_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_HS_DEV_NEEDCLK_A::FALLING)
    }
    #[doc = "Rising edge of DEV_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_HS_DEV_NEEDCLK_A::RISING)
    }
}
#[doc = "Field `AP_HS_HOST_NEEDCLK` reader - USB1 Host need clock signal control:"]
pub type AP_HS_HOST_NEEDCLK_R = crate::BitReader<AP_HS_HOST_NEEDCLK_A>;
#[doc = "USB1 Host need clock signal control:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AP_HS_HOST_NEEDCLK_A {
    #[doc = "0: HOST_NEEDCLK is under hardware control."]
    HW_CTRL = 0,
    #[doc = "1: HOST_NEEDCLK is forced high."]
    FORCED = 1,
}
impl From<AP_HS_HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: AP_HS_HOST_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl AP_HS_HOST_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AP_HS_HOST_NEEDCLK_A {
        match self.bits {
            false => AP_HS_HOST_NEEDCLK_A::HW_CTRL,
            true => AP_HS_HOST_NEEDCLK_A::FORCED,
        }
    }
    #[doc = "Checks if the value of the field is `HW_CTRL`"]
    #[inline(always)]
    pub fn is_hw_ctrl(&self) -> bool {
        *self == AP_HS_HOST_NEEDCLK_A::HW_CTRL
    }
    #[doc = "Checks if the value of the field is `FORCED`"]
    #[inline(always)]
    pub fn is_forced(&self) -> bool {
        *self == AP_HS_HOST_NEEDCLK_A::FORCED
    }
}
#[doc = "Field `AP_HS_HOST_NEEDCLK` writer - USB1 Host need clock signal control:"]
pub type AP_HS_HOST_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1NEEDCLKCTRL_SPEC, AP_HS_HOST_NEEDCLK_A, O>;
impl<'a, const O: u8> AP_HS_HOST_NEEDCLK_W<'a, O> {
    #[doc = "HOST_NEEDCLK is under hardware control."]
    #[inline(always)]
    pub fn hw_ctrl(self) -> &'a mut W {
        self.variant(AP_HS_HOST_NEEDCLK_A::HW_CTRL)
    }
    #[doc = "HOST_NEEDCLK is forced high."]
    #[inline(always)]
    pub fn forced(self) -> &'a mut W {
        self.variant(AP_HS_HOST_NEEDCLK_A::FORCED)
    }
}
#[doc = "Field `POL_HS_HOST_NEEDCLK` reader - USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
pub type POL_HS_HOST_NEEDCLK_R = crate::BitReader<POL_HS_HOST_NEEDCLK_A>;
#[doc = "USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_HS_HOST_NEEDCLK_A {
    #[doc = "0: Falling edge of HOST_NEEDCLK triggers wake-up."]
    FALLING = 0,
    #[doc = "1: Rising edge of HOST_NEEDCLK triggers wake-up."]
    RISING = 1,
}
impl From<POL_HS_HOST_NEEDCLK_A> for bool {
    #[inline(always)]
    fn from(variant: POL_HS_HOST_NEEDCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_HS_HOST_NEEDCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_HS_HOST_NEEDCLK_A {
        match self.bits {
            false => POL_HS_HOST_NEEDCLK_A::FALLING,
            true => POL_HS_HOST_NEEDCLK_A::RISING,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == POL_HS_HOST_NEEDCLK_A::FALLING
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == POL_HS_HOST_NEEDCLK_A::RISING
    }
}
#[doc = "Field `POL_HS_HOST_NEEDCLK` writer - USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
pub type POL_HS_HOST_NEEDCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1NEEDCLKCTRL_SPEC, POL_HS_HOST_NEEDCLK_A, O>;
impl<'a, const O: u8> POL_HS_HOST_NEEDCLK_W<'a, O> {
    #[doc = "Falling edge of HOST_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(POL_HS_HOST_NEEDCLK_A::FALLING)
    }
    #[doc = "Rising edge of HOST_NEEDCLK triggers wake-up."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(POL_HS_HOST_NEEDCLK_A::RISING)
    }
}
#[doc = "Field `HS_DEV_WAKEUP_N` reader - Software override of device controller PHY wake up logic."]
pub type HS_DEV_WAKEUP_N_R = crate::BitReader<HS_DEV_WAKEUP_N_A>;
#[doc = "Software override of device controller PHY wake up logic.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HS_DEV_WAKEUP_N_A {
    #[doc = "0: Forces USB1_PHY to wake-up."]
    FORCE_WUP = 0,
    #[doc = "1: Normal USB1_PHY behavior."]
    NORMAL_WUP = 1,
}
impl From<HS_DEV_WAKEUP_N_A> for bool {
    #[inline(always)]
    fn from(variant: HS_DEV_WAKEUP_N_A) -> Self {
        variant as u8 != 0
    }
}
impl HS_DEV_WAKEUP_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS_DEV_WAKEUP_N_A {
        match self.bits {
            false => HS_DEV_WAKEUP_N_A::FORCE_WUP,
            true => HS_DEV_WAKEUP_N_A::NORMAL_WUP,
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_WUP`"]
    #[inline(always)]
    pub fn is_force_wup(&self) -> bool {
        *self == HS_DEV_WAKEUP_N_A::FORCE_WUP
    }
    #[doc = "Checks if the value of the field is `NORMAL_WUP`"]
    #[inline(always)]
    pub fn is_normal_wup(&self) -> bool {
        *self == HS_DEV_WAKEUP_N_A::NORMAL_WUP
    }
}
#[doc = "Field `HS_DEV_WAKEUP_N` writer - Software override of device controller PHY wake up logic."]
pub type HS_DEV_WAKEUP_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1NEEDCLKCTRL_SPEC, HS_DEV_WAKEUP_N_A, O>;
impl<'a, const O: u8> HS_DEV_WAKEUP_N_W<'a, O> {
    #[doc = "Forces USB1_PHY to wake-up."]
    #[inline(always)]
    pub fn force_wup(self) -> &'a mut W {
        self.variant(HS_DEV_WAKEUP_N_A::FORCE_WUP)
    }
    #[doc = "Normal USB1_PHY behavior."]
    #[inline(always)]
    pub fn normal_wup(self) -> &'a mut W {
        self.variant(HS_DEV_WAKEUP_N_A::NORMAL_WUP)
    }
}
impl R {
    #[doc = "Bit 0 - USB1 Device need_clock signal control:"]
    #[inline(always)]
    pub fn ap_hs_dev_needclk(&self) -> AP_HS_DEV_NEEDCLK_R {
        AP_HS_DEV_NEEDCLK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
    #[inline(always)]
    pub fn pol_hs_dev_needclk(&self) -> POL_HS_DEV_NEEDCLK_R {
        POL_HS_DEV_NEEDCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB1 Host need clock signal control:"]
    #[inline(always)]
    pub fn ap_hs_host_needclk(&self) -> AP_HS_HOST_NEEDCLK_R {
        AP_HS_HOST_NEEDCLK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
    #[inline(always)]
    pub fn pol_hs_host_needclk(&self) -> POL_HS_HOST_NEEDCLK_R {
        POL_HS_HOST_NEEDCLK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software override of device controller PHY wake up logic."]
    #[inline(always)]
    pub fn hs_dev_wakeup_n(&self) -> HS_DEV_WAKEUP_N_R {
        HS_DEV_WAKEUP_N_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB1 Device need_clock signal control:"]
    #[inline(always)]
    #[must_use]
    pub fn ap_hs_dev_needclk(&mut self) -> AP_HS_DEV_NEEDCLK_W<0> {
        AP_HS_DEV_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 1 - USB1 device need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt:"]
    #[inline(always)]
    #[must_use]
    pub fn pol_hs_dev_needclk(&mut self) -> POL_HS_DEV_NEEDCLK_W<1> {
        POL_HS_DEV_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 2 - USB1 Host need clock signal control:"]
    #[inline(always)]
    #[must_use]
    pub fn ap_hs_host_needclk(&mut self) -> AP_HS_HOST_NEEDCLK_W<2> {
        AP_HS_HOST_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 3 - USB1 host need clock polarity for triggering the USB1_NEEDCLK wake-up interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pol_hs_host_needclk(&mut self) -> POL_HS_HOST_NEEDCLK_W<3> {
        POL_HS_HOST_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 4 - Software override of device controller PHY wake up logic."]
    #[inline(always)]
    #[must_use]
    pub fn hs_dev_wakeup_n(&mut self) -> HS_DEV_WAKEUP_N_W<4> {
        HS_DEV_WAKEUP_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB1 need clock control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1needclkctrl](index.html) module"]
pub struct USB1NEEDCLKCTRL_SPEC;
impl crate::RegisterSpec for USB1NEEDCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1needclkctrl::R](R) reader structure"]
impl crate::Readable for USB1NEEDCLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1needclkctrl::W](W) writer structure"]
impl crate::Writable for USB1NEEDCLKCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1NEEDCLKCTRL to value 0x10"]
impl crate::Resettable for USB1NEEDCLKCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
