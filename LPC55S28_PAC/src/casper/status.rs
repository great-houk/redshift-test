#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DONE` reader - Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
pub type DONE_R = crate::BitReader<DONE_A>;
#[doc = "Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DONE_A {
    #[doc = "0: Busy or just cleared"]
    BUSY = 0,
    #[doc = "1: Completed last operation"]
    COMPLETED = 1,
}
impl From<DONE_A> for bool {
    #[inline(always)]
    fn from(variant: DONE_A) -> Self {
        variant as u8 != 0
    }
}
impl DONE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DONE_A {
        match self.bits {
            false => DONE_A::BUSY,
            true => DONE_A::COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == DONE_A::BUSY
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == DONE_A::COMPLETED
    }
}
#[doc = "Field `DONE` writer - Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, DONE_A, O>;
impl<'a, const O: u8> DONE_W<'a, O> {
    #[doc = "Busy or just cleared"]
    #[inline(always)]
    pub fn busy(self) -> &'a mut W {
        self.variant(DONE_A::BUSY)
    }
    #[doc = "Completed last operation"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(DONE_A::COMPLETED)
    }
}
#[doc = "Field `CARRY` reader - Last carry value if operation produced a carry bit"]
pub type CARRY_R = crate::BitReader<CARRY_A>;
#[doc = "Last carry value if operation produced a carry bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARRY_A {
    #[doc = "0: Carry was 0 or no carry"]
    NO_CARRY = 0,
    #[doc = "1: Carry was 1"]
    CARRY = 1,
}
impl From<CARRY_A> for bool {
    #[inline(always)]
    fn from(variant: CARRY_A) -> Self {
        variant as u8 != 0
    }
}
impl CARRY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARRY_A {
        match self.bits {
            false => CARRY_A::NO_CARRY,
            true => CARRY_A::CARRY,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CARRY`"]
    #[inline(always)]
    pub fn is_no_carry(&self) -> bool {
        *self == CARRY_A::NO_CARRY
    }
    #[doc = "Checks if the value of the field is `CARRY`"]
    #[inline(always)]
    pub fn is_carry(&self) -> bool {
        *self == CARRY_A::CARRY
    }
}
#[doc = "Field `BUSY` reader - Indicates if the accelerator is busy performing an operation"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Indicates if the accelerator is busy performing an operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: Not busy - is idle"]
    IDLE = 0,
    #[doc = "1: Is busy"]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::IDLE,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Last carry value if operation produced a carry bit"]
    #[inline(always)]
    pub fn carry(&self) -> CARRY_R {
        CARRY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates if the accelerator is busy performing an operation"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates if the accelerator has finished an operation. Write 1 to clear, or write CTRL1 to clear."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<0> {
        DONE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicates operational status and would contain the carry bit if used.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
