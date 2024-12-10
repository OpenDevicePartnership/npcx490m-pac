#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Cluster CHANNEL%s, containing GDMA_CTL*, GDMA_SRCB*, GDMA_DSTB*, GDMA_TCNT*, GDMA_CSRC*, GDMA_CDST*, GDMA_CTCNT*, GDMA_REQ_SL*"]
#[doc(alias = "CHANNEL")]
pub struct Channel {
    ctl: Ctl,
    srcb: Srcb,
    dstb: Dstb,
    tcnt: Tcnt,
    csrc: Csrc,
    cdst: Cdst,
    ctcnt: Ctcnt,
    req_sl: ReqSl,
}
impl Channel {
    #[doc = "0x00 - Channel 0/1 Control Register (GDMAn_CTL0, GDMAn_CTL1)"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Channel 0/1 Source Base Address Register (GDMAn_SRCB0, GDMAn_SRCB1)"]
    #[inline(always)]
    pub const fn srcb(&self) -> &Srcb {
        &self.srcb
    }
    #[doc = "0x08 - Channel 0/1 Destination Base Address Register (GDMAn_DSTB0, GDMAn_DSTB1)"]
    #[inline(always)]
    pub const fn dstb(&self) -> &Dstb {
        &self.dstb
    }
    #[doc = "0x0c - Channel 0/1 Transfer Count Register (GDMAn_TCNT0, GDMAn_TCNT1)"]
    #[inline(always)]
    pub const fn tcnt(&self) -> &Tcnt {
        &self.tcnt
    }
    #[doc = "0x10 - Channel 0/1 Current Source Register (GDMAn_CSRC0, GDMAn_CSRC1)"]
    #[inline(always)]
    pub const fn csrc(&self) -> &Csrc {
        &self.csrc
    }
    #[doc = "0x14 - Channel 0/1 Current Destination Register (GDMAn_CDST0, GDMAn_CDST1)"]
    #[inline(always)]
    pub const fn cdst(&self) -> &Cdst {
        &self.cdst
    }
    #[doc = "0x18 - Channel 0/1 Current Transfer Count Register (GDMAn_CTCNT0, GDMAn_CTCNT1)"]
    #[inline(always)]
    pub const fn ctcnt(&self) -> &Ctcnt {
        &self.ctcnt
    }
    #[doc = "0x1c - Channel 0/1 GDMA Request Select Register (GDMAn_REQ_SL0, GDMAn_REQ_SL1)"]
    #[inline(always)]
    pub const fn req_sl(&self) -> &ReqSl {
        &self.req_sl
    }
}
#[doc = "CTL (rw) register accessor: Channel 0/1 Control Register (GDMAn_CTL0, GDMAn_CTL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Channel 0/1 Control Register (GDMAn_CTL0, GDMAn_CTL1)"]
pub mod ctl;
#[doc = "SRCB (rw) register accessor: Channel 0/1 Source Base Address Register (GDMAn_SRCB0, GDMAn_SRCB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`srcb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcb`]
module"]
#[doc(alias = "SRCB")]
pub type Srcb = crate::Reg<srcb::SrcbSpec>;
#[doc = "Channel 0/1 Source Base Address Register (GDMAn_SRCB0, GDMAn_SRCB1)"]
pub mod srcb;
#[doc = "DSTB (rw) register accessor: Channel 0/1 Destination Base Address Register (GDMAn_DSTB0, GDMAn_DSTB1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dstb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dstb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dstb`]
module"]
#[doc(alias = "DSTB")]
pub type Dstb = crate::Reg<dstb::DstbSpec>;
#[doc = "Channel 0/1 Destination Base Address Register (GDMAn_DSTB0, GDMAn_DSTB1)"]
pub mod dstb;
#[doc = "TCNT (rw) register accessor: Channel 0/1 Transfer Count Register (GDMAn_TCNT0, GDMAn_TCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`tcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcnt`]
module"]
#[doc(alias = "TCNT")]
pub type Tcnt = crate::Reg<tcnt::TcntSpec>;
#[doc = "Channel 0/1 Transfer Count Register (GDMAn_TCNT0, GDMAn_TCNT1)"]
pub mod tcnt;
#[doc = "CSRC (rw) register accessor: Channel 0/1 Current Source Register (GDMAn_CSRC0, GDMAn_CSRC1)\n\nYou can [`read`](crate::Reg::read) this register and get [`csrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csrc`]
module"]
#[doc(alias = "CSRC")]
pub type Csrc = crate::Reg<csrc::CsrcSpec>;
#[doc = "Channel 0/1 Current Source Register (GDMAn_CSRC0, GDMAn_CSRC1)"]
pub mod csrc;
#[doc = "CDST (rw) register accessor: Channel 0/1 Current Destination Register (GDMAn_CDST0, GDMAn_CDST1)\n\nYou can [`read`](crate::Reg::read) this register and get [`cdst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdst`]
module"]
#[doc(alias = "CDST")]
pub type Cdst = crate::Reg<cdst::CdstSpec>;
#[doc = "Channel 0/1 Current Destination Register (GDMAn_CDST0, GDMAn_CDST1)"]
pub mod cdst;
#[doc = "CTCNT (rw) register accessor: Channel 0/1 Current Transfer Count Register (GDMAn_CTCNT0, GDMAn_CTCNT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctcnt`]
module"]
#[doc(alias = "CTCNT")]
pub type Ctcnt = crate::Reg<ctcnt::CtcntSpec>;
#[doc = "Channel 0/1 Current Transfer Count Register (GDMAn_CTCNT0, GDMAn_CTCNT1)"]
pub mod ctcnt;
#[doc = "REQ_SL (rw) register accessor: Channel 0/1 GDMA Request Select Register (GDMAn_REQ_SL0, GDMAn_REQ_SL1)\n\nYou can [`read`](crate::Reg::read) this register and get [`req_sl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`req_sl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@req_sl`]
module"]
#[doc(alias = "REQ_SL")]
pub type ReqSl = crate::Reg<req_sl::ReqSlSpec>;
#[doc = "Channel 0/1 GDMA Request Select Register (GDMAn_REQ_SL0, GDMAn_REQ_SL1)"]
pub mod req_sl;
