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
#[doc = "Field `MSTEN` reader - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
pub type MSTEN_R = crate::BitReader<MSTEN_A>;
#[doc = "Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTEN_A {
    #[doc = "0: Disabled. The I2C Master function is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The I2C Master function is enabled."]
    ENABLED = 1,
}
impl From<MSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTEN_A {
        match self.bits {
            false => MSTEN_A::DISABLED,
            true => MSTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTEN_A::ENABLED
    }
}
#[doc = "Field `MSTEN` writer - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
pub type MSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, MSTEN_A, O>;
impl<'a, const O: u8> MSTEN_W<'a, O> {
    #[doc = "Disabled. The I2C Master function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MSTEN_A::DISABLED)
    }
    #[doc = "Enabled. The I2C Master function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MSTEN_A::ENABLED)
    }
}
#[doc = "Field `SLVEN` reader - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
pub type SLVEN_R = crate::BitReader<SLVEN_A>;
#[doc = "Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVEN_A {
    #[doc = "0: Disabled. The I2C slave function is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The I2C slave function is enabled."]
    ENABLED = 1,
}
impl From<SLVEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLVEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVEN_A {
        match self.bits {
            false => SLVEN_A::DISABLED,
            true => SLVEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SLVEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SLVEN_A::ENABLED
    }
}
#[doc = "Field `SLVEN` writer - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
pub type SLVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, SLVEN_A, O>;
impl<'a, const O: u8> SLVEN_W<'a, O> {
    #[doc = "Disabled. The I2C slave function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SLVEN_A::DISABLED)
    }
    #[doc = "Enabled. The I2C slave function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SLVEN_A::ENABLED)
    }
}
#[doc = "Field `MONEN` reader - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
pub type MONEN_R = crate::BitReader<MONEN_A>;
#[doc = "Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONEN_A {
    #[doc = "0: Disabled. The I2C Monitor function is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. The I2C Monitor function is enabled."]
    ENABLED = 1,
}
impl From<MONEN_A> for bool {
    #[inline(always)]
    fn from(variant: MONEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MONEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONEN_A {
        match self.bits {
            false => MONEN_A::DISABLED,
            true => MONEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONEN_A::ENABLED
    }
}
#[doc = "Field `MONEN` writer - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
pub type MONEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, MONEN_A, O>;
impl<'a, const O: u8> MONEN_W<'a, O> {
    #[doc = "Disabled. The I2C Monitor function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONEN_A::DISABLED)
    }
    #[doc = "Enabled. The I2C Monitor function is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONEN_A::ENABLED)
    }
}
#[doc = "Field `TIMEOUTEN` reader - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
pub type TIMEOUTEN_R = crate::BitReader<TIMEOUTEN_A>;
#[doc = "I2C bus Time-out Enable. When disabled, the time-out function is internally reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUTEN_A {
    #[doc = "0: Disabled. Time-out function is disabled."]
    DISABLED = 0,
    #[doc = "1: Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one time-out will be used in a system."]
    ENABLED = 1,
}
impl From<TIMEOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMEOUTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUTEN_A {
        match self.bits {
            false => TIMEOUTEN_A::DISABLED,
            true => TIMEOUTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIMEOUTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIMEOUTEN_A::ENABLED
    }
}
#[doc = "Field `TIMEOUTEN` writer - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
pub type TIMEOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, TIMEOUTEN_A, O>;
impl<'a, const O: u8> TIMEOUTEN_W<'a, O> {
    #[doc = "Disabled. Time-out function is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIMEOUTEN_A::DISABLED)
    }
    #[doc = "Enabled. Time-out function is enabled. Both types of time-out flags will be generated and will cause interrupts if they are enabled. Typically, only one time-out will be used in a system."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIMEOUTEN_A::ENABLED)
    }
}
#[doc = "Field `MONCLKSTR` reader - Monitor function Clock Stretching."]
pub type MONCLKSTR_R = crate::BitReader<MONCLKSTR_A>;
#[doc = "Monitor function Clock Stretching.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONCLKSTR_A {
    #[doc = "0: Disabled. The Monitor function will not perform clock stretching. Software or DMA may not always be able to read data provided by the Monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    DISABLED = 0,
    #[doc = "1: Enabled. The Monitor function will perform clock stretching in order to ensure that software or DMA can read all incoming data supplied by the Monitor function."]
    ENABLED = 1,
}
impl From<MONCLKSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MONCLKSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl MONCLKSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONCLKSTR_A {
        match self.bits {
            false => MONCLKSTR_A::DISABLED,
            true => MONCLKSTR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MONCLKSTR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MONCLKSTR_A::ENABLED
    }
}
#[doc = "Field `MONCLKSTR` writer - Monitor function Clock Stretching."]
pub type MONCLKSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, MONCLKSTR_A, O>;
impl<'a, const O: u8> MONCLKSTR_W<'a, O> {
    #[doc = "Disabled. The Monitor function will not perform clock stretching. Software or DMA may not always be able to read data provided by the Monitor function before it is overwritten. This mode may be used when non-invasive monitoring is critical."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MONCLKSTR_A::DISABLED)
    }
    #[doc = "Enabled. The Monitor function will perform clock stretching in order to ensure that software or DMA can read all incoming data supplied by the Monitor function."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MONCLKSTR_A::ENABLED)
    }
}
#[doc = "Field `HSCAPABLE` reader - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
pub type HSCAPABLE_R = crate::BitReader<HSCAPABLE_A>;
#[doc = "High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSCAPABLE_A {
    #[doc = "0: Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,"]
    FAST_MODE_PLUS = 0,
    #[doc = "1: High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    HIGH_SPEED = 1,
}
impl From<HSCAPABLE_A> for bool {
    #[inline(always)]
    fn from(variant: HSCAPABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl HSCAPABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSCAPABLE_A {
        match self.bits {
            false => HSCAPABLE_A::FAST_MODE_PLUS,
            true => HSCAPABLE_A::HIGH_SPEED,
        }
    }
    #[doc = "Checks if the value of the field is `FAST_MODE_PLUS`"]
    #[inline(always)]
    pub fn is_fast_mode_plus(&self) -> bool {
        *self == HSCAPABLE_A::FAST_MODE_PLUS
    }
    #[doc = "Checks if the value of the field is `HIGH_SPEED`"]
    #[inline(always)]
    pub fn is_high_speed(&self) -> bool {
        *self == HSCAPABLE_A::HIGH_SPEED
    }
}
#[doc = "Field `HSCAPABLE` writer - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
pub type HSCAPABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, HSCAPABLE_A, O>;
impl<'a, const O: u8> HSCAPABLE_W<'a, O> {
    #[doc = "Fast-mode plus. The I 2C interface will support Standard-mode, Fast-mode, and Fast-mode Plus, to the extent that the pin electronics support these modes. Any changes that need to be made to the pin controls, such as changing the drive strength or filtering, must be made by software via the IOCON register associated with each I2C pin,"]
    #[inline(always)]
    pub fn fast_mode_plus(self) -> &'a mut W {
        self.variant(HSCAPABLE_A::FAST_MODE_PLUS)
    }
    #[doc = "High-speed. In addition to Standard-mode, Fast-mode, and Fast-mode Plus, the I 2C interface will support High-speed mode to the extent that the pin electronics support these modes. See Section 25.7.2.2 for more information."]
    #[inline(always)]
    pub fn high_speed(self) -> &'a mut W {
        self.variant(HSCAPABLE_A::HIGH_SPEED)
    }
}
impl R {
    #[doc = "Bit 0 - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline(always)]
    pub fn msten(&self) -> MSTEN_R {
        MSTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline(always)]
    pub fn monen(&self) -> MONEN_R {
        MONEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[inline(always)]
    pub fn timeouten(&self) -> TIMEOUTEN_R {
        TIMEOUTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Monitor function Clock Stretching."]
    #[inline(always)]
    pub fn monclkstr(&self) -> MONCLKSTR_R {
        MONCLKSTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[inline(always)]
    pub fn hscapable(&self) -> HSCAPABLE_R {
        HSCAPABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline(always)]
    #[must_use]
    pub fn msten(&mut self) -> MSTEN_W<0> {
        MSTEN_W::new(self)
    }
    #[doc = "Bit 1 - Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline(always)]
    #[must_use]
    pub fn slven(&mut self) -> SLVEN_W<1> {
        SLVEN_W::new(self)
    }
    #[doc = "Bit 2 - Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline(always)]
    #[must_use]
    pub fn monen(&mut self) -> MONEN_W<2> {
        MONEN_W::new(self)
    }
    #[doc = "Bit 3 - I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[inline(always)]
    #[must_use]
    pub fn timeouten(&mut self) -> TIMEOUTEN_W<3> {
        TIMEOUTEN_W::new(self)
    }
    #[doc = "Bit 4 - Monitor function Clock Stretching."]
    #[inline(always)]
    #[must_use]
    pub fn monclkstr(&mut self) -> MONCLKSTR_W<4> {
        MONCLKSTR_W::new(self)
    }
    #[doc = "Bit 5 - High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[inline(always)]
    #[must_use]
    pub fn hscapable(&mut self) -> HSCAPABLE_W<5> {
        HSCAPABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for shared functions.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
