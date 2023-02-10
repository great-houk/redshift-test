#[doc = "Register `PMCFG` reader"]
pub struct R(crate::R<PMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMCFG` writer"]
pub struct W(crate::W<PMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCFG_SPEC>;
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
impl From<crate::W<PMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROD_ENDPTS0` reader - Determines whether slice 0 is an endpoint."]
pub type PROD_ENDPTS0_R = crate::BitReader<PROD_ENDPTS0_A>;
#[doc = "Determines whether slice 0 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROD_ENDPTS0_A {
    #[doc = "0: No effect. Slice 0 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS0_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS0_A) -> Self {
        variant as u8 != 0
    }
}
impl PROD_ENDPTS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS0_A {
        match self.bits {
            false => PROD_ENDPTS0_A::NO_EFFECT,
            true => PROD_ENDPTS0_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS0_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS0_A::ENDPOINT
    }
}
#[doc = "Field `PROD_ENDPTS0` writer - Determines whether slice 0 is an endpoint."]
pub type PROD_ENDPTS0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCFG_SPEC, PROD_ENDPTS0_A, O>;
impl<'a, const O: u8> PROD_ENDPTS0_W<'a, O> {
    #[doc = "No effect. Slice 0 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS0_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 0 is the endpoint of a product term (minterm). Pin interrupt 0 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS0_A::ENDPOINT)
    }
}
#[doc = "Field `PROD_ENDPTS1` reader - Determines whether slice 1 is an endpoint."]
pub type PROD_ENDPTS1_R = crate::BitReader<PROD_ENDPTS1_A>;
#[doc = "Determines whether slice 1 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROD_ENDPTS1_A {
    #[doc = "0: No effect. Slice 1 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS1_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS1_A) -> Self {
        variant as u8 != 0
    }
}
impl PROD_ENDPTS1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS1_A {
        match self.bits {
            false => PROD_ENDPTS1_A::NO_EFFECT,
            true => PROD_ENDPTS1_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS1_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS1_A::ENDPOINT
    }
}
#[doc = "Field `PROD_ENDPTS1` writer - Determines whether slice 1 is an endpoint."]
pub type PROD_ENDPTS1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCFG_SPEC, PROD_ENDPTS1_A, O>;
impl<'a, const O: u8> PROD_ENDPTS1_W<'a, O> {
    #[doc = "No effect. Slice 1 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS1_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 1 is the endpoint of a product term (minterm). Pin interrupt 1 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS1_A::ENDPOINT)
    }
}
#[doc = "Field `PROD_ENDPTS2` reader - Determines whether slice 2 is an endpoint."]
pub type PROD_ENDPTS2_R = crate::BitReader<PROD_ENDPTS2_A>;
#[doc = "Determines whether slice 2 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROD_ENDPTS2_A {
    #[doc = "0: No effect. Slice 2 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS2_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS2_A) -> Self {
        variant as u8 != 0
    }
}
impl PROD_ENDPTS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS2_A {
        match self.bits {
            false => PROD_ENDPTS2_A::NO_EFFECT,
            true => PROD_ENDPTS2_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS2_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS2_A::ENDPOINT
    }
}
#[doc = "Field `PROD_ENDPTS2` writer - Determines whether slice 2 is an endpoint."]
pub type PROD_ENDPTS2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCFG_SPEC, PROD_ENDPTS2_A, O>;
impl<'a, const O: u8> PROD_ENDPTS2_W<'a, O> {
    #[doc = "No effect. Slice 2 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS2_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 2 is the endpoint of a product term (minterm). Pin interrupt 2 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS2_A::ENDPOINT)
    }
}
#[doc = "Field `PROD_ENDPTS3` reader - Determines whether slice 3 is an endpoint."]
pub type PROD_ENDPTS3_R = crate::BitReader<PROD_ENDPTS3_A>;
#[doc = "Determines whether slice 3 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROD_ENDPTS3_A {
    #[doc = "0: No effect. Slice 3 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS3_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS3_A) -> Self {
        variant as u8 != 0
    }
}
impl PROD_ENDPTS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS3_A {
        match self.bits {
            false => PROD_ENDPTS3_A::NO_EFFECT,
            true => PROD_ENDPTS3_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS3_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS3_A::ENDPOINT
    }
}
#[doc = "Field `PROD_ENDPTS3` writer - Determines whether slice 3 is an endpoint."]
pub type PROD_ENDPTS3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCFG_SPEC, PROD_ENDPTS3_A, O>;
impl<'a, const O: u8> PROD_ENDPTS3_W<'a, O> {
    #[doc = "No effect. Slice 3 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS3_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 3 is the endpoint of a product term (minterm). Pin interrupt 3 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS3_A::ENDPOINT)
    }
}
#[doc = "Field `PROD_ENDPTS4` reader - Determines whether slice 4 is an endpoint."]
pub type PROD_ENDPTS4_R = crate::BitReader<PROD_ENDPTS4_A>;
#[doc = "Determines whether slice 4 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROD_ENDPTS4_A {
    #[doc = "0: No effect. Slice 4 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS4_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS4_A) -> Self {
        variant as u8 != 0
    }
}
impl PROD_ENDPTS4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS4_A {
        match self.bits {
            false => PROD_ENDPTS4_A::NO_EFFECT,
            true => PROD_ENDPTS4_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS4_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS4_A::ENDPOINT
    }
}
#[doc = "Field `PROD_ENDPTS4` writer - Determines whether slice 4 is an endpoint."]
pub type PROD_ENDPTS4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCFG_SPEC, PROD_ENDPTS4_A, O>;
impl<'a, const O: u8> PROD_ENDPTS4_W<'a, O> {
    #[doc = "No effect. Slice 4 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS4_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 4 is the endpoint of a product term (minterm). Pin interrupt 4 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS4_A::ENDPOINT)
    }
}
#[doc = "Field `PROD_ENDPTS5` reader - Determines whether slice 5 is an endpoint."]
pub type PROD_ENDPTS5_R = crate::BitReader<PROD_ENDPTS5_A>;
#[doc = "Determines whether slice 5 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROD_ENDPTS5_A {
    #[doc = "0: No effect. Slice 5 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS5_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS5_A) -> Self {
        variant as u8 != 0
    }
}
impl PROD_ENDPTS5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS5_A {
        match self.bits {
            false => PROD_ENDPTS5_A::NO_EFFECT,
            true => PROD_ENDPTS5_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS5_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS5_A::ENDPOINT
    }
}
#[doc = "Field `PROD_ENDPTS5` writer - Determines whether slice 5 is an endpoint."]
pub type PROD_ENDPTS5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCFG_SPEC, PROD_ENDPTS5_A, O>;
impl<'a, const O: u8> PROD_ENDPTS5_W<'a, O> {
    #[doc = "No effect. Slice 5 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS5_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 5 is the endpoint of a product term (minterm). Pin interrupt 5 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS5_A::ENDPOINT)
    }
}
#[doc = "Field `PROD_ENDPTS6` reader - Determines whether slice 6 is an endpoint."]
pub type PROD_ENDPTS6_R = crate::BitReader<PROD_ENDPTS6_A>;
#[doc = "Determines whether slice 6 is an endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PROD_ENDPTS6_A {
    #[doc = "0: No effect. Slice 6 is not an endpoint."]
    NO_EFFECT = 0,
    #[doc = "1: endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    ENDPOINT = 1,
}
impl From<PROD_ENDPTS6_A> for bool {
    #[inline(always)]
    fn from(variant: PROD_ENDPTS6_A) -> Self {
        variant as u8 != 0
    }
}
impl PROD_ENDPTS6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PROD_ENDPTS6_A {
        match self.bits {
            false => PROD_ENDPTS6_A::NO_EFFECT,
            true => PROD_ENDPTS6_A::ENDPOINT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PROD_ENDPTS6_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ENDPOINT`"]
    #[inline(always)]
    pub fn is_endpoint(&self) -> bool {
        *self == PROD_ENDPTS6_A::ENDPOINT
    }
}
#[doc = "Field `PROD_ENDPTS6` writer - Determines whether slice 6 is an endpoint."]
pub type PROD_ENDPTS6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCFG_SPEC, PROD_ENDPTS6_A, O>;
impl<'a, const O: u8> PROD_ENDPTS6_W<'a, O> {
    #[doc = "No effect. Slice 6 is not an endpoint."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PROD_ENDPTS6_A::NO_EFFECT)
    }
    #[doc = "endpoint. Slice 6 is the endpoint of a product term (minterm). Pin interrupt 6 in the NVIC is raised if the minterm evaluates as true."]
    #[inline(always)]
    pub fn endpoint(self) -> &'a mut W {
        self.variant(PROD_ENDPTS6_A::ENDPOINT)
    }
}
#[doc = "Field `CFG0` reader - Specifies the match contribution condition for bit slice 0."]
pub type CFG0_R = crate::FieldReader<u8, CFG0_A>;
#[doc = "Specifies the match contribution condition for bit slice 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFG0_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG0_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG0_A) -> Self {
        variant as _
    }
}
impl CFG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG0_A {
        match self.bits {
            0 => CFG0_A::CONSTANT_HIGH,
            1 => CFG0_A::STICKY_RISING_EDGE,
            2 => CFG0_A::STICKY_FALLING_EDGE,
            3 => CFG0_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG0_A::HIGH_LEVEL,
            5 => CFG0_A::LOW_LEVEL,
            6 => CFG0_A::CONSTANT_ZERO,
            7 => CFG0_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG0_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG0_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG0_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG0_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG0_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG0_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG0_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG0_A::EVENT
    }
}
#[doc = "Field `CFG0` writer - Specifies the match contribution condition for bit slice 0."]
pub type CFG0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMCFG_SPEC, u8, CFG0_A, 3, O>;
impl<'a, const O: u8> CFG0_W<'a, O> {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG0_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG0_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG0_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG0_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG0_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG0_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG0_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG0_A::EVENT)
    }
}
#[doc = "Field `CFG1` reader - Specifies the match contribution condition for bit slice 1."]
pub type CFG1_R = crate::FieldReader<u8, CFG1_A>;
#[doc = "Specifies the match contribution condition for bit slice 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFG1_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG1_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG1_A) -> Self {
        variant as _
    }
}
impl CFG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG1_A {
        match self.bits {
            0 => CFG1_A::CONSTANT_HIGH,
            1 => CFG1_A::STICKY_RISING_EDGE,
            2 => CFG1_A::STICKY_FALLING_EDGE,
            3 => CFG1_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG1_A::HIGH_LEVEL,
            5 => CFG1_A::LOW_LEVEL,
            6 => CFG1_A::CONSTANT_ZERO,
            7 => CFG1_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG1_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG1_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG1_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG1_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG1_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG1_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG1_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG1_A::EVENT
    }
}
#[doc = "Field `CFG1` writer - Specifies the match contribution condition for bit slice 1."]
pub type CFG1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMCFG_SPEC, u8, CFG1_A, 3, O>;
impl<'a, const O: u8> CFG1_W<'a, O> {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG1_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG1_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG1_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG1_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG1_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG1_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG1_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG1_A::EVENT)
    }
}
#[doc = "Field `CFG2` reader - Specifies the match contribution condition for bit slice 2."]
pub type CFG2_R = crate::FieldReader<u8, CFG2_A>;
#[doc = "Specifies the match contribution condition for bit slice 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFG2_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG2_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG2_A) -> Self {
        variant as _
    }
}
impl CFG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG2_A {
        match self.bits {
            0 => CFG2_A::CONSTANT_HIGH,
            1 => CFG2_A::STICKY_RISING_EDGE,
            2 => CFG2_A::STICKY_FALLING_EDGE,
            3 => CFG2_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG2_A::HIGH_LEVEL,
            5 => CFG2_A::LOW_LEVEL,
            6 => CFG2_A::CONSTANT_ZERO,
            7 => CFG2_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG2_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG2_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG2_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG2_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG2_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG2_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG2_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG2_A::EVENT
    }
}
#[doc = "Field `CFG2` writer - Specifies the match contribution condition for bit slice 2."]
pub type CFG2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMCFG_SPEC, u8, CFG2_A, 3, O>;
impl<'a, const O: u8> CFG2_W<'a, O> {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG2_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG2_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG2_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG2_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG2_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG2_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG2_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG2_A::EVENT)
    }
}
#[doc = "Field `CFG3` reader - Specifies the match contribution condition for bit slice 3."]
pub type CFG3_R = crate::FieldReader<u8, CFG3_A>;
#[doc = "Specifies the match contribution condition for bit slice 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFG3_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG3_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG3_A) -> Self {
        variant as _
    }
}
impl CFG3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG3_A {
        match self.bits {
            0 => CFG3_A::CONSTANT_HIGH,
            1 => CFG3_A::STICKY_RISING_EDGE,
            2 => CFG3_A::STICKY_FALLING_EDGE,
            3 => CFG3_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG3_A::HIGH_LEVEL,
            5 => CFG3_A::LOW_LEVEL,
            6 => CFG3_A::CONSTANT_ZERO,
            7 => CFG3_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG3_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG3_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG3_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG3_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG3_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG3_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG3_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG3_A::EVENT
    }
}
#[doc = "Field `CFG3` writer - Specifies the match contribution condition for bit slice 3."]
pub type CFG3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMCFG_SPEC, u8, CFG3_A, 3, O>;
impl<'a, const O: u8> CFG3_W<'a, O> {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG3_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG3_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG3_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG3_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG3_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG3_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG3_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG3_A::EVENT)
    }
}
#[doc = "Field `CFG4` reader - Specifies the match contribution condition for bit slice 4."]
pub type CFG4_R = crate::FieldReader<u8, CFG4_A>;
#[doc = "Specifies the match contribution condition for bit slice 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFG4_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG4_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG4_A) -> Self {
        variant as _
    }
}
impl CFG4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG4_A {
        match self.bits {
            0 => CFG4_A::CONSTANT_HIGH,
            1 => CFG4_A::STICKY_RISING_EDGE,
            2 => CFG4_A::STICKY_FALLING_EDGE,
            3 => CFG4_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG4_A::HIGH_LEVEL,
            5 => CFG4_A::LOW_LEVEL,
            6 => CFG4_A::CONSTANT_ZERO,
            7 => CFG4_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG4_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG4_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG4_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG4_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG4_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG4_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG4_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG4_A::EVENT
    }
}
#[doc = "Field `CFG4` writer - Specifies the match contribution condition for bit slice 4."]
pub type CFG4_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMCFG_SPEC, u8, CFG4_A, 3, O>;
impl<'a, const O: u8> CFG4_W<'a, O> {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG4_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG4_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG4_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG4_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG4_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG4_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG4_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG4_A::EVENT)
    }
}
#[doc = "Field `CFG5` reader - Specifies the match contribution condition for bit slice 5."]
pub type CFG5_R = crate::FieldReader<u8, CFG5_A>;
#[doc = "Specifies the match contribution condition for bit slice 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFG5_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG5_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG5_A) -> Self {
        variant as _
    }
}
impl CFG5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG5_A {
        match self.bits {
            0 => CFG5_A::CONSTANT_HIGH,
            1 => CFG5_A::STICKY_RISING_EDGE,
            2 => CFG5_A::STICKY_FALLING_EDGE,
            3 => CFG5_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG5_A::HIGH_LEVEL,
            5 => CFG5_A::LOW_LEVEL,
            6 => CFG5_A::CONSTANT_ZERO,
            7 => CFG5_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG5_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG5_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG5_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG5_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG5_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG5_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG5_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG5_A::EVENT
    }
}
#[doc = "Field `CFG5` writer - Specifies the match contribution condition for bit slice 5."]
pub type CFG5_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMCFG_SPEC, u8, CFG5_A, 3, O>;
impl<'a, const O: u8> CFG5_W<'a, O> {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG5_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG5_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG5_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG5_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG5_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG5_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG5_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG5_A::EVENT)
    }
}
#[doc = "Field `CFG6` reader - Specifies the match contribution condition for bit slice 6."]
pub type CFG6_R = crate::FieldReader<u8, CFG6_A>;
#[doc = "Specifies the match contribution condition for bit slice 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFG6_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG6_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG6_A) -> Self {
        variant as _
    }
}
impl CFG6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG6_A {
        match self.bits {
            0 => CFG6_A::CONSTANT_HIGH,
            1 => CFG6_A::STICKY_RISING_EDGE,
            2 => CFG6_A::STICKY_FALLING_EDGE,
            3 => CFG6_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG6_A::HIGH_LEVEL,
            5 => CFG6_A::LOW_LEVEL,
            6 => CFG6_A::CONSTANT_ZERO,
            7 => CFG6_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG6_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG6_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG6_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG6_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG6_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG6_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG6_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG6_A::EVENT
    }
}
#[doc = "Field `CFG6` writer - Specifies the match contribution condition for bit slice 6."]
pub type CFG6_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMCFG_SPEC, u8, CFG6_A, 3, O>;
impl<'a, const O: u8> CFG6_W<'a, O> {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG6_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG6_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG6_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG6_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG6_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG6_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG6_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG6_A::EVENT)
    }
}
#[doc = "Field `CFG7` reader - Specifies the match contribution condition for bit slice 7."]
pub type CFG7_R = crate::FieldReader<u8, CFG7_A>;
#[doc = "Specifies the match contribution condition for bit slice 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CFG7_A {
    #[doc = "0: Constant HIGH. This bit slice always contributes to a product term match."]
    CONSTANT_HIGH = 0,
    #[doc = "1: Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_EDGE = 1,
    #[doc = "2: Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_FALLING_EDGE = 2,
    #[doc = "3: Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    STICKY_RISING_FALLING_EDGE = 3,
    #[doc = "4: High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    HIGH_LEVEL = 4,
    #[doc = "5: Low level. Match occurs when there is a low level on the specified input."]
    LOW_LEVEL = 5,
    #[doc = "6: Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    CONSTANT_ZERO = 6,
    #[doc = "7: Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    EVENT = 7,
}
impl From<CFG7_A> for u8 {
    #[inline(always)]
    fn from(variant: CFG7_A) -> Self {
        variant as _
    }
}
impl CFG7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFG7_A {
        match self.bits {
            0 => CFG7_A::CONSTANT_HIGH,
            1 => CFG7_A::STICKY_RISING_EDGE,
            2 => CFG7_A::STICKY_FALLING_EDGE,
            3 => CFG7_A::STICKY_RISING_FALLING_EDGE,
            4 => CFG7_A::HIGH_LEVEL,
            5 => CFG7_A::LOW_LEVEL,
            6 => CFG7_A::CONSTANT_ZERO,
            7 => CFG7_A::EVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CONSTANT_HIGH`"]
    #[inline(always)]
    pub fn is_constant_high(&self) -> bool {
        *self == CFG7_A::CONSTANT_HIGH
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_edge(&self) -> bool {
        *self == CFG7_A::STICKY_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_falling_edge(&self) -> bool {
        *self == CFG7_A::STICKY_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `STICKY_RISING_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_sticky_rising_falling_edge(&self) -> bool {
        *self == CFG7_A::STICKY_RISING_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_high_level(&self) -> bool {
        *self == CFG7_A::HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `LOW_LEVEL`"]
    #[inline(always)]
    pub fn is_low_level(&self) -> bool {
        *self == CFG7_A::LOW_LEVEL
    }
    #[doc = "Checks if the value of the field is `CONSTANT_ZERO`"]
    #[inline(always)]
    pub fn is_constant_zero(&self) -> bool {
        *self == CFG7_A::CONSTANT_ZERO
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CFG7_A::EVENT
    }
}
#[doc = "Field `CFG7` writer - Specifies the match contribution condition for bit slice 7."]
pub type CFG7_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMCFG_SPEC, u8, CFG7_A, 3, O>;
impl<'a, const O: u8> CFG7_W<'a, O> {
    #[doc = "Constant HIGH. This bit slice always contributes to a product term match."]
    #[inline(always)]
    pub fn constant_high(self) -> &'a mut W {
        self.variant(CFG7_A::CONSTANT_HIGH)
    }
    #[doc = "Sticky rising edge. Match occurs if a rising edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_edge(self) -> &'a mut W {
        self.variant(CFG7_A::STICKY_RISING_EDGE)
    }
    #[doc = "Sticky falling edge. Match occurs if a falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_falling_edge(self) -> &'a mut W {
        self.variant(CFG7_A::STICKY_FALLING_EDGE)
    }
    #[doc = "Sticky rising or falling edge. Match occurs if either a rising or falling edge on the specified input has occurred since the last time the edge detection for this bit slice was cleared. This bit is only cleared when the PMCFG or the PMSRC registers are written to."]
    #[inline(always)]
    pub fn sticky_rising_falling_edge(self) -> &'a mut W {
        self.variant(CFG7_A::STICKY_RISING_FALLING_EDGE)
    }
    #[doc = "High level. Match (for this bit slice) occurs when there is a high level on the input specified for this bit slice in the PMSRC register."]
    #[inline(always)]
    pub fn high_level(self) -> &'a mut W {
        self.variant(CFG7_A::HIGH_LEVEL)
    }
    #[doc = "Low level. Match occurs when there is a low level on the specified input."]
    #[inline(always)]
    pub fn low_level(self) -> &'a mut W {
        self.variant(CFG7_A::LOW_LEVEL)
    }
    #[doc = "Constant 0. This bit slice never contributes to a match (should be used to disable any unused bit slices)."]
    #[inline(always)]
    pub fn constant_zero(self) -> &'a mut W {
        self.variant(CFG7_A::CONSTANT_ZERO)
    }
    #[doc = "Event. Non-sticky rising or falling edge. Match occurs on an event - i.e. when either a rising or falling edge is first detected on the specified input (this is a non-sticky version of value 0x3) . This bit is cleared after one clock cycle."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CFG7_A::EVENT)
    }
}
impl R {
    #[doc = "Bit 0 - Determines whether slice 0 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts0(&self) -> PROD_ENDPTS0_R {
        PROD_ENDPTS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Determines whether slice 1 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts1(&self) -> PROD_ENDPTS1_R {
        PROD_ENDPTS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Determines whether slice 2 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts2(&self) -> PROD_ENDPTS2_R {
        PROD_ENDPTS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Determines whether slice 3 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts3(&self) -> PROD_ENDPTS3_R {
        PROD_ENDPTS3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Determines whether slice 4 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts4(&self) -> PROD_ENDPTS4_R {
        PROD_ENDPTS4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Determines whether slice 5 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts5(&self) -> PROD_ENDPTS5_R {
        PROD_ENDPTS5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Determines whether slice 6 is an endpoint."]
    #[inline(always)]
    pub fn prod_endpts6(&self) -> PROD_ENDPTS6_R {
        PROD_ENDPTS6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    pub fn cfg0(&self) -> CFG0_R {
        CFG0_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    pub fn cfg1(&self) -> CFG1_R {
        CFG1_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:16 - Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    pub fn cfg2(&self) -> CFG2_R {
        CFG2_R::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    pub fn cfg3(&self) -> CFG3_R {
        CFG3_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    pub fn cfg4(&self) -> CFG4_R {
        CFG4_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    pub fn cfg5(&self) -> CFG5_R {
        CFG5_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    pub fn cfg6(&self) -> CFG6_R {
        CFG6_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Specifies the match contribution condition for bit slice 7."]
    #[inline(always)]
    pub fn cfg7(&self) -> CFG7_R {
        CFG7_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether slice 0 is an endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn prod_endpts0(&mut self) -> PROD_ENDPTS0_W<0> {
        PROD_ENDPTS0_W::new(self)
    }
    #[doc = "Bit 1 - Determines whether slice 1 is an endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn prod_endpts1(&mut self) -> PROD_ENDPTS1_W<1> {
        PROD_ENDPTS1_W::new(self)
    }
    #[doc = "Bit 2 - Determines whether slice 2 is an endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn prod_endpts2(&mut self) -> PROD_ENDPTS2_W<2> {
        PROD_ENDPTS2_W::new(self)
    }
    #[doc = "Bit 3 - Determines whether slice 3 is an endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn prod_endpts3(&mut self) -> PROD_ENDPTS3_W<3> {
        PROD_ENDPTS3_W::new(self)
    }
    #[doc = "Bit 4 - Determines whether slice 4 is an endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn prod_endpts4(&mut self) -> PROD_ENDPTS4_W<4> {
        PROD_ENDPTS4_W::new(self)
    }
    #[doc = "Bit 5 - Determines whether slice 5 is an endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn prod_endpts5(&mut self) -> PROD_ENDPTS5_W<5> {
        PROD_ENDPTS5_W::new(self)
    }
    #[doc = "Bit 6 - Determines whether slice 6 is an endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn prod_endpts6(&mut self) -> PROD_ENDPTS6_W<6> {
        PROD_ENDPTS6_W::new(self)
    }
    #[doc = "Bits 8:10 - Specifies the match contribution condition for bit slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn cfg0(&mut self) -> CFG0_W<8> {
        CFG0_W::new(self)
    }
    #[doc = "Bits 11:13 - Specifies the match contribution condition for bit slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn cfg1(&mut self) -> CFG1_W<11> {
        CFG1_W::new(self)
    }
    #[doc = "Bits 14:16 - Specifies the match contribution condition for bit slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn cfg2(&mut self) -> CFG2_W<14> {
        CFG2_W::new(self)
    }
    #[doc = "Bits 17:19 - Specifies the match contribution condition for bit slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn cfg3(&mut self) -> CFG3_W<17> {
        CFG3_W::new(self)
    }
    #[doc = "Bits 20:22 - Specifies the match contribution condition for bit slice 4."]
    #[inline(always)]
    #[must_use]
    pub fn cfg4(&mut self) -> CFG4_W<20> {
        CFG4_W::new(self)
    }
    #[doc = "Bits 23:25 - Specifies the match contribution condition for bit slice 5."]
    #[inline(always)]
    #[must_use]
    pub fn cfg5(&mut self) -> CFG5_W<23> {
        CFG5_W::new(self)
    }
    #[doc = "Bits 26:28 - Specifies the match contribution condition for bit slice 6."]
    #[inline(always)]
    #[must_use]
    pub fn cfg6(&mut self) -> CFG6_W<26> {
        CFG6_W::new(self)
    }
    #[doc = "Bits 29:31 - Specifies the match contribution condition for bit slice 7."]
    #[inline(always)]
    #[must_use]
    pub fn cfg7(&mut self) -> CFG7_W<29> {
        CFG7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pattern match interrupt bit slice configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmcfg](index.html) module"]
pub struct PMCFG_SPEC;
impl crate::RegisterSpec for PMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmcfg::R](R) reader structure"]
impl crate::Readable for PMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmcfg::W](W) writer structure"]
impl crate::Writable for PMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMCFG to value 0"]
impl crate::Resettable for PMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
