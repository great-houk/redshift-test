#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Mode` reader - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: SHA1 is enabled"]
    SHA1 = 1,
    #[doc = "2: SHA2-256 is enabled"]
    SHA2_256 = 2,
    #[doc = "4: AES if available (see also CRYPTCFG register for more controls)"]
    AES = 4,
    #[doc = "5: ICB-AES if available (see also CRYPTCFG register for more controls)"]
    ICB_AES = 5,
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
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::DISABLED),
            1 => Some(MODE_A::SHA1),
            2 => Some(MODE_A::SHA2_256),
            4 => Some(MODE_A::AES),
            5 => Some(MODE_A::ICB_AES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `SHA1`"]
    #[inline(always)]
    pub fn is_sha1(&self) -> bool {
        *self == MODE_A::SHA1
    }
    #[doc = "Checks if the value of the field is `SHA2_256`"]
    #[inline(always)]
    pub fn is_sha2_256(&self) -> bool {
        *self == MODE_A::SHA2_256
    }
    #[doc = "Checks if the value of the field is `AES`"]
    #[inline(always)]
    pub fn is_aes(&self) -> bool {
        *self == MODE_A::AES
    }
    #[doc = "Checks if the value of the field is `ICB_AES`"]
    #[inline(always)]
    pub fn is_icb_aes(&self) -> bool {
        *self == MODE_A::ICB_AES
    }
}
#[doc = "Field `Mode` writer - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, MODE_A, 3, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE_A::DISABLED)
    }
    #[doc = "SHA1 is enabled"]
    #[inline(always)]
    pub fn sha1(self) -> &'a mut W {
        self.variant(MODE_A::SHA1)
    }
    #[doc = "SHA2-256 is enabled"]
    #[inline(always)]
    pub fn sha2_256(self) -> &'a mut W {
        self.variant(MODE_A::SHA2_256)
    }
    #[doc = "AES if available (see also CRYPTCFG register for more controls)"]
    #[inline(always)]
    pub fn aes(self) -> &'a mut W {
        self.variant(MODE_A::AES)
    }
    #[doc = "ICB-AES if available (see also CRYPTCFG register for more controls)"]
    #[inline(always)]
    pub fn icb_aes(self) -> &'a mut W {
        self.variant(MODE_A::ICB_AES)
    }
}
#[doc = "Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEW_HASH_AW {
    #[doc = "1: Starts a new Hash/Crypto and initializes the Digest/Result."]
    START = 1,
}
impl From<NEW_HASH_AW> for bool {
    #[inline(always)]
    fn from(variant: NEW_HASH_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `New_Hash` writer - Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1."]
pub type NEW_HASH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, NEW_HASH_AW, O>;
impl<'a, const O: u8> NEW_HASH_W<'a, O> {
    #[doc = "Starts a new Hash/Crypto and initializes the Digest/Result."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(NEW_HASH_AW::START)
    }
}
#[doc = "Field `DMA_I` reader - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
pub type DMA_I_R = crate::BitReader<DMA_I_A>;
#[doc = "Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_I_A {
    #[doc = "0: DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    NOT_USED = 0,
    #[doc = "1: DMA will push in the data."]
    PUSH = 1,
}
impl From<DMA_I_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_I_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_I_A {
        match self.bits {
            false => DMA_I_A::NOT_USED,
            true => DMA_I_A::PUSH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_USED`"]
    #[inline(always)]
    pub fn is_not_used(&self) -> bool {
        *self == DMA_I_A::NOT_USED
    }
    #[doc = "Checks if the value of the field is `PUSH`"]
    #[inline(always)]
    pub fn is_push(&self) -> bool {
        *self == DMA_I_A::PUSH
    }
}
#[doc = "Field `DMA_I` writer - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
pub type DMA_I_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DMA_I_A, O>;
impl<'a, const O: u8> DMA_I_W<'a, O> {
    #[doc = "DMA is not used. Processor writes the necessary words when WAITING is set (interrupts), unless AHB Master is used."]
    #[inline(always)]
    pub fn not_used(self) -> &'a mut W {
        self.variant(DMA_I_A::NOT_USED)
    }
    #[doc = "DMA will push in the data."]
    #[inline(always)]
    pub fn push(self) -> &'a mut W {
        self.variant(DMA_I_A::PUSH)
    }
}
#[doc = "Field `DMA_O` reader - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
pub type DMA_O_R = crate::BitReader<DMA_O_A>;
#[doc = "Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_O_A {
    #[doc = "0: DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    NOTUSED = 0,
}
impl From<DMA_O_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_O_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_O_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMA_O_A> {
        match self.bits {
            false => Some(DMA_O_A::NOTUSED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOTUSED`"]
    #[inline(always)]
    pub fn is_notused(&self) -> bool {
        *self == DMA_O_A::NOTUSED
    }
}
#[doc = "Field `DMA_O` writer - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
pub type DMA_O_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DMA_O_A, O>;
impl<'a, const O: u8> DMA_O_W<'a, O> {
    #[doc = "DMA is not used. Processor reads the digest/output in response to DIGEST interrupt."]
    #[inline(always)]
    pub fn notused(self) -> &'a mut W {
        self.variant(DMA_O_A::NOTUSED)
    }
}
#[doc = "Field `HASHSWPB` reader - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
pub type HASHSWPB_R = crate::BitReader<bool>;
#[doc = "Field `HASHSWPB` writer - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
pub type HASHSWPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[inline(always)]
    pub fn dma_i(&self) -> DMA_I_R {
        DMA_I_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[inline(always)]
    pub fn dma_o(&self) -> DMA_O_R {
        DMA_O_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[inline(always)]
    pub fn hashswpb(&self) -> HASHSWPB_R {
        HASHSWPB_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - The operational mode to use, or 0 if none. Note that the CONFIG register will indicate if specific modes beyond SHA1 and SHA2-256 are available."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - Written with 1 when starting a new Hash/Crypto. It self clears. Note that the WAITING Status bit will clear for a cycle during the initialization from New=1."]
    #[inline(always)]
    #[must_use]
    pub fn new_hash(&mut self) -> NEW_HASH_W<4> {
        NEW_HASH_W::new(self)
    }
    #[doc = "Bit 8 - Written with 1 to use DMA to fill INDATA. If Hash, will request from DMA for 16 words and then will process the Hash. If Cryptographic, it will load as many words as needed, including key if not already loaded. It will then request again. Normal model is that the DMA interrupts the processor when its length expires. Note that if the processor will write the key and optionally IV, it should not enable this until it has done so. Otherwise, the DMA will be expected to load those for the 1st block (when needed)."]
    #[inline(always)]
    #[must_use]
    pub fn dma_i(&mut self) -> DMA_I_W<8> {
        DMA_I_W::new(self)
    }
    #[doc = "Bit 9 - Written to 1 to use DMA to drain the digest/output. If both DMA_I and DMA_O are set, the DMA has to know to switch direction and the locations. This can be used for crypto uses."]
    #[inline(always)]
    #[must_use]
    pub fn dma_o(&mut self) -> DMA_O_W<9> {
        DMA_O_W::new(self)
    }
    #[doc = "Bit 12 - If 1, will swap bytes in the word for SHA hashing. The default is byte order (so LSB is 1st byte) but this allows swapping to MSB is 1st such as is shown in SHS spec. For cryptographic swapping, see the CRYPTCFG register."]
    #[inline(always)]
    #[must_use]
    pub fn hashswpb(&mut self) -> HASHSWPB_W<12> {
        HASHSWPB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register to enable and operate Hash and Crypto\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
