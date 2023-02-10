#[doc = "Register `PMCTRL` reader"]
pub struct R(crate::R<PMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMCTRL` writer"]
pub struct W(crate::W<PMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCTRL_SPEC>;
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
impl From<crate::W<PMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEL_PMATCH` reader - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
pub type SEL_PMATCH_R = crate::BitReader<SEL_PMATCH_A>;
#[doc = "Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEL_PMATCH_A {
    #[doc = "0: Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    PIN_INTERRUPT = 0,
    #[doc = "1: Pattern match. Interrupts are driven in response to pattern matches."]
    PATTERN_MATCH = 1,
}
impl From<SEL_PMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: SEL_PMATCH_A) -> Self {
        variant as u8 != 0
    }
}
impl SEL_PMATCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEL_PMATCH_A {
        match self.bits {
            false => SEL_PMATCH_A::PIN_INTERRUPT,
            true => SEL_PMATCH_A::PATTERN_MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `PIN_INTERRUPT`"]
    #[inline(always)]
    pub fn is_pin_interrupt(&self) -> bool {
        *self == SEL_PMATCH_A::PIN_INTERRUPT
    }
    #[doc = "Checks if the value of the field is `PATTERN_MATCH`"]
    #[inline(always)]
    pub fn is_pattern_match(&self) -> bool {
        *self == SEL_PMATCH_A::PATTERN_MATCH
    }
}
#[doc = "Field `SEL_PMATCH` writer - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
pub type SEL_PMATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCTRL_SPEC, SEL_PMATCH_A, O>;
impl<'a, const O: u8> SEL_PMATCH_W<'a, O> {
    #[doc = "Pin interrupt. Interrupts are driven in response to the standard pin interrupt function."]
    #[inline(always)]
    pub fn pin_interrupt(self) -> &'a mut W {
        self.variant(SEL_PMATCH_A::PIN_INTERRUPT)
    }
    #[doc = "Pattern match. Interrupts are driven in response to pattern matches."]
    #[inline(always)]
    pub fn pattern_match(self) -> &'a mut W {
        self.variant(SEL_PMATCH_A::PATTERN_MATCH)
    }
}
#[doc = "Field `ENA_RXEV` reader - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
pub type ENA_RXEV_R = crate::BitReader<ENA_RXEV_A>;
#[doc = "Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENA_RXEV_A {
    #[doc = "0: Disabled. RXEV output to the CPU is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. RXEV output to the CPU is enabled."]
    ENABLED = 1,
}
impl From<ENA_RXEV_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_RXEV_A) -> Self {
        variant as u8 != 0
    }
}
impl ENA_RXEV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_RXEV_A {
        match self.bits {
            false => ENA_RXEV_A::DISABLED,
            true => ENA_RXEV_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENA_RXEV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENA_RXEV_A::ENABLED
    }
}
#[doc = "Field `ENA_RXEV` writer - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
pub type ENA_RXEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMCTRL_SPEC, ENA_RXEV_A, O>;
impl<'a, const O: u8> ENA_RXEV_W<'a, O> {
    #[doc = "Disabled. RXEV output to the CPU is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENA_RXEV_A::DISABLED)
    }
    #[doc = "Enabled. RXEV output to the CPU is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENA_RXEV_A::ENABLED)
    }
}
#[doc = "Field `PMAT` reader - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
pub type PMAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PMAT` writer - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
pub type PMAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMCTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline(always)]
    pub fn sel_pmatch(&self) -> SEL_PMATCH_R {
        SEL_PMATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline(always)]
    pub fn ena_rxev(&self) -> ENA_RXEV_R {
        ENA_RXEV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 24:31 - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline(always)]
    pub fn pmat(&self) -> PMAT_R {
        PMAT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies whether the 8 pin interrupts are controlled by the pin interrupt function or by the pattern match function."]
    #[inline(always)]
    #[must_use]
    pub fn sel_pmatch(&mut self) -> SEL_PMATCH_W<0> {
        SEL_PMATCH_W::new(self)
    }
    #[doc = "Bit 1 - Enables the RXEV output to the CPU and/or to a GPIO output when the specified boolean expression evaluates to true."]
    #[inline(always)]
    #[must_use]
    pub fn ena_rxev(&mut self) -> ENA_RXEV_W<1> {
        ENA_RXEV_W::new(self)
    }
    #[doc = "Bits 24:31 - This field displays the current state of pattern matches. A 1 in any bit of this field indicates that the corresponding product term is matched by the current state of the appropriate inputs."]
    #[inline(always)]
    #[must_use]
    pub fn pmat(&mut self) -> PMAT_W<24> {
        PMAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pattern match interrupt control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmctrl](index.html) module"]
pub struct PMCTRL_SPEC;
impl crate::RegisterSpec for PMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmctrl::R](R) reader structure"]
impl crate::Readable for PMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmctrl::W](W) writer structure"]
impl crate::Writable for PMCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMCTRL to value 0"]
impl crate::Resettable for PMCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
