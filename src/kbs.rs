#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_kbs_dly1: [u8; 0x01],
    kbs_dly2: KbsDly2,
    kbs_rtyto: KbsRtyto,
    kbs_cnum: KbsCnum,
    _reserved_4_kbsin: [u8; 0x01],
    kbsinpu: Kbsinpu,
    kbsout0: Kbsout0,
    kbsout1: Kbsout1,
    kbs_buf_indx: KbsBufIndx,
    kbs_buf_data: KbsBufData,
    kbsevt: Kbsevt,
    kbsctl: Kbsctl,
    kbs_cfg_indx: KbsCfgIndx,
    kbs_cfg_data: KbsCfgData,
}
impl RegisterBlock {
    #[doc = "0x00 - Keyboard Scan Delay 1 Register (KBS_DLY1)"]
    #[inline(always)]
    pub const fn kbs_dly1(&self) -> &KbsDly1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Keyboard Scan Buffer Data 0-17 Register (KBSBUF0-17)"]
    #[inline(always)]
    pub const fn kbsbuf017(&self) -> &Kbsbuf017 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x01 - Keyboard Scan Delay 2 Register (KBS_DLY2)"]
    #[inline(always)]
    pub const fn kbs_dly2(&self) -> &KbsDly2 {
        &self.kbs_dly2
    }
    #[doc = "0x02 - Keyboard Scan Retry Timeout Register (KBS_RTYTO)"]
    #[inline(always)]
    pub const fn kbs_rtyto(&self) -> &KbsRtyto {
        &self.kbs_rtyto
    }
    #[doc = "0x03 - Keyboard Scan Columns Number Register (KBS_CNUM)"]
    #[inline(always)]
    pub const fn kbs_cnum(&self) -> &KbsCnum {
        &self.kbs_cnum
    }
    #[doc = "0x04 - Keyboard Scan Clock Divisor Register (KBS_CDIV)"]
    #[inline(always)]
    pub const fn kbs_cdiv(&self) -> &KbsCdiv {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Keyboard Scan In Register (KBSIN)"]
    #[inline(always)]
    pub const fn kbsin(&self) -> &Kbsin {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x05 - Keyboard Scan In Pull-Up Enable Register (KBSINPU)"]
    #[inline(always)]
    pub const fn kbsinpu(&self) -> &Kbsinpu {
        &self.kbsinpu
    }
    #[doc = "0x06 - Keyboard Scan Out 0 Register (KBSOUT0)"]
    #[inline(always)]
    pub const fn kbsout0(&self) -> &Kbsout0 {
        &self.kbsout0
    }
    #[doc = "0x08 - Keyboard Scan Out 1 Register (KBSOUT1)"]
    #[inline(always)]
    pub const fn kbsout1(&self) -> &Kbsout1 {
        &self.kbsout1
    }
    #[doc = "0x0a - Keyboard Scan Buffer Index Register (KBS_BUF_INDX)"]
    #[inline(always)]
    pub const fn kbs_buf_indx(&self) -> &KbsBufIndx {
        &self.kbs_buf_indx
    }
    #[doc = "0x0b - Keyboard Scan Buffer Data Register (KBS_BUF_DATA)"]
    #[inline(always)]
    pub const fn kbs_buf_data(&self) -> &KbsBufData {
        &self.kbs_buf_data
    }
    #[doc = "0x0c - Keyboard Scan Event Register (KBSEVT)"]
    #[inline(always)]
    pub const fn kbsevt(&self) -> &Kbsevt {
        &self.kbsevt
    }
    #[doc = "0x0d - Keyboard Scan Control Register (KBSCTL)"]
    #[inline(always)]
    pub const fn kbsctl(&self) -> &Kbsctl {
        &self.kbsctl
    }
    #[doc = "0x0e - Keyboard Scan Configuration Index Register (KBS_CFG_INDX)"]
    #[inline(always)]
    pub const fn kbs_cfg_indx(&self) -> &KbsCfgIndx {
        &self.kbs_cfg_indx
    }
    #[doc = "0x0f - Keyboard Scan Configuration Data Register (KBS_CFG_DATA)"]
    #[inline(always)]
    pub const fn kbs_cfg_data(&self) -> &KbsCfgData {
        &self.kbs_cfg_data
    }
}
#[doc = "KBSBUF0-17 (rw) register accessor: Keyboard Scan Buffer Data 0-17 Register (KBSBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsbuf017::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsbuf017::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbsbuf017`]
module"]
#[doc(alias = "KBSBUF0-17")]
pub type Kbsbuf017 = crate::Reg<kbsbuf017::Kbsbuf017Spec>;
#[doc = "Keyboard Scan Buffer Data 0-17 Register (KBSBUF0-17)"]
pub mod kbsbuf017;
#[doc = "KBS_DLY1 (rw) register accessor: Keyboard Scan Delay 1 Register (KBS_DLY1)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_dly1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_dly1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbs_dly1`]
module"]
#[doc(alias = "KBS_DLY1")]
pub type KbsDly1 = crate::Reg<kbs_dly1::KbsDly1Spec>;
#[doc = "Keyboard Scan Delay 1 Register (KBS_DLY1)"]
pub mod kbs_dly1;
#[doc = "KBS_DLY2 (rw) register accessor: Keyboard Scan Delay 2 Register (KBS_DLY2)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_dly2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_dly2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbs_dly2`]
module"]
#[doc(alias = "KBS_DLY2")]
pub type KbsDly2 = crate::Reg<kbs_dly2::KbsDly2Spec>;
#[doc = "Keyboard Scan Delay 2 Register (KBS_DLY2)"]
pub mod kbs_dly2;
#[doc = "KBS_RTYTO (rw) register accessor: Keyboard Scan Retry Timeout Register (KBS_RTYTO)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_rtyto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_rtyto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbs_rtyto`]
module"]
#[doc(alias = "KBS_RTYTO")]
pub type KbsRtyto = crate::Reg<kbs_rtyto::KbsRtytoSpec>;
#[doc = "Keyboard Scan Retry Timeout Register (KBS_RTYTO)"]
pub mod kbs_rtyto;
#[doc = "KBS_CNUM (rw) register accessor: Keyboard Scan Columns Number Register (KBS_CNUM)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_cnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_cnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbs_cnum`]
module"]
#[doc(alias = "KBS_CNUM")]
pub type KbsCnum = crate::Reg<kbs_cnum::KbsCnumSpec>;
#[doc = "Keyboard Scan Columns Number Register (KBS_CNUM)"]
pub mod kbs_cnum;
#[doc = "KBSIN (r) register accessor: Keyboard Scan In Register (KBSIN)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsin::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbsin`]
module"]
#[doc(alias = "KBSIN")]
pub type Kbsin = crate::Reg<kbsin::KbsinSpec>;
#[doc = "Keyboard Scan In Register (KBSIN)"]
pub mod kbsin;
#[doc = "KBS_CDIV (rw) register accessor: Keyboard Scan Clock Divisor Register (KBS_CDIV)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_cdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_cdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbs_cdiv`]
module"]
#[doc(alias = "KBS_CDIV")]
pub type KbsCdiv = crate::Reg<kbs_cdiv::KbsCdivSpec>;
#[doc = "Keyboard Scan Clock Divisor Register (KBS_CDIV)"]
pub mod kbs_cdiv;
#[doc = "KBSINPU (rw) register accessor: Keyboard Scan In Pull-Up Enable Register (KBSINPU)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsinpu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsinpu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbsinpu`]
module"]
#[doc(alias = "KBSINPU")]
pub type Kbsinpu = crate::Reg<kbsinpu::KbsinpuSpec>;
#[doc = "Keyboard Scan In Pull-Up Enable Register (KBSINPU)"]
pub mod kbsinpu;
#[doc = "KBSOUT0 (rw) register accessor: Keyboard Scan Out 0 Register (KBSOUT0)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsout0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsout0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbsout0`]
module"]
#[doc(alias = "KBSOUT0")]
pub type Kbsout0 = crate::Reg<kbsout0::Kbsout0Spec>;
#[doc = "Keyboard Scan Out 0 Register (KBSOUT0)"]
pub mod kbsout0;
#[doc = "KBSOUT1 (rw) register accessor: Keyboard Scan Out 1 Register (KBSOUT1)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsout1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsout1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbsout1`]
module"]
#[doc(alias = "KBSOUT1")]
pub type Kbsout1 = crate::Reg<kbsout1::Kbsout1Spec>;
#[doc = "Keyboard Scan Out 1 Register (KBSOUT1)"]
pub mod kbsout1;
#[doc = "KBS_BUF_INDX (rw) register accessor: Keyboard Scan Buffer Index Register (KBS_BUF_INDX)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_buf_indx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_buf_indx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbs_buf_indx`]
module"]
#[doc(alias = "KBS_BUF_INDX")]
pub type KbsBufIndx = crate::Reg<kbs_buf_indx::KbsBufIndxSpec>;
#[doc = "Keyboard Scan Buffer Index Register (KBS_BUF_INDX)"]
pub mod kbs_buf_indx;
#[doc = "KBS_BUF_DATA (rw) register accessor: Keyboard Scan Buffer Data Register (KBS_BUF_DATA)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_buf_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_buf_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbs_buf_data`]
module"]
#[doc(alias = "KBS_BUF_DATA")]
pub type KbsBufData = crate::Reg<kbs_buf_data::KbsBufDataSpec>;
#[doc = "Keyboard Scan Buffer Data Register (KBS_BUF_DATA)"]
pub mod kbs_buf_data;
#[doc = "KBSEVT (rw) register accessor: Keyboard Scan Event Register (KBSEVT)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsevt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsevt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbsevt`]
module"]
#[doc(alias = "KBSEVT")]
pub type Kbsevt = crate::Reg<kbsevt::KbsevtSpec>;
#[doc = "Keyboard Scan Event Register (KBSEVT)"]
pub mod kbsevt;
#[doc = "KBSCTL (rw) register accessor: Keyboard Scan Control Register (KBSCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbsctl`]
module"]
#[doc(alias = "KBSCTL")]
pub type Kbsctl = crate::Reg<kbsctl::KbsctlSpec>;
#[doc = "Keyboard Scan Control Register (KBSCTL)"]
pub mod kbsctl;
#[doc = "KBS_CFG_INDX (rw) register accessor: Keyboard Scan Configuration Index Register (KBS_CFG_INDX)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_cfg_indx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_cfg_indx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbs_cfg_indx`]
module"]
#[doc(alias = "KBS_CFG_INDX")]
pub type KbsCfgIndx = crate::Reg<kbs_cfg_indx::KbsCfgIndxSpec>;
#[doc = "Keyboard Scan Configuration Index Register (KBS_CFG_INDX)"]
pub mod kbs_cfg_indx;
#[doc = "KBS_CFG_DATA (rw) register accessor: Keyboard Scan Configuration Data Register (KBS_CFG_DATA)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbs_cfg_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbs_cfg_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kbs_cfg_data`]
module"]
#[doc(alias = "KBS_CFG_DATA")]
pub type KbsCfgData = crate::Reg<kbs_cfg_data::KbsCfgDataSpec>;
#[doc = "Keyboard Scan Configuration Data Register (KBS_CFG_DATA)"]
pub mod kbs_cfg_data;
