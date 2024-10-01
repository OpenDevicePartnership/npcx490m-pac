#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    hipmn_st: HipmnSt,
    _reserved1: [u8; 0x01],
    hipmn_do: HipmnDo,
    _reserved2: [u8; 0x01],
    hipmn_di: HipmnDi,
    shipmn_di: ShipmnDi,
    hipmn_doc: HipmnDoc,
    _reserved5: [u8; 0x01],
    hipmn_dom: HipmnDom,
    _reserved6: [u8; 0x01],
    hipmn_dic: HipmnDic,
    _reserved7: [u8; 0x01],
    hipmn_ctl: HipmnCtl,
    hipmn_ctl2: HipmnCtl2,
    hipmn_ic: HipmnIc,
    _reserved10: [u8; 0x01],
    hipmn_ie: HipmnIe,
}
impl RegisterBlock {
    #[doc = "0x00 - Host Interface PM n Status Register (HIPMnST)"]
    #[inline(always)]
    pub const fn hipmn_st(&self) -> &HipmnSt {
        &self.hipmn_st
    }
    #[doc = "0x02 - Host Interface PM n Data Out Buffer Register (HIPMnDO)"]
    #[inline(always)]
    pub const fn hipmn_do(&self) -> &HipmnDo {
        &self.hipmn_do
    }
    #[doc = "0x04 - Host Interface PM n Data In Buffer Register (HIPMnDI)"]
    #[inline(always)]
    pub const fn hipmn_di(&self) -> &HipmnDi {
        &self.hipmn_di
    }
    #[doc = "0x05 - Host Interface PM n Shadow Data In Buffer Register (SHIPMnDI)"]
    #[inline(always)]
    pub const fn shipmn_di(&self) -> &ShipmnDi {
        &self.shipmn_di
    }
    #[doc = "0x06 - Host Interface PM n Data Out Buffer with SCI Register (HIPMnDOC)"]
    #[inline(always)]
    pub const fn hipmn_doc(&self) -> &HipmnDoc {
        &self.hipmn_doc
    }
    #[doc = "0x08 - Host Interface PM n Data Out Buffer with SMI Register (HIPMnDOM)"]
    #[inline(always)]
    pub const fn hipmn_dom(&self) -> &HipmnDom {
        &self.hipmn_dom
    }
    #[doc = "0x0a - Host Interface PM n Data In Buffer with SCI Register (HIPMnDIC)"]
    #[inline(always)]
    pub const fn hipmn_dic(&self) -> &HipmnDic {
        &self.hipmn_dic
    }
    #[doc = "0x0c - Host Interface PM n Control Register (HIPMnCTL)"]
    #[inline(always)]
    pub const fn hipmn_ctl(&self) -> &HipmnCtl {
        &self.hipmn_ctl
    }
    #[doc = "0x0d - Host Interface PM n Control 2 Register (HIPMnCTL2)"]
    #[inline(always)]
    pub const fn hipmn_ctl2(&self) -> &HipmnCtl2 {
        &self.hipmn_ctl2
    }
    #[doc = "0x0e - Host Interface PM n Interrupt Control Register (HIPMnIC)"]
    #[inline(always)]
    pub const fn hipmn_ic(&self) -> &HipmnIc {
        &self.hipmn_ic
    }
    #[doc = "0x10 - Host Interface PM n Interrupt Enable Register (HIPMnIE)"]
    #[inline(always)]
    pub const fn hipmn_ie(&self) -> &HipmnIe {
        &self.hipmn_ie
    }
}
#[doc = "HIPMnST (rw) register accessor: Host Interface PM n Status Register (HIPMnST)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_st::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_st::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hipmn_st`]
module"]
#[doc(alias = "HIPMnST")]
pub type HipmnSt = crate::Reg<hipmn_st::HipmnStSpec>;
#[doc = "Host Interface PM n Status Register (HIPMnST)"]
pub mod hipmn_st;
#[doc = "HIPMnDO (rw) register accessor: Host Interface PM n Data Out Buffer Register (HIPMnDO)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_do::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_do::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hipmn_do`]
module"]
#[doc(alias = "HIPMnDO")]
pub type HipmnDo = crate::Reg<hipmn_do::HipmnDoSpec>;
#[doc = "Host Interface PM n Data Out Buffer Register (HIPMnDO)"]
pub mod hipmn_do;
#[doc = "HIPMnDI (rw) register accessor: Host Interface PM n Data In Buffer Register (HIPMnDI)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_di::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_di::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hipmn_di`]
module"]
#[doc(alias = "HIPMnDI")]
pub type HipmnDi = crate::Reg<hipmn_di::HipmnDiSpec>;
#[doc = "Host Interface PM n Data In Buffer Register (HIPMnDI)"]
pub mod hipmn_di;
#[doc = "SHIPMnDI (rw) register accessor: Host Interface PM n Shadow Data In Buffer Register (SHIPMnDI)\n\nYou can [`read`](crate::Reg::read) this register and get [`shipmn_di::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shipmn_di::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shipmn_di`]
module"]
#[doc(alias = "SHIPMnDI")]
pub type ShipmnDi = crate::Reg<shipmn_di::ShipmnDiSpec>;
#[doc = "Host Interface PM n Shadow Data In Buffer Register (SHIPMnDI)"]
pub mod shipmn_di;
#[doc = "HIPMnDOC (rw) register accessor: Host Interface PM n Data Out Buffer with SCI Register (HIPMnDOC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_doc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_doc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hipmn_doc`]
module"]
#[doc(alias = "HIPMnDOC")]
pub type HipmnDoc = crate::Reg<hipmn_doc::HipmnDocSpec>;
#[doc = "Host Interface PM n Data Out Buffer with SCI Register (HIPMnDOC)"]
pub mod hipmn_doc;
#[doc = "HIPMnDOM (rw) register accessor: Host Interface PM n Data Out Buffer with SMI Register (HIPMnDOM)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_dom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_dom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hipmn_dom`]
module"]
#[doc(alias = "HIPMnDOM")]
pub type HipmnDom = crate::Reg<hipmn_dom::HipmnDomSpec>;
#[doc = "Host Interface PM n Data Out Buffer with SMI Register (HIPMnDOM)"]
pub mod hipmn_dom;
#[doc = "HIPMnDIC (rw) register accessor: Host Interface PM n Data In Buffer with SCI Register (HIPMnDIC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_dic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_dic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hipmn_dic`]
module"]
#[doc(alias = "HIPMnDIC")]
pub type HipmnDic = crate::Reg<hipmn_dic::HipmnDicSpec>;
#[doc = "Host Interface PM n Data In Buffer with SCI Register (HIPMnDIC)"]
pub mod hipmn_dic;
#[doc = "HIPMnCTL (rw) register accessor: Host Interface PM n Control Register (HIPMnCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hipmn_ctl`]
module"]
#[doc(alias = "HIPMnCTL")]
pub type HipmnCtl = crate::Reg<hipmn_ctl::HipmnCtlSpec>;
#[doc = "Host Interface PM n Control Register (HIPMnCTL)"]
pub mod hipmn_ctl;
#[doc = "HIPMnCTL2 (rw) register accessor: Host Interface PM n Control 2 Register (HIPMnCTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_ctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_ctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hipmn_ctl2`]
module"]
#[doc(alias = "HIPMnCTL2")]
pub type HipmnCtl2 = crate::Reg<hipmn_ctl2::HipmnCtl2Spec>;
#[doc = "Host Interface PM n Control 2 Register (HIPMnCTL2)"]
pub mod hipmn_ctl2;
#[doc = "HIPMnIC (rw) register accessor: Host Interface PM n Interrupt Control Register (HIPMnIC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_ic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_ic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hipmn_ic`]
module"]
#[doc(alias = "HIPMnIC")]
pub type HipmnIc = crate::Reg<hipmn_ic::HipmnIcSpec>;
#[doc = "Host Interface PM n Interrupt Control Register (HIPMnIC)"]
pub mod hipmn_ic;
#[doc = "HIPMnIE (rw) register accessor: Host Interface PM n Interrupt Enable Register (HIPMnIE)\n\nYou can [`read`](crate::Reg::read) this register and get [`hipmn_ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hipmn_ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hipmn_ie`]
module"]
#[doc(alias = "HIPMnIE")]
pub type HipmnIe = crate::Reg<hipmn_ie::HipmnIeSpec>;
#[doc = "Host Interface PM n Interrupt Enable Register (HIPMnIE)"]
pub mod hipmn_ie;
