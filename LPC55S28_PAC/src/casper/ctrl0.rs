#[doc = "Register `CTRL0` reader"]
pub struct R(crate::R<CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL0` writer"]
pub struct W(crate::W<CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL0_SPEC>;
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
impl From<crate::W<CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ABBPAIR` reader - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
pub type ABBPAIR_R = crate::BitReader<ABBPAIR_A>;
#[doc = "Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABBPAIR_A {
    #[doc = "0: Bank-pair 0 (1st)"]
    PAIR0 = 0,
    #[doc = "1: Bank-pair 1 (2nd)"]
    PAIR1 = 1,
}
impl From<ABBPAIR_A> for bool {
    #[inline(always)]
    fn from(variant: ABBPAIR_A) -> Self {
        variant as u8 != 0
    }
}
impl ABBPAIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABBPAIR_A {
        match self.bits {
            false => ABBPAIR_A::PAIR0,
            true => ABBPAIR_A::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        *self == ABBPAIR_A::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        *self == ABBPAIR_A::PAIR1
    }
}
#[doc = "Field `ABBPAIR` writer - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
pub type ABBPAIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0_SPEC, ABBPAIR_A, O>;
impl<'a, const O: u8> ABBPAIR_W<'a, O> {
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut W {
        self.variant(ABBPAIR_A::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut W {
        self.variant(ABBPAIR_A::PAIR1)
    }
}
#[doc = "Field `ABOFF` reader - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
pub type ABOFF_R = crate::BitReader<bool>;
#[doc = "Field `ABOFF` writer - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
pub type ABOFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0_SPEC, bool, O>;
#[doc = "Field `CDBPAIR` reader - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
pub type CDBPAIR_R = crate::BitReader<CDBPAIR_A>;
#[doc = "Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDBPAIR_A {
    #[doc = "0: Bank-pair 0 (1st)"]
    PAIR0 = 0,
    #[doc = "1: Bank-pair 1 (2nd)"]
    PAIR1 = 1,
}
impl From<CDBPAIR_A> for bool {
    #[inline(always)]
    fn from(variant: CDBPAIR_A) -> Self {
        variant as u8 != 0
    }
}
impl CDBPAIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDBPAIR_A {
        match self.bits {
            false => CDBPAIR_A::PAIR0,
            true => CDBPAIR_A::PAIR1,
        }
    }
    #[doc = "Checks if the value of the field is `PAIR0`"]
    #[inline(always)]
    pub fn is_pair0(&self) -> bool {
        *self == CDBPAIR_A::PAIR0
    }
    #[doc = "Checks if the value of the field is `PAIR1`"]
    #[inline(always)]
    pub fn is_pair1(&self) -> bool {
        *self == CDBPAIR_A::PAIR1
    }
}
#[doc = "Field `CDBPAIR` writer - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
pub type CDBPAIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL0_SPEC, CDBPAIR_A, O>;
impl<'a, const O: u8> CDBPAIR_W<'a, O> {
    #[doc = "Bank-pair 0 (1st)"]
    #[inline(always)]
    pub fn pair0(self) -> &'a mut W {
        self.variant(CDBPAIR_A::PAIR0)
    }
    #[doc = "Bank-pair 1 (2nd)"]
    #[inline(always)]
    pub fn pair1(self) -> &'a mut W {
        self.variant(CDBPAIR_A::PAIR1)
    }
}
#[doc = "Field `CDOFF` reader - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
pub type CDOFF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CDOFF` writer - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
pub type CDOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL0_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bit 0 - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn abbpair(&self) -> ABBPAIR_R {
        ABBPAIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[inline(always)]
    pub fn aboff(&self) -> ABOFF_R {
        ABOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    pub fn cdbpair(&self) -> CDBPAIR_R {
        CDBPAIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[inline(always)]
    pub fn cdoff(&self) -> CDOFF_R {
        CDOFF_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Which bank-pair the offset ABOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    #[must_use]
    pub fn abbpair(&mut self) -> ABBPAIR_W<0> {
        ABBPAIR_W::new(self)
    }
    #[doc = "Bit 2 - Word or DWord Offset of AB values, with B at \\[2\\]=0 and A at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the CD values if 4-up"]
    #[inline(always)]
    #[must_use]
    pub fn aboff(&mut self) -> ABOFF_W<2> {
        ABOFF_W::new(self)
    }
    #[doc = "Bit 16 - Which bank-pair the offset CDOFF is within. This must be 0 if only 2-up"]
    #[inline(always)]
    #[must_use]
    pub fn cdbpair(&mut self) -> CDBPAIR_W<16> {
        CDBPAIR_W::new(self)
    }
    #[doc = "Bits 18:28 - Word or DWord Offset of CD, with D at \\[2\\]=0 and C at \\[2\\]=1 as far as the code sees (normally will be an interleaved bank so only sequential to AHB). Word offset only allowed if 32 bit operation. Ideally not in the same RAM as the AB values"]
    #[inline(always)]
    #[must_use]
    pub fn cdoff(&mut self) -> CDOFF_W<18> {
        CDOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Contains the offsets of AB and CD in the RAM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](index.html) module"]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl0::R](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
