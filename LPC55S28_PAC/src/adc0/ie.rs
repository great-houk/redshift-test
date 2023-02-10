#[doc = "Register `IE` reader"]
pub struct R(crate::R<IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE` writer"]
pub struct W(crate::W<IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE_SPEC>;
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
impl From<crate::W<IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FWMIE0` reader - FIFO 0 Watermark Interrupt Enable"]
pub type FWMIE0_R = crate::BitReader<FWMIE0_A>;
#[doc = "FIFO 0 Watermark Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWMIE0_A {
    #[doc = "0: FIFO 0 watermark interrupts are not enabled."]
    FWMIE0_0 = 0,
    #[doc = "1: FIFO 0 watermark interrupts are enabled."]
    FWMIE0_1 = 1,
}
impl From<FWMIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FWMIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl FWMIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWMIE0_A {
        match self.bits {
            false => FWMIE0_A::FWMIE0_0,
            true => FWMIE0_A::FWMIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMIE0_0`"]
    #[inline(always)]
    pub fn is_fwmie0_0(&self) -> bool {
        *self == FWMIE0_A::FWMIE0_0
    }
    #[doc = "Checks if the value of the field is `FWMIE0_1`"]
    #[inline(always)]
    pub fn is_fwmie0_1(&self) -> bool {
        *self == FWMIE0_A::FWMIE0_1
    }
}
#[doc = "Field `FWMIE0` writer - FIFO 0 Watermark Interrupt Enable"]
pub type FWMIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, FWMIE0_A, O>;
impl<'a, const O: u8> FWMIE0_W<'a, O> {
    #[doc = "FIFO 0 watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn fwmie0_0(self) -> &'a mut W {
        self.variant(FWMIE0_A::FWMIE0_0)
    }
    #[doc = "FIFO 0 watermark interrupts are enabled."]
    #[inline(always)]
    pub fn fwmie0_1(self) -> &'a mut W {
        self.variant(FWMIE0_A::FWMIE0_1)
    }
}
#[doc = "Field `FOFIE0` reader - Result FIFO 0 Overflow Interrupt Enable"]
pub type FOFIE0_R = crate::BitReader<FOFIE0_A>;
#[doc = "Result FIFO 0 Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOFIE0_A {
    #[doc = "0: FIFO 0 overflow interrupts are not enabled."]
    FOFIE0_0 = 0,
    #[doc = "1: FIFO 0 overflow interrupts are enabled."]
    FOFIE0_1 = 1,
}
impl From<FOFIE0_A> for bool {
    #[inline(always)]
    fn from(variant: FOFIE0_A) -> Self {
        variant as u8 != 0
    }
}
impl FOFIE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFIE0_A {
        match self.bits {
            false => FOFIE0_A::FOFIE0_0,
            true => FOFIE0_A::FOFIE0_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFIE0_0`"]
    #[inline(always)]
    pub fn is_fofie0_0(&self) -> bool {
        *self == FOFIE0_A::FOFIE0_0
    }
    #[doc = "Checks if the value of the field is `FOFIE0_1`"]
    #[inline(always)]
    pub fn is_fofie0_1(&self) -> bool {
        *self == FOFIE0_A::FOFIE0_1
    }
}
#[doc = "Field `FOFIE0` writer - Result FIFO 0 Overflow Interrupt Enable"]
pub type FOFIE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, FOFIE0_A, O>;
impl<'a, const O: u8> FOFIE0_W<'a, O> {
    #[doc = "FIFO 0 overflow interrupts are not enabled."]
    #[inline(always)]
    pub fn fofie0_0(self) -> &'a mut W {
        self.variant(FOFIE0_A::FOFIE0_0)
    }
    #[doc = "FIFO 0 overflow interrupts are enabled."]
    #[inline(always)]
    pub fn fofie0_1(self) -> &'a mut W {
        self.variant(FOFIE0_A::FOFIE0_1)
    }
}
#[doc = "Field `FWMIE1` reader - FIFO1 Watermark Interrupt Enable"]
pub type FWMIE1_R = crate::BitReader<FWMIE1_A>;
#[doc = "FIFO1 Watermark Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWMIE1_A {
    #[doc = "0: FIFO1 watermark interrupts are not enabled."]
    FWMIE1_0 = 0,
    #[doc = "1: FIFO1 watermark interrupts are enabled."]
    FWMIE1_1 = 1,
}
impl From<FWMIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FWMIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl FWMIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWMIE1_A {
        match self.bits {
            false => FWMIE1_A::FWMIE1_0,
            true => FWMIE1_A::FWMIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FWMIE1_0`"]
    #[inline(always)]
    pub fn is_fwmie1_0(&self) -> bool {
        *self == FWMIE1_A::FWMIE1_0
    }
    #[doc = "Checks if the value of the field is `FWMIE1_1`"]
    #[inline(always)]
    pub fn is_fwmie1_1(&self) -> bool {
        *self == FWMIE1_A::FWMIE1_1
    }
}
#[doc = "Field `FWMIE1` writer - FIFO1 Watermark Interrupt Enable"]
pub type FWMIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, FWMIE1_A, O>;
impl<'a, const O: u8> FWMIE1_W<'a, O> {
    #[doc = "FIFO1 watermark interrupts are not enabled."]
    #[inline(always)]
    pub fn fwmie1_0(self) -> &'a mut W {
        self.variant(FWMIE1_A::FWMIE1_0)
    }
    #[doc = "FIFO1 watermark interrupts are enabled."]
    #[inline(always)]
    pub fn fwmie1_1(self) -> &'a mut W {
        self.variant(FWMIE1_A::FWMIE1_1)
    }
}
#[doc = "Field `FOFIE1` reader - Result FIFO1 Overflow Interrupt Enable"]
pub type FOFIE1_R = crate::BitReader<FOFIE1_A>;
#[doc = "Result FIFO1 Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOFIE1_A {
    #[doc = "0: No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_0 = 0,
    #[doc = "1: At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    FOFIE1_1 = 1,
}
impl From<FOFIE1_A> for bool {
    #[inline(always)]
    fn from(variant: FOFIE1_A) -> Self {
        variant as u8 != 0
    }
}
impl FOFIE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFIE1_A {
        match self.bits {
            false => FOFIE1_A::FOFIE1_0,
            true => FOFIE1_A::FOFIE1_1,
        }
    }
    #[doc = "Checks if the value of the field is `FOFIE1_0`"]
    #[inline(always)]
    pub fn is_fofie1_0(&self) -> bool {
        *self == FOFIE1_A::FOFIE1_0
    }
    #[doc = "Checks if the value of the field is `FOFIE1_1`"]
    #[inline(always)]
    pub fn is_fofie1_1(&self) -> bool {
        *self == FOFIE1_A::FOFIE1_1
    }
}
#[doc = "Field `FOFIE1` writer - Result FIFO1 Overflow Interrupt Enable"]
pub type FOFIE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, FOFIE1_A, O>;
impl<'a, const O: u8> FOFIE1_W<'a, O> {
    #[doc = "No result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fofie1_0(self) -> &'a mut W {
        self.variant(FOFIE1_A::FOFIE1_0)
    }
    #[doc = "At least one result FIFO1 overflow has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn fofie1_1(self) -> &'a mut W {
        self.variant(FOFIE1_A::FOFIE1_1)
    }
}
#[doc = "Field `TEXC_IE` reader - Trigger Exception Interrupt Enable"]
pub type TEXC_IE_R = crate::BitReader<TEXC_IE_A>;
#[doc = "Trigger Exception Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEXC_IE_A {
    #[doc = "0: Trigger exception interrupts are disabled."]
    TEXC_IE_0 = 0,
    #[doc = "1: Trigger exception interrupts are enabled."]
    TEXC_IE_1 = 1,
}
impl From<TEXC_IE_A> for bool {
    #[inline(always)]
    fn from(variant: TEXC_IE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEXC_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEXC_IE_A {
        match self.bits {
            false => TEXC_IE_A::TEXC_IE_0,
            true => TEXC_IE_A::TEXC_IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEXC_IE_0`"]
    #[inline(always)]
    pub fn is_texc_ie_0(&self) -> bool {
        *self == TEXC_IE_A::TEXC_IE_0
    }
    #[doc = "Checks if the value of the field is `TEXC_IE_1`"]
    #[inline(always)]
    pub fn is_texc_ie_1(&self) -> bool {
        *self == TEXC_IE_A::TEXC_IE_1
    }
}
#[doc = "Field `TEXC_IE` writer - Trigger Exception Interrupt Enable"]
pub type TEXC_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IE_SPEC, TEXC_IE_A, O>;
impl<'a, const O: u8> TEXC_IE_W<'a, O> {
    #[doc = "Trigger exception interrupts are disabled."]
    #[inline(always)]
    pub fn texc_ie_0(self) -> &'a mut W {
        self.variant(TEXC_IE_A::TEXC_IE_0)
    }
    #[doc = "Trigger exception interrupts are enabled."]
    #[inline(always)]
    pub fn texc_ie_1(self) -> &'a mut W {
        self.variant(TEXC_IE_A::TEXC_IE_1)
    }
}
#[doc = "Field `TCOMP_IE` reader - Trigger Completion Interrupt Enable"]
pub type TCOMP_IE_R = crate::FieldReader<u16, TCOMP_IE_A>;
#[doc = "Trigger Completion Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TCOMP_IE_A {
    #[doc = "0: Trigger completion interrupts are disabled."]
    TCOMP_IE_0 = 0,
    #[doc = "1: Trigger completion interrupts are enabled for trigger source 0 only."]
    TCOMP_IE_1 = 1,
    #[doc = "2: Trigger completion interrupts are enabled for trigger source 1 only."]
    TCOMP_IE_2 = 2,
    #[doc = "3: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_3 = 3,
    #[doc = "4: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_4 = 4,
    #[doc = "5: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_5 = 5,
    #[doc = "6: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_6 = 6,
    #[doc = "7: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_7 = 7,
    #[doc = "8: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_8 = 8,
    #[doc = "9: Associated trigger completion interrupts are enabled."]
    TCOMP_IE_9 = 9,
    #[doc = "65535: Trigger completion interrupts are enabled for every trigger source."]
    TCOMP_IE_65535 = 65535,
}
impl From<TCOMP_IE_A> for u16 {
    #[inline(always)]
    fn from(variant: TCOMP_IE_A) -> Self {
        variant as _
    }
}
impl TCOMP_IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCOMP_IE_A> {
        match self.bits {
            0 => Some(TCOMP_IE_A::TCOMP_IE_0),
            1 => Some(TCOMP_IE_A::TCOMP_IE_1),
            2 => Some(TCOMP_IE_A::TCOMP_IE_2),
            3 => Some(TCOMP_IE_A::TCOMP_IE_3),
            4 => Some(TCOMP_IE_A::TCOMP_IE_4),
            5 => Some(TCOMP_IE_A::TCOMP_IE_5),
            6 => Some(TCOMP_IE_A::TCOMP_IE_6),
            7 => Some(TCOMP_IE_A::TCOMP_IE_7),
            8 => Some(TCOMP_IE_A::TCOMP_IE_8),
            9 => Some(TCOMP_IE_A::TCOMP_IE_9),
            65535 => Some(TCOMP_IE_A::TCOMP_IE_65535),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_0`"]
    #[inline(always)]
    pub fn is_tcomp_ie_0(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_0
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_1`"]
    #[inline(always)]
    pub fn is_tcomp_ie_1(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_1
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_2`"]
    #[inline(always)]
    pub fn is_tcomp_ie_2(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_2
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_3`"]
    #[inline(always)]
    pub fn is_tcomp_ie_3(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_3
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_4`"]
    #[inline(always)]
    pub fn is_tcomp_ie_4(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_4
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_5`"]
    #[inline(always)]
    pub fn is_tcomp_ie_5(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_5
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_6`"]
    #[inline(always)]
    pub fn is_tcomp_ie_6(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_6
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_7`"]
    #[inline(always)]
    pub fn is_tcomp_ie_7(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_7
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_8`"]
    #[inline(always)]
    pub fn is_tcomp_ie_8(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_8
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_9`"]
    #[inline(always)]
    pub fn is_tcomp_ie_9(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_9
    }
    #[doc = "Checks if the value of the field is `TCOMP_IE_65535`"]
    #[inline(always)]
    pub fn is_tcomp_ie_65535(&self) -> bool {
        *self == TCOMP_IE_A::TCOMP_IE_65535
    }
}
#[doc = "Field `TCOMP_IE` writer - Trigger Completion Interrupt Enable"]
pub type TCOMP_IE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IE_SPEC, u16, TCOMP_IE_A, 16, O>;
impl<'a, const O: u8> TCOMP_IE_W<'a, O> {
    #[doc = "Trigger completion interrupts are disabled."]
    #[inline(always)]
    pub fn tcomp_ie_0(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_0)
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 0 only."]
    #[inline(always)]
    pub fn tcomp_ie_1(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_1)
    }
    #[doc = "Trigger completion interrupts are enabled for trigger source 1 only."]
    #[inline(always)]
    pub fn tcomp_ie_2(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_2)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_3(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_3)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_4(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_4)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_5(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_5)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_6(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_6)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_7(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_7)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_8(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_8)
    }
    #[doc = "Associated trigger completion interrupts are enabled."]
    #[inline(always)]
    pub fn tcomp_ie_9(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_9)
    }
    #[doc = "Trigger completion interrupts are enabled for every trigger source."]
    #[inline(always)]
    pub fn tcomp_ie_65535(self) -> &'a mut W {
        self.variant(TCOMP_IE_A::TCOMP_IE_65535)
    }
}
impl R {
    #[doc = "Bit 0 - FIFO 0 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie0(&self) -> FWMIE0_R {
        FWMIE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie0(&self) -> FOFIE0_R {
        FOFIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO1 Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn fwmie1(&self) -> FWMIE1_R {
        FWMIE1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofie1(&self) -> FOFIE1_R {
        FOFIE1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Trigger Exception Interrupt Enable"]
    #[inline(always)]
    pub fn texc_ie(&self) -> TEXC_IE_R {
        TEXC_IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Trigger Completion Interrupt Enable"]
    #[inline(always)]
    pub fn tcomp_ie(&self) -> TCOMP_IE_R {
        TCOMP_IE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO 0 Watermark Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwmie0(&mut self) -> FWMIE0_W<0> {
        FWMIE0_W::new(self)
    }
    #[doc = "Bit 1 - Result FIFO 0 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fofie0(&mut self) -> FOFIE0_W<1> {
        FOFIE0_W::new(self)
    }
    #[doc = "Bit 2 - FIFO1 Watermark Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwmie1(&mut self) -> FWMIE1_W<2> {
        FWMIE1_W::new(self)
    }
    #[doc = "Bit 3 - Result FIFO1 Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fofie1(&mut self) -> FOFIE1_W<3> {
        FOFIE1_W::new(self)
    }
    #[doc = "Bit 8 - Trigger Exception Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn texc_ie(&mut self) -> TEXC_IE_W<8> {
        TEXC_IE_W::new(self)
    }
    #[doc = "Bits 16:31 - Trigger Completion Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcomp_ie(&mut self) -> TCOMP_IE_W<16> {
        TCOMP_IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie](index.html) module"]
pub struct IE_SPEC;
impl crate::RegisterSpec for IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ie::R](R) reader structure"]
impl crate::Readable for IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie::W](W) writer structure"]
impl crate::Writable for IE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
