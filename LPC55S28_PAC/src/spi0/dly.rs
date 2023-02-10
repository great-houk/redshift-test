#[doc = "Register `DLY` reader"]
pub struct R(crate::R<DLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DLY` writer"]
pub struct W(crate::W<DLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DLY_SPEC>;
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
impl From<crate::W<DLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRE_DELAY` reader - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type PRE_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE_DELAY` writer - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type PRE_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLY_SPEC, u8, u8, 4, O>;
#[doc = "Field `POST_DELAY` reader - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type POST_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POST_DELAY` writer - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type POST_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLY_SPEC, u8, u8, 4, O>;
#[doc = "Field `FRAME_DELAY` reader - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type FRAME_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAME_DELAY` writer - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
pub type FRAME_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLY_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRANSFER_DELAY` reader - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
pub type TRANSFER_DELAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRANSFER_DELAY` writer - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
pub type TRANSFER_DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DLY_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn pre_delay(&self) -> PRE_DELAY_R {
        PRE_DELAY_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn post_delay(&self) -> POST_DELAY_R {
        POST_DELAY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    pub fn frame_delay(&self) -> FRAME_DELAY_R {
        FRAME_DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    pub fn transfer_delay(&self) -> TRANSFER_DELAY_R {
        TRANSFER_DELAY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Controls the amount of time between SSEL assertion and the beginning of a data transfer. There is always one SPI clock time between SSEL assertion and the first clock edge. This is not considered part of the pre-delay. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    #[must_use]
    pub fn pre_delay(&mut self) -> PRE_DELAY_W<0> {
        PRE_DELAY_W::new(self)
    }
    #[doc = "Bits 4:7 - Controls the amount of time between the end of a data transfer and SSEL deassertion. 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    #[must_use]
    pub fn post_delay(&mut self) -> POST_DELAY_W<4> {
        POST_DELAY_W::new(self)
    }
    #[doc = "Bits 8:11 - If the EOF flag is set, controls the minimum amount of time between the current frame and the next frame (or SSEL deassertion if EOT). 0x0 = No additional time is inserted. 0x1 = 1 SPI clock time is inserted. 0x2 = 2 SPI clock times are inserted. 0xF = 15 SPI clock times are inserted."]
    #[inline(always)]
    #[must_use]
    pub fn frame_delay(&mut self) -> FRAME_DELAY_W<8> {
        FRAME_DELAY_W::new(self)
    }
    #[doc = "Bits 12:15 - Controls the minimum amount of time that the SSEL is deasserted between transfers. 0x0 = The minimum time that SSEL is deasserted is 1 SPI clock time. (Zero added time.) 0x1 = The minimum time that SSEL is deasserted is 2 SPI clock times. 0x2 = The minimum time that SSEL is deasserted is 3 SPI clock times. 0xF = The minimum time that SSEL is deasserted is 16 SPI clock times."]
    #[inline(always)]
    #[must_use]
    pub fn transfer_delay(&mut self) -> TRANSFER_DELAY_W<12> {
        TRANSFER_DELAY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dly](index.html) module"]
pub struct DLY_SPEC;
impl crate::RegisterSpec for DLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dly::R](R) reader structure"]
impl crate::Readable for DLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dly::W](W) writer structure"]
impl crate::Writable for DLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DLY to value 0"]
impl crate::Resettable for DLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
