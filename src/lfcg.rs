#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    lfcgctl: Lfcgctl,
    _reserved1: [u8; 0x01],
    hfrdi: Hfrdi,
    hfrdf: Hfrdf,
    frcdiv: Frcdiv,
    divcor1: Divcor1,
    divcor2: Divcor2,
    _reserved6: [u8; 0x08],
    lfcgctl2: Lfcgctl2,
}
impl RegisterBlock {
    #[doc = "0x00 - LFCG Control Register (LFCGCTL)"]
    #[inline(always)]
    pub const fn lfcgctl(&self) -> &Lfcgctl {
        &self.lfcgctl
    }
    #[doc = "0x02 - High-Frequency Reference Divisor I Register (HFRDI)"]
    #[inline(always)]
    pub const fn hfrdi(&self) -> &Hfrdi {
        &self.hfrdi
    }
    #[doc = "0x04 - High-Frequency Reference Divisor F Register (HFRDF)"]
    #[inline(always)]
    pub const fn hfrdf(&self) -> &Hfrdf {
        &self.hfrdf
    }
    #[doc = "0x06 - FRCLK Clock Divisor Register (FRCDIV)"]
    #[inline(always)]
    pub const fn frcdiv(&self) -> &Frcdiv {
        &self.frcdiv
    }
    #[doc = "0x08 - Divisor Correction Value 1 Register (DIVCOR1)"]
    #[inline(always)]
    pub const fn divcor1(&self) -> &Divcor1 {
        &self.divcor1
    }
    #[doc = "0x0a - Divisor Correction Value 2 Register (DIVCOR2)"]
    #[inline(always)]
    pub const fn divcor2(&self) -> &Divcor2 {
        &self.divcor2
    }
    #[doc = "0x14 - LFCG Control 2 Register (LFCGCTL2)"]
    #[inline(always)]
    pub const fn lfcgctl2(&self) -> &Lfcgctl2 {
        &self.lfcgctl2
    }
}
#[doc = "LFCGCTL (rw) register accessor: LFCG Control Register (LFCGCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfcgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfcgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfcgctl`]
module"]
#[doc(alias = "LFCGCTL")]
pub type Lfcgctl = crate::Reg<lfcgctl::LfcgctlSpec>;
#[doc = "LFCG Control Register (LFCGCTL)"]
pub mod lfcgctl;
#[doc = "HFRDI (rw) register accessor: High-Frequency Reference Divisor I Register (HFRDI)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrdi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrdi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrdi`]
module"]
#[doc(alias = "HFRDI")]
pub type Hfrdi = crate::Reg<hfrdi::HfrdiSpec>;
#[doc = "High-Frequency Reference Divisor I Register (HFRDI)"]
pub mod hfrdi;
#[doc = "HFRDF (rw) register accessor: High-Frequency Reference Divisor F Register (HFRDF)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrdf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrdf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrdf`]
module"]
#[doc(alias = "HFRDF")]
pub type Hfrdf = crate::Reg<hfrdf::HfrdfSpec>;
#[doc = "High-Frequency Reference Divisor F Register (HFRDF)"]
pub mod hfrdf;
#[doc = "FRCDIV (rw) register accessor: FRCLK Clock Divisor Register (FRCDIV)\n\nYou can [`read`](crate::Reg::read) this register and get [`frcdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frcdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frcdiv`]
module"]
#[doc(alias = "FRCDIV")]
pub type Frcdiv = crate::Reg<frcdiv::FrcdivSpec>;
#[doc = "FRCLK Clock Divisor Register (FRCDIV)"]
pub mod frcdiv;
#[doc = "DIVCOR1 (rw) register accessor: Divisor Correction Value 1 Register (DIVCOR1)\n\nYou can [`read`](crate::Reg::read) this register and get [`divcor1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divcor1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divcor1`]
module"]
#[doc(alias = "DIVCOR1")]
pub type Divcor1 = crate::Reg<divcor1::Divcor1Spec>;
#[doc = "Divisor Correction Value 1 Register (DIVCOR1)"]
pub mod divcor1;
#[doc = "DIVCOR2 (rw) register accessor: Divisor Correction Value 2 Register (DIVCOR2)\n\nYou can [`read`](crate::Reg::read) this register and get [`divcor2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divcor2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divcor2`]
module"]
#[doc(alias = "DIVCOR2")]
pub type Divcor2 = crate::Reg<divcor2::Divcor2Spec>;
#[doc = "Divisor Correction Value 2 Register (DIVCOR2)"]
pub mod divcor2;
#[doc = "LFCGCTL2 (rw) register accessor: LFCG Control 2 Register (LFCGCTL2)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfcgctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfcgctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfcgctl2`]
module"]
#[doc(alias = "LFCGCTL2")]
pub type Lfcgctl2 = crate::Reg<lfcgctl2::Lfcgctl2Spec>;
#[doc = "LFCG Control 2 Register (LFCGCTL2)"]
pub mod lfcgctl2;
