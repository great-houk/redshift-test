#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOWN_L` reader - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
pub type DOWN_L_R = crate::BitReader<bool>;
#[doc = "Field `DOWN_L` writer - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
pub type DOWN_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STOP_L` reader - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
pub type STOP_L_R = crate::BitReader<bool>;
#[doc = "Field `STOP_L` writer - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
pub type STOP_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `HALT_L` reader - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
pub type HALT_L_R = crate::BitReader<bool>;
#[doc = "Field `HALT_L` writer - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
pub type HALT_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CLRCTR_L` reader - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
pub type CLRCTR_L_R = crate::BitReader<bool>;
#[doc = "Field `CLRCTR_L` writer - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
pub type CLRCTR_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BIDIR_L` reader - L or unified counter direction select"]
pub type BIDIR_L_R = crate::BitReader<BIDIR_L_A>;
#[doc = "L or unified counter direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIR_L_A {
    #[doc = "0: Up. The counter counts up to a limit condition, then is cleared to zero."]
    UP = 0,
    #[doc = "1: Up-down. The counter counts up to a limit, then counts down to a limit condition or to 0."]
    UP_DOWN = 1,
}
impl From<BIDIR_L_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIR_L_A) -> Self {
        variant as u8 != 0
    }
}
impl BIDIR_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDIR_L_A {
        match self.bits {
            false => BIDIR_L_A::UP,
            true => BIDIR_L_A::UP_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == BIDIR_L_A::UP
    }
    #[doc = "Checks if the value of the field is `UP_DOWN`"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == BIDIR_L_A::UP_DOWN
    }
}
#[doc = "Field `BIDIR_L` writer - L or unified counter direction select"]
pub type BIDIR_L_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BIDIR_L_A, O>;
impl<'a, const O: u8> BIDIR_L_W<'a, O> {
    #[doc = "Up. The counter counts up to a limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(BIDIR_L_A::UP)
    }
    #[doc = "Up-down. The counter counts up to a limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut W {
        self.variant(BIDIR_L_A::UP_DOWN)
    }
}
#[doc = "Field `PRE_L` reader - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
pub type PRE_L_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE_L` writer - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
pub type PRE_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DOWN_H` reader - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
pub type DOWN_H_R = crate::BitReader<bool>;
#[doc = "Field `DOWN_H` writer - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
pub type DOWN_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `STOP_H` reader - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
pub type STOP_H_R = crate::BitReader<bool>;
#[doc = "Field `STOP_H` writer - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
pub type STOP_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `HALT_H` reader - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
pub type HALT_H_R = crate::BitReader<bool>;
#[doc = "Field `HALT_H` writer - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
pub type HALT_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CLRCTR_H` reader - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
pub type CLRCTR_H_R = crate::BitReader<bool>;
#[doc = "Field `CLRCTR_H` writer - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
pub type CLRCTR_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BIDIR_H` reader - Direction select"]
pub type BIDIR_H_R = crate::BitReader<BIDIR_H_A>;
#[doc = "Direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIR_H_A {
    #[doc = "0: The H counter counts up to its limit condition, then is cleared to zero."]
    UP = 0,
    #[doc = "1: The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    UP_DOWN = 1,
}
impl From<BIDIR_H_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIR_H_A) -> Self {
        variant as u8 != 0
    }
}
impl BIDIR_H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDIR_H_A {
        match self.bits {
            false => BIDIR_H_A::UP,
            true => BIDIR_H_A::UP_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == BIDIR_H_A::UP
    }
    #[doc = "Checks if the value of the field is `UP_DOWN`"]
    #[inline(always)]
    pub fn is_up_down(&self) -> bool {
        *self == BIDIR_H_A::UP_DOWN
    }
}
#[doc = "Field `BIDIR_H` writer - Direction select"]
pub type BIDIR_H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, BIDIR_H_A, O>;
impl<'a, const O: u8> BIDIR_H_W<'a, O> {
    #[doc = "The H counter counts up to its limit condition, then is cleared to zero."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(BIDIR_H_A::UP)
    }
    #[doc = "The H counter counts up to its limit, then counts down to a limit condition or to 0."]
    #[inline(always)]
    pub fn up_down(self) -> &'a mut W {
        self.variant(BIDIR_H_A::UP_DOWN)
    }
}
#[doc = "Field `PRE_H` reader - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
pub type PRE_H_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE_H` writer - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
pub type PRE_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_l(&self) -> DOWN_L_R {
        DOWN_L_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_l(&self) -> STOP_L_R {
        STOP_L_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub fn halt_l(&self) -> HALT_L_R {
        HALT_L_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_l(&self) -> CLRCTR_L_R {
        CLRCTR_L_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L or unified counter direction select"]
    #[inline(always)]
    pub fn bidir_l(&self) -> BIDIR_L_R {
        BIDIR_L_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:12 - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_l(&self) -> PRE_L_R {
        PRE_L_R::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bit 16 - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    pub fn down_h(&self) -> DOWN_H_R {
        DOWN_H_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    pub fn stop_h(&self) -> STOP_H_R {
        STOP_H_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    pub fn halt_h(&self) -> HALT_H_R {
        HALT_H_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[inline(always)]
    pub fn clrctr_h(&self) -> CLRCTR_H_R {
        CLRCTR_H_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Direction select"]
    #[inline(always)]
    pub fn bidir_h(&self) -> BIDIR_H_R {
        BIDIR_H_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:28 - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    pub fn pre_h(&self) -> PRE_H_R {
        PRE_H_R::new(((self.bits >> 21) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is 1 when the L or unified counter is counting down. Hardware sets this bit when the counter is counting up, counter limit occurs, and BIDIR = 1.Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    #[must_use]
    pub fn down_l(&mut self) -> DOWN_L_W<0> {
        DOWN_L_W::new(self)
    }
    #[doc = "Bit 1 - When this bit is 1 and HALT is 0, the L or unified counter does not run, but I/O events related to the counter can occur. If a designated start event occurs, this bit is cleared and counting resumes."]
    #[inline(always)]
    #[must_use]
    pub fn stop_l(&mut self) -> STOP_L_W<1> {
        STOP_L_W::new(self)
    }
    #[doc = "Bit 2 - When this bit is 1, the L or unified counter does not run and no events can occur. A reset sets this bit. When the HALT_L bit is one, the STOP_L bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, only software can clear this bit to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    #[must_use]
    pub fn halt_l(&mut self) -> HALT_L_W<2> {
        HALT_L_W::new(self)
    }
    #[doc = "Bit 3 - Writing a 1 to this bit clears the L or unified counter. This bit always reads as 0."]
    #[inline(always)]
    #[must_use]
    pub fn clrctr_l(&mut self) -> CLRCTR_L_W<3> {
        CLRCTR_L_W::new(self)
    }
    #[doc = "Bit 4 - L or unified counter direction select"]
    #[inline(always)]
    #[must_use]
    pub fn bidir_l(&mut self) -> BIDIR_L_W<4> {
        BIDIR_L_W::new(self)
    }
    #[doc = "Bits 5:12 - Specifies the factor by which the SCT clock is prescaled to produce the L or unified counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRE_L+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    #[must_use]
    pub fn pre_l(&mut self) -> PRE_L_W<5> {
        PRE_L_W::new(self)
    }
    #[doc = "Bit 16 - This bit is 1 when the H counter is counting down. Hardware sets this bit when the counter is counting, a counter limit condition occurs, and BIDIR is 1. Hardware clears this bit when the counter is counting down and a limit condition occurs or when the counter reaches 0."]
    #[inline(always)]
    #[must_use]
    pub fn down_h(&mut self) -> DOWN_H_W<16> {
        DOWN_H_W::new(self)
    }
    #[doc = "Bit 17 - When this bit is 1 and HALT is 0, the H counter does not, run but I/O events related to the counter can occur. If such an event matches the mask in the Start register, this bit is cleared and counting resumes."]
    #[inline(always)]
    #[must_use]
    pub fn stop_h(&mut self) -> STOP_H_W<17> {
        STOP_H_W::new(self)
    }
    #[doc = "Bit 18 - When this bit is 1, the H counter does not run and no events can occur. A reset sets this bit. When the HALT_H bit is one, the STOP_H bit is cleared. It is possible to remove the halt condition while keeping the SCT in the stop condition (not running) with a single write to this register to simultaneously clear the HALT bit and set the STOP bit. Once set, this bit can only be cleared by software to restore counter operation. This bit is set on reset."]
    #[inline(always)]
    #[must_use]
    pub fn halt_h(&mut self) -> HALT_H_W<18> {
        HALT_H_W::new(self)
    }
    #[doc = "Bit 19 - Writing a 1 to this bit clears the H counter. This bit always reads as 0."]
    #[inline(always)]
    #[must_use]
    pub fn clrctr_h(&mut self) -> CLRCTR_H_W<19> {
        CLRCTR_H_W::new(self)
    }
    #[doc = "Bit 20 - Direction select"]
    #[inline(always)]
    #[must_use]
    pub fn bidir_h(&mut self) -> BIDIR_H_W<20> {
        BIDIR_H_W::new(self)
    }
    #[doc = "Bits 21:28 - Specifies the factor by which the SCT clock is prescaled to produce the H counter clock. The counter clock is clocked at the rate of the SCT clock divided by PRELH+1. Clear the counter (by writing a 1 to the CLRCTR bit) whenever changing the PRE value."]
    #[inline(always)]
    #[must_use]
    pub fn pre_h(&mut self) -> PRE_H_W<21> {
        PRE_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCT control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0004_0004"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0004;
}
