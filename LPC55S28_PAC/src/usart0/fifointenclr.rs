#[doc = "Register `FIFOINTENCLR` reader"]
pub struct R(crate::R<FIFOINTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOINTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOINTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOINTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOINTENCLR` writer"]
pub struct W(crate::W<FIFOINTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOINTENCLR_SPEC>;
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
impl From<crate::W<FIFOINTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOINTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXERR` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type TXERR_R = crate::BitReader<bool>;
#[doc = "Field `TXERR` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type TXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, bool, O>;
#[doc = "Field `RXERR` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type RXERR_R = crate::BitReader<bool>;
#[doc = "Field `RXERR` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type RXERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, bool, O>;
#[doc = "Field `TXLVL` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type TXLVL_R = crate::BitReader<bool>;
#[doc = "Field `TXLVL` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type TXLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, bool, O>;
#[doc = "Field `RXLVL` reader - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type RXLVL_R = crate::BitReader<bool>;
#[doc = "Field `RXLVL` writer - Writing one clears the corresponding bits in the FIFOINTENSET register."]
pub type RXLVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FIFOINTENCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txerr(&self) -> TXERR_R {
        TXERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxerr(&self) -> RXERR_R {
        RXERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn txlvl(&self) -> TXLVL_R {
        TXLVL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    pub fn rxlvl(&self) -> RXLVL_R {
        RXLVL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn txerr(&mut self) -> TXERR_W<0> {
        TXERR_W::new(self)
    }
    #[doc = "Bit 1 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn rxerr(&mut self) -> RXERR_W<1> {
        RXERR_W::new(self)
    }
    #[doc = "Bit 2 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn txlvl(&mut self) -> TXLVL_W<2> {
        TXLVL_W::new(self)
    }
    #[doc = "Bit 3 - Writing one clears the corresponding bits in the FIFOINTENSET register."]
    #[inline(always)]
    #[must_use]
    pub fn rxlvl(&mut self) -> RXLVL_W<3> {
        RXLVL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO interrupt enable clear (disable) and read register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifointenclr](index.html) module"]
pub struct FIFOINTENCLR_SPEC;
impl crate::RegisterSpec for FIFOINTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifointenclr::R](R) reader structure"]
impl crate::Readable for FIFOINTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifointenclr::W](W) writer structure"]
impl crate::Writable for FIFOINTENCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FIFOINTENCLR to value 0"]
impl crate::Resettable for FIFOINTENCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
