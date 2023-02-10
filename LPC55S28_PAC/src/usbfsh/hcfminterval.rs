#[doc = "Register `HCFMINTERVAL` reader"]
pub struct R(crate::R<HCFMINTERVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCFMINTERVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCFMINTERVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCFMINTERVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCFMINTERVAL` writer"]
pub struct W(crate::W<HCFMINTERVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCFMINTERVAL_SPEC>;
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
impl From<crate::W<HCFMINTERVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCFMINTERVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FI` reader - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
pub type FI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FI` writer - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
pub type FI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCFMINTERVAL_SPEC, u16, u16, 14, O>;
#[doc = "Field `FSMPS` reader - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
pub type FSMPS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FSMPS` writer - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
pub type FSMPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCFMINTERVAL_SPEC, u16, u16, 15, O>;
#[doc = "Field `FIT` reader - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
pub type FIT_R = crate::BitReader<bool>;
#[doc = "Field `FIT` writer - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
pub type FIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, HCFMINTERVAL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    pub fn fi(&self) -> FI_R {
        FI_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    pub fn fsmps(&self) -> FSMPS_R {
        FSMPS_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    pub fn fit(&self) -> FIT_R {
        FIT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - FrameInterval This specifies the interval between two consecutive SOFs in bit times."]
    #[inline(always)]
    #[must_use]
    pub fn fi(&mut self) -> FI_W<0> {
        FI_W::new(self)
    }
    #[doc = "Bits 16:30 - FSLargestDataPacket This field specifies a value which is loaded into the Largest Data Packet Counter at the beginning of each frame."]
    #[inline(always)]
    #[must_use]
    pub fn fsmps(&mut self) -> FSMPS_W<16> {
        FSMPS_W::new(self)
    }
    #[doc = "Bit 31 - FrameIntervalToggle HCD toggles this bit whenever it loads a new value to FrameInterval."]
    #[inline(always)]
    #[must_use]
    pub fn fit(&mut self) -> FIT_W<31> {
        FIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Defines the bit time interval in a frame and the full speed maximum packet size which would not cause an overrun\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfminterval](index.html) module"]
pub struct HCFMINTERVAL_SPEC;
impl crate::RegisterSpec for HCFMINTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcfminterval::R](R) reader structure"]
impl crate::Readable for HCFMINTERVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcfminterval::W](W) writer structure"]
impl crate::Writable for HCFMINTERVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HCFMINTERVAL to value 0x2edf"]
impl crate::Resettable for HCFMINTERVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2edf;
}
