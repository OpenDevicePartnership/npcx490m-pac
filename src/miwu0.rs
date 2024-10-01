#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    wkedgn0: Wkedgn0,
    wkaedgn0: Wkaedgn0,
    wkmodn0: Wkmodn0,
    wkpndn0: Wkpndn0,
    wkpcln0: Wkpcln0,
    wkenn0: Wkenn0,
    wkstn0: Wkstn0,
    wkinenn0: Wkinenn0,
    _reserved8: [u8; 0x06],
    low_pnd_wun: LowPndWun,
    _reserved9: [u8; 0x01],
    wkedgn2: Wkedgn2,
    wkaedgn2: Wkaedgn2,
    wkmodn2: Wkmodn2,
    wkpndn2: Wkpndn2,
    wkpcln2: Wkpcln2,
    wkenn2: Wkenn2,
    wkstn2: Wkstn2,
    wkinenn2: Wkinenn2,
    _reserved17: [u8; 0x08],
    wkedgn4: Wkedgn4,
    wkaedgn4: Wkaedgn4,
    wkmodn4: Wkmodn4,
    wkpndn4: Wkpndn4,
    wkpcln4: Wkpcln4,
    wkenn4: Wkenn4,
    wkstn4: Wkstn4,
    wkinenn4: Wkinenn4,
    _reserved25: [u8; 0x08],
    wkedgn6: Wkedgn6,
    wkaedgn6: Wkaedgn6,
    wkmodn6: Wkmodn6,
    wkpndn6: Wkpndn6,
    wkpcln6: Wkpcln6,
    wkenn6: Wkenn6,
    wkstn6: Wkstn6,
    wkinenn6: Wkinenn6,
    _reserved33: [u8; 0x08],
    wkedgn1: Wkedgn1,
    wkaedgn1: Wkaedgn1,
    wkmodn1: Wkmodn1,
    wkpndn1: Wkpndn1,
    wkpcln1: Wkpcln1,
    wkenn1: Wkenn1,
    wkstn1: Wkstn1,
    wkinenn1: Wkinenn1,
    _reserved41: [u8; 0x08],
    wkedgn3: Wkedgn3,
    wkaedgn3: Wkaedgn3,
    wkmodn3: Wkmodn3,
    wkpndn3: Wkpndn3,
    wkpcln3: Wkpcln3,
    wkenn3: Wkenn3,
    wkstn3: Wkstn3,
    wkinenn3: Wkinenn3,
    _reserved49: [u8; 0x08],
    wkedgn5: Wkedgn5,
    wkaedgn5: Wkaedgn5,
    wkmodn5: Wkmodn5,
    wkpndn5: Wkpndn5,
    wkpcln5: Wkpcln5,
    wkenn5: Wkenn5,
    wkstn5: Wkstn5,
    wkinenn5: Wkinenn5,
    _reserved57: [u8; 0x08],
    wkedgn7: Wkedgn7,
    wkaedgn7: Wkaedgn7,
    wkmodn7: Wkmodn7,
    wkpndn7: Wkpndn7,
    wkpcln7: Wkpcln7,
    wkenn7: Wkenn7,
    wkstn7: Wkstn7,
    wkinenn7: Wkinenn7,
}
impl RegisterBlock {
    #[doc = "0x00 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn0(&self) -> &Wkedgn0 {
        &self.wkedgn0
    }
    #[doc = "0x01 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn0(&self) -> &Wkaedgn0 {
        &self.wkaedgn0
    }
    #[doc = "0x02 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn0(&self) -> &Wkmodn0 {
        &self.wkmodn0
    }
    #[doc = "0x03 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn0(&self) -> &Wkpndn0 {
        &self.wkpndn0
    }
    #[doc = "0x04 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln0(&self) -> &Wkpcln0 {
        &self.wkpcln0
    }
    #[doc = "0x05 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn0(&self) -> &Wkenn0 {
        &self.wkenn0
    }
    #[doc = "0x06 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn0(&self) -> &Wkstn0 {
        &self.wkstn0
    }
    #[doc = "0x07 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn0(&self) -> &Wkinenn0 {
        &self.wkinenn0
    }
    #[doc = "0x0e - Lowest Pending Wake-Up Register (LOW_PND_WUn)"]
    #[inline(always)]
    pub const fn low_pnd_wun(&self) -> &LowPndWun {
        &self.low_pnd_wun
    }
    #[doc = "0x10 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn2(&self) -> &Wkedgn2 {
        &self.wkedgn2
    }
    #[doc = "0x11 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn2(&self) -> &Wkaedgn2 {
        &self.wkaedgn2
    }
    #[doc = "0x12 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn2(&self) -> &Wkmodn2 {
        &self.wkmodn2
    }
    #[doc = "0x13 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn2(&self) -> &Wkpndn2 {
        &self.wkpndn2
    }
    #[doc = "0x14 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln2(&self) -> &Wkpcln2 {
        &self.wkpcln2
    }
    #[doc = "0x15 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn2(&self) -> &Wkenn2 {
        &self.wkenn2
    }
    #[doc = "0x16 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn2(&self) -> &Wkstn2 {
        &self.wkstn2
    }
    #[doc = "0x17 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn2(&self) -> &Wkinenn2 {
        &self.wkinenn2
    }
    #[doc = "0x20 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn4(&self) -> &Wkedgn4 {
        &self.wkedgn4
    }
    #[doc = "0x21 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn4(&self) -> &Wkaedgn4 {
        &self.wkaedgn4
    }
    #[doc = "0x22 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn4(&self) -> &Wkmodn4 {
        &self.wkmodn4
    }
    #[doc = "0x23 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn4(&self) -> &Wkpndn4 {
        &self.wkpndn4
    }
    #[doc = "0x24 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln4(&self) -> &Wkpcln4 {
        &self.wkpcln4
    }
    #[doc = "0x25 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn4(&self) -> &Wkenn4 {
        &self.wkenn4
    }
    #[doc = "0x26 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn4(&self) -> &Wkstn4 {
        &self.wkstn4
    }
    #[doc = "0x27 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn4(&self) -> &Wkinenn4 {
        &self.wkinenn4
    }
    #[doc = "0x30 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn6(&self) -> &Wkedgn6 {
        &self.wkedgn6
    }
    #[doc = "0x31 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn6(&self) -> &Wkaedgn6 {
        &self.wkaedgn6
    }
    #[doc = "0x32 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn6(&self) -> &Wkmodn6 {
        &self.wkmodn6
    }
    #[doc = "0x33 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn6(&self) -> &Wkpndn6 {
        &self.wkpndn6
    }
    #[doc = "0x34 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln6(&self) -> &Wkpcln6 {
        &self.wkpcln6
    }
    #[doc = "0x35 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn6(&self) -> &Wkenn6 {
        &self.wkenn6
    }
    #[doc = "0x36 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn6(&self) -> &Wkstn6 {
        &self.wkstn6
    }
    #[doc = "0x37 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn6(&self) -> &Wkinenn6 {
        &self.wkinenn6
    }
    #[doc = "0x40 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn1(&self) -> &Wkedgn1 {
        &self.wkedgn1
    }
    #[doc = "0x41 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn1(&self) -> &Wkaedgn1 {
        &self.wkaedgn1
    }
    #[doc = "0x42 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn1(&self) -> &Wkmodn1 {
        &self.wkmodn1
    }
    #[doc = "0x43 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn1(&self) -> &Wkpndn1 {
        &self.wkpndn1
    }
    #[doc = "0x44 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln1(&self) -> &Wkpcln1 {
        &self.wkpcln1
    }
    #[doc = "0x45 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn1(&self) -> &Wkenn1 {
        &self.wkenn1
    }
    #[doc = "0x46 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn1(&self) -> &Wkstn1 {
        &self.wkstn1
    }
    #[doc = "0x47 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn1(&self) -> &Wkinenn1 {
        &self.wkinenn1
    }
    #[doc = "0x50 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn3(&self) -> &Wkedgn3 {
        &self.wkedgn3
    }
    #[doc = "0x51 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn3(&self) -> &Wkaedgn3 {
        &self.wkaedgn3
    }
    #[doc = "0x52 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn3(&self) -> &Wkmodn3 {
        &self.wkmodn3
    }
    #[doc = "0x53 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn3(&self) -> &Wkpndn3 {
        &self.wkpndn3
    }
    #[doc = "0x54 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln3(&self) -> &Wkpcln3 {
        &self.wkpcln3
    }
    #[doc = "0x55 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn3(&self) -> &Wkenn3 {
        &self.wkenn3
    }
    #[doc = "0x56 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn3(&self) -> &Wkstn3 {
        &self.wkstn3
    }
    #[doc = "0x57 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn3(&self) -> &Wkinenn3 {
        &self.wkinenn3
    }
    #[doc = "0x60 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn5(&self) -> &Wkedgn5 {
        &self.wkedgn5
    }
    #[doc = "0x61 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn5(&self) -> &Wkaedgn5 {
        &self.wkaedgn5
    }
    #[doc = "0x62 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn5(&self) -> &Wkmodn5 {
        &self.wkmodn5
    }
    #[doc = "0x63 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn5(&self) -> &Wkpndn5 {
        &self.wkpndn5
    }
    #[doc = "0x64 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln5(&self) -> &Wkpcln5 {
        &self.wkpcln5
    }
    #[doc = "0x65 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn5(&self) -> &Wkenn5 {
        &self.wkenn5
    }
    #[doc = "0x66 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn5(&self) -> &Wkstn5 {
        &self.wkstn5
    }
    #[doc = "0x67 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn5(&self) -> &Wkinenn5 {
        &self.wkinenn5
    }
    #[doc = "0x70 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn7(&self) -> &Wkedgn7 {
        &self.wkedgn7
    }
    #[doc = "0x71 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn7(&self) -> &Wkaedgn7 {
        &self.wkaedgn7
    }
    #[doc = "0x72 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn7(&self) -> &Wkmodn7 {
        &self.wkmodn7
    }
    #[doc = "0x73 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn7(&self) -> &Wkpndn7 {
        &self.wkpndn7
    }
    #[doc = "0x74 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln7(&self) -> &Wkpcln7 {
        &self.wkpcln7
    }
    #[doc = "0x75 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn7(&self) -> &Wkenn7 {
        &self.wkenn7
    }
    #[doc = "0x76 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn7(&self) -> &Wkstn7 {
        &self.wkstn7
    }
    #[doc = "0x77 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn7(&self) -> &Wkinenn7 {
        &self.wkinenn7
    }
}
#[doc = "WKEDGn0 (rw) register accessor: Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkedgn0`]
module"]
#[doc(alias = "WKEDGn0")]
pub type Wkedgn0 = crate::Reg<wkedgn0::Wkedgn0Spec>;
#[doc = "Edge Detection nx Register (WKEDGnx)"]
pub mod wkedgn0;
#[doc = "WKEDGn1 (rw) register accessor: Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkedgn1`]
module"]
#[doc(alias = "WKEDGn1")]
pub type Wkedgn1 = crate::Reg<wkedgn1::Wkedgn1Spec>;
#[doc = "Edge Detection nx Register (WKEDGnx)"]
pub mod wkedgn1;
#[doc = "WKEDGn2 (rw) register accessor: Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkedgn2`]
module"]
#[doc(alias = "WKEDGn2")]
pub type Wkedgn2 = crate::Reg<wkedgn2::Wkedgn2Spec>;
#[doc = "Edge Detection nx Register (WKEDGnx)"]
pub mod wkedgn2;
#[doc = "WKEDGn3 (rw) register accessor: Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkedgn3`]
module"]
#[doc(alias = "WKEDGn3")]
pub type Wkedgn3 = crate::Reg<wkedgn3::Wkedgn3Spec>;
#[doc = "Edge Detection nx Register (WKEDGnx)"]
pub mod wkedgn3;
#[doc = "WKEDGn4 (rw) register accessor: Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkedgn4`]
module"]
#[doc(alias = "WKEDGn4")]
pub type Wkedgn4 = crate::Reg<wkedgn4::Wkedgn4Spec>;
#[doc = "Edge Detection nx Register (WKEDGnx)"]
pub mod wkedgn4;
#[doc = "WKEDGn5 (rw) register accessor: Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkedgn5`]
module"]
#[doc(alias = "WKEDGn5")]
pub type Wkedgn5 = crate::Reg<wkedgn5::Wkedgn5Spec>;
#[doc = "Edge Detection nx Register (WKEDGnx)"]
pub mod wkedgn5;
#[doc = "WKEDGn6 (rw) register accessor: Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkedgn6`]
module"]
#[doc(alias = "WKEDGn6")]
pub type Wkedgn6 = crate::Reg<wkedgn6::Wkedgn6Spec>;
#[doc = "Edge Detection nx Register (WKEDGnx)"]
pub mod wkedgn6;
#[doc = "WKEDGn7 (rw) register accessor: Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkedgn7`]
module"]
#[doc(alias = "WKEDGn7")]
pub type Wkedgn7 = crate::Reg<wkedgn7::Wkedgn7Spec>;
#[doc = "Edge Detection nx Register (WKEDGnx)"]
pub mod wkedgn7;
#[doc = "WKAEDGn0 (rw) register accessor: Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkaedgn0`]
module"]
#[doc(alias = "WKAEDGn0")]
pub type Wkaedgn0 = crate::Reg<wkaedgn0::Wkaedgn0Spec>;
#[doc = "Any Edge Detection nx Register (WKAEDGnx)"]
pub mod wkaedgn0;
#[doc = "WKAEDGn1 (rw) register accessor: Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkaedgn1`]
module"]
#[doc(alias = "WKAEDGn1")]
pub type Wkaedgn1 = crate::Reg<wkaedgn1::Wkaedgn1Spec>;
#[doc = "Any Edge Detection nx Register (WKAEDGnx)"]
pub mod wkaedgn1;
#[doc = "WKAEDGn2 (rw) register accessor: Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkaedgn2`]
module"]
#[doc(alias = "WKAEDGn2")]
pub type Wkaedgn2 = crate::Reg<wkaedgn2::Wkaedgn2Spec>;
#[doc = "Any Edge Detection nx Register (WKAEDGnx)"]
pub mod wkaedgn2;
#[doc = "WKAEDGn3 (rw) register accessor: Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkaedgn3`]
module"]
#[doc(alias = "WKAEDGn3")]
pub type Wkaedgn3 = crate::Reg<wkaedgn3::Wkaedgn3Spec>;
#[doc = "Any Edge Detection nx Register (WKAEDGnx)"]
pub mod wkaedgn3;
#[doc = "WKAEDGn4 (rw) register accessor: Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkaedgn4`]
module"]
#[doc(alias = "WKAEDGn4")]
pub type Wkaedgn4 = crate::Reg<wkaedgn4::Wkaedgn4Spec>;
#[doc = "Any Edge Detection nx Register (WKAEDGnx)"]
pub mod wkaedgn4;
#[doc = "WKAEDGn5 (rw) register accessor: Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkaedgn5`]
module"]
#[doc(alias = "WKAEDGn5")]
pub type Wkaedgn5 = crate::Reg<wkaedgn5::Wkaedgn5Spec>;
#[doc = "Any Edge Detection nx Register (WKAEDGnx)"]
pub mod wkaedgn5;
#[doc = "WKAEDGn6 (rw) register accessor: Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkaedgn6`]
module"]
#[doc(alias = "WKAEDGn6")]
pub type Wkaedgn6 = crate::Reg<wkaedgn6::Wkaedgn6Spec>;
#[doc = "Any Edge Detection nx Register (WKAEDGnx)"]
pub mod wkaedgn6;
#[doc = "WKAEDGn7 (rw) register accessor: Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkaedgn7`]
module"]
#[doc(alias = "WKAEDGn7")]
pub type Wkaedgn7 = crate::Reg<wkaedgn7::Wkaedgn7Spec>;
#[doc = "Any Edge Detection nx Register (WKAEDGnx)"]
pub mod wkaedgn7;
#[doc = "WKMODn0 (rw) register accessor: Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkmodn0`]
module"]
#[doc(alias = "WKMODn0")]
pub type Wkmodn0 = crate::Reg<wkmodn0::Wkmodn0Spec>;
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)"]
pub mod wkmodn0;
#[doc = "WKMODn1 (rw) register accessor: Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkmodn1`]
module"]
#[doc(alias = "WKMODn1")]
pub type Wkmodn1 = crate::Reg<wkmodn1::Wkmodn1Spec>;
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)"]
pub mod wkmodn1;
#[doc = "WKMODn2 (rw) register accessor: Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkmodn2`]
module"]
#[doc(alias = "WKMODn2")]
pub type Wkmodn2 = crate::Reg<wkmodn2::Wkmodn2Spec>;
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)"]
pub mod wkmodn2;
#[doc = "WKMODn3 (rw) register accessor: Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkmodn3`]
module"]
#[doc(alias = "WKMODn3")]
pub type Wkmodn3 = crate::Reg<wkmodn3::Wkmodn3Spec>;
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)"]
pub mod wkmodn3;
#[doc = "WKMODn4 (rw) register accessor: Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkmodn4`]
module"]
#[doc(alias = "WKMODn4")]
pub type Wkmodn4 = crate::Reg<wkmodn4::Wkmodn4Spec>;
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)"]
pub mod wkmodn4;
#[doc = "WKMODn5 (rw) register accessor: Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkmodn5`]
module"]
#[doc(alias = "WKMODn5")]
pub type Wkmodn5 = crate::Reg<wkmodn5::Wkmodn5Spec>;
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)"]
pub mod wkmodn5;
#[doc = "WKMODn6 (rw) register accessor: Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkmodn6`]
module"]
#[doc(alias = "WKMODn6")]
pub type Wkmodn6 = crate::Reg<wkmodn6::Wkmodn6Spec>;
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)"]
pub mod wkmodn6;
#[doc = "WKMODn7 (rw) register accessor: Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkmodn7`]
module"]
#[doc(alias = "WKMODn7")]
pub type Wkmodn7 = crate::Reg<wkmodn7::Wkmodn7Spec>;
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)"]
pub mod wkmodn7;
#[doc = "WKPNDn0 (rw) register accessor: Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpndn0`]
module"]
#[doc(alias = "WKPNDn0")]
pub type Wkpndn0 = crate::Reg<wkpndn0::Wkpndn0Spec>;
#[doc = "Pending nx Register (WKPNDnx)"]
pub mod wkpndn0;
#[doc = "WKPNDn1 (rw) register accessor: Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpndn1`]
module"]
#[doc(alias = "WKPNDn1")]
pub type Wkpndn1 = crate::Reg<wkpndn1::Wkpndn1Spec>;
#[doc = "Pending nx Register (WKPNDnx)"]
pub mod wkpndn1;
#[doc = "WKPNDn2 (rw) register accessor: Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpndn2`]
module"]
#[doc(alias = "WKPNDn2")]
pub type Wkpndn2 = crate::Reg<wkpndn2::Wkpndn2Spec>;
#[doc = "Pending nx Register (WKPNDnx)"]
pub mod wkpndn2;
#[doc = "WKPNDn3 (rw) register accessor: Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpndn3`]
module"]
#[doc(alias = "WKPNDn3")]
pub type Wkpndn3 = crate::Reg<wkpndn3::Wkpndn3Spec>;
#[doc = "Pending nx Register (WKPNDnx)"]
pub mod wkpndn3;
#[doc = "WKPNDn4 (rw) register accessor: Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpndn4`]
module"]
#[doc(alias = "WKPNDn4")]
pub type Wkpndn4 = crate::Reg<wkpndn4::Wkpndn4Spec>;
#[doc = "Pending nx Register (WKPNDnx)"]
pub mod wkpndn4;
#[doc = "WKPNDn5 (rw) register accessor: Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpndn5`]
module"]
#[doc(alias = "WKPNDn5")]
pub type Wkpndn5 = crate::Reg<wkpndn5::Wkpndn5Spec>;
#[doc = "Pending nx Register (WKPNDnx)"]
pub mod wkpndn5;
#[doc = "WKPNDn6 (rw) register accessor: Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpndn6`]
module"]
#[doc(alias = "WKPNDn6")]
pub type Wkpndn6 = crate::Reg<wkpndn6::Wkpndn6Spec>;
#[doc = "Pending nx Register (WKPNDnx)"]
pub mod wkpndn6;
#[doc = "WKPNDn7 (rw) register accessor: Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpndn7`]
module"]
#[doc(alias = "WKPNDn7")]
pub type Wkpndn7 = crate::Reg<wkpndn7::Wkpndn7Spec>;
#[doc = "Pending nx Register (WKPNDnx)"]
pub mod wkpndn7;
#[doc = "WKPCLn0 (rw) register accessor: Pending Clear nx Register (WKPCLnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcln0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpcln0`]
module"]
#[doc(alias = "WKPCLn0")]
pub type Wkpcln0 = crate::Reg<wkpcln0::Wkpcln0Spec>;
#[doc = "Pending Clear nx Register (WKPCLnx)"]
pub mod wkpcln0;
#[doc = "WKPCLn1 (rw) register accessor: Pending Clear nx Register (WKPCLnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcln1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpcln1`]
module"]
#[doc(alias = "WKPCLn1")]
pub type Wkpcln1 = crate::Reg<wkpcln1::Wkpcln1Spec>;
#[doc = "Pending Clear nx Register (WKPCLnx)"]
pub mod wkpcln1;
#[doc = "WKPCLn2 (rw) register accessor: Pending Clear nx Register (WKPCLnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcln2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpcln2`]
module"]
#[doc(alias = "WKPCLn2")]
pub type Wkpcln2 = crate::Reg<wkpcln2::Wkpcln2Spec>;
#[doc = "Pending Clear nx Register (WKPCLnx)"]
pub mod wkpcln2;
#[doc = "WKPCLn3 (rw) register accessor: Pending Clear nx Register (WKPCLnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcln3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpcln3`]
module"]
#[doc(alias = "WKPCLn3")]
pub type Wkpcln3 = crate::Reg<wkpcln3::Wkpcln3Spec>;
#[doc = "Pending Clear nx Register (WKPCLnx)"]
pub mod wkpcln3;
#[doc = "WKPCLn4 (rw) register accessor: Pending Clear nx Register (WKPCLnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcln4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpcln4`]
module"]
#[doc(alias = "WKPCLn4")]
pub type Wkpcln4 = crate::Reg<wkpcln4::Wkpcln4Spec>;
#[doc = "Pending Clear nx Register (WKPCLnx)"]
pub mod wkpcln4;
#[doc = "WKPCLn5 (rw) register accessor: Pending Clear nx Register (WKPCLnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcln5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpcln5`]
module"]
#[doc(alias = "WKPCLn5")]
pub type Wkpcln5 = crate::Reg<wkpcln5::Wkpcln5Spec>;
#[doc = "Pending Clear nx Register (WKPCLnx)"]
pub mod wkpcln5;
#[doc = "WKPCLn6 (rw) register accessor: Pending Clear nx Register (WKPCLnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcln6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpcln6`]
module"]
#[doc(alias = "WKPCLn6")]
pub type Wkpcln6 = crate::Reg<wkpcln6::Wkpcln6Spec>;
#[doc = "Pending Clear nx Register (WKPCLnx)"]
pub mod wkpcln6;
#[doc = "WKPCLn7 (rw) register accessor: Pending Clear nx Register (WKPCLnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpcln7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpcln7`]
module"]
#[doc(alias = "WKPCLn7")]
pub type Wkpcln7 = crate::Reg<wkpcln7::Wkpcln7Spec>;
#[doc = "Pending Clear nx Register (WKPCLnx)"]
pub mod wkpcln7;
#[doc = "WKENn0 (rw) register accessor: Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkenn0`]
module"]
#[doc(alias = "WKENn0")]
pub type Wkenn0 = crate::Reg<wkenn0::Wkenn0Spec>;
#[doc = "Enable nx Register (WKENnx)"]
pub mod wkenn0;
#[doc = "WKENn1 (rw) register accessor: Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkenn1`]
module"]
#[doc(alias = "WKENn1")]
pub type Wkenn1 = crate::Reg<wkenn1::Wkenn1Spec>;
#[doc = "Enable nx Register (WKENnx)"]
pub mod wkenn1;
#[doc = "WKENn2 (rw) register accessor: Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkenn2`]
module"]
#[doc(alias = "WKENn2")]
pub type Wkenn2 = crate::Reg<wkenn2::Wkenn2Spec>;
#[doc = "Enable nx Register (WKENnx)"]
pub mod wkenn2;
#[doc = "WKENn3 (rw) register accessor: Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkenn3`]
module"]
#[doc(alias = "WKENn3")]
pub type Wkenn3 = crate::Reg<wkenn3::Wkenn3Spec>;
#[doc = "Enable nx Register (WKENnx)"]
pub mod wkenn3;
#[doc = "WKENn4 (rw) register accessor: Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkenn4`]
module"]
#[doc(alias = "WKENn4")]
pub type Wkenn4 = crate::Reg<wkenn4::Wkenn4Spec>;
#[doc = "Enable nx Register (WKENnx)"]
pub mod wkenn4;
#[doc = "WKENn5 (rw) register accessor: Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkenn5`]
module"]
#[doc(alias = "WKENn5")]
pub type Wkenn5 = crate::Reg<wkenn5::Wkenn5Spec>;
#[doc = "Enable nx Register (WKENnx)"]
pub mod wkenn5;
#[doc = "WKENn6 (rw) register accessor: Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkenn6`]
module"]
#[doc(alias = "WKENn6")]
pub type Wkenn6 = crate::Reg<wkenn6::Wkenn6Spec>;
#[doc = "Enable nx Register (WKENnx)"]
pub mod wkenn6;
#[doc = "WKENn7 (rw) register accessor: Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkenn7`]
module"]
#[doc(alias = "WKENn7")]
pub type Wkenn7 = crate::Reg<wkenn7::Wkenn7Spec>;
#[doc = "Enable nx Register (WKENnx)"]
pub mod wkenn7;
#[doc = "WKSTn0 (rw) register accessor: Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkstn0`]
module"]
#[doc(alias = "WKSTn0")]
pub type Wkstn0 = crate::Reg<wkstn0::Wkstn0Spec>;
#[doc = "Wake-Up Status nx Register (WKSTnx)"]
pub mod wkstn0;
#[doc = "WKSTn1 (rw) register accessor: Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkstn1`]
module"]
#[doc(alias = "WKSTn1")]
pub type Wkstn1 = crate::Reg<wkstn1::Wkstn1Spec>;
#[doc = "Wake-Up Status nx Register (WKSTnx)"]
pub mod wkstn1;
#[doc = "WKSTn2 (rw) register accessor: Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkstn2`]
module"]
#[doc(alias = "WKSTn2")]
pub type Wkstn2 = crate::Reg<wkstn2::Wkstn2Spec>;
#[doc = "Wake-Up Status nx Register (WKSTnx)"]
pub mod wkstn2;
#[doc = "WKSTn3 (rw) register accessor: Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkstn3`]
module"]
#[doc(alias = "WKSTn3")]
pub type Wkstn3 = crate::Reg<wkstn3::Wkstn3Spec>;
#[doc = "Wake-Up Status nx Register (WKSTnx)"]
pub mod wkstn3;
#[doc = "WKSTn4 (rw) register accessor: Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkstn4`]
module"]
#[doc(alias = "WKSTn4")]
pub type Wkstn4 = crate::Reg<wkstn4::Wkstn4Spec>;
#[doc = "Wake-Up Status nx Register (WKSTnx)"]
pub mod wkstn4;
#[doc = "WKSTn5 (rw) register accessor: Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkstn5`]
module"]
#[doc(alias = "WKSTn5")]
pub type Wkstn5 = crate::Reg<wkstn5::Wkstn5Spec>;
#[doc = "Wake-Up Status nx Register (WKSTnx)"]
pub mod wkstn5;
#[doc = "WKSTn6 (rw) register accessor: Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkstn6`]
module"]
#[doc(alias = "WKSTn6")]
pub type Wkstn6 = crate::Reg<wkstn6::Wkstn6Spec>;
#[doc = "Wake-Up Status nx Register (WKSTnx)"]
pub mod wkstn6;
#[doc = "WKSTn7 (rw) register accessor: Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkstn7`]
module"]
#[doc(alias = "WKSTn7")]
pub type Wkstn7 = crate::Reg<wkstn7::Wkstn7Spec>;
#[doc = "Wake-Up Status nx Register (WKSTnx)"]
pub mod wkstn7;
#[doc = "WKINENn0 (rw) register accessor: Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkinenn0`]
module"]
#[doc(alias = "WKINENn0")]
pub type Wkinenn0 = crate::Reg<wkinenn0::Wkinenn0Spec>;
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)"]
pub mod wkinenn0;
#[doc = "WKINENn1 (rw) register accessor: Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkinenn1`]
module"]
#[doc(alias = "WKINENn1")]
pub type Wkinenn1 = crate::Reg<wkinenn1::Wkinenn1Spec>;
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)"]
pub mod wkinenn1;
#[doc = "WKINENn2 (rw) register accessor: Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkinenn2`]
module"]
#[doc(alias = "WKINENn2")]
pub type Wkinenn2 = crate::Reg<wkinenn2::Wkinenn2Spec>;
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)"]
pub mod wkinenn2;
#[doc = "WKINENn3 (rw) register accessor: Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkinenn3`]
module"]
#[doc(alias = "WKINENn3")]
pub type Wkinenn3 = crate::Reg<wkinenn3::Wkinenn3Spec>;
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)"]
pub mod wkinenn3;
#[doc = "WKINENn4 (rw) register accessor: Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkinenn4`]
module"]
#[doc(alias = "WKINENn4")]
pub type Wkinenn4 = crate::Reg<wkinenn4::Wkinenn4Spec>;
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)"]
pub mod wkinenn4;
#[doc = "WKINENn5 (rw) register accessor: Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkinenn5`]
module"]
#[doc(alias = "WKINENn5")]
pub type Wkinenn5 = crate::Reg<wkinenn5::Wkinenn5Spec>;
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)"]
pub mod wkinenn5;
#[doc = "WKINENn6 (rw) register accessor: Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkinenn6`]
module"]
#[doc(alias = "WKINENn6")]
pub type Wkinenn6 = crate::Reg<wkinenn6::Wkinenn6Spec>;
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)"]
pub mod wkinenn6;
#[doc = "WKINENn7 (rw) register accessor: Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkinenn7`]
module"]
#[doc(alias = "WKINENn7")]
pub type Wkinenn7 = crate::Reg<wkinenn7::Wkinenn7Spec>;
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)"]
pub mod wkinenn7;
#[doc = "LOW_PND_WUn (rw) register accessor: Lowest Pending Wake-Up Register (LOW_PND_WUn)\n\nYou can [`read`](crate::Reg::read) this register and get [`low_pnd_wun::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low_pnd_wun::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low_pnd_wun`]
module"]
#[doc(alias = "LOW_PND_WUn")]
pub type LowPndWun = crate::Reg<low_pnd_wun::LowPndWunSpec>;
#[doc = "Lowest Pending Wake-Up Register (LOW_PND_WUn)"]
pub mod low_pnd_wun;
