#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    ihioa: Ihioa,
    ihd: Ihd,
    _reserved2: [u8; 0x01],
    lksioha: Lksioha,
    _reserved3: [u8; 0x02],
    crsmae: Crsmae,
    sibctrl: Sibctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Indirect Host I/O Address Register (IHIOA)"]
    #[inline(always)]
    pub const fn ihioa(&self) -> &Ihioa {
        &self.ihioa
    }
    #[doc = "0x02 - Indirect Host Data Register (IHD)"]
    #[inline(always)]
    pub const fn ihd(&self) -> &Ihd {
        &self.ihd
    }
    #[doc = "0x04 - Lock Host Access Register (LKSIOHA)"]
    #[inline(always)]
    pub const fn lksioha(&self) -> &Lksioha {
        &self.lksioha
    }
    #[doc = "0x08 - Core-to-Host Modules Access Enable Register (CRSMAE)"]
    #[inline(always)]
    pub const fn crsmae(&self) -> &Crsmae {
        &self.crsmae
    }
    #[doc = "0x0a - HMIB Control Register (SIBCTRL)"]
    #[inline(always)]
    pub const fn sibctrl(&self) -> &Sibctrl {
        &self.sibctrl
    }
}
#[doc = "IHIOA (rw) register accessor: Indirect Host I/O Address Register (IHIOA)\n\nYou can [`read`](crate::Reg::read) this register and get [`ihioa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ihioa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ihioa`]
module"]
#[doc(alias = "IHIOA")]
pub type Ihioa = crate::Reg<ihioa::IhioaSpec>;
#[doc = "Indirect Host I/O Address Register (IHIOA)"]
pub mod ihioa;
#[doc = "IHD (rw) register accessor: Indirect Host Data Register (IHD)\n\nYou can [`read`](crate::Reg::read) this register and get [`ihd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ihd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ihd`]
module"]
#[doc(alias = "IHD")]
pub type Ihd = crate::Reg<ihd::IhdSpec>;
#[doc = "Indirect Host Data Register (IHD)"]
pub mod ihd;
#[doc = "LKSIOHA (rw) register accessor: Lock Host Access Register (LKSIOHA)\n\nYou can [`read`](crate::Reg::read) this register and get [`lksioha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lksioha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lksioha`]
module"]
#[doc(alias = "LKSIOHA")]
pub type Lksioha = crate::Reg<lksioha::LksiohaSpec>;
#[doc = "Lock Host Access Register (LKSIOHA)"]
pub mod lksioha;
#[doc = "CRSMAE (rw) register accessor: Core-to-Host Modules Access Enable Register (CRSMAE)\n\nYou can [`read`](crate::Reg::read) this register and get [`crsmae::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsmae::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsmae`]
module"]
#[doc(alias = "CRSMAE")]
pub type Crsmae = crate::Reg<crsmae::CrsmaeSpec>;
#[doc = "Core-to-Host Modules Access Enable Register (CRSMAE)"]
pub mod crsmae;
#[doc = "SIBCTRL (rw) register accessor: HMIB Control Register (SIBCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`sibctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sibctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sibctrl`]
module"]
#[doc(alias = "SIBCTRL")]
pub type Sibctrl = crate::Reg<sibctrl::SibctrlSpec>;
#[doc = "HMIB Control Register (SIBCTRL)"]
pub mod sibctrl;
