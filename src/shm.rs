#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    smc_sts: SmcSts,
    smc_ctl: SmcCtl,
    shm_ctl: ShmCtl,
    _reserved3: [u8; 0x02],
    ima_win_size: ImaWinSize,
    _reserved4: [u8; 0x01],
    win_size: WinSize,
    shaw12_sem0: Shaw12Sem0,
    shaw12_sem1: Shaw12Sem1,
    _reserved7: [u8; 0x01],
    ima_sem: ImaSem,
    _reserved8: [u8; 0x02],
    shcfg: Shcfg,
    win1_wr_prot: Win1WrProt,
    win1_rd_prot: Win1RdProt,
    win2_wr_prot: Win2WrProt,
    win2_rd_prot: Win2RdProt,
    _reserved13: [u8; 0x02],
    ima_wr_prot: ImaWrProt,
    ima_rd_prot: ImaRdProt,
    _reserved15: [u8; 0x08],
    win_base1: WinBase1,
    win_base2: WinBase2,
    _reserved17: [u8; 0x04],
    ima_base: ImaBase,
    _reserved18: [u8; 0x0a],
    rst_cfg: RstCfg,
    _reserved19: [u8; 0x05],
    dp80buf: Dp80buf,
    dp80sts: Dp80sts,
    _reserved21: [u8; 0x01],
    dp80ctl: Dp80ctl,
    _reserved22: [u8; 0x03],
    hofs_sts: HofsSts,
    hofs_ctl: HofsCtl,
    cofs2: Cofs2,
    cofs1: Cofs1,
}
impl RegisterBlock {
    #[doc = "0x00 - Shared Memory Core Status Register (SMC_STS)"]
    #[inline(always)]
    pub const fn smc_sts(&self) -> &SmcSts {
        &self.smc_sts
    }
    #[doc = "0x01 - Shared Memory Core Control Register (SMC_CTL)"]
    #[inline(always)]
    pub const fn smc_ctl(&self) -> &SmcCtl {
        &self.smc_ctl
    }
    #[doc = "0x02 - Shared Memory Control Register (SHM_CTL)"]
    #[inline(always)]
    pub const fn shm_ctl(&self) -> &ShmCtl {
        &self.shm_ctl
    }
    #[doc = "0x05 - Indirect Memory Access Window Size Register (IMA_WIN_SIZE)"]
    #[inline(always)]
    pub const fn ima_win_size(&self) -> &ImaWinSize {
        &self.ima_win_size
    }
    #[doc = "0x07 - Shared Access Windows Size Register (WIN_SIZE)"]
    #[inline(always)]
    pub const fn win_size(&self) -> &WinSize {
        &self.win_size
    }
    #[doc = "0x08 - Shared Access Window 1-2, Semaphore Register (SHAW1-2_SEM)"]
    #[inline(always)]
    pub const fn shaw12_sem0(&self) -> &Shaw12Sem0 {
        &self.shaw12_sem0
    }
    #[doc = "0x09 - Shared Access Window 1-2, Semaphore Register (SHAW1-2_SEM)"]
    #[inline(always)]
    pub const fn shaw12_sem1(&self) -> &Shaw12Sem1 {
        &self.shaw12_sem1
    }
    #[doc = "0x0b - Indirect Memory Access, Semaphore Register (IMA_SEM)"]
    #[inline(always)]
    pub const fn ima_sem(&self) -> &ImaSem {
        &self.ima_sem
    }
    #[doc = "0x0e - Shared Memory Configuration Register (SHCFG)"]
    #[inline(always)]
    pub const fn shcfg(&self) -> &Shcfg {
        &self.shcfg
    }
    #[doc = "0x10 - Shared Access Window 1 Write Protect Register (WIN1_WR_PROT)"]
    #[inline(always)]
    pub const fn win1_wr_prot(&self) -> &Win1WrProt {
        &self.win1_wr_prot
    }
    #[doc = "0x11 - Shared Access Window 1 Read Protect Register (WIN1_RD_PROT)"]
    #[inline(always)]
    pub const fn win1_rd_prot(&self) -> &Win1RdProt {
        &self.win1_rd_prot
    }
    #[doc = "0x12 - Shared Access Window 2 Write Protect Register (WIN2_WR_PROT)"]
    #[inline(always)]
    pub const fn win2_wr_prot(&self) -> &Win2WrProt {
        &self.win2_wr_prot
    }
    #[doc = "0x13 - Shared Access Window 2 Read Protect Register (WIN2_RD_PROT)"]
    #[inline(always)]
    pub const fn win2_rd_prot(&self) -> &Win2RdProt {
        &self.win2_rd_prot
    }
    #[doc = "0x16 - Indirect Memory Access Write Protect Register (IMA_WR_PROT)"]
    #[inline(always)]
    pub const fn ima_wr_prot(&self) -> &ImaWrProt {
        &self.ima_wr_prot
    }
    #[doc = "0x17 - Indirect Memory Access Read Protect Register (IMA_RD_PROT)"]
    #[inline(always)]
    pub const fn ima_rd_prot(&self) -> &ImaRdProt {
        &self.ima_rd_prot
    }
    #[doc = "0x20 - Shared Access Window 1 Base Register (WIN_BASE1)"]
    #[inline(always)]
    pub const fn win_base1(&self) -> &WinBase1 {
        &self.win_base1
    }
    #[doc = "0x24 - Shared Access Window 2 Base Register (WIN_BASE2)"]
    #[inline(always)]
    pub const fn win_base2(&self) -> &WinBase2 {
        &self.win_base2
    }
    #[doc = "0x2c - Indirect Memory Access Window Base Register (IMA_BASE)"]
    #[inline(always)]
    pub const fn ima_base(&self) -> &ImaBase {
        &self.ima_base
    }
    #[doc = "0x3a - Reset Configuration Register (RST_CFG)"]
    #[inline(always)]
    pub const fn rst_cfg(&self) -> &RstCfg {
        &self.rst_cfg
    }
    #[doc = "0x40 - Debug Port 80 Buffered Data Register (DP80BUF)"]
    #[inline(always)]
    pub const fn dp80buf(&self) -> &Dp80buf {
        &self.dp80buf
    }
    #[doc = "0x42 - Debug Port 80 Status Register (DP80STS)"]
    #[inline(always)]
    pub const fn dp80sts(&self) -> &Dp80sts {
        &self.dp80sts
    }
    #[doc = "0x44 - Debug Port 80 Control Register (DP80CTL)"]
    #[inline(always)]
    pub const fn dp80ctl(&self) -> &Dp80ctl {
        &self.dp80ctl
    }
    #[doc = "0x48 - Host_Offset in Windows 1,2 Status Register (HOFS_STS)"]
    #[inline(always)]
    pub const fn hofs_sts(&self) -> &HofsSts {
        &self.hofs_sts
    }
    #[doc = "0x49 - Host_Offset in Windows 1,2 Control Register (HOFS_CTL)"]
    #[inline(always)]
    pub const fn hofs_ctl(&self) -> &HofsCtl {
        &self.hofs_ctl
    }
    #[doc = "0x4a - Core_Offset in Window 2 Address Register (COFS2)"]
    #[inline(always)]
    pub const fn cofs2(&self) -> &Cofs2 {
        &self.cofs2
    }
    #[doc = "0x4c - Core_Offset in Window 1 Address Register (COFS1)"]
    #[inline(always)]
    pub const fn cofs1(&self) -> &Cofs1 {
        &self.cofs1
    }
}
#[doc = "SMC_STS (rw) register accessor: Shared Memory Core Status Register (SMC_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smc_sts`]
module"]
#[doc(alias = "SMC_STS")]
pub type SmcSts = crate::Reg<smc_sts::SmcStsSpec>;
#[doc = "Shared Memory Core Status Register (SMC_STS)"]
pub mod smc_sts;
#[doc = "SMC_CTL (rw) register accessor: Shared Memory Core Control Register (SMC_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smc_ctl`]
module"]
#[doc(alias = "SMC_CTL")]
pub type SmcCtl = crate::Reg<smc_ctl::SmcCtlSpec>;
#[doc = "Shared Memory Core Control Register (SMC_CTL)"]
pub mod smc_ctl;
#[doc = "SHM_CTL (rw) register accessor: Shared Memory Control Register (SHM_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`shm_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shm_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shm_ctl`]
module"]
#[doc(alias = "SHM_CTL")]
pub type ShmCtl = crate::Reg<shm_ctl::ShmCtlSpec>;
#[doc = "Shared Memory Control Register (SHM_CTL)"]
pub mod shm_ctl;
#[doc = "IMA_WIN_SIZE (rw) register accessor: Indirect Memory Access Window Size Register (IMA_WIN_SIZE)\n\nYou can [`read`](crate::Reg::read) this register and get [`ima_win_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ima_win_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ima_win_size`]
module"]
#[doc(alias = "IMA_WIN_SIZE")]
pub type ImaWinSize = crate::Reg<ima_win_size::ImaWinSizeSpec>;
#[doc = "Indirect Memory Access Window Size Register (IMA_WIN_SIZE)"]
pub mod ima_win_size;
#[doc = "WIN_SIZE (rw) register accessor: Shared Access Windows Size Register (WIN_SIZE)\n\nYou can [`read`](crate::Reg::read) this register and get [`win_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win_size`]
module"]
#[doc(alias = "WIN_SIZE")]
pub type WinSize = crate::Reg<win_size::WinSizeSpec>;
#[doc = "Shared Access Windows Size Register (WIN_SIZE)"]
pub mod win_size;
#[doc = "SHAW1-2_SEM0 (rw) register accessor: Shared Access Window 1-2, Semaphore Register (SHAW1-2_SEM)\n\nYou can [`read`](crate::Reg::read) this register and get [`shaw12_sem0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shaw12_sem0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shaw12_sem0`]
module"]
#[doc(alias = "SHAW1-2_SEM0")]
pub type Shaw12Sem0 = crate::Reg<shaw12_sem0::Shaw12Sem0Spec>;
#[doc = "Shared Access Window 1-2, Semaphore Register (SHAW1-2_SEM)"]
pub mod shaw12_sem0;
#[doc = "SHAW1-2_SEM1 (rw) register accessor: Shared Access Window 1-2, Semaphore Register (SHAW1-2_SEM)\n\nYou can [`read`](crate::Reg::read) this register and get [`shaw12_sem1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shaw12_sem1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shaw12_sem1`]
module"]
#[doc(alias = "SHAW1-2_SEM1")]
pub type Shaw12Sem1 = crate::Reg<shaw12_sem1::Shaw12Sem1Spec>;
#[doc = "Shared Access Window 1-2, Semaphore Register (SHAW1-2_SEM)"]
pub mod shaw12_sem1;
#[doc = "IMA_SEM (rw) register accessor: Indirect Memory Access, Semaphore Register (IMA_SEM)\n\nYou can [`read`](crate::Reg::read) this register and get [`ima_sem::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ima_sem::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ima_sem`]
module"]
#[doc(alias = "IMA_SEM")]
pub type ImaSem = crate::Reg<ima_sem::ImaSemSpec>;
#[doc = "Indirect Memory Access, Semaphore Register (IMA_SEM)"]
pub mod ima_sem;
#[doc = "SHCFG (rw) register accessor: Shared Memory Configuration Register (SHCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`shcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shcfg`]
module"]
#[doc(alias = "SHCFG")]
pub type Shcfg = crate::Reg<shcfg::ShcfgSpec>;
#[doc = "Shared Memory Configuration Register (SHCFG)"]
pub mod shcfg;
#[doc = "WIN1_WR_PROT (rw) register accessor: Shared Access Window 1 Write Protect Register (WIN1_WR_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`win1_wr_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win1_wr_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_wr_prot`]
module"]
#[doc(alias = "WIN1_WR_PROT")]
pub type Win1WrProt = crate::Reg<win1_wr_prot::Win1WrProtSpec>;
#[doc = "Shared Access Window 1 Write Protect Register (WIN1_WR_PROT)"]
pub mod win1_wr_prot;
#[doc = "WIN1_RD_PROT (rw) register accessor: Shared Access Window 1 Read Protect Register (WIN1_RD_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`win1_rd_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win1_rd_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win1_rd_prot`]
module"]
#[doc(alias = "WIN1_RD_PROT")]
pub type Win1RdProt = crate::Reg<win1_rd_prot::Win1RdProtSpec>;
#[doc = "Shared Access Window 1 Read Protect Register (WIN1_RD_PROT)"]
pub mod win1_rd_prot;
#[doc = "WIN2_WR_PROT (rw) register accessor: Shared Access Window 2 Write Protect Register (WIN2_WR_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`win2_wr_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win2_wr_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_wr_prot`]
module"]
#[doc(alias = "WIN2_WR_PROT")]
pub type Win2WrProt = crate::Reg<win2_wr_prot::Win2WrProtSpec>;
#[doc = "Shared Access Window 2 Write Protect Register (WIN2_WR_PROT)"]
pub mod win2_wr_prot;
#[doc = "WIN2_RD_PROT (rw) register accessor: Shared Access Window 2 Read Protect Register (WIN2_RD_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`win2_rd_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win2_rd_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win2_rd_prot`]
module"]
#[doc(alias = "WIN2_RD_PROT")]
pub type Win2RdProt = crate::Reg<win2_rd_prot::Win2RdProtSpec>;
#[doc = "Shared Access Window 2 Read Protect Register (WIN2_RD_PROT)"]
pub mod win2_rd_prot;
#[doc = "IMA_WR_PROT (rw) register accessor: Indirect Memory Access Write Protect Register (IMA_WR_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`ima_wr_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ima_wr_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ima_wr_prot`]
module"]
#[doc(alias = "IMA_WR_PROT")]
pub type ImaWrProt = crate::Reg<ima_wr_prot::ImaWrProtSpec>;
#[doc = "Indirect Memory Access Write Protect Register (IMA_WR_PROT)"]
pub mod ima_wr_prot;
#[doc = "IMA_RD_PROT (rw) register accessor: Indirect Memory Access Read Protect Register (IMA_RD_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`ima_rd_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ima_rd_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ima_rd_prot`]
module"]
#[doc(alias = "IMA_RD_PROT")]
pub type ImaRdProt = crate::Reg<ima_rd_prot::ImaRdProtSpec>;
#[doc = "Indirect Memory Access Read Protect Register (IMA_RD_PROT)"]
pub mod ima_rd_prot;
#[doc = "WIN_BASE1 (rw) register accessor: Shared Access Window 1 Base Register (WIN_BASE1)\n\nYou can [`read`](crate::Reg::read) this register and get [`win_base1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win_base1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win_base1`]
module"]
#[doc(alias = "WIN_BASE1")]
pub type WinBase1 = crate::Reg<win_base1::WinBase1Spec>;
#[doc = "Shared Access Window 1 Base Register (WIN_BASE1)"]
pub mod win_base1;
#[doc = "WIN_BASE2 (rw) register accessor: Shared Access Window 2 Base Register (WIN_BASE2)\n\nYou can [`read`](crate::Reg::read) this register and get [`win_base2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win_base2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win_base2`]
module"]
#[doc(alias = "WIN_BASE2")]
pub type WinBase2 = crate::Reg<win_base2::WinBase2Spec>;
#[doc = "Shared Access Window 2 Base Register (WIN_BASE2)"]
pub mod win_base2;
#[doc = "IMA_BASE (rw) register accessor: Indirect Memory Access Window Base Register (IMA_BASE)\n\nYou can [`read`](crate::Reg::read) this register and get [`ima_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ima_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ima_base`]
module"]
#[doc(alias = "IMA_BASE")]
pub type ImaBase = crate::Reg<ima_base::ImaBaseSpec>;
#[doc = "Indirect Memory Access Window Base Register (IMA_BASE)"]
pub mod ima_base;
#[doc = "RST_CFG (rw) register accessor: Reset Configuration Register (RST_CFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_cfg`]
module"]
#[doc(alias = "RST_CFG")]
pub type RstCfg = crate::Reg<rst_cfg::RstCfgSpec>;
#[doc = "Reset Configuration Register (RST_CFG)"]
pub mod rst_cfg;
#[doc = "DP80BUF (rw) register accessor: Debug Port 80 Buffered Data Register (DP80BUF)\n\nYou can [`read`](crate::Reg::read) this register and get [`dp80buf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dp80buf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp80buf`]
module"]
#[doc(alias = "DP80BUF")]
pub type Dp80buf = crate::Reg<dp80buf::Dp80bufSpec>;
#[doc = "Debug Port 80 Buffered Data Register (DP80BUF)"]
pub mod dp80buf;
#[doc = "DP80STS (rw) register accessor: Debug Port 80 Status Register (DP80STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`dp80sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dp80sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp80sts`]
module"]
#[doc(alias = "DP80STS")]
pub type Dp80sts = crate::Reg<dp80sts::Dp80stsSpec>;
#[doc = "Debug Port 80 Status Register (DP80STS)"]
pub mod dp80sts;
#[doc = "DP80CTL (rw) register accessor: Debug Port 80 Control Register (DP80CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dp80ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dp80ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dp80ctl`]
module"]
#[doc(alias = "DP80CTL")]
pub type Dp80ctl = crate::Reg<dp80ctl::Dp80ctlSpec>;
#[doc = "Debug Port 80 Control Register (DP80CTL)"]
pub mod dp80ctl;
#[doc = "HOFS_STS (rw) register accessor: Host_Offset in Windows 1,2 Status Register (HOFS_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`hofs_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hofs_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hofs_sts`]
module"]
#[doc(alias = "HOFS_STS")]
pub type HofsSts = crate::Reg<hofs_sts::HofsStsSpec>;
#[doc = "Host_Offset in Windows 1,2 Status Register (HOFS_STS)"]
pub mod hofs_sts;
#[doc = "HOFS_CTL (rw) register accessor: Host_Offset in Windows 1,2 Control Register (HOFS_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hofs_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hofs_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hofs_ctl`]
module"]
#[doc(alias = "HOFS_CTL")]
pub type HofsCtl = crate::Reg<hofs_ctl::HofsCtlSpec>;
#[doc = "Host_Offset in Windows 1,2 Control Register (HOFS_CTL)"]
pub mod hofs_ctl;
#[doc = "COFS2 (rw) register accessor: Core_Offset in Window 2 Address Register (COFS2)\n\nYou can [`read`](crate::Reg::read) this register and get [`cofs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cofs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cofs2`]
module"]
#[doc(alias = "COFS2")]
pub type Cofs2 = crate::Reg<cofs2::Cofs2Spec>;
#[doc = "Core_Offset in Window 2 Address Register (COFS2)"]
pub mod cofs2;
#[doc = "COFS1 (rw) register accessor: Core_Offset in Window 1 Address Register (COFS1)\n\nYou can [`read`](crate::Reg::read) this register and get [`cofs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cofs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cofs1`]
module"]
#[doc(alias = "COFS1")]
pub type Cofs1 = crate::Reg<cofs1::Cofs1Spec>;
#[doc = "Core_Offset in Window 1 Address Register (COFS1)"]
pub mod cofs1;
