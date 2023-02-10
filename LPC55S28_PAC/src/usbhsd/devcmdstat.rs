#[doc = "Register `DEVCMDSTAT` reader"]
pub struct R(crate::R<DEVCMDSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVCMDSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVCMDSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVCMDSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVCMDSTAT` writer"]
pub struct W(crate::W<DEVCMDSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVCMDSTAT_SPEC>;
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
impl From<crate::W<DEVCMDSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVCMDSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEV_ADDR` reader - USB device address."]
pub type DEV_ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEV_ADDR` writer - USB device address."]
pub type DEV_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVCMDSTAT_SPEC, u8, u8, 7, O>;
#[doc = "Field `DEV_EN` reader - USB device enable."]
pub type DEV_EN_R = crate::BitReader<bool>;
#[doc = "Field `DEV_EN` writer - USB device enable."]
pub type DEV_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `SETUP` reader - SETUP token received."]
pub type SETUP_R = crate::BitReader<bool>;
#[doc = "Field `SETUP` writer - SETUP token received."]
pub type SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `FORCE_NEEDCLK` reader - Forces the NEEDCLK output to always be on:."]
pub type FORCE_NEEDCLK_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_NEEDCLK` writer - Forces the NEEDCLK output to always be on:."]
pub type FORCE_NEEDCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `LPM_SUP` reader - LPM Supported:."]
pub type LPM_SUP_R = crate::BitReader<bool>;
#[doc = "Field `LPM_SUP` writer - LPM Supported:."]
pub type LPM_SUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `INTONNAK_AO` reader - Interrupt on NAK for interrupt and bulk OUT EP:."]
pub type INTONNAK_AO_R = crate::BitReader<bool>;
#[doc = "Field `INTONNAK_AO` writer - Interrupt on NAK for interrupt and bulk OUT EP:."]
pub type INTONNAK_AO_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `INTONNAK_AI` reader - Interrupt on NAK for interrupt and bulk IN EP:."]
pub type INTONNAK_AI_R = crate::BitReader<bool>;
#[doc = "Field `INTONNAK_AI` writer - Interrupt on NAK for interrupt and bulk IN EP:."]
pub type INTONNAK_AI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `INTONNAK_CO` reader - Interrupt on NAK for control OUT EP:."]
pub type INTONNAK_CO_R = crate::BitReader<bool>;
#[doc = "Field `INTONNAK_CO` writer - Interrupt on NAK for control OUT EP:."]
pub type INTONNAK_CO_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `INTONNAK_CI` reader - Interrupt on NAK for control IN EP:."]
pub type INTONNAK_CI_R = crate::BitReader<bool>;
#[doc = "Field `INTONNAK_CI` writer - Interrupt on NAK for control IN EP:."]
pub type INTONNAK_CI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `DCON` reader - Device status - connect."]
pub type DCON_R = crate::BitReader<bool>;
#[doc = "Field `DCON` writer - Device status - connect."]
pub type DCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `DSUS` reader - Device status - suspend."]
pub type DSUS_R = crate::BitReader<bool>;
#[doc = "Field `DSUS` writer - Device status - suspend."]
pub type DSUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `LPM_SUS` reader - Device status - LPM Suspend."]
pub type LPM_SUS_R = crate::BitReader<bool>;
#[doc = "Field `LPM_SUS` writer - Device status - LPM Suspend."]
pub type LPM_SUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `LPM_REWP` reader - LPM Remote Wake-up Enabled by USB host."]
pub type LPM_REWP_R = crate::BitReader<bool>;
#[doc = "Field `Speed` reader - This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use)."]
pub type SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DCON_C` reader - Device status - connect change."]
pub type DCON_C_R = crate::BitReader<bool>;
#[doc = "Field `DCON_C` writer - Device status - connect change."]
pub type DCON_C_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `DSUS_C` reader - Device status - suspend change."]
pub type DSUS_C_R = crate::BitReader<bool>;
#[doc = "Field `DSUS_C` writer - Device status - suspend change."]
pub type DSUS_C_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `DRES_C` reader - Device status - reset change."]
pub type DRES_C_R = crate::BitReader<bool>;
#[doc = "Field `DRES_C` writer - Device status - reset change."]
pub type DRES_C_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVCMDSTAT_SPEC, bool, O>;
#[doc = "Field `VBUS_DEBOUNCED` reader - This bit indicates if VBUS is detected or not."]
pub type VBUS_DEBOUNCED_R = crate::BitReader<bool>;
#[doc = "Field `PHY_TEST_MODE` reader - This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification"]
pub type PHY_TEST_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PHY_TEST_MODE` writer - This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification"]
pub type PHY_TEST_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEVCMDSTAT_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:6 - USB device address."]
    #[inline(always)]
    pub fn dev_addr(&self) -> DEV_ADDR_R {
        DEV_ADDR_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - USB device enable."]
    #[inline(always)]
    pub fn dev_en(&self) -> DEV_EN_R {
        DEV_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SETUP token received."]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Forces the NEEDCLK output to always be on:."]
    #[inline(always)]
    pub fn force_needclk(&self) -> FORCE_NEEDCLK_R {
        FORCE_NEEDCLK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - LPM Supported:."]
    #[inline(always)]
    pub fn lpm_sup(&self) -> LPM_SUP_R {
        LPM_SUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[inline(always)]
    pub fn intonnak_ao(&self) -> INTONNAK_AO_R {
        INTONNAK_AO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP:."]
    #[inline(always)]
    pub fn intonnak_ai(&self) -> INTONNAK_AI_R {
        INTONNAK_AI_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP:."]
    #[inline(always)]
    pub fn intonnak_co(&self) -> INTONNAK_CO_R {
        INTONNAK_CO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP:."]
    #[inline(always)]
    pub fn intonnak_ci(&self) -> INTONNAK_CI_R {
        INTONNAK_CI_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Device status - connect."]
    #[inline(always)]
    pub fn dcon(&self) -> DCON_R {
        DCON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Device status - suspend."]
    #[inline(always)]
    pub fn dsus(&self) -> DSUS_R {
        DSUS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend."]
    #[inline(always)]
    pub fn lpm_sus(&self) -> LPM_SUS_R {
        LPM_SUS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPM Remote Wake-up Enabled by USB host."]
    #[inline(always)]
    pub fn lpm_rewp(&self) -> LPM_REWP_R {
        LPM_REWP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 22:23 - This field indicates the speed at which the device operates: 00b: reserved 01b: full-speed 10b: high-speed 11b: super-speed (reserved for future use)."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Device status - connect change."]
    #[inline(always)]
    pub fn dcon_c(&self) -> DCON_C_R {
        DCON_C_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Device status - suspend change."]
    #[inline(always)]
    pub fn dsus_c(&self) -> DSUS_C_R {
        DSUS_C_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Device status - reset change."]
    #[inline(always)]
    pub fn dres_c(&self) -> DRES_C_R {
        DRES_C_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit indicates if VBUS is detected or not."]
    #[inline(always)]
    pub fn vbus_debounced(&self) -> VBUS_DEBOUNCED_R {
        VBUS_DEBOUNCED_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification"]
    #[inline(always)]
    pub fn phy_test_mode(&self) -> PHY_TEST_MODE_R {
        PHY_TEST_MODE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USB device address."]
    #[inline(always)]
    #[must_use]
    pub fn dev_addr(&mut self) -> DEV_ADDR_W<0> {
        DEV_ADDR_W::new(self)
    }
    #[doc = "Bit 7 - USB device enable."]
    #[inline(always)]
    #[must_use]
    pub fn dev_en(&mut self) -> DEV_EN_W<7> {
        DEV_EN_W::new(self)
    }
    #[doc = "Bit 8 - SETUP token received."]
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<8> {
        SETUP_W::new(self)
    }
    #[doc = "Bit 9 - Forces the NEEDCLK output to always be on:."]
    #[inline(always)]
    #[must_use]
    pub fn force_needclk(&mut self) -> FORCE_NEEDCLK_W<9> {
        FORCE_NEEDCLK_W::new(self)
    }
    #[doc = "Bit 11 - LPM Supported:."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_sup(&mut self) -> LPM_SUP_W<11> {
        LPM_SUP_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt on NAK for interrupt and bulk OUT EP:."]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_ao(&mut self) -> INTONNAK_AO_W<12> {
        INTONNAK_AO_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt on NAK for interrupt and bulk IN EP:."]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_ai(&mut self) -> INTONNAK_AI_W<13> {
        INTONNAK_AI_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt on NAK for control OUT EP:."]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_co(&mut self) -> INTONNAK_CO_W<14> {
        INTONNAK_CO_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt on NAK for control IN EP:."]
    #[inline(always)]
    #[must_use]
    pub fn intonnak_ci(&mut self) -> INTONNAK_CI_W<15> {
        INTONNAK_CI_W::new(self)
    }
    #[doc = "Bit 16 - Device status - connect."]
    #[inline(always)]
    #[must_use]
    pub fn dcon(&mut self) -> DCON_W<16> {
        DCON_W::new(self)
    }
    #[doc = "Bit 17 - Device status - suspend."]
    #[inline(always)]
    #[must_use]
    pub fn dsus(&mut self) -> DSUS_W<17> {
        DSUS_W::new(self)
    }
    #[doc = "Bit 19 - Device status - LPM Suspend."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_sus(&mut self) -> LPM_SUS_W<19> {
        LPM_SUS_W::new(self)
    }
    #[doc = "Bit 24 - Device status - connect change."]
    #[inline(always)]
    #[must_use]
    pub fn dcon_c(&mut self) -> DCON_C_W<24> {
        DCON_C_W::new(self)
    }
    #[doc = "Bit 25 - Device status - suspend change."]
    #[inline(always)]
    #[must_use]
    pub fn dsus_c(&mut self) -> DSUS_C_W<25> {
        DSUS_C_W::new(self)
    }
    #[doc = "Bit 26 - Device status - reset change."]
    #[inline(always)]
    #[must_use]
    pub fn dres_c(&mut self) -> DRES_C_W<26> {
        DRES_C_W::new(self)
    }
    #[doc = "Bits 29:31 - This field is written by firmware to put the PHY into a test mode as defined by the USB2.0 specification"]
    #[inline(always)]
    #[must_use]
    pub fn phy_test_mode(&mut self) -> PHY_TEST_MODE_W<29> {
        PHY_TEST_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Command/Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devcmdstat](index.html) module"]
pub struct DEVCMDSTAT_SPEC;
impl crate::RegisterSpec for DEVCMDSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devcmdstat::R](R) reader structure"]
impl crate::Readable for DEVCMDSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devcmdstat::W](W) writer structure"]
impl crate::Writable for DEVCMDSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVCMDSTAT to value 0x0800"]
impl crate::Resettable for DEVCMDSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
