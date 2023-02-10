#[doc = "Register `PWD_CLR` reader"]
pub struct R(crate::R<PWD_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWD_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWD_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWD_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWD_CLR` writer"]
pub struct W(crate::W<PWD_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWD_CLR_SPEC>;
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
impl From<crate::W<PWD_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWD_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPWDFS` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TXPWDFS_R = crate::BitReader<TXPWDFS_A>;
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPWDFS_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    VALUE1 = 1,
}
impl From<TXPWDFS_A> for bool {
    #[inline(always)]
    fn from(variant: TXPWDFS_A) -> Self {
        variant as u8 != 0
    }
}
impl TXPWDFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPWDFS_A {
        match self.bits {
            false => TXPWDFS_A::VALUE0,
            true => TXPWDFS_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == TXPWDFS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXPWDFS_A::VALUE1
    }
}
#[doc = "Field `TXPWDFS` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TXPWDFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_CLR_SPEC, TXPWDFS_A, O>;
impl<'a, const O: u8> TXPWDFS_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(TXPWDFS_A::VALUE0)
    }
    #[doc = "Power-down the USB full-speed drivers. This turns off the current starvation sources and puts the"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXPWDFS_A::VALUE1)
    }
}
#[doc = "Field `TXPWDIBIAS` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TXPWDIBIAS_R = crate::BitReader<TXPWDIBIAS_A>;
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPWDIBIAS_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    VALUE1 = 1,
}
impl From<TXPWDIBIAS_A> for bool {
    #[inline(always)]
    fn from(variant: TXPWDIBIAS_A) -> Self {
        variant as u8 != 0
    }
}
impl TXPWDIBIAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPWDIBIAS_A {
        match self.bits {
            false => TXPWDIBIAS_A::VALUE0,
            true => TXPWDIBIAS_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == TXPWDIBIAS_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXPWDIBIAS_A::VALUE1
    }
}
#[doc = "Field `TXPWDIBIAS` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TXPWDIBIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_CLR_SPEC, TXPWDIBIAS_A, O>;
impl<'a, const O: u8> TXPWDIBIAS_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(TXPWDIBIAS_A::VALUE0)
    }
    #[doc = "Power-down the USB PHY current bias block for the transmitter. This bit should be set only when the"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXPWDIBIAS_A::VALUE1)
    }
}
#[doc = "Field `TXPWDV2I` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TXPWDV2I_R = crate::BitReader<TXPWDV2I_A>;
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPWDV2I_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    VALUE1 = 1,
}
impl From<TXPWDV2I_A> for bool {
    #[inline(always)]
    fn from(variant: TXPWDV2I_A) -> Self {
        variant as u8 != 0
    }
}
impl TXPWDV2I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPWDV2I_A {
        match self.bits {
            false => TXPWDV2I_A::VALUE0,
            true => TXPWDV2I_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == TXPWDV2I_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == TXPWDV2I_A::VALUE1
    }
}
#[doc = "Field `TXPWDV2I` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type TXPWDV2I_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_CLR_SPEC, TXPWDV2I_A, O>;
impl<'a, const O: u8> TXPWDV2I_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(TXPWDV2I_A::VALUE0)
    }
    #[doc = "Power-down the USB PHY transmit V-to-I converter and the current mirror"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(TXPWDV2I_A::VALUE1)
    }
}
#[doc = "Field `RXPWDENV` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RXPWDENV_R = crate::BitReader<RXPWDENV_A>;
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPWDENV_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    VALUE1 = 1,
}
impl From<RXPWDENV_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWDENV_A) -> Self {
        variant as u8 != 0
    }
}
impl RXPWDENV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWDENV_A {
        match self.bits {
            false => RXPWDENV_A::VALUE0,
            true => RXPWDENV_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == RXPWDENV_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXPWDENV_A::VALUE1
    }
}
#[doc = "Field `RXPWDENV` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RXPWDENV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_CLR_SPEC, RXPWDENV_A, O>;
impl<'a, const O: u8> RXPWDENV_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWDENV_A::VALUE0)
    }
    #[doc = "Power-down the USB high-speed receiver envelope detector (squelch signal)"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWDENV_A::VALUE1)
    }
}
#[doc = "Field `RXPWD1PT1` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RXPWD1PT1_R = crate::BitReader<RXPWD1PT1_A>;
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPWD1PT1_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB full-speed differential receiver."]
    VALUE1 = 1,
}
impl From<RXPWD1PT1_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWD1PT1_A) -> Self {
        variant as u8 != 0
    }
}
impl RXPWD1PT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWD1PT1_A {
        match self.bits {
            false => RXPWD1PT1_A::VALUE0,
            true => RXPWD1PT1_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == RXPWD1PT1_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXPWD1PT1_A::VALUE1
    }
}
#[doc = "Field `RXPWD1PT1` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RXPWD1PT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_CLR_SPEC, RXPWD1PT1_A, O>;
impl<'a, const O: u8> RXPWD1PT1_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWD1PT1_A::VALUE0)
    }
    #[doc = "Power-down the USB full-speed differential receiver."]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWD1PT1_A::VALUE1)
    }
}
#[doc = "Field `RXPWDDIFF` reader - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RXPWDDIFF_R = crate::BitReader<RXPWDDIFF_A>;
#[doc = "Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPWDDIFF_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the USB high-speed differential receive"]
    VALUE1 = 1,
}
impl From<RXPWDDIFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWDDIFF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXPWDDIFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWDDIFF_A {
        match self.bits {
            false => RXPWDDIFF_A::VALUE0,
            true => RXPWDDIFF_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == RXPWDDIFF_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXPWDDIFF_A::VALUE1
    }
}
#[doc = "Field `RXPWDDIFF` writer - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RXPWDDIFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_CLR_SPEC, RXPWDDIFF_A, O>;
impl<'a, const O: u8> RXPWDDIFF_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWDDIFF_A::VALUE0)
    }
    #[doc = "Power-down the USB high-speed differential receive"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWDDIFF_A::VALUE1)
    }
}
#[doc = "Field `RXPWDRX` reader - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RXPWDRX_R = crate::BitReader<RXPWDRX_A>;
#[doc = "This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPWDRX_A {
    #[doc = "0: Normal operation."]
    VALUE0 = 0,
    #[doc = "1: Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    VALUE1 = 1,
}
impl From<RXPWDRX_A> for bool {
    #[inline(always)]
    fn from(variant: RXPWDRX_A) -> Self {
        variant as u8 != 0
    }
}
impl RXPWDRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPWDRX_A {
        match self.bits {
            false => RXPWDRX_A::VALUE0,
            true => RXPWDRX_A::VALUE1,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE0`"]
    #[inline(always)]
    pub fn is_value0(&self) -> bool {
        *self == RXPWDRX_A::VALUE0
    }
    #[doc = "Checks if the value of the field is `VALUE1`"]
    #[inline(always)]
    pub fn is_value1(&self) -> bool {
        *self == RXPWDRX_A::VALUE1
    }
}
#[doc = "Field `RXPWDRX` writer - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
pub type RXPWDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWD_CLR_SPEC, RXPWDRX_A, O>;
impl<'a, const O: u8> RXPWDRX_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn value0(self) -> &'a mut W {
        self.variant(RXPWDRX_A::VALUE0)
    }
    #[doc = "Power-down the entire USB PHY receiver block except for the full-speed differential receiver"]
    #[inline(always)]
    pub fn value1(self) -> &'a mut W {
        self.variant(RXPWDRX_A::VALUE1)
    }
}
impl R {
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdfs(&self) -> TXPWDFS_R {
        TXPWDFS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdibias(&self) -> TXPWDIBIAS_R {
        TXPWDIBIAS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn txpwdv2i(&self) -> TXPWDV2I_R {
        TXPWDV2I_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdenv(&self) -> RXPWDENV_R {
        RXPWDENV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwd1pt1(&self) -> RXPWD1PT1_R {
        RXPWD1PT1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwddiff(&self) -> RXPWDDIFF_R {
        RXPWDDIFF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    pub fn rxpwdrx(&self) -> RXPWDRX_R {
        RXPWDRX_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn txpwdfs(&mut self) -> TXPWDFS_W<10> {
        TXPWDFS_W::new(self)
    }
    #[doc = "Bit 11 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn txpwdibias(&mut self) -> TXPWDIBIAS_W<11> {
        TXPWDIBIAS_W::new(self)
    }
    #[doc = "Bit 12 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn txpwdv2i(&mut self) -> TXPWDV2I_W<12> {
        TXPWDV2I_W::new(self)
    }
    #[doc = "Bit 17 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwdenv(&mut self) -> RXPWDENV_W<17> {
        RXPWDENV_W::new(self)
    }
    #[doc = "Bit 18 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwd1pt1(&mut self) -> RXPWD1PT1_W<18> {
        RXPWD1PT1_W::new(self)
    }
    #[doc = "Bit 19 - Note that this bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwddiff(&mut self) -> RXPWDDIFF_W<19> {
        RXPWDDIFF_W::new(self)
    }
    #[doc = "Bit 20 - This bit will be auto cleared if there is USB wakeup event while ENAUTOCLR_PHY_PWD bit of CTRL is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rxpwdrx(&mut self) -> RXPWDRX_W<20> {
        RXPWDRX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Power-Down Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_clr](index.html) module"]
pub struct PWD_CLR_SPEC;
impl crate::RegisterSpec for PWD_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwd_clr::R](R) reader structure"]
impl crate::Readable for PWD_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwd_clr::W](W) writer structure"]
impl crate::Writable for PWD_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWD_CLR to value 0x001e_1c00"]
impl crate::Resettable for PWD_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x001e_1c00;
}
