#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x02],
    smb_sbd: SmbSbd,
    smb_een: SmbEen,
    _reserved2: [u8; 0x0c],
    sdpd0: Sdpd0,
    _reserved3: [u8; 0x01],
    sdpd1: Sdpd1,
    _reserved4: [u8; 0x01],
    sdp_cts: SdpCts,
    _reserved5: [u8; 0x0c],
    smb_sel: SmbSel,
    _reserved6: [u8; 0x05],
    psl_cts: PslCts,
    _reserved7: [u8; 0x0c],
    psl_mctl1: PslMctl1,
    _reserved8: [u8; 0x03],
    psl_mctl2: PslMctl2,
    _reserved9: [u8; 0x01],
    psl_in_sts: PslInSts,
    _reserved10: [u8; 0x01],
    psl_dbc: PslDbc,
    _reserved11: [u8; 0x03],
    snt_sts: SntSts,
    _reserved12: [u8; 0x01],
    snt_ctl: SntCtl,
    snt_cfg: SntCfg,
}
impl RegisterBlock {
    #[doc = "0x02 - SMBus/I2C Start Bit Detection Register (SMB_SBD)"]
    #[inline(always)]
    pub const fn smb_sbd(&self) -> &SmbSbd {
        &self.smb_sbd
    }
    #[doc = "0x03 - SMBus/I2C Event Enable Register (SMB_EEN)"]
    #[inline(always)]
    pub const fn smb_een(&self) -> &SmbEen {
        &self.smb_een
    }
    #[doc = "0x10 - Simple Debug Port Data 0 Register (SDPD0)"]
    #[inline(always)]
    pub const fn sdpd0(&self) -> &Sdpd0 {
        &self.sdpd0
    }
    #[doc = "0x12 - Simple Debug Port Data 1 Register (SDPD1)"]
    #[inline(always)]
    pub const fn sdpd1(&self) -> &Sdpd1 {
        &self.sdpd1
    }
    #[doc = "0x14 - Simple Debug Port Control and Status Register (SDP_CTS)"]
    #[inline(always)]
    pub const fn sdp_cts(&self) -> &SdpCts {
        &self.sdp_cts
    }
    #[doc = "0x21 - SMBus/I2C Bus Select Register (SMB_SEL)"]
    #[inline(always)]
    pub const fn smb_sel(&self) -> &SmbSel {
        &self.smb_sel
    }
    #[doc = "0x27 - PSL Control and Status Register (PSL_CTS)"]
    #[inline(always)]
    pub const fn psl_cts(&self) -> &PslCts {
        &self.psl_cts
    }
    #[doc = "0x34 - PSL Miscellaneous Control 1 Register (PSL_MCTL1)"]
    #[inline(always)]
    pub const fn psl_mctl1(&self) -> &PslMctl1 {
        &self.psl_mctl1
    }
    #[doc = "0x38 - PSL Miscellaneous Control 2 Register (PSL_MCTL2)"]
    #[inline(always)]
    pub const fn psl_mctl2(&self) -> &PslMctl2 {
        &self.psl_mctl2
    }
    #[doc = "0x3a - PSL Input Status Register (PSL_IN_STS)"]
    #[inline(always)]
    pub const fn psl_in_sts(&self) -> &PslInSts {
        &self.psl_in_sts
    }
    #[doc = "0x3c - PSL Debounce Register (PSL_DBC)"]
    #[inline(always)]
    pub const fn psl_dbc(&self) -> &PslDbc {
        &self.psl_dbc
    }
    #[doc = "0x40 - Snooze Timer Status Register (SNT_STS)"]
    #[inline(always)]
    pub const fn snt_sts(&self) -> &SntSts {
        &self.snt_sts
    }
    #[doc = "0x42 - Snooze Timer Control Register (SNT_CTL)"]
    #[inline(always)]
    pub const fn snt_ctl(&self) -> &SntCtl {
        &self.snt_ctl
    }
    #[doc = "0x44 - Snooze Timer Configuration Register (SNT_CFG)"]
    #[inline(always)]
    pub const fn snt_cfg(&self) -> &SntCfg {
        &self.snt_cfg
    }
}
#[doc = "SMB_SBD (rw) register accessor: SMBus/I2C Start Bit Detection Register (SMB_SBD)\n\nYou can [`read`](crate::Reg::read) this register and get [`smb_sbd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smb_sbd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smb_sbd`]
module"]
#[doc(alias = "SMB_SBD")]
pub type SmbSbd = crate::Reg<smb_sbd::SmbSbdSpec>;
#[doc = "SMBus/I2C Start Bit Detection Register (SMB_SBD)"]
pub mod smb_sbd;
#[doc = "SMB_EEN (rw) register accessor: SMBus/I2C Event Enable Register (SMB_EEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`smb_een::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smb_een::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smb_een`]
module"]
#[doc(alias = "SMB_EEN")]
pub type SmbEen = crate::Reg<smb_een::SmbEenSpec>;
#[doc = "SMBus/I2C Event Enable Register (SMB_EEN)"]
pub mod smb_een;
#[doc = "SDPD0 (rw) register accessor: Simple Debug Port Data 0 Register (SDPD0)\n\nYou can [`read`](crate::Reg::read) this register and get [`sdpd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdpd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdpd0`]
module"]
#[doc(alias = "SDPD0")]
pub type Sdpd0 = crate::Reg<sdpd0::Sdpd0Spec>;
#[doc = "Simple Debug Port Data 0 Register (SDPD0)"]
pub mod sdpd0;
#[doc = "SDPD1 (rw) register accessor: Simple Debug Port Data 1 Register (SDPD1)\n\nYou can [`read`](crate::Reg::read) this register and get [`sdpd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdpd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdpd1`]
module"]
#[doc(alias = "SDPD1")]
pub type Sdpd1 = crate::Reg<sdpd1::Sdpd1Spec>;
#[doc = "Simple Debug Port Data 1 Register (SDPD1)"]
pub mod sdpd1;
#[doc = "SDP_CTS (rw) register accessor: Simple Debug Port Control and Status Register (SDP_CTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`sdp_cts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdp_cts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdp_cts`]
module"]
#[doc(alias = "SDP_CTS")]
pub type SdpCts = crate::Reg<sdp_cts::SdpCtsSpec>;
#[doc = "Simple Debug Port Control and Status Register (SDP_CTS)"]
pub mod sdp_cts;
#[doc = "SMB_SEL (rw) register accessor: SMBus/I2C Bus Select Register (SMB_SEL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smb_sel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smb_sel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smb_sel`]
module"]
#[doc(alias = "SMB_SEL")]
pub type SmbSel = crate::Reg<smb_sel::SmbSelSpec>;
#[doc = "SMBus/I2C Bus Select Register (SMB_SEL)"]
pub mod smb_sel;
#[doc = "PSL_CTS (rw) register accessor: PSL Control and Status Register (PSL_CTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`psl_cts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl_cts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psl_cts`]
module"]
#[doc(alias = "PSL_CTS")]
pub type PslCts = crate::Reg<psl_cts::PslCtsSpec>;
#[doc = "PSL Control and Status Register (PSL_CTS)"]
pub mod psl_cts;
#[doc = "PSL_MCTL1 (rw) register accessor: PSL Miscellaneous Control 1 Register (PSL_MCTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`psl_mctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl_mctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psl_mctl1`]
module"]
#[doc(alias = "PSL_MCTL1")]
pub type PslMctl1 = crate::Reg<psl_mctl1::PslMctl1Spec>;
#[doc = "PSL Miscellaneous Control 1 Register (PSL_MCTL1)"]
pub mod psl_mctl1;
#[doc = "PSL_MCTL2 (rw) register accessor: PSL Miscellaneous Control 2 Register (PSL_MCTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`psl_mctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl_mctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psl_mctl2`]
module"]
#[doc(alias = "PSL_MCTL2")]
pub type PslMctl2 = crate::Reg<psl_mctl2::PslMctl2Spec>;
#[doc = "PSL Miscellaneous Control 2 Register (PSL_MCTL2)"]
pub mod psl_mctl2;
#[doc = "PSL_IN_STS (rw) register accessor: PSL Input Status Register (PSL_IN_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`psl_in_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl_in_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psl_in_sts`]
module"]
#[doc(alias = "PSL_IN_STS")]
pub type PslInSts = crate::Reg<psl_in_sts::PslInStsSpec>;
#[doc = "PSL Input Status Register (PSL_IN_STS)"]
pub mod psl_in_sts;
#[doc = "PSL_DBC (rw) register accessor: PSL Debounce Register (PSL_DBC)\n\nYou can [`read`](crate::Reg::read) this register and get [`psl_dbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psl_dbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psl_dbc`]
module"]
#[doc(alias = "PSL_DBC")]
pub type PslDbc = crate::Reg<psl_dbc::PslDbcSpec>;
#[doc = "PSL Debounce Register (PSL_DBC)"]
pub mod psl_dbc;
#[doc = "SNT_STS (rw) register accessor: Snooze Timer Status Register (SNT_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`snt_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snt_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snt_sts`]
module"]
#[doc(alias = "SNT_STS")]
pub type SntSts = crate::Reg<snt_sts::SntStsSpec>;
#[doc = "Snooze Timer Status Register (SNT_STS)"]
pub mod snt_sts;
#[doc = "SNT_CTL (rw) register accessor: Snooze Timer Control Register (SNT_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`snt_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snt_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snt_ctl`]
module"]
#[doc(alias = "SNT_CTL")]
pub type SntCtl = crate::Reg<snt_ctl::SntCtlSpec>;
#[doc = "Snooze Timer Control Register (SNT_CTL)"]
pub mod snt_ctl;
#[doc = "SNT_CFG (rw) register accessor: Snooze Timer Configuration Register (SNT_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`snt_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snt_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@snt_cfg`]
module"]
#[doc(alias = "SNT_CFG")]
pub type SntCfg = crate::Reg<snt_cfg::SntCfgSpec>;
#[doc = "Snooze Timer Configuration Register (SNT_CFG)"]
pub mod snt_cfg;
