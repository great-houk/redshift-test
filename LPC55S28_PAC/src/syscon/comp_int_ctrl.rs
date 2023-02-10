#[doc = "Register `COMP_INT_CTRL` reader"]
pub struct R(crate::R<COMP_INT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_INT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_INT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_INT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP_INT_CTRL` writer"]
pub struct W(crate::W<COMP_INT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_INT_CTRL_SPEC>;
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
impl From<crate::W<COMP_INT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_INT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_ENABLE` reader - Analog Comparator interrupt enable control:."]
pub type INT_ENABLE_R = crate::BitReader<INT_ENABLE_A>;
#[doc = "Analog Comparator interrupt enable control:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_ENABLE_A {
    #[doc = "0: interrupt disable."]
    INT_DISABLE = 0,
    #[doc = "1: interrupt enable."]
    INT_ENABLE = 1,
}
impl From<INT_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: INT_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_ENABLE_A {
        match self.bits {
            false => INT_ENABLE_A::INT_DISABLE,
            true => INT_ENABLE_A::INT_ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `INT_DISABLE`"]
    #[inline(always)]
    pub fn is_int_disable(&self) -> bool {
        *self == INT_ENABLE_A::INT_DISABLE
    }
    #[doc = "Checks if the value of the field is `INT_ENABLE`"]
    #[inline(always)]
    pub fn is_int_enable(&self) -> bool {
        *self == INT_ENABLE_A::INT_ENABLE
    }
}
#[doc = "Field `INT_ENABLE` writer - Analog Comparator interrupt enable control:."]
pub type INT_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMP_INT_CTRL_SPEC, INT_ENABLE_A, O>;
impl<'a, const O: u8> INT_ENABLE_W<'a, O> {
    #[doc = "interrupt disable."]
    #[inline(always)]
    pub fn int_disable(self) -> &'a mut W {
        self.variant(INT_ENABLE_A::INT_DISABLE)
    }
    #[doc = "interrupt enable."]
    #[inline(always)]
    pub fn int_enable(self) -> &'a mut W {
        self.variant(INT_ENABLE_A::INT_ENABLE)
    }
}
#[doc = "Field `INT_CLEAR` reader - Analog Comparator interrupt clear."]
pub type INT_CLEAR_R = crate::BitReader<INT_CLEAR_A>;
#[doc = "Analog Comparator interrupt clear.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_CLEAR_A {
    #[doc = "0: No effect."]
    NONE = 0,
    #[doc = "1: Clear the interrupt. Self-cleared bit."]
    CLEAR = 1,
}
impl From<INT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: INT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_CLEAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_CLEAR_A {
        match self.bits {
            false => INT_CLEAR_A::NONE,
            true => INT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == INT_CLEAR_A::NONE
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == INT_CLEAR_A::CLEAR
    }
}
#[doc = "Field `INT_CLEAR` writer - Analog Comparator interrupt clear."]
pub type INT_CLEAR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMP_INT_CTRL_SPEC, INT_CLEAR_A, O>;
impl<'a, const O: u8> INT_CLEAR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(INT_CLEAR_A::NONE)
    }
    #[doc = "Clear the interrupt. Self-cleared bit."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(INT_CLEAR_A::CLEAR)
    }
}
#[doc = "Field `INT_CTRL` reader - Comparator interrupt type selector:."]
pub type INT_CTRL_R = crate::FieldReader<u8, INT_CTRL_A>;
#[doc = "Comparator interrupt type selector:.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INT_CTRL_A {
    #[doc = "0: The analog comparator interrupt edge sensitive is disabled."]
    EDGE_DISABLE = 0,
    #[doc = "1: The analog comparator interrupt level sensitive is disabled."]
    LVL_DISABLE = 1,
    #[doc = "2: analog comparator interrupt is rising edge sensitive."]
    EDGE_RISING = 2,
    #[doc = "3: Analog Comparator interrupt is high level sensitive."]
    LVL_HIGH = 3,
    #[doc = "4: analog comparator interrupt is falling edge sensitive."]
    EDGE_FALLING = 4,
    #[doc = "5: Analog Comparator interrupt is low level sensitive."]
    LVL_LOW = 5,
    #[doc = "6: analog comparator interrupt is rising and falling edge sensitive."]
    EDGE_BOTH = 6,
    #[doc = "7: The analog comparator interrupt level sensitive is disabled."]
    LVL_DIS2 = 7,
}
impl From<INT_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: INT_CTRL_A) -> Self {
        variant as _
    }
}
impl INT_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_CTRL_A {
        match self.bits {
            0 => INT_CTRL_A::EDGE_DISABLE,
            1 => INT_CTRL_A::LVL_DISABLE,
            2 => INT_CTRL_A::EDGE_RISING,
            3 => INT_CTRL_A::LVL_HIGH,
            4 => INT_CTRL_A::EDGE_FALLING,
            5 => INT_CTRL_A::LVL_LOW,
            6 => INT_CTRL_A::EDGE_BOTH,
            7 => INT_CTRL_A::LVL_DIS2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE_DISABLE`"]
    #[inline(always)]
    pub fn is_edge_disable(&self) -> bool {
        *self == INT_CTRL_A::EDGE_DISABLE
    }
    #[doc = "Checks if the value of the field is `LVL_DISABLE`"]
    #[inline(always)]
    pub fn is_lvl_disable(&self) -> bool {
        *self == INT_CTRL_A::LVL_DISABLE
    }
    #[doc = "Checks if the value of the field is `EDGE_RISING`"]
    #[inline(always)]
    pub fn is_edge_rising(&self) -> bool {
        *self == INT_CTRL_A::EDGE_RISING
    }
    #[doc = "Checks if the value of the field is `LVL_HIGH`"]
    #[inline(always)]
    pub fn is_lvl_high(&self) -> bool {
        *self == INT_CTRL_A::LVL_HIGH
    }
    #[doc = "Checks if the value of the field is `EDGE_FALLING`"]
    #[inline(always)]
    pub fn is_edge_falling(&self) -> bool {
        *self == INT_CTRL_A::EDGE_FALLING
    }
    #[doc = "Checks if the value of the field is `LVL_LOW`"]
    #[inline(always)]
    pub fn is_lvl_low(&self) -> bool {
        *self == INT_CTRL_A::LVL_LOW
    }
    #[doc = "Checks if the value of the field is `EDGE_BOTH`"]
    #[inline(always)]
    pub fn is_edge_both(&self) -> bool {
        *self == INT_CTRL_A::EDGE_BOTH
    }
    #[doc = "Checks if the value of the field is `LVL_DIS2`"]
    #[inline(always)]
    pub fn is_lvl_dis2(&self) -> bool {
        *self == INT_CTRL_A::LVL_DIS2
    }
}
#[doc = "Field `INT_CTRL` writer - Comparator interrupt type selector:."]
pub type INT_CTRL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, COMP_INT_CTRL_SPEC, u8, INT_CTRL_A, 3, O>;
impl<'a, const O: u8> INT_CTRL_W<'a, O> {
    #[doc = "The analog comparator interrupt edge sensitive is disabled."]
    #[inline(always)]
    pub fn edge_disable(self) -> &'a mut W {
        self.variant(INT_CTRL_A::EDGE_DISABLE)
    }
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    #[inline(always)]
    pub fn lvl_disable(self) -> &'a mut W {
        self.variant(INT_CTRL_A::LVL_DISABLE)
    }
    #[doc = "analog comparator interrupt is rising edge sensitive."]
    #[inline(always)]
    pub fn edge_rising(self) -> &'a mut W {
        self.variant(INT_CTRL_A::EDGE_RISING)
    }
    #[doc = "Analog Comparator interrupt is high level sensitive."]
    #[inline(always)]
    pub fn lvl_high(self) -> &'a mut W {
        self.variant(INT_CTRL_A::LVL_HIGH)
    }
    #[doc = "analog comparator interrupt is falling edge sensitive."]
    #[inline(always)]
    pub fn edge_falling(self) -> &'a mut W {
        self.variant(INT_CTRL_A::EDGE_FALLING)
    }
    #[doc = "Analog Comparator interrupt is low level sensitive."]
    #[inline(always)]
    pub fn lvl_low(self) -> &'a mut W {
        self.variant(INT_CTRL_A::LVL_LOW)
    }
    #[doc = "analog comparator interrupt is rising and falling edge sensitive."]
    #[inline(always)]
    pub fn edge_both(self) -> &'a mut W {
        self.variant(INT_CTRL_A::EDGE_BOTH)
    }
    #[doc = "The analog comparator interrupt level sensitive is disabled."]
    #[inline(always)]
    pub fn lvl_dis2(self) -> &'a mut W {
        self.variant(INT_CTRL_A::LVL_DIS2)
    }
}
#[doc = "Field `INT_SOURCE` reader - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
pub type INT_SOURCE_R = crate::BitReader<INT_SOURCE_A>;
#[doc = "Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_SOURCE_A {
    #[doc = "0: Select Analog Comparator filtered output as input for interrupt detection."]
    FILTER_INT = 0,
    #[doc = "1: Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    RAW_INT = 1,
}
impl From<INT_SOURCE_A> for bool {
    #[inline(always)]
    fn from(variant: INT_SOURCE_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_SOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_SOURCE_A {
        match self.bits {
            false => INT_SOURCE_A::FILTER_INT,
            true => INT_SOURCE_A::RAW_INT,
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_INT`"]
    #[inline(always)]
    pub fn is_filter_int(&self) -> bool {
        *self == INT_SOURCE_A::FILTER_INT
    }
    #[doc = "Checks if the value of the field is `RAW_INT`"]
    #[inline(always)]
    pub fn is_raw_int(&self) -> bool {
        *self == INT_SOURCE_A::RAW_INT
    }
}
#[doc = "Field `INT_SOURCE` writer - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
pub type INT_SOURCE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMP_INT_CTRL_SPEC, INT_SOURCE_A, O>;
impl<'a, const O: u8> INT_SOURCE_W<'a, O> {
    #[doc = "Select Analog Comparator filtered output as input for interrupt detection."]
    #[inline(always)]
    pub fn filter_int(self) -> &'a mut W {
        self.variant(INT_SOURCE_A::FILTER_INT)
    }
    #[doc = "Select Analog Comparator raw output (unfiltered) as input for interrupt detection. Must be used when Analog comparator is used as wake up source in Power down mode."]
    #[inline(always)]
    pub fn raw_int(self) -> &'a mut W {
        self.variant(INT_SOURCE_A::RAW_INT)
    }
}
impl R {
    #[doc = "Bit 0 - Analog Comparator interrupt enable control:."]
    #[inline(always)]
    pub fn int_enable(&self) -> INT_ENABLE_R {
        INT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator interrupt clear."]
    #[inline(always)]
    pub fn int_clear(&self) -> INT_CLEAR_R {
        INT_CLEAR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Comparator interrupt type selector:."]
    #[inline(always)]
    pub fn int_ctrl(&self) -> INT_CTRL_R {
        INT_CTRL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[inline(always)]
    pub fn int_source(&self) -> INT_SOURCE_R {
        INT_SOURCE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Analog Comparator interrupt enable control:."]
    #[inline(always)]
    #[must_use]
    pub fn int_enable(&mut self) -> INT_ENABLE_W<0> {
        INT_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Analog Comparator interrupt clear."]
    #[inline(always)]
    #[must_use]
    pub fn int_clear(&mut self) -> INT_CLEAR_W<1> {
        INT_CLEAR_W::new(self)
    }
    #[doc = "Bits 2:4 - Comparator interrupt type selector:."]
    #[inline(always)]
    #[must_use]
    pub fn int_ctrl(&mut self) -> INT_CTRL_W<2> {
        INT_CTRL_W::new(self)
    }
    #[doc = "Bit 5 - Select which Analog comparator output (filtered our un-filtered) is used for interrupt detection."]
    #[inline(always)]
    #[must_use]
    pub fn int_source(&mut self) -> INT_SOURCE_W<5> {
        INT_SOURCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Interrupt control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_int_ctrl](index.html) module"]
pub struct COMP_INT_CTRL_SPEC;
impl crate::RegisterSpec for COMP_INT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_int_ctrl::R](R) reader structure"]
impl crate::Readable for COMP_INT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp_int_ctrl::W](W) writer structure"]
impl crate::Writable for COMP_INT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP_INT_CTRL to value 0"]
impl crate::Resettable for COMP_INT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
