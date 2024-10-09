#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Channel%s"]
pub struct Channel {
    gdma_ctl: GdmaCtl,
    gdma_srcb: GdmaSrcb,
    gdma_dstb: GdmaDstb,
    gdma_tcnt: GdmaTcnt,
    gdma_csrc: GdmaCsrc,
    gdma_cdst: GdmaCdst,
    gdma_ctcnt: GdmaCtcnt,
    gdma_req_sl: GdmaReqSl,
}
impl Channel {
    #[doc = "0x00 - Channel 0/1 Control Register"]
    #[inline(always)]
    pub const fn gdma_ctl(&self) -> &GdmaCtl {
        &self.gdma_ctl
    }
    #[doc = "0x04 - Channel 0/1 Source Base Address Register"]
    #[inline(always)]
    pub const fn gdma_srcb(&self) -> &GdmaSrcb {
        &self.gdma_srcb
    }
    #[doc = "0x08 - Channel 0/1 Destination Base Address Register"]
    #[inline(always)]
    pub const fn gdma_dstb(&self) -> &GdmaDstb {
        &self.gdma_dstb
    }
    #[doc = "0x0c - Channel 0/1 Transfer Count Register"]
    #[inline(always)]
    pub const fn gdma_tcnt(&self) -> &GdmaTcnt {
        &self.gdma_tcnt
    }
    #[doc = "0x10 - Channel 0/1 Current Source Register"]
    #[inline(always)]
    pub const fn gdma_csrc(&self) -> &GdmaCsrc {
        &self.gdma_csrc
    }
    #[doc = "0x14 - Channel 0/1 Current Destination Register"]
    #[inline(always)]
    pub const fn gdma_cdst(&self) -> &GdmaCdst {
        &self.gdma_cdst
    }
    #[doc = "0x18 - Channel 0/1 Current Transfer Count Register"]
    #[inline(always)]
    pub const fn gdma_ctcnt(&self) -> &GdmaCtcnt {
        &self.gdma_ctcnt
    }
    #[doc = "0x1c - Channel 0/1 GDMA Request Select Register"]
    #[inline(always)]
    pub const fn gdma_req_sl(&self) -> &GdmaReqSl {
        &self.gdma_req_sl
    }
}
#[doc = "GDMA_CTL (rw) register accessor: Channel 0/1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_ctl`]
module"]
#[doc(alias = "GDMA_CTL")]
pub type GdmaCtl = crate::Reg<gdma_ctl::GdmaCtlSpec>;
#[doc = "Channel 0/1 Control Register"]
pub mod gdma_ctl;
#[doc = "GDMA_SRCB (rw) register accessor: Channel 0/1 Source Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_srcb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_srcb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_srcb`]
module"]
#[doc(alias = "GDMA_SRCB")]
pub type GdmaSrcb = crate::Reg<gdma_srcb::GdmaSrcbSpec>;
#[doc = "Channel 0/1 Source Base Address Register"]
pub mod gdma_srcb;
#[doc = "GDMA_DSTB (rw) register accessor: Channel 0/1 Destination Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_dstb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_dstb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_dstb`]
module"]
#[doc(alias = "GDMA_DSTB")]
pub type GdmaDstb = crate::Reg<gdma_dstb::GdmaDstbSpec>;
#[doc = "Channel 0/1 Destination Base Address Register"]
pub mod gdma_dstb;
#[doc = "GDMA_TCNT (rw) register accessor: Channel 0/1 Transfer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_tcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_tcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_tcnt`]
module"]
#[doc(alias = "GDMA_TCNT")]
pub type GdmaTcnt = crate::Reg<gdma_tcnt::GdmaTcntSpec>;
#[doc = "Channel 0/1 Transfer Count Register"]
pub mod gdma_tcnt;
#[doc = "GDMA_CSRC (r) register accessor: Channel 0/1 Current Source Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_csrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_csrc`]
module"]
#[doc(alias = "GDMA_CSRC")]
pub type GdmaCsrc = crate::Reg<gdma_csrc::GdmaCsrcSpec>;
#[doc = "Channel 0/1 Current Source Register"]
pub mod gdma_csrc;
#[doc = "GDMA_CDST (r) register accessor: Channel 0/1 Current Destination Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_cdst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_cdst`]
module"]
#[doc(alias = "GDMA_CDST")]
pub type GdmaCdst = crate::Reg<gdma_cdst::GdmaCdstSpec>;
#[doc = "Channel 0/1 Current Destination Register"]
pub mod gdma_cdst;
#[doc = "GDMA_CTCNT (r) register accessor: Channel 0/1 Current Transfer Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_ctcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_ctcnt`]
module"]
#[doc(alias = "GDMA_CTCNT")]
pub type GdmaCtcnt = crate::Reg<gdma_ctcnt::GdmaCtcntSpec>;
#[doc = "Channel 0/1 Current Transfer Count Register"]
pub mod gdma_ctcnt;
#[doc = "GDMA_REQ_SL (rw) register accessor: Channel 0/1 GDMA Request Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gdma_req_sl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdma_req_sl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdma_req_sl`]
module"]
#[doc(alias = "GDMA_REQ_SL")]
pub type GdmaReqSl = crate::Reg<gdma_req_sl::GdmaReqSlSpec>;
#[doc = "Channel 0/1 GDMA Request Select Register"]
pub mod gdma_req_sl;
