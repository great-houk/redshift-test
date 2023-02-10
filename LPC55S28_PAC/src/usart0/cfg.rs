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
#[doc = "Field `ENABLE` reader - USART Enable."]
pub type ENABLE_R = crate::BitReader<ENABLE_A>;
#[doc = "USART Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE_A {
    #[doc = "0: Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. When re-enabled, the USART will immediately be ready to transmit because the transmitter has been reset and is therefore available."]
    DISABLED = 0,
    #[doc = "1: Enabled. The USART is enabled for operation."]
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
#[doc = "Field `ENABLE` writer - USART Enable."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, ENABLE_A, O>;
impl<'a, const O: u8> ENABLE_W<'a, O> {
    #[doc = "Disabled. The USART is disabled and the internal state machine and counters are reset. While Enable = 0, all USART interrupts and DMA transfers are disabled. When Enable is set again, CFG and most other control bits remain unchanged. When re-enabled, the USART will immediately be ready to transmit because the transmitter has been reset and is therefore available."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Enabled. The USART is enabled for operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
    }
}
#[doc = "Field `DATALEN` reader - Selects the data size for the USART."]
pub type DATALEN_R = crate::FieldReader<u8, DATALEN_A>;
#[doc = "Selects the data size for the USART.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATALEN_A {
    #[doc = "0: 7 bit Data length."]
    BIT_7 = 0,
    #[doc = "1: 8 bit Data length."]
    BIT_8 = 1,
    #[doc = "2: 9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    BIT_9 = 2,
}
impl From<DATALEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATALEN_A) -> Self {
        variant as _
    }
}
impl DATALEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATALEN_A> {
        match self.bits {
            0 => Some(DATALEN_A::BIT_7),
            1 => Some(DATALEN_A::BIT_8),
            2 => Some(DATALEN_A::BIT_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BIT_7`"]
    #[inline(always)]
    pub fn is_bit_7(&self) -> bool {
        *self == DATALEN_A::BIT_7
    }
    #[doc = "Checks if the value of the field is `BIT_8`"]
    #[inline(always)]
    pub fn is_bit_8(&self) -> bool {
        *self == DATALEN_A::BIT_8
    }
    #[doc = "Checks if the value of the field is `BIT_9`"]
    #[inline(always)]
    pub fn is_bit_9(&self) -> bool {
        *self == DATALEN_A::BIT_9
    }
}
#[doc = "Field `DATALEN` writer - Selects the data size for the USART."]
pub type DATALEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, DATALEN_A, 2, O>;
impl<'a, const O: u8> DATALEN_W<'a, O> {
    #[doc = "7 bit Data length."]
    #[inline(always)]
    pub fn bit_7(self) -> &'a mut W {
        self.variant(DATALEN_A::BIT_7)
    }
    #[doc = "8 bit Data length."]
    #[inline(always)]
    pub fn bit_8(self) -> &'a mut W {
        self.variant(DATALEN_A::BIT_8)
    }
    #[doc = "9 bit data length. The 9th bit is commonly used for addressing in multidrop mode. See the ADDRDET bit in the CTL register."]
    #[inline(always)]
    pub fn bit_9(self) -> &'a mut W {
        self.variant(DATALEN_A::BIT_9)
    }
}
#[doc = "Field `PARITYSEL` reader - Selects what type of parity is used by the USART."]
pub type PARITYSEL_R = crate::FieldReader<u8, PARITYSEL_A>;
#[doc = "Selects what type of parity is used by the USART.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARITYSEL_A {
    #[doc = "0: No parity."]
    NO_PARITY = 0,
    #[doc = "2: Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    EVEN_PARITY = 2,
    #[doc = "3: Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    ODD_PARITY = 3,
}
impl From<PARITYSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PARITYSEL_A) -> Self {
        variant as _
    }
}
impl PARITYSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PARITYSEL_A> {
        match self.bits {
            0 => Some(PARITYSEL_A::NO_PARITY),
            2 => Some(PARITYSEL_A::EVEN_PARITY),
            3 => Some(PARITYSEL_A::ODD_PARITY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PARITY`"]
    #[inline(always)]
    pub fn is_no_parity(&self) -> bool {
        *self == PARITYSEL_A::NO_PARITY
    }
    #[doc = "Checks if the value of the field is `EVEN_PARITY`"]
    #[inline(always)]
    pub fn is_even_parity(&self) -> bool {
        *self == PARITYSEL_A::EVEN_PARITY
    }
    #[doc = "Checks if the value of the field is `ODD_PARITY`"]
    #[inline(always)]
    pub fn is_odd_parity(&self) -> bool {
        *self == PARITYSEL_A::ODD_PARITY
    }
}
#[doc = "Field `PARITYSEL` writer - Selects what type of parity is used by the USART."]
pub type PARITYSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CFG_SPEC, u8, PARITYSEL_A, 2, O>;
impl<'a, const O: u8> PARITYSEL_W<'a, O> {
    #[doc = "No parity."]
    #[inline(always)]
    pub fn no_parity(self) -> &'a mut W {
        self.variant(PARITYSEL_A::NO_PARITY)
    }
    #[doc = "Even parity. Adds a bit to each character such that the number of 1s in a transmitted character is even, and the number of 1s in a received character is expected to be even."]
    #[inline(always)]
    pub fn even_parity(self) -> &'a mut W {
        self.variant(PARITYSEL_A::EVEN_PARITY)
    }
    #[doc = "Odd parity. Adds a bit to each character such that the number of 1s in a transmitted character is odd, and the number of 1s in a received character is expected to be odd."]
    #[inline(always)]
    pub fn odd_parity(self) -> &'a mut W {
        self.variant(PARITYSEL_A::ODD_PARITY)
    }
}
#[doc = "Field `STOPLEN` reader - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
pub type STOPLEN_R = crate::BitReader<STOPLEN_A>;
#[doc = "Number of stop bits appended to transmitted data. Only a single stop bit is required for received data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPLEN_A {
    #[doc = "0: 1 stop bit."]
    BIT_1 = 0,
    #[doc = "1: 2 stop bits. This setting should only be used for asynchronous communication."]
    BITS_2 = 1,
}
impl From<STOPLEN_A> for bool {
    #[inline(always)]
    fn from(variant: STOPLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPLEN_A {
        match self.bits {
            false => STOPLEN_A::BIT_1,
            true => STOPLEN_A::BITS_2,
        }
    }
    #[doc = "Checks if the value of the field is `BIT_1`"]
    #[inline(always)]
    pub fn is_bit_1(&self) -> bool {
        *self == STOPLEN_A::BIT_1
    }
    #[doc = "Checks if the value of the field is `BITS_2`"]
    #[inline(always)]
    pub fn is_bits_2(&self) -> bool {
        *self == STOPLEN_A::BITS_2
    }
}
#[doc = "Field `STOPLEN` writer - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
pub type STOPLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, STOPLEN_A, O>;
impl<'a, const O: u8> STOPLEN_W<'a, O> {
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn bit_1(self) -> &'a mut W {
        self.variant(STOPLEN_A::BIT_1)
    }
    #[doc = "2 stop bits. This setting should only be used for asynchronous communication."]
    #[inline(always)]
    pub fn bits_2(self) -> &'a mut W {
        self.variant(STOPLEN_A::BITS_2)
    }
}
#[doc = "Field `MODE32K` reader - Selects standard or 32 kHz clocking mode."]
pub type MODE32K_R = crate::BitReader<MODE32K_A>;
#[doc = "Selects standard or 32 kHz clocking mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE32K_A {
    #[doc = "0: Disabled. USART uses standard clocking."]
    DISABLED = 0,
    #[doc = "1: Enabled. USART uses the 32 kHz clock from the RTC oscillator as the clock source to the BRG, and uses a special bit clocking scheme."]
    ENABLED = 1,
}
impl From<MODE32K_A> for bool {
    #[inline(always)]
    fn from(variant: MODE32K_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE32K_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE32K_A {
        match self.bits {
            false => MODE32K_A::DISABLED,
            true => MODE32K_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE32K_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MODE32K_A::ENABLED
    }
}
#[doc = "Field `MODE32K` writer - Selects standard or 32 kHz clocking mode."]
pub type MODE32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, MODE32K_A, O>;
impl<'a, const O: u8> MODE32K_W<'a, O> {
    #[doc = "Disabled. USART uses standard clocking."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE32K_A::DISABLED)
    }
    #[doc = "Enabled. USART uses the 32 kHz clock from the RTC oscillator as the clock source to the BRG, and uses a special bit clocking scheme."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MODE32K_A::ENABLED)
    }
}
#[doc = "Field `LINMODE` reader - LIN break mode enable."]
pub type LINMODE_R = crate::BitReader<LINMODE_A>;
#[doc = "LIN break mode enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINMODE_A {
    #[doc = "0: Disabled. Break detect and generate is configured for normal operation."]
    DISABLED = 0,
    #[doc = "1: Enabled. Break detect and generate is configured for LIN bus operation."]
    ENABLED = 1,
}
impl From<LINMODE_A> for bool {
    #[inline(always)]
    fn from(variant: LINMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl LINMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINMODE_A {
        match self.bits {
            false => LINMODE_A::DISABLED,
            true => LINMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LINMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LINMODE_A::ENABLED
    }
}
#[doc = "Field `LINMODE` writer - LIN break mode enable."]
pub type LINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, LINMODE_A, O>;
impl<'a, const O: u8> LINMODE_W<'a, O> {
    #[doc = "Disabled. Break detect and generate is configured for normal operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LINMODE_A::DISABLED)
    }
    #[doc = "Enabled. Break detect and generate is configured for LIN bus operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LINMODE_A::ENABLED)
    }
}
#[doc = "Field `CTSEN` reader - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
pub type CTSEN_R = crate::BitReader<CTSEN_A>;
#[doc = "CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSEN_A {
    #[doc = "0: No flow control. The transmitter does not receive any automatic flow control signal."]
    DISABLED = 0,
    #[doc = "1: Flow control enabled. The transmitter uses the CTS input (or RTS output in loopback mode) for flow control purposes."]
    ENABLED = 1,
}
impl From<CTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSEN_A {
        match self.bits {
            false => CTSEN_A::DISABLED,
            true => CTSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSEN_A::ENABLED
    }
}
#[doc = "Field `CTSEN` writer - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
pub type CTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CTSEN_A, O>;
impl<'a, const O: u8> CTSEN_W<'a, O> {
    #[doc = "No flow control. The transmitter does not receive any automatic flow control signal."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSEN_A::DISABLED)
    }
    #[doc = "Flow control enabled. The transmitter uses the CTS input (or RTS output in loopback mode) for flow control purposes."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSEN_A::ENABLED)
    }
}
#[doc = "Field `SYNCEN` reader - Selects synchronous or asynchronous operation."]
pub type SYNCEN_R = crate::BitReader<SYNCEN_A>;
#[doc = "Selects synchronous or asynchronous operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCEN_A {
    #[doc = "0: Asynchronous mode."]
    ASYNCHRONOUS_MODE = 0,
    #[doc = "1: Synchronous mode."]
    SYNCHRONOUS_MODE = 1,
}
impl From<SYNCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCEN_A {
        match self.bits {
            false => SYNCEN_A::ASYNCHRONOUS_MODE,
            true => SYNCEN_A::SYNCHRONOUS_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_asynchronous_mode(&self) -> bool {
        *self == SYNCEN_A::ASYNCHRONOUS_MODE
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS_MODE`"]
    #[inline(always)]
    pub fn is_synchronous_mode(&self) -> bool {
        *self == SYNCEN_A::SYNCHRONOUS_MODE
    }
}
#[doc = "Field `SYNCEN` writer - Selects synchronous or asynchronous operation."]
pub type SYNCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, SYNCEN_A, O>;
impl<'a, const O: u8> SYNCEN_W<'a, O> {
    #[doc = "Asynchronous mode."]
    #[inline(always)]
    pub fn asynchronous_mode(self) -> &'a mut W {
        self.variant(SYNCEN_A::ASYNCHRONOUS_MODE)
    }
    #[doc = "Synchronous mode."]
    #[inline(always)]
    pub fn synchronous_mode(self) -> &'a mut W {
        self.variant(SYNCEN_A::SYNCHRONOUS_MODE)
    }
}
#[doc = "Field `CLKPOL` reader - Selects the clock polarity and sampling edge of received data in synchronous mode."]
pub type CLKPOL_R = crate::BitReader<CLKPOL_A>;
#[doc = "Selects the clock polarity and sampling edge of received data in synchronous mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKPOL_A {
    #[doc = "0: Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    FALLING_EDGE = 0,
    #[doc = "1: Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    RISING_EDGE = 1,
}
impl From<CLKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CLKPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKPOL_A {
        match self.bits {
            false => CLKPOL_A::FALLING_EDGE,
            true => CLKPOL_A::RISING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CLKPOL_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CLKPOL_A::RISING_EDGE
    }
}
#[doc = "Field `CLKPOL` writer - Selects the clock polarity and sampling edge of received data in synchronous mode."]
pub type CLKPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, CLKPOL_A, O>;
impl<'a, const O: u8> CLKPOL_W<'a, O> {
    #[doc = "Falling edge. Un_RXD is sampled on the falling edge of SCLK."]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CLKPOL_A::FALLING_EDGE)
    }
    #[doc = "Rising edge. Un_RXD is sampled on the rising edge of SCLK."]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CLKPOL_A::RISING_EDGE)
    }
}
#[doc = "Field `SYNCMST` reader - Synchronous mode Master select."]
pub type SYNCMST_R = crate::BitReader<SYNCMST_A>;
#[doc = "Synchronous mode Master select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCMST_A {
    #[doc = "0: Slave. When synchronous mode is enabled, the USART is a slave."]
    SLAVE = 0,
    #[doc = "1: Master. When synchronous mode is enabled, the USART is a master."]
    MASTER = 1,
}
impl From<SYNCMST_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCMST_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCMST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCMST_A {
        match self.bits {
            false => SYNCMST_A::SLAVE,
            true => SYNCMST_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == SYNCMST_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == SYNCMST_A::MASTER
    }
}
#[doc = "Field `SYNCMST` writer - Synchronous mode Master select."]
pub type SYNCMST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, SYNCMST_A, O>;
impl<'a, const O: u8> SYNCMST_W<'a, O> {
    #[doc = "Slave. When synchronous mode is enabled, the USART is a slave."]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(SYNCMST_A::SLAVE)
    }
    #[doc = "Master. When synchronous mode is enabled, the USART is a master."]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(SYNCMST_A::MASTER)
    }
}
#[doc = "Field `LOOP` reader - Selects data loopback mode."]
pub type LOOP_R = crate::BitReader<LOOP_A>;
#[doc = "Selects data loopback mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOP_A {
    #[doc = "0: Normal operation."]
    NORMAL = 0,
    #[doc = "1: Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    LOOPBACK = 1,
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
            false => LOOP_A::NORMAL,
            true => LOOP_A::LOOPBACK,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == LOOP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOOPBACK`"]
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        *self == LOOP_A::LOOPBACK
    }
}
#[doc = "Field `LOOP` writer - Selects data loopback mode."]
pub type LOOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, LOOP_A, O>;
impl<'a, const O: u8> LOOP_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(LOOP_A::NORMAL)
    }
    #[doc = "Loopback mode. This provides a mechanism to perform diagnostic loopback testing for USART data. Serial data from the transmitter (Un_TXD) is connected internally to serial input of the receive (Un_RXD). Un_TXD and Un_RTS activity will also appear on external pins if these functions are configured to appear on device pins. The receiver RTS signal is also looped back to CTS and performs flow control if enabled by CTSEN."]
    #[inline(always)]
    pub fn loopback(self) -> &'a mut W {
        self.variant(LOOP_A::LOOPBACK)
    }
}
#[doc = "Field `OETA` reader - Output Enable Turnaround time enable for RS-485 operation."]
pub type OETA_R = crate::BitReader<OETA_A>;
#[doc = "Output Enable Turnaround time enable for RS-485 operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OETA_A {
    #[doc = "0: Disabled. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission."]
    DISABLED = 0,
    #[doc = "1: Enabled. If selected by OESEL, the Output Enable signal remains asserted for one character time after the end of the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted."]
    ENABLED = 1,
}
impl From<OETA_A> for bool {
    #[inline(always)]
    fn from(variant: OETA_A) -> Self {
        variant as u8 != 0
    }
}
impl OETA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OETA_A {
        match self.bits {
            false => OETA_A::DISABLED,
            true => OETA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OETA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OETA_A::ENABLED
    }
}
#[doc = "Field `OETA` writer - Output Enable Turnaround time enable for RS-485 operation."]
pub type OETA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, OETA_A, O>;
impl<'a, const O: u8> OETA_W<'a, O> {
    #[doc = "Disabled. If selected by OESEL, the Output Enable signal deasserted at the end of the last stop bit of a transmission."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OETA_A::DISABLED)
    }
    #[doc = "Enabled. If selected by OESEL, the Output Enable signal remains asserted for one character time after the end of the last stop bit of a transmission. OE will also remain asserted if another transmit begins before it is deasserted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OETA_A::ENABLED)
    }
}
#[doc = "Field `AUTOADDR` reader - Automatic Address matching enable."]
pub type AUTOADDR_R = crate::BitReader<AUTOADDR_A>;
#[doc = "Automatic Address matching enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOADDR_A {
    #[doc = "0: Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address)."]
    DISABLED = 0,
    #[doc = "1: Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match."]
    ENABLED = 1,
}
impl From<AUTOADDR_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOADDR_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOADDR_A {
        match self.bits {
            false => AUTOADDR_A::DISABLED,
            true => AUTOADDR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AUTOADDR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AUTOADDR_A::ENABLED
    }
}
#[doc = "Field `AUTOADDR` writer - Automatic Address matching enable."]
pub type AUTOADDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, AUTOADDR_A, O>;
impl<'a, const O: u8> AUTOADDR_W<'a, O> {
    #[doc = "Disabled. When addressing is enabled by ADDRDET, address matching is done by software. This provides the possibility of versatile addressing (e.g. respond to more than one address)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AUTOADDR_A::DISABLED)
    }
    #[doc = "Enabled. When addressing is enabled by ADDRDET, address matching is done by hardware, using the value in the ADDR register as the address to match."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AUTOADDR_A::ENABLED)
    }
}
#[doc = "Field `OESEL` reader - Output Enable Select."]
pub type OESEL_R = crate::BitReader<OESEL_A>;
#[doc = "Output Enable Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OESEL_A {
    #[doc = "0: Standard. The RTS signal is used as the standard flow control function."]
    STANDARD = 0,
    #[doc = "1: RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver."]
    RS_485 = 1,
}
impl From<OESEL_A> for bool {
    #[inline(always)]
    fn from(variant: OESEL_A) -> Self {
        variant as u8 != 0
    }
}
impl OESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OESEL_A {
        match self.bits {
            false => OESEL_A::STANDARD,
            true => OESEL_A::RS_485,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == OESEL_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `RS_485`"]
    #[inline(always)]
    pub fn is_rs_485(&self) -> bool {
        *self == OESEL_A::RS_485
    }
}
#[doc = "Field `OESEL` writer - Output Enable Select."]
pub type OESEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, OESEL_A, O>;
impl<'a, const O: u8> OESEL_W<'a, O> {
    #[doc = "Standard. The RTS signal is used as the standard flow control function."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(OESEL_A::STANDARD)
    }
    #[doc = "RS-485. The RTS signal configured to provide an output enable signal to control an RS-485 transceiver."]
    #[inline(always)]
    pub fn rs_485(self) -> &'a mut W {
        self.variant(OESEL_A::RS_485)
    }
}
#[doc = "Field `OEPOL` reader - Output Enable Polarity."]
pub type OEPOL_R = crate::BitReader<OEPOL_A>;
#[doc = "Output Enable Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OEPOL_A {
    #[doc = "0: Low. If selected by OESEL, the output enable is active low."]
    LOW = 0,
    #[doc = "1: High. If selected by OESEL, the output enable is active high."]
    HIGH = 1,
}
impl From<OEPOL_A> for bool {
    #[inline(always)]
    fn from(variant: OEPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl OEPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OEPOL_A {
        match self.bits {
            false => OEPOL_A::LOW,
            true => OEPOL_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OEPOL_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OEPOL_A::HIGH
    }
}
#[doc = "Field `OEPOL` writer - Output Enable Polarity."]
pub type OEPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, OEPOL_A, O>;
impl<'a, const O: u8> OEPOL_W<'a, O> {
    #[doc = "Low. If selected by OESEL, the output enable is active low."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OEPOL_A::LOW)
    }
    #[doc = "High. If selected by OESEL, the output enable is active high."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OEPOL_A::HIGH)
    }
}
#[doc = "Field `RXPOL` reader - Receive data polarity."]
pub type RXPOL_R = crate::BitReader<RXPOL_A>;
#[doc = "Receive data polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXPOL_A {
    #[doc = "0: Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    STANDARD = 0,
    #[doc = "1: Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED = 1,
}
impl From<RXPOL_A> for bool {
    #[inline(always)]
    fn from(variant: RXPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl RXPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPOL_A {
        match self.bits {
            false => RXPOL_A::STANDARD,
            true => RXPOL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == RXPOL_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == RXPOL_A::INVERTED
    }
}
#[doc = "Field `RXPOL` writer - Receive data polarity."]
pub type RXPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, RXPOL_A, O>;
impl<'a, const O: u8> RXPOL_W<'a, O> {
    #[doc = "Standard. The RX signal is used as it arrives from the pin. This means that the RX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(RXPOL_A::STANDARD)
    }
    #[doc = "Inverted. The RX signal is inverted before being used by the USART. This means that the RX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(RXPOL_A::INVERTED)
    }
}
#[doc = "Field `TXPOL` reader - Transmit data polarity."]
pub type TXPOL_R = crate::BitReader<TXPOL_A>;
#[doc = "Transmit data polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXPOL_A {
    #[doc = "0: Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    STANDARD = 0,
    #[doc = "1: Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    INVERTED = 1,
}
impl From<TXPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TXPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl TXPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXPOL_A {
        match self.bits {
            false => TXPOL_A::STANDARD,
            true => TXPOL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == TXPOL_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TXPOL_A::INVERTED
    }
}
#[doc = "Field `TXPOL` writer - Transmit data polarity."]
pub type TXPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, TXPOL_A, O>;
impl<'a, const O: u8> TXPOL_W<'a, O> {
    #[doc = "Standard. The TX signal is sent out without change. This means that the TX rest value is 1, start bit is 0, data is not inverted, and the stop bit is 1."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(TXPOL_A::STANDARD)
    }
    #[doc = "Inverted. The TX signal is inverted by the USART before being sent out. This means that the TX rest value is 0, start bit is 1, data is inverted, and the stop bit is 0."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TXPOL_A::INVERTED)
    }
}
impl R {
    #[doc = "Bit 0 - USART Enable."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Selects the data size for the USART."]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Selects what type of parity is used by the USART."]
    #[inline(always)]
    pub fn paritysel(&self) -> PARITYSEL_R {
        PARITYSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline(always)]
    pub fn stoplen(&self) -> STOPLEN_R {
        STOPLEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects standard or 32 kHz clocking mode."]
    #[inline(always)]
    pub fn mode32k(&self) -> MODE32K_R {
        MODE32K_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LIN break mode enable."]
    #[inline(always)]
    pub fn linmode(&self) -> LINMODE_R {
        LINMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects synchronous or asynchronous operation."]
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline(always)]
    pub fn clkpol(&self) -> CLKPOL_R {
        CLKPOL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Synchronous mode Master select."]
    #[inline(always)]
    pub fn syncmst(&self) -> SYNCMST_R {
        SYNCMST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects data loopback mode."]
    #[inline(always)]
    pub fn loop_(&self) -> LOOP_R {
        LOOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Output Enable Turnaround time enable for RS-485 operation."]
    #[inline(always)]
    pub fn oeta(&self) -> OETA_R {
        OETA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Automatic Address matching enable."]
    #[inline(always)]
    pub fn autoaddr(&self) -> AUTOADDR_R {
        AUTOADDR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output Enable Select."]
    #[inline(always)]
    pub fn oesel(&self) -> OESEL_R {
        OESEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Output Enable Polarity."]
    #[inline(always)]
    pub fn oepol(&self) -> OEPOL_R {
        OEPOL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive data polarity."]
    #[inline(always)]
    pub fn rxpol(&self) -> RXPOL_R {
        RXPOL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit data polarity."]
    #[inline(always)]
    pub fn txpol(&self) -> TXPOL_R {
        TXPOL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART Enable."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 2:3 - Selects the data size for the USART."]
    #[inline(always)]
    #[must_use]
    pub fn datalen(&mut self) -> DATALEN_W<2> {
        DATALEN_W::new(self)
    }
    #[doc = "Bits 4:5 - Selects what type of parity is used by the USART."]
    #[inline(always)]
    #[must_use]
    pub fn paritysel(&mut self) -> PARITYSEL_W<4> {
        PARITYSEL_W::new(self)
    }
    #[doc = "Bit 6 - Number of stop bits appended to transmitted data. Only a single stop bit is required for received data."]
    #[inline(always)]
    #[must_use]
    pub fn stoplen(&mut self) -> STOPLEN_W<6> {
        STOPLEN_W::new(self)
    }
    #[doc = "Bit 7 - Selects standard or 32 kHz clocking mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode32k(&mut self) -> MODE32K_W<7> {
        MODE32K_W::new(self)
    }
    #[doc = "Bit 8 - LIN break mode enable."]
    #[inline(always)]
    #[must_use]
    pub fn linmode(&mut self) -> LINMODE_W<8> {
        LINMODE_W::new(self)
    }
    #[doc = "Bit 9 - CTS Enable. Determines whether CTS is used for flow control. CTS can be from the input pin, or from the USART's own RTS if loopback mode is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CTSEN_W<9> {
        CTSEN_W::new(self)
    }
    #[doc = "Bit 11 - Selects synchronous or asynchronous operation."]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<11> {
        SYNCEN_W::new(self)
    }
    #[doc = "Bit 12 - Selects the clock polarity and sampling edge of received data in synchronous mode."]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> CLKPOL_W<12> {
        CLKPOL_W::new(self)
    }
    #[doc = "Bit 14 - Synchronous mode Master select."]
    #[inline(always)]
    #[must_use]
    pub fn syncmst(&mut self) -> SYNCMST_W<14> {
        SYNCMST_W::new(self)
    }
    #[doc = "Bit 15 - Selects data loopback mode."]
    #[inline(always)]
    #[must_use]
    pub fn loop_(&mut self) -> LOOP_W<15> {
        LOOP_W::new(self)
    }
    #[doc = "Bit 18 - Output Enable Turnaround time enable for RS-485 operation."]
    #[inline(always)]
    #[must_use]
    pub fn oeta(&mut self) -> OETA_W<18> {
        OETA_W::new(self)
    }
    #[doc = "Bit 19 - Automatic Address matching enable."]
    #[inline(always)]
    #[must_use]
    pub fn autoaddr(&mut self) -> AUTOADDR_W<19> {
        AUTOADDR_W::new(self)
    }
    #[doc = "Bit 20 - Output Enable Select."]
    #[inline(always)]
    #[must_use]
    pub fn oesel(&mut self) -> OESEL_W<20> {
        OESEL_W::new(self)
    }
    #[doc = "Bit 21 - Output Enable Polarity."]
    #[inline(always)]
    #[must_use]
    pub fn oepol(&mut self) -> OEPOL_W<21> {
        OEPOL_W::new(self)
    }
    #[doc = "Bit 22 - Receive data polarity."]
    #[inline(always)]
    #[must_use]
    pub fn rxpol(&mut self) -> RXPOL_W<22> {
        RXPOL_W::new(self)
    }
    #[doc = "Bit 23 - Transmit data polarity."]
    #[inline(always)]
    #[must_use]
    pub fn txpol(&mut self) -> TXPOL_W<23> {
        TXPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Configuration register. Basic USART configuration settings that typically are not changed during operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
