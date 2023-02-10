#[doc = "Register `LOADER` reader"]
pub struct R(crate::R<LOADER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOADER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOADER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOADER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOADER` writer"]
pub struct W(crate::W<LOADER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOADER_SPEC>;
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
impl From<crate::W<LOADER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOADER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
pub type COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COUNT` writer - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOADER_SPEC, u8, u8, 8, O>;
#[doc = "Field `CTRLBPAIR` reader - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
pub type CTRLBPAIR_R = crate::BitReader<CTRLBPAIR_A>;
#[doc = "Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRLBPAIR_A {
    #[doc = "0: Bank-pair 0 (1st)"]
    PAIR0 = 0,
    #[doc = "1: Bank-pair 1 (2nd)"]
    PAIR1 = 1,
}
impl From<CTRLBPAIR_A> for bool {
    #[inline(always)]
    fn from(variant: CTRLBPAIR_A) -> Self {
        variant as u8 != 0
    }
}
impl CTRLBPAIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRLBPAIR_A {
        match self.bits {
            false => CTRLBPAIR_A::PAIR0,
            true => CTRLBPAIR_A::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        *self == CTRLBPAIR_A::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        *self == CTRLBPAIR_A::PAIR1
    }
}
#[doc = "Field `CTRLBPAIR` writer - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
pub type CTRLBPAIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, LOADER_SPEC, CTRLBPAIR_A, O>;
impl<'a, const O: u8> CTRLBPAIR_W<'a, O> {
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut W {
        self.variant(CTRLBPAIR_A::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut W {
        self.variant(CTRLBPAIR_A::PAIR1)
    }
}
#[doc = "Field `CTRLOFF` reader - DWord Offset of CTRL pair to load next."]
pub type CTRLOFF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CTRLOFF` writer - DWord Offset of CTRL pair to load next."]
pub type CTRLOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOADER_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:7 - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline(always)]
    pub fn ctrlbpair(&self) -> CTRLBPAIR_R {
        CTRLBPAIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:28 - DWord Offset of CTRL pair to load next."]
    #[inline(always)]
    pub fn ctrloff(&self) -> CTRLOFF_R {
        CTRLOFF_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of control pairs to load 0 relative (so 1 means load 1). write 1 means Does one op - does not iterate, write N means N control pairs to load"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<0> {
        COUNT_W::new(self)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CTRLOFF is within. This must be 0 if only 2-up. Does not matter which bank is used as this is loaded when not performing an operation."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlbpair(&mut self) -> CTRLBPAIR_W<16> {
        CTRLBPAIR_W::new(self)
    }
    #[doc = "Bits 18:28 - DWord Offset of CTRL pair to load next."]
    #[inline(always)]
    #[must_use]
    pub fn ctrloff(&mut self) -> CTRLOFF_W<18> {
        CTRLOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains an optional loader to load into CTRL0/1 in steps to perform a set of operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [loader](index.html) module"]
pub struct LOADER_SPEC;
impl crate::RegisterSpec for LOADER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [loader::R](R) reader structure"]
impl crate::Readable for LOADER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [loader::W](W) writer structure"]
impl crate::Writable for LOADER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOADER to value 0"]
impl crate::Resettable for LOADER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
