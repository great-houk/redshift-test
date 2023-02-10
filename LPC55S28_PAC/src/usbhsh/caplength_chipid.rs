#[doc = "Register `CAPLENGTH_CHIPID` reader"]
pub struct R(crate::R<CAPLENGTH_CHIPID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPLENGTH_CHIPID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPLENGTH_CHIPID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPLENGTH_CHIPID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CAPLENGTH` reader - Capability Length: This is used as an offset."]
pub type CAPLENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHIPID` reader - Chip identification: indicates major and minor revision of the IP: \\[31:24\\]
= Major revision \\[23:16\\]
= Minor revision Major revisions used: 0x01: USB2."]
pub type CHIPID_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:7 - Capability Length: This is used as an offset."]
    #[inline(always)]
    pub fn caplength(&self) -> CAPLENGTH_R {
        CAPLENGTH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Chip identification: indicates major and minor revision of the IP: \\[31:24\\]
= Major revision \\[23:16\\]
= Minor revision Major revisions used: 0x01: USB2."]
    #[inline(always)]
    pub fn chipid(&self) -> CHIPID_R {
        CHIPID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "This register contains the offset value towards the start of the operational register space and the version number of the IP block\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [caplength_chipid](index.html) module"]
pub struct CAPLENGTH_CHIPID_SPEC;
impl crate::RegisterSpec for CAPLENGTH_CHIPID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [caplength_chipid::R](R) reader structure"]
impl crate::Readable for CAPLENGTH_CHIPID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAPLENGTH_CHIPID to value 0x0101_0010"]
impl crate::Resettable for CAPLENGTH_CHIPID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101_0010;
}
