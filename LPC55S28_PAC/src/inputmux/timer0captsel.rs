#[doc = "Register `TIMER0CAPTSEL[%s]` reader"]
pub struct R(crate::R<TIMER0CAPTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0CAPTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0CAPTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0CAPTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0CAPTSEL[%s]` writer"]
pub struct W(crate::W<TIMER0CAPTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0CAPTSEL_SPEC>;
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
impl From<crate::W<TIMER0CAPTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0CAPTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAPTSEL` reader - Input number to TIMER0 capture inputs 0 to 4"]
pub type CAPTSEL_R = crate::FieldReader<u8, CAPTSEL_A>;
#[doc = "Input number to TIMER0 capture inputs 0 to 4\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAPTSEL_A {
    #[doc = "0: CT_INP0 function selected from IOCON register"]
    VAL0 = 0,
    #[doc = "1: CT_INP1 function selected from IOCON register"]
    VAL1 = 1,
    #[doc = "2: CT_INP2 function selected from IOCON register"]
    VAL2 = 2,
    #[doc = "3: CT_INP3 function selected from IOCON register"]
    VAL3 = 3,
    #[doc = "4: CT_INP4 function selected from IOCON register"]
    VAL4 = 4,
    #[doc = "5: CT_INP5 function selected from IOCON register"]
    VAL5 = 5,
    #[doc = "6: CT_INP6 function selected from IOCON register"]
    VAL6 = 6,
    #[doc = "7: CT_INP7 function selected from IOCON register"]
    VAL7 = 7,
    #[doc = "8: CT_INP8 function selected from IOCON register"]
    VAL8 = 8,
    #[doc = "9: CT_INP9 function selected from IOCON register"]
    VAL9 = 9,
    #[doc = "10: CT_INP10 function selected from IOCON register"]
    VAL10 = 10,
    #[doc = "11: CT_INP11 function selected from IOCON register"]
    VAL11 = 11,
    #[doc = "12: CT_INP12 function selected from IOCON register"]
    VAL12 = 12,
    #[doc = "13: CT_INP13 function selected from IOCON register"]
    VAL13 = 13,
    #[doc = "14: CT_INP14 function selected from IOCON register"]
    VAL14 = 14,
    #[doc = "15: CT_INP15 function selected from IOCON register"]
    VAL15 = 15,
    #[doc = "16: CT_INP16 function selected from IOCON register"]
    VAL16 = 16,
    #[doc = "17: None"]
    VAL17 = 17,
    #[doc = "18: None"]
    VAL18 = 18,
    #[doc = "19: None"]
    VAL19 = 19,
    #[doc = "20: USB0_FRAME_TOGGLE"]
    VAL20 = 20,
    #[doc = "21: USB1_FRAME_TOGGLE"]
    VAL21 = 21,
    #[doc = "22: COMP_OUTPUT output from analog comparator"]
    VAL22 = 22,
    #[doc = "23: I2S_SHARED_WS\\[0\\]
output from I2S pin sharing"]
    VAL23 = 23,
    #[doc = "24: I2S_SHARED_WS\\[1\\]
output from I2S pin sharing"]
    VAL24 = 24,
}
impl From<CAPTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTSEL_A) -> Self {
        variant as _
    }
}
impl CAPTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CAPTSEL_A> {
        match self.bits {
            0 => Some(CAPTSEL_A::VAL0),
            1 => Some(CAPTSEL_A::VAL1),
            2 => Some(CAPTSEL_A::VAL2),
            3 => Some(CAPTSEL_A::VAL3),
            4 => Some(CAPTSEL_A::VAL4),
            5 => Some(CAPTSEL_A::VAL5),
            6 => Some(CAPTSEL_A::VAL6),
            7 => Some(CAPTSEL_A::VAL7),
            8 => Some(CAPTSEL_A::VAL8),
            9 => Some(CAPTSEL_A::VAL9),
            10 => Some(CAPTSEL_A::VAL10),
            11 => Some(CAPTSEL_A::VAL11),
            12 => Some(CAPTSEL_A::VAL12),
            13 => Some(CAPTSEL_A::VAL13),
            14 => Some(CAPTSEL_A::VAL14),
            15 => Some(CAPTSEL_A::VAL15),
            16 => Some(CAPTSEL_A::VAL16),
            17 => Some(CAPTSEL_A::VAL17),
            18 => Some(CAPTSEL_A::VAL18),
            19 => Some(CAPTSEL_A::VAL19),
            20 => Some(CAPTSEL_A::VAL20),
            21 => Some(CAPTSEL_A::VAL21),
            22 => Some(CAPTSEL_A::VAL22),
            23 => Some(CAPTSEL_A::VAL23),
            24 => Some(CAPTSEL_A::VAL24),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == CAPTSEL_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == CAPTSEL_A::VAL1
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == CAPTSEL_A::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == CAPTSEL_A::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == CAPTSEL_A::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == CAPTSEL_A::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6`"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == CAPTSEL_A::VAL6
    }
    #[doc = "Checks if the value of the field is `VAL7`"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        *self == CAPTSEL_A::VAL7
    }
    #[doc = "Checks if the value of the field is `VAL8`"]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        *self == CAPTSEL_A::VAL8
    }
    #[doc = "Checks if the value of the field is `VAL9`"]
    #[inline(always)]
    pub fn is_val9(&self) -> bool {
        *self == CAPTSEL_A::VAL9
    }
    #[doc = "Checks if the value of the field is `VAL10`"]
    #[inline(always)]
    pub fn is_val10(&self) -> bool {
        *self == CAPTSEL_A::VAL10
    }
    #[doc = "Checks if the value of the field is `VAL11`"]
    #[inline(always)]
    pub fn is_val11(&self) -> bool {
        *self == CAPTSEL_A::VAL11
    }
    #[doc = "Checks if the value of the field is `VAL12`"]
    #[inline(always)]
    pub fn is_val12(&self) -> bool {
        *self == CAPTSEL_A::VAL12
    }
    #[doc = "Checks if the value of the field is `VAL13`"]
    #[inline(always)]
    pub fn is_val13(&self) -> bool {
        *self == CAPTSEL_A::VAL13
    }
    #[doc = "Checks if the value of the field is `VAL14`"]
    #[inline(always)]
    pub fn is_val14(&self) -> bool {
        *self == CAPTSEL_A::VAL14
    }
    #[doc = "Checks if the value of the field is `VAL15`"]
    #[inline(always)]
    pub fn is_val15(&self) -> bool {
        *self == CAPTSEL_A::VAL15
    }
    #[doc = "Checks if the value of the field is `VAL16`"]
    #[inline(always)]
    pub fn is_val16(&self) -> bool {
        *self == CAPTSEL_A::VAL16
    }
    #[doc = "Checks if the value of the field is `VAL17`"]
    #[inline(always)]
    pub fn is_val17(&self) -> bool {
        *self == CAPTSEL_A::VAL17
    }
    #[doc = "Checks if the value of the field is `VAL18`"]
    #[inline(always)]
    pub fn is_val18(&self) -> bool {
        *self == CAPTSEL_A::VAL18
    }
    #[doc = "Checks if the value of the field is `VAL19`"]
    #[inline(always)]
    pub fn is_val19(&self) -> bool {
        *self == CAPTSEL_A::VAL19
    }
    #[doc = "Checks if the value of the field is `VAL20`"]
    #[inline(always)]
    pub fn is_val20(&self) -> bool {
        *self == CAPTSEL_A::VAL20
    }
    #[doc = "Checks if the value of the field is `VAL21`"]
    #[inline(always)]
    pub fn is_val21(&self) -> bool {
        *self == CAPTSEL_A::VAL21
    }
    #[doc = "Checks if the value of the field is `VAL22`"]
    #[inline(always)]
    pub fn is_val22(&self) -> bool {
        *self == CAPTSEL_A::VAL22
    }
    #[doc = "Checks if the value of the field is `VAL23`"]
    #[inline(always)]
    pub fn is_val23(&self) -> bool {
        *self == CAPTSEL_A::VAL23
    }
    #[doc = "Checks if the value of the field is `VAL24`"]
    #[inline(always)]
    pub fn is_val24(&self) -> bool {
        *self == CAPTSEL_A::VAL24
    }
}
#[doc = "Field `CAPTSEL` writer - Input number to TIMER0 capture inputs 0 to 4"]
pub type CAPTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER0CAPTSEL_SPEC, u8, CAPTSEL_A, 5, O>;
impl<'a, const O: u8> CAPTSEL_W<'a, O> {
    #[doc = "CT_INP0 function selected from IOCON register"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL0)
    }
    #[doc = "CT_INP1 function selected from IOCON register"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL1)
    }
    #[doc = "CT_INP2 function selected from IOCON register"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL2)
    }
    #[doc = "CT_INP3 function selected from IOCON register"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL3)
    }
    #[doc = "CT_INP4 function selected from IOCON register"]
    #[inline(always)]
    pub fn val4(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL4)
    }
    #[doc = "CT_INP5 function selected from IOCON register"]
    #[inline(always)]
    pub fn val5(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL5)
    }
    #[doc = "CT_INP6 function selected from IOCON register"]
    #[inline(always)]
    pub fn val6(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL6)
    }
    #[doc = "CT_INP7 function selected from IOCON register"]
    #[inline(always)]
    pub fn val7(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL7)
    }
    #[doc = "CT_INP8 function selected from IOCON register"]
    #[inline(always)]
    pub fn val8(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL8)
    }
    #[doc = "CT_INP9 function selected from IOCON register"]
    #[inline(always)]
    pub fn val9(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL9)
    }
    #[doc = "CT_INP10 function selected from IOCON register"]
    #[inline(always)]
    pub fn val10(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL10)
    }
    #[doc = "CT_INP11 function selected from IOCON register"]
    #[inline(always)]
    pub fn val11(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL11)
    }
    #[doc = "CT_INP12 function selected from IOCON register"]
    #[inline(always)]
    pub fn val12(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL12)
    }
    #[doc = "CT_INP13 function selected from IOCON register"]
    #[inline(always)]
    pub fn val13(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL13)
    }
    #[doc = "CT_INP14 function selected from IOCON register"]
    #[inline(always)]
    pub fn val14(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL14)
    }
    #[doc = "CT_INP15 function selected from IOCON register"]
    #[inline(always)]
    pub fn val15(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL15)
    }
    #[doc = "CT_INP16 function selected from IOCON register"]
    #[inline(always)]
    pub fn val16(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL16)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val17(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL17)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val18(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL18)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val19(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL19)
    }
    #[doc = "USB0_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn val20(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL20)
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn val21(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL21)
    }
    #[doc = "COMP_OUTPUT output from analog comparator"]
    #[inline(always)]
    pub fn val22(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL22)
    }
    #[doc = "I2S_SHARED_WS\\[0\\]
output from I2S pin sharing"]
    #[inline(always)]
    pub fn val23(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL23)
    }
    #[doc = "I2S_SHARED_WS\\[1\\]
output from I2S pin sharing"]
    #[inline(always)]
    pub fn val24(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL24)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input number to TIMER0 capture inputs 0 to 4"]
    #[inline(always)]
    pub fn captsel(&self) -> CAPTSEL_R {
        CAPTSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input number to TIMER0 capture inputs 0 to 4"]
    #[inline(always)]
    #[must_use]
    pub fn captsel(&mut self) -> CAPTSEL_W<0> {
        CAPTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture select registers for TIMER0 inputs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0captsel](index.html) module"]
pub struct TIMER0CAPTSEL_SPEC;
impl crate::RegisterSpec for TIMER0CAPTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0captsel::R](R) reader structure"]
impl crate::Readable for TIMER0CAPTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0captsel::W](W) writer structure"]
impl crate::Writable for TIMER0CAPTSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER0CAPTSEL[%s]
to value 0x1f"]
impl crate::Resettable for TIMER0CAPTSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
