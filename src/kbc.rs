#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    hictrl: Hictrl,
    _reserved1: [u8; 0x01],
    hiirqc: Hiirqc,
    _reserved2: [u8; 0x01],
    hikmst: Hikmst,
    _reserved3: [u8; 0x01],
    hikdo: Hikdo,
    _reserved4: [u8; 0x01],
    himdo: Himdo,
    _reserved5: [u8; 0x01],
    hikmdi: Hikmdi,
    shikmdi: Shikmdi,
}
impl RegisterBlock {
    #[doc = "0x00 - Host Interface Control Register (HICTRL)"]
    #[inline(always)]
    pub const fn hictrl(&self) -> &Hictrl {
        &self.hictrl
    }
    #[doc = "0x02 - Host Interface IRQ Control Register (HIIRQC)"]
    #[inline(always)]
    pub const fn hiirqc(&self) -> &Hiirqc {
        &self.hiirqc
    }
    #[doc = "0x04 - Host Interface Keyboard/Mouse Status Register (HIKMST)"]
    #[inline(always)]
    pub const fn hikmst(&self) -> &Hikmst {
        &self.hikmst
    }
    #[doc = "0x06 - Host Interface Keyboard Data Out Buffer Register (HIKDO)"]
    #[inline(always)]
    pub const fn hikdo(&self) -> &Hikdo {
        &self.hikdo
    }
    #[doc = "0x08 - Host Interface Mouse Data Out Buffer Register (HIMDO)"]
    #[inline(always)]
    pub const fn himdo(&self) -> &Himdo {
        &self.himdo
    }
    #[doc = "0x0a - Host Interface Keyboard/Mouse Data In Buffer Register (HIKMDI)"]
    #[inline(always)]
    pub const fn hikmdi(&self) -> &Hikmdi {
        &self.hikmdi
    }
    #[doc = "0x0b - Host Interface Keyboard/Mouse Shadow Data In Buffer Register (SHIKMDI)"]
    #[inline(always)]
    pub const fn shikmdi(&self) -> &Shikmdi {
        &self.shikmdi
    }
}
#[doc = "HICTRL (rw) register accessor: Host Interface Control Register (HICTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hictrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hictrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hictrl`]
module"]
#[doc(alias = "HICTRL")]
pub type Hictrl = crate::Reg<hictrl::HictrlSpec>;
#[doc = "Host Interface Control Register (HICTRL)"]
pub mod hictrl;
#[doc = "HIIRQC (rw) register accessor: Host Interface IRQ Control Register (HIIRQC)\n\nYou can [`read`](crate::Reg::read) this register and get [`hiirqc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hiirqc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hiirqc`]
module"]
#[doc(alias = "HIIRQC")]
pub type Hiirqc = crate::Reg<hiirqc::HiirqcSpec>;
#[doc = "Host Interface IRQ Control Register (HIIRQC)"]
pub mod hiirqc;
#[doc = "HIKMST (rw) register accessor: Host Interface Keyboard/Mouse Status Register (HIKMST)\n\nYou can [`read`](crate::Reg::read) this register and get [`hikmst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hikmst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hikmst`]
module"]
#[doc(alias = "HIKMST")]
pub type Hikmst = crate::Reg<hikmst::HikmstSpec>;
#[doc = "Host Interface Keyboard/Mouse Status Register (HIKMST)"]
pub mod hikmst;
#[doc = "HIKDO (rw) register accessor: Host Interface Keyboard Data Out Buffer Register (HIKDO)\n\nYou can [`read`](crate::Reg::read) this register and get [`hikdo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hikdo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hikdo`]
module"]
#[doc(alias = "HIKDO")]
pub type Hikdo = crate::Reg<hikdo::HikdoSpec>;
#[doc = "Host Interface Keyboard Data Out Buffer Register (HIKDO)"]
pub mod hikdo;
#[doc = "HIMDO (rw) register accessor: Host Interface Mouse Data Out Buffer Register (HIMDO)\n\nYou can [`read`](crate::Reg::read) this register and get [`himdo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`himdo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@himdo`]
module"]
#[doc(alias = "HIMDO")]
pub type Himdo = crate::Reg<himdo::HimdoSpec>;
#[doc = "Host Interface Mouse Data Out Buffer Register (HIMDO)"]
pub mod himdo;
#[doc = "HIKMDI (rw) register accessor: Host Interface Keyboard/Mouse Data In Buffer Register (HIKMDI)\n\nYou can [`read`](crate::Reg::read) this register and get [`hikmdi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hikmdi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hikmdi`]
module"]
#[doc(alias = "HIKMDI")]
pub type Hikmdi = crate::Reg<hikmdi::HikmdiSpec>;
#[doc = "Host Interface Keyboard/Mouse Data In Buffer Register (HIKMDI)"]
pub mod hikmdi;
#[doc = "SHIKMDI (rw) register accessor: Host Interface Keyboard/Mouse Shadow Data In Buffer Register (SHIKMDI)\n\nYou can [`read`](crate::Reg::read) this register and get [`shikmdi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shikmdi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shikmdi`]
module"]
#[doc(alias = "SHIKMDI")]
pub type Shikmdi = crate::Reg<shikmdi::ShikmdiSpec>;
#[doc = "Host Interface Keyboard/Mouse Shadow Data In Buffer Register (SHIKMDI)"]
pub mod shikmdi;
