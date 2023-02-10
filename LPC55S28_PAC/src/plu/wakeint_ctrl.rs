#[doc = "Register `WAKEINT_CTRL` reader"]
pub struct R(crate::R<WAKEINT_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAKEINT_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAKEINT_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAKEINT_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAKEINT_CTRL` writer"]
pub struct W(crate::W<WAKEINT_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAKEINT_CTRL_SPEC>;
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
impl From<crate::W<WAKEINT_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAKEINT_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
pub type MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK` writer - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WAKEINT_CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `FILTER_MODE` reader - control input of the PLU, add filtering for glitch."]
pub type FILTER_MODE_R = crate::FieldReader<u8, FILTER_MODE_A>;
#[doc = "control input of the PLU, add filtering for glitch.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTER_MODE_A {
    #[doc = "0: Bypass mode."]
    BYPASS = 0,
    #[doc = "1: Filter 1 clock period."]
    FILTER1CLK = 1,
    #[doc = "2: Filter 2 clock period."]
    FILTER2CLK = 2,
    #[doc = "3: Filter 3 clock period."]
    FILTER3CLK = 3,
}
impl From<FILTER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_MODE_A) -> Self {
        variant as _
    }
}
impl FILTER_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_MODE_A {
        match self.bits {
            0 => FILTER_MODE_A::BYPASS,
            1 => FILTER_MODE_A::FILTER1CLK,
            2 => FILTER_MODE_A::FILTER2CLK,
            3 => FILTER_MODE_A::FILTER3CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BYPASS`"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == FILTER_MODE_A::BYPASS
    }
    #[doc = "Checks if the value of the field is `FILTER1CLK`"]
    #[inline(always)]
    pub fn is_filter1clk(&self) -> bool {
        *self == FILTER_MODE_A::FILTER1CLK
    }
    #[doc = "Checks if the value of the field is `FILTER2CLK`"]
    #[inline(always)]
    pub fn is_filter2clk(&self) -> bool {
        *self == FILTER_MODE_A::FILTER2CLK
    }
    #[doc = "Checks if the value of the field is `FILTER3CLK`"]
    #[inline(always)]
    pub fn is_filter3clk(&self) -> bool {
        *self == FILTER_MODE_A::FILTER3CLK
    }
}
#[doc = "Field `FILTER_MODE` writer - control input of the PLU, add filtering for glitch."]
pub type FILTER_MODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAKEINT_CTRL_SPEC, u8, FILTER_MODE_A, 2, O>;
impl<'a, const O: u8> FILTER_MODE_W<'a, O> {
    #[doc = "Bypass mode."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::BYPASS)
    }
    #[doc = "Filter 1 clock period."]
    #[inline(always)]
    pub fn filter1clk(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::FILTER1CLK)
    }
    #[doc = "Filter 2 clock period."]
    #[inline(always)]
    pub fn filter2clk(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::FILTER2CLK)
    }
    #[doc = "Filter 3 clock period."]
    #[inline(always)]
    pub fn filter3clk(self) -> &'a mut W {
        self.variant(FILTER_MODE_A::FILTER3CLK)
    }
}
#[doc = "Field `FILTER_CLKSEL` reader - hclk is divided by 2**filter_clksel."]
pub type FILTER_CLKSEL_R = crate::FieldReader<u8, FILTER_CLKSEL_A>;
#[doc = "hclk is divided by 2**filter_clksel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTER_CLKSEL_A {
    #[doc = "0: Selects the 1 MHz low-power oscillator as the filter clock."]
    FRO1MHZ = 0,
    #[doc = "1: Selects the 12 Mhz FRO as the filter clock."]
    FRO12MHZ = 1,
    #[doc = "2: Selects a third filter clock source, if provided."]
    OTHER_CLOCK = 2,
}
impl From<FILTER_CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_CLKSEL_A) -> Self {
        variant as _
    }
}
impl FILTER_CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FILTER_CLKSEL_A> {
        match self.bits {
            0 => Some(FILTER_CLKSEL_A::FRO1MHZ),
            1 => Some(FILTER_CLKSEL_A::FRO12MHZ),
            2 => Some(FILTER_CLKSEL_A::OTHER_CLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FRO1MHZ`"]
    #[inline(always)]
    pub fn is_fro1mhz(&self) -> bool {
        *self == FILTER_CLKSEL_A::FRO1MHZ
    }
    #[doc = "Checks if the value of the field is `FRO12MHZ`"]
    #[inline(always)]
    pub fn is_fro12mhz(&self) -> bool {
        *self == FILTER_CLKSEL_A::FRO12MHZ
    }
    #[doc = "Checks if the value of the field is `OTHER_CLOCK`"]
    #[inline(always)]
    pub fn is_other_clock(&self) -> bool {
        *self == FILTER_CLKSEL_A::OTHER_CLOCK
    }
}
#[doc = "Field `FILTER_CLKSEL` writer - hclk is divided by 2**filter_clksel."]
pub type FILTER_CLKSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAKEINT_CTRL_SPEC, u8, FILTER_CLKSEL_A, 2, O>;
impl<'a, const O: u8> FILTER_CLKSEL_W<'a, O> {
    #[doc = "Selects the 1 MHz low-power oscillator as the filter clock."]
    #[inline(always)]
    pub fn fro1mhz(self) -> &'a mut W {
        self.variant(FILTER_CLKSEL_A::FRO1MHZ)
    }
    #[doc = "Selects the 12 Mhz FRO as the filter clock."]
    #[inline(always)]
    pub fn fro12mhz(self) -> &'a mut W {
        self.variant(FILTER_CLKSEL_A::FRO12MHZ)
    }
    #[doc = "Selects a third filter clock source, if provided."]
    #[inline(always)]
    pub fn other_clock(self) -> &'a mut W {
        self.variant(FILTER_CLKSEL_A::OTHER_CLOCK)
    }
}
#[doc = "Field `LATCH_ENABLE` reader - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
pub type LATCH_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `LATCH_ENABLE` writer - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
pub type LATCH_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAKEINT_CTRL_SPEC, bool, O>;
#[doc = "Field `INTR_CLEAR` reader - Write to clear wakeint_latched"]
pub type INTR_CLEAR_R = crate::BitReader<bool>;
#[doc = "Field `INTR_CLEAR` writer - Write to clear wakeint_latched"]
pub type INTR_CLEAR_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, WAKEINT_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - control input of the PLU, add filtering for glitch."]
    #[inline(always)]
    pub fn filter_mode(&self) -> FILTER_MODE_R {
        FILTER_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - hclk is divided by 2**filter_clksel."]
    #[inline(always)]
    pub fn filter_clksel(&self) -> FILTER_CLKSEL_R {
        FILTER_CLKSEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
    #[inline(always)]
    pub fn latch_enable(&self) -> LATCH_ENABLE_R {
        LATCH_ENABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write to clear wakeint_latched"]
    #[inline(always)]
    pub fn intr_clear(&self) -> INTR_CLEAR_R {
        INTR_CLEAR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Interrupt mask (which of the 8 PLU Outputs contribute to interrupt)"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<0> {
        MASK_W::new(self)
    }
    #[doc = "Bits 8:9 - control input of the PLU, add filtering for glitch."]
    #[inline(always)]
    #[must_use]
    pub fn filter_mode(&mut self) -> FILTER_MODE_W<8> {
        FILTER_MODE_W::new(self)
    }
    #[doc = "Bits 10:11 - hclk is divided by 2**filter_clksel."]
    #[inline(always)]
    #[must_use]
    pub fn filter_clksel(&mut self) -> FILTER_CLKSEL_W<10> {
        FILTER_CLKSEL_W::new(self)
    }
    #[doc = "Bit 12 - latch the interrupt , then can be cleared with next bit INTR_CLEAR"]
    #[inline(always)]
    #[must_use]
    pub fn latch_enable(&mut self) -> LATCH_ENABLE_W<12> {
        LATCH_ENABLE_W::new(self)
    }
    #[doc = "Bit 13 - Write to clear wakeint_latched"]
    #[inline(always)]
    #[must_use]
    pub fn intr_clear(&mut self) -> INTR_CLEAR_W<13> {
        INTR_CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wakeup interrupt control for PLU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wakeint_ctrl](index.html) module"]
pub struct WAKEINT_CTRL_SPEC;
impl crate::RegisterSpec for WAKEINT_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wakeint_ctrl::R](R) reader structure"]
impl crate::Readable for WAKEINT_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wakeint_ctrl::W](W) writer structure"]
impl crate::Writable for WAKEINT_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x2000;
}
#[doc = "`reset()` method sets WAKEINT_CTRL to value 0"]
impl crate::Resettable for WAKEINT_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
