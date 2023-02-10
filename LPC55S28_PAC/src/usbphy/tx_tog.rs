#[doc = "Register `TX_TOG` reader"]
pub struct R(crate::R<TX_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX_TOG` writer"]
pub struct W(crate::W<TX_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_TOG_SPEC>;
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
impl From<crate::W<TX_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_CAL` reader - Decode to trim the nominal 17"]
pub type D_CAL_R = crate::FieldReader<u8, D_CAL_A>;
#[doc = "Decode to trim the nominal 17\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum D_CAL_A {
    #[doc = "0: Maximum current, approximately 19% above nominal."]
    VALUE0 = 0,
    #[doc = "7: Nominal"]
    VALUE7 = 7,
    #[doc = "15: Minimum current, approximately 19% below nominal."]
    VALUE15 = 15,
}
impl From<D_CAL_A> for u8 {
    #[inline(always)]
    fn from(variant: D_CAL_A) -> Self {
        variant as _
    }
}
impl D_CAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<D_CAL_A> {
        match self.bits {
            0 => Some(D_CAL_A::VALUE0),
            7 => Some(D_CAL_A::VALUE7),
            15 => Some(D_CAL_A::VALUE15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == D_CAL_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE7`"]
    #[inline(always)]
    pub fn is_value7(&self) -> bool {
        *self == D_CAL_A::VALUE7
    }
    #[doc = "Checks if the value of the field is `VALUE15`"]
    #[inline(always)]
    pub fn is_value15(&self) -> bool {
        *self == D_CAL_A::VALUE15
    }
}
#[doc = "Field `D_CAL` writer - Decode to trim the nominal 17"]
pub type D_CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_TOG_SPEC, u8, D_CAL_A, 4, O>;
impl<'a, const O: u8> D_CAL_W<'a, O> {
    #[doc = "Maximum current, approximately 19% above nominal."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(D_CAL_A::VALUE0)
    }
    #[doc = "Nominal"]
    #[inline(always)]
    pub fn value7(self) -> &'a mut W {
        self.variant(D_CAL_A::VALUE7)
    }
    #[doc = "Minimum current, approximately 19% below nominal."]
    #[inline(always)]
    pub fn value15(self) -> &'a mut W {
        self.variant(D_CAL_A::VALUE15)
    }
}
#[doc = "Field `TXCAL45DM` reader - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
pub type TXCAL45DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCAL45DM` writer - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
pub type TXCAL45DM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_TOG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXENCAL45DN` reader - Enable resistance calibration on DN."]
pub type TXENCAL45DN_R = crate::BitReader<bool>;
#[doc = "Field `TXENCAL45DN` writer - Enable resistance calibration on DN."]
pub type TXENCAL45DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_TOG_SPEC, bool, O>;
#[doc = "Field `TXCAL45DP` reader - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
pub type TXCAL45DP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCAL45DP` writer - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
pub type TXCAL45DP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_TOG_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXENCAL45DP` reader - Enable resistance calibration on DP."]
pub type TXENCAL45DP_R = crate::BitReader<bool>;
#[doc = "Field `TXENCAL45DP` writer - Enable resistance calibration on DP."]
pub type TXENCAL45DP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TX_TOG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline(always)]
    pub fn d_cal(&self) -> D_CAL_R {
        D_CAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    pub fn txcal45dm(&self) -> TXCAL45DM_R {
        TXCAL45DM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline(always)]
    pub fn txencal45dn(&self) -> TXENCAL45DN_R {
        TXENCAL45DN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    pub fn txcal45dp(&self) -> TXCAL45DP_R {
        TXCAL45DP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP."]
    #[inline(always)]
    pub fn txencal45dp(&self) -> TXENCAL45DP_R {
        TXENCAL45DP_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Decode to trim the nominal 17"]
    #[inline(always)]
    #[must_use]
    pub fn d_cal(&mut self) -> D_CAL_W<0> {
        D_CAL_W::new(self)
    }
    #[doc = "Bits 8:11 - Decode to trim the nominal 45ohm series termination resistance to the USB_DM output pin"]
    #[inline(always)]
    #[must_use]
    pub fn txcal45dm(&mut self) -> TXCAL45DM_W<8> {
        TXCAL45DM_W::new(self)
    }
    #[doc = "Bit 13 - Enable resistance calibration on DN."]
    #[inline(always)]
    #[must_use]
    pub fn txencal45dn(&mut self) -> TXENCAL45DN_W<13> {
        TXENCAL45DN_W::new(self)
    }
    #[doc = "Bits 16:19 - Decode to trim the nominal 45ohm series termination resistance to the USB_DP output pin"]
    #[inline(always)]
    #[must_use]
    pub fn txcal45dp(&mut self) -> TXCAL45DP_W<16> {
        TXCAL45DP_W::new(self)
    }
    #[doc = "Bit 21 - Enable resistance calibration on DP."]
    #[inline(always)]
    #[must_use]
    pub fn txencal45dp(&mut self) -> TXENCAL45DP_W<21> {
        TXENCAL45DP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Transmitter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx_tog](index.html) module"]
pub struct TX_TOG_SPEC;
impl crate::RegisterSpec for TX_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx_tog::R](R) reader structure"]
impl crate::Readable for TX_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx_tog::W](W) writer structure"]
impl crate::Writable for TX_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX_TOG to value 0x0a00_0402"]
impl crate::Resettable for TX_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a00_0402;
}
