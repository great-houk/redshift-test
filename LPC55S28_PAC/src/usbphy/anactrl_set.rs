#[doc = "Register `ANACTRL_SET` reader"]
pub struct R(crate::R<ANACTRL_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANACTRL_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANACTRL_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANACTRL_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANACTRL_SET` writer"]
pub struct W(crate::W<ANACTRL_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANACTRL_SET_SPEC>;
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
impl From<crate::W<ANACTRL_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANACTRL_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVI_EN` reader - Vow voltage detector enable bit."]
pub type LVI_EN_R = crate::BitReader<bool>;
#[doc = "Field `LVI_EN` writer - Vow voltage detector enable bit."]
pub type LVI_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ANACTRL_SET_SPEC, bool, O>;
#[doc = "Field `PFD_CLK_SEL` reader - For normal USB operation, this bit field must remain at value 2'b00."]
pub type PFD_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFD_CLK_SEL` writer - For normal USB operation, this bit field must remain at value 2'b00."]
pub type PFD_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANACTRL_SET_SPEC, u8, u8, 2, O>;
#[doc = "Field `DEV_PULLDOWN` reader - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
pub type DEV_PULLDOWN_R = crate::BitReader<DEV_PULLDOWN_A>;
#[doc = "Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEV_PULLDOWN_A {
    #[doc = "0: The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    VALUE0 = 0,
    #[doc = "1: The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    VALUE1 = 1,
}
impl From<DEV_PULLDOWN_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_PULLDOWN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEV_PULLDOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_PULLDOWN_A {
        match self.bits {
            false => DEV_PULLDOWN_A::VALUE0,
            true => DEV_PULLDOWN_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == DEV_PULLDOWN_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == DEV_PULLDOWN_A::VALUE1
    }
}
#[doc = "Field `DEV_PULLDOWN` writer - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
pub type DEV_PULLDOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, ANACTRL_SET_SPEC, DEV_PULLDOWN_A, O>;
impl<'a, const O: u8> DEV_PULLDOWN_W<'a, O> {
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare disabled in device mode."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::VALUE0)
    }
    #[doc = "The 15kohm nominal pulldowns on the USB_DP and USB_DM pinsare enabled in device mode."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(DEV_PULLDOWN_A::VALUE1)
    }
}
impl R {
    #[doc = "Bit 1 - Vow voltage detector enable bit."]
    #[inline(always)]
    pub fn lvi_en(&self) -> LVI_EN_R {
        LVI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    pub fn pfd_clk_sel(&self) -> PFD_CLK_SEL_R {
        PFD_CLK_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    pub fn dev_pulldown(&self) -> DEV_PULLDOWN_R {
        DEV_PULLDOWN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Vow voltage detector enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn lvi_en(&mut self) -> LVI_EN_W<1> {
        LVI_EN_W::new(self)
    }
    #[doc = "Bits 2:3 - For normal USB operation, this bit field must remain at value 2'b00."]
    #[inline(always)]
    #[must_use]
    pub fn pfd_clk_sel(&mut self) -> PFD_CLK_SEL_W<2> {
        PFD_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 10 - Setting this field to 1'b1 will enable the 15kohm pulldown resistors on both USB_DP and USB_DM pins"]
    #[inline(always)]
    #[must_use]
    pub fn dev_pulldown(&mut self) -> DEV_PULLDOWN_W<10> {
        DEV_PULLDOWN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anactrl_set](index.html) module"]
pub struct ANACTRL_SET_SPEC;
impl crate::RegisterSpec for ANACTRL_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anactrl_set::R](R) reader structure"]
impl crate::Readable for ANACTRL_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anactrl_set::W](W) writer structure"]
impl crate::Writable for ANACTRL_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANACTRL_SET to value 0x0a00_0402"]
impl crate::Resettable for ANACTRL_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a00_0402;
}
