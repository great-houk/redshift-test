#[doc = "Register `DIEID` reader"]
pub struct R(crate::R<DIEID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REV_ID` reader - Chip Metal Revision ID."]
pub type REV_ID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCO_NUM_IN_DIE_ID` reader - Chip Number 0x426B."]
pub type MCO_NUM_IN_DIE_ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:3 - Chip Metal Revision ID."]
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:23 - Chip Number 0x426B."]
    #[inline(always)]
    pub fn mco_num_in_die_id(&self) -> MCO_NUM_IN_DIE_ID_R {
        MCO_NUM_IN_DIE_ID_R::new((self.bits >> 4) & 0x000f_ffff)
    }
}
#[doc = "Chip revision ID and Number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieid](index.html) module"]
pub struct DIEID_SPEC;
impl crate::RegisterSpec for DIEID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dieid::R](R) reader structure"]
impl crate::Readable for DIEID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DIEID to value 0x0004_26b0"]
impl crate::Resettable for DIEID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_26b0;
}
