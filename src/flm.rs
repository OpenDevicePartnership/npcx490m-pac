#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    flm_cfg: FlmCfg,
    flm_stat: FlmStat,
    flm_log1: FlmLog1,
    flm_log2: FlmLog2,
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
    flm_cq0: FlmCq0,
    flm_cq1: FlmCq1,
    flm_cq2: FlmCq2,
    flm_cq3: FlmCq3,
    _reserved77: [u8; 0x30],
    flm_cmdars0: FlmCmdars0,
    flm_cmdars1: FlmCmdars1,
    flm_cmdars2: FlmCmdars2,
    flm_cmdars3: FlmCmdars3,
    flm_cmdars4: FlmCmdars4,
    flm_cmdars5: FlmCmdars5,
    flm_cmdars6: FlmCmdars6,
    flm_cmdars7: FlmCmdars7,
    flm_cmdars8: FlmCmdars8,
    flm_cmdars9: FlmCmdars9,
    flm_cmdars10: FlmCmdars10,
    flm_cmdars11: FlmCmdars11,
    _reserved_89_flm_cmdars1: [u8; 0x04],
    _reserved_90_flm_cmdars1: [u8; 0x04],
    _reserved91: [u8; 0x08],
    flm_cmdars18: FlmCmdars18,
    flm_cmdars19: FlmCmdars19,
    flm_cmdars16: FlmCmdars16,
    flm_cmdars17: FlmCmdars17,
    flm_cmdars22: FlmCmdars22,
    flm_cmdars23: FlmCmdars23,
    flm_cmdars20: FlmCmdars20,
    flm_cmdars21: FlmCmdars21,
    flm_cmdars24: FlmCmdars24,
    flm_cmdars25: FlmCmdars25,
    flm_cmdars26: FlmCmdars26,
    flm_cmdars27: FlmCmdars27,
    flm_cmdars28: FlmCmdars28,
    flm_cmdars29: FlmCmdars29,
    flm_cmdars30: FlmCmdars30,
    flm_cmdars31: FlmCmdars31,
    flm_cmb0: FlmCmb0,
    flm_cmb1: FlmCmb1,
    flm_cmb2: FlmCmb2,
    flm_cmb3: FlmCmb3,
    flm_cmb4: FlmCmb4,
    flm_cmb5: FlmCmb5,
    flm_cmb6: FlmCmb6,
    flm_cmb7: FlmCmb7,
    flm_cmb8: FlmCmb8,
    flm_cmb9: FlmCmb9,
    flm_cmb10: FlmCmb10,
    flm_cmb11: FlmCmb11,
    _reserved_119_flm_cmb1: [u8; 0x04],
    _reserved_120_flm_cmb1: [u8; 0x04],
    _reserved121: [u8; 0x08],
    flm_cmb18: FlmCmb18,
    flm_cmb19: FlmCmb19,
    flm_cmb16: FlmCmb16,
    flm_cmb17: FlmCmb17,
    flm_cmb22: FlmCmb22,
    flm_cmb23: FlmCmb23,
    flm_cmb20: FlmCmb20,
    flm_cmb21: FlmCmb21,
    flm_cmb24: FlmCmb24,
    flm_cmb25: FlmCmb25,
    flm_cmb26: FlmCmb26,
    flm_cmb27: FlmCmb27,
    flm_cmb28: FlmCmb28,
    flm_cmb29: FlmCmb29,
    flm_cmb30: FlmCmb30,
    flm_cmb31: FlmCmb31,
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
    pub const fn flm_log1(&self) -> &FlmLog1 {
        &self.flm_log1
    }
    #[doc = "0x0c - FLM LOG Register 1-2 (FLM_LOG1-2)"]
    #[inline(always)]
    pub const fn flm_log2(&self) -> &FlmLog2 {
        &self.flm_log2
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
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb0 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd12(&self) -> &FlmCmd12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(176).cast() }
    }
    #[doc = "0xb4 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd15(&self) -> &FlmCmd15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(180).cast() }
    }
    #[doc = "0xb4 - FLM Command Register 0-31 (FLM_CMD0-31)"]
    #[inline(always)]
    pub const fn flm_cmd13(&self) -> &FlmCmd13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(180).cast() }
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
    pub const fn flm_cq0(&self) -> &FlmCq0 {
        &self.flm_cq0
    }
    #[doc = "0x1c4 - FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
    #[inline(always)]
    pub const fn flm_cq1(&self) -> &FlmCq1 {
        &self.flm_cq1
    }
    #[doc = "0x1c8 - FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
    #[inline(always)]
    pub const fn flm_cq2(&self) -> &FlmCq2 {
        &self.flm_cq2
    }
    #[doc = "0x1cc - FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
    #[inline(always)]
    pub const fn flm_cq3(&self) -> &FlmCq3 {
        &self.flm_cq3
    }
    #[doc = "0x200 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars0(&self) -> &FlmCmdars0 {
        &self.flm_cmdars0
    }
    #[doc = "0x204 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars1(&self) -> &FlmCmdars1 {
        &self.flm_cmdars1
    }
    #[doc = "0x208 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars2(&self) -> &FlmCmdars2 {
        &self.flm_cmdars2
    }
    #[doc = "0x20c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars3(&self) -> &FlmCmdars3 {
        &self.flm_cmdars3
    }
    #[doc = "0x210 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars4(&self) -> &FlmCmdars4 {
        &self.flm_cmdars4
    }
    #[doc = "0x214 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars5(&self) -> &FlmCmdars5 {
        &self.flm_cmdars5
    }
    #[doc = "0x218 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars6(&self) -> &FlmCmdars6 {
        &self.flm_cmdars6
    }
    #[doc = "0x21c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars7(&self) -> &FlmCmdars7 {
        &self.flm_cmdars7
    }
    #[doc = "0x220 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars8(&self) -> &FlmCmdars8 {
        &self.flm_cmdars8
    }
    #[doc = "0x224 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars9(&self) -> &FlmCmdars9 {
        &self.flm_cmdars9
    }
    #[doc = "0x228 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars10(&self) -> &FlmCmdars10 {
        &self.flm_cmdars10
    }
    #[doc = "0x22c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars11(&self) -> &FlmCmdars11 {
        &self.flm_cmdars11
    }
    #[doc = "0x230 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars14(&self) -> &FlmCmdars14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x230 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars12(&self) -> &FlmCmdars12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x234 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars15(&self) -> &FlmCmdars15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x234 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars13(&self) -> &FlmCmdars13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x240 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars18(&self) -> &FlmCmdars18 {
        &self.flm_cmdars18
    }
    #[doc = "0x244 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars19(&self) -> &FlmCmdars19 {
        &self.flm_cmdars19
    }
    #[doc = "0x248 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars16(&self) -> &FlmCmdars16 {
        &self.flm_cmdars16
    }
    #[doc = "0x24c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars17(&self) -> &FlmCmdars17 {
        &self.flm_cmdars17
    }
    #[doc = "0x250 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars22(&self) -> &FlmCmdars22 {
        &self.flm_cmdars22
    }
    #[doc = "0x254 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars23(&self) -> &FlmCmdars23 {
        &self.flm_cmdars23
    }
    #[doc = "0x258 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars20(&self) -> &FlmCmdars20 {
        &self.flm_cmdars20
    }
    #[doc = "0x25c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars21(&self) -> &FlmCmdars21 {
        &self.flm_cmdars21
    }
    #[doc = "0x260 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars24(&self) -> &FlmCmdars24 {
        &self.flm_cmdars24
    }
    #[doc = "0x264 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars25(&self) -> &FlmCmdars25 {
        &self.flm_cmdars25
    }
    #[doc = "0x268 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars26(&self) -> &FlmCmdars26 {
        &self.flm_cmdars26
    }
    #[doc = "0x26c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars27(&self) -> &FlmCmdars27 {
        &self.flm_cmdars27
    }
    #[doc = "0x270 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars28(&self) -> &FlmCmdars28 {
        &self.flm_cmdars28
    }
    #[doc = "0x274 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars29(&self) -> &FlmCmdars29 {
        &self.flm_cmdars29
    }
    #[doc = "0x278 - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars30(&self) -> &FlmCmdars30 {
        &self.flm_cmdars30
    }
    #[doc = "0x27c - FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
    #[inline(always)]
    pub const fn flm_cmdars31(&self) -> &FlmCmdars31 {
        &self.flm_cmdars31
    }
    #[doc = "0x280 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb0(&self) -> &FlmCmb0 {
        &self.flm_cmb0
    }
    #[doc = "0x284 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb1(&self) -> &FlmCmb1 {
        &self.flm_cmb1
    }
    #[doc = "0x288 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb2(&self) -> &FlmCmb2 {
        &self.flm_cmb2
    }
    #[doc = "0x28c - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb3(&self) -> &FlmCmb3 {
        &self.flm_cmb3
    }
    #[doc = "0x290 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb4(&self) -> &FlmCmb4 {
        &self.flm_cmb4
    }
    #[doc = "0x294 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb5(&self) -> &FlmCmb5 {
        &self.flm_cmb5
    }
    #[doc = "0x298 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb6(&self) -> &FlmCmb6 {
        &self.flm_cmb6
    }
    #[doc = "0x29c - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb7(&self) -> &FlmCmb7 {
        &self.flm_cmb7
    }
    #[doc = "0x2a0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb8(&self) -> &FlmCmb8 {
        &self.flm_cmb8
    }
    #[doc = "0x2a4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb9(&self) -> &FlmCmb9 {
        &self.flm_cmb9
    }
    #[doc = "0x2a8 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb10(&self) -> &FlmCmb10 {
        &self.flm_cmb10
    }
    #[doc = "0x2ac - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb11(&self) -> &FlmCmb11 {
        &self.flm_cmb11
    }
    #[doc = "0x2b0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb14(&self) -> &FlmCmb14 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(688).cast() }
    }
    #[doc = "0x2b0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb12(&self) -> &FlmCmb12 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(688).cast() }
    }
    #[doc = "0x2b4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb15(&self) -> &FlmCmb15 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(692).cast() }
    }
    #[doc = "0x2b4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb13(&self) -> &FlmCmb13 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(692).cast() }
    }
    #[doc = "0x2c0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb18(&self) -> &FlmCmb18 {
        &self.flm_cmb18
    }
    #[doc = "0x2c4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb19(&self) -> &FlmCmb19 {
        &self.flm_cmb19
    }
    #[doc = "0x2c8 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb16(&self) -> &FlmCmb16 {
        &self.flm_cmb16
    }
    #[doc = "0x2cc - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb17(&self) -> &FlmCmb17 {
        &self.flm_cmb17
    }
    #[doc = "0x2d0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb22(&self) -> &FlmCmb22 {
        &self.flm_cmb22
    }
    #[doc = "0x2d4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb23(&self) -> &FlmCmb23 {
        &self.flm_cmb23
    }
    #[doc = "0x2d8 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb20(&self) -> &FlmCmb20 {
        &self.flm_cmb20
    }
    #[doc = "0x2dc - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb21(&self) -> &FlmCmb21 {
        &self.flm_cmb21
    }
    #[doc = "0x2e0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb24(&self) -> &FlmCmb24 {
        &self.flm_cmb24
    }
    #[doc = "0x2e4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb25(&self) -> &FlmCmb25 {
        &self.flm_cmb25
    }
    #[doc = "0x2e8 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb26(&self) -> &FlmCmb26 {
        &self.flm_cmb26
    }
    #[doc = "0x2ec - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb27(&self) -> &FlmCmb27 {
        &self.flm_cmb27
    }
    #[doc = "0x2f0 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb28(&self) -> &FlmCmb28 {
        &self.flm_cmb28
    }
    #[doc = "0x2f4 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb29(&self) -> &FlmCmb29 {
        &self.flm_cmb29
    }
    #[doc = "0x2f8 - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb30(&self) -> &FlmCmb30 {
        &self.flm_cmb30
    }
    #[doc = "0x2fc - FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
    #[inline(always)]
    pub const fn flm_cmb31(&self) -> &FlmCmb31 {
        &self.flm_cmb31
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
#[doc = "FLM_LOG1 (rw) register accessor: FLM LOG Register 1-2 (FLM_LOG1-2)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_log1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_log1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_log1`]
module"]
#[doc(alias = "FLM_LOG1")]
pub type FlmLog1 = crate::Reg<flm_log1::FlmLog1Spec>;
#[doc = "FLM LOG Register 1-2 (FLM_LOG1-2)"]
pub mod flm_log1;
pub use flm_log1 as flm_log2;
pub use FlmLog1 as FlmLog2;
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
pub use flm_rang0 as flm_rang1;
pub use flm_rang0 as flm_rang2;
pub use flm_rang0 as flm_rang3;
pub use flm_rang0 as flm_rang4;
pub use flm_rang0 as flm_rang5;
pub use flm_rang0 as flm_rang6;
pub use flm_rang0 as flm_rang7;
pub use flm_rang0 as flm_rang8;
pub use flm_rang0 as flm_rang9;
pub use flm_rang0 as flm_rang10;
pub use flm_rang0 as flm_rang11;
pub use flm_rang0 as flm_rang12;
pub use flm_rang0 as flm_rang13;
pub use flm_rang0 as flm_rang14;
pub use flm_rang0 as flm_rang15;
pub use FlmRang0 as FlmRang1;
pub use FlmRang0 as FlmRang2;
pub use FlmRang0 as FlmRang3;
pub use FlmRang0 as FlmRang4;
pub use FlmRang0 as FlmRang5;
pub use FlmRang0 as FlmRang6;
pub use FlmRang0 as FlmRang7;
pub use FlmRang0 as FlmRang8;
pub use FlmRang0 as FlmRang9;
pub use FlmRang0 as FlmRang10;
pub use FlmRang0 as FlmRang11;
pub use FlmRang0 as FlmRang12;
pub use FlmRang0 as FlmRang13;
pub use FlmRang0 as FlmRang14;
pub use FlmRang0 as FlmRang15;
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
pub use flm_cmd0 as flm_cmd1;
pub use flm_cmd0 as flm_cmd2;
pub use flm_cmd0 as flm_cmd3;
pub use flm_cmd0 as flm_cmd4;
pub use flm_cmd0 as flm_cmd5;
pub use flm_cmd0 as flm_cmd6;
pub use flm_cmd0 as flm_cmd7;
pub use flm_cmd0 as flm_cmd8;
pub use flm_cmd0 as flm_cmd9;
pub use flm_cmd0 as flm_cmd10;
pub use flm_cmd0 as flm_cmd11;
pub use flm_cmd0 as flm_cmd12;
pub use flm_cmd0 as flm_cmd13;
pub use flm_cmd0 as flm_cmd14;
pub use flm_cmd0 as flm_cmd15;
pub use flm_cmd0 as flm_cmd16;
pub use flm_cmd0 as flm_cmd17;
pub use flm_cmd0 as flm_cmd18;
pub use flm_cmd0 as flm_cmd19;
pub use flm_cmd0 as flm_cmd20;
pub use flm_cmd0 as flm_cmd21;
pub use flm_cmd0 as flm_cmd22;
pub use flm_cmd0 as flm_cmd23;
pub use flm_cmd0 as flm_cmd24;
pub use flm_cmd0 as flm_cmd25;
pub use flm_cmd0 as flm_cmd26;
pub use flm_cmd0 as flm_cmd27;
pub use flm_cmd0 as flm_cmd28;
pub use flm_cmd0 as flm_cmd29;
pub use flm_cmd0 as flm_cmd30;
pub use flm_cmd0 as flm_cmd31;
pub use FlmCmd0 as FlmCmd1;
pub use FlmCmd0 as FlmCmd2;
pub use FlmCmd0 as FlmCmd3;
pub use FlmCmd0 as FlmCmd4;
pub use FlmCmd0 as FlmCmd5;
pub use FlmCmd0 as FlmCmd6;
pub use FlmCmd0 as FlmCmd7;
pub use FlmCmd0 as FlmCmd8;
pub use FlmCmd0 as FlmCmd9;
pub use FlmCmd0 as FlmCmd10;
pub use FlmCmd0 as FlmCmd11;
pub use FlmCmd0 as FlmCmd12;
pub use FlmCmd0 as FlmCmd13;
pub use FlmCmd0 as FlmCmd14;
pub use FlmCmd0 as FlmCmd15;
pub use FlmCmd0 as FlmCmd16;
pub use FlmCmd0 as FlmCmd17;
pub use FlmCmd0 as FlmCmd18;
pub use FlmCmd0 as FlmCmd19;
pub use FlmCmd0 as FlmCmd20;
pub use FlmCmd0 as FlmCmd21;
pub use FlmCmd0 as FlmCmd22;
pub use FlmCmd0 as FlmCmd23;
pub use FlmCmd0 as FlmCmd24;
pub use FlmCmd0 as FlmCmd25;
pub use FlmCmd0 as FlmCmd26;
pub use FlmCmd0 as FlmCmd27;
pub use FlmCmd0 as FlmCmd28;
pub use FlmCmd0 as FlmCmd29;
pub use FlmCmd0 as FlmCmd30;
pub use FlmCmd0 as FlmCmd31;
#[doc = "FLM_TCRA0 (rw) register accessor: FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcra0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcra0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcra0`]
module"]
#[doc(alias = "FLM_TCRA0")]
pub type FlmTcra0 = crate::Reg<flm_tcra0::FlmTcra0Spec>;
#[doc = "FLM Transaction Counter Read Register A0-A3 (FLM_TCRA)"]
pub mod flm_tcra0;
pub use flm_tcra0 as flm_tcra1;
pub use flm_tcra0 as flm_tcra2;
pub use flm_tcra0 as flm_tcra3;
pub use flm_tcra0 as flm_tcrb0;
pub use flm_tcra0 as flm_tcrb1;
pub use flm_tcra0 as flm_tcrb2;
pub use flm_tcra0 as flm_tcrb3;
pub use FlmTcra0 as FlmTcra1;
pub use FlmTcra0 as FlmTcra2;
pub use FlmTcra0 as FlmTcra3;
pub use FlmTcra0 as FlmTcrb0;
pub use FlmTcra0 as FlmTcrb1;
pub use FlmTcra0 as FlmTcrb2;
pub use FlmTcra0 as FlmTcrb3;
#[doc = "FLM_TCCA0 (rw) register accessor: FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcca0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcca0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcca0`]
module"]
#[doc(alias = "FLM_TCCA0")]
pub type FlmTcca0 = crate::Reg<flm_tcca0::FlmTcca0Spec>;
#[doc = "FLM Transaction Counter Control Register A0-A3 (FLM_TCCA)"]
pub mod flm_tcca0;
pub use flm_tcca0 as flm_tcca1;
pub use flm_tcca0 as flm_tcca2;
pub use flm_tcca0 as flm_tcca3;
pub use flm_tcca0 as flm_tccb0;
pub use flm_tcca0 as flm_tccb1;
pub use flm_tcca0 as flm_tccb2;
pub use flm_tcca0 as flm_tccb3;
pub use FlmTcca0 as FlmTcca1;
pub use FlmTcca0 as FlmTcca2;
pub use FlmTcca0 as FlmTcca3;
pub use FlmTcca0 as FlmTccb0;
pub use FlmTcca0 as FlmTccb1;
pub use FlmTcca0 as FlmTccb2;
pub use FlmTcca0 as FlmTccb3;
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
#[doc = "FLM_CQ0 (rw) register accessor: FLM Command Qualifier Register 0-3 (FLM_CQ0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cq0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cq0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cq0`]
module"]
#[doc(alias = "FLM_CQ0")]
pub type FlmCq0 = crate::Reg<flm_cq0::FlmCq0Spec>;
#[doc = "FLM Command Qualifier Register 0-3 (FLM_CQ0-3)"]
pub mod flm_cq0;
pub use flm_cq0 as flm_cq1;
pub use flm_cq0 as flm_cq2;
pub use flm_cq0 as flm_cq3;
pub use FlmCq0 as FlmCq1;
pub use FlmCq0 as FlmCq2;
pub use FlmCq0 as FlmCq3;
#[doc = "FLM_CMDARS0 (rw) register accessor: FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars0`]
module"]
#[doc(alias = "FLM_CMDARS0")]
pub type FlmCmdars0 = crate::Reg<flm_cmdars0::FlmCmdars0Spec>;
#[doc = "FLM Command Address Range Select Register 0-31 (FLM_CMDARS0-31)"]
pub mod flm_cmdars0;
pub use flm_cmdars0 as flm_cmdars1;
pub use flm_cmdars0 as flm_cmdars2;
pub use flm_cmdars0 as flm_cmdars3;
pub use flm_cmdars0 as flm_cmdars4;
pub use flm_cmdars0 as flm_cmdars5;
pub use flm_cmdars0 as flm_cmdars6;
pub use flm_cmdars0 as flm_cmdars7;
pub use flm_cmdars0 as flm_cmdars8;
pub use flm_cmdars0 as flm_cmdars9;
pub use flm_cmdars0 as flm_cmdars10;
pub use flm_cmdars0 as flm_cmdars11;
pub use flm_cmdars0 as flm_cmdars12;
pub use flm_cmdars0 as flm_cmdars13;
pub use flm_cmdars0 as flm_cmdars14;
pub use flm_cmdars0 as flm_cmdars15;
pub use flm_cmdars0 as flm_cmdars16;
pub use flm_cmdars0 as flm_cmdars17;
pub use flm_cmdars0 as flm_cmdars18;
pub use flm_cmdars0 as flm_cmdars19;
pub use flm_cmdars0 as flm_cmdars20;
pub use flm_cmdars0 as flm_cmdars21;
pub use flm_cmdars0 as flm_cmdars22;
pub use flm_cmdars0 as flm_cmdars23;
pub use flm_cmdars0 as flm_cmdars24;
pub use flm_cmdars0 as flm_cmdars25;
pub use flm_cmdars0 as flm_cmdars26;
pub use flm_cmdars0 as flm_cmdars27;
pub use flm_cmdars0 as flm_cmdars28;
pub use flm_cmdars0 as flm_cmdars29;
pub use flm_cmdars0 as flm_cmdars30;
pub use flm_cmdars0 as flm_cmdars31;
pub use FlmCmdars0 as FlmCmdars1;
pub use FlmCmdars0 as FlmCmdars2;
pub use FlmCmdars0 as FlmCmdars3;
pub use FlmCmdars0 as FlmCmdars4;
pub use FlmCmdars0 as FlmCmdars5;
pub use FlmCmdars0 as FlmCmdars6;
pub use FlmCmdars0 as FlmCmdars7;
pub use FlmCmdars0 as FlmCmdars8;
pub use FlmCmdars0 as FlmCmdars9;
pub use FlmCmdars0 as FlmCmdars10;
pub use FlmCmdars0 as FlmCmdars11;
pub use FlmCmdars0 as FlmCmdars12;
pub use FlmCmdars0 as FlmCmdars13;
pub use FlmCmdars0 as FlmCmdars14;
pub use FlmCmdars0 as FlmCmdars15;
pub use FlmCmdars0 as FlmCmdars16;
pub use FlmCmdars0 as FlmCmdars17;
pub use FlmCmdars0 as FlmCmdars18;
pub use FlmCmdars0 as FlmCmdars19;
pub use FlmCmdars0 as FlmCmdars20;
pub use FlmCmdars0 as FlmCmdars21;
pub use FlmCmdars0 as FlmCmdars22;
pub use FlmCmdars0 as FlmCmdars23;
pub use FlmCmdars0 as FlmCmdars24;
pub use FlmCmdars0 as FlmCmdars25;
pub use FlmCmdars0 as FlmCmdars26;
pub use FlmCmdars0 as FlmCmdars27;
pub use FlmCmdars0 as FlmCmdars28;
pub use FlmCmdars0 as FlmCmdars29;
pub use FlmCmdars0 as FlmCmdars30;
pub use FlmCmdars0 as FlmCmdars31;
#[doc = "FLM_CMB0 (rw) register accessor: FLM Command Byte Register 0-31 (FLM_CMB0-31)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb0`]
module"]
#[doc(alias = "FLM_CMB0")]
pub type FlmCmb0 = crate::Reg<flm_cmb0::FlmCmb0Spec>;
#[doc = "FLM Command Byte Register 0-31 (FLM_CMB0-31)"]
pub mod flm_cmb0;
pub use flm_cmb0 as flm_cmb1;
pub use flm_cmb0 as flm_cmb2;
pub use flm_cmb0 as flm_cmb3;
pub use flm_cmb0 as flm_cmb4;
pub use flm_cmb0 as flm_cmb5;
pub use flm_cmb0 as flm_cmb6;
pub use flm_cmb0 as flm_cmb7;
pub use flm_cmb0 as flm_cmb8;
pub use flm_cmb0 as flm_cmb9;
pub use flm_cmb0 as flm_cmb10;
pub use flm_cmb0 as flm_cmb11;
pub use flm_cmb0 as flm_cmb12;
pub use flm_cmb0 as flm_cmb13;
pub use flm_cmb0 as flm_cmb14;
pub use flm_cmb0 as flm_cmb15;
pub use flm_cmb0 as flm_cmb16;
pub use flm_cmb0 as flm_cmb17;
pub use flm_cmb0 as flm_cmb18;
pub use flm_cmb0 as flm_cmb19;
pub use flm_cmb0 as flm_cmb20;
pub use flm_cmb0 as flm_cmb21;
pub use flm_cmb0 as flm_cmb22;
pub use flm_cmb0 as flm_cmb23;
pub use flm_cmb0 as flm_cmb24;
pub use flm_cmb0 as flm_cmb25;
pub use flm_cmb0 as flm_cmb26;
pub use flm_cmb0 as flm_cmb27;
pub use flm_cmb0 as flm_cmb28;
pub use flm_cmb0 as flm_cmb29;
pub use flm_cmb0 as flm_cmb30;
pub use flm_cmb0 as flm_cmb31;
pub use FlmCmb0 as FlmCmb1;
pub use FlmCmb0 as FlmCmb2;
pub use FlmCmb0 as FlmCmb3;
pub use FlmCmb0 as FlmCmb4;
pub use FlmCmb0 as FlmCmb5;
pub use FlmCmb0 as FlmCmb6;
pub use FlmCmb0 as FlmCmb7;
pub use FlmCmb0 as FlmCmb8;
pub use FlmCmb0 as FlmCmb9;
pub use FlmCmb0 as FlmCmb10;
pub use FlmCmb0 as FlmCmb11;
pub use FlmCmb0 as FlmCmb12;
pub use FlmCmb0 as FlmCmb13;
pub use FlmCmb0 as FlmCmb14;
pub use FlmCmb0 as FlmCmb15;
pub use FlmCmb0 as FlmCmb16;
pub use FlmCmb0 as FlmCmb17;
pub use FlmCmb0 as FlmCmb18;
pub use FlmCmb0 as FlmCmb19;
pub use FlmCmb0 as FlmCmb20;
pub use FlmCmb0 as FlmCmb21;
pub use FlmCmb0 as FlmCmb22;
pub use FlmCmb0 as FlmCmb23;
pub use FlmCmb0 as FlmCmb24;
pub use FlmCmb0 as FlmCmb25;
pub use FlmCmb0 as FlmCmb26;
pub use FlmCmb0 as FlmCmb27;
pub use FlmCmb0 as FlmCmb28;
pub use FlmCmb0 as FlmCmb29;
pub use FlmCmb0 as FlmCmb30;
pub use FlmCmb0 as FlmCmb31;
