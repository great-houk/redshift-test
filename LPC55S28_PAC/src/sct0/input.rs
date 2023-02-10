#[doc = "Register `INPUT` reader"]
pub struct R(crate::R<INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AIN0` reader - Input 0 state. Input 0 state on the last SCT clock edge."]
pub type AIN0_R = crate::BitReader<bool>;
#[doc = "Field `AIN1` reader - Input 1 state. Input 1 state on the last SCT clock edge."]
pub type AIN1_R = crate::BitReader<bool>;
#[doc = "Field `AIN2` reader - Input 2 state. Input 2 state on the last SCT clock edge."]
pub type AIN2_R = crate::BitReader<bool>;
#[doc = "Field `AIN3` reader - Input 3 state. Input 3 state on the last SCT clock edge."]
pub type AIN3_R = crate::BitReader<bool>;
#[doc = "Field `AIN4` reader - Input 4 state. Input 4 state on the last SCT clock edge."]
pub type AIN4_R = crate::BitReader<bool>;
#[doc = "Field `AIN5` reader - Input 5 state. Input 5 state on the last SCT clock edge."]
pub type AIN5_R = crate::BitReader<bool>;
#[doc = "Field `AIN6` reader - Input 6 state. Input 6 state on the last SCT clock edge."]
pub type AIN6_R = crate::BitReader<bool>;
#[doc = "Field `AIN7` reader - Input 7 state. Input 7 state on the last SCT clock edge."]
pub type AIN7_R = crate::BitReader<bool>;
#[doc = "Field `AIN8` reader - Input 8 state. Input 8 state on the last SCT clock edge."]
pub type AIN8_R = crate::BitReader<bool>;
#[doc = "Field `AIN9` reader - Input 9 state. Input 9 state on the last SCT clock edge."]
pub type AIN9_R = crate::BitReader<bool>;
#[doc = "Field `AIN10` reader - Input 10 state. Input 10 state on the last SCT clock edge."]
pub type AIN10_R = crate::BitReader<bool>;
#[doc = "Field `AIN11` reader - Input 11 state. Input 11 state on the last SCT clock edge."]
pub type AIN11_R = crate::BitReader<bool>;
#[doc = "Field `AIN12` reader - Input 12 state. Input 12 state on the last SCT clock edge."]
pub type AIN12_R = crate::BitReader<bool>;
#[doc = "Field `AIN13` reader - Input 13 state. Input 13 state on the last SCT clock edge."]
pub type AIN13_R = crate::BitReader<bool>;
#[doc = "Field `AIN14` reader - Input 14 state. Input 14 state on the last SCT clock edge."]
pub type AIN14_R = crate::BitReader<bool>;
#[doc = "Field `AIN15` reader - Input 15 state. Input 15 state on the last SCT clock edge."]
pub type AIN15_R = crate::BitReader<bool>;
#[doc = "Field `SIN0` reader - Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
pub type SIN0_R = crate::BitReader<bool>;
#[doc = "Field `SIN1` reader - Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
pub type SIN1_R = crate::BitReader<bool>;
#[doc = "Field `SIN2` reader - Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
pub type SIN2_R = crate::BitReader<bool>;
#[doc = "Field `SIN3` reader - Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
pub type SIN3_R = crate::BitReader<bool>;
#[doc = "Field `SIN4` reader - Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
pub type SIN4_R = crate::BitReader<bool>;
#[doc = "Field `SIN5` reader - Input 5 state. Input 5 state following the synchronization specified by INSYNC."]
pub type SIN5_R = crate::BitReader<bool>;
#[doc = "Field `SIN6` reader - Input 6 state. Input 6 state following the synchronization specified by INSYNC."]
pub type SIN6_R = crate::BitReader<bool>;
#[doc = "Field `SIN7` reader - Input 7 state. Input 7 state following the synchronization specified by INSYNC."]
pub type SIN7_R = crate::BitReader<bool>;
#[doc = "Field `SIN8` reader - Input 8 state. Input 8 state following the synchronization specified by INSYNC."]
pub type SIN8_R = crate::BitReader<bool>;
#[doc = "Field `SIN9` reader - Input 9 state. Input 9 state following the synchronization specified by INSYNC."]
pub type SIN9_R = crate::BitReader<bool>;
#[doc = "Field `SIN10` reader - Input 10 state. Input 10 state following the synchronization specified by INSYNC."]
pub type SIN10_R = crate::BitReader<bool>;
#[doc = "Field `SIN11` reader - Input 11 state. Input 11 state following the synchronization specified by INSYNC."]
pub type SIN11_R = crate::BitReader<bool>;
#[doc = "Field `SIN12` reader - Input 12 state. Input 12 state following the synchronization specified by INSYNC."]
pub type SIN12_R = crate::BitReader<bool>;
#[doc = "Field `SIN13` reader - Input 13 state. Input 13 state following the synchronization specified by INSYNC."]
pub type SIN13_R = crate::BitReader<bool>;
#[doc = "Field `SIN14` reader - Input 14 state. Input 14 state following the synchronization specified by INSYNC."]
pub type SIN14_R = crate::BitReader<bool>;
#[doc = "Field `SIN15` reader - Input 15 state. Input 15 state following the synchronization specified by INSYNC."]
pub type SIN15_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Input 0 state. Input 0 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain0(&self) -> AIN0_R {
        AIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input 1 state. Input 1 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain1(&self) -> AIN1_R {
        AIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input 2 state. Input 2 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain2(&self) -> AIN2_R {
        AIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input 3 state. Input 3 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain3(&self) -> AIN3_R {
        AIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input 4 state. Input 4 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain4(&self) -> AIN4_R {
        AIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input 5 state. Input 5 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain5(&self) -> AIN5_R {
        AIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input 6 state. Input 6 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain6(&self) -> AIN6_R {
        AIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input 7 state. Input 7 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain7(&self) -> AIN7_R {
        AIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Input 8 state. Input 8 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain8(&self) -> AIN8_R {
        AIN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Input 9 state. Input 9 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain9(&self) -> AIN9_R {
        AIN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Input 10 state. Input 10 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain10(&self) -> AIN10_R {
        AIN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input 11 state. Input 11 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain11(&self) -> AIN11_R {
        AIN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Input 12 state. Input 12 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain12(&self) -> AIN12_R {
        AIN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input 13 state. Input 13 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain13(&self) -> AIN13_R {
        AIN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Input 14 state. Input 14 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain14(&self) -> AIN14_R {
        AIN14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Input 15 state. Input 15 state on the last SCT clock edge."]
    #[inline(always)]
    pub fn ain15(&self) -> AIN15_R {
        AIN15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Input 0 state. Input 0 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin0(&self) -> SIN0_R {
        SIN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input 1 state. Input 1 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin1(&self) -> SIN1_R {
        SIN1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input 2 state. Input 2 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin2(&self) -> SIN2_R {
        SIN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Input 3 state. Input 3 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin3(&self) -> SIN3_R {
        SIN3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Input 4 state. Input 4 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin4(&self) -> SIN4_R {
        SIN4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Input 5 state. Input 5 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin5(&self) -> SIN5_R {
        SIN5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Input 6 state. Input 6 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin6(&self) -> SIN6_R {
        SIN6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Input 7 state. Input 7 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin7(&self) -> SIN7_R {
        SIN7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Input 8 state. Input 8 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin8(&self) -> SIN8_R {
        SIN8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Input 9 state. Input 9 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin9(&self) -> SIN9_R {
        SIN9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Input 10 state. Input 10 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin10(&self) -> SIN10_R {
        SIN10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Input 11 state. Input 11 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin11(&self) -> SIN11_R {
        SIN11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Input 12 state. Input 12 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin12(&self) -> SIN12_R {
        SIN12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Input 13 state. Input 13 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin13(&self) -> SIN13_R {
        SIN13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Input 14 state. Input 14 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin14(&self) -> SIN14_R {
        SIN14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Input 15 state. Input 15 state following the synchronization specified by INSYNC."]
    #[inline(always)]
    pub fn sin15(&self) -> SIN15_R {
        SIN15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SCT input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [input](index.html) module"]
pub struct INPUT_SPEC;
impl crate::RegisterSpec for INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [input::R](R) reader structure"]
impl crate::Readable for INPUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
