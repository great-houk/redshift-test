#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Memory Remap control register"]
    pub memoryremap: MEMORYREMAP,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
    pub ahbmatprio: AHBMATPRIO,
    _reserved2: [u8; 0x24],
    #[doc = "0x38 - System tick calibration for secure part of CPU0"]
    pub cpu0stckcal: CPU0STCKCAL,
    #[doc = "0x3c - System tick calibration for non-secure part of CPU0"]
    pub cpu0nstckcal: CPU0NSTCKCAL,
    #[doc = "0x40 - System tick calibration for CPU1"]
    pub cpu1stckcal: CPU1STCKCAL,
    _reserved5: [u8; 0x04],
    #[doc = "0x48 - NMI Source Select"]
    pub nmisrc: NMISRC,
    _reserved6: [u8; 0xb4],
    _reserved_6_presetctrl: [u8; 0x04],
    _reserved_7_presetctrl: [u8; 0x04],
    _reserved_8_presetctrl: [u8; 0x04],
    _reserved9: [u8; 0x14],
    #[doc = "0x120..0x12c - Peripheral reset control set register"]
    pub presetctrlset: [PRESETCTRLSET; 3],
    _reserved10: [u8; 0x14],
    #[doc = "0x140..0x14c - Peripheral reset control clear register"]
    pub presetctrlclr: [PRESETCTRLCLR; 3],
    _reserved11: [u8; 0x14],
    #[doc = "0x160 - generate a software_reset"]
    pub swr_reset: SWR_RESET,
    _reserved12: [u8; 0x9c],
    _reserved_12_ahbclkctrl: [u8; 0x04],
    _reserved_13_ahbclkctrl: [u8; 0x04],
    _reserved_14_ahbclkctrl: [u8; 0x04],
    _reserved15: [u8; 0x14],
    #[doc = "0x220..0x22c - Peripheral reset control register"]
    pub ahbclkctrlset: [AHBCLKCTRLSET; 3],
    _reserved16: [u8; 0x14],
    #[doc = "0x240..0x24c - Peripheral reset control register"]
    pub ahbclkctrlclr: [AHBCLKCTRLCLR; 3],
    _reserved17: [u8; 0x14],
    _reserved_17_systickclksel: [u8; 0x04],
    _reserved18: [u8; 0x04],
    #[doc = "0x268 - Trace clock source select"]
    pub traceclksel: TRACECLKSEL,
    _reserved_19_ctimerclksel: [u8; 0x04],
    _reserved_20_ctimerclksel: [u8; 0x04],
    _reserved_21_ctimerclksel: [u8; 0x04],
    _reserved_22_ctimerclksel: [u8; 0x04],
    _reserved_23_ctimerclksel: [u8; 0x04],
    #[doc = "0x280 - Main clock A source select"]
    pub mainclksela: MAINCLKSELA,
    #[doc = "0x284 - Main clock source select"]
    pub mainclkselb: MAINCLKSELB,
    #[doc = "0x288 - CLKOUT clock source select"]
    pub clkoutsel: CLKOUTSEL,
    _reserved27: [u8; 0x04],
    #[doc = "0x290 - PLL0 clock source select"]
    pub pll0clksel: PLL0CLKSEL,
    #[doc = "0x294 - PLL1 clock source select"]
    pub pll1clksel: PLL1CLKSEL,
    _reserved29: [u8; 0x0c],
    #[doc = "0x2a4 - ADC clock source select"]
    pub adcclksel: ADCCLKSEL,
    #[doc = "0x2a8 - FS USB clock source select"]
    pub usb0clksel: USB0CLKSEL,
    _reserved31: [u8; 0x04],
    _reserved_31_fcclksel: [u8; 0x04],
    _reserved_32_fcclksel: [u8; 0x04],
    _reserved_33_fcclksel: [u8; 0x04],
    _reserved_34_fcclksel: [u8; 0x04],
    _reserved_35_fcclksel: [u8; 0x04],
    _reserved_36_fcclksel: [u8; 0x04],
    _reserved_37_fcclksel: [u8; 0x04],
    _reserved_38_fcclksel: [u8; 0x04],
    #[doc = "0x2d0 - HS LSPI clock source select"]
    pub hslspiclksel: HSLSPICLKSEL,
    _reserved40: [u8; 0x0c],
    #[doc = "0x2e0 - MCLK clock source select"]
    pub mclkclksel: MCLKCLKSEL,
    _reserved41: [u8; 0x0c],
    #[doc = "0x2f0 - SCTimer/PWM clock source select"]
    pub sctclksel: SCTCLKSEL,
    _reserved42: [u8; 0x04],
    #[doc = "0x2f8 - SDIO clock source select"]
    pub sdioclksel: SDIOCLKSEL,
    _reserved43: [u8; 0x04],
    #[doc = "0x300 - System Tick Timer divider for CPU0"]
    pub systickclkdiv0: SYSTICKCLKDIV0,
    #[doc = "0x304 - System Tick Timer divider for CPU1"]
    pub systickclkdiv1: SYSTICKCLKDIV1,
    #[doc = "0x308 - TRACE clock divider"]
    pub traceclkdiv: TRACECLKDIV,
    _reserved46: [u8; 0x14],
    _reserved_46_flexfrgctrl: [u8; 0x04],
    _reserved_47_flexfrgctrl: [u8; 0x04],
    _reserved_48_flexfrgctrl: [u8; 0x04],
    _reserved_49_flexfrgctrl: [u8; 0x04],
    _reserved_50_flexfrgctrl: [u8; 0x04],
    _reserved_51_flexfrgctrl: [u8; 0x04],
    _reserved_52_flexfrgctrl: [u8; 0x04],
    _reserved_53_flexfrgctrl: [u8; 0x04],
    _reserved54: [u8; 0x40],
    #[doc = "0x380 - System clock divider"]
    pub ahbclkdiv: AHBCLKDIV,
    #[doc = "0x384 - CLKOUT clock divider"]
    pub clkoutdiv: CLKOUTDIV,
    #[doc = "0x388 - FRO_HF (96MHz) clock divider"]
    pub frohfdiv: FROHFDIV,
    #[doc = "0x38c - WDT clock divider"]
    pub wdtclkdiv: WDTCLKDIV,
    _reserved58: [u8; 0x04],
    #[doc = "0x394 - ADC clock divider"]
    pub adcclkdiv: ADCCLKDIV,
    #[doc = "0x398 - USB0 Clock divider"]
    pub usb0clkdiv: USB0CLKDIV,
    _reserved60: [u8; 0x10],
    #[doc = "0x3ac - I2S MCLK clock divider"]
    pub mclkdiv: MCLKDIV,
    _reserved61: [u8; 0x04],
    #[doc = "0x3b4 - SCT/PWM clock divider"]
    pub sctclkdiv: SCTCLKDIV,
    _reserved62: [u8; 0x04],
    #[doc = "0x3bc - SDIO clock divider"]
    pub sdioclkdiv: SDIOCLKDIV,
    _reserved63: [u8; 0x04],
    #[doc = "0x3c4 - PLL0 clock divider"]
    pub pll0clkdiv: PLL0CLKDIV,
    _reserved64: [u8; 0x34],
    #[doc = "0x3fc - Control clock configuration registers access (like xxxDIV, xxxSEL)"]
    pub clockgenupdatelockout: CLOCKGENUPDATELOCKOUT,
    #[doc = "0x400 - FMC configuration register"]
    pub fmccr: FMCCR,
    _reserved66: [u8; 0x08],
    #[doc = "0x40c - USB0 need clock control"]
    pub usb0needclkctrl: USB0NEEDCLKCTRL,
    #[doc = "0x410 - USB0 need clock status"]
    pub usb0needclkstat: USB0NEEDCLKSTAT,
    _reserved68: [u8; 0x08],
    #[doc = "0x41c - FMCflush control"]
    pub fmcflush: FMCFLUSH,
    #[doc = "0x420 - MCLK control"]
    pub mclkio: MCLKIO,
    #[doc = "0x424 - USB1 need clock control"]
    pub usb1needclkctrl: USB1NEEDCLKCTRL,
    #[doc = "0x428 - USB1 need clock status"]
    pub usb1needclkstat: USB1NEEDCLKSTAT,
    _reserved72: [u8; 0x34],
    #[doc = "0x460 - SDIO CCLKIN phase and delay control"]
    pub sdioclkctrl: SDIOCLKCTRL,
    _reserved73: [u8; 0xfc],
    #[doc = "0x560 - PLL1 550m control"]
    pub pll1ctrl: PLL1CTRL,
    #[doc = "0x564 - PLL1 550m status"]
    pub pll1stat: PLL1STAT,
    #[doc = "0x568 - PLL1 550m N divider"]
    pub pll1ndec: PLL1NDEC,
    #[doc = "0x56c - PLL1 550m M divider"]
    pub pll1mdec: PLL1MDEC,
    #[doc = "0x570 - PLL1 550m P divider"]
    pub pll1pdec: PLL1PDEC,
    _reserved78: [u8; 0x0c],
    #[doc = "0x580 - PLL0 550m control"]
    pub pll0ctrl: PLL0CTRL,
    #[doc = "0x584 - PLL0 550m status"]
    pub pll0stat: PLL0STAT,
    #[doc = "0x588 - PLL0 550m N divider"]
    pub pll0ndec: PLL0NDEC,
    #[doc = "0x58c - PLL0 550m P divider"]
    pub pll0pdec: PLL0PDEC,
    #[doc = "0x590 - PLL0 Spread Spectrum Wrapper control register 0"]
    pub pll0sscg0: PLL0SSCG0,
    #[doc = "0x594 - PLL0 Spread Spectrum Wrapper control register 1"]
    pub pll0sscg1: PLL0SSCG1,
    _reserved84: [u8; 0x0268],
    #[doc = "0x800 - CPU Control for multiple processors"]
    pub cpuctrl: CPUCTRL,
    #[doc = "0x804 - Coprocessor Boot Address"]
    pub cpboot: CPBOOT,
    _reserved86: [u8; 0x04],
    #[doc = "0x80c - CPU Status"]
    pub cpstat: CPSTAT,
    _reserved87: [u8; 0x0208],
    #[doc = "0xa18 - Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
    pub clock_ctrl: CLOCK_CTRL,
    _reserved88: [u8; 0xf4],
    #[doc = "0xb10 - Comparator Interrupt control"]
    pub comp_int_ctrl: COMP_INT_CTRL,
    #[doc = "0xb14 - Comparator Interrupt status"]
    pub comp_int_status: COMP_INT_STATUS,
    _reserved90: [u8; 0x02ec],
    #[doc = "0xe04 - Control automatic clock gating"]
    pub autoclkgateoverride: AUTOCLKGATEOVERRIDE,
    #[doc = "0xe08 - Enable bypass of the first stage of synchonization inside GPIO_INT module"]
    pub gpiopsync: GPIOPSYNC,
    _reserved92: [u8; 0x0194],
    #[doc = "0xfa0 - Control write access to security registers."]
    pub debug_lock_en: DEBUG_LOCK_EN,
    #[doc = "0xfa4 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control."]
    pub debug_features: DEBUG_FEATURES,
    #[doc = "0xfa8 - Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register."]
    pub debug_features_dp: DEBUG_FEATURES_DP,
    _reserved95: [u8; 0x10],
    #[doc = "0xfbc - block quiddikey/PUF all index."]
    pub key_block: KEY_BLOCK,
    #[doc = "0xfc0 - Debug authentication BEACON register"]
    pub debug_auth_beacon: DEBUG_AUTH_BEACON,
    _reserved97: [u8; 0x34],
    #[doc = "0xff8 - Device ID"]
    pub device_id0: DEVICE_ID0,
    #[doc = "0xffc - Chip revision ID and Number"]
    pub dieid: DIEID,
}
impl RegisterBlock {
    #[doc = "0x100 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn presetctrl_presetctrlx0(&self) -> &PRESETCTRL_PRESETCTRLX0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(256usize).cast() }
    }
    #[doc = "0x100 - Peripheral reset control 0"]
    #[inline(always)]
    pub const fn presetctrl_presetctrl0(&self) -> &PRESETCTRL_PRESETCTRL0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(256usize).cast() }
    }
    #[doc = "0x104 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn presetctrl_presetctrlx1(&self) -> &PRESETCTRL_PRESETCTRLX1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(260usize).cast() }
    }
    #[doc = "0x104 - Peripheral reset control 1"]
    #[inline(always)]
    pub const fn presetctrl_presetctrl1(&self) -> &PRESETCTRL_PRESETCTRL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(260usize).cast() }
    }
    #[doc = "0x108 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn presetctrl_presetctrlx2(&self) -> &PRESETCTRL_PRESETCTRLX2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(264usize).cast() }
    }
    #[doc = "0x108 - Peripheral reset control 2"]
    #[inline(always)]
    pub const fn presetctrl_presetctrl2(&self) -> &PRESETCTRL_PRESETCTRL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(264usize).cast() }
    }
    #[doc = "0x200 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrlx0(&self) -> &AHBCLKCTRL_AHBCLKCTRLX0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(512usize).cast() }
    }
    #[doc = "0x200 - AHB Clock control 0"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrl0(&self) -> &AHBCLKCTRL_AHBCLKCTRL0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(512usize).cast() }
    }
    #[doc = "0x204 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrlx1(&self) -> &AHBCLKCTRL_AHBCLKCTRLX1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(516usize).cast() }
    }
    #[doc = "0x204 - AHB Clock control 1"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrl1(&self) -> &AHBCLKCTRL_AHBCLKCTRL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(516usize).cast() }
    }
    #[doc = "0x208 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrlx2(&self) -> &AHBCLKCTRL_AHBCLKCTRLX2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(520usize).cast() }
    }
    #[doc = "0x208 - AHB Clock control 2"]
    #[inline(always)]
    pub const fn ahbclkctrl_ahbclkctrl2(&self) -> &AHBCLKCTRL_AHBCLKCTRL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(520usize).cast() }
    }
    #[doc = "0x260 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn systickclksel_systickclkselx0(&self) -> &SYSTICKCLKSEL_SYSTICKCLKSELX0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(608usize).cast() }
    }
    #[doc = "0x260 - System Tick Timer for CPU0 source select"]
    #[inline(always)]
    pub const fn systickclksel_systickclksel0(&self) -> &SYSTICKCLKSEL_SYSTICKCLKSEL0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(608usize).cast() }
    }
    #[doc = "0x26c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclkselx0(&self) -> &CTIMERCLKSEL_CTIMERCLKSELX0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(620usize).cast() }
    }
    #[doc = "0x26c - CTimer 0 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclksel0(&self) -> &CTIMERCLKSEL_CTIMERCLKSEL0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(620usize).cast() }
    }
    #[doc = "0x270 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclkselx1(&self) -> &CTIMERCLKSEL_CTIMERCLKSELX1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(624usize).cast() }
    }
    #[doc = "0x270 - CTimer 1 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclksel1(&self) -> &CTIMERCLKSEL_CTIMERCLKSEL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(624usize).cast() }
    }
    #[doc = "0x274 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclkselx2(&self) -> &CTIMERCLKSEL_CTIMERCLKSELX2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(628usize).cast() }
    }
    #[doc = "0x274 - CTimer 2 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclksel2(&self) -> &CTIMERCLKSEL_CTIMERCLKSEL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(628usize).cast() }
    }
    #[doc = "0x278 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclkselx3(&self) -> &CTIMERCLKSEL_CTIMERCLKSELX3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(632usize).cast() }
    }
    #[doc = "0x278 - CTimer 3 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclksel3(&self) -> &CTIMERCLKSEL_CTIMERCLKSEL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(632usize).cast() }
    }
    #[doc = "0x27c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclkselx4(&self) -> &CTIMERCLKSEL_CTIMERCLKSELX4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(636usize).cast() }
    }
    #[doc = "0x27c - CTimer 4 clock source select"]
    #[inline(always)]
    pub const fn ctimerclksel_ctimerclksel4(&self) -> &CTIMERCLKSEL_CTIMERCLKSEL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(636usize).cast() }
    }
    #[doc = "0x2b0 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx0(&self) -> &FCCLKSEL_FCCLKSELX0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(688usize).cast() }
    }
    #[doc = "0x2b0 - Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel0(&self) -> &FCCLKSEL_FCCLKSEL0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(688usize).cast() }
    }
    #[doc = "0x2b4 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx1(&self) -> &FCCLKSEL_FCCLKSELX1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(692usize).cast() }
    }
    #[doc = "0x2b4 - Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel1(&self) -> &FCCLKSEL_FCCLKSEL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(692usize).cast() }
    }
    #[doc = "0x2b8 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx2(&self) -> &FCCLKSEL_FCCLKSELX2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(696usize).cast() }
    }
    #[doc = "0x2b8 - Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel2(&self) -> &FCCLKSEL_FCCLKSEL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(696usize).cast() }
    }
    #[doc = "0x2bc - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx3(&self) -> &FCCLKSEL_FCCLKSELX3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(700usize).cast() }
    }
    #[doc = "0x2bc - Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel3(&self) -> &FCCLKSEL_FCCLKSEL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(700usize).cast() }
    }
    #[doc = "0x2c0 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx4(&self) -> &FCCLKSEL_FCCLKSELX4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(704usize).cast() }
    }
    #[doc = "0x2c0 - Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel4(&self) -> &FCCLKSEL_FCCLKSEL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(704usize).cast() }
    }
    #[doc = "0x2c4 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx5(&self) -> &FCCLKSEL_FCCLKSELX5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(708usize).cast() }
    }
    #[doc = "0x2c4 - Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel5(&self) -> &FCCLKSEL_FCCLKSEL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(708usize).cast() }
    }
    #[doc = "0x2c8 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx6(&self) -> &FCCLKSEL_FCCLKSELX6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(712usize).cast() }
    }
    #[doc = "0x2c8 - Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel6(&self) -> &FCCLKSEL_FCCLKSEL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(712usize).cast() }
    }
    #[doc = "0x2cc - Peripheral reset control register"]
    #[inline(always)]
    pub const fn fcclksel_fcclkselx7(&self) -> &FCCLKSEL_FCCLKSELX7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(716usize).cast() }
    }
    #[doc = "0x2cc - Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
    #[inline(always)]
    pub const fn fcclksel_fcclksel7(&self) -> &FCCLKSEL_FCCLKSEL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(716usize).cast() }
    }
    #[doc = "0x320 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl0(&self) -> &FLEXFRGCTRL_FLEXFRGXCTRL0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(800usize).cast() }
    }
    #[doc = "0x320 - Fractional rate divider for flexcomm 0"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg0ctrl(&self) -> &FLEXFRGCTRL_FLEXFRG0CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(800usize).cast() }
    }
    #[doc = "0x324 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl1(&self) -> &FLEXFRGCTRL_FLEXFRGXCTRL1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(804usize).cast() }
    }
    #[doc = "0x324 - Fractional rate divider for flexcomm 1"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg1ctrl(&self) -> &FLEXFRGCTRL_FLEXFRG1CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(804usize).cast() }
    }
    #[doc = "0x328 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl2(&self) -> &FLEXFRGCTRL_FLEXFRGXCTRL2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(808usize).cast() }
    }
    #[doc = "0x328 - Fractional rate divider for flexcomm 2"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg2ctrl(&self) -> &FLEXFRGCTRL_FLEXFRG2CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(808usize).cast() }
    }
    #[doc = "0x32c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl3(&self) -> &FLEXFRGCTRL_FLEXFRGXCTRL3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(812usize).cast() }
    }
    #[doc = "0x32c - Fractional rate divider for flexcomm 3"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg3ctrl(&self) -> &FLEXFRGCTRL_FLEXFRG3CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(812usize).cast() }
    }
    #[doc = "0x330 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl4(&self) -> &FLEXFRGCTRL_FLEXFRGXCTRL4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(816usize).cast() }
    }
    #[doc = "0x330 - Fractional rate divider for flexcomm 4"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg4ctrl(&self) -> &FLEXFRGCTRL_FLEXFRG4CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(816usize).cast() }
    }
    #[doc = "0x334 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl5(&self) -> &FLEXFRGCTRL_FLEXFRGXCTRL5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(820usize).cast() }
    }
    #[doc = "0x334 - Fractional rate divider for flexcomm 5"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg5ctrl(&self) -> &FLEXFRGCTRL_FLEXFRG5CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(820usize).cast() }
    }
    #[doc = "0x338 - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl6(&self) -> &FLEXFRGCTRL_FLEXFRGXCTRL6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(824usize).cast() }
    }
    #[doc = "0x338 - Fractional rate divider for flexcomm 6"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg6ctrl(&self) -> &FLEXFRGCTRL_FLEXFRG6CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(824usize).cast() }
    }
    #[doc = "0x33c - Peripheral reset control register"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrgxctrl7(&self) -> &FLEXFRGCTRL_FLEXFRGXCTRL7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(828usize).cast() }
    }
    #[doc = "0x33c - Fractional rate divider for flexcomm 7"]
    #[inline(always)]
    pub const fn flexfrgctrl_flexfrg7ctrl(&self) -> &FLEXFRGCTRL_FLEXFRG7CTRL {
        unsafe { &*(self as *const Self).cast::<u8>().add(828usize).cast() }
    }
}
#[doc = "MEMORYREMAP (rw) register accessor: an alias for `Reg<MEMORYREMAP_SPEC>`"]
pub type MEMORYREMAP = crate::Reg<memoryremap::MEMORYREMAP_SPEC>;
#[doc = "Memory Remap control register"]
pub mod memoryremap;
#[doc = "AHBMATPRIO (rw) register accessor: an alias for `Reg<AHBMATPRIO_SPEC>`"]
pub type AHBMATPRIO = crate::Reg<ahbmatprio::AHBMATPRIO_SPEC>;
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest"]
pub mod ahbmatprio;
#[doc = "CPU0STCKCAL (rw) register accessor: an alias for `Reg<CPU0STCKCAL_SPEC>`"]
pub type CPU0STCKCAL = crate::Reg<cpu0stckcal::CPU0STCKCAL_SPEC>;
#[doc = "System tick calibration for secure part of CPU0"]
pub mod cpu0stckcal;
#[doc = "CPU0NSTCKCAL (rw) register accessor: an alias for `Reg<CPU0NSTCKCAL_SPEC>`"]
pub type CPU0NSTCKCAL = crate::Reg<cpu0nstckcal::CPU0NSTCKCAL_SPEC>;
#[doc = "System tick calibration for non-secure part of CPU0"]
pub mod cpu0nstckcal;
#[doc = "CPU1STCKCAL (rw) register accessor: an alias for `Reg<CPU1STCKCAL_SPEC>`"]
pub type CPU1STCKCAL = crate::Reg<cpu1stckcal::CPU1STCKCAL_SPEC>;
#[doc = "System tick calibration for CPU1"]
pub mod cpu1stckcal;
#[doc = "NMISRC (rw) register accessor: an alias for `Reg<NMISRC_SPEC>`"]
pub type NMISRC = crate::Reg<nmisrc::NMISRC_SPEC>;
#[doc = "NMI Source Select"]
pub mod nmisrc;
#[doc = "PRESETCTRL_PRESETCTRL0 (rw) register accessor: an alias for `Reg<PRESETCTRL_PRESETCTRL0_SPEC>`"]
pub type PRESETCTRL_PRESETCTRL0 = crate::Reg<presetctrl_presetctrl0::PRESETCTRL_PRESETCTRL0_SPEC>;
#[doc = "Peripheral reset control 0"]
pub mod presetctrl_presetctrl0;
#[doc = "PRESETCTRL_PRESETCTRLX0 (rw) register accessor: an alias for `Reg<PRESETCTRL_PRESETCTRLX0_SPEC>`"]
pub type PRESETCTRL_PRESETCTRLX0 =
    crate::Reg<presetctrl_presetctrlx0::PRESETCTRL_PRESETCTRLX0_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod presetctrl_presetctrlx0;
#[doc = "PRESETCTRL_PRESETCTRL1 (rw) register accessor: an alias for `Reg<PRESETCTRL_PRESETCTRL1_SPEC>`"]
pub type PRESETCTRL_PRESETCTRL1 = crate::Reg<presetctrl_presetctrl1::PRESETCTRL_PRESETCTRL1_SPEC>;
#[doc = "Peripheral reset control 1"]
pub mod presetctrl_presetctrl1;
#[doc = "PRESETCTRL_PRESETCTRLX1 (rw) register accessor: an alias for `Reg<PRESETCTRL_PRESETCTRLX1_SPEC>`"]
pub type PRESETCTRL_PRESETCTRLX1 =
    crate::Reg<presetctrl_presetctrlx1::PRESETCTRL_PRESETCTRLX1_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod presetctrl_presetctrlx1;
#[doc = "PRESETCTRL_PRESETCTRL2 (rw) register accessor: an alias for `Reg<PRESETCTRL_PRESETCTRL2_SPEC>`"]
pub type PRESETCTRL_PRESETCTRL2 = crate::Reg<presetctrl_presetctrl2::PRESETCTRL_PRESETCTRL2_SPEC>;
#[doc = "Peripheral reset control 2"]
pub mod presetctrl_presetctrl2;
#[doc = "PRESETCTRL_PRESETCTRLX2 (rw) register accessor: an alias for `Reg<PRESETCTRL_PRESETCTRLX2_SPEC>`"]
pub type PRESETCTRL_PRESETCTRLX2 =
    crate::Reg<presetctrl_presetctrlx2::PRESETCTRL_PRESETCTRLX2_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod presetctrl_presetctrlx2;
#[doc = "PRESETCTRLSET (rw) register accessor: an alias for `Reg<PRESETCTRLSET_SPEC>`"]
pub type PRESETCTRLSET = crate::Reg<presetctrlset::PRESETCTRLSET_SPEC>;
#[doc = "Peripheral reset control set register"]
pub mod presetctrlset;
#[doc = "PRESETCTRLCLR (rw) register accessor: an alias for `Reg<PRESETCTRLCLR_SPEC>`"]
pub type PRESETCTRLCLR = crate::Reg<presetctrlclr::PRESETCTRLCLR_SPEC>;
#[doc = "Peripheral reset control clear register"]
pub mod presetctrlclr;
#[doc = "SWR_RESET (w) register accessor: an alias for `Reg<SWR_RESET_SPEC>`"]
pub type SWR_RESET = crate::Reg<swr_reset::SWR_RESET_SPEC>;
#[doc = "generate a software_reset"]
pub mod swr_reset;
#[doc = "AHBCLKCTRL_AHBCLKCTRL0 (rw) register accessor: an alias for `Reg<AHBCLKCTRL_AHBCLKCTRL0_SPEC>`"]
pub type AHBCLKCTRL_AHBCLKCTRL0 = crate::Reg<ahbclkctrl_ahbclkctrl0::AHBCLKCTRL_AHBCLKCTRL0_SPEC>;
#[doc = "AHB Clock control 0"]
pub mod ahbclkctrl_ahbclkctrl0;
#[doc = "AHBCLKCTRL_AHBCLKCTRLX0 (rw) register accessor: an alias for `Reg<AHBCLKCTRL_AHBCLKCTRLX0_SPEC>`"]
pub type AHBCLKCTRL_AHBCLKCTRLX0 =
    crate::Reg<ahbclkctrl_ahbclkctrlx0::AHBCLKCTRL_AHBCLKCTRLX0_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrl_ahbclkctrlx0;
#[doc = "AHBCLKCTRL_AHBCLKCTRL1 (rw) register accessor: an alias for `Reg<AHBCLKCTRL_AHBCLKCTRL1_SPEC>`"]
pub type AHBCLKCTRL_AHBCLKCTRL1 = crate::Reg<ahbclkctrl_ahbclkctrl1::AHBCLKCTRL_AHBCLKCTRL1_SPEC>;
#[doc = "AHB Clock control 1"]
pub mod ahbclkctrl_ahbclkctrl1;
#[doc = "AHBCLKCTRL_AHBCLKCTRLX1 (rw) register accessor: an alias for `Reg<AHBCLKCTRL_AHBCLKCTRLX1_SPEC>`"]
pub type AHBCLKCTRL_AHBCLKCTRLX1 =
    crate::Reg<ahbclkctrl_ahbclkctrlx1::AHBCLKCTRL_AHBCLKCTRLX1_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrl_ahbclkctrlx1;
#[doc = "AHBCLKCTRL_AHBCLKCTRL2 (rw) register accessor: an alias for `Reg<AHBCLKCTRL_AHBCLKCTRL2_SPEC>`"]
pub type AHBCLKCTRL_AHBCLKCTRL2 = crate::Reg<ahbclkctrl_ahbclkctrl2::AHBCLKCTRL_AHBCLKCTRL2_SPEC>;
#[doc = "AHB Clock control 2"]
pub mod ahbclkctrl_ahbclkctrl2;
#[doc = "AHBCLKCTRL_AHBCLKCTRLX2 (rw) register accessor: an alias for `Reg<AHBCLKCTRL_AHBCLKCTRLX2_SPEC>`"]
pub type AHBCLKCTRL_AHBCLKCTRLX2 =
    crate::Reg<ahbclkctrl_ahbclkctrlx2::AHBCLKCTRL_AHBCLKCTRLX2_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrl_ahbclkctrlx2;
#[doc = "AHBCLKCTRLSET (rw) register accessor: an alias for `Reg<AHBCLKCTRLSET_SPEC>`"]
pub type AHBCLKCTRLSET = crate::Reg<ahbclkctrlset::AHBCLKCTRLSET_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlset;
#[doc = "AHBCLKCTRLCLR (rw) register accessor: an alias for `Reg<AHBCLKCTRLCLR_SPEC>`"]
pub type AHBCLKCTRLCLR = crate::Reg<ahbclkctrlclr::AHBCLKCTRLCLR_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ahbclkctrlclr;
#[doc = "SYSTICKCLKSEL_SYSTICKCLKSEL0 (rw) register accessor: an alias for `Reg<SYSTICKCLKSEL_SYSTICKCLKSEL0_SPEC>`"]
pub type SYSTICKCLKSEL_SYSTICKCLKSEL0 =
    crate::Reg<systickclksel_systickclksel0::SYSTICKCLKSEL_SYSTICKCLKSEL0_SPEC>;
#[doc = "System Tick Timer for CPU0 source select"]
pub mod systickclksel_systickclksel0;
#[doc = "SYSTICKCLKSEL_SYSTICKCLKSELX0 (rw) register accessor: an alias for `Reg<SYSTICKCLKSEL_SYSTICKCLKSELX0_SPEC>`"]
pub type SYSTICKCLKSEL_SYSTICKCLKSELX0 =
    crate::Reg<systickclksel_systickclkselx0::SYSTICKCLKSEL_SYSTICKCLKSELX0_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod systickclksel_systickclkselx0;
#[doc = "TRACECLKSEL (rw) register accessor: an alias for `Reg<TRACECLKSEL_SPEC>`"]
pub type TRACECLKSEL = crate::Reg<traceclksel::TRACECLKSEL_SPEC>;
#[doc = "Trace clock source select"]
pub mod traceclksel;
#[doc = "CTIMERCLKSEL_CTIMERCLKSEL0 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL_CTIMERCLKSEL0_SPEC>`"]
pub type CTIMERCLKSEL_CTIMERCLKSEL0 =
    crate::Reg<ctimerclksel_ctimerclksel0::CTIMERCLKSEL_CTIMERCLKSEL0_SPEC>;
#[doc = "CTimer 0 clock source select"]
pub mod ctimerclksel_ctimerclksel0;
#[doc = "CTIMERCLKSEL_CTIMERCLKSELX0 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL_CTIMERCLKSELX0_SPEC>`"]
pub type CTIMERCLKSEL_CTIMERCLKSELX0 =
    crate::Reg<ctimerclksel_ctimerclkselx0::CTIMERCLKSEL_CTIMERCLKSELX0_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclksel_ctimerclkselx0;
#[doc = "CTIMERCLKSEL_CTIMERCLKSEL1 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL_CTIMERCLKSEL1_SPEC>`"]
pub type CTIMERCLKSEL_CTIMERCLKSEL1 =
    crate::Reg<ctimerclksel_ctimerclksel1::CTIMERCLKSEL_CTIMERCLKSEL1_SPEC>;
#[doc = "CTimer 1 clock source select"]
pub mod ctimerclksel_ctimerclksel1;
#[doc = "CTIMERCLKSEL_CTIMERCLKSELX1 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL_CTIMERCLKSELX1_SPEC>`"]
pub type CTIMERCLKSEL_CTIMERCLKSELX1 =
    crate::Reg<ctimerclksel_ctimerclkselx1::CTIMERCLKSEL_CTIMERCLKSELX1_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclksel_ctimerclkselx1;
#[doc = "CTIMERCLKSEL_CTIMERCLKSEL2 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL_CTIMERCLKSEL2_SPEC>`"]
pub type CTIMERCLKSEL_CTIMERCLKSEL2 =
    crate::Reg<ctimerclksel_ctimerclksel2::CTIMERCLKSEL_CTIMERCLKSEL2_SPEC>;
#[doc = "CTimer 2 clock source select"]
pub mod ctimerclksel_ctimerclksel2;
#[doc = "CTIMERCLKSEL_CTIMERCLKSELX2 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL_CTIMERCLKSELX2_SPEC>`"]
pub type CTIMERCLKSEL_CTIMERCLKSELX2 =
    crate::Reg<ctimerclksel_ctimerclkselx2::CTIMERCLKSEL_CTIMERCLKSELX2_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclksel_ctimerclkselx2;
#[doc = "CTIMERCLKSEL_CTIMERCLKSEL3 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL_CTIMERCLKSEL3_SPEC>`"]
pub type CTIMERCLKSEL_CTIMERCLKSEL3 =
    crate::Reg<ctimerclksel_ctimerclksel3::CTIMERCLKSEL_CTIMERCLKSEL3_SPEC>;
#[doc = "CTimer 3 clock source select"]
pub mod ctimerclksel_ctimerclksel3;
#[doc = "CTIMERCLKSEL_CTIMERCLKSELX3 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL_CTIMERCLKSELX3_SPEC>`"]
pub type CTIMERCLKSEL_CTIMERCLKSELX3 =
    crate::Reg<ctimerclksel_ctimerclkselx3::CTIMERCLKSEL_CTIMERCLKSELX3_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclksel_ctimerclkselx3;
#[doc = "CTIMERCLKSEL_CTIMERCLKSEL4 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL_CTIMERCLKSEL4_SPEC>`"]
pub type CTIMERCLKSEL_CTIMERCLKSEL4 =
    crate::Reg<ctimerclksel_ctimerclksel4::CTIMERCLKSEL_CTIMERCLKSEL4_SPEC>;
#[doc = "CTimer 4 clock source select"]
pub mod ctimerclksel_ctimerclksel4;
#[doc = "CTIMERCLKSEL_CTIMERCLKSELX4 (rw) register accessor: an alias for `Reg<CTIMERCLKSEL_CTIMERCLKSELX4_SPEC>`"]
pub type CTIMERCLKSEL_CTIMERCLKSELX4 =
    crate::Reg<ctimerclksel_ctimerclkselx4::CTIMERCLKSEL_CTIMERCLKSELX4_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod ctimerclksel_ctimerclkselx4;
#[doc = "MAINCLKSELA (rw) register accessor: an alias for `Reg<MAINCLKSELA_SPEC>`"]
pub type MAINCLKSELA = crate::Reg<mainclksela::MAINCLKSELA_SPEC>;
#[doc = "Main clock A source select"]
pub mod mainclksela;
#[doc = "MAINCLKSELB (rw) register accessor: an alias for `Reg<MAINCLKSELB_SPEC>`"]
pub type MAINCLKSELB = crate::Reg<mainclkselb::MAINCLKSELB_SPEC>;
#[doc = "Main clock source select"]
pub mod mainclkselb;
#[doc = "CLKOUTSEL (rw) register accessor: an alias for `Reg<CLKOUTSEL_SPEC>`"]
pub type CLKOUTSEL = crate::Reg<clkoutsel::CLKOUTSEL_SPEC>;
#[doc = "CLKOUT clock source select"]
pub mod clkoutsel;
#[doc = "PLL0CLKSEL (rw) register accessor: an alias for `Reg<PLL0CLKSEL_SPEC>`"]
pub type PLL0CLKSEL = crate::Reg<pll0clksel::PLL0CLKSEL_SPEC>;
#[doc = "PLL0 clock source select"]
pub mod pll0clksel;
#[doc = "PLL1CLKSEL (rw) register accessor: an alias for `Reg<PLL1CLKSEL_SPEC>`"]
pub type PLL1CLKSEL = crate::Reg<pll1clksel::PLL1CLKSEL_SPEC>;
#[doc = "PLL1 clock source select"]
pub mod pll1clksel;
#[doc = "ADCCLKSEL (rw) register accessor: an alias for `Reg<ADCCLKSEL_SPEC>`"]
pub type ADCCLKSEL = crate::Reg<adcclksel::ADCCLKSEL_SPEC>;
#[doc = "ADC clock source select"]
pub mod adcclksel;
#[doc = "USB0CLKSEL (rw) register accessor: an alias for `Reg<USB0CLKSEL_SPEC>`"]
pub type USB0CLKSEL = crate::Reg<usb0clksel::USB0CLKSEL_SPEC>;
#[doc = "FS USB clock source select"]
pub mod usb0clksel;
#[doc = "FCCLKSEL_FCCLKSEL0 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSEL0_SPEC>`"]
pub type FCCLKSEL_FCCLKSEL0 = crate::Reg<fcclksel_fcclksel0::FCCLKSEL_FCCLKSEL0_SPEC>;
#[doc = "Flexcomm Interface 0 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel0;
#[doc = "FCCLKSEL_FCCLKSELX0 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSELX0_SPEC>`"]
pub type FCCLKSEL_FCCLKSELX0 = crate::Reg<fcclksel_fcclkselx0::FCCLKSEL_FCCLKSELX0_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx0;
#[doc = "FCCLKSEL_FCCLKSEL1 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSEL1_SPEC>`"]
pub type FCCLKSEL_FCCLKSEL1 = crate::Reg<fcclksel_fcclksel1::FCCLKSEL_FCCLKSEL1_SPEC>;
#[doc = "Flexcomm Interface 1 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel1;
#[doc = "FCCLKSEL_FCCLKSELX1 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSELX1_SPEC>`"]
pub type FCCLKSEL_FCCLKSELX1 = crate::Reg<fcclksel_fcclkselx1::FCCLKSEL_FCCLKSELX1_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx1;
#[doc = "FCCLKSEL_FCCLKSEL2 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSEL2_SPEC>`"]
pub type FCCLKSEL_FCCLKSEL2 = crate::Reg<fcclksel_fcclksel2::FCCLKSEL_FCCLKSEL2_SPEC>;
#[doc = "Flexcomm Interface 2 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel2;
#[doc = "FCCLKSEL_FCCLKSELX2 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSELX2_SPEC>`"]
pub type FCCLKSEL_FCCLKSELX2 = crate::Reg<fcclksel_fcclkselx2::FCCLKSEL_FCCLKSELX2_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx2;
#[doc = "FCCLKSEL_FCCLKSEL3 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSEL3_SPEC>`"]
pub type FCCLKSEL_FCCLKSEL3 = crate::Reg<fcclksel_fcclksel3::FCCLKSEL_FCCLKSEL3_SPEC>;
#[doc = "Flexcomm Interface 3 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel3;
#[doc = "FCCLKSEL_FCCLKSELX3 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSELX3_SPEC>`"]
pub type FCCLKSEL_FCCLKSELX3 = crate::Reg<fcclksel_fcclkselx3::FCCLKSEL_FCCLKSELX3_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx3;
#[doc = "FCCLKSEL_FCCLKSEL4 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSEL4_SPEC>`"]
pub type FCCLKSEL_FCCLKSEL4 = crate::Reg<fcclksel_fcclksel4::FCCLKSEL_FCCLKSEL4_SPEC>;
#[doc = "Flexcomm Interface 4 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel4;
#[doc = "FCCLKSEL_FCCLKSELX4 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSELX4_SPEC>`"]
pub type FCCLKSEL_FCCLKSELX4 = crate::Reg<fcclksel_fcclkselx4::FCCLKSEL_FCCLKSELX4_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx4;
#[doc = "FCCLKSEL_FCCLKSEL5 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSEL5_SPEC>`"]
pub type FCCLKSEL_FCCLKSEL5 = crate::Reg<fcclksel_fcclksel5::FCCLKSEL_FCCLKSEL5_SPEC>;
#[doc = "Flexcomm Interface 5 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel5;
#[doc = "FCCLKSEL_FCCLKSELX5 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSELX5_SPEC>`"]
pub type FCCLKSEL_FCCLKSELX5 = crate::Reg<fcclksel_fcclkselx5::FCCLKSEL_FCCLKSELX5_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx5;
#[doc = "FCCLKSEL_FCCLKSEL6 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSEL6_SPEC>`"]
pub type FCCLKSEL_FCCLKSEL6 = crate::Reg<fcclksel_fcclksel6::FCCLKSEL_FCCLKSEL6_SPEC>;
#[doc = "Flexcomm Interface 6 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel6;
#[doc = "FCCLKSEL_FCCLKSELX6 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSELX6_SPEC>`"]
pub type FCCLKSEL_FCCLKSELX6 = crate::Reg<fcclksel_fcclkselx6::FCCLKSEL_FCCLKSELX6_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx6;
#[doc = "FCCLKSEL_FCCLKSEL7 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSEL7_SPEC>`"]
pub type FCCLKSEL_FCCLKSEL7 = crate::Reg<fcclksel_fcclksel7::FCCLKSEL_FCCLKSEL7_SPEC>;
#[doc = "Flexcomm Interface 7 clock source select for Fractional Rate Divider"]
pub mod fcclksel_fcclksel7;
#[doc = "FCCLKSEL_FCCLKSELX7 (rw) register accessor: an alias for `Reg<FCCLKSEL_FCCLKSELX7_SPEC>`"]
pub type FCCLKSEL_FCCLKSELX7 = crate::Reg<fcclksel_fcclkselx7::FCCLKSEL_FCCLKSELX7_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod fcclksel_fcclkselx7;
#[doc = "HSLSPICLKSEL (rw) register accessor: an alias for `Reg<HSLSPICLKSEL_SPEC>`"]
pub type HSLSPICLKSEL = crate::Reg<hslspiclksel::HSLSPICLKSEL_SPEC>;
#[doc = "HS LSPI clock source select"]
pub mod hslspiclksel;
#[doc = "MCLKCLKSEL (rw) register accessor: an alias for `Reg<MCLKCLKSEL_SPEC>`"]
pub type MCLKCLKSEL = crate::Reg<mclkclksel::MCLKCLKSEL_SPEC>;
#[doc = "MCLK clock source select"]
pub mod mclkclksel;
#[doc = "SCTCLKSEL (rw) register accessor: an alias for `Reg<SCTCLKSEL_SPEC>`"]
pub type SCTCLKSEL = crate::Reg<sctclksel::SCTCLKSEL_SPEC>;
#[doc = "SCTimer/PWM clock source select"]
pub mod sctclksel;
#[doc = "SDIOCLKSEL (rw) register accessor: an alias for `Reg<SDIOCLKSEL_SPEC>`"]
pub type SDIOCLKSEL = crate::Reg<sdioclksel::SDIOCLKSEL_SPEC>;
#[doc = "SDIO clock source select"]
pub mod sdioclksel;
#[doc = "SYSTICKCLKDIV0 (rw) register accessor: an alias for `Reg<SYSTICKCLKDIV0_SPEC>`"]
pub type SYSTICKCLKDIV0 = crate::Reg<systickclkdiv0::SYSTICKCLKDIV0_SPEC>;
#[doc = "System Tick Timer divider for CPU0"]
pub mod systickclkdiv0;
#[doc = "SYSTICKCLKDIV1 (rw) register accessor: an alias for `Reg<SYSTICKCLKDIV1_SPEC>`"]
pub type SYSTICKCLKDIV1 = crate::Reg<systickclkdiv1::SYSTICKCLKDIV1_SPEC>;
#[doc = "System Tick Timer divider for CPU1"]
pub mod systickclkdiv1;
#[doc = "TRACECLKDIV (rw) register accessor: an alias for `Reg<TRACECLKDIV_SPEC>`"]
pub type TRACECLKDIV = crate::Reg<traceclkdiv::TRACECLKDIV_SPEC>;
#[doc = "TRACE clock divider"]
pub mod traceclkdiv;
#[doc = "FLEXFRGCTRL_FLEXFRG0CTRL (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRG0CTRL_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRG0CTRL =
    crate::Reg<flexfrgctrl_flexfrg0ctrl::FLEXFRGCTRL_FLEXFRG0CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 0"]
pub mod flexfrgctrl_flexfrg0ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL0 (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRGXCTRL0_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRGXCTRL0 =
    crate::Reg<flexfrgctrl_flexfrgxctrl0::FLEXFRGCTRL_FLEXFRGXCTRL0_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl0;
#[doc = "FLEXFRGCTRL_FLEXFRG1CTRL (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRG1CTRL_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRG1CTRL =
    crate::Reg<flexfrgctrl_flexfrg1ctrl::FLEXFRGCTRL_FLEXFRG1CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 1"]
pub mod flexfrgctrl_flexfrg1ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL1 (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRGXCTRL1_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRGXCTRL1 =
    crate::Reg<flexfrgctrl_flexfrgxctrl1::FLEXFRGCTRL_FLEXFRGXCTRL1_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl1;
#[doc = "FLEXFRGCTRL_FLEXFRG2CTRL (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRG2CTRL_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRG2CTRL =
    crate::Reg<flexfrgctrl_flexfrg2ctrl::FLEXFRGCTRL_FLEXFRG2CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 2"]
pub mod flexfrgctrl_flexfrg2ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL2 (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRGXCTRL2_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRGXCTRL2 =
    crate::Reg<flexfrgctrl_flexfrgxctrl2::FLEXFRGCTRL_FLEXFRGXCTRL2_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl2;
#[doc = "FLEXFRGCTRL_FLEXFRG3CTRL (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRG3CTRL_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRG3CTRL =
    crate::Reg<flexfrgctrl_flexfrg3ctrl::FLEXFRGCTRL_FLEXFRG3CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 3"]
pub mod flexfrgctrl_flexfrg3ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL3 (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRGXCTRL3_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRGXCTRL3 =
    crate::Reg<flexfrgctrl_flexfrgxctrl3::FLEXFRGCTRL_FLEXFRGXCTRL3_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl3;
#[doc = "FLEXFRGCTRL_FLEXFRG4CTRL (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRG4CTRL_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRG4CTRL =
    crate::Reg<flexfrgctrl_flexfrg4ctrl::FLEXFRGCTRL_FLEXFRG4CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 4"]
pub mod flexfrgctrl_flexfrg4ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL4 (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRGXCTRL4_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRGXCTRL4 =
    crate::Reg<flexfrgctrl_flexfrgxctrl4::FLEXFRGCTRL_FLEXFRGXCTRL4_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl4;
#[doc = "FLEXFRGCTRL_FLEXFRG5CTRL (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRG5CTRL_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRG5CTRL =
    crate::Reg<flexfrgctrl_flexfrg5ctrl::FLEXFRGCTRL_FLEXFRG5CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 5"]
pub mod flexfrgctrl_flexfrg5ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL5 (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRGXCTRL5_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRGXCTRL5 =
    crate::Reg<flexfrgctrl_flexfrgxctrl5::FLEXFRGCTRL_FLEXFRGXCTRL5_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl5;
#[doc = "FLEXFRGCTRL_FLEXFRG6CTRL (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRG6CTRL_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRG6CTRL =
    crate::Reg<flexfrgctrl_flexfrg6ctrl::FLEXFRGCTRL_FLEXFRG6CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 6"]
pub mod flexfrgctrl_flexfrg6ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL6 (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRGXCTRL6_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRGXCTRL6 =
    crate::Reg<flexfrgctrl_flexfrgxctrl6::FLEXFRGCTRL_FLEXFRGXCTRL6_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl6;
#[doc = "FLEXFRGCTRL_FLEXFRG7CTRL (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRG7CTRL_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRG7CTRL =
    crate::Reg<flexfrgctrl_flexfrg7ctrl::FLEXFRGCTRL_FLEXFRG7CTRL_SPEC>;
#[doc = "Fractional rate divider for flexcomm 7"]
pub mod flexfrgctrl_flexfrg7ctrl;
#[doc = "FLEXFRGCTRL_FLEXFRGXCTRL7 (rw) register accessor: an alias for `Reg<FLEXFRGCTRL_FLEXFRGXCTRL7_SPEC>`"]
pub type FLEXFRGCTRL_FLEXFRGXCTRL7 =
    crate::Reg<flexfrgctrl_flexfrgxctrl7::FLEXFRGCTRL_FLEXFRGXCTRL7_SPEC>;
#[doc = "Peripheral reset control register"]
pub mod flexfrgctrl_flexfrgxctrl7;
#[doc = "AHBCLKDIV (rw) register accessor: an alias for `Reg<AHBCLKDIV_SPEC>`"]
pub type AHBCLKDIV = crate::Reg<ahbclkdiv::AHBCLKDIV_SPEC>;
#[doc = "System clock divider"]
pub mod ahbclkdiv;
#[doc = "CLKOUTDIV (rw) register accessor: an alias for `Reg<CLKOUTDIV_SPEC>`"]
pub type CLKOUTDIV = crate::Reg<clkoutdiv::CLKOUTDIV_SPEC>;
#[doc = "CLKOUT clock divider"]
pub mod clkoutdiv;
#[doc = "FROHFDIV (rw) register accessor: an alias for `Reg<FROHFDIV_SPEC>`"]
pub type FROHFDIV = crate::Reg<frohfdiv::FROHFDIV_SPEC>;
#[doc = "FRO_HF (96MHz) clock divider"]
pub mod frohfdiv;
#[doc = "WDTCLKDIV (rw) register accessor: an alias for `Reg<WDTCLKDIV_SPEC>`"]
pub type WDTCLKDIV = crate::Reg<wdtclkdiv::WDTCLKDIV_SPEC>;
#[doc = "WDT clock divider"]
pub mod wdtclkdiv;
#[doc = "ADCCLKDIV (rw) register accessor: an alias for `Reg<ADCCLKDIV_SPEC>`"]
pub type ADCCLKDIV = crate::Reg<adcclkdiv::ADCCLKDIV_SPEC>;
#[doc = "ADC clock divider"]
pub mod adcclkdiv;
#[doc = "USB0CLKDIV (rw) register accessor: an alias for `Reg<USB0CLKDIV_SPEC>`"]
pub type USB0CLKDIV = crate::Reg<usb0clkdiv::USB0CLKDIV_SPEC>;
#[doc = "USB0 Clock divider"]
pub mod usb0clkdiv;
#[doc = "MCLKDIV (rw) register accessor: an alias for `Reg<MCLKDIV_SPEC>`"]
pub type MCLKDIV = crate::Reg<mclkdiv::MCLKDIV_SPEC>;
#[doc = "I2S MCLK clock divider"]
pub mod mclkdiv;
#[doc = "SCTCLKDIV (rw) register accessor: an alias for `Reg<SCTCLKDIV_SPEC>`"]
pub type SCTCLKDIV = crate::Reg<sctclkdiv::SCTCLKDIV_SPEC>;
#[doc = "SCT/PWM clock divider"]
pub mod sctclkdiv;
#[doc = "SDIOCLKDIV (rw) register accessor: an alias for `Reg<SDIOCLKDIV_SPEC>`"]
pub type SDIOCLKDIV = crate::Reg<sdioclkdiv::SDIOCLKDIV_SPEC>;
#[doc = "SDIO clock divider"]
pub mod sdioclkdiv;
#[doc = "PLL0CLKDIV (rw) register accessor: an alias for `Reg<PLL0CLKDIV_SPEC>`"]
pub type PLL0CLKDIV = crate::Reg<pll0clkdiv::PLL0CLKDIV_SPEC>;
#[doc = "PLL0 clock divider"]
pub mod pll0clkdiv;
#[doc = "CLOCKGENUPDATELOCKOUT (rw) register accessor: an alias for `Reg<CLOCKGENUPDATELOCKOUT_SPEC>`"]
pub type CLOCKGENUPDATELOCKOUT = crate::Reg<clockgenupdatelockout::CLOCKGENUPDATELOCKOUT_SPEC>;
#[doc = "Control clock configuration registers access (like xxxDIV, xxxSEL)"]
pub mod clockgenupdatelockout;
#[doc = "FMCCR (rw) register accessor: an alias for `Reg<FMCCR_SPEC>`"]
pub type FMCCR = crate::Reg<fmccr::FMCCR_SPEC>;
#[doc = "FMC configuration register"]
pub mod fmccr;
#[doc = "USB0NEEDCLKCTRL (rw) register accessor: an alias for `Reg<USB0NEEDCLKCTRL_SPEC>`"]
pub type USB0NEEDCLKCTRL = crate::Reg<usb0needclkctrl::USB0NEEDCLKCTRL_SPEC>;
#[doc = "USB0 need clock control"]
pub mod usb0needclkctrl;
#[doc = "USB0NEEDCLKSTAT (rw) register accessor: an alias for `Reg<USB0NEEDCLKSTAT_SPEC>`"]
pub type USB0NEEDCLKSTAT = crate::Reg<usb0needclkstat::USB0NEEDCLKSTAT_SPEC>;
#[doc = "USB0 need clock status"]
pub mod usb0needclkstat;
#[doc = "FMCFLUSH (w) register accessor: an alias for `Reg<FMCFLUSH_SPEC>`"]
pub type FMCFLUSH = crate::Reg<fmcflush::FMCFLUSH_SPEC>;
#[doc = "FMCflush control"]
pub mod fmcflush;
#[doc = "MCLKIO (rw) register accessor: an alias for `Reg<MCLKIO_SPEC>`"]
pub type MCLKIO = crate::Reg<mclkio::MCLKIO_SPEC>;
#[doc = "MCLK control"]
pub mod mclkio;
#[doc = "USB1NEEDCLKCTRL (rw) register accessor: an alias for `Reg<USB1NEEDCLKCTRL_SPEC>`"]
pub type USB1NEEDCLKCTRL = crate::Reg<usb1needclkctrl::USB1NEEDCLKCTRL_SPEC>;
#[doc = "USB1 need clock control"]
pub mod usb1needclkctrl;
#[doc = "USB1NEEDCLKSTAT (rw) register accessor: an alias for `Reg<USB1NEEDCLKSTAT_SPEC>`"]
pub type USB1NEEDCLKSTAT = crate::Reg<usb1needclkstat::USB1NEEDCLKSTAT_SPEC>;
#[doc = "USB1 need clock status"]
pub mod usb1needclkstat;
#[doc = "SDIOCLKCTRL (rw) register accessor: an alias for `Reg<SDIOCLKCTRL_SPEC>`"]
pub type SDIOCLKCTRL = crate::Reg<sdioclkctrl::SDIOCLKCTRL_SPEC>;
#[doc = "SDIO CCLKIN phase and delay control"]
pub mod sdioclkctrl;
#[doc = "PLL1CTRL (rw) register accessor: an alias for `Reg<PLL1CTRL_SPEC>`"]
pub type PLL1CTRL = crate::Reg<pll1ctrl::PLL1CTRL_SPEC>;
#[doc = "PLL1 550m control"]
pub mod pll1ctrl;
#[doc = "PLL1STAT (rw) register accessor: an alias for `Reg<PLL1STAT_SPEC>`"]
pub type PLL1STAT = crate::Reg<pll1stat::PLL1STAT_SPEC>;
#[doc = "PLL1 550m status"]
pub mod pll1stat;
#[doc = "PLL1NDEC (rw) register accessor: an alias for `Reg<PLL1NDEC_SPEC>`"]
pub type PLL1NDEC = crate::Reg<pll1ndec::PLL1NDEC_SPEC>;
#[doc = "PLL1 550m N divider"]
pub mod pll1ndec;
#[doc = "PLL1MDEC (rw) register accessor: an alias for `Reg<PLL1MDEC_SPEC>`"]
pub type PLL1MDEC = crate::Reg<pll1mdec::PLL1MDEC_SPEC>;
#[doc = "PLL1 550m M divider"]
pub mod pll1mdec;
#[doc = "PLL1PDEC (rw) register accessor: an alias for `Reg<PLL1PDEC_SPEC>`"]
pub type PLL1PDEC = crate::Reg<pll1pdec::PLL1PDEC_SPEC>;
#[doc = "PLL1 550m P divider"]
pub mod pll1pdec;
#[doc = "PLL0CTRL (rw) register accessor: an alias for `Reg<PLL0CTRL_SPEC>`"]
pub type PLL0CTRL = crate::Reg<pll0ctrl::PLL0CTRL_SPEC>;
#[doc = "PLL0 550m control"]
pub mod pll0ctrl;
#[doc = "PLL0STAT (rw) register accessor: an alias for `Reg<PLL0STAT_SPEC>`"]
pub type PLL0STAT = crate::Reg<pll0stat::PLL0STAT_SPEC>;
#[doc = "PLL0 550m status"]
pub mod pll0stat;
#[doc = "PLL0NDEC (rw) register accessor: an alias for `Reg<PLL0NDEC_SPEC>`"]
pub type PLL0NDEC = crate::Reg<pll0ndec::PLL0NDEC_SPEC>;
#[doc = "PLL0 550m N divider"]
pub mod pll0ndec;
#[doc = "PLL0PDEC (rw) register accessor: an alias for `Reg<PLL0PDEC_SPEC>`"]
pub type PLL0PDEC = crate::Reg<pll0pdec::PLL0PDEC_SPEC>;
#[doc = "PLL0 550m P divider"]
pub mod pll0pdec;
#[doc = "PLL0SSCG0 (rw) register accessor: an alias for `Reg<PLL0SSCG0_SPEC>`"]
pub type PLL0SSCG0 = crate::Reg<pll0sscg0::PLL0SSCG0_SPEC>;
#[doc = "PLL0 Spread Spectrum Wrapper control register 0"]
pub mod pll0sscg0;
#[doc = "PLL0SSCG1 (rw) register accessor: an alias for `Reg<PLL0SSCG1_SPEC>`"]
pub type PLL0SSCG1 = crate::Reg<pll0sscg1::PLL0SSCG1_SPEC>;
#[doc = "PLL0 Spread Spectrum Wrapper control register 1"]
pub mod pll0sscg1;
#[doc = "CPUCTRL (rw) register accessor: an alias for `Reg<CPUCTRL_SPEC>`"]
pub type CPUCTRL = crate::Reg<cpuctrl::CPUCTRL_SPEC>;
#[doc = "CPU Control for multiple processors"]
pub mod cpuctrl;
#[doc = "CPBOOT (rw) register accessor: an alias for `Reg<CPBOOT_SPEC>`"]
pub type CPBOOT = crate::Reg<cpboot::CPBOOT_SPEC>;
#[doc = "Coprocessor Boot Address"]
pub mod cpboot;
#[doc = "CPSTAT (rw) register accessor: an alias for `Reg<CPSTAT_SPEC>`"]
pub type CPSTAT = crate::Reg<cpstat::CPSTAT_SPEC>;
#[doc = "CPU Status"]
pub mod cpstat;
#[doc = "CLOCK_CTRL (rw) register accessor: an alias for `Reg<CLOCK_CTRL_SPEC>`"]
pub type CLOCK_CTRL = crate::Reg<clock_ctrl::CLOCK_CTRL_SPEC>;
#[doc = "Various system clock controls : Flash clock (48 MHz) control, clocks to Frequency Measures"]
pub mod clock_ctrl;
#[doc = "COMP_INT_CTRL (rw) register accessor: an alias for `Reg<COMP_INT_CTRL_SPEC>`"]
pub type COMP_INT_CTRL = crate::Reg<comp_int_ctrl::COMP_INT_CTRL_SPEC>;
#[doc = "Comparator Interrupt control"]
pub mod comp_int_ctrl;
#[doc = "COMP_INT_STATUS (rw) register accessor: an alias for `Reg<COMP_INT_STATUS_SPEC>`"]
pub type COMP_INT_STATUS = crate::Reg<comp_int_status::COMP_INT_STATUS_SPEC>;
#[doc = "Comparator Interrupt status"]
pub mod comp_int_status;
#[doc = "AUTOCLKGATEOVERRIDE (rw) register accessor: an alias for `Reg<AUTOCLKGATEOVERRIDE_SPEC>`"]
pub type AUTOCLKGATEOVERRIDE = crate::Reg<autoclkgateoverride::AUTOCLKGATEOVERRIDE_SPEC>;
#[doc = "Control automatic clock gating"]
pub mod autoclkgateoverride;
#[doc = "GPIOPSYNC (rw) register accessor: an alias for `Reg<GPIOPSYNC_SPEC>`"]
pub type GPIOPSYNC = crate::Reg<gpiopsync::GPIOPSYNC_SPEC>;
#[doc = "Enable bypass of the first stage of synchonization inside GPIO_INT module"]
pub mod gpiopsync;
#[doc = "DEBUG_LOCK_EN (rw) register accessor: an alias for `Reg<DEBUG_LOCK_EN_SPEC>`"]
pub type DEBUG_LOCK_EN = crate::Reg<debug_lock_en::DEBUG_LOCK_EN_SPEC>;
#[doc = "Control write access to security registers."]
pub mod debug_lock_en;
#[doc = "DEBUG_FEATURES (rw) register accessor: an alias for `Reg<DEBUG_FEATURES_SPEC>`"]
pub type DEBUG_FEATURES = crate::Reg<debug_features::DEBUG_FEATURES_SPEC>;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control."]
pub mod debug_features;
#[doc = "DEBUG_FEATURES_DP (rw) register accessor: an alias for `Reg<DEBUG_FEATURES_DP_SPEC>`"]
pub type DEBUG_FEATURES_DP = crate::Reg<debug_features_dp::DEBUG_FEATURES_DP_SPEC>;
#[doc = "Cortex M33 (CPU0) and micro Cortex M33 (CPU1) debug features control DUPLICATE register."]
pub mod debug_features_dp;
#[doc = "KEY_BLOCK (w) register accessor: an alias for `Reg<KEY_BLOCK_SPEC>`"]
pub type KEY_BLOCK = crate::Reg<key_block::KEY_BLOCK_SPEC>;
#[doc = "block quiddikey/PUF all index."]
pub mod key_block;
#[doc = "DEBUG_AUTH_BEACON (rw) register accessor: an alias for `Reg<DEBUG_AUTH_BEACON_SPEC>`"]
pub type DEBUG_AUTH_BEACON = crate::Reg<debug_auth_beacon::DEBUG_AUTH_BEACON_SPEC>;
#[doc = "Debug authentication BEACON register"]
pub mod debug_auth_beacon;
#[doc = "DEVICE_ID0 (r) register accessor: an alias for `Reg<DEVICE_ID0_SPEC>`"]
pub type DEVICE_ID0 = crate::Reg<device_id0::DEVICE_ID0_SPEC>;
#[doc = "Device ID"]
pub mod device_id0;
#[doc = "DIEID (r) register accessor: an alias for `Reg<DIEID_SPEC>`"]
pub type DIEID = crate::Reg<dieid::DIEID_SPEC>;
#[doc = "Chip revision ID and Number"]
pub mod dieid;
