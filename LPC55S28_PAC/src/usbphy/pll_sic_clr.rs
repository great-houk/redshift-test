#[doc = "Register `PLL_SIC_CLR` reader"]
pub struct R(crate::R<PLL_SIC_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_SIC_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_SIC_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_SIC_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL_SIC_CLR` writer"]
pub struct W(crate::W<PLL_SIC_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_SIC_CLR_SPEC>;
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
impl From<crate::W<PLL_SIC_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_SIC_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL_EN_USB_CLKS` reader - Enables the USB clock from PLL to USB PHY"]
pub type PLL_EN_USB_CLKS_R = crate::BitReader<bool>;
#[doc = "Field `PLL_EN_USB_CLKS` writer - Enables the USB clock from PLL to USB PHY"]
pub type PLL_EN_USB_CLKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, bool, O>;
#[doc = "Field `PLL_POWER` reader - Power up the USB PLL"]
pub type PLL_POWER_R = crate::BitReader<bool>;
#[doc = "Field `PLL_POWER` writer - Power up the USB PLL"]
pub type PLL_POWER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, bool, O>;
#[doc = "Field `PLL_ENABLE` reader - Enables the clock output from the USB PLL"]
pub type PLL_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PLL_ENABLE` writer - Enables the clock output from the USB PLL"]
pub type PLL_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, bool, O>;
#[doc = "Field `REFBIAS_PWD_SEL` reader - Reference bias power down select."]
pub type REFBIAS_PWD_SEL_R = crate::BitReader<REFBIAS_PWD_SEL_A>;
#[doc = "Reference bias power down select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFBIAS_PWD_SEL_A {
    #[doc = "0: Selects PLL_POWER to control the reference bias"]
    VALUE0 = 0,
    #[doc = "1: Selects REFBIAS_PWD to control the reference bias"]
    VALUE1 = 1,
}
impl From<REFBIAS_PWD_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: REFBIAS_PWD_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl REFBIAS_PWD_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFBIAS_PWD_SEL_A {
        match self.bits {
            false => REFBIAS_PWD_SEL_A::VALUE0,
            true => REFBIAS_PWD_SEL_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == REFBIAS_PWD_SEL_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == REFBIAS_PWD_SEL_A::VALUE1
    }
}
#[doc = "Field `REFBIAS_PWD_SEL` writer - Reference bias power down select."]
pub type REFBIAS_PWD_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, REFBIAS_PWD_SEL_A, O>;
impl<'a, const O: u8> REFBIAS_PWD_SEL_W<'a, O> {
    #[doc = "Selects PLL_POWER to control the reference bias"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_SEL_A::VALUE0)
    }
    #[doc = "Selects REFBIAS_PWD to control the reference bias"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(REFBIAS_PWD_SEL_A::VALUE1)
    }
}
#[doc = "Field `REFBIAS_PWD` reader - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
pub type REFBIAS_PWD_R = crate::BitReader<bool>;
#[doc = "Field `REFBIAS_PWD` writer - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
pub type REFBIAS_PWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, bool, O>;
#[doc = "Field `PLL_REG_ENABLE` reader - This field controls the USB PLL regulator, set to enable the regulator"]
pub type PLL_REG_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `PLL_REG_ENABLE` writer - This field controls the USB PLL regulator, set to enable the regulator"]
pub type PLL_REG_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, bool, O>;
#[doc = "Field `PLL_DIV_SEL` reader - This field controls the USB PLL feedback loop divider"]
pub type PLL_DIV_SEL_R = crate::FieldReader<u8, PLL_DIV_SEL_A>;
#[doc = "This field controls the USB PLL feedback loop divider\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLL_DIV_SEL_A {
    #[doc = "0: Divide by 13"]
    VALUE0 = 0,
    #[doc = "1: Divide by 15"]
    VALUE1 = 1,
    #[doc = "2: Divide by 16"]
    VALUE2 = 2,
    #[doc = "3: Divide by 20"]
    VALUE3 = 3,
    #[doc = "4: Divide by 22"]
    VALUE4 = 4,
    #[doc = "5: Divide by 25"]
    VALUE5 = 5,
    #[doc = "6: Divide by 30"]
    VALUE6 = 6,
    #[doc = "7: Divide by 240"]
    VALUE7 = 7,
}
impl From<PLL_DIV_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLL_DIV_SEL_A) -> Self {
        variant as _
    }
}
impl PLL_DIV_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_DIV_SEL_A {
        match self.bits {
            0 => PLL_DIV_SEL_A::VALUE0,
            1 => PLL_DIV_SEL_A::VALUE1,
            2 => PLL_DIV_SEL_A::VALUE2,
            3 => PLL_DIV_SEL_A::VALUE3,
            4 => PLL_DIV_SEL_A::VALUE4,
            5 => PLL_DIV_SEL_A::VALUE5,
            6 => PLL_DIV_SEL_A::VALUE6,
            7 => PLL_DIV_SEL_A::VALUE7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE1
    }
    #[doc = "Checks if the value of the field is `VALUE2`"]
    #[inline(always)]
    pub fn is_value2(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE2
    }
    #[doc = "Checks if the value of the field is `VALUE3`"]
    #[inline(always)]
    pub fn is_value3(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE3
    }
    #[doc = "Checks if the value of the field is `VALUE4`"]
    #[inline(always)]
    pub fn is_value4(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE4
    }
    #[doc = "Checks if the value of the field is `VALUE5`"]
    #[inline(always)]
    pub fn is_value5(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE5
    }
    #[doc = "Checks if the value of the field is `VALUE6`"]
    #[inline(always)]
    pub fn is_value6(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE6
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == PLL_DIV_SEL_A::VALUE7
    }
}
#[doc = "Field `PLL_DIV_SEL` writer - This field controls the USB PLL feedback loop divider"]
pub type PLL_DIV_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PLL_SIC_CLR_SPEC, u8, PLL_DIV_SEL_A, 3, O>;
impl<'a, const O: u8> PLL_DIV_SEL_W<'a, O> {
    #[doc = "Divide by 13"]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE0)
    }
    #[doc = "Divide by 15"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE1)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn value2(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE2)
    }
    #[doc = "Divide by 20"]
    #[inline(always)]
    pub fn value3(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE3)
    }
    #[doc = "Divide by 22"]
    #[inline(always)]
    pub fn value4(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE4)
    }
    #[doc = "Divide by 25"]
    #[inline(always)]
    pub fn value5(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE5)
    }
    #[doc = "Divide by 30"]
    #[inline(always)]
    pub fn value6(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE6)
    }
    #[doc = "Divide by 240"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(PLL_DIV_SEL_A::VALUE7)
    }
}
#[doc = "Field `PLL_PREDIV` reader - This is selection between /1 or /2 to expand the range of ref input clock."]
pub type PLL_PREDIV_R = crate::BitReader<bool>;
#[doc = "Field `PLL_PREDIV` writer - This is selection between /1 or /2 to expand the range of ref input clock."]
pub type PLL_PREDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SIC_CLR_SPEC, bool, O>;
#[doc = "Field `PLL_LOCK` reader - USB PLL lock status indicator"]
pub type PLL_LOCK_R = crate::BitReader<PLL_LOCK_A>;
#[doc = "USB PLL lock status indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLL_LOCK_A {
    #[doc = "0: PLL is not currently locked"]
    VALUE0 = 0,
    #[doc = "1: PLL is currently locked"]
    VALUE1 = 1,
}
impl From<PLL_LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PLL_LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl PLL_LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLL_LOCK_A {
        match self.bits {
            false => PLL_LOCK_A::VALUE0,
            true => PLL_LOCK_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == PLL_LOCK_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == PLL_LOCK_A::VALUE1
    }
}
impl R {
    #[doc = "Bit 6 - Enables the USB clock from PLL to USB PHY"]
    #[inline(always)]
    pub fn pll_en_usb_clks(&self) -> PLL_EN_USB_CLKS_R {
        PLL_EN_USB_CLKS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Power up the USB PLL"]
    #[inline(always)]
    pub fn pll_power(&self) -> PLL_POWER_R {
        PLL_POWER_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the clock output from the USB PLL"]
    #[inline(always)]
    pub fn pll_enable(&self) -> PLL_ENABLE_R {
        PLL_ENABLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - Reference bias power down select."]
    #[inline(always)]
    pub fn refbias_pwd_sel(&self) -> REFBIAS_PWD_SEL_R {
        REFBIAS_PWD_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    pub fn refbias_pwd(&self) -> REFBIAS_PWD_R {
        REFBIAS_PWD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline(always)]
    pub fn pll_reg_enable(&self) -> PLL_REG_ENABLE_R {
        PLL_REG_ENABLE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:24 - This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    pub fn pll_div_sel(&self) -> PLL_DIV_SEL_R {
        PLL_DIV_SEL_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 30 - This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    pub fn pll_prediv(&self) -> PLL_PREDIV_R {
        PLL_PREDIV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USB PLL lock status indicator"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PLL_LOCK_R {
        PLL_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Enables the USB clock from PLL to USB PHY"]
    #[inline(always)]
    #[must_use]
    pub fn pll_en_usb_clks(&mut self) -> PLL_EN_USB_CLKS_W<6> {
        PLL_EN_USB_CLKS_W::new(self)
    }
    #[doc = "Bit 12 - Power up the USB PLL"]
    #[inline(always)]
    #[must_use]
    pub fn pll_power(&mut self) -> PLL_POWER_W<12> {
        PLL_POWER_W::new(self)
    }
    #[doc = "Bit 13 - Enables the clock output from the USB PLL"]
    #[inline(always)]
    #[must_use]
    pub fn pll_enable(&mut self) -> PLL_ENABLE_W<13> {
        PLL_ENABLE_W::new(self)
    }
    #[doc = "Bit 19 - Reference bias power down select."]
    #[inline(always)]
    #[must_use]
    pub fn refbias_pwd_sel(&mut self) -> REFBIAS_PWD_SEL_W<19> {
        REFBIAS_PWD_SEL_W::new(self)
    }
    #[doc = "Bit 20 - Power down the reference bias This bit is only used when REFBIAS_PWD_SEL is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn refbias_pwd(&mut self) -> REFBIAS_PWD_W<20> {
        REFBIAS_PWD_W::new(self)
    }
    #[doc = "Bit 21 - This field controls the USB PLL regulator, set to enable the regulator"]
    #[inline(always)]
    #[must_use]
    pub fn pll_reg_enable(&mut self) -> PLL_REG_ENABLE_W<21> {
        PLL_REG_ENABLE_W::new(self)
    }
    #[doc = "Bits 22:24 - This field controls the USB PLL feedback loop divider"]
    #[inline(always)]
    #[must_use]
    pub fn pll_div_sel(&mut self) -> PLL_DIV_SEL_W<22> {
        PLL_DIV_SEL_W::new(self)
    }
    #[doc = "Bit 30 - This is selection between /1 or /2 to expand the range of ref input clock."]
    #[inline(always)]
    #[must_use]
    pub fn pll_prediv(&mut self) -> PLL_PREDIV_W<30> {
        PLL_PREDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY PLL Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll_sic_clr](index.html) module"]
pub struct PLL_SIC_CLR_SPEC;
impl crate::RegisterSpec for PLL_SIC_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll_sic_clr::R](R) reader structure"]
impl crate::Readable for PLL_SIC_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll_sic_clr::W](W) writer structure"]
impl crate::Writable for PLL_SIC_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL_SIC_CLR to value 0x00d1_2000"]
impl crate::Resettable for PLL_SIC_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x00d1_2000;
}
