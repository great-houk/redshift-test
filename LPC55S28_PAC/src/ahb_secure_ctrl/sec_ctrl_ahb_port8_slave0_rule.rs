#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE0_RULE` reader"]
pub struct R(crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE0_RULE` writer"]
pub struct W(crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>;
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
impl From<crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA0_RULE` reader - DMA Controller"]
pub type DMA0_RULE_R = crate::FieldReader<u8, DMA0_RULE_A>;
#[doc = "DMA Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DMA0_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<DMA0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0_RULE_A) -> Self {
        variant as _
    }
}
impl DMA0_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0_RULE_A {
        match self.bits {
            0 => DMA0_RULE_A::ENUM_NS_NP,
            1 => DMA0_RULE_A::ENUM_NS_P,
            2 => DMA0_RULE_A::ENUM_S_NP,
            3 => DMA0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == DMA0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == DMA0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == DMA0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == DMA0_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `DMA0_RULE` writer - DMA Controller"]
pub type DMA0_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC, u8, DMA0_RULE_A, 2, O>;
impl<'a, const O: u8> DMA0_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(DMA0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(DMA0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(DMA0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(DMA0_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `FS_USB_DEV_RULE` reader - USB Full-speed device"]
pub type FS_USB_DEV_RULE_R = crate::FieldReader<u8, FS_USB_DEV_RULE_A>;
#[doc = "USB Full-speed device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FS_USB_DEV_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FS_USB_DEV_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FS_USB_DEV_RULE_A) -> Self {
        variant as _
    }
}
impl FS_USB_DEV_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FS_USB_DEV_RULE_A {
        match self.bits {
            0 => FS_USB_DEV_RULE_A::ENUM_NS_NP,
            1 => FS_USB_DEV_RULE_A::ENUM_NS_P,
            2 => FS_USB_DEV_RULE_A::ENUM_S_NP,
            3 => FS_USB_DEV_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FS_USB_DEV_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FS_USB_DEV_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FS_USB_DEV_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FS_USB_DEV_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `FS_USB_DEV_RULE` writer - USB Full-speed device"]
pub type FS_USB_DEV_RULE_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC,
    u8,
    FS_USB_DEV_RULE_A,
    2,
    O,
>;
impl<'a, const O: u8> FS_USB_DEV_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FS_USB_DEV_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FS_USB_DEV_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FS_USB_DEV_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FS_USB_DEV_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `SCT_RULE` reader - SCTimer"]
pub type SCT_RULE_R = crate::FieldReader<u8, SCT_RULE_A>;
#[doc = "SCTimer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCT_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<SCT_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: SCT_RULE_A) -> Self {
        variant as _
    }
}
impl SCT_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCT_RULE_A {
        match self.bits {
            0 => SCT_RULE_A::ENUM_NS_NP,
            1 => SCT_RULE_A::ENUM_NS_P,
            2 => SCT_RULE_A::ENUM_S_NP,
            3 => SCT_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == SCT_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == SCT_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == SCT_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == SCT_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `SCT_RULE` writer - SCTimer"]
pub type SCT_RULE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC, u8, SCT_RULE_A, 2, O>;
impl<'a, const O: u8> SCT_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(SCT_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(SCT_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(SCT_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(SCT_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `FLEXCOMM0_RULE` reader - Flexcomm interface 0"]
pub type FLEXCOMM0_RULE_R = crate::FieldReader<u8, FLEXCOMM0_RULE_A>;
#[doc = "Flexcomm interface 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM0_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM0_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM0_RULE_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM0_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM0_RULE_A {
        match self.bits {
            0 => FLEXCOMM0_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM0_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM0_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM0_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLEXCOMM0_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM0_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM0_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM0_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `FLEXCOMM0_RULE` writer - Flexcomm interface 0"]
pub type FLEXCOMM0_RULE_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC,
    u8,
    FLEXCOMM0_RULE_A,
    2,
    O,
>;
impl<'a, const O: u8> FLEXCOMM0_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM0_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM0_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM0_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM0_RULE_A::ENUM_S_P)
    }
}
#[doc = "Field `FLEXCOMM1_RULE` reader - Flexcomm interface 1"]
pub type FLEXCOMM1_RULE_R = crate::FieldReader<u8, FLEXCOMM1_RULE_A>;
#[doc = "Flexcomm interface 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLEXCOMM1_RULE_A {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    ENUM_NS_NP = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    ENUM_NS_P = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    ENUM_S_NP = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    ENUM_S_P = 3,
}
impl From<FLEXCOMM1_RULE_A> for u8 {
    #[inline(always)]
    fn from(variant: FLEXCOMM1_RULE_A) -> Self {
        variant as _
    }
}
impl FLEXCOMM1_RULE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCOMM1_RULE_A {
        match self.bits {
            0 => FLEXCOMM1_RULE_A::ENUM_NS_NP,
            1 => FLEXCOMM1_RULE_A::ENUM_NS_P,
            2 => FLEXCOMM1_RULE_A::ENUM_S_NP,
            3 => FLEXCOMM1_RULE_A::ENUM_S_P,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_NP`"]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == FLEXCOMM1_RULE_A::ENUM_NS_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_NS_P`"]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == FLEXCOMM1_RULE_A::ENUM_NS_P
    }
    #[doc = "Checks if the value of the field is `ENUM_S_NP`"]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == FLEXCOMM1_RULE_A::ENUM_S_NP
    }
    #[doc = "Checks if the value of the field is `ENUM_S_P`"]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == FLEXCOMM1_RULE_A::ENUM_S_P
    }
}
#[doc = "Field `FLEXCOMM1_RULE` writer - Flexcomm interface 1"]
pub type FLEXCOMM1_RULE_W<'a, const O: u8> = crate::FieldWriterSafe<
    'a,
    u32,
    SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC,
    u8,
    FLEXCOMM1_RULE_A,
    2,
    O,
>;
impl<'a, const O: u8> FLEXCOMM1_RULE_W<'a, O> {
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut W {
        self.variant(FLEXCOMM1_RULE_A::ENUM_NS_NP)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut W {
        self.variant(FLEXCOMM1_RULE_A::ENUM_NS_P)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut W {
        self.variant(FLEXCOMM1_RULE_A::ENUM_S_NP)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut W {
        self.variant(FLEXCOMM1_RULE_A::ENUM_S_P)
    }
}
impl R {
    #[doc = "Bits 8:9 - DMA Controller"]
    #[inline(always)]
    pub fn dma0_rule(&self) -> DMA0_RULE_R {
        DMA0_RULE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - USB Full-speed device"]
    #[inline(always)]
    pub fn fs_usb_dev_rule(&self) -> FS_USB_DEV_RULE_R {
        FS_USB_DEV_RULE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - SCTimer"]
    #[inline(always)]
    pub fn sct_rule(&self) -> SCT_RULE_R {
        SCT_RULE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Flexcomm interface 0"]
    #[inline(always)]
    pub fn flexcomm0_rule(&self) -> FLEXCOMM0_RULE_R {
        FLEXCOMM0_RULE_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Flexcomm interface 1"]
    #[inline(always)]
    pub fn flexcomm1_rule(&self) -> FLEXCOMM1_RULE_R {
        FLEXCOMM1_RULE_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - DMA Controller"]
    #[inline(always)]
    #[must_use]
    pub fn dma0_rule(&mut self) -> DMA0_RULE_W<8> {
        DMA0_RULE_W::new(self)
    }
    #[doc = "Bits 16:17 - USB Full-speed device"]
    #[inline(always)]
    #[must_use]
    pub fn fs_usb_dev_rule(&mut self) -> FS_USB_DEV_RULE_W<16> {
        FS_USB_DEV_RULE_W::new(self)
    }
    #[doc = "Bits 20:21 - SCTimer"]
    #[inline(always)]
    #[must_use]
    pub fn sct_rule(&mut self) -> SCT_RULE_W<20> {
        SCT_RULE_W::new(self)
    }
    #[doc = "Bits 24:25 - Flexcomm interface 0"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm0_rule(&mut self) -> FLEXCOMM0_RULE_W<24> {
        FLEXCOMM0_RULE_W::new(self)
    }
    #[doc = "Bits 28:29 - Flexcomm interface 1"]
    #[inline(always)]
    #[must_use]
    pub fn flexcomm1_rule(&mut self) -> FLEXCOMM1_RULE_W<28> {
        FLEXCOMM1_RULE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sec_ctrl_ahb_port8_slave0_rule](index.html) module"]
pub struct SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC;
impl crate::RegisterSpec for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sec_ctrl_ahb_port8_slave0_rule::R](R) reader structure"]
impl crate::Readable for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sec_ctrl_ahb_port8_slave0_rule::W](W) writer structure"]
impl crate::Writable for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT8_SLAVE0_RULE to value 0"]
impl crate::Resettable for SEC_CTRL_AHB_PORT8_SLAVE0_RULE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
