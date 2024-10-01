#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    mconfig: Mconfig,
    _reserved1: [u8; 0x80],
    mctrl: Mctrl,
    mstatus: Mstatus,
    ibirules: Ibirules,
    mintset: Mintset,
    mintclr: Mintclr,
    mintmasked: Mintmasked,
    merrwarn: Merrwarn,
    mdmactrl: Mdmactrl,
    _reserved9: [u8; 0x08],
    mdatactrl: Mdatactrl,
    _reserved10: [u8; 0x04],
    mwdatabe: Mwdatabe,
    mwdatah: Mwdatah,
    mwdatahe: Mwdatahe,
    mrdatab: Mrdatab,
    _reserved14: [u8; 0x04],
    _reserved_14_mrdatah: [u8; 0x04],
    _reserved15: [u8; 0x0c],
    mwmsg_ddr: MwmsgDdr,
    mrmsg_ddr: MrmsgDdr,
}
impl RegisterBlock {
    #[doc = "0x00 - Controller Configuration"]
    #[inline(always)]
    pub const fn mconfig(&self) -> &Mconfig {
        &self.mconfig
    }
    #[doc = "0x84 - Controller Control Register"]
    #[inline(always)]
    pub const fn mctrl(&self) -> &Mctrl {
        &self.mctrl
    }
    #[doc = "0x88 - Controller Status Register"]
    #[inline(always)]
    pub const fn mstatus(&self) -> &Mstatus {
        &self.mstatus
    }
    #[doc = "0x8c - IBI Registry and Rules Register"]
    #[inline(always)]
    pub const fn ibirules(&self) -> &Ibirules {
        &self.ibirules
    }
    #[doc = "0x90 - Controller Interrupt Enable Set Register"]
    #[inline(always)]
    pub const fn mintset(&self) -> &Mintset {
        &self.mintset
    }
    #[doc = "0x94 - Controller Interrupt Enable Clear Register"]
    #[inline(always)]
    pub const fn mintclr(&self) -> &Mintclr {
        &self.mintclr
    }
    #[doc = "0x98 - Controller Interrupt Masked Register"]
    #[inline(always)]
    pub const fn mintmasked(&self) -> &Mintmasked {
        &self.mintmasked
    }
    #[doc = "0x9c - Controller Error and Warning Register"]
    #[inline(always)]
    pub const fn merrwarn(&self) -> &Merrwarn {
        &self.merrwarn
    }
    #[doc = "0xa0 - Controller DMA Control Register"]
    #[inline(always)]
    pub const fn mdmactrl(&self) -> &Mdmactrl {
        &self.mdmactrl
    }
    #[doc = "0xac - Controller Data Control Register"]
    #[inline(always)]
    pub const fn mdatactrl(&self) -> &Mdatactrl {
        &self.mdatactrl
    }
    #[doc = "0xb4 - Controller Write Byte Data as End Register"]
    #[inline(always)]
    pub const fn mwdatabe(&self) -> &Mwdatabe {
        &self.mwdatabe
    }
    #[doc = "0xb8 - Controller Write Half-Word Data Register"]
    #[inline(always)]
    pub const fn mwdatah(&self) -> &Mwdatah {
        &self.mwdatah
    }
    #[doc = "0xbc - Controller Write Half-Word Data as End Register"]
    #[inline(always)]
    pub const fn mwdatahe(&self) -> &Mwdatahe {
        &self.mwdatahe
    }
    #[doc = "0xc0 - Controller Read Byte Data Register"]
    #[inline(always)]
    pub const fn mrdatab(&self) -> &Mrdatab {
        &self.mrdatab
    }
    #[doc = "0xc8 - Controller Dynamic Address Register"]
    #[inline(always)]
    pub const fn mdynaddr(&self) -> &Mdynaddr {
        unsafe { &*(self as *const Self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xc8 - Controller Read Half-Word Data Register"]
    #[inline(always)]
    pub const fn mrdatah(&self) -> &Mrdatah {
        unsafe { &*(self as *const Self).cast::<u8>().add(200).cast() }
    }
    #[doc = "0xd8 - Start or Continue DDR Message Register"]
    #[inline(always)]
    pub const fn mwmsg_ddr(&self) -> &MwmsgDdr {
        &self.mwmsg_ddr
    }
    #[doc = "0xdc - Read DDR Message Register"]
    #[inline(always)]
    pub const fn mrmsg_ddr(&self) -> &MrmsgDdr {
        &self.mrmsg_ddr
    }
}
#[doc = "MCONFIG (rw) register accessor: Controller Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mconfig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mconfig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mconfig`]
module"]
#[doc(alias = "MCONFIG")]
pub type Mconfig = crate::Reg<mconfig::MconfigSpec>;
#[doc = "Controller Configuration"]
pub mod mconfig;
#[doc = "MCTRL (rw) register accessor: Controller Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctrl`]
module"]
#[doc(alias = "MCTRL")]
pub type Mctrl = crate::Reg<mctrl::MctrlSpec>;
#[doc = "Controller Control Register"]
pub mod mctrl;
#[doc = "MSTATUS (rw) register accessor: Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mstatus`]
module"]
#[doc(alias = "MSTATUS")]
pub type Mstatus = crate::Reg<mstatus::MstatusSpec>;
#[doc = "Controller Status Register"]
pub mod mstatus;
#[doc = "IBIRULES (rw) register accessor: IBI Registry and Rules Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ibirules::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibirules::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibirules`]
module"]
#[doc(alias = "IBIRULES")]
pub type Ibirules = crate::Reg<ibirules::IbirulesSpec>;
#[doc = "IBI Registry and Rules Register"]
pub mod ibirules;
#[doc = "MINTSET (rw) register accessor: Controller Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mintset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mintset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mintset`]
module"]
#[doc(alias = "MINTSET")]
pub type Mintset = crate::Reg<mintset::MintsetSpec>;
#[doc = "Controller Interrupt Enable Set Register"]
pub mod mintset;
#[doc = "MINTCLR (rw) register accessor: Controller Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mintclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mintclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mintclr`]
module"]
#[doc(alias = "MINTCLR")]
pub type Mintclr = crate::Reg<mintclr::MintclrSpec>;
#[doc = "Controller Interrupt Enable Clear Register"]
pub mod mintclr;
#[doc = "MINTMASKED (rw) register accessor: Controller Interrupt Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mintmasked::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mintmasked::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mintmasked`]
module"]
#[doc(alias = "MINTMASKED")]
pub type Mintmasked = crate::Reg<mintmasked::MintmaskedSpec>;
#[doc = "Controller Interrupt Masked Register"]
pub mod mintmasked;
#[doc = "MERRWARN (rw) register accessor: Controller Error and Warning Register\n\nYou can [`read`](crate::Reg::read) this register and get [`merrwarn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`merrwarn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@merrwarn`]
module"]
#[doc(alias = "MERRWARN")]
pub type Merrwarn = crate::Reg<merrwarn::MerrwarnSpec>;
#[doc = "Controller Error and Warning Register"]
pub mod merrwarn;
#[doc = "MDMACTRL (rw) register accessor: Controller DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdmactrl`]
module"]
#[doc(alias = "MDMACTRL")]
pub type Mdmactrl = crate::Reg<mdmactrl::MdmactrlSpec>;
#[doc = "Controller DMA Control Register"]
pub mod mdmactrl;
#[doc = "MDATACTRL (rw) register accessor: Controller Data Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdatactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdatactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdatactrl`]
module"]
#[doc(alias = "MDATACTRL")]
pub type Mdatactrl = crate::Reg<mdatactrl::MdatactrlSpec>;
#[doc = "Controller Data Control Register"]
pub mod mdatactrl;
#[doc = "MWDATABE (rw) register accessor: Controller Write Byte Data as End Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mwdatabe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatabe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwdatabe`]
module"]
#[doc(alias = "MWDATABE")]
pub type Mwdatabe = crate::Reg<mwdatabe::MwdatabeSpec>;
#[doc = "Controller Write Byte Data as End Register"]
pub mod mwdatabe;
#[doc = "MWDATAH (rw) register accessor: Controller Write Half-Word Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mwdatah::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatah::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwdatah`]
module"]
#[doc(alias = "MWDATAH")]
pub type Mwdatah = crate::Reg<mwdatah::MwdatahSpec>;
#[doc = "Controller Write Half-Word Data Register"]
pub mod mwdatah;
#[doc = "MWDATAHE (rw) register accessor: Controller Write Half-Word Data as End Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mwdatahe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwdatahe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwdatahe`]
module"]
#[doc(alias = "MWDATAHE")]
pub type Mwdatahe = crate::Reg<mwdatahe::MwdataheSpec>;
#[doc = "Controller Write Half-Word Data as End Register"]
pub mod mwdatahe;
#[doc = "MRDATAB (rw) register accessor: Controller Read Byte Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdatab::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrdatab::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrdatab`]
module"]
#[doc(alias = "MRDATAB")]
pub type Mrdatab = crate::Reg<mrdatab::MrdatabSpec>;
#[doc = "Controller Read Byte Data Register"]
pub mod mrdatab;
#[doc = "MRDATAH (rw) register accessor: Controller Read Half-Word Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdatah::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrdatah::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrdatah`]
module"]
#[doc(alias = "MRDATAH")]
pub type Mrdatah = crate::Reg<mrdatah::MrdatahSpec>;
#[doc = "Controller Read Half-Word Data Register"]
pub mod mrdatah;
#[doc = "MDYNADDR (rw) register accessor: Controller Dynamic Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdynaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdynaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdynaddr`]
module"]
#[doc(alias = "MDYNADDR")]
pub type Mdynaddr = crate::Reg<mdynaddr::MdynaddrSpec>;
#[doc = "Controller Dynamic Address Register"]
pub mod mdynaddr;
#[doc = "MWMSG_DDR (rw) register accessor: Start or Continue DDR Message Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mwmsg_ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwmsg_ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwmsg_ddr`]
module"]
#[doc(alias = "MWMSG_DDR")]
pub type MwmsgDdr = crate::Reg<mwmsg_ddr::MwmsgDdrSpec>;
#[doc = "Start or Continue DDR Message Register"]
pub mod mwmsg_ddr;
#[doc = "MRMSG_DDR (rw) register accessor: Read DDR Message Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mrmsg_ddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrmsg_ddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrmsg_ddr`]
module"]
#[doc(alias = "MRMSG_DDR")]
pub type MrmsgDdr = crate::Reg<mrmsg_ddr::MrmsgDdrSpec>;
#[doc = "Read DDR Message Register"]
pub mod mrmsg_ddr;
