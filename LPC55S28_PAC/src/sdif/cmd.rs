#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD_INDEX` reader - Command index."]
pub type CMD_INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMD_INDEX` writer - Command index."]
pub type CMD_INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESPONSE_EXPECT` reader - Response expect."]
pub type RESPONSE_EXPECT_R = crate::BitReader<bool>;
#[doc = "Field `RESPONSE_EXPECT` writer - Response expect."]
pub type RESPONSE_EXPECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RESPONSE_LENGTH` reader - Response length."]
pub type RESPONSE_LENGTH_R = crate::BitReader<bool>;
#[doc = "Field `RESPONSE_LENGTH` writer - Response length."]
pub type RESPONSE_LENGTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CHECK_RESPONSE_CRC` reader - Check response CRC."]
pub type CHECK_RESPONSE_CRC_R = crate::BitReader<bool>;
#[doc = "Field `CHECK_RESPONSE_CRC` writer - Check response CRC."]
pub type CHECK_RESPONSE_CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `DATA_EXPECTED` reader - Data expected."]
pub type DATA_EXPECTED_R = crate::BitReader<bool>;
#[doc = "Field `DATA_EXPECTED` writer - Data expected."]
pub type DATA_EXPECTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `READ_WRITE` reader - read/write."]
pub type READ_WRITE_R = crate::BitReader<bool>;
#[doc = "Field `READ_WRITE` writer - read/write."]
pub type READ_WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TRANSFER_MODE` reader - Transfer mode."]
pub type TRANSFER_MODE_R = crate::BitReader<bool>;
#[doc = "Field `TRANSFER_MODE` writer - Transfer mode."]
pub type TRANSFER_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `SEND_AUTO_STOP` reader - Send auto stop."]
pub type SEND_AUTO_STOP_R = crate::BitReader<bool>;
#[doc = "Field `SEND_AUTO_STOP` writer - Send auto stop."]
pub type SEND_AUTO_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `WAIT_PRVDATA_COMPLETE` reader - Wait prvdata complete."]
pub type WAIT_PRVDATA_COMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `WAIT_PRVDATA_COMPLETE` writer - Wait prvdata complete."]
pub type WAIT_PRVDATA_COMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `STOP_ABORT_CMD` reader - Stop abort command."]
pub type STOP_ABORT_CMD_R = crate::BitReader<bool>;
#[doc = "Field `STOP_ABORT_CMD` writer - Stop abort command."]
pub type STOP_ABORT_CMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `SEND_INITIALIZATION` reader - Send initialization."]
pub type SEND_INITIALIZATION_R = crate::BitReader<bool>;
#[doc = "Field `SEND_INITIALIZATION` writer - Send initialization."]
pub type SEND_INITIALIZATION_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CARD_NUMBER` reader - Specifies the card number of SDCARD for which the current Command is being executed"]
pub type CARD_NUMBER_R = crate::FieldReader<u8, CARD_NUMBER_A>;
#[doc = "Specifies the card number of SDCARD for which the current Command is being executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CARD_NUMBER_A {
    #[doc = "0: Command will be execute on SDCARD 0"]
    CARD0 = 0,
    #[doc = "1: Command will be execute on SDCARD 1"]
    CARD1 = 1,
}
impl From<CARD_NUMBER_A> for u8 {
    #[inline(always)]
    fn from(variant: CARD_NUMBER_A) -> Self {
        variant as _
    }
}
impl CARD_NUMBER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CARD_NUMBER_A> {
        match self.bits {
            0 => Some(CARD_NUMBER_A::CARD0),
            1 => Some(CARD_NUMBER_A::CARD1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CARD0`"]
    #[inline(always)]
    pub fn is_card0(&self) -> bool {
        *self == CARD_NUMBER_A::CARD0
    }
    #[doc = "Checks if the value of the field is `CARD1`"]
    #[inline(always)]
    pub fn is_card1(&self) -> bool {
        *self == CARD_NUMBER_A::CARD1
    }
}
#[doc = "Field `CARD_NUMBER` writer - Specifies the card number of SDCARD for which the current Command is being executed"]
pub type CARD_NUMBER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CMD_SPEC, u8, CARD_NUMBER_A, 5, O>;
impl<'a, const O: u8> CARD_NUMBER_W<'a, O> {
    #[doc = "Command will be execute on SDCARD 0"]
    #[inline(always)]
    pub fn card0(self) -> &'a mut W {
        self.variant(CARD_NUMBER_A::CARD0)
    }
    #[doc = "Command will be execute on SDCARD 1"]
    #[inline(always)]
    pub fn card1(self) -> &'a mut W {
        self.variant(CARD_NUMBER_A::CARD1)
    }
}
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` reader - Update clock registers only."]
pub type UPDATE_CLOCK_REGISTERS_ONLY_R = crate::BitReader<bool>;
#[doc = "Field `UPDATE_CLOCK_REGISTERS_ONLY` writer - Update clock registers only."]
pub type UPDATE_CLOCK_REGISTERS_ONLY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `READ_CEATA_DEVICE` reader - Read ceata device."]
pub type READ_CEATA_DEVICE_R = crate::BitReader<bool>;
#[doc = "Field `READ_CEATA_DEVICE` writer - Read ceata device."]
pub type READ_CEATA_DEVICE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CCS_EXPECTED` reader - CCS expected."]
pub type CCS_EXPECTED_R = crate::BitReader<bool>;
#[doc = "Field `CCS_EXPECTED` writer - CCS expected."]
pub type CCS_EXPECTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `ENABLE_BOOT` reader - Enable Boot - this bit should be set only for mandatory boot mode."]
pub type ENABLE_BOOT_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_BOOT` writer - Enable Boot - this bit should be set only for mandatory boot mode."]
pub type ENABLE_BOOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `EXPECT_BOOT_ACK` reader - Expect Boot Acknowledge."]
pub type EXPECT_BOOT_ACK_R = crate::BitReader<bool>;
#[doc = "Field `EXPECT_BOOT_ACK` writer - Expect Boot Acknowledge."]
pub type EXPECT_BOOT_ACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `DISABLE_BOOT` reader - Disable Boot."]
pub type DISABLE_BOOT_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE_BOOT` writer - Disable Boot."]
pub type DISABLE_BOOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `BOOT_MODE` reader - Boot Mode."]
pub type BOOT_MODE_R = crate::BitReader<bool>;
#[doc = "Field `BOOT_MODE` writer - Boot Mode."]
pub type BOOT_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `VOLT_SWITCH` reader - Voltage switch bit."]
pub type VOLT_SWITCH_R = crate::BitReader<bool>;
#[doc = "Field `VOLT_SWITCH` writer - Voltage switch bit."]
pub type VOLT_SWITCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `USE_HOLD_REG` reader - Use Hold Register."]
pub type USE_HOLD_REG_R = crate::BitReader<bool>;
#[doc = "Field `USE_HOLD_REG` writer - Use Hold Register."]
pub type USE_HOLD_REG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `START_CMD` reader - Start command."]
pub type START_CMD_R = crate::BitReader<bool>;
#[doc = "Field `START_CMD` writer - Start command."]
pub type START_CMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CMD_INDEX_R {
        CMD_INDEX_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Response expect."]
    #[inline(always)]
    pub fn response_expect(&self) -> RESPONSE_EXPECT_R {
        RESPONSE_EXPECT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Response length."]
    #[inline(always)]
    pub fn response_length(&self) -> RESPONSE_LENGTH_R {
        RESPONSE_LENGTH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Check response CRC."]
    #[inline(always)]
    pub fn check_response_crc(&self) -> CHECK_RESPONSE_CRC_R {
        CHECK_RESPONSE_CRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data expected."]
    #[inline(always)]
    pub fn data_expected(&self) -> DATA_EXPECTED_R {
        DATA_EXPECTED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - read/write."]
    #[inline(always)]
    pub fn read_write(&self) -> READ_WRITE_R {
        READ_WRITE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transfer mode."]
    #[inline(always)]
    pub fn transfer_mode(&self) -> TRANSFER_MODE_R {
        TRANSFER_MODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Send auto stop."]
    #[inline(always)]
    pub fn send_auto_stop(&self) -> SEND_AUTO_STOP_R {
        SEND_AUTO_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wait prvdata complete."]
    #[inline(always)]
    pub fn wait_prvdata_complete(&self) -> WAIT_PRVDATA_COMPLETE_R {
        WAIT_PRVDATA_COMPLETE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop abort command."]
    #[inline(always)]
    pub fn stop_abort_cmd(&self) -> STOP_ABORT_CMD_R {
        STOP_ABORT_CMD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Send initialization."]
    #[inline(always)]
    pub fn send_initialization(&self) -> SEND_INITIALIZATION_R {
        SEND_INITIALIZATION_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Specifies the card number of SDCARD for which the current Command is being executed"]
    #[inline(always)]
    pub fn card_number(&self) -> CARD_NUMBER_R {
        CARD_NUMBER_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Update clock registers only."]
    #[inline(always)]
    pub fn update_clock_registers_only(&self) -> UPDATE_CLOCK_REGISTERS_ONLY_R {
        UPDATE_CLOCK_REGISTERS_ONLY_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Read ceata device."]
    #[inline(always)]
    pub fn read_ceata_device(&self) -> READ_CEATA_DEVICE_R {
        READ_CEATA_DEVICE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CCS expected."]
    #[inline(always)]
    pub fn ccs_expected(&self) -> CCS_EXPECTED_R {
        CCS_EXPECTED_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline(always)]
    pub fn enable_boot(&self) -> ENABLE_BOOT_R {
        ENABLE_BOOT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge."]
    #[inline(always)]
    pub fn expect_boot_ack(&self) -> EXPECT_BOOT_ACK_R {
        EXPECT_BOOT_ACK_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable Boot."]
    #[inline(always)]
    pub fn disable_boot(&self) -> DISABLE_BOOT_R {
        DISABLE_BOOT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline(always)]
    pub fn boot_mode(&self) -> BOOT_MODE_R {
        BOOT_MODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline(always)]
    pub fn volt_switch(&self) -> VOLT_SWITCH_R {
        VOLT_SWITCH_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Use Hold Register."]
    #[inline(always)]
    pub fn use_hold_reg(&self) -> USE_HOLD_REG_R {
        USE_HOLD_REG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Start command."]
    #[inline(always)]
    pub fn start_cmd(&self) -> START_CMD_R {
        START_CMD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Command index."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CMD_INDEX_W<0> {
        CMD_INDEX_W::new(self)
    }
    #[doc = "Bit 6 - Response expect."]
    #[inline(always)]
    #[must_use]
    pub fn response_expect(&mut self) -> RESPONSE_EXPECT_W<6> {
        RESPONSE_EXPECT_W::new(self)
    }
    #[doc = "Bit 7 - Response length."]
    #[inline(always)]
    #[must_use]
    pub fn response_length(&mut self) -> RESPONSE_LENGTH_W<7> {
        RESPONSE_LENGTH_W::new(self)
    }
    #[doc = "Bit 8 - Check response CRC."]
    #[inline(always)]
    #[must_use]
    pub fn check_response_crc(&mut self) -> CHECK_RESPONSE_CRC_W<8> {
        CHECK_RESPONSE_CRC_W::new(self)
    }
    #[doc = "Bit 9 - Data expected."]
    #[inline(always)]
    #[must_use]
    pub fn data_expected(&mut self) -> DATA_EXPECTED_W<9> {
        DATA_EXPECTED_W::new(self)
    }
    #[doc = "Bit 10 - read/write."]
    #[inline(always)]
    #[must_use]
    pub fn read_write(&mut self) -> READ_WRITE_W<10> {
        READ_WRITE_W::new(self)
    }
    #[doc = "Bit 11 - Transfer mode."]
    #[inline(always)]
    #[must_use]
    pub fn transfer_mode(&mut self) -> TRANSFER_MODE_W<11> {
        TRANSFER_MODE_W::new(self)
    }
    #[doc = "Bit 12 - Send auto stop."]
    #[inline(always)]
    #[must_use]
    pub fn send_auto_stop(&mut self) -> SEND_AUTO_STOP_W<12> {
        SEND_AUTO_STOP_W::new(self)
    }
    #[doc = "Bit 13 - Wait prvdata complete."]
    #[inline(always)]
    #[must_use]
    pub fn wait_prvdata_complete(&mut self) -> WAIT_PRVDATA_COMPLETE_W<13> {
        WAIT_PRVDATA_COMPLETE_W::new(self)
    }
    #[doc = "Bit 14 - Stop abort command."]
    #[inline(always)]
    #[must_use]
    pub fn stop_abort_cmd(&mut self) -> STOP_ABORT_CMD_W<14> {
        STOP_ABORT_CMD_W::new(self)
    }
    #[doc = "Bit 15 - Send initialization."]
    #[inline(always)]
    #[must_use]
    pub fn send_initialization(&mut self) -> SEND_INITIALIZATION_W<15> {
        SEND_INITIALIZATION_W::new(self)
    }
    #[doc = "Bits 16:20 - Specifies the card number of SDCARD for which the current Command is being executed"]
    #[inline(always)]
    #[must_use]
    pub fn card_number(&mut self) -> CARD_NUMBER_W<16> {
        CARD_NUMBER_W::new(self)
    }
    #[doc = "Bit 21 - Update clock registers only."]
    #[inline(always)]
    #[must_use]
    pub fn update_clock_registers_only(&mut self) -> UPDATE_CLOCK_REGISTERS_ONLY_W<21> {
        UPDATE_CLOCK_REGISTERS_ONLY_W::new(self)
    }
    #[doc = "Bit 22 - Read ceata device."]
    #[inline(always)]
    #[must_use]
    pub fn read_ceata_device(&mut self) -> READ_CEATA_DEVICE_W<22> {
        READ_CEATA_DEVICE_W::new(self)
    }
    #[doc = "Bit 23 - CCS expected."]
    #[inline(always)]
    #[must_use]
    pub fn ccs_expected(&mut self) -> CCS_EXPECTED_W<23> {
        CCS_EXPECTED_W::new(self)
    }
    #[doc = "Bit 24 - Enable Boot - this bit should be set only for mandatory boot mode."]
    #[inline(always)]
    #[must_use]
    pub fn enable_boot(&mut self) -> ENABLE_BOOT_W<24> {
        ENABLE_BOOT_W::new(self)
    }
    #[doc = "Bit 25 - Expect Boot Acknowledge."]
    #[inline(always)]
    #[must_use]
    pub fn expect_boot_ack(&mut self) -> EXPECT_BOOT_ACK_W<25> {
        EXPECT_BOOT_ACK_W::new(self)
    }
    #[doc = "Bit 26 - Disable Boot."]
    #[inline(always)]
    #[must_use]
    pub fn disable_boot(&mut self) -> DISABLE_BOOT_W<26> {
        DISABLE_BOOT_W::new(self)
    }
    #[doc = "Bit 27 - Boot Mode."]
    #[inline(always)]
    #[must_use]
    pub fn boot_mode(&mut self) -> BOOT_MODE_W<27> {
        BOOT_MODE_W::new(self)
    }
    #[doc = "Bit 28 - Voltage switch bit."]
    #[inline(always)]
    #[must_use]
    pub fn volt_switch(&mut self) -> VOLT_SWITCH_W<28> {
        VOLT_SWITCH_W::new(self)
    }
    #[doc = "Bit 29 - Use Hold Register."]
    #[inline(always)]
    #[must_use]
    pub fn use_hold_reg(&mut self) -> USE_HOLD_REG_W<29> {
        USE_HOLD_REG_W::new(self)
    }
    #[doc = "Bit 31 - Start command."]
    #[inline(always)]
    #[must_use]
    pub fn start_cmd(&mut self) -> START_CMD_W<31> {
        START_CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
