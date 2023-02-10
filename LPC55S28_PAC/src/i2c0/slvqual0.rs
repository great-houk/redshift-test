#[doc = "Register `SLVQUAL0` reader"]
pub struct R(crate::R<SLVQUAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLVQUAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLVQUAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLVQUAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLVQUAL0` writer"]
pub struct W(crate::W<SLVQUAL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLVQUAL0_SPEC>;
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
impl From<crate::W<SLVQUAL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLVQUAL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QUALMODE0` reader - Qualify mode for slave address 0."]
pub type QUALMODE0_R = crate::BitReader<QUALMODE0_A>;
#[doc = "Qualify mode for slave address 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QUALMODE0_A {
    #[doc = "0: Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    MASK = 0,
    #[doc = "1: Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    EXTEND = 1,
}
impl From<QUALMODE0_A> for bool {
    #[inline(always)]
    fn from(variant: QUALMODE0_A) -> Self {
        variant as u8 != 0
    }
}
impl QUALMODE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QUALMODE0_A {
        match self.bits {
            false => QUALMODE0_A::MASK,
            true => QUALMODE0_A::EXTEND,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == QUALMODE0_A::MASK
    }
    #[doc = "Checks if the value of the field is `EXTEND`"]
    #[inline(always)]
    pub fn is_extend(&self) -> bool {
        *self == QUALMODE0_A::EXTEND
    }
}
#[doc = "Field `QUALMODE0` writer - Qualify mode for slave address 0."]
pub type QUALMODE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SLVQUAL0_SPEC, QUALMODE0_A, O>;
impl<'a, const O: u8> QUALMODE0_W<'a, O> {
    #[doc = "Mask. The SLVQUAL0 field is used as a logical mask for matching address 0."]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(QUALMODE0_A::MASK)
    }
    #[doc = "Extend. The SLVQUAL0 field is used to extend address 0 matching in a range of addresses."]
    #[inline(always)]
    pub fn extend(self) -> &'a mut W {
        self.variant(QUALMODE0_A::EXTEND)
    }
}
#[doc = "Field `SLVQUAL0` reader - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\]
<= received address <= SLVQUAL0\\[7:1\\])."]
pub type SLVQUAL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLVQUAL0` writer - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\]
<= received address <= SLVQUAL0\\[7:1\\])."]
pub type SLVQUAL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SLVQUAL0_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bit 0 - Qualify mode for slave address 0."]
    #[inline(always)]
    pub fn qualmode0(&self) -> QUALMODE0_R {
        QUALMODE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\]
<= received address <= SLVQUAL0\\[7:1\\])."]
    #[inline(always)]
    pub fn slvqual0(&self) -> SLVQUAL0_R {
        SLVQUAL0_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Qualify mode for slave address 0."]
    #[inline(always)]
    #[must_use]
    pub fn qualmode0(&mut self) -> QUALMODE0_W<0> {
        QUALMODE0_W::new(self)
    }
    #[doc = "Bits 1:7 - Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\]
<= received address <= SLVQUAL0\\[7:1\\])."]
    #[inline(always)]
    #[must_use]
    pub fn slvqual0(&mut self) -> SLVQUAL0_W<1> {
        SLVQUAL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Qualification for address 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slvqual0](index.html) module"]
pub struct SLVQUAL0_SPEC;
impl crate::RegisterSpec for SLVQUAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slvqual0::R](R) reader structure"]
impl crate::Readable for SLVQUAL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slvqual0::W](W) writer structure"]
impl crate::Writable for SLVQUAL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLVQUAL0 to value 0"]
impl crate::Resettable for SLVQUAL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
