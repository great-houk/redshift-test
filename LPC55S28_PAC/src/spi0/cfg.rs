#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - SPI enable."]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "SPI enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    DISABLED = 0,
    #[doc = "1: Enabled. The SPI is enabled for operation."]
    ENABLED = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Field `ENABLE` writer - SPI enable."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Disabled. The SPI is disabled and the internal state machine and counters are reset."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Enabled. The SPI is enabled for operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
}
#[doc = "Field `MASTER` reader - Master mode select."]
pub type MASTER_R = crate::BitReader<MASTER_A>;
#[doc = "Master mode select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER_A {
    #[doc = "0: Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    SLAVE_MODE = 0,
    #[doc = "1: Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    MASTER_MODE = 1,
}
impl From<MASTER_A> for bool {
    #[inline(always)]
    fn from(variant: MASTER_A) -> Self {
        variant as u8 != 0
    }
}
impl MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTER_A {
        match self.bits {
            false => MASTER_A::SLAVE_MODE,
            true => MASTER_A::MASTER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_MODE`"]
    #[inline(always)]
    pub fn is_slave_mode(&self) -> bool {
        *self == MASTER_A::SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `MASTER_MODE`"]
    #[inline(always)]
    pub fn is_master_mode(&self) -> bool {
        *self == MASTER_A::MASTER_MODE
    }
}
#[doc = "Field `MASTER` writer - Master mode select."]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, MASTER_A, O>;
impl<'a, const O: u8> MASTER_W<'a, O> {
    #[doc = "Slave mode. The SPI will operate in slave mode. SCK, MOSI, and the SSEL signals are inputs, MISO is an output."]
    #[inline(always)]
    pub fn slave_mode(self) -> &'a mut W {
        self.variant(MASTER_A::SLAVE_MODE)
    }
    #[doc = "Master mode. The SPI will operate in master mode. SCK, MOSI, and the SSEL signals are outputs, MISO is an input."]
    #[inline(always)]
    pub fn master_mode(self) -> &'a mut W {
        self.variant(MASTER_A::MASTER_MODE)
    }
}
#[doc = "Field `LSBF` reader - LSB First mode enable."]
pub type LSBF_R = crate::BitReader<LSBF_A>;
#[doc = "LSB First mode enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBF_A {
    #[doc = "0: Standard. Data is transmitted and received in standard MSB first order."]
    STANDARD = 0,
    #[doc = "1: Reverse. Data is transmitted and received in reverse order (LSB first)."]
    REVERSE = 1,
}
impl From<LSBF_A> for bool {
    #[inline(always)]
    fn from(variant: LSBF_A) -> Self {
        variant as u8 != 0
    }
}
impl LSBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBF_A {
        match self.bits {
            false => LSBF_A::STANDARD,
            true => LSBF_A::REVERSE,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == LSBF_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `REVERSE`"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == LSBF_A::REVERSE
    }
}
#[doc = "Field `LSBF` writer - LSB First mode enable."]
pub type LSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, LSBF_A, O>;
impl<'a, const O: u8> LSBF_W<'a, O> {
    #[doc = "Standard. Data is transmitted and received in standard MSB first order."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(LSBF_A::STANDARD)
    }
    #[doc = "Reverse. Data is transmitted and received in reverse order (LSB first)."]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut W {
        self.variant(LSBF_A::REVERSE)
    }
}
#[doc = "Field `CPHA` reader - Clock Phase select."]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "Clock Phase select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: Change. The SPI captures serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is changed on the following edge."]
    CHANGE = 0,
    #[doc = "1: Capture. The SPI changes serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is captured on the following edge."]
    CAPTURE = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::CHANGE,
            true => CPHA_A::CAPTURE,
        }
    }
    #[doc = "Checks if the value of the field is `CHANGE`"]
    #[inline(always)]
    pub fn is_change(&self) -> bool {
        *self == CPHA_A::CHANGE
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == CPHA_A::CAPTURE
    }
}
#[doc = "Field `CPHA` writer - Clock Phase select."]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CPHA_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    #[doc = "Change. The SPI captures serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is changed on the following edge."]
    #[inline(always)]
    pub fn change(self) -> &'a mut W {
        self.variant(CPHA_A::CHANGE)
    }
    #[doc = "Capture. The SPI changes serial data on the first clock transition of the transfer (when the clock changes away from the rest state). Data is captured on the following edge."]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(CPHA_A::CAPTURE)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity select."]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "Clock Polarity select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: Low. The rest state of the clock (between transfers) is low."]
    LOW = 0,
    #[doc = "1: High. The rest state of the clock (between transfers) is high."]
    HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::LOW,
            true => CPOL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CPOL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CPOL_A::HIGH
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity select."]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CPOL_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "Low. The rest state of the clock (between transfers) is low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CPOL_A::LOW)
    }
    #[doc = "High. The rest state of the clock (between transfers) is high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CPOL_A::HIGH)
    }
}
#[doc = "Field `LOOP` reader - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
pub type LOOP_R = crate::BitReader<LOOP_A>;
#[doc = "Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOP_A {
    #[doc = "0: Disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled."]
    ENABLED = 1,
}
impl From<LOOP_A> for bool {
    #[inline(always)]
    fn from(variant: LOOP_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOP_A {
        match self.bits {
            false => LOOP_A::DISABLED,
            true => LOOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LOOP_A::ENABLED
    }
}
#[doc = "Field `LOOP` writer - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
pub type LOOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, LOOP_A, O>;
impl<'a, const O: u8> LOOP_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOOP_A::DISABLED)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LOOP_A::ENABLED)
    }
}
#[doc = "Field `SPOL0` reader - SSEL0 Polarity select."]
pub type SPOL0_R = crate::BitReader<SPOL0_A>;
#[doc = "SSEL0 Polarity select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPOL0_A {
    #[doc = "0: Low. The SSEL0 pin is active low."]
    LOW = 0,
    #[doc = "1: High. The SSEL0 pin is active high."]
    HIGH = 1,
}
impl From<SPOL0_A> for bool {
    #[inline(always)]
    fn from(variant: SPOL0_A) -> Self {
        variant as u8 != 0
    }
}
impl SPOL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL0_A {
        match self.bits {
            false => SPOL0_A::LOW,
            true => SPOL0_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPOL0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPOL0_A::HIGH
    }
}
#[doc = "Field `SPOL0` writer - SSEL0 Polarity select."]
pub type SPOL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, SPOL0_A, O>;
impl<'a, const O: u8> SPOL0_W<'a, O> {
    #[doc = "Low. The SSEL0 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPOL0_A::LOW)
    }
    #[doc = "High. The SSEL0 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPOL0_A::HIGH)
    }
}
#[doc = "Field `SPOL1` reader - SSEL1 Polarity select."]
pub type SPOL1_R = crate::BitReader<SPOL1_A>;
#[doc = "SSEL1 Polarity select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPOL1_A {
    #[doc = "0: Low. The SSEL1 pin is active low."]
    LOW = 0,
    #[doc = "1: High. The SSEL1 pin is active high."]
    HIGH = 1,
}
impl From<SPOL1_A> for bool {
    #[inline(always)]
    fn from(variant: SPOL1_A) -> Self {
        variant as u8 != 0
    }
}
impl SPOL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL1_A {
        match self.bits {
            false => SPOL1_A::LOW,
            true => SPOL1_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPOL1_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPOL1_A::HIGH
    }
}
#[doc = "Field `SPOL1` writer - SSEL1 Polarity select."]
pub type SPOL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, SPOL1_A, O>;
impl<'a, const O: u8> SPOL1_W<'a, O> {
    #[doc = "Low. The SSEL1 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPOL1_A::LOW)
    }
    #[doc = "High. The SSEL1 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPOL1_A::HIGH)
    }
}
#[doc = "Field `SPOL2` reader - SSEL2 Polarity select."]
pub type SPOL2_R = crate::BitReader<SPOL2_A>;
#[doc = "SSEL2 Polarity select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPOL2_A {
    #[doc = "0: Low. The SSEL2 pin is active low."]
    LOW = 0,
    #[doc = "1: High. The SSEL2 pin is active high."]
    HIGH = 1,
}
impl From<SPOL2_A> for bool {
    #[inline(always)]
    fn from(variant: SPOL2_A) -> Self {
        variant as u8 != 0
    }
}
impl SPOL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL2_A {
        match self.bits {
            false => SPOL2_A::LOW,
            true => SPOL2_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPOL2_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPOL2_A::HIGH
    }
}
#[doc = "Field `SPOL2` writer - SSEL2 Polarity select."]
pub type SPOL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, SPOL2_A, O>;
impl<'a, const O: u8> SPOL2_W<'a, O> {
    #[doc = "Low. The SSEL2 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPOL2_A::LOW)
    }
    #[doc = "High. The SSEL2 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPOL2_A::HIGH)
    }
}
#[doc = "Field `SPOL3` reader - SSEL3 Polarity select."]
pub type SPOL3_R = crate::BitReader<SPOL3_A>;
#[doc = "SSEL3 Polarity select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPOL3_A {
    #[doc = "0: Low. The SSEL3 pin is active low."]
    LOW = 0,
    #[doc = "1: High. The SSEL3 pin is active high."]
    HIGH = 1,
}
impl From<SPOL3_A> for bool {
    #[inline(always)]
    fn from(variant: SPOL3_A) -> Self {
        variant as u8 != 0
    }
}
impl SPOL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL3_A {
        match self.bits {
            false => SPOL3_A::LOW,
            true => SPOL3_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPOL3_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPOL3_A::HIGH
    }
}
#[doc = "Field `SPOL3` writer - SSEL3 Polarity select."]
pub type SPOL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, SPOL3_A, O>;
impl<'a, const O: u8> SPOL3_W<'a, O> {
    #[doc = "Low. The SSEL3 pin is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(SPOL3_A::LOW)
    }
    #[doc = "High. The SSEL3 pin is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(SPOL3_A::HIGH)
    }
}
impl R {
    #[doc = "Bit 0 - SPI enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Master mode select."]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LSB First mode enable."]
    #[inline(always)]
    pub fn lsbf(&self) -> LSBF_R {
        LSBF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Phase select."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Polarity select."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SSEL0 Polarity select."]
    #[inline(always)]
    pub fn spol0(&self) -> SPOL0_R {
        SPOL0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SSEL1 Polarity select."]
    #[inline(always)]
    pub fn spol1(&self) -> SPOL1_R {
        SPOL1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SSEL2 Polarity select."]
    #[inline(always)]
    pub fn spol2(&self) -> SPOL2_R {
        SPOL2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SSEL3 Polarity select."]
    #[inline(always)]
    pub fn spol3(&self) -> SPOL3_R {
        SPOL3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI enable."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Master mode select."]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<2> {
        MASTER_W::new(self)
    }
    #[doc = "Bit 3 - LSB First mode enable."]
    #[inline(always)]
    #[must_use]
    pub fn lsbf(&mut self) -> LSBF_W<3> {
        LSBF_W::new(self)
    }
    #[doc = "Bit 4 - Clock Phase select."]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<4> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 5 - Clock Polarity select."]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<5> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 7 - Loopback mode enable. Loopback mode applies only to Master mode, and connects transmit and receive data connected together to allow simple software testing."]
    #[inline(always)]
    #[must_use]
    pub fn loop_(&mut self) -> LOOP_W<7> {
        LOOP_W::new(self)
    }
    #[doc = "Bit 8 - SSEL0 Polarity select."]
    #[inline(always)]
    #[must_use]
    pub fn spol0(&mut self) -> SPOL0_W<8> {
        SPOL0_W::new(self)
    }
    #[doc = "Bit 9 - SSEL1 Polarity select."]
    #[inline(always)]
    #[must_use]
    pub fn spol1(&mut self) -> SPOL1_W<9> {
        SPOL1_W::new(self)
    }
    #[doc = "Bit 10 - SSEL2 Polarity select."]
    #[inline(always)]
    #[must_use]
    pub fn spol2(&mut self) -> SPOL2_W<10> {
        SPOL2_W::new(self)
    }
    #[doc = "Bit 11 - SSEL3 Polarity select."]
    #[inline(always)]
    #[must_use]
    pub fn spol3(&mut self) -> SPOL3_W<11> {
        SPOL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
