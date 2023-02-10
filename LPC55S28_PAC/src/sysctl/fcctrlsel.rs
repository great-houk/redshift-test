#[doc = "Register `FCCTRLSEL%s` reader"]
pub struct R(crate::R<FCCTRLSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCCTRLSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCCTRLSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCCTRLSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCCTRLSEL%s` writer"]
pub struct W(crate::W<FCCTRLSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCCTRLSEL_SPEC>;
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
impl From<crate::W<FCCTRLSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCCTRLSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCKINSEL` reader - Selects the source for SCK going into this Flexcomm."]
pub type SCKINSEL_R = crate::FieldReader<u8, SCKINSEL_A>;
#[doc = "Selects the source for SCK going into this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCKINSEL_A {
    #[doc = "0: Selects the dedicated FCn_SCK function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0,
    #[doc = "1: SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 1,
    #[doc = "2: SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 2,
}
impl From<SCKINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCKINSEL_A) -> Self {
        variant as _
    }
}
impl SCKINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SCKINSEL_A> {
        match self.bits {
            0 => Some(SCKINSEL_A::ORIG_FLEX_I2S_SIGNALS),
            1 => Some(SCKINSEL_A::SHARED_SET0_I2S_SIGNALS),
            2 => Some(SCKINSEL_A::SHARED_SET1_I2S_SIGNALS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == SCKINSEL_A::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == SCKINSEL_A::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == SCKINSEL_A::SHARED_SET1_I2S_SIGNALS
    }
}
#[doc = "Field `SCKINSEL` writer - Selects the source for SCK going into this Flexcomm."]
pub type SCKINSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCCTRLSEL_SPEC, u8, SCKINSEL_A, 2, O>;
impl<'a, const O: u8> SCKINSEL_W<'a, O> {
    #[doc = "Selects the dedicated FCn_SCK function for this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(SCKINSEL_A::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "SCK is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(SCKINSEL_A::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "SCK is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(SCKINSEL_A::SHARED_SET1_I2S_SIGNALS)
    }
}
#[doc = "Field `WSINSEL` reader - Selects the source for WS going into this Flexcomm."]
pub type WSINSEL_R = crate::FieldReader<u8, WSINSEL_A>;
#[doc = "Selects the source for WS going into this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WSINSEL_A {
    #[doc = "0: Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0,
    #[doc = "1: WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 1,
    #[doc = "2: WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 2,
}
impl From<WSINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WSINSEL_A) -> Self {
        variant as _
    }
}
impl WSINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WSINSEL_A> {
        match self.bits {
            0 => Some(WSINSEL_A::ORIG_FLEX_I2S_SIGNALS),
            1 => Some(WSINSEL_A::SHARED_SET0_I2S_SIGNALS),
            2 => Some(WSINSEL_A::SHARED_SET1_I2S_SIGNALS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == WSINSEL_A::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == WSINSEL_A::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == WSINSEL_A::SHARED_SET1_I2S_SIGNALS
    }
}
#[doc = "Field `WSINSEL` writer - Selects the source for WS going into this Flexcomm."]
pub type WSINSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCCTRLSEL_SPEC, u8, WSINSEL_A, 2, O>;
impl<'a, const O: u8> WSINSEL_W<'a, O> {
    #[doc = "Selects the dedicated (FCn_TXD_SCL_MISO_WS) function for this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(WSINSEL_A::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "WS is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(WSINSEL_A::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "WS is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(WSINSEL_A::SHARED_SET1_I2S_SIGNALS)
    }
}
#[doc = "Field `DATAINSEL` reader - Selects the source for DATA input to this Flexcomm."]
pub type DATAINSEL_R = crate::FieldReader<u8, DATAINSEL_A>;
#[doc = "Selects the source for DATA input to this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATAINSEL_A {
    #[doc = "0: Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0,
    #[doc = "1: Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 1,
    #[doc = "2: Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 2,
}
impl From<DATAINSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAINSEL_A) -> Self {
        variant as _
    }
}
impl DATAINSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATAINSEL_A> {
        match self.bits {
            0 => Some(DATAINSEL_A::ORIG_FLEX_I2S_SIGNALS),
            1 => Some(DATAINSEL_A::SHARED_SET0_I2S_SIGNALS),
            2 => Some(DATAINSEL_A::SHARED_SET1_I2S_SIGNALS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == DATAINSEL_A::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == DATAINSEL_A::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == DATAINSEL_A::SHARED_SET1_I2S_SIGNALS
    }
}
#[doc = "Field `DATAINSEL` writer - Selects the source for DATA input to this Flexcomm."]
pub type DATAINSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCCTRLSEL_SPEC, u8, DATAINSEL_A, 2, O>;
impl<'a, const O: u8> DATAINSEL_W<'a, O> {
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA input for this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(DATAINSEL_A::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "Input data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(DATAINSEL_A::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "Input data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(DATAINSEL_A::SHARED_SET1_I2S_SIGNALS)
    }
}
#[doc = "Field `DATAOUTSEL` reader - Selects the source for DATA output from this Flexcomm."]
pub type DATAOUTSEL_R = crate::FieldReader<u8, DATAOUTSEL_A>;
#[doc = "Selects the source for DATA output from this Flexcomm.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATAOUTSEL_A {
    #[doc = "0: Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    ORIG_FLEX_I2S_SIGNALS = 0,
    #[doc = "1: Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    SHARED_SET0_I2S_SIGNALS = 1,
    #[doc = "2: Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    SHARED_SET1_I2S_SIGNALS = 2,
}
impl From<DATAOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAOUTSEL_A) -> Self {
        variant as _
    }
}
impl DATAOUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATAOUTSEL_A> {
        match self.bits {
            0 => Some(DATAOUTSEL_A::ORIG_FLEX_I2S_SIGNALS),
            1 => Some(DATAOUTSEL_A::SHARED_SET0_I2S_SIGNALS),
            2 => Some(DATAOUTSEL_A::SHARED_SET1_I2S_SIGNALS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ORIG_FLEX_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_orig_flex_i2s_signals(&self) -> bool {
        *self == DATAOUTSEL_A::ORIG_FLEX_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET0_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set0_i2s_signals(&self) -> bool {
        *self == DATAOUTSEL_A::SHARED_SET0_I2S_SIGNALS
    }
    #[doc = "Checks if the value of the field is `SHARED_SET1_I2S_SIGNALS`"]
    #[inline(always)]
    pub fn is_shared_set1_i2s_signals(&self) -> bool {
        *self == DATAOUTSEL_A::SHARED_SET1_I2S_SIGNALS
    }
}
#[doc = "Field `DATAOUTSEL` writer - Selects the source for DATA output from this Flexcomm."]
pub type DATAOUTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCCTRLSEL_SPEC, u8, DATAOUTSEL_A, 2, O>;
impl<'a, const O: u8> DATAOUTSEL_W<'a, O> {
    #[doc = "Selects the dedicated FCn_RXD_SDA_MOSI_DATA output from this Flexcomm."]
    #[inline(always)]
    pub fn orig_flex_i2s_signals(self) -> &'a mut W {
        self.variant(DATAOUTSEL_A::ORIG_FLEX_I2S_SIGNALS)
    }
    #[doc = "Output data is taken from shared signal set 0 (defined by SHAREDCTRLSET0)."]
    #[inline(always)]
    pub fn shared_set0_i2s_signals(self) -> &'a mut W {
        self.variant(DATAOUTSEL_A::SHARED_SET0_I2S_SIGNALS)
    }
    #[doc = "Output data is taken from shared signal set 1 (defined by SHAREDCTRLSET1)."]
    #[inline(always)]
    pub fn shared_set1_i2s_signals(self) -> &'a mut W {
        self.variant(DATAOUTSEL_A::SHARED_SET1_I2S_SIGNALS)
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects the source for SCK going into this Flexcomm."]
    #[inline(always)]
    pub fn sckinsel(&self) -> SCKINSEL_R {
        SCKINSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Selects the source for WS going into this Flexcomm."]
    #[inline(always)]
    pub fn wsinsel(&self) -> WSINSEL_R {
        WSINSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Selects the source for DATA input to this Flexcomm."]
    #[inline(always)]
    pub fn datainsel(&self) -> DATAINSEL_R {
        DATAINSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Selects the source for DATA output from this Flexcomm."]
    #[inline(always)]
    pub fn dataoutsel(&self) -> DATAOUTSEL_R {
        DATAOUTSEL_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the source for SCK going into this Flexcomm."]
    #[inline(always)]
    #[must_use]
    pub fn sckinsel(&mut self) -> SCKINSEL_W<0> {
        SCKINSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Selects the source for WS going into this Flexcomm."]
    #[inline(always)]
    #[must_use]
    pub fn wsinsel(&mut self) -> WSINSEL_W<8> {
        WSINSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Selects the source for DATA input to this Flexcomm."]
    #[inline(always)]
    #[must_use]
    pub fn datainsel(&mut self) -> DATAINSEL_W<16> {
        DATAINSEL_W::new(self)
    }
    #[doc = "Bits 24:25 - Selects the source for DATA output from this Flexcomm."]
    #[inline(always)]
    #[must_use]
    pub fn dataoutsel(&mut self) -> DATAOUTSEL_W<24> {
        DATAOUTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects the source for SCK going into Flexcomm index\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcctrlsel](index.html) module"]
pub struct FCCTRLSEL_SPEC;
impl crate::RegisterSpec for FCCTRLSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcctrlsel::R](R) reader structure"]
impl crate::Readable for FCCTRLSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcctrlsel::W](W) writer structure"]
impl crate::Writable for FCCTRLSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCCTRLSEL%s to value 0"]
impl crate::Resettable for FCCTRLSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
