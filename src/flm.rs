#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    flm_cfg: FlmCfg,
    flm_stat: FlmStat,
    flm_log120: FlmLog120,
    flm_log121: FlmLog121,
    flm_ie: FlmIe,
    flm_ctl: FlmCtl,
    _reserved6: [u8; 0x08],
    flm_rang0: FlmRang0,
    flm_rang1: FlmRang1,
    flm_rang2: FlmRang2,
    flm_rang3: FlmRang3,
    flm_rang4: FlmRang4,
    flm_rang5: FlmRang5,
    flm_rang6: FlmRang6,
    flm_rang7: FlmRang7,
    flm_rang8: FlmRang8,
    flm_rang9: FlmRang9,
    flm_rang10: FlmRang10,
    flm_rang11: FlmRang11,
    flm_rang12: FlmRang12,
    flm_rang13: FlmRang13,
    flm_rang14: FlmRang14,
    flm_rang15: FlmRang15,
    flm_cmden: FlmCmden,
    flm_cmben: FlmCmben,
    _reserved24: [u8; 0x18],
    flm_cmd0: FlmCmd0,
    flm_cmd1: FlmCmd1,
    flm_cmd2: FlmCmd2,
    flm_cmd3: FlmCmd3,
    flm_cmd4: FlmCmd4,
    flm_cmd5: FlmCmd5,
    flm_cmd6: FlmCmd6,
    flm_cmd7: FlmCmd7,
    flm_cmd8: FlmCmd8,
    flm_cmd9: FlmCmd9,
    flm_cmd10: FlmCmd10,
    flm_cmd11: FlmCmd11,
    _reserved_36_flm_cmd1: [u8; 0x04],
    _reserved_37_flm_cmd1: [u8; 0x04],
    _reserved38: [u8; 0x08],
    flm_cmd18: FlmCmd18,
    flm_cmd19: FlmCmd19,
    flm_cmd16: FlmCmd16,
    flm_cmd17: FlmCmd17,
    flm_cmd22: FlmCmd22,
    flm_cmd23: FlmCmd23,
    flm_cmd20: FlmCmd20,
    flm_cmd21: FlmCmd21,
    flm_cmd24: FlmCmd24,
    flm_cmd25: FlmCmd25,
    flm_cmd26: FlmCmd26,
    flm_cmd27: FlmCmd27,
    flm_cmd28: FlmCmd28,
    flm_cmd29: FlmCmd29,
    flm_cmd30: FlmCmd30,
    flm_cmd31: FlmCmd31,
    flm_tcra0: FlmTcra0,
    flm_tcra1: FlmTcra1,
    flm_tcra2: FlmTcra2,
    flm_tcra3: FlmTcra3,
    flm_tcrb0: FlmTcrb0,
    flm_tcrb1: FlmTcrb1,
    flm_tcrb2: FlmTcrb2,
    flm_tcrb3: FlmTcrb3,
    _reserved62: [u8; 0x20],
    flm_tcca0: FlmTcca0,
    flm_tcca1: FlmTcca1,
    flm_tcca2: FlmTcca2,
    flm_tcca3: FlmTcca3,
    flm_tccb0: FlmTccb0,
    flm_tccb1: FlmTccb1,
    flm_tccb2: FlmTccb2,
    flm_tccb3: FlmTccb3,
    _reserved70: [u8; 0x20],
    flm_cmdev: FlmCmdev,
    flm_cmbev: FlmCmbev,
    flm_tcgc: FlmTcgc,
    _reserved73: [u8; 0x34],
    flm_cq030: FlmCq030,
    flm_cq031: FlmCq031,
    flm_cq032: FlmCq032,
    flm_cq033: FlmCq033,
    _reserved77: [u8; 0x30],
    flm_cmdars0310: FlmCmdars0310,
    flm_cmdars0311: FlmCmdars0311,
    flm_cmdars0312: FlmCmdars0312,
    flm_cmdars0313: FlmCmdars0313,
    flm_cmdars0314: FlmCmdars0314,
    flm_cmdars0315: FlmCmdars0315,
    flm_cmdars0316: FlmCmdars0316,
    flm_cmdars0317: FlmCmdars0317,
    flm_cmdars0318: FlmCmdars0318,
    flm_cmdars0319: FlmCmdars0319,
    flm_cmdars03110: FlmCmdars03110,
    flm_cmdars03111: FlmCmdars03111,
    _reserved_89_flm_cmdars0311: [u8; 0x04],
    _reserved_90_flm_cmdars0311: [u8; 0x04],
    _reserved91: [u8; 0x08],
    flm_cmdars03118: FlmCmdars03118,
    flm_cmdars03119: FlmCmdars03119,
    flm_cmdars03116: FlmCmdars03116,
    flm_cmdars03117: FlmCmdars03117,
    flm_cmdars03122: FlmCmdars03122,
    flm_cmdars03123: FlmCmdars03123,
    flm_cmdars03120: FlmCmdars03120,
    flm_cmdars03121: FlmCmdars03121,
    flm_cmdars03124: FlmCmdars03124,
    flm_cmdars03125: FlmCmdars03125,
    flm_cmdars03126: FlmCmdars03126,
    flm_cmdars03127: FlmCmdars03127,
    flm_cmdars03128: FlmCmdars03128,
    flm_cmdars03129: FlmCmdars03129,
    flm_cmdars03130: FlmCmdars03130,
    flm_cmdars03131: FlmCmdars03131,
    flm_cmb0310: FlmCmb0310,
    flm_cmb0311: FlmCmb0311,
    flm_cmb0312: FlmCmb0312,
    flm_cmb0313: FlmCmb0313,
    flm_cmb0314: FlmCmb0314,
    flm_cmb0315: FlmCmb0315,
    flm_cmb0316: FlmCmb0316,
    flm_cmb0317: FlmCmb0317,
    flm_cmb0318: FlmCmb0318,
    flm_cmb0319: FlmCmb0319,
    flm_cmb03110: FlmCmb03110,
    flm_cmb03111: FlmCmb03111,
    _reserved_119_flm_cmb0311: [u8; 0x04],
    _reserved_120_flm_cmb0311: [u8; 0x04],
    _reserved121: [u8; 0x08],
    flm_cmb03118: FlmCmb03118,
    flm_cmb03119: FlmCmb03119,
    flm_cmb03116: FlmCmb03116,
    flm_cmb03117: FlmCmb03117,
    flm_cmb03122: FlmCmb03122,
    flm_cmb03123: FlmCmb03123,
    flm_cmb03120: FlmCmb03120,
    flm_cmb03121: FlmCmb03121,
    flm_cmb03124: FlmCmb03124,
    flm_cmb03125: FlmCmb03125,
    flm_cmb03126: FlmCmb03126,
    flm_cmb03127: FlmCmb03127,
    flm_cmb03128: FlmCmb03128,
    flm_cmb03129: FlmCmb03129,
    flm_cmb03130: FlmCmb03130,
    flm_cmb03131: FlmCmb03131,
}
impl RegisterBlock {
    #[doc = "0x00 - FLM Configuration Register (FLM_CFG)"]
    #[inline(always)]
    pub const fn flm_cfg(&self) -> &FlmCfg {
        &self.flm_cfg
    }
    #[doc = "0x04 - FLM Status Register (FLM_STAT)"]
    #[inline(always)]
    pub const fn flm_stat(&self) -> &FlmStat {
        &self.flm_stat
    }
    #[doc = "0x08 - FLM LOG Register 1-2 (FLM_LOG1-2)"]
    #[inline(always)]
    pub const fn flm_log120(&self) -> &FlmLog120 {
        &self.flm_log120
    }
    #[doc = "0x0c - FLM LOG Register 1-2 (FLM_LOG1-2)"]
    #[inline(always)]
    pub const fn flm_log121(&self) -> &FlmLog121 {
        &self.flm_log121
    }
    #[doc = "0x10 - FLM Interrupt Enable Register (FLM_IE)"]
    #[inline(always)]
    pub const fn flm_ie(&self) -> &FlmIe {
        &self.flm_ie
    }
    #[doc = "0x14 - FLM Control Register (FLM_CTL)"]
    #[inline(always)]
    pub const fn flm_ctl(&self) -> &FlmCtl {
        &self.flm_ctl
    }
    #[doc = "0x20 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang0(&self) -> &FlmRang0 {
        &self.flm_rang0
    }
    #[doc = "0x24 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang1(&self) -> &FlmRang1 {
        &self.flm_rang1
    }
    #[doc = "0x28 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang2(&self) -> &FlmRang2 {
        &self.flm_rang2
    }
    #[doc = "0x2c - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang3(&self) -> &FlmRang3 {
        &self.flm_rang3
    }
    #[doc = "0x30 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang4(&self) -> &FlmRang4 {
        &self.flm_rang4
    }
    #[doc = "0x34 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang5(&self) -> &FlmRang5 {
        &self.flm_rang5
    }
    #[doc = "0x38 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang6(&self) -> &FlmRang6 {
        &self.flm_rang6
    }
    #[doc = "0x3c - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang7(&self) -> &FlmRang7 {
        &self.flm_rang7
    }
    #[doc = "0x40 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang8(&self) -> &FlmRang8 {
        &self.flm_rang8
    }
    #[doc = "0x44 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang9(&self) -> &FlmRang9 {
        &self.flm_rang9
    }
    #[doc = "0x48 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang10(&self) -> &FlmRang10 {
        &self.flm_rang10
    }
    #[doc = "0x4c - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang11(&self) -> &FlmRang11 {
        &self.flm_rang11
    }
    #[doc = "0x50 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang12(&self) -> &FlmRang12 {
        &self.flm_rang12
    }
    #[doc = "0x54 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang13(&self) -> &FlmRang13 {
        &self.flm_rang13
    }
    #[doc = "0x58 - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang14(&self) -> &FlmRang14 {
        &self.flm_rang14
    }
    #[doc = "0x5c - FLM Range Register 0-15 (FLM_RANG0-15)"]
    #[inline(always)]
    pub const fn flm_rang15(&self) -> &FlmRang15 {
        &self.flm_rang15
    }
    #[doc = "0x60 - FLM Command Enable (FLM_CMDEN)"]
    #[inline(always)]
    pub const fn flm_cmden(&self) -> &FlmCmden {
        &self.flm_cmden
    }
    #[doc = "0x64 - FLM Command Byte Enable (FLM_CMBEN)"]
    #[inline(always)]
    pub const fn flm_cmben(&self) -> &FlmCmben {
        &self.flm_cmben
    }
    #[doc = "0x80 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd0(&self) -> &FlmCmd0 {
        &self.flm_cmd0
    }
    #[doc = "0x84 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd1(&self) -> &FlmCmd1 {
        &self.flm_cmd1
    }
    #[doc = "0x88 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd2(&self) -> &FlmCmd2 {
        &self.flm_cmd2
    }
    #[doc = "0x8c - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd3(&self) -> &FlmCmd3 {
        &self.flm_cmd3
    }
    #[doc = "0x90 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd4(&self) -> &FlmCmd4 {
        &self.flm_cmd4
    }
    #[doc = "0x94 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd5(&self) -> &FlmCmd5 {
        &self.flm_cmd5
    }
    #[doc = "0x98 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd6(&self) -> &FlmCmd6 {
        &self.flm_cmd6
    }
    #[doc = "0x9c - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd7(&self) -> &FlmCmd7 {
        &self.flm_cmd7
    }
    #[doc = "0xa0 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd8(&self) -> &FlmCmd8 {
        &self.flm_cmd8
    }
    #[doc = "0xa4 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd9(&self) -> &FlmCmd9 {
        &self.flm_cmd9
    }
    #[doc = "0xa8 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd10(&self) -> &FlmCmd10 {
        &self.flm_cmd10
    }
    #[doc = "0xac - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd11(&self) -> &FlmCmd11 {
        &self.flm_cmd11
    }
    #[doc = "0xb0 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd14(&self) -> &FlmCmd14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb0 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd12(&self) -> &FlmCmd12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb4 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd15(&self) -> &FlmCmd15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb4 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd13(&self) -> &FlmCmd13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xc0 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd18(&self) -> &FlmCmd18 {
        &self.flm_cmd18
    }
    #[doc = "0xc4 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd19(&self) -> &FlmCmd19 {
        &self.flm_cmd19
    }
    #[doc = "0xc8 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd16(&self) -> &FlmCmd16 {
        &self.flm_cmd16
    }
    #[doc = "0xcc - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd17(&self) -> &FlmCmd17 {
        &self.flm_cmd17
    }
    #[doc = "0xd0 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd22(&self) -> &FlmCmd22 {
        &self.flm_cmd22
    }
    #[doc = "0xd4 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd23(&self) -> &FlmCmd23 {
        &self.flm_cmd23
    }
    #[doc = "0xd8 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd20(&self) -> &FlmCmd20 {
        &self.flm_cmd20
    }
    #[doc = "0xdc - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd21(&self) -> &FlmCmd21 {
        &self.flm_cmd21
    }
    #[doc = "0xe0 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd24(&self) -> &FlmCmd24 {
        &self.flm_cmd24
    }
    #[doc = "0xe4 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd25(&self) -> &FlmCmd25 {
        &self.flm_cmd25
    }
    #[doc = "0xe8 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd26(&self) -> &FlmCmd26 {
        &self.flm_cmd26
    }
    #[doc = "0xec - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd27(&self) -> &FlmCmd27 {
        &self.flm_cmd27
    }
    #[doc = "0xf0 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd28(&self) -> &FlmCmd28 {
        &self.flm_cmd28
    }
    #[doc = "0xf4 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd29(&self) -> &FlmCmd29 {
        &self.flm_cmd29
    }
    #[doc = "0xf8 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd30(&self) -> &FlmCmd30 {
        &self.flm_cmd30
    }
    #[doc = "0xfc - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd31(&self) -> &FlmCmd31 {
        &self.flm_cmd31
    }
    #[doc = "0x100 - FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)"]
    #[inline(always)]
    pub const fn flm_tcra0(&self) -> &FlmTcra0 {
        &self.flm_tcra0
    }
    #[doc = "0x104 - FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)"]
    #[inline(always)]
    pub const fn flm_tcra1(&self) -> &FlmTcra1 {
        &self.flm_tcra1
    }
    #[doc = "0x108 - FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)"]
    #[inline(always)]
    pub const fn flm_tcra2(&self) -> &FlmTcra2 {
        &self.flm_tcra2
    }
    #[doc = "0x10c - FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)"]
    #[inline(always)]
    pub const fn flm_tcra3(&self) -> &FlmTcra3 {
        &self.flm_tcra3
    }
    #[doc = "0x110 - FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)"]
    #[inline(always)]
    pub const fn flm_tcrb0(&self) -> &FlmTcrb0 {
        &self.flm_tcrb0
    }
    #[doc = "0x114 - FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)"]
    #[inline(always)]
    pub const fn flm_tcrb1(&self) -> &FlmTcrb1 {
        &self.flm_tcrb1
    }
    #[doc = "0x118 - FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)"]
    #[inline(always)]
    pub const fn flm_tcrb2(&self) -> &FlmTcrb2 {
        &self.flm_tcrb2
    }
    #[doc = "0x11c - FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)"]
    #[inline(always)]
    pub const fn flm_tcrb3(&self) -> &FlmTcrb3 {
        &self.flm_tcrb3
    }
    #[doc = "0x140 - FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)"]
    #[inline(always)]
    pub const fn flm_tcca0(&self) -> &FlmTcca0 {
        &self.flm_tcca0
    }
    #[doc = "0x144 - FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)"]
    #[inline(always)]
    pub const fn flm_tcca1(&self) -> &FlmTcca1 {
        &self.flm_tcca1
    }
    #[doc = "0x148 - FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)"]
    #[inline(always)]
    pub const fn flm_tcca2(&self) -> &FlmTcca2 {
        &self.flm_tcca2
    }
    #[doc = "0x14c - FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)"]
    #[inline(always)]
    pub const fn flm_tcca3(&self) -> &FlmTcca3 {
        &self.flm_tcca3
    }
    #[doc = "0x150 - FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)"]
    #[inline(always)]
    pub const fn flm_tccb0(&self) -> &FlmTccb0 {
        &self.flm_tccb0
    }
    #[doc = "0x154 - FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)"]
    #[inline(always)]
    pub const fn flm_tccb1(&self) -> &FlmTccb1 {
        &self.flm_tccb1
    }
    #[doc = "0x158 - FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)"]
    #[inline(always)]
    pub const fn flm_tccb2(&self) -> &FlmTccb2 {
        &self.flm_tccb2
    }
    #[doc = "0x15c - FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)"]
    #[inline(always)]
    pub const fn flm_tccb3(&self) -> &FlmTccb3 {
        &self.flm_tccb3
    }
    #[doc = "0x180 - FLM CMD Event Register (FLM_CMDEV)"]
    #[inline(always)]
    pub const fn flm_cmdev(&self) -> &FlmCmdev {
        &self.flm_cmdev
    }
    #[doc = "0x184 - FLM CMB Event Register (FLM_CMBEV)"]
    #[inline(always)]
    pub const fn flm_cmbev(&self) -> &FlmCmbev {
        &self.flm_cmbev
    }
    #[doc = "0x188 - FLM Transaction Counter Global Control Register (FLM_TCGC)"]
    #[inline(always)]
    pub const fn flm_tcgc(&self) -> &FlmTcgc {
        &self.flm_tcgc
    }
    #[doc = "0x1c0 - FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
    #[inline(always)]
    pub const fn flm_cq030(&self) -> &FlmCq030 {
        &self.flm_cq030
    }
    #[doc = "0x1c4 - FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
    #[inline(always)]
    pub const fn flm_cq031(&self) -> &FlmCq031 {
        &self.flm_cq031
    }
    #[doc = "0x1c8 - FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
    #[inline(always)]
    pub const fn flm_cq032(&self) -> &FlmCq032 {
        &self.flm_cq032
    }
    #[doc = "0x1cc - FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
    #[inline(always)]
    pub const fn flm_cq033(&self) -> &FlmCq033 {
        &self.flm_cq033
    }
    #[doc = "0x200 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars0310(&self) -> &FlmCmdars0310 {
        &self.flm_cmdars0310
    }
    #[doc = "0x204 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars0311(&self) -> &FlmCmdars0311 {
        &self.flm_cmdars0311
    }
    #[doc = "0x208 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars0312(&self) -> &FlmCmdars0312 {
        &self.flm_cmdars0312
    }
    #[doc = "0x20c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars0313(&self) -> &FlmCmdars0313 {
        &self.flm_cmdars0313
    }
    #[doc = "0x210 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars0314(&self) -> &FlmCmdars0314 {
        &self.flm_cmdars0314
    }
    #[doc = "0x214 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars0315(&self) -> &FlmCmdars0315 {
        &self.flm_cmdars0315
    }
    #[doc = "0x218 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars0316(&self) -> &FlmCmdars0316 {
        &self.flm_cmdars0316
    }
    #[doc = "0x21c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars0317(&self) -> &FlmCmdars0317 {
        &self.flm_cmdars0317
    }
    #[doc = "0x220 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars0318(&self) -> &FlmCmdars0318 {
        &self.flm_cmdars0318
    }
    #[doc = "0x224 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars0319(&self) -> &FlmCmdars0319 {
        &self.flm_cmdars0319
    }
    #[doc = "0x228 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03110(&self) -> &FlmCmdars03110 {
        &self.flm_cmdars03110
    }
    #[doc = "0x22c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03111(&self) -> &FlmCmdars03111 {
        &self.flm_cmdars03111
    }
    #[doc = "0x230 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03114(&self) -> &FlmCmdars03114 {
        unsafe { &*(self as *const Self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x230 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03112(&self) -> &FlmCmdars03112 {
        unsafe { &*(self as *const Self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x234 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03115(&self) -> &FlmCmdars03115 {
        unsafe { &*(self as *const Self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x234 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03113(&self) -> &FlmCmdars03113 {
        unsafe { &*(self as *const Self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x240 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03118(&self) -> &FlmCmdars03118 {
        &self.flm_cmdars03118
    }
    #[doc = "0x244 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03119(&self) -> &FlmCmdars03119 {
        &self.flm_cmdars03119
    }
    #[doc = "0x248 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03116(&self) -> &FlmCmdars03116 {
        &self.flm_cmdars03116
    }
    #[doc = "0x24c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03117(&self) -> &FlmCmdars03117 {
        &self.flm_cmdars03117
    }
    #[doc = "0x250 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03122(&self) -> &FlmCmdars03122 {
        &self.flm_cmdars03122
    }
    #[doc = "0x254 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03123(&self) -> &FlmCmdars03123 {
        &self.flm_cmdars03123
    }
    #[doc = "0x258 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03120(&self) -> &FlmCmdars03120 {
        &self.flm_cmdars03120
    }
    #[doc = "0x25c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03121(&self) -> &FlmCmdars03121 {
        &self.flm_cmdars03121
    }
    #[doc = "0x260 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03124(&self) -> &FlmCmdars03124 {
        &self.flm_cmdars03124
    }
    #[doc = "0x264 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03125(&self) -> &FlmCmdars03125 {
        &self.flm_cmdars03125
    }
    #[doc = "0x268 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03126(&self) -> &FlmCmdars03126 {
        &self.flm_cmdars03126
    }
    #[doc = "0x26c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03127(&self) -> &FlmCmdars03127 {
        &self.flm_cmdars03127
    }
    #[doc = "0x270 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03128(&self) -> &FlmCmdars03128 {
        &self.flm_cmdars03128
    }
    #[doc = "0x274 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03129(&self) -> &FlmCmdars03129 {
        &self.flm_cmdars03129
    }
    #[doc = "0x278 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03130(&self) -> &FlmCmdars03130 {
        &self.flm_cmdars03130
    }
    #[doc = "0x27c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars03131(&self) -> &FlmCmdars03131 {
        &self.flm_cmdars03131
    }
    #[doc = "0x280 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb0310(&self) -> &FlmCmb0310 {
        &self.flm_cmb0310
    }
    #[doc = "0x284 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb0311(&self) -> &FlmCmb0311 {
        &self.flm_cmb0311
    }
    #[doc = "0x288 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb0312(&self) -> &FlmCmb0312 {
        &self.flm_cmb0312
    }
    #[doc = "0x28c - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb0313(&self) -> &FlmCmb0313 {
        &self.flm_cmb0313
    }
    #[doc = "0x290 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb0314(&self) -> &FlmCmb0314 {
        &self.flm_cmb0314
    }
    #[doc = "0x294 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb0315(&self) -> &FlmCmb0315 {
        &self.flm_cmb0315
    }
    #[doc = "0x298 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb0316(&self) -> &FlmCmb0316 {
        &self.flm_cmb0316
    }
    #[doc = "0x29c - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb0317(&self) -> &FlmCmb0317 {
        &self.flm_cmb0317
    }
    #[doc = "0x2a0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb0318(&self) -> &FlmCmb0318 {
        &self.flm_cmb0318
    }
    #[doc = "0x2a4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb0319(&self) -> &FlmCmb0319 {
        &self.flm_cmb0319
    }
    #[doc = "0x2a8 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03110(&self) -> &FlmCmb03110 {
        &self.flm_cmb03110
    }
    #[doc = "0x2ac - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03111(&self) -> &FlmCmb03111 {
        &self.flm_cmb03111
    }
    #[doc = "0x2b0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03114(&self) -> &FlmCmb03114 {
        unsafe { &*(self as *const Self).cast::<u8>().add(688).cast() }
    }
    #[doc = "0x2b0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03112(&self) -> &FlmCmb03112 {
        unsafe { &*(self as *const Self).cast::<u8>().add(688).cast() }
    }
    #[doc = "0x2b4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03115(&self) -> &FlmCmb03115 {
        unsafe { &*(self as *const Self).cast::<u8>().add(692).cast() }
    }
    #[doc = "0x2b4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03113(&self) -> &FlmCmb03113 {
        unsafe { &*(self as *const Self).cast::<u8>().add(692).cast() }
    }
    #[doc = "0x2c0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03118(&self) -> &FlmCmb03118 {
        &self.flm_cmb03118
    }
    #[doc = "0x2c4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03119(&self) -> &FlmCmb03119 {
        &self.flm_cmb03119
    }
    #[doc = "0x2c8 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03116(&self) -> &FlmCmb03116 {
        &self.flm_cmb03116
    }
    #[doc = "0x2cc - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03117(&self) -> &FlmCmb03117 {
        &self.flm_cmb03117
    }
    #[doc = "0x2d0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03122(&self) -> &FlmCmb03122 {
        &self.flm_cmb03122
    }
    #[doc = "0x2d4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03123(&self) -> &FlmCmb03123 {
        &self.flm_cmb03123
    }
    #[doc = "0x2d8 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03120(&self) -> &FlmCmb03120 {
        &self.flm_cmb03120
    }
    #[doc = "0x2dc - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03121(&self) -> &FlmCmb03121 {
        &self.flm_cmb03121
    }
    #[doc = "0x2e0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03124(&self) -> &FlmCmb03124 {
        &self.flm_cmb03124
    }
    #[doc = "0x2e4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03125(&self) -> &FlmCmb03125 {
        &self.flm_cmb03125
    }
    #[doc = "0x2e8 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03126(&self) -> &FlmCmb03126 {
        &self.flm_cmb03126
    }
    #[doc = "0x2ec - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03127(&self) -> &FlmCmb03127 {
        &self.flm_cmb03127
    }
    #[doc = "0x2f0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03128(&self) -> &FlmCmb03128 {
        &self.flm_cmb03128
    }
    #[doc = "0x2f4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03129(&self) -> &FlmCmb03129 {
        &self.flm_cmb03129
    }
    #[doc = "0x2f8 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03130(&self) -> &FlmCmb03130 {
        &self.flm_cmb03130
    }
    #[doc = "0x2fc - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb03131(&self) -> &FlmCmb03131 {
        &self.flm_cmb03131
    }
}
#[doc = "FLM_CFG (rw) register accessor: FLM Configuration Register (FLM_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cfg`]
module"]
#[doc(alias = "FLM_CFG")]
pub type FlmCfg = crate::Reg<flm_cfg::FlmCfgSpec>;
#[doc = "FLM Configuration Register (FLM_CFG)"]
pub mod flm_cfg;
#[doc = "FLM_STAT (rw) register accessor: FLM Status Register (FLM_STAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_stat`]
module"]
#[doc(alias = "FLM_STAT")]
pub type FlmStat = crate::Reg<flm_stat::FlmStatSpec>;
#[doc = "FLM Status Register (FLM_STAT)"]
pub mod flm_stat;
#[doc = "FLM_LOG1-20 (rw) register accessor: FLM LOG Register 1-2 (FLM_LOG1-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_log120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_log120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_log120`]
module"]
#[doc(alias = "FLM_LOG1-20")]
pub type FlmLog120 = crate::Reg<flm_log120::FlmLog120Spec>;
#[doc = "FLM LOG Register 1-2 (FLM_LOG1-2)"]
pub mod flm_log120;
#[doc = "FLM_LOG1-21 (rw) register accessor: FLM LOG Register 1-2 (FLM_LOG1-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_log121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_log121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_log121`]
module"]
#[doc(alias = "FLM_LOG1-21")]
pub type FlmLog121 = crate::Reg<flm_log121::FlmLog121Spec>;
#[doc = "FLM LOG Register 1-2 (FLM_LOG1-2)"]
pub mod flm_log121;
#[doc = "FLM_IE (rw) register accessor: FLM Interrupt Enable Register (FLM_IE)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_ie`]
module"]
#[doc(alias = "FLM_IE")]
pub type FlmIe = crate::Reg<flm_ie::FlmIeSpec>;
#[doc = "FLM Interrupt Enable Register (FLM_IE)"]
pub mod flm_ie;
#[doc = "FLM_CTL (rw) register accessor: FLM Control Register (FLM_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_ctl`]
module"]
#[doc(alias = "FLM_CTL")]
pub type FlmCtl = crate::Reg<flm_ctl::FlmCtlSpec>;
#[doc = "FLM Control Register (FLM_CTL)"]
pub mod flm_ctl;
#[doc = "FLM_RANG0 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang0`]
module"]
#[doc(alias = "FLM_RANG0")]
pub type FlmRang0 = crate::Reg<flm_rang0::FlmRang0Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang0;
#[doc = "FLM_RANG1 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang1`]
module"]
#[doc(alias = "FLM_RANG1")]
pub type FlmRang1 = crate::Reg<flm_rang1::FlmRang1Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang1;
#[doc = "FLM_RANG2 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang2`]
module"]
#[doc(alias = "FLM_RANG2")]
pub type FlmRang2 = crate::Reg<flm_rang2::FlmRang2Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang2;
#[doc = "FLM_RANG3 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang3`]
module"]
#[doc(alias = "FLM_RANG3")]
pub type FlmRang3 = crate::Reg<flm_rang3::FlmRang3Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang3;
#[doc = "FLM_RANG4 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang4`]
module"]
#[doc(alias = "FLM_RANG4")]
pub type FlmRang4 = crate::Reg<flm_rang4::FlmRang4Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang4;
#[doc = "FLM_RANG5 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang5`]
module"]
#[doc(alias = "FLM_RANG5")]
pub type FlmRang5 = crate::Reg<flm_rang5::FlmRang5Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang5;
#[doc = "FLM_RANG6 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang6`]
module"]
#[doc(alias = "FLM_RANG6")]
pub type FlmRang6 = crate::Reg<flm_rang6::FlmRang6Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang6;
#[doc = "FLM_RANG7 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang7`]
module"]
#[doc(alias = "FLM_RANG7")]
pub type FlmRang7 = crate::Reg<flm_rang7::FlmRang7Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang7;
#[doc = "FLM_RANG8 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang8`]
module"]
#[doc(alias = "FLM_RANG8")]
pub type FlmRang8 = crate::Reg<flm_rang8::FlmRang8Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang8;
#[doc = "FLM_RANG9 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang9`]
module"]
#[doc(alias = "FLM_RANG9")]
pub type FlmRang9 = crate::Reg<flm_rang9::FlmRang9Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang9;
#[doc = "FLM_RANG10 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang10`]
module"]
#[doc(alias = "FLM_RANG10")]
pub type FlmRang10 = crate::Reg<flm_rang10::FlmRang10Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang10;
#[doc = "FLM_RANG11 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang11`]
module"]
#[doc(alias = "FLM_RANG11")]
pub type FlmRang11 = crate::Reg<flm_rang11::FlmRang11Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang11;
#[doc = "FLM_RANG12 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang12`]
module"]
#[doc(alias = "FLM_RANG12")]
pub type FlmRang12 = crate::Reg<flm_rang12::FlmRang12Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang12;
#[doc = "FLM_RANG13 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang13`]
module"]
#[doc(alias = "FLM_RANG13")]
pub type FlmRang13 = crate::Reg<flm_rang13::FlmRang13Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang13;
#[doc = "FLM_RANG14 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang14`]
module"]
#[doc(alias = "FLM_RANG14")]
pub type FlmRang14 = crate::Reg<flm_rang14::FlmRang14Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang14;
#[doc = "FLM_RANG15 (rw) register accessor: FLM Range Register 0-15 (FLM_RANG0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang15`]
module"]
#[doc(alias = "FLM_RANG15")]
pub type FlmRang15 = crate::Reg<flm_rang15::FlmRang15Spec>;
#[doc = "FLM Range Register 0-15 (FLM_RANG0-15)"]
pub mod flm_rang15;
#[doc = "FLM_CMDEN (rw) register accessor: FLM Command Enable (FLM_CMDEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmden::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmden::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmden`]
module"]
#[doc(alias = "FLM_CMDEN")]
pub type FlmCmden = crate::Reg<flm_cmden::FlmCmdenSpec>;
#[doc = "FLM Command Enable (FLM_CMDEN)"]
pub mod flm_cmden;
#[doc = "FLM_CMBEN (rw) register accessor: FLM Command Byte Enable (FLM_CMBEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmben::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmben::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmben`]
module"]
#[doc(alias = "FLM_CMBEN")]
pub type FlmCmben = crate::Reg<flm_cmben::FlmCmbenSpec>;
#[doc = "FLM Command Byte Enable (FLM_CMBEN)"]
pub mod flm_cmben;
#[doc = "FLM_CMD0 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd0`]
module"]
#[doc(alias = "FLM_CMD0")]
pub type FlmCmd0 = crate::Reg<flm_cmd0::FlmCmd0Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd0;
#[doc = "FLM_CMD1 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd1`]
module"]
#[doc(alias = "FLM_CMD1")]
pub type FlmCmd1 = crate::Reg<flm_cmd1::FlmCmd1Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd1;
#[doc = "FLM_CMD2 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd2`]
module"]
#[doc(alias = "FLM_CMD2")]
pub type FlmCmd2 = crate::Reg<flm_cmd2::FlmCmd2Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd2;
#[doc = "FLM_CMD3 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd3`]
module"]
#[doc(alias = "FLM_CMD3")]
pub type FlmCmd3 = crate::Reg<flm_cmd3::FlmCmd3Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd3;
#[doc = "FLM_CMD4 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd4`]
module"]
#[doc(alias = "FLM_CMD4")]
pub type FlmCmd4 = crate::Reg<flm_cmd4::FlmCmd4Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd4;
#[doc = "FLM_CMD5 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd5`]
module"]
#[doc(alias = "FLM_CMD5")]
pub type FlmCmd5 = crate::Reg<flm_cmd5::FlmCmd5Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd5;
#[doc = "FLM_CMD6 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd6`]
module"]
#[doc(alias = "FLM_CMD6")]
pub type FlmCmd6 = crate::Reg<flm_cmd6::FlmCmd6Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd6;
#[doc = "FLM_CMD7 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd7`]
module"]
#[doc(alias = "FLM_CMD7")]
pub type FlmCmd7 = crate::Reg<flm_cmd7::FlmCmd7Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd7;
#[doc = "FLM_CMD8 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd8`]
module"]
#[doc(alias = "FLM_CMD8")]
pub type FlmCmd8 = crate::Reg<flm_cmd8::FlmCmd8Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd8;
#[doc = "FLM_CMD9 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd9`]
module"]
#[doc(alias = "FLM_CMD9")]
pub type FlmCmd9 = crate::Reg<flm_cmd9::FlmCmd9Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd9;
#[doc = "FLM_CMD10 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd10`]
module"]
#[doc(alias = "FLM_CMD10")]
pub type FlmCmd10 = crate::Reg<flm_cmd10::FlmCmd10Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd10;
#[doc = "FLM_CMD11 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd11`]
module"]
#[doc(alias = "FLM_CMD11")]
pub type FlmCmd11 = crate::Reg<flm_cmd11::FlmCmd11Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd11;
#[doc = "FLM_CMD12 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd12`]
module"]
#[doc(alias = "FLM_CMD12")]
pub type FlmCmd12 = crate::Reg<flm_cmd12::FlmCmd12Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd12;
#[doc = "FLM_CMD13 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd13`]
module"]
#[doc(alias = "FLM_CMD13")]
pub type FlmCmd13 = crate::Reg<flm_cmd13::FlmCmd13Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd13;
#[doc = "FLM_CMD14 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd14`]
module"]
#[doc(alias = "FLM_CMD14")]
pub type FlmCmd14 = crate::Reg<flm_cmd14::FlmCmd14Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd14;
#[doc = "FLM_CMD15 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd15`]
module"]
#[doc(alias = "FLM_CMD15")]
pub type FlmCmd15 = crate::Reg<flm_cmd15::FlmCmd15Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd15;
#[doc = "FLM_CMD16 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd16`]
module"]
#[doc(alias = "FLM_CMD16")]
pub type FlmCmd16 = crate::Reg<flm_cmd16::FlmCmd16Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd16;
#[doc = "FLM_CMD17 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd17`]
module"]
#[doc(alias = "FLM_CMD17")]
pub type FlmCmd17 = crate::Reg<flm_cmd17::FlmCmd17Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd17;
#[doc = "FLM_CMD18 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd18`]
module"]
#[doc(alias = "FLM_CMD18")]
pub type FlmCmd18 = crate::Reg<flm_cmd18::FlmCmd18Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd18;
#[doc = "FLM_CMD19 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd19`]
module"]
#[doc(alias = "FLM_CMD19")]
pub type FlmCmd19 = crate::Reg<flm_cmd19::FlmCmd19Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd19;
#[doc = "FLM_CMD20 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd20`]
module"]
#[doc(alias = "FLM_CMD20")]
pub type FlmCmd20 = crate::Reg<flm_cmd20::FlmCmd20Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd20;
#[doc = "FLM_CMD21 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd21`]
module"]
#[doc(alias = "FLM_CMD21")]
pub type FlmCmd21 = crate::Reg<flm_cmd21::FlmCmd21Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd21;
#[doc = "FLM_CMD22 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd22`]
module"]
#[doc(alias = "FLM_CMD22")]
pub type FlmCmd22 = crate::Reg<flm_cmd22::FlmCmd22Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd22;
#[doc = "FLM_CMD23 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd23`]
module"]
#[doc(alias = "FLM_CMD23")]
pub type FlmCmd23 = crate::Reg<flm_cmd23::FlmCmd23Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd23;
#[doc = "FLM_CMD24 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd24`]
module"]
#[doc(alias = "FLM_CMD24")]
pub type FlmCmd24 = crate::Reg<flm_cmd24::FlmCmd24Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd24;
#[doc = "FLM_CMD25 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd25`]
module"]
#[doc(alias = "FLM_CMD25")]
pub type FlmCmd25 = crate::Reg<flm_cmd25::FlmCmd25Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd25;
#[doc = "FLM_CMD26 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd26`]
module"]
#[doc(alias = "FLM_CMD26")]
pub type FlmCmd26 = crate::Reg<flm_cmd26::FlmCmd26Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd26;
#[doc = "FLM_CMD27 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd27`]
module"]
#[doc(alias = "FLM_CMD27")]
pub type FlmCmd27 = crate::Reg<flm_cmd27::FlmCmd27Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd27;
#[doc = "FLM_CMD28 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd28`]
module"]
#[doc(alias = "FLM_CMD28")]
pub type FlmCmd28 = crate::Reg<flm_cmd28::FlmCmd28Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd28;
#[doc = "FLM_CMD29 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd29`]
module"]
#[doc(alias = "FLM_CMD29")]
pub type FlmCmd29 = crate::Reg<flm_cmd29::FlmCmd29Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd29;
#[doc = "FLM_CMD30 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd30`]
module"]
#[doc(alias = "FLM_CMD30")]
pub type FlmCmd30 = crate::Reg<flm_cmd30::FlmCmd30Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd30;
#[doc = "FLM_CMD31 (rw) register accessor: FLM Command Register 0-31 (FLM_CMD0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd31`]
module"]
#[doc(alias = "FLM_CMD31")]
pub type FlmCmd31 = crate::Reg<flm_cmd31::FlmCmd31Spec>;
#[doc = "FLM Command Register 0-31 (FLM_CMD0-31)"]
pub mod flm_cmd31;
#[doc = "FLM_TCRA0 (rw) register accessor: FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcra0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcra0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcra0`]
module"]
#[doc(alias = "FLM_TCRA0")]
pub type FlmTcra0 = crate::Reg<flm_tcra0::FlmTcra0Spec>;
#[doc = "FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)"]
pub mod flm_tcra0;
#[doc = "FLM_TCRA1 (rw) register accessor: FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcra1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcra1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcra1`]
module"]
#[doc(alias = "FLM_TCRA1")]
pub type FlmTcra1 = crate::Reg<flm_tcra1::FlmTcra1Spec>;
#[doc = "FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)"]
pub mod flm_tcra1;
#[doc = "FLM_TCRA2 (rw) register accessor: FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcra2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcra2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcra2`]
module"]
#[doc(alias = "FLM_TCRA2")]
pub type FlmTcra2 = crate::Reg<flm_tcra2::FlmTcra2Spec>;
#[doc = "FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)"]
pub mod flm_tcra2;
#[doc = "FLM_TCRA3 (rw) register accessor: FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcra3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcra3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcra3`]
module"]
#[doc(alias = "FLM_TCRA3")]
pub type FlmTcra3 = crate::Reg<flm_tcra3::FlmTcra3Spec>;
#[doc = "FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)"]
pub mod flm_tcra3;
#[doc = "FLM_TCRB0 (rw) register accessor: FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcrb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcrb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcrb0`]
module"]
#[doc(alias = "FLM_TCRB0")]
pub type FlmTcrb0 = crate::Reg<flm_tcrb0::FlmTcrb0Spec>;
#[doc = "FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)"]
pub mod flm_tcrb0;
#[doc = "FLM_TCRB1 (rw) register accessor: FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcrb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcrb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcrb1`]
module"]
#[doc(alias = "FLM_TCRB1")]
pub type FlmTcrb1 = crate::Reg<flm_tcrb1::FlmTcrb1Spec>;
#[doc = "FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)"]
pub mod flm_tcrb1;
#[doc = "FLM_TCRB2 (rw) register accessor: FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcrb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcrb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcrb2`]
module"]
#[doc(alias = "FLM_TCRB2")]
pub type FlmTcrb2 = crate::Reg<flm_tcrb2::FlmTcrb2Spec>;
#[doc = "FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)"]
pub mod flm_tcrb2;
#[doc = "FLM_TCRB3 (rw) register accessor: FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcrb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcrb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcrb3`]
module"]
#[doc(alias = "FLM_TCRB3")]
pub type FlmTcrb3 = crate::Reg<flm_tcrb3::FlmTcrb3Spec>;
#[doc = "FLM Transaction Counter Read Register B0-B3 (FLM_TCRB)"]
pub mod flm_tcrb3;
#[doc = "FLM_TCCA0 (rw) register accessor: FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcca0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcca0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcca0`]
module"]
#[doc(alias = "FLM_TCCA0")]
pub type FlmTcca0 = crate::Reg<flm_tcca0::FlmTcca0Spec>;
#[doc = "FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)"]
pub mod flm_tcca0;
#[doc = "FLM_TCCA1 (rw) register accessor: FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcca1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcca1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcca1`]
module"]
#[doc(alias = "FLM_TCCA1")]
pub type FlmTcca1 = crate::Reg<flm_tcca1::FlmTcca1Spec>;
#[doc = "FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)"]
pub mod flm_tcca1;
#[doc = "FLM_TCCA2 (rw) register accessor: FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcca2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcca2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcca2`]
module"]
#[doc(alias = "FLM_TCCA2")]
pub type FlmTcca2 = crate::Reg<flm_tcca2::FlmTcca2Spec>;
#[doc = "FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)"]
pub mod flm_tcca2;
#[doc = "FLM_TCCA3 (rw) register accessor: FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcca3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcca3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcca3`]
module"]
#[doc(alias = "FLM_TCCA3")]
pub type FlmTcca3 = crate::Reg<flm_tcca3::FlmTcca3Spec>;
#[doc = "FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)"]
pub mod flm_tcca3;
#[doc = "FLM_TCCB0 (rw) register accessor: FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tccb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tccb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tccb0`]
module"]
#[doc(alias = "FLM_TCCB0")]
pub type FlmTccb0 = crate::Reg<flm_tccb0::FlmTccb0Spec>;
#[doc = "FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)"]
pub mod flm_tccb0;
#[doc = "FLM_TCCB1 (rw) register accessor: FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tccb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tccb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tccb1`]
module"]
#[doc(alias = "FLM_TCCB1")]
pub type FlmTccb1 = crate::Reg<flm_tccb1::FlmTccb1Spec>;
#[doc = "FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)"]
pub mod flm_tccb1;
#[doc = "FLM_TCCB2 (rw) register accessor: FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tccb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tccb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tccb2`]
module"]
#[doc(alias = "FLM_TCCB2")]
pub type FlmTccb2 = crate::Reg<flm_tccb2::FlmTccb2Spec>;
#[doc = "FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)"]
pub mod flm_tccb2;
#[doc = "FLM_TCCB3 (rw) register accessor: FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tccb3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tccb3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tccb3`]
module"]
#[doc(alias = "FLM_TCCB3")]
pub type FlmTccb3 = crate::Reg<flm_tccb3::FlmTccb3Spec>;
#[doc = "FLM Transaction Counter Control Register B0-B3 (FLM_TCCB)"]
pub mod flm_tccb3;
#[doc = "FLM_CMDEV (rw) register accessor: FLM CMD Event Register (FLM_CMDEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdev`]
module"]
#[doc(alias = "FLM_CMDEV")]
pub type FlmCmdev = crate::Reg<flm_cmdev::FlmCmdevSpec>;
#[doc = "FLM CMD Event Register (FLM_CMDEV)"]
pub mod flm_cmdev;
#[doc = "FLM_CMBEV (rw) register accessor: FLM CMB Event Register (FLM_CMBEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmbev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmbev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmbev`]
module"]
#[doc(alias = "FLM_CMBEV")]
pub type FlmCmbev = crate::Reg<flm_cmbev::FlmCmbevSpec>;
#[doc = "FLM CMB Event Register (FLM_CMBEV)"]
pub mod flm_cmbev;
#[doc = "FLM_TCGC (rw) register accessor: FLM Transaction Counter Global Control Register (FLM_TCGC)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcgc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcgc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcgc`]
module"]
#[doc(alias = "FLM_TCGC")]
pub type FlmTcgc = crate::Reg<flm_tcgc::FlmTcgcSpec>;
#[doc = "FLM Transaction Counter Global Control Register (FLM_TCGC)"]
pub mod flm_tcgc;
#[doc = "FLM_CQ0-30 (rw) register accessor: FLM Command Qualifier Register 0-3 (FLM_CQ0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cq030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cq030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cq030`]
module"]
#[doc(alias = "FLM_CQ0-30")]
pub type FlmCq030 = crate::Reg<flm_cq030::FlmCq030Spec>;
#[doc = "FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
pub mod flm_cq030;
#[doc = "FLM_CQ0-31 (rw) register accessor: FLM Command Qualifier Register 0-3 (FLM_CQ0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cq031::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cq031::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cq031`]
module"]
#[doc(alias = "FLM_CQ0-31")]
pub type FlmCq031 = crate::Reg<flm_cq031::FlmCq031Spec>;
#[doc = "FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
pub mod flm_cq031;
#[doc = "FLM_CQ0-32 (rw) register accessor: FLM Command Qualifier Register 0-3 (FLM_CQ0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cq032::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cq032::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cq032`]
module"]
#[doc(alias = "FLM_CQ0-32")]
pub type FlmCq032 = crate::Reg<flm_cq032::FlmCq032Spec>;
#[doc = "FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
pub mod flm_cq032;
#[doc = "FLM_CQ0-33 (rw) register accessor: FLM Command Qualifier Register 0-3 (FLM_CQ0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cq033::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cq033::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cq033`]
module"]
#[doc(alias = "FLM_CQ0-33")]
pub type FlmCq033 = crate::Reg<flm_cq033::FlmCq033Spec>;
#[doc = "FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
pub mod flm_cq033;
#[doc = "FLM_CMDARS0-310 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars0310::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars0310::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars0310`]
module"]
#[doc(alias = "FLM_CMDARS0-310")]
pub type FlmCmdars0310 = crate::Reg<flm_cmdars0310::FlmCmdars0310Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars0310;
#[doc = "FLM_CMDARS0-311 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars0311::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars0311::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars0311`]
module"]
#[doc(alias = "FLM_CMDARS0-311")]
pub type FlmCmdars0311 = crate::Reg<flm_cmdars0311::FlmCmdars0311Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars0311;
#[doc = "FLM_CMDARS0-312 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars0312::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars0312::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars0312`]
module"]
#[doc(alias = "FLM_CMDARS0-312")]
pub type FlmCmdars0312 = crate::Reg<flm_cmdars0312::FlmCmdars0312Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars0312;
#[doc = "FLM_CMDARS0-313 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars0313::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars0313::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars0313`]
module"]
#[doc(alias = "FLM_CMDARS0-313")]
pub type FlmCmdars0313 = crate::Reg<flm_cmdars0313::FlmCmdars0313Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars0313;
#[doc = "FLM_CMDARS0-314 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars0314::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars0314::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars0314`]
module"]
#[doc(alias = "FLM_CMDARS0-314")]
pub type FlmCmdars0314 = crate::Reg<flm_cmdars0314::FlmCmdars0314Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars0314;
#[doc = "FLM_CMDARS0-315 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars0315::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars0315::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars0315`]
module"]
#[doc(alias = "FLM_CMDARS0-315")]
pub type FlmCmdars0315 = crate::Reg<flm_cmdars0315::FlmCmdars0315Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars0315;
#[doc = "FLM_CMDARS0-316 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars0316::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars0316::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars0316`]
module"]
#[doc(alias = "FLM_CMDARS0-316")]
pub type FlmCmdars0316 = crate::Reg<flm_cmdars0316::FlmCmdars0316Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars0316;
#[doc = "FLM_CMDARS0-317 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars0317::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars0317::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars0317`]
module"]
#[doc(alias = "FLM_CMDARS0-317")]
pub type FlmCmdars0317 = crate::Reg<flm_cmdars0317::FlmCmdars0317Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars0317;
#[doc = "FLM_CMDARS0-318 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars0318::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars0318::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars0318`]
module"]
#[doc(alias = "FLM_CMDARS0-318")]
pub type FlmCmdars0318 = crate::Reg<flm_cmdars0318::FlmCmdars0318Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars0318;
#[doc = "FLM_CMDARS0-319 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars0319::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars0319::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars0319`]
module"]
#[doc(alias = "FLM_CMDARS0-319")]
pub type FlmCmdars0319 = crate::Reg<flm_cmdars0319::FlmCmdars0319Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars0319;
#[doc = "FLM_CMDARS0-3110 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03110`]
module"]
#[doc(alias = "FLM_CMDARS0-3110")]
pub type FlmCmdars03110 = crate::Reg<flm_cmdars03110::FlmCmdars03110Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03110;
#[doc = "FLM_CMDARS0-3111 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03111`]
module"]
#[doc(alias = "FLM_CMDARS0-3111")]
pub type FlmCmdars03111 = crate::Reg<flm_cmdars03111::FlmCmdars03111Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03111;
#[doc = "FLM_CMDARS0-3112 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03112`]
module"]
#[doc(alias = "FLM_CMDARS0-3112")]
pub type FlmCmdars03112 = crate::Reg<flm_cmdars03112::FlmCmdars03112Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03112;
#[doc = "FLM_CMDARS0-3113 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03113`]
module"]
#[doc(alias = "FLM_CMDARS0-3113")]
pub type FlmCmdars03113 = crate::Reg<flm_cmdars03113::FlmCmdars03113Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03113;
#[doc = "FLM_CMDARS0-3114 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03114`]
module"]
#[doc(alias = "FLM_CMDARS0-3114")]
pub type FlmCmdars03114 = crate::Reg<flm_cmdars03114::FlmCmdars03114Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03114;
#[doc = "FLM_CMDARS0-3115 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03115`]
module"]
#[doc(alias = "FLM_CMDARS0-3115")]
pub type FlmCmdars03115 = crate::Reg<flm_cmdars03115::FlmCmdars03115Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03115;
#[doc = "FLM_CMDARS0-3116 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03116`]
module"]
#[doc(alias = "FLM_CMDARS0-3116")]
pub type FlmCmdars03116 = crate::Reg<flm_cmdars03116::FlmCmdars03116Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03116;
#[doc = "FLM_CMDARS0-3117 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03117`]
module"]
#[doc(alias = "FLM_CMDARS0-3117")]
pub type FlmCmdars03117 = crate::Reg<flm_cmdars03117::FlmCmdars03117Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03117;
#[doc = "FLM_CMDARS0-3118 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03118`]
module"]
#[doc(alias = "FLM_CMDARS0-3118")]
pub type FlmCmdars03118 = crate::Reg<flm_cmdars03118::FlmCmdars03118Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03118;
#[doc = "FLM_CMDARS0-3119 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03119`]
module"]
#[doc(alias = "FLM_CMDARS0-3119")]
pub type FlmCmdars03119 = crate::Reg<flm_cmdars03119::FlmCmdars03119Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03119;
#[doc = "FLM_CMDARS0-3120 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03120`]
module"]
#[doc(alias = "FLM_CMDARS0-3120")]
pub type FlmCmdars03120 = crate::Reg<flm_cmdars03120::FlmCmdars03120Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03120;
#[doc = "FLM_CMDARS0-3121 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03121`]
module"]
#[doc(alias = "FLM_CMDARS0-3121")]
pub type FlmCmdars03121 = crate::Reg<flm_cmdars03121::FlmCmdars03121Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03121;
#[doc = "FLM_CMDARS0-3122 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03122`]
module"]
#[doc(alias = "FLM_CMDARS0-3122")]
pub type FlmCmdars03122 = crate::Reg<flm_cmdars03122::FlmCmdars03122Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03122;
#[doc = "FLM_CMDARS0-3123 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03123`]
module"]
#[doc(alias = "FLM_CMDARS0-3123")]
pub type FlmCmdars03123 = crate::Reg<flm_cmdars03123::FlmCmdars03123Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03123;
#[doc = "FLM_CMDARS0-3124 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03124`]
module"]
#[doc(alias = "FLM_CMDARS0-3124")]
pub type FlmCmdars03124 = crate::Reg<flm_cmdars03124::FlmCmdars03124Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03124;
#[doc = "FLM_CMDARS0-3125 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03125`]
module"]
#[doc(alias = "FLM_CMDARS0-3125")]
pub type FlmCmdars03125 = crate::Reg<flm_cmdars03125::FlmCmdars03125Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03125;
#[doc = "FLM_CMDARS0-3126 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03126`]
module"]
#[doc(alias = "FLM_CMDARS0-3126")]
pub type FlmCmdars03126 = crate::Reg<flm_cmdars03126::FlmCmdars03126Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03126;
#[doc = "FLM_CMDARS0-3127 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03127`]
module"]
#[doc(alias = "FLM_CMDARS0-3127")]
pub type FlmCmdars03127 = crate::Reg<flm_cmdars03127::FlmCmdars03127Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03127;
#[doc = "FLM_CMDARS0-3128 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03128`]
module"]
#[doc(alias = "FLM_CMDARS0-3128")]
pub type FlmCmdars03128 = crate::Reg<flm_cmdars03128::FlmCmdars03128Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03128;
#[doc = "FLM_CMDARS0-3129 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03129::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03129::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03129`]
module"]
#[doc(alias = "FLM_CMDARS0-3129")]
pub type FlmCmdars03129 = crate::Reg<flm_cmdars03129::FlmCmdars03129Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03129;
#[doc = "FLM_CMDARS0-3130 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03130`]
module"]
#[doc(alias = "FLM_CMDARS0-3130")]
pub type FlmCmdars03130 = crate::Reg<flm_cmdars03130::FlmCmdars03130Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03130;
#[doc = "FLM_CMDARS0-3131 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars03131::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars03131::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars03131`]
module"]
#[doc(alias = "FLM_CMDARS0-3131")]
pub type FlmCmdars03131 = crate::Reg<flm_cmdars03131::FlmCmdars03131Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars03131;
#[doc = "FLM_CMB0-310 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb0310::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb0310::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb0310`]
module"]
#[doc(alias = "FLM_CMB0-310")]
pub type FlmCmb0310 = crate::Reg<flm_cmb0310::FlmCmb0310Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb0310;
#[doc = "FLM_CMB0-311 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb0311::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb0311::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb0311`]
module"]
#[doc(alias = "FLM_CMB0-311")]
pub type FlmCmb0311 = crate::Reg<flm_cmb0311::FlmCmb0311Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb0311;
#[doc = "FLM_CMB0-312 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb0312::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb0312::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb0312`]
module"]
#[doc(alias = "FLM_CMB0-312")]
pub type FlmCmb0312 = crate::Reg<flm_cmb0312::FlmCmb0312Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb0312;
#[doc = "FLM_CMB0-313 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb0313::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb0313::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb0313`]
module"]
#[doc(alias = "FLM_CMB0-313")]
pub type FlmCmb0313 = crate::Reg<flm_cmb0313::FlmCmb0313Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb0313;
#[doc = "FLM_CMB0-314 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb0314::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb0314::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb0314`]
module"]
#[doc(alias = "FLM_CMB0-314")]
pub type FlmCmb0314 = crate::Reg<flm_cmb0314::FlmCmb0314Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb0314;
#[doc = "FLM_CMB0-315 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb0315::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb0315::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb0315`]
module"]
#[doc(alias = "FLM_CMB0-315")]
pub type FlmCmb0315 = crate::Reg<flm_cmb0315::FlmCmb0315Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb0315;
#[doc = "FLM_CMB0-316 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb0316::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb0316::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb0316`]
module"]
#[doc(alias = "FLM_CMB0-316")]
pub type FlmCmb0316 = crate::Reg<flm_cmb0316::FlmCmb0316Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb0316;
#[doc = "FLM_CMB0-317 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb0317::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb0317::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb0317`]
module"]
#[doc(alias = "FLM_CMB0-317")]
pub type FlmCmb0317 = crate::Reg<flm_cmb0317::FlmCmb0317Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb0317;
#[doc = "FLM_CMB0-318 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb0318::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb0318::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb0318`]
module"]
#[doc(alias = "FLM_CMB0-318")]
pub type FlmCmb0318 = crate::Reg<flm_cmb0318::FlmCmb0318Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb0318;
#[doc = "FLM_CMB0-319 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb0319::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb0319::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb0319`]
module"]
#[doc(alias = "FLM_CMB0-319")]
pub type FlmCmb0319 = crate::Reg<flm_cmb0319::FlmCmb0319Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb0319;
#[doc = "FLM_CMB0-3110 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03110`]
module"]
#[doc(alias = "FLM_CMB0-3110")]
pub type FlmCmb03110 = crate::Reg<flm_cmb03110::FlmCmb03110Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03110;
#[doc = "FLM_CMB0-3111 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03111`]
module"]
#[doc(alias = "FLM_CMB0-3111")]
pub type FlmCmb03111 = crate::Reg<flm_cmb03111::FlmCmb03111Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03111;
#[doc = "FLM_CMB0-3112 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03112`]
module"]
#[doc(alias = "FLM_CMB0-3112")]
pub type FlmCmb03112 = crate::Reg<flm_cmb03112::FlmCmb03112Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03112;
#[doc = "FLM_CMB0-3113 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03113`]
module"]
#[doc(alias = "FLM_CMB0-3113")]
pub type FlmCmb03113 = crate::Reg<flm_cmb03113::FlmCmb03113Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03113;
#[doc = "FLM_CMB0-3114 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03114`]
module"]
#[doc(alias = "FLM_CMB0-3114")]
pub type FlmCmb03114 = crate::Reg<flm_cmb03114::FlmCmb03114Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03114;
#[doc = "FLM_CMB0-3115 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03115`]
module"]
#[doc(alias = "FLM_CMB0-3115")]
pub type FlmCmb03115 = crate::Reg<flm_cmb03115::FlmCmb03115Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03115;
#[doc = "FLM_CMB0-3116 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03116`]
module"]
#[doc(alias = "FLM_CMB0-3116")]
pub type FlmCmb03116 = crate::Reg<flm_cmb03116::FlmCmb03116Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03116;
#[doc = "FLM_CMB0-3117 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03117`]
module"]
#[doc(alias = "FLM_CMB0-3117")]
pub type FlmCmb03117 = crate::Reg<flm_cmb03117::FlmCmb03117Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03117;
#[doc = "FLM_CMB0-3118 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03118`]
module"]
#[doc(alias = "FLM_CMB0-3118")]
pub type FlmCmb03118 = crate::Reg<flm_cmb03118::FlmCmb03118Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03118;
#[doc = "FLM_CMB0-3119 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03119`]
module"]
#[doc(alias = "FLM_CMB0-3119")]
pub type FlmCmb03119 = crate::Reg<flm_cmb03119::FlmCmb03119Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03119;
#[doc = "FLM_CMB0-3120 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03120`]
module"]
#[doc(alias = "FLM_CMB0-3120")]
pub type FlmCmb03120 = crate::Reg<flm_cmb03120::FlmCmb03120Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03120;
#[doc = "FLM_CMB0-3121 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03121`]
module"]
#[doc(alias = "FLM_CMB0-3121")]
pub type FlmCmb03121 = crate::Reg<flm_cmb03121::FlmCmb03121Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03121;
#[doc = "FLM_CMB0-3122 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03122`]
module"]
#[doc(alias = "FLM_CMB0-3122")]
pub type FlmCmb03122 = crate::Reg<flm_cmb03122::FlmCmb03122Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03122;
#[doc = "FLM_CMB0-3123 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03123`]
module"]
#[doc(alias = "FLM_CMB0-3123")]
pub type FlmCmb03123 = crate::Reg<flm_cmb03123::FlmCmb03123Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03123;
#[doc = "FLM_CMB0-3124 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03124`]
module"]
#[doc(alias = "FLM_CMB0-3124")]
pub type FlmCmb03124 = crate::Reg<flm_cmb03124::FlmCmb03124Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03124;
#[doc = "FLM_CMB0-3125 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03125`]
module"]
#[doc(alias = "FLM_CMB0-3125")]
pub type FlmCmb03125 = crate::Reg<flm_cmb03125::FlmCmb03125Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03125;
#[doc = "FLM_CMB0-3126 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03126`]
module"]
#[doc(alias = "FLM_CMB0-3126")]
pub type FlmCmb03126 = crate::Reg<flm_cmb03126::FlmCmb03126Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03126;
#[doc = "FLM_CMB0-3127 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03127`]
module"]
#[doc(alias = "FLM_CMB0-3127")]
pub type FlmCmb03127 = crate::Reg<flm_cmb03127::FlmCmb03127Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03127;
#[doc = "FLM_CMB0-3128 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03128`]
module"]
#[doc(alias = "FLM_CMB0-3128")]
pub type FlmCmb03128 = crate::Reg<flm_cmb03128::FlmCmb03128Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03128;
#[doc = "FLM_CMB0-3129 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03129::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03129::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03129`]
module"]
#[doc(alias = "FLM_CMB0-3129")]
pub type FlmCmb03129 = crate::Reg<flm_cmb03129::FlmCmb03129Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03129;
#[doc = "FLM_CMB0-3130 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03130`]
module"]
#[doc(alias = "FLM_CMB0-3130")]
pub type FlmCmb03130 = crate::Reg<flm_cmb03130::FlmCmb03130Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03130;
#[doc = "FLM_CMB0-3131 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb03131::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb03131::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb03131`]
module"]
#[doc(alias = "FLM_CMB0-3131")]
pub type FlmCmb03131 = crate::Reg<flm_cmb03131::FlmCmb03131Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb03131;
