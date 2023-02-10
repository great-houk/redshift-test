#[doc = "Register `TST` reader"]
pub struct R(crate::R<TST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST` writer"]
pub struct W(crate::W<TST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_SPEC>;
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
impl From<crate::W<TST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CST_LONG` reader - Calibration Sample Time Long"]
pub type CST_LONG_R = crate::BitReader<CST_LONG_A>;
#[doc = "Calibration Sample Time Long\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CST_LONG_A {
    #[doc = "0: Normal sample time. Minimum sample time of 3 ADCK cycles."]
    CST_LONG_0 = 0,
    #[doc = "1: Increased sample time. 67 ADCK cycles total sample time."]
    CST_LONG_1 = 1,
}
impl From<CST_LONG_A> for bool {
    #[inline(always)]
    fn from(variant: CST_LONG_A) -> Self {
        variant as u8 != 0
    }
}
impl CST_LONG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CST_LONG_A {
        match self.bits {
            false => CST_LONG_A::CST_LONG_0,
            true => CST_LONG_A::CST_LONG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CST_LONG_0`"]
    #[inline(always)]
    pub fn is_cst_long_0(&self) -> bool {
        *self == CST_LONG_A::CST_LONG_0
    }
    #[doc = "Checks if the value of the field is `CST_LONG_1`"]
    #[inline(always)]
    pub fn is_cst_long_1(&self) -> bool {
        *self == CST_LONG_A::CST_LONG_1
    }
}
#[doc = "Field `CST_LONG` writer - Calibration Sample Time Long"]
pub type CST_LONG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TST_SPEC, CST_LONG_A, O>;
impl<'a, const O: u8> CST_LONG_W<'a, O> {
    #[doc = "Normal sample time. Minimum sample time of 3 ADCK cycles."]
    #[inline(always)]
    pub fn cst_long_0(self) -> &'a mut W {
        self.variant(CST_LONG_A::CST_LONG_0)
    }
    #[doc = "Increased sample time. 67 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn cst_long_1(self) -> &'a mut W {
        self.variant(CST_LONG_A::CST_LONG_1)
    }
}
#[doc = "Field `FOFFM` reader - Force M-side positive offset"]
pub type FOFFM_R = crate::BitReader<FOFFM_A>;
#[doc = "Force M-side positive offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOFFM_A {
    #[doc = "0: Normal operation. No forced offset."]
    FOFFM_0 = 0,
    #[doc = "1: Test configuration. Forced positive offset on MDAC."]
    FOFFM_1 = 1,
}
impl From<FOFFM_A> for bool {
    #[inline(always)]
    fn from(variant: FOFFM_A) -> Self {
        variant as u8 != 0
    }
}
impl FOFFM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFFM_A {
        match self.bits {
            false => FOFFM_A::FOFFM_0,
            true => FOFFM_A::FOFFM_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFM_0`"]
    #[inline(always)]
    pub fn is_foffm_0(&self) -> bool {
        *self == FOFFM_A::FOFFM_0
    }
    #[doc = "Checks if the value of the field is `FOFFM_1`"]
    #[inline(always)]
    pub fn is_foffm_1(&self) -> bool {
        *self == FOFFM_A::FOFFM_1
    }
}
#[doc = "Field `FOFFM` writer - Force M-side positive offset"]
pub type FOFFM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TST_SPEC, FOFFM_A, O>;
impl<'a, const O: u8> FOFFM_W<'a, O> {
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffm_0(self) -> &'a mut W {
        self.variant(FOFFM_A::FOFFM_0)
    }
    #[doc = "Test configuration. Forced positive offset on MDAC."]
    #[inline(always)]
    pub fn foffm_1(self) -> &'a mut W {
        self.variant(FOFFM_A::FOFFM_1)
    }
}
#[doc = "Field `FOFFP` reader - Force P-side positive offset"]
pub type FOFFP_R = crate::BitReader<FOFFP_A>;
#[doc = "Force P-side positive offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOFFP_A {
    #[doc = "0: Normal operation. No forced offset."]
    FOFFP_0 = 0,
    #[doc = "1: Test configuration. Forced positive offset on PDAC."]
    FOFFP_1 = 1,
}
impl From<FOFFP_A> for bool {
    #[inline(always)]
    fn from(variant: FOFFP_A) -> Self {
        variant as u8 != 0
    }
}
impl FOFFP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFFP_A {
        match self.bits {
            false => FOFFP_A::FOFFP_0,
            true => FOFFP_A::FOFFP_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFP_0`"]
    #[inline(always)]
    pub fn is_foffp_0(&self) -> bool {
        *self == FOFFP_A::FOFFP_0
    }
    #[doc = "Checks if the value of the field is `FOFFP_1`"]
    #[inline(always)]
    pub fn is_foffp_1(&self) -> bool {
        *self == FOFFP_A::FOFFP_1
    }
}
#[doc = "Field `FOFFP` writer - Force P-side positive offset"]
pub type FOFFP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TST_SPEC, FOFFP_A, O>;
impl<'a, const O: u8> FOFFP_W<'a, O> {
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffp_0(self) -> &'a mut W {
        self.variant(FOFFP_A::FOFFP_0)
    }
    #[doc = "Test configuration. Forced positive offset on PDAC."]
    #[inline(always)]
    pub fn foffp_1(self) -> &'a mut W {
        self.variant(FOFFP_A::FOFFP_1)
    }
}
#[doc = "Field `FOFFM2` reader - Force M-side negative offset"]
pub type FOFFM2_R = crate::BitReader<FOFFM2_A>;
#[doc = "Force M-side negative offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOFFM2_A {
    #[doc = "0: Normal operation. No forced offset."]
    FOFFM2_0 = 0,
    #[doc = "1: Test configuration. Forced negative offset on MDAC."]
    FOFFM2_1 = 1,
}
impl From<FOFFM2_A> for bool {
    #[inline(always)]
    fn from(variant: FOFFM2_A) -> Self {
        variant as u8 != 0
    }
}
impl FOFFM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFFM2_A {
        match self.bits {
            false => FOFFM2_A::FOFFM2_0,
            true => FOFFM2_A::FOFFM2_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFM2_0`"]
    #[inline(always)]
    pub fn is_foffm2_0(&self) -> bool {
        *self == FOFFM2_A::FOFFM2_0
    }
    #[doc = "Checks if the value of the field is `FOFFM2_1`"]
    #[inline(always)]
    pub fn is_foffm2_1(&self) -> bool {
        *self == FOFFM2_A::FOFFM2_1
    }
}
#[doc = "Field `FOFFM2` writer - Force M-side negative offset"]
pub type FOFFM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TST_SPEC, FOFFM2_A, O>;
impl<'a, const O: u8> FOFFM2_W<'a, O> {
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffm2_0(self) -> &'a mut W {
        self.variant(FOFFM2_A::FOFFM2_0)
    }
    #[doc = "Test configuration. Forced negative offset on MDAC."]
    #[inline(always)]
    pub fn foffm2_1(self) -> &'a mut W {
        self.variant(FOFFM2_A::FOFFM2_1)
    }
}
#[doc = "Field `FOFFP2` reader - Force P-side negative offset"]
pub type FOFFP2_R = crate::BitReader<FOFFP2_A>;
#[doc = "Force P-side negative offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOFFP2_A {
    #[doc = "0: Normal operation. No forced offset."]
    FOFFP2_0 = 0,
    #[doc = "1: Test configuration. Forced negative offset on PDAC."]
    FOFFP2_1 = 1,
}
impl From<FOFFP2_A> for bool {
    #[inline(always)]
    fn from(variant: FOFFP2_A) -> Self {
        variant as u8 != 0
    }
}
impl FOFFP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFFP2_A {
        match self.bits {
            false => FOFFP2_A::FOFFP2_0,
            true => FOFFP2_A::FOFFP2_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFFP2_0`"]
    #[inline(always)]
    pub fn is_foffp2_0(&self) -> bool {
        *self == FOFFP2_A::FOFFP2_0
    }
    #[doc = "Checks if the value of the field is `FOFFP2_1`"]
    #[inline(always)]
    pub fn is_foffp2_1(&self) -> bool {
        *self == FOFFP2_A::FOFFP2_1
    }
}
#[doc = "Field `FOFFP2` writer - Force P-side negative offset"]
pub type FOFFP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TST_SPEC, FOFFP2_A, O>;
impl<'a, const O: u8> FOFFP2_W<'a, O> {
    #[doc = "Normal operation. No forced offset."]
    #[inline(always)]
    pub fn foffp2_0(self) -> &'a mut W {
        self.variant(FOFFP2_A::FOFFP2_0)
    }
    #[doc = "Test configuration. Forced negative offset on PDAC."]
    #[inline(always)]
    pub fn foffp2_1(self) -> &'a mut W {
        self.variant(FOFFP2_A::FOFFP2_1)
    }
}
#[doc = "Field `TESTEN` reader - Enable test configuration"]
pub type TESTEN_R = crate::BitReader<TESTEN_A>;
#[doc = "Enable test configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TESTEN_A {
    #[doc = "0: Normal operation. Test configuration not enabled."]
    TESTEN_0 = 0,
    #[doc = "1: Hardware BIST Test in progress."]
    TESTEN_1 = 1,
}
impl From<TESTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TESTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TESTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TESTEN_A {
        match self.bits {
            false => TESTEN_A::TESTEN_0,
            true => TESTEN_A::TESTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TESTEN_0`"]
    #[inline(always)]
    pub fn is_testen_0(&self) -> bool {
        *self == TESTEN_A::TESTEN_0
    }
    #[doc = "Checks if the value of the field is `TESTEN_1`"]
    #[inline(always)]
    pub fn is_testen_1(&self) -> bool {
        *self == TESTEN_A::TESTEN_1
    }
}
#[doc = "Field `TESTEN` writer - Enable test configuration"]
pub type TESTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TST_SPEC, TESTEN_A, O>;
impl<'a, const O: u8> TESTEN_W<'a, O> {
    #[doc = "Normal operation. Test configuration not enabled."]
    #[inline(always)]
    pub fn testen_0(self) -> &'a mut W {
        self.variant(TESTEN_A::TESTEN_0)
    }
    #[doc = "Hardware BIST Test in progress."]
    #[inline(always)]
    pub fn testen_1(self) -> &'a mut W {
        self.variant(TESTEN_A::TESTEN_1)
    }
}
impl R {
    #[doc = "Bit 0 - Calibration Sample Time Long"]
    #[inline(always)]
    pub fn cst_long(&self) -> CST_LONG_R {
        CST_LONG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Force M-side positive offset"]
    #[inline(always)]
    pub fn foffm(&self) -> FOFFM_R {
        FOFFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Force P-side positive offset"]
    #[inline(always)]
    pub fn foffp(&self) -> FOFFP_R {
        FOFFP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Force M-side negative offset"]
    #[inline(always)]
    pub fn foffm2(&self) -> FOFFM2_R {
        FOFFM2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Force P-side negative offset"]
    #[inline(always)]
    pub fn foffp2(&self) -> FOFFP2_R {
        FOFFP2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable test configuration"]
    #[inline(always)]
    pub fn testen(&self) -> TESTEN_R {
        TESTEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Sample Time Long"]
    #[inline(always)]
    #[must_use]
    pub fn cst_long(&mut self) -> CST_LONG_W<0> {
        CST_LONG_W::new(self)
    }
    #[doc = "Bit 8 - Force M-side positive offset"]
    #[inline(always)]
    #[must_use]
    pub fn foffm(&mut self) -> FOFFM_W<8> {
        FOFFM_W::new(self)
    }
    #[doc = "Bit 9 - Force P-side positive offset"]
    #[inline(always)]
    #[must_use]
    pub fn foffp(&mut self) -> FOFFP_W<9> {
        FOFFP_W::new(self)
    }
    #[doc = "Bit 10 - Force M-side negative offset"]
    #[inline(always)]
    #[must_use]
    pub fn foffm2(&mut self) -> FOFFM2_W<10> {
        FOFFM2_W::new(self)
    }
    #[doc = "Bit 11 - Force P-side negative offset"]
    #[inline(always)]
    #[must_use]
    pub fn foffp2(&mut self) -> FOFFP2_W<11> {
        FOFFP2_W::new(self)
    }
    #[doc = "Bit 23 - Enable test configuration"]
    #[inline(always)]
    #[must_use]
    pub fn testen(&mut self) -> TESTEN_W<23> {
        TESTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst](index.html) module"]
pub struct TST_SPEC;
impl crate::RegisterSpec for TST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tst::R](R) reader structure"]
impl crate::Readable for TST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst::W](W) writer structure"]
impl crate::Writable for TST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TST to value 0"]
impl crate::Resettable for TST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
