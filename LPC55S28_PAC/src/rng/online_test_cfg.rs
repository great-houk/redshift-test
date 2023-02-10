#[doc = "Register `ONLINE_TEST_CFG` reader"]
pub struct R(crate::R<ONLINE_TEST_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ONLINE_TEST_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ONLINE_TEST_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ONLINE_TEST_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ONLINE_TEST_CFG` writer"]
pub struct W(crate::W<ONLINE_TEST_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ONLINE_TEST_CFG_SPEC>;
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
impl From<crate::W<ONLINE_TEST_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ONLINE_TEST_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVATE` reader - 0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
pub type ACTIVATE_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVATE` writer - 0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
pub type ACTIVATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ONLINE_TEST_CFG_SPEC, bool, O>;
#[doc = "Field `DATA_SEL` reader - Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
pub type DATA_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA_SEL` writer - Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
pub type DATA_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ONLINE_TEST_CFG_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - 0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
    #[inline(always)]
    pub fn activate(&self) -> ACTIVATE_R {
        ACTIVATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
    #[inline(always)]
    pub fn data_sel(&self) -> DATA_SEL_R {
        DATA_SEL_R::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: disabled 1: activated Update rythm for VAL depends on COUNTER_CFG if data_sel is set to COUNTER."]
    #[inline(always)]
    #[must_use]
    pub fn activate(&mut self) -> ACTIVATE_W<0> {
        ACTIVATE_W::new(self)
    }
    #[doc = "Bits 1:2 - Selects source on which to apply online test: 00: LSB of COUNTER: raw data from one or all sources of entropy 01: MSB of COUNTER: raw data from one or all sources of entropy 10: RANDOM_NUMBER 11: ENCRYPTED_NUMBER 'activate' should be set to 'disabled' before changing this field."]
    #[inline(always)]
    #[must_use]
    pub fn data_sel(&mut self) -> DATA_SEL_W<1> {
        DATA_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "no description available\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [online_test_cfg](index.html) module"]
pub struct ONLINE_TEST_CFG_SPEC;
impl crate::RegisterSpec for ONLINE_TEST_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [online_test_cfg::R](R) reader structure"]
impl crate::Readable for ONLINE_TEST_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [online_test_cfg::W](W) writer structure"]
impl crate::Writable for ONLINE_TEST_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ONLINE_TEST_CFG to value 0"]
impl crate::Resettable for ONLINE_TEST_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
