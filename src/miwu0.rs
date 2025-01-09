#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    wkedgn: (),
    _reserved1: [u8; 0x01],
    wkaedgn: (),
    _reserved2: [u8; 0x01],
    wkmodn: (),
    _reserved3: [u8; 0x01],
    wkpndn: (),
    _reserved4: [u8; 0x01],
    wkpcln: (),
    _reserved5: [u8; 0x01],
    wkenn: (),
    _reserved6: [u8; 0x01],
    wkstn: (),
    _reserved7: [u8; 0x01],
    wkinenn: (),
    _reserved8: [u8; 0x07],
    low_pnd_wun: LowPndWun,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - Edge Detection nx Register (WKEDGnx)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `WKEDGn1` register.</div>"]
    #[inline(always)]
    pub const fn wkedgn(&self, n: usize) -> &Wkedgn {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub fn wkedgn_iter(&self) -> impl Iterator<Item = &Wkedgn> {
        (0..8).map(move |n| unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16 * n).cast() })
    }
    #[doc = "0x00 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn1(&self) -> &Wkedgn {
        self.wkedgn(0)
    }
    #[doc = "0x10 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn2(&self) -> &Wkedgn {
        self.wkedgn(1)
    }
    #[doc = "0x20 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn3(&self) -> &Wkedgn {
        self.wkedgn(2)
    }
    #[doc = "0x30 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn4(&self) -> &Wkedgn {
        self.wkedgn(3)
    }
    #[doc = "0x40 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn5(&self) -> &Wkedgn {
        self.wkedgn(4)
    }
    #[doc = "0x50 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn6(&self) -> &Wkedgn {
        self.wkedgn(5)
    }
    #[doc = "0x60 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn7(&self) -> &Wkedgn {
        self.wkedgn(6)
    }
    #[doc = "0x70 - Edge Detection nx Register (WKEDGnx)"]
    #[inline(always)]
    pub const fn wkedgn8(&self) -> &Wkedgn {
        self.wkedgn(7)
    }
    #[doc = "0x01..0x09 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `WKAEDGn1` register.</div>"]
    #[inline(always)]
    pub const fn wkaedgn(&self, n: usize) -> &Wkaedgn {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x01..0x09 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub fn wkaedgn_iter(&self) -> impl Iterator<Item = &Wkaedgn> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(1)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x01 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn1(&self) -> &Wkaedgn {
        self.wkaedgn(0)
    }
    #[doc = "0x11 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn2(&self) -> &Wkaedgn {
        self.wkaedgn(1)
    }
    #[doc = "0x21 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn3(&self) -> &Wkaedgn {
        self.wkaedgn(2)
    }
    #[doc = "0x31 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn4(&self) -> &Wkaedgn {
        self.wkaedgn(3)
    }
    #[doc = "0x41 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn5(&self) -> &Wkaedgn {
        self.wkaedgn(4)
    }
    #[doc = "0x51 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn6(&self) -> &Wkaedgn {
        self.wkaedgn(5)
    }
    #[doc = "0x61 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn7(&self) -> &Wkaedgn {
        self.wkaedgn(6)
    }
    #[doc = "0x71 - Any Edge Detection nx Register (WKAEDGnx)"]
    #[inline(always)]
    pub const fn wkaedgn8(&self) -> &Wkaedgn {
        self.wkaedgn(7)
    }
    #[doc = "0x02..0x0a - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `WKMODn1` register.</div>"]
    #[inline(always)]
    pub const fn wkmodn(&self, n: usize) -> &Wkmodn {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x02..0x0a - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub fn wkmodn_iter(&self) -> impl Iterator<Item = &Wkmodn> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(2)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x02 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn1(&self) -> &Wkmodn {
        self.wkmodn(0)
    }
    #[doc = "0x12 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn2(&self) -> &Wkmodn {
        self.wkmodn(1)
    }
    #[doc = "0x22 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn3(&self) -> &Wkmodn {
        self.wkmodn(2)
    }
    #[doc = "0x32 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn4(&self) -> &Wkmodn {
        self.wkmodn(3)
    }
    #[doc = "0x42 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn5(&self) -> &Wkmodn {
        self.wkmodn(4)
    }
    #[doc = "0x52 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn6(&self) -> &Wkmodn {
        self.wkmodn(5)
    }
    #[doc = "0x62 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn7(&self) -> &Wkmodn {
        self.wkmodn(6)
    }
    #[doc = "0x72 - Wake-Up Detection Mode nx Register (WKMODnx)"]
    #[inline(always)]
    pub const fn wkmodn8(&self) -> &Wkmodn {
        self.wkmodn(7)
    }
    #[doc = "0x03..0x0b - Pending nx Register (WKPNDnx)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `WKPNDn1` register.</div>"]
    #[inline(always)]
    pub const fn wkpndn(&self, n: usize) -> &Wkpndn {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(3)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x03..0x0b - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub fn wkpndn_iter(&self) -> impl Iterator<Item = &Wkpndn> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(3)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x03 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn1(&self) -> &Wkpndn {
        self.wkpndn(0)
    }
    #[doc = "0x13 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn2(&self) -> &Wkpndn {
        self.wkpndn(1)
    }
    #[doc = "0x23 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn3(&self) -> &Wkpndn {
        self.wkpndn(2)
    }
    #[doc = "0x33 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn4(&self) -> &Wkpndn {
        self.wkpndn(3)
    }
    #[doc = "0x43 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn5(&self) -> &Wkpndn {
        self.wkpndn(4)
    }
    #[doc = "0x53 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn6(&self) -> &Wkpndn {
        self.wkpndn(5)
    }
    #[doc = "0x63 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn7(&self) -> &Wkpndn {
        self.wkpndn(6)
    }
    #[doc = "0x73 - Pending nx Register (WKPNDnx)"]
    #[inline(always)]
    pub const fn wkpndn8(&self) -> &Wkpndn {
        self.wkpndn(7)
    }
    #[doc = "0x04..0x0c - Pending Clear nx Register (WKPCLnx)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `WKPCLn1` register.</div>"]
    #[inline(always)]
    pub const fn wkpcln(&self, n: usize) -> &Wkpcln {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x0c - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub fn wkpcln_iter(&self) -> impl Iterator<Item = &Wkpcln> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(4)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x04 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln1(&self) -> &Wkpcln {
        self.wkpcln(0)
    }
    #[doc = "0x14 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln2(&self) -> &Wkpcln {
        self.wkpcln(1)
    }
    #[doc = "0x24 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln3(&self) -> &Wkpcln {
        self.wkpcln(2)
    }
    #[doc = "0x34 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln4(&self) -> &Wkpcln {
        self.wkpcln(3)
    }
    #[doc = "0x44 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln5(&self) -> &Wkpcln {
        self.wkpcln(4)
    }
    #[doc = "0x54 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln6(&self) -> &Wkpcln {
        self.wkpcln(5)
    }
    #[doc = "0x64 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln7(&self) -> &Wkpcln {
        self.wkpcln(6)
    }
    #[doc = "0x74 - Pending Clear nx Register (WKPCLnx)"]
    #[inline(always)]
    pub const fn wkpcln8(&self) -> &Wkpcln {
        self.wkpcln(7)
    }
    #[doc = "0x05..0x0d - Enable nx Register (WKENnx)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `WKENn1` register.</div>"]
    #[inline(always)]
    pub const fn wkenn(&self, n: usize) -> &Wkenn {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x05..0x0d - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub fn wkenn_iter(&self) -> impl Iterator<Item = &Wkenn> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(5)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x05 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn1(&self) -> &Wkenn {
        self.wkenn(0)
    }
    #[doc = "0x15 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn2(&self) -> &Wkenn {
        self.wkenn(1)
    }
    #[doc = "0x25 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn3(&self) -> &Wkenn {
        self.wkenn(2)
    }
    #[doc = "0x35 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn4(&self) -> &Wkenn {
        self.wkenn(3)
    }
    #[doc = "0x45 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn5(&self) -> &Wkenn {
        self.wkenn(4)
    }
    #[doc = "0x55 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn6(&self) -> &Wkenn {
        self.wkenn(5)
    }
    #[doc = "0x65 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn7(&self) -> &Wkenn {
        self.wkenn(6)
    }
    #[doc = "0x75 - Enable nx Register (WKENnx)"]
    #[inline(always)]
    pub const fn wkenn8(&self) -> &Wkenn {
        self.wkenn(7)
    }
    #[doc = "0x06..0x0e - Wake-Up Status nx Register (WKSTnx)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `WKSTn1` register.</div>"]
    #[inline(always)]
    pub const fn wkstn(&self, n: usize) -> &Wkstn {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x06..0x0e - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub fn wkstn_iter(&self) -> impl Iterator<Item = &Wkstn> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(6)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x06 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn1(&self) -> &Wkstn {
        self.wkstn(0)
    }
    #[doc = "0x16 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn2(&self) -> &Wkstn {
        self.wkstn(1)
    }
    #[doc = "0x26 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn3(&self) -> &Wkstn {
        self.wkstn(2)
    }
    #[doc = "0x36 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn4(&self) -> &Wkstn {
        self.wkstn(3)
    }
    #[doc = "0x46 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn5(&self) -> &Wkstn {
        self.wkstn(4)
    }
    #[doc = "0x56 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn6(&self) -> &Wkstn {
        self.wkstn(5)
    }
    #[doc = "0x66 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn7(&self) -> &Wkstn {
        self.wkstn(6)
    }
    #[doc = "0x76 - Wake-Up Status nx Register (WKSTnx)"]
    #[inline(always)]
    pub const fn wkstn8(&self) -> &Wkstn {
        self.wkstn(7)
    }
    #[doc = "0x07..0x0f - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `WKINENn1` register.</div>"]
    #[inline(always)]
    pub const fn wkinenn(&self, n: usize) -> &Wkinenn {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(7)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x07..0x0f - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub fn wkinenn_iter(&self) -> impl Iterator<Item = &Wkinenn> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(7)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x07 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn1(&self) -> &Wkinenn {
        self.wkinenn(0)
    }
    #[doc = "0x17 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn2(&self) -> &Wkinenn {
        self.wkinenn(1)
    }
    #[doc = "0x27 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn3(&self) -> &Wkinenn {
        self.wkinenn(2)
    }
    #[doc = "0x37 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn4(&self) -> &Wkinenn {
        self.wkinenn(3)
    }
    #[doc = "0x47 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn5(&self) -> &Wkinenn {
        self.wkinenn(4)
    }
    #[doc = "0x57 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn6(&self) -> &Wkinenn {
        self.wkinenn(5)
    }
    #[doc = "0x67 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn7(&self) -> &Wkinenn {
        self.wkinenn(6)
    }
    #[doc = "0x77 - Wake-Up Input Enable nx Register (WKINENnx)"]
    #[inline(always)]
    pub const fn wkinenn8(&self) -> &Wkinenn {
        self.wkinenn(7)
    }
    #[doc = "0x0e - Lowest Pending Wake-Up Register (LOW_PND_WUn)"]
    #[inline(always)]
    pub const fn low_pnd_wun(&self) -> &LowPndWun {
        &self.low_pnd_wun
    }
}
#[doc = "WKEDGn (rw) register accessor: Edge Detection nx Register (WKEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkedgn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkedgn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkedgn`]
module"]
#[doc(alias = "WKEDGn")]
pub type Wkedgn = crate::Reg<wkedgn::WkedgnSpec>;
#[doc = "Edge Detection nx Register (WKEDGnx)"]
pub mod wkedgn;
#[doc = "WKAEDGn (rw) register accessor: Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkaedgn`]
module"]
#[doc(alias = "WKAEDGn")]
pub type Wkaedgn = crate::Reg<wkaedgn::WkaedgnSpec>;
#[doc = "Any Edge Detection nx Register (WKAEDGnx)"]
pub mod wkaedgn;
#[doc = "WKMODn (rw) register accessor: Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkmodn`]
module"]
#[doc(alias = "WKMODn")]
pub type Wkmodn = crate::Reg<wkmodn::WkmodnSpec>;
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)"]
pub mod wkmodn;
#[doc = "WKPNDn (rw) register accessor: Pending nx Register (WKPNDnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkpndn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpndn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpndn`]
module"]
#[doc(alias = "WKPNDn")]
pub type Wkpndn = crate::Reg<wkpndn::WkpndnSpec>;
#[doc = "Pending nx Register (WKPNDnx)"]
pub mod wkpndn;
#[doc = "WKPCLn (w) register accessor: Pending Clear nx Register (WKPCLnx)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkpcln`]
module"]
#[doc(alias = "WKPCLn")]
pub type Wkpcln = crate::Reg<wkpcln::WkpclnSpec>;
#[doc = "Pending Clear nx Register (WKPCLnx)"]
pub mod wkpcln;
#[doc = "WKENn (rw) register accessor: Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkenn`]
module"]
#[doc(alias = "WKENn")]
pub type Wkenn = crate::Reg<wkenn::WkennSpec>;
#[doc = "Enable nx Register (WKENnx)"]
pub mod wkenn;
#[doc = "WKSTn (r) register accessor: Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkstn`]
module"]
#[doc(alias = "WKSTn")]
pub type Wkstn = crate::Reg<wkstn::WkstnSpec>;
#[doc = "Wake-Up Status nx Register (WKSTnx)"]
pub mod wkstn;
#[doc = "WKINENn (rw) register accessor: Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkinenn`]
module"]
#[doc(alias = "WKINENn")]
pub type Wkinenn = crate::Reg<wkinenn::WkinennSpec>;
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)"]
pub mod wkinenn;
#[doc = "LOW_PND_WUn (rw) register accessor: Lowest Pending Wake-Up Register (LOW_PND_WUn)\n\nYou can [`read`](crate::Reg::read) this register and get [`low_pnd_wun::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`low_pnd_wun::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low_pnd_wun`]
module"]
#[doc(alias = "LOW_PND_WUn")]
pub type LowPndWun = crate::Reg<low_pnd_wun::LowPndWunSpec>;
#[doc = "Lowest Pending Wake-Up Register (LOW_PND_WUn)"]
pub mod low_pnd_wun;
