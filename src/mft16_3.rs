#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    tn_cnt1: TnCnt1,
    tn_cra: TnCra,
    tn_crb: TnCrb,
    tn_cnt2: TnCnt2,
    tn_prsc: TnPrsc,
    _reserved5: [u8; 0x01],
    tn_ckc: TnCkc,
    _reserved6: [u8; 0x01],
    tn_mctrl: TnMctrl,
    _reserved7: [u8; 0x01],
    tn_ectrl: TnEctrl,
    _reserved8: [u8; 0x01],
    tn_eclr: TnEclr,
    _reserved9: [u8; 0x01],
    tn_ien: TnIen,
    _reserved10: [u8; 0x01],
    tn_cpa: TnCpa,
    tn_cpb: TnCpb,
    tn_cpcfg: TnCpcfg,
    _reserved13: [u8; 0x01],
    tn_wuen: TnWuen,
    _reserved14: [u8; 0x01],
    tn_cfg: TnCfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer/Counter 1 Register (TnCNT1)"]
    #[inline(always)]
    pub const fn tn_cnt1(&self) -> &TnCnt1 {
        &self.tn_cnt1
    }
    #[doc = "0x02 - Reload/Capture A Register (TnCRA)"]
    #[inline(always)]
    pub const fn tn_cra(&self) -> &TnCra {
        &self.tn_cra
    }
    #[doc = "0x04 - Reload/Capture B Register (TnCRB)"]
    #[inline(always)]
    pub const fn tn_crb(&self) -> &TnCrb {
        &self.tn_crb
    }
    #[doc = "0x06 - Timer/Counter 2 Register (TnCNT2)"]
    #[inline(always)]
    pub const fn tn_cnt2(&self) -> &TnCnt2 {
        &self.tn_cnt2
    }
    #[doc = "0x08 - Clock Prescaler Register (TnPRSC)"]
    #[inline(always)]
    pub const fn tn_prsc(&self) -> &TnPrsc {
        &self.tn_prsc
    }
    #[doc = "0x0a - Clock Unit Control Register (TnCKC)"]
    #[inline(always)]
    pub const fn tn_ckc(&self) -> &TnCkc {
        &self.tn_ckc
    }
    #[doc = "0x0c - Timer Mode Control Register (TnMCTRL)"]
    #[inline(always)]
    pub const fn tn_mctrl(&self) -> &TnMctrl {
        &self.tn_mctrl
    }
    #[doc = "0x0e - Timer Event Control Register (TnECTRL)"]
    #[inline(always)]
    pub const fn tn_ectrl(&self) -> &TnEctrl {
        &self.tn_ectrl
    }
    #[doc = "0x10 - Timer Event Clear Register (TnECLR)"]
    #[inline(always)]
    pub const fn tn_eclr(&self) -> &TnEclr {
        &self.tn_eclr
    }
    #[doc = "0x12 - Timer Interrupt Enable Register (TnIEN)"]
    #[inline(always)]
    pub const fn tn_ien(&self) -> &TnIen {
        &self.tn_ien
    }
    #[doc = "0x14 - Compare A Register (TnCPA)"]
    #[inline(always)]
    pub const fn tn_cpa(&self) -> &TnCpa {
        &self.tn_cpa
    }
    #[doc = "0x16 - Compare B Register (TnCPB)"]
    #[inline(always)]
    pub const fn tn_cpb(&self) -> &TnCpb {
        &self.tn_cpb
    }
    #[doc = "0x18 - Compare Configuration Register (TnCPCFG)"]
    #[inline(always)]
    pub const fn tn_cpcfg(&self) -> &TnCpcfg {
        &self.tn_cpcfg
    }
    #[doc = "0x1a - Timer Wake-Up Enable Register (TnWUEN)"]
    #[inline(always)]
    pub const fn tn_wuen(&self) -> &TnWuen {
        &self.tn_wuen
    }
    #[doc = "0x1c - Timer Configuration Register (TnCFG)"]
    #[inline(always)]
    pub const fn tn_cfg(&self) -> &TnCfg {
        &self.tn_cfg
    }
}
#[doc = "TnCNT1 (rw) register accessor: Timer/Counter 1 Register (TnCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_cnt1`]
module"]
#[doc(alias = "TnCNT1")]
pub type TnCnt1 = crate::Reg<tn_cnt1::TnCnt1Spec>;
#[doc = "Timer/Counter 1 Register (TnCNT1)"]
pub mod tn_cnt1;
#[doc = "TnCRA (rw) register accessor: Reload/Capture A Register (TnCRA)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_cra`]
module"]
#[doc(alias = "TnCRA")]
pub type TnCra = crate::Reg<tn_cra::TnCraSpec>;
#[doc = "Reload/Capture A Register (TnCRA)"]
pub mod tn_cra;
#[doc = "TnCRB (rw) register accessor: Reload/Capture B Register (TnCRB)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_crb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_crb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_crb`]
module"]
#[doc(alias = "TnCRB")]
pub type TnCrb = crate::Reg<tn_crb::TnCrbSpec>;
#[doc = "Reload/Capture B Register (TnCRB)"]
pub mod tn_crb;
#[doc = "TnCNT2 (rw) register accessor: Timer/Counter 2 Register (TnCNT2)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cnt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cnt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_cnt2`]
module"]
#[doc(alias = "TnCNT2")]
pub type TnCnt2 = crate::Reg<tn_cnt2::TnCnt2Spec>;
#[doc = "Timer/Counter 2 Register (TnCNT2)"]
pub mod tn_cnt2;
#[doc = "TnPRSC (rw) register accessor: Clock Prescaler Register (TnPRSC)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_prsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_prsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_prsc`]
module"]
#[doc(alias = "TnPRSC")]
pub type TnPrsc = crate::Reg<tn_prsc::TnPrscSpec>;
#[doc = "Clock Prescaler Register (TnPRSC)"]
pub mod tn_prsc;
#[doc = "TnCKC (rw) register accessor: Clock Unit Control Register (TnCKC)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_ckc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_ckc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_ckc`]
module"]
#[doc(alias = "TnCKC")]
pub type TnCkc = crate::Reg<tn_ckc::TnCkcSpec>;
#[doc = "Clock Unit Control Register (TnCKC)"]
pub mod tn_ckc;
#[doc = "TnMCTRL (rw) register accessor: Timer Mode Control Register (TnMCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_mctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_mctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_mctrl`]
module"]
#[doc(alias = "TnMCTRL")]
pub type TnMctrl = crate::Reg<tn_mctrl::TnMctrlSpec>;
#[doc = "Timer Mode Control Register (TnMCTRL)"]
pub mod tn_mctrl;
#[doc = "TnECTRL (rw) register accessor: Timer Event Control Register (TnECTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_ectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_ectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_ectrl`]
module"]
#[doc(alias = "TnECTRL")]
pub type TnEctrl = crate::Reg<tn_ectrl::TnEctrlSpec>;
#[doc = "Timer Event Control Register (TnECTRL)"]
pub mod tn_ectrl;
#[doc = "TnECLR (rw) register accessor: Timer Event Clear Register (TnECLR)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_eclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_eclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_eclr`]
module"]
#[doc(alias = "TnECLR")]
pub type TnEclr = crate::Reg<tn_eclr::TnEclrSpec>;
#[doc = "Timer Event Clear Register (TnECLR)"]
pub mod tn_eclr;
#[doc = "TnIEN (rw) register accessor: Timer Interrupt Enable Register (TnIEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_ien`]
module"]
#[doc(alias = "TnIEN")]
pub type TnIen = crate::Reg<tn_ien::TnIenSpec>;
#[doc = "Timer Interrupt Enable Register (TnIEN)"]
pub mod tn_ien;
#[doc = "TnCPA (rw) register accessor: Compare A Register (TnCPA)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cpa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cpa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_cpa`]
module"]
#[doc(alias = "TnCPA")]
pub type TnCpa = crate::Reg<tn_cpa::TnCpaSpec>;
#[doc = "Compare A Register (TnCPA)"]
pub mod tn_cpa;
#[doc = "TnCPB (rw) register accessor: Compare B Register (TnCPB)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cpb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cpb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_cpb`]
module"]
#[doc(alias = "TnCPB")]
pub type TnCpb = crate::Reg<tn_cpb::TnCpbSpec>;
#[doc = "Compare B Register (TnCPB)"]
pub mod tn_cpb;
#[doc = "TnCPCFG (rw) register accessor: Compare Configuration Register (TnCPCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cpcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cpcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_cpcfg`]
module"]
#[doc(alias = "TnCPCFG")]
pub type TnCpcfg = crate::Reg<tn_cpcfg::TnCpcfgSpec>;
#[doc = "Compare Configuration Register (TnCPCFG)"]
pub mod tn_cpcfg;
#[doc = "TnWUEN (rw) register accessor: Timer Wake-Up Enable Register (TnWUEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_wuen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_wuen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_wuen`]
module"]
#[doc(alias = "TnWUEN")]
pub type TnWuen = crate::Reg<tn_wuen::TnWuenSpec>;
#[doc = "Timer Wake-Up Enable Register (TnWUEN)"]
pub mod tn_wuen;
#[doc = "TnCFG (rw) register accessor: Timer Configuration Register (TnCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tn_cfg`]
module"]
#[doc(alias = "TnCFG")]
pub type TnCfg = crate::Reg<tn_cfg::TnCfgSpec>;
#[doc = "Timer Configuration Register (TnCFG)"]
pub mod tn_cfg;
