#[doc = "Register `LOCK` reader"]
pub struct R(crate::R<LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCK` writer"]
pub struct W(crate::W<LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCK_SPEC>;
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
impl From<crate::W<LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECLOCK` reader - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
pub type SECLOCK_R = crate::FieldReader<u8, SECLOCK_A>;
#[doc = "Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SECLOCK_A {
    #[doc = "0: Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    UNLOCK = 0,
    #[doc = "1: Locks to the current security level. AHB Master will issue requests at this level."]
    LOCK = 1,
}
impl From<SECLOCK_A> for u8 {
    #[inline(always)]
    fn from(variant: SECLOCK_A) -> Self {
        variant as _
    }
}
impl SECLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SECLOCK_A> {
        match self.bits {
            0 => Some(SECLOCK_A::UNLOCK),
            1 => Some(SECLOCK_A::LOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCK`"]
    #[inline(always)]
    pub fn is_unlock(&self) -> bool {
        *self == SECLOCK_A::UNLOCK
    }
    #[doc = "Checks if the value of the field is `LOCK`"]
    #[inline(always)]
    pub fn is_lock(&self) -> bool {
        *self == SECLOCK_A::LOCK
    }
}
#[doc = "Field `SECLOCK` writer - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
pub type SECLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOCK_SPEC, u8, SECLOCK_A, 2, O>;
impl<'a, const O: u8> SECLOCK_W<'a, O> {
    #[doc = "Unlocks, so block is open to all. But, AHB Master will only issue non-secure requests."]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut W {
        self.variant(SECLOCK_A::UNLOCK)
    }
    #[doc = "Locks to the current security level. AHB Master will issue requests at this level."]
    #[inline(always)]
    pub fn lock(self) -> &'a mut W {
        self.variant(SECLOCK_A::LOCK)
    }
}
#[doc = "Field `PATTERN` reader - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
pub type PATTERN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PATTERN` writer - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
pub type PATTERN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LOCK_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:1 - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[inline(always)]
    pub fn seclock(&self) -> SECLOCK_R {
        SECLOCK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:15 - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
    #[inline(always)]
    pub fn pattern(&self) -> PATTERN_R {
        PATTERN_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Write 1 to secure-lock this block (if running in a security state). Write 0 to unlock. If locked already, may only write if at same or higher security level as lock. Reads as: 0 if unlocked, else 1, 2, 3 to indicate security level it is locked at. NOTE: this and ID are the only readable registers if locked and current state is lower than lock level."]
    #[inline(always)]
    #[must_use]
    pub fn seclock(&mut self) -> SECLOCK_W<0> {
        SECLOCK_W::new(self)
    }
    #[doc = "Bits 4:15 - Must write 0xA75 to change lock state. A75:Pattern needed to change bits 1:0"]
    #[inline(always)]
    #[must_use]
    pub fn pattern(&mut self) -> PATTERN_W<4> {
        PATTERN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock register allows locking to the current security level or unlocking by the lock holding level.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](index.html) module"]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lock::R](R) reader structure"]
impl crate::Readable for LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lock::W](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
