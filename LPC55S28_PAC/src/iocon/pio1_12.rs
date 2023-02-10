#[doc = "Register `PIO1_12` reader"]
pub struct R(crate::R<PIO1_12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO1_12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO1_12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO1_12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIO1_12` writer"]
pub struct W(crate::W<PIO1_12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO1_12_SPEC>;
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
impl From<crate::W<PIO1_12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO1_12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FUNC` reader - Selects pin function."]
pub type FUNC_R = crate::FieldReader<u8, FUNC_A>;
#[doc = "Selects pin function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FUNC_A {
    #[doc = "0: Alternative connection 0."]
    ALT0 = 0,
    #[doc = "1: Alternative connection 1."]
    ALT1 = 1,
    #[doc = "2: Alternative connection 2."]
    ALT2 = 2,
    #[doc = "3: Alternative connection 3."]
    ALT3 = 3,
    #[doc = "4: Alternative connection 4."]
    ALT4 = 4,
    #[doc = "5: Alternative connection 5."]
    ALT5 = 5,
    #[doc = "6: Alternative connection 6."]
    ALT6 = 6,
    #[doc = "7: Alternative connection 7."]
    ALT7 = 7,
}
impl From<FUNC_A> for u8 {
    #[inline(always)]
    fn from(variant: FUNC_A) -> Self {
        variant as _
    }
}
impl FUNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FUNC_A> {
        match self.bits {
            0 => Some(FUNC_A::ALT0),
            1 => Some(FUNC_A::ALT1),
            2 => Some(FUNC_A::ALT2),
            3 => Some(FUNC_A::ALT3),
            4 => Some(FUNC_A::ALT4),
            5 => Some(FUNC_A::ALT5),
            6 => Some(FUNC_A::ALT6),
            7 => Some(FUNC_A::ALT7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALT0`"]
    #[inline(always)]
    pub fn is_alt0(&self) -> bool {
        *self == FUNC_A::ALT0
    }
    #[doc = "Checks if the value of the field is `ALT1`"]
    #[inline(always)]
    pub fn is_alt1(&self) -> bool {
        *self == FUNC_A::ALT1
    }
    #[doc = "Checks if the value of the field is `ALT2`"]
    #[inline(always)]
    pub fn is_alt2(&self) -> bool {
        *self == FUNC_A::ALT2
    }
    #[doc = "Checks if the value of the field is `ALT3`"]
    #[inline(always)]
    pub fn is_alt3(&self) -> bool {
        *self == FUNC_A::ALT3
    }
    #[doc = "Checks if the value of the field is `ALT4`"]
    #[inline(always)]
    pub fn is_alt4(&self) -> bool {
        *self == FUNC_A::ALT4
    }
    #[doc = "Checks if the value of the field is `ALT5`"]
    #[inline(always)]
    pub fn is_alt5(&self) -> bool {
        *self == FUNC_A::ALT5
    }
    #[doc = "Checks if the value of the field is `ALT6`"]
    #[inline(always)]
    pub fn is_alt6(&self) -> bool {
        *self == FUNC_A::ALT6
    }
    #[doc = "Checks if the value of the field is `ALT7`"]
    #[inline(always)]
    pub fn is_alt7(&self) -> bool {
        *self == FUNC_A::ALT7
    }
}
#[doc = "Field `FUNC` writer - Selects pin function."]
pub type FUNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIO1_12_SPEC, u8, FUNC_A, 4, O>;
impl<'a, const O: u8> FUNC_W<'a, O> {
    #[doc = "Alternative connection 0."]
    #[inline(always)]
    pub fn alt0(self) -> &'a mut W {
        self.variant(FUNC_A::ALT0)
    }
    #[doc = "Alternative connection 1."]
    #[inline(always)]
    pub fn alt1(self) -> &'a mut W {
        self.variant(FUNC_A::ALT1)
    }
    #[doc = "Alternative connection 2."]
    #[inline(always)]
    pub fn alt2(self) -> &'a mut W {
        self.variant(FUNC_A::ALT2)
    }
    #[doc = "Alternative connection 3."]
    #[inline(always)]
    pub fn alt3(self) -> &'a mut W {
        self.variant(FUNC_A::ALT3)
    }
    #[doc = "Alternative connection 4."]
    #[inline(always)]
    pub fn alt4(self) -> &'a mut W {
        self.variant(FUNC_A::ALT4)
    }
    #[doc = "Alternative connection 5."]
    #[inline(always)]
    pub fn alt5(self) -> &'a mut W {
        self.variant(FUNC_A::ALT5)
    }
    #[doc = "Alternative connection 6."]
    #[inline(always)]
    pub fn alt6(self) -> &'a mut W {
        self.variant(FUNC_A::ALT6)
    }
    #[doc = "Alternative connection 7."]
    #[inline(always)]
    pub fn alt7(self) -> &'a mut W {
        self.variant(FUNC_A::ALT7)
    }
}
#[doc = "Field `MODE` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    INACTIVE = 0,
    #[doc = "1: Pull-down. Pull-down resistor enabled."]
    PULL_DOWN = 1,
    #[doc = "2: Pull-up. Pull-up resistor enabled."]
    PULL_UP = 2,
    #[doc = "3: Repeater. Repeater mode."]
    REPEATER = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::INACTIVE,
            1 => MODE_A::PULL_DOWN,
            2 => MODE_A::PULL_UP,
            3 => MODE_A::REPEATER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == MODE_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `PULL_DOWN`"]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == MODE_A::PULL_DOWN
    }
    #[doc = "Checks if the value of the field is `PULL_UP`"]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == MODE_A::PULL_UP
    }
    #[doc = "Checks if the value of the field is `REPEATER`"]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == MODE_A::REPEATER
    }
}
#[doc = "Field `MODE` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PIO1_12_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Inactive. Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(MODE_A::INACTIVE)
    }
    #[doc = "Pull-down. Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut W {
        self.variant(MODE_A::PULL_DOWN)
    }
    #[doc = "Pull-up. Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut W {
        self.variant(MODE_A::PULL_UP)
    }
    #[doc = "Repeater. Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut W {
        self.variant(MODE_A::REPEATER)
    }
}
#[doc = "Field `SLEW` reader - Driver slew rate."]
pub type SLEW_R = crate::BitReader<SLEW_A>;
#[doc = "Driver slew rate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEW_A {
    #[doc = "0: Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    STANDARD = 0,
    #[doc = "1: Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    FAST = 1,
}
impl From<SLEW_A> for bool {
    #[inline(always)]
    fn from(variant: SLEW_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEW_A {
        match self.bits {
            false => SLEW_A::STANDARD,
            true => SLEW_A::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `STANDARD`"]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == SLEW_A::STANDARD
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == SLEW_A::FAST
    }
}
#[doc = "Field `SLEW` writer - Driver slew rate."]
pub type SLEW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO1_12_SPEC, SLEW_A, O>;
impl<'a, const O: u8> SLEW_W<'a, O> {
    #[doc = "Standard-mode, output slew rate is slower. More outputs can be switched simultaneously."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut W {
        self.variant(SLEW_A::STANDARD)
    }
    #[doc = "Fast-mode, output slew rate is faster. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(SLEW_A::FAST)
    }
}
#[doc = "Field `INVERT` reader - Input polarity."]
pub type INVERT_R = crate::BitReader<INVERT_A>;
#[doc = "Input polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVERT_A {
    #[doc = "0: Disabled. Input function is not inverted."]
    DISABLED = 0,
    #[doc = "1: Enabled. Input is function inverted."]
    ENABLED = 1,
}
impl From<INVERT_A> for bool {
    #[inline(always)]
    fn from(variant: INVERT_A) -> Self {
        variant as u8 != 0
    }
}
impl INVERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVERT_A {
        match self.bits {
            false => INVERT_A::DISABLED,
            true => INVERT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INVERT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INVERT_A::ENABLED
    }
}
#[doc = "Field `INVERT` writer - Input polarity."]
pub type INVERT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO1_12_SPEC, INVERT_A, O>;
impl<'a, const O: u8> INVERT_W<'a, O> {
    #[doc = "Disabled. Input function is not inverted."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(INVERT_A::DISABLED)
    }
    #[doc = "Enabled. Input is function inverted."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(INVERT_A::ENABLED)
    }
}
#[doc = "Field `DIGIMODE` reader - Select Digital mode."]
pub type DIGIMODE_R = crate::BitReader<DIGIMODE_A>;
#[doc = "Select Digital mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIGIMODE_A {
    #[doc = "0: Disable digital mode. Digital input set to 0."]
    ANALOG = 0,
    #[doc = "1: Enable Digital mode. Digital input is enabled."]
    DIGITAL = 1,
}
impl From<DIGIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DIGIMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl DIGIMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIGIMODE_A {
        match self.bits {
            false => DIGIMODE_A::ANALOG,
            true => DIGIMODE_A::DIGITAL,
        }
    }
    #[doc = "Checks if the value of the field is `ANALOG`"]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == DIGIMODE_A::ANALOG
    }
    #[doc = "Checks if the value of the field is `DIGITAL`"]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == DIGIMODE_A::DIGITAL
    }
}
#[doc = "Field `DIGIMODE` writer - Select Digital mode."]
pub type DIGIMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO1_12_SPEC, DIGIMODE_A, O>;
impl<'a, const O: u8> DIGIMODE_W<'a, O> {
    #[doc = "Disable digital mode. Digital input set to 0."]
    #[inline(always)]
    pub fn analog(self) -> &'a mut W {
        self.variant(DIGIMODE_A::ANALOG)
    }
    #[doc = "Enable Digital mode. Digital input is enabled."]
    #[inline(always)]
    pub fn digital(self) -> &'a mut W {
        self.variant(DIGIMODE_A::DIGITAL)
    }
}
#[doc = "Field `OD` reader - Controls open-drain mode."]
pub type OD_R = crate::BitReader<OD_A>;
#[doc = "Controls open-drain mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OD_A {
    #[doc = "0: Normal. Normal push-pull output"]
    NORMAL = 0,
    #[doc = "1: Open-drain. Simulated open-drain output (high drive disabled)."]
    OPEN_DRAIN = 1,
}
impl From<OD_A> for bool {
    #[inline(always)]
    fn from(variant: OD_A) -> Self {
        variant as u8 != 0
    }
}
impl OD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OD_A {
        match self.bits {
            false => OD_A::NORMAL,
            true => OD_A::OPEN_DRAIN,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == OD_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OD_A::OPEN_DRAIN
    }
}
#[doc = "Field `OD` writer - Controls open-drain mode."]
pub type OD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIO1_12_SPEC, OD_A, O>;
impl<'a, const O: u8> OD_W<'a, O> {
    #[doc = "Normal. Normal push-pull output"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(OD_A::NORMAL)
    }
    #[doc = "Open-drain. Simulated open-drain output (high drive disabled)."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(OD_A::OPEN_DRAIN)
    }
}
impl R {
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline(always)]
    pub fn func(&self) -> FUNC_R {
        FUNC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Driver slew rate."]
    #[inline(always)]
    pub fn slew(&self) -> SLEW_R {
        SLEW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline(always)]
    pub fn invert(&self) -> INVERT_R {
        INVERT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Select Digital mode."]
    #[inline(always)]
    pub fn digimode(&self) -> DIGIMODE_R {
        DIGIMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Controls open-drain mode."]
    #[inline(always)]
    pub fn od(&self) -> OD_R {
        OD_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects pin function."]
    #[inline(always)]
    #[must_use]
    pub fn func(&mut self) -> FUNC_W<0> {
        FUNC_W::new(self)
    }
    #[doc = "Bits 4:5 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Bit 6 - Driver slew rate."]
    #[inline(always)]
    #[must_use]
    pub fn slew(&mut self) -> SLEW_W<6> {
        SLEW_W::new(self)
    }
    #[doc = "Bit 7 - Input polarity."]
    #[inline(always)]
    #[must_use]
    pub fn invert(&mut self) -> INVERT_W<7> {
        INVERT_W::new(self)
    }
    #[doc = "Bit 8 - Select Digital mode."]
    #[inline(always)]
    #[must_use]
    pub fn digimode(&mut self) -> DIGIMODE_W<8> {
        DIGIMODE_W::new(self)
    }
    #[doc = "Bit 9 - Controls open-drain mode."]
    #[inline(always)]
    #[must_use]
    pub fn od(&mut self) -> OD_W<9> {
        OD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital I/O control for port 1 pins PIO1_12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio1_12](index.html) module"]
pub struct PIO1_12_SPEC;
impl crate::RegisterSpec for PIO1_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio1_12::R](R) reader structure"]
impl crate::Readable for PIO1_12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio1_12::W](W) writer structure"]
impl crate::Writable for PIO1_12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIO1_12 to value 0"]
impl crate::Resettable for PIO1_12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
