#[doc = "Register `DCFG_CC_SOCU_DFLT` reader"]
pub struct R(crate::R<DCFG_CC_SOCU_DFLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFG_CC_SOCU_DFLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFG_CC_SOCU_DFLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFG_CC_SOCU_DFLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCFG_CC_SOCU_DFLT` writer"]
pub struct W(crate::W<DCFG_CC_SOCU_DFLT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFG_CC_SOCU_DFLT_SPEC>;
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
impl From<crate::W<DCFG_CC_SOCU_DFLT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFG_CC_SOCU_DFLT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NIDEN` reader - Non Secure non-invasive debug fixed state"]
pub type NIDEN_R = crate::BitReader<NIDEN_A>;
#[doc = "Non Secure non-invasive debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NIDEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<NIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: NIDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NIDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NIDEN_A {
        match self.bits {
            false => NIDEN_A::DISABLE,
            true => NIDEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == NIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == NIDEN_A::ENABLE
    }
}
#[doc = "Field `NIDEN` writer - Non Secure non-invasive debug fixed state"]
pub type NIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFG_CC_SOCU_DFLT_SPEC, NIDEN_A, O>;
impl<'a, const O: u8> NIDEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(NIDEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(NIDEN_A::ENABLE)
    }
}
#[doc = "Field `DBGEN` reader - Non Secure debug fixed state"]
pub type DBGEN_R = crate::BitReader<DBGEN_A>;
#[doc = "Non Secure debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::DISABLE,
            true => DBGEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBGEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DBGEN_A::ENABLE
    }
}
#[doc = "Field `DBGEN` writer - Non Secure debug fixed state"]
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFG_CC_SOCU_DFLT_SPEC, DBGEN_A, O>;
impl<'a, const O: u8> DBGEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DBGEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DBGEN_A::ENABLE)
    }
}
#[doc = "Field `SPNIDEN` reader - Secure non-invasive debug fixed state"]
pub type SPNIDEN_R = crate::BitReader<SPNIDEN_A>;
#[doc = "Secure non-invasive debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPNIDEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<SPNIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPNIDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPNIDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPNIDEN_A {
        match self.bits {
            false => SPNIDEN_A::DISABLE,
            true => SPNIDEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SPNIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPNIDEN_A::ENABLE
    }
}
#[doc = "Field `SPNIDEN` writer - Secure non-invasive debug fixed state"]
pub type SPNIDEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCFG_CC_SOCU_DFLT_SPEC, SPNIDEN_A, O>;
impl<'a, const O: u8> SPNIDEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPNIDEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPNIDEN_A::ENABLE)
    }
}
#[doc = "Field `SPIDEN` reader - Secure invasive debug fixed state"]
pub type SPIDEN_R = crate::BitReader<SPIDEN_A>;
#[doc = "Secure invasive debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIDEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<SPIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SPIDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPIDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIDEN_A {
        match self.bits {
            false => SPIDEN_A::DISABLE,
            true => SPIDEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SPIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SPIDEN_A::ENABLE
    }
}
#[doc = "Field `SPIDEN` writer - Secure invasive debug fixed state"]
pub type SPIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFG_CC_SOCU_DFLT_SPEC, SPIDEN_A, O>;
impl<'a, const O: u8> SPIDEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SPIDEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SPIDEN_A::ENABLE)
    }
}
#[doc = "Field `TAPEN` reader - JTAG TAP fixed state"]
pub type TAPEN_R = crate::BitReader<TAPEN_A>;
#[doc = "JTAG TAP fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAPEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<TAPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TAPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TAPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAPEN_A {
        match self.bits {
            false => TAPEN_A::DISABLE,
            true => TAPEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TAPEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TAPEN_A::ENABLE
    }
}
#[doc = "Field `TAPEN` writer - JTAG TAP fixed state"]
pub type TAPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCFG_CC_SOCU_DFLT_SPEC, TAPEN_A, O>;
impl<'a, const O: u8> TAPEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TAPEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TAPEN_A::ENABLE)
    }
}
#[doc = "Field `CPU1_DBGEN` reader - CPU1 (Micro cortex M33) invasive debug fixed state"]
pub type CPU1_DBGEN_R = crate::BitReader<CPU1_DBGEN_A>;
#[doc = "CPU1 (Micro cortex M33) invasive debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPU1_DBGEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CPU1_DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1_DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CPU1_DBGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1_DBGEN_A {
        match self.bits {
            false => CPU1_DBGEN_A::DISABLE,
            true => CPU1_DBGEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU1_DBGEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU1_DBGEN_A::ENABLE
    }
}
#[doc = "Field `CPU1_DBGEN` writer - CPU1 (Micro cortex M33) invasive debug fixed state"]
pub type CPU1_DBGEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCFG_CC_SOCU_DFLT_SPEC, CPU1_DBGEN_A, O>;
impl<'a, const O: u8> CPU1_DBGEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU1_DBGEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1_DBGEN_A::ENABLE)
    }
}
#[doc = "Field `ISP_CMD_EN` reader - ISP Boot Command fixed state"]
pub type ISP_CMD_EN_R = crate::BitReader<ISP_CMD_EN_A>;
#[doc = "ISP Boot Command fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISP_CMD_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ISP_CMD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ISP_CMD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ISP_CMD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISP_CMD_EN_A {
        match self.bits {
            false => ISP_CMD_EN_A::DISABLE,
            true => ISP_CMD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ISP_CMD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ISP_CMD_EN_A::ENABLE
    }
}
#[doc = "Field `ISP_CMD_EN` writer - ISP Boot Command fixed state"]
pub type ISP_CMD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCFG_CC_SOCU_DFLT_SPEC, ISP_CMD_EN_A, O>;
impl<'a, const O: u8> ISP_CMD_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ISP_CMD_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ISP_CMD_EN_A::ENABLE)
    }
}
#[doc = "Field `FA_CMD_EN` reader - FA Command fixed state"]
pub type FA_CMD_EN_R = crate::BitReader<FA_CMD_EN_A>;
#[doc = "FA Command fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FA_CMD_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FA_CMD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FA_CMD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl FA_CMD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FA_CMD_EN_A {
        match self.bits {
            false => FA_CMD_EN_A::DISABLE,
            true => FA_CMD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FA_CMD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FA_CMD_EN_A::ENABLE
    }
}
#[doc = "Field `FA_CMD_EN` writer - FA Command fixed state"]
pub type FA_CMD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCFG_CC_SOCU_DFLT_SPEC, FA_CMD_EN_A, O>;
impl<'a, const O: u8> FA_CMD_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FA_CMD_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FA_CMD_EN_A::ENABLE)
    }
}
#[doc = "Field `ME_CMD_EN` reader - Flash Mass Erase Command fixed state"]
pub type ME_CMD_EN_R = crate::BitReader<ME_CMD_EN_A>;
#[doc = "Flash Mass Erase Command fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ME_CMD_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ME_CMD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ME_CMD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ME_CMD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ME_CMD_EN_A {
        match self.bits {
            false => ME_CMD_EN_A::DISABLE,
            true => ME_CMD_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ME_CMD_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ME_CMD_EN_A::ENABLE
    }
}
#[doc = "Field `ME_CMD_EN` writer - Flash Mass Erase Command fixed state"]
pub type ME_CMD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCFG_CC_SOCU_DFLT_SPEC, ME_CMD_EN_A, O>;
impl<'a, const O: u8> ME_CMD_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ME_CMD_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ME_CMD_EN_A::ENABLE)
    }
}
#[doc = "Field `CPU1_NIDEN` reader - CPU1 (Micro cortex M33) non-invasive debug fixed state"]
pub type CPU1_NIDEN_R = crate::BitReader<CPU1_NIDEN_A>;
#[doc = "CPU1 (Micro cortex M33) non-invasive debug fixed state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPU1_NIDEN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<CPU1_NIDEN_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1_NIDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CPU1_NIDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1_NIDEN_A {
        match self.bits {
            false => CPU1_NIDEN_A::DISABLE,
            true => CPU1_NIDEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CPU1_NIDEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CPU1_NIDEN_A::ENABLE
    }
}
#[doc = "Field `CPU1_NIDEN` writer - CPU1 (Micro cortex M33) non-invasive debug fixed state"]
pub type CPU1_NIDEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DCFG_CC_SOCU_DFLT_SPEC, CPU1_NIDEN_A, O>;
impl<'a, const O: u8> CPU1_NIDEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CPU1_NIDEN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CPU1_NIDEN_A::ENABLE)
    }
}
#[doc = "Field `INVERSE_VALUE` reader - inverse value of bits \\[15:0\\]"]
pub type INVERSE_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INVERSE_VALUE` writer - inverse value of bits \\[15:0\\]"]
pub type INVERSE_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCFG_CC_SOCU_DFLT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Non Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub fn niden(&self) -> NIDEN_R {
        NIDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non Secure debug fixed state"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure non-invasive debug fixed state"]
    #[inline(always)]
    pub fn spniden(&self) -> SPNIDEN_R {
        SPNIDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secure invasive debug fixed state"]
    #[inline(always)]
    pub fn spiden(&self) -> SPIDEN_R {
        SPIDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - JTAG TAP fixed state"]
    #[inline(always)]
    pub fn tapen(&self) -> TAPEN_R {
        TAPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU1 (Micro cortex M33) invasive debug fixed state"]
    #[inline(always)]
    pub fn cpu1_dbgen(&self) -> CPU1_DBGEN_R {
        CPU1_DBGEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ISP Boot Command fixed state"]
    #[inline(always)]
    pub fn isp_cmd_en(&self) -> ISP_CMD_EN_R {
        ISP_CMD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FA Command fixed state"]
    #[inline(always)]
    pub fn fa_cmd_en(&self) -> FA_CMD_EN_R {
        FA_CMD_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Mass Erase Command fixed state"]
    #[inline(always)]
    pub fn me_cmd_en(&self) -> ME_CMD_EN_R {
        ME_CMD_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU1 (Micro cortex M33) non-invasive debug fixed state"]
    #[inline(always)]
    pub fn cpu1_niden(&self) -> CPU1_NIDEN_R {
        CPU1_NIDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    pub fn inverse_value(&self) -> INVERSE_VALUE_R {
        INVERSE_VALUE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Non Secure non-invasive debug fixed state"]
    #[inline(always)]
    #[must_use]
    pub fn niden(&mut self) -> NIDEN_W<0> {
        NIDEN_W::new(self)
    }
    #[doc = "Bit 1 - Non Secure debug fixed state"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<1> {
        DBGEN_W::new(self)
    }
    #[doc = "Bit 2 - Secure non-invasive debug fixed state"]
    #[inline(always)]
    #[must_use]
    pub fn spniden(&mut self) -> SPNIDEN_W<2> {
        SPNIDEN_W::new(self)
    }
    #[doc = "Bit 3 - Secure invasive debug fixed state"]
    #[inline(always)]
    #[must_use]
    pub fn spiden(&mut self) -> SPIDEN_W<3> {
        SPIDEN_W::new(self)
    }
    #[doc = "Bit 4 - JTAG TAP fixed state"]
    #[inline(always)]
    #[must_use]
    pub fn tapen(&mut self) -> TAPEN_W<4> {
        TAPEN_W::new(self)
    }
    #[doc = "Bit 5 - CPU1 (Micro cortex M33) invasive debug fixed state"]
    #[inline(always)]
    #[must_use]
    pub fn cpu1_dbgen(&mut self) -> CPU1_DBGEN_W<5> {
        CPU1_DBGEN_W::new(self)
    }
    #[doc = "Bit 6 - ISP Boot Command fixed state"]
    #[inline(always)]
    #[must_use]
    pub fn isp_cmd_en(&mut self) -> ISP_CMD_EN_W<6> {
        ISP_CMD_EN_W::new(self)
    }
    #[doc = "Bit 7 - FA Command fixed state"]
    #[inline(always)]
    #[must_use]
    pub fn fa_cmd_en(&mut self) -> FA_CMD_EN_W<7> {
        FA_CMD_EN_W::new(self)
    }
    #[doc = "Bit 8 - Flash Mass Erase Command fixed state"]
    #[inline(always)]
    #[must_use]
    pub fn me_cmd_en(&mut self) -> ME_CMD_EN_W<8> {
        ME_CMD_EN_W::new(self)
    }
    #[doc = "Bit 9 - CPU1 (Micro cortex M33) non-invasive debug fixed state"]
    #[inline(always)]
    #[must_use]
    pub fn cpu1_niden(&mut self) -> CPU1_NIDEN_W<9> {
        CPU1_NIDEN_W::new(self)
    }
    #[doc = "Bits 16:31 - inverse value of bits \\[15:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn inverse_value(&mut self) -> INVERSE_VALUE_W<16> {
        INVERSE_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "With TZ-M, the part can be sold by level 1 customers (secure code developer) to level-2 customers who develops non-secure code only. - In this scenario, or easy of development, Level-I customer releases the part to always allow non-secure debug. - To allow level-2 customers to further seal the part DCFG_CC_SOCU_NS is used. - ROM will use this word to further restrict the debug access.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg_cc_socu_dflt](index.html) module"]
pub struct DCFG_CC_SOCU_DFLT_SPEC;
impl crate::RegisterSpec for DCFG_CC_SOCU_DFLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcfg_cc_socu_dflt::R](R) reader structure"]
impl crate::Readable for DCFG_CC_SOCU_DFLT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcfg_cc_socu_dflt::W](W) writer structure"]
impl crate::Writable for DCFG_CC_SOCU_DFLT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCFG_CC_SOCU_DFLT to value 0"]
impl crate::Resettable for DCFG_CC_SOCU_DFLT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
