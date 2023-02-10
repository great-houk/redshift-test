#[doc = "Register `KEYLOCK` reader"]
pub struct R(crate::R<KEYLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYLOCK` writer"]
pub struct W(crate::W<KEYLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYLOCK_SPEC>;
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
impl From<crate::W<KEYLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY0` reader - \"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type KEY0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY0` writer - \"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type KEY0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYLOCK_SPEC, u8, u8, 2, O>;
#[doc = "Field `KEY1` reader - \"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type KEY1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY1` writer - \"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type KEY1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYLOCK_SPEC, u8, u8, 2, O>;
#[doc = "Field `KEY2` reader - \"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type KEY2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY2` writer - \"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type KEY2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYLOCK_SPEC, u8, u8, 2, O>;
#[doc = "Field `KEY3` reader - \"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type KEY3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY3` writer - \"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
pub type KEY3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYLOCK_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - \"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key0(&self) -> KEY0_R {
        KEY0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - \"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key1(&self) -> KEY1_R {
        KEY1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - \"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key2(&self) -> KEY2_R {
        KEY2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - \"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    pub fn key3(&self) -> KEY3_R {
        KEY3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - \"10:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is allowed. 00, 01, 11:Write access to KEY0MASK, KEYENABLE.KEY0 and KEYRESET.KEY0 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    #[must_use]
    pub fn key0(&mut self) -> KEY0_W<0> {
        KEY0_W::new(self)
    }
    #[doc = "Bits 2:3 - \"10:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is allowed. 00, 01, 11:Write access to KEY1MASK, KEYENABLE.KEY1 and KEYRESET.KEY1 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    #[must_use]
    pub fn key1(&mut self) -> KEY1_W<2> {
        KEY1_W::new(self)
    }
    #[doc = "Bits 4:5 - \"10:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is allowed. 00, 01, 11:Write access to KEY2MASK, KEYENABLE.KEY2 and KEYRESET.KEY2 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    #[must_use]
    pub fn key2(&mut self) -> KEY2_W<4> {
        KEY2_W::new(self)
    }
    #[doc = "Bits 6:7 - \"10:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is allowed. 00, 01, 11:Write access to KEY3MASK, KEYENABLE.KEY3 and KEYRESET.KEY3 is NOT allowed. Important Note : Once this field is written with a value different from '10', its value can no longer be modified until un Power On Reset occurs.\""]
    #[inline(always)]
    #[must_use]
    pub fn key3(&mut self) -> KEY3_W<6> {
        KEY3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Only reset in case of full IC reset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keylock](index.html) module"]
pub struct KEYLOCK_SPEC;
impl crate::RegisterSpec for KEYLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keylock::R](R) reader structure"]
impl crate::Readable for KEYLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keylock::W](W) writer structure"]
impl crate::Writable for KEYLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEYLOCK to value 0xaa"]
impl crate::Resettable for KEYLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0xaa;
}
