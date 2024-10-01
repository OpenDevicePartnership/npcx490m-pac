#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    smce_sts: SmceSts,
    smce_ctl: SmceCtl,
    _reserved2: [u8; 0x05],
    wine_size: WineSize,
    shaw34_sem0: Shaw34Sem0,
    shaw34_sem1: Shaw34Sem1,
    _reserved5: [u8; 0x06],
    win3_wr_prot: Win3WrProt,
    win3_rd_prot: Win3RdProt,
    win4_wr_prot: Win4WrProt,
    win4_rd_prot: Win4RdProt,
    _reserved9: [u8; 0x0c],
    win_base3: WinBase3,
    win_base4: WinBase4,
    _reserved11: [u8; 0x20],
    hofse_sts: HofseSts,
    hofse_ctl: HofseCtl,
    cofs4: Cofs4,
    cofs3: Cofs3,
}
impl RegisterBlock {
    #[doc = "0x00 - Extended Shared Memory Core Status Register (SMCE_STS)"]
    #[inline(always)]
    pub const fn smce_sts(&self) -> &SmceSts {
        &self.smce_sts
    }
    #[doc = "0x01 - Extended Shared Memory Core Control Register (SMCE_CTL)"]
    #[inline(always)]
    pub const fn smce_ctl(&self) -> &SmceCtl {
        &self.smce_ctl
    }
    #[doc = "0x07 - Extended Shared Access Windows Size Register (WINE_SIZE)"]
    #[inline(always)]
    pub const fn wine_size(&self) -> &WineSize {
        &self.wine_size
    }
    #[doc = "0x08 - Shared Access Window 3-4, Semaphore Register (SHAW3-4_SEM)"]
    #[inline(always)]
    pub const fn shaw34_sem0(&self) -> &Shaw34Sem0 {
        &self.shaw34_sem0
    }
    #[doc = "0x09 - Shared Access Window 3-4, Semaphore Register (SHAW3-4_SEM)"]
    #[inline(always)]
    pub const fn shaw34_sem1(&self) -> &Shaw34Sem1 {
        &self.shaw34_sem1
    }
    #[doc = "0x10 - Shared Access Window 3 Write Protect Register (WIN3_WR_PROT)"]
    #[inline(always)]
    pub const fn win3_wr_prot(&self) -> &Win3WrProt {
        &self.win3_wr_prot
    }
    #[doc = "0x11 - Shared Access Window 3 Read Protect Register (WIN3_RD_PROT)"]
    #[inline(always)]
    pub const fn win3_rd_prot(&self) -> &Win3RdProt {
        &self.win3_rd_prot
    }
    #[doc = "0x12 - Shared Access Window 4 Write Protect Register (WIN4_WR_PROT)"]
    #[inline(always)]
    pub const fn win4_wr_prot(&self) -> &Win4WrProt {
        &self.win4_wr_prot
    }
    #[doc = "0x13 - Shared Access Window 4 Read Protect Register (WIN4_RD_PROT)"]
    #[inline(always)]
    pub const fn win4_rd_prot(&self) -> &Win4RdProt {
        &self.win4_rd_prot
    }
    #[doc = "0x20 - Shared Access Window 3 Base Register (WIN_BASE3)"]
    #[inline(always)]
    pub const fn win_base3(&self) -> &WinBase3 {
        &self.win_base3
    }
    #[doc = "0x24 - Shared Access Window 4 Base Register (WIN_BASE4)"]
    #[inline(always)]
    pub const fn win_base4(&self) -> &WinBase4 {
        &self.win_base4
    }
    #[doc = "0x48 - Host_Offset in Windows 3,4 Status Register (HOFSE_STS)"]
    #[inline(always)]
    pub const fn hofse_sts(&self) -> &HofseSts {
        &self.hofse_sts
    }
    #[doc = "0x49 - Host_Offset in Windows 3,4 Control Register (HOFSE_CTL)"]
    #[inline(always)]
    pub const fn hofse_ctl(&self) -> &HofseCtl {
        &self.hofse_ctl
    }
    #[doc = "0x4a - Core_Offset in Window 4 Address (COFS4)"]
    #[inline(always)]
    pub const fn cofs4(&self) -> &Cofs4 {
        &self.cofs4
    }
    #[doc = "0x4c - Core_Offset in Window 3 Address (COFS3)"]
    #[inline(always)]
    pub const fn cofs3(&self) -> &Cofs3 {
        &self.cofs3
    }
}
#[doc = "SMCE_STS (rw) register accessor: Extended Shared Memory Core Status Register (SMCE_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`smce_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smce_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smce_sts`]
module"]
#[doc(alias = "SMCE_STS")]
pub type SmceSts = crate::Reg<smce_sts::SmceStsSpec>;
#[doc = "Extended Shared Memory Core Status Register (SMCE_STS)"]
pub mod smce_sts;
#[doc = "SMCE_CTL (rw) register accessor: Extended Shared Memory Core Control Register (SMCE_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`smce_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smce_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smce_ctl`]
module"]
#[doc(alias = "SMCE_CTL")]
pub type SmceCtl = crate::Reg<smce_ctl::SmceCtlSpec>;
#[doc = "Extended Shared Memory Core Control Register (SMCE_CTL)"]
pub mod smce_ctl;
#[doc = "WINE_SIZE (rw) register accessor: Extended Shared Access Windows Size Register (WINE_SIZE)\n\nYou can [`read`](crate::Reg::read) this register and get [`wine_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wine_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wine_size`]
module"]
#[doc(alias = "WINE_SIZE")]
pub type WineSize = crate::Reg<wine_size::WineSizeSpec>;
#[doc = "Extended Shared Access Windows Size Register (WINE_SIZE)"]
pub mod wine_size;
#[doc = "SHAW3-4_SEM0 (rw) register accessor: Shared Access Window 3-4, Semaphore Register (SHAW3-4_SEM)\n\nYou can [`read`](crate::Reg::read) this register and get [`shaw34_sem0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shaw34_sem0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shaw34_sem0`]
module"]
#[doc(alias = "SHAW3-4_SEM0")]
pub type Shaw34Sem0 = crate::Reg<shaw34_sem0::Shaw34Sem0Spec>;
#[doc = "Shared Access Window 3-4, Semaphore Register (SHAW3-4_SEM)"]
pub mod shaw34_sem0;
#[doc = "SHAW3-4_SEM1 (rw) register accessor: Shared Access Window 3-4, Semaphore Register (SHAW3-4_SEM)\n\nYou can [`read`](crate::Reg::read) this register and get [`shaw34_sem1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shaw34_sem1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shaw34_sem1`]
module"]
#[doc(alias = "SHAW3-4_SEM1")]
pub type Shaw34Sem1 = crate::Reg<shaw34_sem1::Shaw34Sem1Spec>;
#[doc = "Shared Access Window 3-4, Semaphore Register (SHAW3-4_SEM)"]
pub mod shaw34_sem1;
#[doc = "WIN3_WR_PROT (rw) register accessor: Shared Access Window 3 Write Protect Register (WIN3_WR_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`win3_wr_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win3_wr_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_wr_prot`]
module"]
#[doc(alias = "WIN3_WR_PROT")]
pub type Win3WrProt = crate::Reg<win3_wr_prot::Win3WrProtSpec>;
#[doc = "Shared Access Window 3 Write Protect Register (WIN3_WR_PROT)"]
pub mod win3_wr_prot;
#[doc = "WIN3_RD_PROT (rw) register accessor: Shared Access Window 3 Read Protect Register (WIN3_RD_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`win3_rd_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win3_rd_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win3_rd_prot`]
module"]
#[doc(alias = "WIN3_RD_PROT")]
pub type Win3RdProt = crate::Reg<win3_rd_prot::Win3RdProtSpec>;
#[doc = "Shared Access Window 3 Read Protect Register (WIN3_RD_PROT)"]
pub mod win3_rd_prot;
#[doc = "WIN4_WR_PROT (rw) register accessor: Shared Access Window 4 Write Protect Register (WIN4_WR_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`win4_wr_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win4_wr_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win4_wr_prot`]
module"]
#[doc(alias = "WIN4_WR_PROT")]
pub type Win4WrProt = crate::Reg<win4_wr_prot::Win4WrProtSpec>;
#[doc = "Shared Access Window 4 Write Protect Register (WIN4_WR_PROT)"]
pub mod win4_wr_prot;
#[doc = "WIN4_RD_PROT (rw) register accessor: Shared Access Window 4 Read Protect Register (WIN4_RD_PROT)\n\nYou can [`read`](crate::Reg::read) this register and get [`win4_rd_prot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win4_rd_prot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win4_rd_prot`]
module"]
#[doc(alias = "WIN4_RD_PROT")]
pub type Win4RdProt = crate::Reg<win4_rd_prot::Win4RdProtSpec>;
#[doc = "Shared Access Window 4 Read Protect Register (WIN4_RD_PROT)"]
pub mod win4_rd_prot;
#[doc = "WIN_BASE3 (rw) register accessor: Shared Access Window 3 Base Register (WIN_BASE3)\n\nYou can [`read`](crate::Reg::read) this register and get [`win_base3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win_base3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win_base3`]
module"]
#[doc(alias = "WIN_BASE3")]
pub type WinBase3 = crate::Reg<win_base3::WinBase3Spec>;
#[doc = "Shared Access Window 3 Base Register (WIN_BASE3)"]
pub mod win_base3;
#[doc = "WIN_BASE4 (rw) register accessor: Shared Access Window 4 Base Register (WIN_BASE4)\n\nYou can [`read`](crate::Reg::read) this register and get [`win_base4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win_base4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win_base4`]
module"]
#[doc(alias = "WIN_BASE4")]
pub type WinBase4 = crate::Reg<win_base4::WinBase4Spec>;
#[doc = "Shared Access Window 4 Base Register (WIN_BASE4)"]
pub mod win_base4;
#[doc = "HOFSE_STS (rw) register accessor: Host_Offset in Windows 3,4 Status Register (HOFSE_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`hofse_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hofse_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hofse_sts`]
module"]
#[doc(alias = "HOFSE_STS")]
pub type HofseSts = crate::Reg<hofse_sts::HofseStsSpec>;
#[doc = "Host_Offset in Windows 3,4 Status Register (HOFSE_STS)"]
pub mod hofse_sts;
#[doc = "HOFSE_CTL (rw) register accessor: Host_Offset in Windows 3,4 Control Register (HOFSE_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hofse_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hofse_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hofse_ctl`]
module"]
#[doc(alias = "HOFSE_CTL")]
pub type HofseCtl = crate::Reg<hofse_ctl::HofseCtlSpec>;
#[doc = "Host_Offset in Windows 3,4 Control Register (HOFSE_CTL)"]
pub mod hofse_ctl;
#[doc = "COFS4 (rw) register accessor: Core_Offset in Window 4 Address (COFS4)\n\nYou can [`read`](crate::Reg::read) this register and get [`cofs4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cofs4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cofs4`]
module"]
#[doc(alias = "COFS4")]
pub type Cofs4 = crate::Reg<cofs4::Cofs4Spec>;
#[doc = "Core_Offset in Window 4 Address (COFS4)"]
pub mod cofs4;
#[doc = "COFS3 (rw) register accessor: Core_Offset in Window 3 Address (COFS3)\n\nYou can [`read`](crate::Reg::read) this register and get [`cofs3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cofs3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cofs3`]
module"]
#[doc(alias = "COFS3")]
pub type Cofs3 = crate::Reg<cofs3::Cofs3Spec>;
#[doc = "Core_Offset in Window 3 Address (COFS3)"]
pub mod cofs3;
