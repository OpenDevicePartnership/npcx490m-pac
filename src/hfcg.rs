#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    hfcgctrl: Hfcgctrl,
    _reserved1: [u8; 0x01],
    hfcgml: Hfcgml,
    _reserved2: [u8; 0x01],
    hfcgmh: Hfcgmh,
    _reserved3: [u8; 0x01],
    hfcgn: Hfcgn,
    _reserved4: [u8; 0x01],
    hfcgp: Hfcgp,
    _reserved5: [u8; 0x07],
    hfcbcd: Hfcbcd,
    _reserved6: [u8; 0x01],
    hfcbcd1: Hfcbcd1,
    _reserved7: [u8; 0x01],
    hfcbcd2: Hfcbcd2,
    _reserved8: [u8; 0x07],
    hfcgp_idl: HfcgpIdl,
    hfcbcd3: Hfcbcd3,
}
impl RegisterBlock {
    #[doc = "0x00 - HFCG Control Register (HFCGCTRL)"]
    #[inline(always)]
    pub const fn hfcgctrl(&self) -> &Hfcgctrl {
        &self.hfcgctrl
    }
    #[doc = "0x02 - HFCG M Low Value Register (HFCGML)"]
    #[inline(always)]
    pub const fn hfcgml(&self) -> &Hfcgml {
        &self.hfcgml
    }
    #[doc = "0x04 - HFCG M High Value Register (HFCGMH)"]
    #[inline(always)]
    pub const fn hfcgmh(&self) -> &Hfcgmh {
        &self.hfcgmh
    }
    #[doc = "0x06 - HFCG N Value Register (HFCGN)"]
    #[inline(always)]
    pub const fn hfcgn(&self) -> &Hfcgn {
        &self.hfcgn
    }
    #[doc = "0x08 - HFCG Prescaler Register (HFCGP)"]
    #[inline(always)]
    pub const fn hfcgp(&self) -> &Hfcgp {
        &self.hfcgp
    }
    #[doc = "0x10 - HFCG Bus Clock Dividers Register (HFCBCD)"]
    #[inline(always)]
    pub const fn hfcbcd(&self) -> &Hfcbcd {
        &self.hfcbcd
    }
    #[doc = "0x12 - HFCG Bus Clock Dividers 1 Register (HFCBCD1)"]
    #[inline(always)]
    pub const fn hfcbcd1(&self) -> &Hfcbcd1 {
        &self.hfcbcd1
    }
    #[doc = "0x14 - HFCG Bus Clock Dividers 2 Register (HFCBCD2)"]
    #[inline(always)]
    pub const fn hfcbcd2(&self) -> &Hfcbcd2 {
        &self.hfcbcd2
    }
    #[doc = "0x1c - HFCG Prescaler in Idle Register (HFCGP_IDL)"]
    #[inline(always)]
    pub const fn hfcgp_idl(&self) -> &HfcgpIdl {
        &self.hfcgp_idl
    }
    #[doc = "0x1d - HFCG Bus Clock Dividers 3 Register (HFCBCD3)"]
    #[inline(always)]
    pub const fn hfcbcd3(&self) -> &Hfcbcd3 {
        &self.hfcbcd3
    }
}
#[doc = "HFCGCTRL (rw) register accessor: HFCG Control Register (HFCGCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcgctrl`]
module"]
#[doc(alias = "HFCGCTRL")]
pub type Hfcgctrl = crate::Reg<hfcgctrl::HfcgctrlSpec>;
#[doc = "HFCG Control Register (HFCGCTRL)"]
pub mod hfcgctrl;
#[doc = "HFCGML (rw) register accessor: HFCG M Low Value Register (HFCGML)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgml::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgml::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcgml`]
module"]
#[doc(alias = "HFCGML")]
pub type Hfcgml = crate::Reg<hfcgml::HfcgmlSpec>;
#[doc = "HFCG M Low Value Register (HFCGML)"]
pub mod hfcgml;
#[doc = "HFCGMH (rw) register accessor: HFCG M High Value Register (HFCGMH)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgmh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgmh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcgmh`]
module"]
#[doc(alias = "HFCGMH")]
pub type Hfcgmh = crate::Reg<hfcgmh::HfcgmhSpec>;
#[doc = "HFCG M High Value Register (HFCGMH)"]
pub mod hfcgmh;
#[doc = "HFCGN (rw) register accessor: HFCG N Value Register (HFCGN)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcgn`]
module"]
#[doc(alias = "HFCGN")]
pub type Hfcgn = crate::Reg<hfcgn::HfcgnSpec>;
#[doc = "HFCG N Value Register (HFCGN)"]
pub mod hfcgn;
#[doc = "HFCGP (rw) register accessor: HFCG Prescaler Register (HFCGP)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcgp`]
module"]
#[doc(alias = "HFCGP")]
pub type Hfcgp = crate::Reg<hfcgp::HfcgpSpec>;
#[doc = "HFCG Prescaler Register (HFCGP)"]
pub mod hfcgp;
#[doc = "HFCBCD (rw) register accessor: HFCG Bus Clock Dividers Register (HFCBCD)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcbcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcbcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcbcd`]
module"]
#[doc(alias = "HFCBCD")]
pub type Hfcbcd = crate::Reg<hfcbcd::HfcbcdSpec>;
#[doc = "HFCG Bus Clock Dividers Register (HFCBCD)"]
pub mod hfcbcd;
#[doc = "HFCBCD1 (rw) register accessor: HFCG Bus Clock Dividers 1 Register (HFCBCD1)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcbcd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcbcd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcbcd1`]
module"]
#[doc(alias = "HFCBCD1")]
pub type Hfcbcd1 = crate::Reg<hfcbcd1::Hfcbcd1Spec>;
#[doc = "HFCG Bus Clock Dividers 1 Register (HFCBCD1)"]
pub mod hfcbcd1;
#[doc = "HFCBCD2 (rw) register accessor: HFCG Bus Clock Dividers 2 Register (HFCBCD2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcbcd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcbcd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcbcd2`]
module"]
#[doc(alias = "HFCBCD2")]
pub type Hfcbcd2 = crate::Reg<hfcbcd2::Hfcbcd2Spec>;
#[doc = "HFCG Bus Clock Dividers 2 Register (HFCBCD2)"]
pub mod hfcbcd2;
#[doc = "HFCGP_IDL (rw) register accessor: HFCG Prescaler in Idle Register (HFCGP_IDL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgp_idl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgp_idl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcgp_idl`]
module"]
#[doc(alias = "HFCGP_IDL")]
pub type HfcgpIdl = crate::Reg<hfcgp_idl::HfcgpIdlSpec>;
#[doc = "HFCG Prescaler in Idle Register (HFCGP_IDL)"]
pub mod hfcgp_idl;
#[doc = "HFCBCD3 (rw) register accessor: HFCG Bus Clock Dividers 3 Register (HFCBCD3)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcbcd3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcbcd3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfcbcd3`]
module"]
#[doc(alias = "HFCBCD3")]
pub type Hfcbcd3 = crate::Reg<hfcbcd3::Hfcbcd3Spec>;
#[doc = "HFCG Bus Clock Dividers 3 Register (HFCBCD3)"]
pub mod hfcbcd3;
