#[doc = "Register `BOOT_CFG` reader"]
pub struct R(crate::R<BOOT_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOT_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOT_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOT_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOT_CFG` writer"]
pub struct W(crate::W<BOOT_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOT_CFG_SPEC>;
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
impl From<crate::W<BOOT_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOT_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEFAULT_ISP_MODE` reader - Default ISP mode:"]
pub type DEFAULT_ISP_MODE_R = crate::FieldReader<u8, DEFAULT_ISP_MODE_A>;
#[doc = "Default ISP mode:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEFAULT_ISP_MODE_A {
    #[doc = "0: Auto ISP"]
    VALUE_0 = 0,
    #[doc = "1: USB_HID_MSC"]
    VALUE_1 = 1,
    #[doc = "2: SPI Slave ISP"]
    VALUE_2 = 2,
    #[doc = "3: I2C Slave ISP"]
    VALUE_3 = 3,
    #[doc = "7: Disable ISP fall through"]
    VALUE_7 = 7,
}
impl From<DEFAULT_ISP_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEFAULT_ISP_MODE_A) -> Self {
        variant as _
    }
}
impl DEFAULT_ISP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEFAULT_ISP_MODE_A> {
        match self.bits {
            0 => Some(DEFAULT_ISP_MODE_A::VALUE_0),
            1 => Some(DEFAULT_ISP_MODE_A::VALUE_1),
            2 => Some(DEFAULT_ISP_MODE_A::VALUE_2),
            3 => Some(DEFAULT_ISP_MODE_A::VALUE_3),
            7 => Some(DEFAULT_ISP_MODE_A::VALUE_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::VALUE_1
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::VALUE_2
    }
    #[doc = "Checks if the value of the field is `VALUE_3`"]
    #[inline(always)]
    pub fn is_value_3(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::VALUE_3
    }
    #[doc = "Checks if the value of the field is `VALUE_7`"]
    #[inline(always)]
    pub fn is_value_7(&self) -> bool {
        *self == DEFAULT_ISP_MODE_A::VALUE_7
    }
}
#[doc = "Field `DEFAULT_ISP_MODE` writer - Default ISP mode:"]
pub type DEFAULT_ISP_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT_CFG_SPEC, u8, DEFAULT_ISP_MODE_A, 3, O>;
impl<'a, const O: u8> DEFAULT_ISP_MODE_W<'a, O> {
    #[doc = "Auto ISP"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::VALUE_0)
    }
    #[doc = "USB_HID_MSC"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::VALUE_1)
    }
    #[doc = "SPI Slave ISP"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::VALUE_2)
    }
    #[doc = "I2C Slave ISP"]
    #[inline(always)]
    pub fn value_3(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::VALUE_3)
    }
    #[doc = "Disable ISP fall through"]
    #[inline(always)]
    pub fn value_7(self) -> &'a mut W {
        self.variant(DEFAULT_ISP_MODE_A::VALUE_7)
    }
}
#[doc = "Field `BOOT_SPEED` reader - Core clock:"]
pub type BOOT_SPEED_R = crate::FieldReader<u8, BOOT_SPEED_A>;
#[doc = "Core clock:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOT_SPEED_A {
    #[doc = "0: Defined by NMPA.SYSTEM_SPEED_CODE"]
    VALUE_0 = 0,
    #[doc = "1: 96MHz FRO"]
    VALUE_1 = 1,
    #[doc = "2: 48MHz FRO"]
    VALUE_2 = 2,
}
impl From<BOOT_SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOT_SPEED_A) -> Self {
        variant as _
    }
}
impl BOOT_SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BOOT_SPEED_A> {
        match self.bits {
            0 => Some(BOOT_SPEED_A::VALUE_0),
            1 => Some(BOOT_SPEED_A::VALUE_1),
            2 => Some(BOOT_SPEED_A::VALUE_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE_0`"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == BOOT_SPEED_A::VALUE_0
    }
    #[doc = "Checks if the value of the field is `VALUE_1`"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == BOOT_SPEED_A::VALUE_1
    }
    #[doc = "Checks if the value of the field is `VALUE_2`"]
    #[inline(always)]
    pub fn is_value_2(&self) -> bool {
        *self == BOOT_SPEED_A::VALUE_2
    }
}
#[doc = "Field `BOOT_SPEED` writer - Core clock:"]
pub type BOOT_SPEED_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT_CFG_SPEC, u8, BOOT_SPEED_A, 2, O>;
impl<'a, const O: u8> BOOT_SPEED_W<'a, O> {
    #[doc = "Defined by NMPA.SYSTEM_SPEED_CODE"]
    #[inline(always)]
    pub fn value_0(self) -> &'a mut W {
        self.variant(BOOT_SPEED_A::VALUE_0)
    }
    #[doc = "96MHz FRO"]
    #[inline(always)]
    pub fn value_1(self) -> &'a mut W {
        self.variant(BOOT_SPEED_A::VALUE_1)
    }
    #[doc = "48MHz FRO"]
    #[inline(always)]
    pub fn value_2(self) -> &'a mut W {
        self.variant(BOOT_SPEED_A::VALUE_2)
    }
}
#[doc = "Field `BOOT_FAILURE_PIN` reader - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\]
- Defines GPIO port \\[7:3\\]
- Defines GPIO pin"]
pub type BOOT_FAILURE_PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOOT_FAILURE_PIN` writer - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\]
- Defines GPIO port \\[7:3\\]
- Defines GPIO pin"]
pub type BOOT_FAILURE_PIN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BOOT_CFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 4:6 - Default ISP mode:"]
    #[inline(always)]
    pub fn default_isp_mode(&self) -> DEFAULT_ISP_MODE_R {
        DEFAULT_ISP_MODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:8 - Core clock:"]
    #[inline(always)]
    pub fn boot_speed(&self) -> BOOT_SPEED_R {
        BOOT_SPEED_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 24:31 - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\]
- Defines GPIO port \\[7:3\\]
- Defines GPIO pin"]
    #[inline(always)]
    pub fn boot_failure_pin(&self) -> BOOT_FAILURE_PIN_R {
        BOOT_FAILURE_PIN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Default ISP mode:"]
    #[inline(always)]
    #[must_use]
    pub fn default_isp_mode(&mut self) -> DEFAULT_ISP_MODE_W<4> {
        DEFAULT_ISP_MODE_W::new(self)
    }
    #[doc = "Bits 7:8 - Core clock:"]
    #[inline(always)]
    #[must_use]
    pub fn boot_speed(&mut self) -> BOOT_SPEED_W<7> {
        BOOT_SPEED_W::new(self)
    }
    #[doc = "Bits 24:31 - GPIO port and pin number to use for indicating failure reason. The toggle rate of the pin is used to decode the error type. \\[2:0\\]
- Defines GPIO port \\[7:3\\]
- Defines GPIO pin"]
    #[inline(always)]
    #[must_use]
    pub fn boot_failure_pin(&mut self) -> BOOT_FAILURE_PIN_W<24> {
        BOOT_FAILURE_PIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = ".\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [boot_cfg](index.html) module"]
pub struct BOOT_CFG_SPEC;
impl crate::RegisterSpec for BOOT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [boot_cfg::R](R) reader structure"]
impl crate::Readable for BOOT_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [boot_cfg::W](W) writer structure"]
impl crate::Writable for BOOT_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOT_CFG to value 0"]
impl crate::Resettable for BOOT_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
