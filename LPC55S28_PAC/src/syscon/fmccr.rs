#[doc = "Register `FMCCR` reader"]
pub struct R(crate::R<FMCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMCCR` writer"]
pub struct W(crate::W<FMCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMCCR_SPEC>;
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
impl From<crate::W<FMCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FETCHCFG` reader - Instruction fetch configuration."]
pub type FETCHCFG_R = crate::FieldReader<u8, FETCHCFG_A>;
#[doc = "Instruction fetch configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FETCHCFG_A {
    #[doc = "0: Instruction fetches from flash are not buffered."]
    NOBUF = 0,
    #[doc = "1: One buffer is used for all instruction fetches."]
    ONEBUF = 1,
    #[doc = "2: All buffers may be used for instruction fetches."]
    ALLBUF = 2,
}
impl From<FETCHCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: FETCHCFG_A) -> Self {
        variant as _
    }
}
impl FETCHCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FETCHCFG_A> {
        match self.bits {
            0 => Some(FETCHCFG_A::NOBUF),
            1 => Some(FETCHCFG_A::ONEBUF),
            2 => Some(FETCHCFG_A::ALLBUF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOBUF`"]
    #[inline(always)]
    pub fn is_nobuf(&self) -> bool {
        *self == FETCHCFG_A::NOBUF
    }
    #[doc = "Checks if the value of the field is `ONEBUF`"]
    #[inline(always)]
    pub fn is_onebuf(&self) -> bool {
        *self == FETCHCFG_A::ONEBUF
    }
    #[doc = "Checks if the value of the field is `ALLBUF`"]
    #[inline(always)]
    pub fn is_allbuf(&self) -> bool {
        *self == FETCHCFG_A::ALLBUF
    }
}
#[doc = "Field `FETCHCFG` writer - Instruction fetch configuration."]
pub type FETCHCFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FMCCR_SPEC, u8, FETCHCFG_A, 2, O>;
impl<'a, const O: u8> FETCHCFG_W<'a, O> {
    #[doc = "Instruction fetches from flash are not buffered."]
    #[inline(always)]
    pub fn nobuf(self) -> &'a mut W {
        self.variant(FETCHCFG_A::NOBUF)
    }
    #[doc = "One buffer is used for all instruction fetches."]
    #[inline(always)]
    pub fn onebuf(self) -> &'a mut W {
        self.variant(FETCHCFG_A::ONEBUF)
    }
    #[doc = "All buffers may be used for instruction fetches."]
    #[inline(always)]
    pub fn allbuf(self) -> &'a mut W {
        self.variant(FETCHCFG_A::ALLBUF)
    }
}
#[doc = "Field `DATACFG` reader - Data read configuration."]
pub type DATACFG_R = crate::FieldReader<u8, DATACFG_A>;
#[doc = "Data read configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATACFG_A {
    #[doc = "0: Data accesses from flash are not buffered."]
    NOBUF = 0,
    #[doc = "1: One buffer is used for all data accesses."]
    ONEBUF = 1,
    #[doc = "2: All buffers can be used for data accesses."]
    ALLBUF = 2,
}
impl From<DATACFG_A> for u8 {
    #[inline(always)]
    fn from(variant: DATACFG_A) -> Self {
        variant as _
    }
}
impl DATACFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATACFG_A> {
        match self.bits {
            0 => Some(DATACFG_A::NOBUF),
            1 => Some(DATACFG_A::ONEBUF),
            2 => Some(DATACFG_A::ALLBUF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOBUF`"]
    #[inline(always)]
    pub fn is_nobuf(&self) -> bool {
        *self == DATACFG_A::NOBUF
    }
    #[doc = "Checks if the value of the field is `ONEBUF`"]
    #[inline(always)]
    pub fn is_onebuf(&self) -> bool {
        *self == DATACFG_A::ONEBUF
    }
    #[doc = "Checks if the value of the field is `ALLBUF`"]
    #[inline(always)]
    pub fn is_allbuf(&self) -> bool {
        *self == DATACFG_A::ALLBUF
    }
}
#[doc = "Field `DATACFG` writer - Data read configuration."]
pub type DATACFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FMCCR_SPEC, u8, DATACFG_A, 2, O>;
impl<'a, const O: u8> DATACFG_W<'a, O> {
    #[doc = "Data accesses from flash are not buffered."]
    #[inline(always)]
    pub fn nobuf(self) -> &'a mut W {
        self.variant(DATACFG_A::NOBUF)
    }
    #[doc = "One buffer is used for all data accesses."]
    #[inline(always)]
    pub fn onebuf(self) -> &'a mut W {
        self.variant(DATACFG_A::ONEBUF)
    }
    #[doc = "All buffers can be used for data accesses."]
    #[inline(always)]
    pub fn allbuf(self) -> &'a mut W {
        self.variant(DATACFG_A::ALLBUF)
    }
}
#[doc = "Field `ACCEL` reader - Acceleration enable."]
pub type ACCEL_R = crate::BitReader<ACCEL_A>;
#[doc = "Acceleration enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACCEL_A {
    #[doc = "0: Flash acceleration is disabled."]
    DISABLE = 0,
    #[doc = "1: Flash acceleration is enabled."]
    ENABLE = 1,
}
impl From<ACCEL_A> for bool {
    #[inline(always)]
    fn from(variant: ACCEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ACCEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCEL_A {
        match self.bits {
            false => ACCEL_A::DISABLE,
            true => ACCEL_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ACCEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ACCEL_A::ENABLE
    }
}
#[doc = "Field `ACCEL` writer - Acceleration enable."]
pub type ACCEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMCCR_SPEC, ACCEL_A, O>;
impl<'a, const O: u8> ACCEL_W<'a, O> {
    #[doc = "Flash acceleration is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ACCEL_A::DISABLE)
    }
    #[doc = "Flash acceleration is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ACCEL_A::ENABLE)
    }
}
#[doc = "Field `PREFEN` reader - Prefetch enable."]
pub type PREFEN_R = crate::BitReader<PREFEN_A>;
#[doc = "Prefetch enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREFEN_A {
    #[doc = "0: No instruction prefetch is performed."]
    DISABLE = 0,
    #[doc = "1: Instruction prefetch is enabled."]
    ENABLE = 1,
}
impl From<PREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: PREFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREFEN_A {
        match self.bits {
            false => PREFEN_A::DISABLE,
            true => PREFEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PREFEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == PREFEN_A::ENABLE
    }
}
#[doc = "Field `PREFEN` writer - Prefetch enable."]
pub type PREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMCCR_SPEC, PREFEN_A, O>;
impl<'a, const O: u8> PREFEN_W<'a, O> {
    #[doc = "No instruction prefetch is performed."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PREFEN_A::DISABLE)
    }
    #[doc = "Instruction prefetch is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(PREFEN_A::ENABLE)
    }
}
#[doc = "Field `PREFOVR` reader - Prefetch override."]
pub type PREFOVR_R = crate::BitReader<PREFOVR_A>;
#[doc = "Prefetch override.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREFOVR_A {
    #[doc = "0: Any previously initiated prefetch will be completed."]
    NORMAL = 0,
    #[doc = "1: Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    OVERRIDE = 1,
}
impl From<PREFOVR_A> for bool {
    #[inline(always)]
    fn from(variant: PREFOVR_A) -> Self {
        variant as u8 != 0
    }
}
impl PREFOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREFOVR_A {
        match self.bits {
            false => PREFOVR_A::NORMAL,
            true => PREFOVR_A::OVERRIDE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == PREFOVR_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == PREFOVR_A::OVERRIDE
    }
}
#[doc = "Field `PREFOVR` writer - Prefetch override."]
pub type PREFOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMCCR_SPEC, PREFOVR_A, O>;
impl<'a, const O: u8> PREFOVR_W<'a, O> {
    #[doc = "Any previously initiated prefetch will be completed."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(PREFOVR_A::NORMAL)
    }
    #[doc = "Any previously initiated prefetch will be aborted, and the next flash line following the current execution address will be prefetched if not already buffered."]
    #[inline(always)]
    pub fn override_(self) -> &'a mut W {
        self.variant(PREFOVR_A::OVERRIDE)
    }
}
#[doc = "Field `FLASHTIM` reader - Flash memory access time."]
pub type FLASHTIM_R = crate::FieldReader<u8, FLASHTIM_A>;
#[doc = "Flash memory access time.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLASHTIM_A {
    #[doc = "0: 1 system clock flash access time (for system clock rates up to 11 MHz)."]
    FLASHTIM0 = 0,
    #[doc = "1: 2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    FLASHTIM1 = 1,
    #[doc = "2: 3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    FLASHTIM2 = 2,
    #[doc = "3: 4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    FLASHTIM3 = 3,
    #[doc = "4: 5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    FLASHTIM4 = 4,
    #[doc = "5: 6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    FLASHTIM5 = 5,
    #[doc = "6: 7 system clocks flash access time (for system clock rates up to 77 MHz)."]
    FLASHTIM6 = 6,
    #[doc = "7: 8 system clocks flash access time (for system clock rates up to 88 MHz)."]
    FLASHTIM7 = 7,
    #[doc = "8: 9 system clocks flash access time (for system clock rates up to 100 MHz)."]
    FLASHTIM8 = 8,
    #[doc = "9: 10 system clocks flash access time (for system clock rates up to 115 MHz)."]
    FLASHTIM9 = 9,
    #[doc = "10: 11 system clocks flash access time (for system clock rates up to 130 MHz)."]
    FLASHTIM10 = 10,
    #[doc = "11: 12 system clocks flash access time (for system clock rates up to 150 MHz)."]
    FLASHTIM11 = 11,
}
impl From<FLASHTIM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLASHTIM_A) -> Self {
        variant as _
    }
}
impl FLASHTIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLASHTIM_A> {
        match self.bits {
            0 => Some(FLASHTIM_A::FLASHTIM0),
            1 => Some(FLASHTIM_A::FLASHTIM1),
            2 => Some(FLASHTIM_A::FLASHTIM2),
            3 => Some(FLASHTIM_A::FLASHTIM3),
            4 => Some(FLASHTIM_A::FLASHTIM4),
            5 => Some(FLASHTIM_A::FLASHTIM5),
            6 => Some(FLASHTIM_A::FLASHTIM6),
            7 => Some(FLASHTIM_A::FLASHTIM7),
            8 => Some(FLASHTIM_A::FLASHTIM8),
            9 => Some(FLASHTIM_A::FLASHTIM9),
            10 => Some(FLASHTIM_A::FLASHTIM10),
            11 => Some(FLASHTIM_A::FLASHTIM11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLASHTIM0`"]
    #[inline(always)]
    pub fn is_flashtim0(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM0
    }
    #[doc = "Checks if the value of the field is `FLASHTIM1`"]
    #[inline(always)]
    pub fn is_flashtim1(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM1
    }
    #[doc = "Checks if the value of the field is `FLASHTIM2`"]
    #[inline(always)]
    pub fn is_flashtim2(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM2
    }
    #[doc = "Checks if the value of the field is `FLASHTIM3`"]
    #[inline(always)]
    pub fn is_flashtim3(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM3
    }
    #[doc = "Checks if the value of the field is `FLASHTIM4`"]
    #[inline(always)]
    pub fn is_flashtim4(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM4
    }
    #[doc = "Checks if the value of the field is `FLASHTIM5`"]
    #[inline(always)]
    pub fn is_flashtim5(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM5
    }
    #[doc = "Checks if the value of the field is `FLASHTIM6`"]
    #[inline(always)]
    pub fn is_flashtim6(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM6
    }
    #[doc = "Checks if the value of the field is `FLASHTIM7`"]
    #[inline(always)]
    pub fn is_flashtim7(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM7
    }
    #[doc = "Checks if the value of the field is `FLASHTIM8`"]
    #[inline(always)]
    pub fn is_flashtim8(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM8
    }
    #[doc = "Checks if the value of the field is `FLASHTIM9`"]
    #[inline(always)]
    pub fn is_flashtim9(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM9
    }
    #[doc = "Checks if the value of the field is `FLASHTIM10`"]
    #[inline(always)]
    pub fn is_flashtim10(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM10
    }
    #[doc = "Checks if the value of the field is `FLASHTIM11`"]
    #[inline(always)]
    pub fn is_flashtim11(&self) -> bool {
        *self == FLASHTIM_A::FLASHTIM11
    }
}
#[doc = "Field `FLASHTIM` writer - Flash memory access time."]
pub type FLASHTIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FMCCR_SPEC, u8, FLASHTIM_A, 4, O>;
impl<'a, const O: u8> FLASHTIM_W<'a, O> {
    #[doc = "1 system clock flash access time (for system clock rates up to 11 MHz)."]
    #[inline(always)]
    pub fn flashtim0(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM0)
    }
    #[doc = "2 system clocks flash access time (for system clock rates up to 22 MHz)."]
    #[inline(always)]
    pub fn flashtim1(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM1)
    }
    #[doc = "3 system clocks flash access time (for system clock rates up to 33 MHz)."]
    #[inline(always)]
    pub fn flashtim2(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM2)
    }
    #[doc = "4 system clocks flash access time (for system clock rates up to 44 MHz)."]
    #[inline(always)]
    pub fn flashtim3(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM3)
    }
    #[doc = "5 system clocks flash access time (for system clock rates up to 55 MHz)."]
    #[inline(always)]
    pub fn flashtim4(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM4)
    }
    #[doc = "6 system clocks flash access time (for system clock rates up to 66 MHz)."]
    #[inline(always)]
    pub fn flashtim5(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM5)
    }
    #[doc = "7 system clocks flash access time (for system clock rates up to 77 MHz)."]
    #[inline(always)]
    pub fn flashtim6(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM6)
    }
    #[doc = "8 system clocks flash access time (for system clock rates up to 88 MHz)."]
    #[inline(always)]
    pub fn flashtim7(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM7)
    }
    #[doc = "9 system clocks flash access time (for system clock rates up to 100 MHz)."]
    #[inline(always)]
    pub fn flashtim8(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM8)
    }
    #[doc = "10 system clocks flash access time (for system clock rates up to 115 MHz)."]
    #[inline(always)]
    pub fn flashtim9(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM9)
    }
    #[doc = "11 system clocks flash access time (for system clock rates up to 130 MHz)."]
    #[inline(always)]
    pub fn flashtim10(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM10)
    }
    #[doc = "12 system clocks flash access time (for system clock rates up to 150 MHz)."]
    #[inline(always)]
    pub fn flashtim11(self) -> &'a mut W {
        self.variant(FLASHTIM_A::FLASHTIM11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Instruction fetch configuration."]
    #[inline(always)]
    pub fn fetchcfg(&self) -> FETCHCFG_R {
        FETCHCFG_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Data read configuration."]
    #[inline(always)]
    pub fn datacfg(&self) -> DATACFG_R {
        DATACFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    pub fn accel(&self) -> ACCEL_R {
        ACCEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    pub fn prefen(&self) -> PREFEN_R {
        PREFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Prefetch override."]
    #[inline(always)]
    pub fn prefovr(&self) -> PREFOVR_R {
        PREFOVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Flash memory access time."]
    #[inline(always)]
    pub fn flashtim(&self) -> FLASHTIM_R {
        FLASHTIM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Instruction fetch configuration."]
    #[inline(always)]
    #[must_use]
    pub fn fetchcfg(&mut self) -> FETCHCFG_W<0> {
        FETCHCFG_W::new(self)
    }
    #[doc = "Bits 2:3 - Data read configuration."]
    #[inline(always)]
    #[must_use]
    pub fn datacfg(&mut self) -> DATACFG_W<2> {
        DATACFG_W::new(self)
    }
    #[doc = "Bit 4 - Acceleration enable."]
    #[inline(always)]
    #[must_use]
    pub fn accel(&mut self) -> ACCEL_W<4> {
        ACCEL_W::new(self)
    }
    #[doc = "Bit 5 - Prefetch enable."]
    #[inline(always)]
    #[must_use]
    pub fn prefen(&mut self) -> PREFEN_W<5> {
        PREFEN_W::new(self)
    }
    #[doc = "Bit 6 - Prefetch override."]
    #[inline(always)]
    #[must_use]
    pub fn prefovr(&mut self) -> PREFOVR_W<6> {
        PREFOVR_W::new(self)
    }
    #[doc = "Bits 12:15 - Flash memory access time."]
    #[inline(always)]
    #[must_use]
    pub fn flashtim(&mut self) -> FLASHTIM_W<12> {
        FLASHTIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmccr](index.html) module"]
pub struct FMCCR_SPEC;
impl crate::RegisterSpec for FMCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmccr::R](R) reader structure"]
impl crate::Readable for FMCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmccr::W](W) writer structure"]
impl crate::Writable for FMCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMCCR to value 0x2000"]
impl crate::Resettable for FMCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
