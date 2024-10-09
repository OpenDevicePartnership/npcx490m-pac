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
    _reserved_6_flm: [u8; 0xe0],
    flm_tcra: [FlmTcra; 4],
    flm_tcrb: [FlmTcrb; 4],
    _reserved9: [u8; 0x20],
    flm_tcca: [FlmTcca; 4],
    flm_tccb: [FlmTccb; 4],
    _reserved11: [u8; 0x20],
    flm_cmdev: FlmCmdev,
    flm_cmbev: FlmCmbev,
    flm_tcgc: FlmTcgc,
    _reserved14: [u8; 0x34],
    flm_cq: [FlmCq; 4],
    _reserved15: [u8; 0x30],
    flm_cmdars: [FlmCmdars; 32],
    flm_cmb: [FlmCmb; 32],
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
    #[doc = "0x08 - FLM LOG Register 1"]
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
    #[doc = "0x20..0xa0 - FLM Range Register %s"]
    #[inline(always)]
    pub const fn flm_rang(&self, n: usize) -> &FlmRang {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(32).add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0xa0 - FLM Range Register %s"]
    #[inline(always)]
    pub fn flm_rang_iter(&self) -> impl Iterator<Item = &FlmRang> {
        (0..32)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(32).add(4 * n).cast() })
    }
    #[doc = "0x60 - FLM Command Enable (FLM_CMDEN)"]
    #[inline(always)]
    pub const fn flm_cmden(&self) -> &FlmCmden {
        unsafe { &*(self as *const Self).cast::<u8>().add(96).cast() }
    }
    #[doc = "0x64 - FLM Command Byte Enable (FLM_CMBEN)"]
    #[inline(always)]
    pub const fn flm_cmben(&self) -> &FlmCmben {
        unsafe { &*(self as *const Self).cast::<u8>().add(100).cast() }
    }
    #[doc = "0x80..0x100 - FLM Command Register %s"]
    #[inline(always)]
    pub const fn flm_cmd(&self, n: usize) -> &FlmCmd {
        #[allow(clippy::no_effect)]
        [(); 32][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(128)
                .add(4 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - FLM Command Register %s"]
    #[inline(always)]
    pub fn flm_cmd_iter(&self) -> impl Iterator<Item = &FlmCmd> {
        (0..32).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(128)
                .add(4 * n)
                .cast()
        })
    }
    #[doc = "0x100..0x110 - FLM Transaction Counter Read Register A%s"]
    #[inline(always)]
    pub const fn flm_tcra(&self, n: usize) -> &FlmTcra {
        &self.flm_tcra[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - FLM Transaction Counter Read Register A%s"]
    #[inline(always)]
    pub fn flm_tcra_iter(&self) -> impl Iterator<Item = &FlmTcra> {
        self.flm_tcra.iter()
    }
    #[doc = "0x110..0x120 - FLM Transaction Counter Read Register B%s"]
    #[inline(always)]
    pub const fn flm_tcrb(&self, n: usize) -> &FlmTcrb {
        &self.flm_tcrb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x120 - FLM Transaction Counter Read Register B%s"]
    #[inline(always)]
    pub fn flm_tcrb_iter(&self) -> impl Iterator<Item = &FlmTcrb> {
        self.flm_tcrb.iter()
    }
    #[doc = "0x140..0x150 - FLM Transaction Counter Control Register A%s"]
    #[inline(always)]
    pub const fn flm_tcca(&self, n: usize) -> &FlmTcca {
        &self.flm_tcca[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x150 - FLM Transaction Counter Control Register A%s"]
    #[inline(always)]
    pub fn flm_tcca_iter(&self) -> impl Iterator<Item = &FlmTcca> {
        self.flm_tcca.iter()
    }
    #[doc = "0x150..0x160 - FLM Transaction Counter Control Register B%s"]
    #[inline(always)]
    pub const fn flm_tccb(&self, n: usize) -> &FlmTccb {
        &self.flm_tccb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x150..0x160 - FLM Transaction Counter Control Register B%s"]
    #[inline(always)]
    pub fn flm_tccb_iter(&self) -> impl Iterator<Item = &FlmTccb> {
        self.flm_tccb.iter()
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
    #[doc = "0x1c0..0x1d0 - FLM Command Qualifier Register %s"]
    #[inline(always)]
    pub const fn flm_cq(&self, n: usize) -> &FlmCq {
        &self.flm_cq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x1d0 - FLM Command Qualifier Register %s"]
    #[inline(always)]
    pub fn flm_cq_iter(&self) -> impl Iterator<Item = &FlmCq> {
        self.flm_cq.iter()
    }
    #[doc = "0x200..0x280 - FLM Command Address Range Select Register %s"]
    #[inline(always)]
    pub const fn flm_cmdars(&self, n: usize) -> &FlmCmdars {
        &self.flm_cmdars[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x280 - FLM Command Address Range Select Register %s"]
    #[inline(always)]
    pub fn flm_cmdars_iter(&self) -> impl Iterator<Item = &FlmCmdars> {
        self.flm_cmdars.iter()
    }
    #[doc = "0x280..0x300 - FLM Command Byte Register %s"]
    #[inline(always)]
    pub const fn flm_cmb(&self, n: usize) -> &FlmCmb {
        &self.flm_cmb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x280..0x300 - FLM Command Byte Register %s"]
    #[inline(always)]
    pub fn flm_cmb_iter(&self) -> impl Iterator<Item = &FlmCmb> {
        self.flm_cmb.iter()
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
#[doc = "FLM_LOG1 (r) register accessor: FLM LOG Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_log1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_log1`]
module"]
#[doc(alias = "FLM_LOG1")]
pub type FlmLog1 = crate::Reg<flm_log1::FlmLog1Spec>;
#[doc = "FLM LOG Register 1"]
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
#[doc = "FLM_RANG (rw) register accessor: FLM Range Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_rang::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_rang::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_rang`]
module"]
#[doc(alias = "FLM_RANG")]
pub type FlmRang = crate::Reg<flm_rang::FlmRangSpec>;
#[doc = "FLM Range Register %s"]
pub mod flm_rang;
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
#[doc = "FLM_CMD (rw) register accessor: FLM Command Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmd`]
module"]
#[doc(alias = "FLM_CMD")]
pub type FlmCmd = crate::Reg<flm_cmd::FlmCmdSpec>;
#[doc = "FLM Command Register %s"]
pub mod flm_cmd;
#[doc = "FLM_TCRA (r) register accessor: FLM Transaction Counter Read Register A%s\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcra::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcra`]
module"]
#[doc(alias = "FLM_TCRA")]
pub type FlmTcra = crate::Reg<flm_tcra::FlmTcraSpec>;
#[doc = "FLM Transaction Counter Read Register A%s"]
pub mod flm_tcra;
#[doc = "FLM_TCRB (r) register accessor: FLM Transaction Counter Read Register B%s\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcrb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcrb`]
module"]
#[doc(alias = "FLM_TCRB")]
pub type FlmTcrb = crate::Reg<flm_tcrb::FlmTcrbSpec>;
#[doc = "FLM Transaction Counter Read Register B%s"]
pub mod flm_tcrb;
#[doc = "FLM_TCCA (rw) register accessor: FLM Transaction Counter Control Register A%s\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tcca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcca`]
module"]
#[doc(alias = "FLM_TCCA")]
pub type FlmTcca = crate::Reg<flm_tcca::FlmTccaSpec>;
#[doc = "FLM Transaction Counter Control Register A%s"]
pub mod flm_tcca;
#[doc = "FLM_TCCB (rw) register accessor: FLM Transaction Counter Control Register B%s\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_tccb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tccb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tccb`]
module"]
#[doc(alias = "FLM_TCCB")]
pub type FlmTccb = crate::Reg<flm_tccb::FlmTccbSpec>;
#[doc = "FLM Transaction Counter Control Register B%s"]
pub mod flm_tccb;
#[doc = "FLM_CMDEV (r) register accessor: FLM CMD Event Register (FLM_CMDEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdev`]
module"]
#[doc(alias = "FLM_CMDEV")]
pub type FlmCmdev = crate::Reg<flm_cmdev::FlmCmdevSpec>;
#[doc = "FLM CMD Event Register (FLM_CMDEV)"]
pub mod flm_cmdev;
#[doc = "FLM_CMBEV (r) register accessor: FLM CMB Event Register (FLM_CMBEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmbev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmbev`]
module"]
#[doc(alias = "FLM_CMBEV")]
pub type FlmCmbev = crate::Reg<flm_cmbev::FlmCmbevSpec>;
#[doc = "FLM CMB Event Register (FLM_CMBEV)"]
pub mod flm_cmbev;
#[doc = "FLM_TCGC (w) register accessor: FLM Transaction Counter Global Control Register (FLM_TCGC)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_tcgc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_tcgc`]
module"]
#[doc(alias = "FLM_TCGC")]
pub type FlmTcgc = crate::Reg<flm_tcgc::FlmTcgcSpec>;
#[doc = "FLM Transaction Counter Global Control Register (FLM_TCGC)"]
pub mod flm_tcgc;
#[doc = "FLM_CQ (rw) register accessor: FLM Command Qualifier Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cq`]
module"]
#[doc(alias = "FLM_CQ")]
pub type FlmCq = crate::Reg<flm_cq::FlmCqSpec>;
#[doc = "FLM Command Qualifier Register %s"]
pub mod flm_cq;
#[doc = "FLM_CMDARS (rw) register accessor: FLM Command Address Range Select Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmdars::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmdars::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmdars`]
module"]
#[doc(alias = "FLM_CMDARS")]
pub type FlmCmdars = crate::Reg<flm_cmdars::FlmCmdarsSpec>;
#[doc = "FLM Command Address Range Select Register %s"]
pub mod flm_cmdars;
#[doc = "FLM_CMB (rw) register accessor: FLM Command Byte Register %s\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flm_cmb`]
module"]
#[doc(alias = "FLM_CMB")]
pub type FlmCmb = crate::Reg<flm_cmb::FlmCmbSpec>;
#[doc = "FLM Command Byte Register %s"]
pub mod flm_cmb;
