#[doc = "Register `XTAL_16MHZ_CAPABANK_TRIM` reader"]
pub struct R(crate::R<XTAL_16MHZ_CAPABANK_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTAL_16MHZ_CAPABANK_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTAL_16MHZ_CAPABANK_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTAL_16MHZ_CAPABANK_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTAL_16MHZ_CAPABANK_TRIM` writer"]
pub struct W(crate::W<XTAL_16MHZ_CAPABANK_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTAL_16MHZ_CAPABANK_TRIM_SPEC>;
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
impl From<crate::W<XTAL_16MHZ_CAPABANK_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTAL_16MHZ_CAPABANK_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM_VALID` reader - 0 : Capa Bank trimmings not valid. Default trimmings value are used. 1 : Capa Bank trimmings valid."]
pub type TRIM_VALID_R = crate::BitReader<bool>;
#[doc = "Field `TRIM_VALID` writer - 0 : Capa Bank trimmings not valid. Default trimmings value are used. 1 : Capa Bank trimmings valid."]
pub type TRIM_VALID_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, XTAL_16MHZ_CAPABANK_TRIM_SPEC, bool, O>;
#[doc = "Field `XTAL_LOAD_CAP_IEC_PF_X100` reader - Load capacitance, pF x 100. For example, 6pF becomes 600."]
pub type XTAL_LOAD_CAP_IEC_PF_X100_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XTAL_LOAD_CAP_IEC_PF_X100` writer - Load capacitance, pF x 100. For example, 6pF becomes 600."]
pub type XTAL_LOAD_CAP_IEC_PF_X100_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_16MHZ_CAPABANK_TRIM_SPEC, u16, u16, 10, O>;
#[doc = "Field `PCB_XIN_PARA_CAP_PF_X100` reader - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub type PCB_XIN_PARA_CAP_PF_X100_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCB_XIN_PARA_CAP_PF_X100` writer - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub type PCB_XIN_PARA_CAP_PF_X100_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_16MHZ_CAPABANK_TRIM_SPEC, u16, u16, 10, O>;
#[doc = "Field `PCB_XOUT_PARA_CAP_PF_X100` reader - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub type PCB_XOUT_PARA_CAP_PF_X100_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PCB_XOUT_PARA_CAP_PF_X100` writer - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
pub type PCB_XOUT_PARA_CAP_PF_X100_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, XTAL_16MHZ_CAPABANK_TRIM_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bit 0 - 0 : Capa Bank trimmings not valid. Default trimmings value are used. 1 : Capa Bank trimmings valid."]
    #[inline(always)]
    pub fn trim_valid(&self) -> TRIM_VALID_R {
        TRIM_VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:10 - Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn xtal_load_cap_iec_pf_x100(&self) -> XTAL_LOAD_CAP_IEC_PF_X100_R {
        XTAL_LOAD_CAP_IEC_PF_X100_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 11:20 - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn pcb_xin_para_cap_pf_x100(&self) -> PCB_XIN_PARA_CAP_PF_X100_R {
        PCB_XIN_PARA_CAP_PF_X100_R::new(((self.bits >> 11) & 0x03ff) as u16)
    }
    #[doc = "Bits 21:30 - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    pub fn pcb_xout_para_cap_pf_x100(&self) -> PCB_XOUT_PARA_CAP_PF_X100_R {
        PCB_XOUT_PARA_CAP_PF_X100_R::new(((self.bits >> 21) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0 : Capa Bank trimmings not valid. Default trimmings value are used. 1 : Capa Bank trimmings valid."]
    #[inline(always)]
    #[must_use]
    pub fn trim_valid(&mut self) -> TRIM_VALID_W<0> {
        TRIM_VALID_W::new(self)
    }
    #[doc = "Bits 1:10 - Load capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    #[must_use]
    pub fn xtal_load_cap_iec_pf_x100(&mut self) -> XTAL_LOAD_CAP_IEC_PF_X100_W<1> {
        XTAL_LOAD_CAP_IEC_PF_X100_W::new(self)
    }
    #[doc = "Bits 11:20 - PCB XIN parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    #[must_use]
    pub fn pcb_xin_para_cap_pf_x100(&mut self) -> PCB_XIN_PARA_CAP_PF_X100_W<11> {
        PCB_XIN_PARA_CAP_PF_X100_W::new(self)
    }
    #[doc = "Bits 21:30 - PCB XOUT parasitic capacitance, pF x 100. For example, 6pF becomes 600."]
    #[inline(always)]
    #[must_use]
    pub fn pcb_xout_para_cap_pf_x100(&mut self) -> PCB_XOUT_PARA_CAP_PF_X100_W<21> {
        PCB_XOUT_PARA_CAP_PF_X100_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Xtal 16MHz capabank triming.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtal_16mhz_capabank_trim](index.html) module"]
pub struct XTAL_16MHZ_CAPABANK_TRIM_SPEC;
impl crate::RegisterSpec for XTAL_16MHZ_CAPABANK_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xtal_16mhz_capabank_trim::R](R) reader structure"]
impl crate::Readable for XTAL_16MHZ_CAPABANK_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xtal_16mhz_capabank_trim::W](W) writer structure"]
impl crate::Writable for XTAL_16MHZ_CAPABANK_TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTAL_16MHZ_CAPABANK_TRIM to value 0"]
impl crate::Resettable for XTAL_16MHZ_CAPABANK_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
