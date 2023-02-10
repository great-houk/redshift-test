#[doc = "Register `CTCR` reader"]
pub struct R(crate::R<CTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTCR` writer"]
pub struct W(crate::W<CTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTCR_SPEC>;
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
impl From<crate::W<CTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTMODE` reader - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
pub type CTMODE_R = crate::FieldReader<u8, CTMODE_A>;
#[doc = "Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTMODE_A {
    #[doc = "0: Timer Mode. Incremented every rising APB bus clock edge."]
    TIMER = 0,
    #[doc = "1: Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    COUNTER_RISING_EDGE = 1,
    #[doc = "2: Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    COUNTER_FALLING_EDGE = 2,
    #[doc = "3: Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    COUNTER_DUAL_EDGE = 3,
}
impl From<CTMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTMODE_A) -> Self {
        variant as _
    }
}
impl CTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTMODE_A {
        match self.bits {
            0 => CTMODE_A::TIMER,
            1 => CTMODE_A::COUNTER_RISING_EDGE,
            2 => CTMODE_A::COUNTER_FALLING_EDGE,
            3 => CTMODE_A::COUNTER_DUAL_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER`"]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == CTMODE_A::TIMER
    }
    #[doc = "Checks if the value of the field is `COUNTER_RISING_EDGE`"]
    #[inline(always)]
    pub fn is_counter_rising_edge(&self) -> bool {
        *self == CTMODE_A::COUNTER_RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `COUNTER_FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_counter_falling_edge(&self) -> bool {
        *self == CTMODE_A::COUNTER_FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `COUNTER_DUAL_EDGE`"]
    #[inline(always)]
    pub fn is_counter_dual_edge(&self) -> bool {
        *self == CTMODE_A::COUNTER_DUAL_EDGE
    }
}
#[doc = "Field `CTMODE` writer - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
pub type CTMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTCR_SPEC, u8, CTMODE_A, 2, O>;
impl<'a, const O: u8> CTMODE_W<'a, O> {
    #[doc = "Timer Mode. Incremented every rising APB bus clock edge."]
    #[inline(always)]
    pub fn timer(self) -> &'a mut W {
        self.variant(CTMODE_A::TIMER)
    }
    #[doc = "Counter Mode rising edge. TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_rising_edge(self) -> &'a mut W {
        self.variant(CTMODE_A::COUNTER_RISING_EDGE)
    }
    #[doc = "Counter Mode falling edge. TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_falling_edge(self) -> &'a mut W {
        self.variant(CTMODE_A::COUNTER_FALLING_EDGE)
    }
    #[doc = "Counter Mode dual edge. TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_dual_edge(self) -> &'a mut W {
        self.variant(CTMODE_A::COUNTER_DUAL_EDGE)
    }
}
#[doc = "Field `CINSEL` reader - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
pub type CINSEL_R = crate::FieldReader<u8, CINSEL_A>;
#[doc = "Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CINSEL_A {
    #[doc = "0: Channel 0. CAPn.0 for CTIMERn"]
    CHANNEL_0 = 0,
    #[doc = "1: Channel 1. CAPn.1 for CTIMERn"]
    CHANNEL_1 = 1,
    #[doc = "2: Channel 2. CAPn.2 for CTIMERn"]
    CHANNEL_2 = 2,
    #[doc = "3: Channel 3. CAPn.3 for CTIMERn"]
    CHANNEL_3 = 3,
}
impl From<CINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CINSEL_A) -> Self {
        variant as _
    }
}
impl CINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINSEL_A {
        match self.bits {
            0 => CINSEL_A::CHANNEL_0,
            1 => CINSEL_A::CHANNEL_1,
            2 => CINSEL_A::CHANNEL_2,
            3 => CINSEL_A::CHANNEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0`"]
    #[inline(always)]
    pub fn is_channel_0(&self) -> bool {
        *self == CINSEL_A::CHANNEL_0
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1`"]
    #[inline(always)]
    pub fn is_channel_1(&self) -> bool {
        *self == CINSEL_A::CHANNEL_1
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2`"]
    #[inline(always)]
    pub fn is_channel_2(&self) -> bool {
        *self == CINSEL_A::CHANNEL_2
    }
    #[doc = "Checks if the value of the field is `CHANNEL_3`"]
    #[inline(always)]
    pub fn is_channel_3(&self) -> bool {
        *self == CINSEL_A::CHANNEL_3
    }
}
#[doc = "Field `CINSEL` writer - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
pub type CINSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTCR_SPEC, u8, CINSEL_A, 2, O>;
impl<'a, const O: u8> CINSEL_W<'a, O> {
    #[doc = "Channel 0. CAPn.0 for CTIMERn"]
    #[inline(always)]
    pub fn channel_0(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_0)
    }
    #[doc = "Channel 1. CAPn.1 for CTIMERn"]
    #[inline(always)]
    pub fn channel_1(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_1)
    }
    #[doc = "Channel 2. CAPn.2 for CTIMERn"]
    #[inline(always)]
    pub fn channel_2(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_2)
    }
    #[doc = "Channel 3. CAPn.3 for CTIMERn"]
    #[inline(always)]
    pub fn channel_3(self) -> &'a mut W {
        self.variant(CINSEL_A::CHANNEL_3)
    }
}
#[doc = "Field `ENCC` reader - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
pub type ENCC_R = crate::BitReader<bool>;
#[doc = "Field `ENCC` writer - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
pub type ENCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTCR_SPEC, bool, O>;
#[doc = "Field `SELCC` reader - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
pub type SELCC_R = crate::FieldReader<u8, SELCC_A>;
#[doc = "Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELCC_A {
    #[doc = "0: Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    CHANNEL_0_RISING = 0,
    #[doc = "1: Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    CHANNEL_0_FALLING = 1,
    #[doc = "2: Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    CHANNEL_1_RISING = 2,
    #[doc = "3: Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    CHANNEL_1_FALLING = 3,
    #[doc = "4: Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    CHANNEL_2_RISING = 4,
    #[doc = "5: Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    CHANNEL_2_FALLING = 5,
}
impl From<SELCC_A> for u8 {
    #[inline(always)]
    fn from(variant: SELCC_A) -> Self {
        variant as _
    }
}
impl SELCC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELCC_A> {
        match self.bits {
            0 => Some(SELCC_A::CHANNEL_0_RISING),
            1 => Some(SELCC_A::CHANNEL_0_FALLING),
            2 => Some(SELCC_A::CHANNEL_1_RISING),
            3 => Some(SELCC_A::CHANNEL_1_FALLING),
            4 => Some(SELCC_A::CHANNEL_2_RISING),
            5 => Some(SELCC_A::CHANNEL_2_FALLING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0_RISING`"]
    #[inline(always)]
    pub fn is_channel_0_rising(&self) -> bool {
        *self == SELCC_A::CHANNEL_0_RISING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_0_FALLING`"]
    #[inline(always)]
    pub fn is_channel_0_falling(&self) -> bool {
        *self == SELCC_A::CHANNEL_0_FALLING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1_RISING`"]
    #[inline(always)]
    pub fn is_channel_1_rising(&self) -> bool {
        *self == SELCC_A::CHANNEL_1_RISING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_1_FALLING`"]
    #[inline(always)]
    pub fn is_channel_1_falling(&self) -> bool {
        *self == SELCC_A::CHANNEL_1_FALLING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2_RISING`"]
    #[inline(always)]
    pub fn is_channel_2_rising(&self) -> bool {
        *self == SELCC_A::CHANNEL_2_RISING
    }
    #[doc = "Checks if the value of the field is `CHANNEL_2_FALLING`"]
    #[inline(always)]
    pub fn is_channel_2_falling(&self) -> bool {
        *self == SELCC_A::CHANNEL_2_FALLING
    }
}
#[doc = "Field `SELCC` writer - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
pub type SELCC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTCR_SPEC, u8, SELCC_A, 3, O>;
impl<'a, const O: u8> SELCC_W<'a, O> {
    #[doc = "Channel 0 Rising Edge. Rising edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_0_rising(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_0_RISING)
    }
    #[doc = "Channel 0 Falling Edge. Falling edge of the signal on capture channel 0 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_0_falling(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_0_FALLING)
    }
    #[doc = "Channel 1 Rising Edge. Rising edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_1_rising(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_1_RISING)
    }
    #[doc = "Channel 1 Falling Edge. Falling edge of the signal on capture channel 1 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_1_falling(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_1_FALLING)
    }
    #[doc = "Channel 2 Rising Edge. Rising edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_2_rising(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_2_RISING)
    }
    #[doc = "Channel 2 Falling Edge. Falling edge of the signal on capture channel 2 clears the timer (if bit 4 is set)."]
    #[inline(always)]
    pub fn channel_2_falling(self) -> &'a mut W {
        self.variant(SELCC_A::CHANNEL_2_FALLING)
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub fn ctmode(&self) -> CTMODE_R {
        CTMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub fn cinsel(&self) -> CINSEL_R {
        CINSEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    pub fn encc(&self) -> ENCC_R {
        ENCC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline(always)]
    pub fn selcc(&self) -> SELCC_R {
        SELCC_R::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising APB bus clock edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    #[must_use]
    pub fn ctmode(&mut self) -> CTMODE_W<0> {
        CTMODE_W::new(self)
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the CTCR, the 3 bits for that input in the Capture Control Register (CCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    #[must_use]
    pub fn cinsel(&mut self) -> CINSEL_W<2> {
        CINSEL_W::new(self)
    }
    #[doc = "Bit 4 - Setting this bit to 1 enables clearing of the timer and the prescaler when the capture-edge event specified in bits 7:5 occurs."]
    #[inline(always)]
    #[must_use]
    pub fn encc(&mut self) -> ENCC_W<4> {
        ENCC_W::new(self)
    }
    #[doc = "Bits 5:7 - Edge select. When bit 4 is 1, these bits select which capture input edge will cause the timer and prescaler to be cleared. These bits have no effect when bit 4 is low. Values 0x2 to 0x3 and 0x6 to 0x7 are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn selcc(&mut self) -> SELCC_W<5> {
        SELCC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctcr](index.html) module"]
pub struct CTCR_SPEC;
impl crate::RegisterSpec for CTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctcr::R](R) reader structure"]
impl crate::Readable for CTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctcr::W](W) writer structure"]
impl crate::Writable for CTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTCR to value 0"]
impl crate::Resettable for CTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
