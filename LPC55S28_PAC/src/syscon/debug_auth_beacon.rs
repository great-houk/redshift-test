#[doc = "Register `DEBUG_AUTH_BEACON` reader"]
pub struct R(crate::R<DEBUG_AUTH_BEACON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_AUTH_BEACON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_AUTH_BEACON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_AUTH_BEACON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_AUTH_BEACON` writer"]
pub struct W(crate::W<DEBUG_AUTH_BEACON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_AUTH_BEACON_SPEC>;
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
impl From<crate::W<DEBUG_AUTH_BEACON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_AUTH_BEACON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BEACON` reader - Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
pub type BEACON_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BEACON` writer - Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
pub type BEACON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG_AUTH_BEACON_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
    #[inline(always)]
    pub fn beacon(&self) -> BEACON_R {
        BEACON_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set by the debug authentication code in ROM to pass the debug beacons (Credential Beacon and Authentication Beacon) to application code."]
    #[inline(always)]
    #[must_use]
    pub fn beacon(&mut self) -> BEACON_W<0> {
        BEACON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug authentication BEACON register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_auth_beacon](index.html) module"]
pub struct DEBUG_AUTH_BEACON_SPEC;
impl crate::RegisterSpec for DEBUG_AUTH_BEACON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_auth_beacon::R](R) reader structure"]
impl crate::Readable for DEBUG_AUTH_BEACON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_auth_beacon::W](W) writer structure"]
impl crate::Writable for DEBUG_AUTH_BEACON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG_AUTH_BEACON to value 0"]
impl crate::Resettable for DEBUG_AUTH_BEACON_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
