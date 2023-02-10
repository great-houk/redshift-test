#[doc = "Register `SCT0_INMUX[%s]` reader"]
pub struct R(crate::R<SCT0_INMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCT0_INMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCT0_INMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCT0_INMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCT0_INMUX[%s]` writer"]
pub struct W(crate::W<SCT0_INMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCT0_INMUX_SPEC>;
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
impl From<crate::W<SCT0_INMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCT0_INMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INP_N` reader - Input number to SCT0 inputs 0 to 6.."]
pub type INP_N_R = crate::FieldReader<u8, INP_N_A>;
#[doc = "Input number to SCT0 inputs 0 to 6..\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INP_N_A {
    #[doc = "0: SCT_GPI0 function selected from IOCON register"]
    VAL0 = 0,
    #[doc = "1: SCT_GPI1 function selected from IOCON register"]
    VAL1 = 1,
    #[doc = "2: SCT_GPI2 function selected from IOCON register"]
    VAL2 = 2,
    #[doc = "3: SCT_GPI3 function selected from IOCON register"]
    VAL3 = 3,
    #[doc = "4: SCT_GPI4 function selected from IOCON register"]
    VAL4 = 4,
    #[doc = "5: SCT_GPI5 function selected from IOCON register"]
    VAL5 = 5,
    #[doc = "6: SCT_GPI6 function selected from IOCON register"]
    VAL6 = 6,
    #[doc = "7: SCT_GPI7 function selected from IOCON register"]
    VAL7 = 7,
    #[doc = "8: T0_OUT0 ctimer 0 match\\[0\\]
output"]
    VAL8 = 8,
    #[doc = "9: T1_OUT0 ctimer 1 match\\[0\\]
output"]
    VAL9 = 9,
    #[doc = "10: T2_OUT0 ctimer 2 match\\[0\\]
output"]
    VAL10 = 10,
    #[doc = "11: T3_OUT0 ctimer 3 match\\[0\\]
output"]
    VAL11 = 11,
    #[doc = "12: T4_OUT0 ctimer 4 match\\[0\\]
output"]
    VAL12 = 12,
    #[doc = "13: ADC_IRQ interrupt request from ADC"]
    VAL13 = 13,
    #[doc = "14: GPIOINT_BMATCH"]
    VAL14 = 14,
    #[doc = "15: USB0_FRAME_TOGGLE"]
    VAL15 = 15,
    #[doc = "16: USB1_FRAME_TOGGLE"]
    VAL16 = 16,
    #[doc = "17: COMP_OUTPUT output from analog comparator"]
    VAL17 = 17,
    #[doc = "18: I2S_SHARED_SCK\\[0\\]
output from I2S pin sharing"]
    VAL18 = 18,
    #[doc = "19: I2S_SHARED_SCK\\[1\\]
output from I2S pin sharing"]
    VAL19 = 19,
    #[doc = "20: I2S_SHARED_WS\\[0\\]
output from I2S pin sharing"]
    VAL20 = 20,
    #[doc = "21: I2S_SHARED_WS\\[1\\]
output from I2S pin sharing"]
    VAL21 = 21,
    #[doc = "22: ARM_TXEV interrupt event from cpu0 or cpu1"]
    VAL22 = 22,
    #[doc = "23: DEBUG_HALTED from cpu0 or cpu1"]
    VAL23 = 23,
}
impl From<INP_N_A> for u8 {
    #[inline(always)]
    fn from(variant: INP_N_A) -> Self {
        variant as _
    }
}
impl INP_N_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INP_N_A> {
        match self.bits {
            0 => Some(INP_N_A::VAL0),
            1 => Some(INP_N_A::VAL1),
            2 => Some(INP_N_A::VAL2),
            3 => Some(INP_N_A::VAL3),
            4 => Some(INP_N_A::VAL4),
            5 => Some(INP_N_A::VAL5),
            6 => Some(INP_N_A::VAL6),
            7 => Some(INP_N_A::VAL7),
            8 => Some(INP_N_A::VAL8),
            9 => Some(INP_N_A::VAL9),
            10 => Some(INP_N_A::VAL10),
            11 => Some(INP_N_A::VAL11),
            12 => Some(INP_N_A::VAL12),
            13 => Some(INP_N_A::VAL13),
            14 => Some(INP_N_A::VAL14),
            15 => Some(INP_N_A::VAL15),
            16 => Some(INP_N_A::VAL16),
            17 => Some(INP_N_A::VAL17),
            18 => Some(INP_N_A::VAL18),
            19 => Some(INP_N_A::VAL19),
            20 => Some(INP_N_A::VAL20),
            21 => Some(INP_N_A::VAL21),
            22 => Some(INP_N_A::VAL22),
            23 => Some(INP_N_A::VAL23),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        *self == INP_N_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        *self == INP_N_A::VAL1
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        *self == INP_N_A::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        *self == INP_N_A::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        *self == INP_N_A::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        *self == INP_N_A::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6`"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        *self == INP_N_A::VAL6
    }
    #[doc = "Checks if the value of the field is `VAL7`"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        *self == INP_N_A::VAL7
    }
    #[doc = "Checks if the value of the field is `VAL8`"]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        *self == INP_N_A::VAL8
    }
    #[doc = "Checks if the value of the field is `VAL9`"]
    #[inline(always)]
    pub fn is_val9(&self) -> bool {
        *self == INP_N_A::VAL9
    }
    #[doc = "Checks if the value of the field is `VAL10`"]
    #[inline(always)]
    pub fn is_val10(&self) -> bool {
        *self == INP_N_A::VAL10
    }
    #[doc = "Checks if the value of the field is `VAL11`"]
    #[inline(always)]
    pub fn is_val11(&self) -> bool {
        *self == INP_N_A::VAL11
    }
    #[doc = "Checks if the value of the field is `VAL12`"]
    #[inline(always)]
    pub fn is_val12(&self) -> bool {
        *self == INP_N_A::VAL12
    }
    #[doc = "Checks if the value of the field is `VAL13`"]
    #[inline(always)]
    pub fn is_val13(&self) -> bool {
        *self == INP_N_A::VAL13
    }
    #[doc = "Checks if the value of the field is `VAL14`"]
    #[inline(always)]
    pub fn is_val14(&self) -> bool {
        *self == INP_N_A::VAL14
    }
    #[doc = "Checks if the value of the field is `VAL15`"]
    #[inline(always)]
    pub fn is_val15(&self) -> bool {
        *self == INP_N_A::VAL15
    }
    #[doc = "Checks if the value of the field is `VAL16`"]
    #[inline(always)]
    pub fn is_val16(&self) -> bool {
        *self == INP_N_A::VAL16
    }
    #[doc = "Checks if the value of the field is `VAL17`"]
    #[inline(always)]
    pub fn is_val17(&self) -> bool {
        *self == INP_N_A::VAL17
    }
    #[doc = "Checks if the value of the field is `VAL18`"]
    #[inline(always)]
    pub fn is_val18(&self) -> bool {
        *self == INP_N_A::VAL18
    }
    #[doc = "Checks if the value of the field is `VAL19`"]
    #[inline(always)]
    pub fn is_val19(&self) -> bool {
        *self == INP_N_A::VAL19
    }
    #[doc = "Checks if the value of the field is `VAL20`"]
    #[inline(always)]
    pub fn is_val20(&self) -> bool {
        *self == INP_N_A::VAL20
    }
    #[doc = "Checks if the value of the field is `VAL21`"]
    #[inline(always)]
    pub fn is_val21(&self) -> bool {
        *self == INP_N_A::VAL21
    }
    #[doc = "Checks if the value of the field is `VAL22`"]
    #[inline(always)]
    pub fn is_val22(&self) -> bool {
        *self == INP_N_A::VAL22
    }
    #[doc = "Checks if the value of the field is `VAL23`"]
    #[inline(always)]
    pub fn is_val23(&self) -> bool {
        *self == INP_N_A::VAL23
    }
}
#[doc = "Field `INP_N` writer - Input number to SCT0 inputs 0 to 6.."]
pub type INP_N_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCT0_INMUX_SPEC, u8, INP_N_A, 5, O>;
impl<'a, const O: u8> INP_N_W<'a, O> {
    #[doc = "SCT_GPI0 function selected from IOCON register"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(INP_N_A::VAL0)
    }
    #[doc = "SCT_GPI1 function selected from IOCON register"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(INP_N_A::VAL1)
    }
    #[doc = "SCT_GPI2 function selected from IOCON register"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut W {
        self.variant(INP_N_A::VAL2)
    }
    #[doc = "SCT_GPI3 function selected from IOCON register"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut W {
        self.variant(INP_N_A::VAL3)
    }
    #[doc = "SCT_GPI4 function selected from IOCON register"]
    #[inline(always)]
    pub fn val4(self) -> &'a mut W {
        self.variant(INP_N_A::VAL4)
    }
    #[doc = "SCT_GPI5 function selected from IOCON register"]
    #[inline(always)]
    pub fn val5(self) -> &'a mut W {
        self.variant(INP_N_A::VAL5)
    }
    #[doc = "SCT_GPI6 function selected from IOCON register"]
    #[inline(always)]
    pub fn val6(self) -> &'a mut W {
        self.variant(INP_N_A::VAL6)
    }
    #[doc = "SCT_GPI7 function selected from IOCON register"]
    #[inline(always)]
    pub fn val7(self) -> &'a mut W {
        self.variant(INP_N_A::VAL7)
    }
    #[doc = "T0_OUT0 ctimer 0 match\\[0\\]
output"]
    #[inline(always)]
    pub fn val8(self) -> &'a mut W {
        self.variant(INP_N_A::VAL8)
    }
    #[doc = "T1_OUT0 ctimer 1 match\\[0\\]
output"]
    #[inline(always)]
    pub fn val9(self) -> &'a mut W {
        self.variant(INP_N_A::VAL9)
    }
    #[doc = "T2_OUT0 ctimer 2 match\\[0\\]
output"]
    #[inline(always)]
    pub fn val10(self) -> &'a mut W {
        self.variant(INP_N_A::VAL10)
    }
    #[doc = "T3_OUT0 ctimer 3 match\\[0\\]
output"]
    #[inline(always)]
    pub fn val11(self) -> &'a mut W {
        self.variant(INP_N_A::VAL11)
    }
    #[doc = "T4_OUT0 ctimer 4 match\\[0\\]
output"]
    #[inline(always)]
    pub fn val12(self) -> &'a mut W {
        self.variant(INP_N_A::VAL12)
    }
    #[doc = "ADC_IRQ interrupt request from ADC"]
    #[inline(always)]
    pub fn val13(self) -> &'a mut W {
        self.variant(INP_N_A::VAL13)
    }
    #[doc = "GPIOINT_BMATCH"]
    #[inline(always)]
    pub fn val14(self) -> &'a mut W {
        self.variant(INP_N_A::VAL14)
    }
    #[doc = "USB0_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn val15(self) -> &'a mut W {
        self.variant(INP_N_A::VAL15)
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn val16(self) -> &'a mut W {
        self.variant(INP_N_A::VAL16)
    }
    #[doc = "COMP_OUTPUT output from analog comparator"]
    #[inline(always)]
    pub fn val17(self) -> &'a mut W {
        self.variant(INP_N_A::VAL17)
    }
    #[doc = "I2S_SHARED_SCK\\[0\\]
output from I2S pin sharing"]
    #[inline(always)]
    pub fn val18(self) -> &'a mut W {
        self.variant(INP_N_A::VAL18)
    }
    #[doc = "I2S_SHARED_SCK\\[1\\]
output from I2S pin sharing"]
    #[inline(always)]
    pub fn val19(self) -> &'a mut W {
        self.variant(INP_N_A::VAL19)
    }
    #[doc = "I2S_SHARED_WS\\[0\\]
output from I2S pin sharing"]
    #[inline(always)]
    pub fn val20(self) -> &'a mut W {
        self.variant(INP_N_A::VAL20)
    }
    #[doc = "I2S_SHARED_WS\\[1\\]
output from I2S pin sharing"]
    #[inline(always)]
    pub fn val21(self) -> &'a mut W {
        self.variant(INP_N_A::VAL21)
    }
    #[doc = "ARM_TXEV interrupt event from cpu0 or cpu1"]
    #[inline(always)]
    pub fn val22(self) -> &'a mut W {
        self.variant(INP_N_A::VAL22)
    }
    #[doc = "DEBUG_HALTED from cpu0 or cpu1"]
    #[inline(always)]
    pub fn val23(self) -> &'a mut W {
        self.variant(INP_N_A::VAL23)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6.."]
    #[inline(always)]
    pub fn inp_n(&self) -> INP_N_R {
        INP_N_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input number to SCT0 inputs 0 to 6.."]
    #[inline(always)]
    #[must_use]
    pub fn inp_n(&mut self) -> INP_N_W<0> {
        INP_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input mux register for SCT0 input\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sct0_inmux](index.html) module"]
pub struct SCT0_INMUX_SPEC;
impl crate::RegisterSpec for SCT0_INMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sct0_inmux::R](R) reader structure"]
impl crate::Readable for SCT0_INMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sct0_inmux::W](W) writer structure"]
impl crate::Writable for SCT0_INMUX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCT0_INMUX[%s]
to value 0x1f"]
impl crate::Resettable for SCT0_INMUX_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
