#[doc = "Register `SFSR` reader"]
pub struct R(crate::R<SFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFSR` writer"]
pub struct W(crate::W<SFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFSR_SPEC>;
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
impl From<crate::W<SFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INVEP` reader - Invalid entry point."]
pub type INVEP_R = crate::BitReader<INVEP_A>;
#[doc = "Invalid entry point.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVEP_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<INVEP_A> for bool {
    #[inline(always)]
    fn from(variant: INVEP_A) -> Self {
        variant as u8 != 0
    }
}
impl INVEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVEP_A {
        match self.bits {
            false => INVEP_A::NO_ERROR,
            true => INVEP_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == INVEP_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == INVEP_A::ERROR
    }
}
#[doc = "Field `INVEP` writer - Invalid entry point."]
pub type INVEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, INVEP_A, O>;
impl<'a, const O: u8> INVEP_W<'a, O> {
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVEP_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(INVEP_A::ERROR)
    }
}
#[doc = "Field `INVIS` reader - Invalid integrity signature flag."]
pub type INVIS_R = crate::BitReader<INVIS_A>;
#[doc = "Invalid integrity signature flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVIS_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<INVIS_A> for bool {
    #[inline(always)]
    fn from(variant: INVIS_A) -> Self {
        variant as u8 != 0
    }
}
impl INVIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVIS_A {
        match self.bits {
            false => INVIS_A::NO_ERROR,
            true => INVIS_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == INVIS_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == INVIS_A::ERROR
    }
}
#[doc = "Field `INVIS` writer - Invalid integrity signature flag."]
pub type INVIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, INVIS_A, O>;
impl<'a, const O: u8> INVIS_W<'a, O> {
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVIS_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(INVIS_A::ERROR)
    }
}
#[doc = "Field `INVER` reader - Invalid exception return flag."]
pub type INVER_R = crate::BitReader<INVER_A>;
#[doc = "Invalid exception return flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVER_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<INVER_A> for bool {
    #[inline(always)]
    fn from(variant: INVER_A) -> Self {
        variant as u8 != 0
    }
}
impl INVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVER_A {
        match self.bits {
            false => INVER_A::NO_ERROR,
            true => INVER_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == INVER_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == INVER_A::ERROR
    }
}
#[doc = "Field `INVER` writer - Invalid exception return flag."]
pub type INVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, INVER_A, O>;
impl<'a, const O: u8> INVER_W<'a, O> {
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVER_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(INVER_A::ERROR)
    }
}
#[doc = "Field `AUVIOL` reader - Attribution unit violation flag."]
pub type AUVIOL_R = crate::BitReader<AUVIOL_A>;
#[doc = "Attribution unit violation flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUVIOL_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<AUVIOL_A> for bool {
    #[inline(always)]
    fn from(variant: AUVIOL_A) -> Self {
        variant as u8 != 0
    }
}
impl AUVIOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUVIOL_A {
        match self.bits {
            false => AUVIOL_A::NO_ERROR,
            true => AUVIOL_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == AUVIOL_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == AUVIOL_A::ERROR
    }
}
#[doc = "Field `AUVIOL` writer - Attribution unit violation flag."]
pub type AUVIOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, AUVIOL_A, O>;
impl<'a, const O: u8> AUVIOL_W<'a, O> {
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(AUVIOL_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(AUVIOL_A::ERROR)
    }
}
#[doc = "Field `INVTRAN` reader - Invalid transition flag."]
pub type INVTRAN_R = crate::BitReader<INVTRAN_A>;
#[doc = "Invalid transition flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVTRAN_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<INVTRAN_A> for bool {
    #[inline(always)]
    fn from(variant: INVTRAN_A) -> Self {
        variant as u8 != 0
    }
}
impl INVTRAN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVTRAN_A {
        match self.bits {
            false => INVTRAN_A::NO_ERROR,
            true => INVTRAN_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == INVTRAN_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == INVTRAN_A::ERROR
    }
}
#[doc = "Field `INVTRAN` writer - Invalid transition flag."]
pub type INVTRAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, INVTRAN_A, O>;
impl<'a, const O: u8> INVTRAN_W<'a, O> {
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(INVTRAN_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(INVTRAN_A::ERROR)
    }
}
#[doc = "Field `LSPERR` reader - Lazy state preservation error flag."]
pub type LSPERR_R = crate::BitReader<LSPERR_A>;
#[doc = "Lazy state preservation error flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSPERR_A {
    #[doc = "0: Error has not occurred."]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<LSPERR_A> for bool {
    #[inline(always)]
    fn from(variant: LSPERR_A) -> Self {
        variant as u8 != 0
    }
}
impl LSPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSPERR_A {
        match self.bits {
            false => LSPERR_A::NO_ERROR,
            true => LSPERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LSPERR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == LSPERR_A::ERROR
    }
}
#[doc = "Field `LSPERR` writer - Lazy state preservation error flag."]
pub type LSPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, LSPERR_A, O>;
impl<'a, const O: u8> LSPERR_W<'a, O> {
    #[doc = "Error has not occurred."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(LSPERR_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(LSPERR_A::ERROR)
    }
}
#[doc = "Field `SFARVALID` reader - Secure fault address valid."]
pub type SFARVALID_R = crate::BitReader<SFARVALID_A>;
#[doc = "Secure fault address valid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFARVALID_A {
    #[doc = "0: SFAR content not valid."]
    NOT_VALID = 0,
    #[doc = "1: SFAR content valid."]
    VALID = 1,
}
impl From<SFARVALID_A> for bool {
    #[inline(always)]
    fn from(variant: SFARVALID_A) -> Self {
        variant as u8 != 0
    }
}
impl SFARVALID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFARVALID_A {
        match self.bits {
            false => SFARVALID_A::NOT_VALID,
            true => SFARVALID_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_VALID`"]
    #[inline(always)]
    pub fn is_not_valid(&self) -> bool {
        *self == SFARVALID_A::NOT_VALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == SFARVALID_A::VALID
    }
}
#[doc = "Field `SFARVALID` writer - Secure fault address valid."]
pub type SFARVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, SFARVALID_A, O>;
impl<'a, const O: u8> SFARVALID_W<'a, O> {
    #[doc = "SFAR content not valid."]
    #[inline(always)]
    pub fn not_valid(self) -> &'a mut W {
        self.variant(SFARVALID_A::NOT_VALID)
    }
    #[doc = "SFAR content valid."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(SFARVALID_A::VALID)
    }
}
#[doc = "Field `LSERR` reader - Lazy state error flag."]
pub type LSERR_R = crate::BitReader<LSERR_A>;
#[doc = "Lazy state error flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERR_A {
    #[doc = "0: Error has not occurred"]
    NO_ERROR = 0,
    #[doc = "1: Error has occurred."]
    ERROR = 1,
}
impl From<LSERR_A> for bool {
    #[inline(always)]
    fn from(variant: LSERR_A) -> Self {
        variant as u8 != 0
    }
}
impl LSERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERR_A {
        match self.bits {
            false => LSERR_A::NO_ERROR,
            true => LSERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LSERR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == LSERR_A::ERROR
    }
}
#[doc = "Field `LSERR` writer - Lazy state error flag."]
pub type LSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFSR_SPEC, LSERR_A, O>;
impl<'a, const O: u8> LSERR_W<'a, O> {
    #[doc = "Error has not occurred"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(LSERR_A::NO_ERROR)
    }
    #[doc = "Error has occurred."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(LSERR_A::ERROR)
    }
}
impl R {
    #[doc = "Bit 0 - Invalid entry point."]
    #[inline(always)]
    pub fn invep(&self) -> INVEP_R {
        INVEP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invalid integrity signature flag."]
    #[inline(always)]
    pub fn invis(&self) -> INVIS_R {
        INVIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invalid exception return flag."]
    #[inline(always)]
    pub fn inver(&self) -> INVER_R {
        INVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Attribution unit violation flag."]
    #[inline(always)]
    pub fn auviol(&self) -> AUVIOL_R {
        AUVIOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Invalid transition flag."]
    #[inline(always)]
    pub fn invtran(&self) -> INVTRAN_R {
        INVTRAN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lazy state preservation error flag."]
    #[inline(always)]
    pub fn lsperr(&self) -> LSPERR_R {
        LSPERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secure fault address valid."]
    #[inline(always)]
    pub fn sfarvalid(&self) -> SFARVALID_R {
        SFARVALID_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lazy state error flag."]
    #[inline(always)]
    pub fn lserr(&self) -> LSERR_R {
        LSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid entry point."]
    #[inline(always)]
    #[must_use]
    pub fn invep(&mut self) -> INVEP_W<0> {
        INVEP_W::new(self)
    }
    #[doc = "Bit 1 - Invalid integrity signature flag."]
    #[inline(always)]
    #[must_use]
    pub fn invis(&mut self) -> INVIS_W<1> {
        INVIS_W::new(self)
    }
    #[doc = "Bit 2 - Invalid exception return flag."]
    #[inline(always)]
    #[must_use]
    pub fn inver(&mut self) -> INVER_W<2> {
        INVER_W::new(self)
    }
    #[doc = "Bit 3 - Attribution unit violation flag."]
    #[inline(always)]
    #[must_use]
    pub fn auviol(&mut self) -> AUVIOL_W<3> {
        AUVIOL_W::new(self)
    }
    #[doc = "Bit 4 - Invalid transition flag."]
    #[inline(always)]
    #[must_use]
    pub fn invtran(&mut self) -> INVTRAN_W<4> {
        INVTRAN_W::new(self)
    }
    #[doc = "Bit 5 - Lazy state preservation error flag."]
    #[inline(always)]
    #[must_use]
    pub fn lsperr(&mut self) -> LSPERR_W<5> {
        LSPERR_W::new(self)
    }
    #[doc = "Bit 6 - Secure fault address valid."]
    #[inline(always)]
    #[must_use]
    pub fn sfarvalid(&mut self) -> SFARVALID_W<6> {
        SFARVALID_W::new(self)
    }
    #[doc = "Bit 7 - Lazy state error flag."]
    #[inline(always)]
    #[must_use]
    pub fn lserr(&mut self) -> LSERR_W<7> {
        LSERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfsr](index.html) module"]
pub struct SFSR_SPEC;
impl crate::RegisterSpec for SFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfsr::R](R) reader structure"]
impl crate::Readable for SFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfsr::W](W) writer structure"]
impl crate::Writable for SFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFSR to value 0"]
impl crate::Resettable for SFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
