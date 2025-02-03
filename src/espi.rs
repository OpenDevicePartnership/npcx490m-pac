#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    espiid: Espiid,
    espicfg: Espicfg,
    espists: Espists,
    espiie: Espiie,
    espiwe: Espiwe,
    vwregidx: Vwregidx,
    vwregdata: Vwregdata,
    oobrxrdhead: Oobrxrdhead,
    oobtxwrhead: Oobtxwrhead,
    oobctl: Oobctl,
    flashrxrdhead: Flashrxrdhead,
    flashtxwrhead: Flashtxwrhead,
    _reserved12: [u8; 0x04],
    flashcfg: Flashcfg,
    flashctl: Flashctl,
    espierr: Espierr,
    pbmrxrdhead: Pbmrxrdhead,
    pbmtxwrhead: Pbmtxwrhead,
    percfg: Percfg,
    perctl: Perctl,
    status_img: StatusImg,
    _reserved20: [u8; 0x02],
    perctlbw: Perctlbw,
    psmrxrdhead: Psmrxrdhead,
    _reserved22: [u8; 0xa4],
    vwevsm: [Vwevsm; 10],
    _reserved23: [u8; 0x14],
    vwswirq: Vwswirq,
    vwevms: [Vwevms; 12],
    _reserved25: [u8; 0x10],
    vwgpsm: [Vwgpsm; 16],
    vwgpms: [Vwgpms; 16],
    _reserved27: [u8; 0xfc],
    vwctl: Vwctl,
    oobrxbuf: [Oobrxbuf; 20],
    _reserved29: [u8; 0x30],
    oobtxbuf: [Oobtxbuf; 20],
    _reserved30: [u8; 0x30],
    flashrxbuf: [Flashrxbuf; 18],
    _reserved31: [u8; 0x38],
    flashtxbuf: [Flashtxbuf; 18],
    _reserved32: [u8; 0x38],
    pbmrxbuf: [Pbmrxbuf; 18],
    _reserved33: [u8; 0x38],
    pbmtxbuf: [Pbmtxbuf; 18],
    _reserved34: [u8; 0x38],
    flash_prtr_baddr: [FlashPrtrBaddr; 16],
    flash_prtr_taddr: [FlashPrtrTaddr; 16],
    flash_rng_tag_ovr: [FlashRngTagOvr; 16],
    _reserved37: [u8; 0x0140],
    flash_rpmc_cfg_1: FlashRpmcCfg1,
    flash_rpmc_cfg_2: FlashRpmcCfg2,
    rmap_flash_offs: RmapFlashOffs,
    rmap_dst_base: RmapDstBase,
    rmap_win_size: RmapWinSize,
    flashbase: Flashbase,
    flrlck: Flrlck,
    _reserved44: [u8; 0xe4],
    psmrxbuf: [Psmrxbuf; 18],
}
impl RegisterBlock {
    #[doc = "0x00 - eSPI Identification Register (ESPIID)"]
    #[inline(always)]
    pub const fn espiid(&self) -> &Espiid {
        &self.espiid
    }
    #[doc = "0x04 - eSPI Configuration Register (ESPICFG)"]
    #[inline(always)]
    pub const fn espicfg(&self) -> &Espicfg {
        &self.espicfg
    }
    #[doc = "0x08 - eSPI Status Register (ESPISTS)"]
    #[inline(always)]
    pub const fn espists(&self) -> &Espists {
        &self.espists
    }
    #[doc = "0x0c - eSPI Interrupt Enable Register (ESPIIE)"]
    #[inline(always)]
    pub const fn espiie(&self) -> &Espiie {
        &self.espiie
    }
    #[doc = "0x10 - eSPI Wake-Up Enable Register (ESPIWE)"]
    #[inline(always)]
    pub const fn espiwe(&self) -> &Espiwe {
        &self.espiwe
    }
    #[doc = "0x14 - Virtual Wire Register Index Register (VWREGIDX)"]
    #[inline(always)]
    pub const fn vwregidx(&self) -> &Vwregidx {
        &self.vwregidx
    }
    #[doc = "0x18 - Virtual Wire Register Data Register (VWREGDATA)"]
    #[inline(always)]
    pub const fn vwregdata(&self) -> &Vwregdata {
        &self.vwregdata
    }
    #[doc = "0x1c - OOB Receive Buffer Read Head Register (OOBRXRDHEAD)"]
    #[inline(always)]
    pub const fn oobrxrdhead(&self) -> &Oobrxrdhead {
        &self.oobrxrdhead
    }
    #[doc = "0x20 - OOB Transmit Buffer Write Head Register (OOBTXWRHEAD)"]
    #[inline(always)]
    pub const fn oobtxwrhead(&self) -> &Oobtxwrhead {
        &self.oobtxwrhead
    }
    #[doc = "0x24 - OOB Channel Control Register (OOBCTL)"]
    #[inline(always)]
    pub const fn oobctl(&self) -> &Oobctl {
        &self.oobctl
    }
    #[doc = "0x28 - Flash Receive Buffer Read Head Register (FLASHRXRDHEAD)"]
    #[inline(always)]
    pub const fn flashrxrdhead(&self) -> &Flashrxrdhead {
        &self.flashrxrdhead
    }
    #[doc = "0x2c - Flash Transmit Buffer Write Head Register (FLASHTXWRHEAD)"]
    #[inline(always)]
    pub const fn flashtxwrhead(&self) -> &Flashtxwrhead {
        &self.flashtxwrhead
    }
    #[doc = "0x34 - Flash Channel Configuration Register (FLASHCFG)"]
    #[inline(always)]
    pub const fn flashcfg(&self) -> &Flashcfg {
        &self.flashcfg
    }
    #[doc = "0x38 - Flash Channel Control Register (FLASHCTL)"]
    #[inline(always)]
    pub const fn flashctl(&self) -> &Flashctl {
        &self.flashctl
    }
    #[doc = "0x3c - eSPI Error Status Register (ESPIERR)"]
    #[inline(always)]
    pub const fn espierr(&self) -> &Espierr {
        &self.espierr
    }
    #[doc = "0x40 - Peripheral Bus Master Receive Buffer Read Head Register (PBMRXRDHEAD)"]
    #[inline(always)]
    pub const fn pbmrxrdhead(&self) -> &Pbmrxrdhead {
        &self.pbmrxrdhead
    }
    #[doc = "0x44 - Peripheral Bus Master Transmit Buffer Write Head Register (PBMTXWRHEAD)"]
    #[inline(always)]
    pub const fn pbmtxwrhead(&self) -> &Pbmtxwrhead {
        &self.pbmtxwrhead
    }
    #[doc = "0x48 - Peripheral Channel Configuration Register (PERCFG)"]
    #[inline(always)]
    pub const fn percfg(&self) -> &Percfg {
        &self.percfg
    }
    #[doc = "0x4c - Peripheral Channel Control Register (PERCTL)"]
    #[inline(always)]
    pub const fn perctl(&self) -> &Perctl {
        &self.perctl
    }
    #[doc = "0x50 - Status Image Register (STATUS_IMG)"]
    #[inline(always)]
    pub const fn status_img(&self) -> &StatusImg {
        &self.status_img
    }
    #[doc = "0x54 - Peripheral Channel Control for Bus Master Write Register (PERCTLBW)"]
    #[inline(always)]
    pub const fn perctlbw(&self) -> &Perctlbw {
        &self.perctlbw
    }
    #[doc = "0x58 - Peripheral Target Message Receive Buffer Read Head Register (PSMRXRDHEAD)"]
    #[inline(always)]
    pub const fn psmrxrdhead(&self) -> &Psmrxrdhead {
        &self.psmrxrdhead
    }
    #[doc = "0x100..0x128 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub const fn vwevsm(&self, n: usize) -> &Vwevsm {
        &self.vwevsm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x128 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub fn vwevsm_iter(&self) -> impl Iterator<Item = &Vwevsm> {
        self.vwevsm.iter()
    }
    #[doc = "0x13c - Virtual Wire Software IRQ Register (VWSWIRQ)"]
    #[inline(always)]
    pub const fn vwswirq(&self) -> &Vwswirq {
        &self.vwswirq
    }
    #[doc = "0x140..0x170 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms(&self, n: usize) -> &Vwevms {
        &self.vwevms[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x140..0x170 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub fn vwevms_iter(&self) -> impl Iterator<Item = &Vwevms> {
        self.vwevms.iter()
    }
    #[doc = "0x180..0x1c0 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm(&self, n: usize) -> &Vwgpsm {
        &self.vwgpsm[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x180..0x1c0 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub fn vwgpsm_iter(&self) -> impl Iterator<Item = &Vwgpsm> {
        self.vwgpsm.iter()
    }
    #[doc = "0x1c0..0x200 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms(&self, n: usize) -> &Vwgpms {
        &self.vwgpms[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1c0..0x200 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub fn vwgpms_iter(&self) -> impl Iterator<Item = &Vwgpms> {
        self.vwgpms.iter()
    }
    #[doc = "0x2fc - Virtual Wire Channel Control Register (VWCTL)"]
    #[inline(always)]
    pub const fn vwctl(&self) -> &Vwctl {
        &self.vwctl
    }
    #[doc = "0x300..0x350 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf(&self, n: usize) -> &Oobrxbuf {
        &self.oobrxbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x350 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub fn oobrxbuf_iter(&self) -> impl Iterator<Item = &Oobrxbuf> {
        self.oobrxbuf.iter()
    }
    #[doc = "0x380..0x3d0 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf(&self, n: usize) -> &Oobtxbuf {
        &self.oobtxbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x380..0x3d0 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub fn oobtxbuf_iter(&self) -> impl Iterator<Item = &Oobtxbuf> {
        self.oobtxbuf.iter()
    }
    #[doc = "0x400..0x448 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf(&self, n: usize) -> &Flashrxbuf {
        &self.flashrxbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x448 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub fn flashrxbuf_iter(&self) -> impl Iterator<Item = &Flashrxbuf> {
        self.flashrxbuf.iter()
    }
    #[doc = "0x480..0x4c8 - Flash transmit buffer"]
    #[inline(always)]
    pub const fn flashtxbuf(&self, n: usize) -> &Flashtxbuf {
        &self.flashtxbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x480..0x4c8 - Flash transmit buffer"]
    #[inline(always)]
    pub fn flashtxbuf_iter(&self) -> impl Iterator<Item = &Flashtxbuf> {
        self.flashtxbuf.iter()
    }
    #[doc = "0x500..0x548 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf(&self, n: usize) -> &Pbmrxbuf {
        &self.pbmrxbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x548 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub fn pbmrxbuf_iter(&self) -> impl Iterator<Item = &Pbmrxbuf> {
        self.pbmrxbuf.iter()
    }
    #[doc = "0x580..0x5c8 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf(&self, n: usize) -> &Pbmtxbuf {
        &self.pbmtxbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x580..0x5c8 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub fn pbmtxbuf_iter(&self) -> impl Iterator<Item = &Pbmtxbuf> {
        self.pbmtxbuf.iter()
    }
    #[doc = "0x600..0x640 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr(&self, n: usize) -> &FlashPrtrBaddr {
        &self.flash_prtr_baddr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x640 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub fn flash_prtr_baddr_iter(&self) -> impl Iterator<Item = &FlashPrtrBaddr> {
        self.flash_prtr_baddr.iter()
    }
    #[doc = "0x640..0x680 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr(&self, n: usize) -> &FlashPrtrTaddr {
        &self.flash_prtr_taddr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x640..0x680 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub fn flash_prtr_taddr_iter(&self) -> impl Iterator<Item = &FlashPrtrTaddr> {
        self.flash_prtr_taddr.iter()
    }
    #[doc = "0x680..0x6c0 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr(&self, n: usize) -> &FlashRngTagOvr {
        &self.flash_rng_tag_ovr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x680..0x6c0 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub fn flash_rng_tag_ovr_iter(&self) -> impl Iterator<Item = &FlashRngTagOvr> {
        self.flash_rng_tag_ovr.iter()
    }
    #[doc = "0x800 - Flash RPMC Configuration 1 Register (FLASH_RPMC_CFG_1)"]
    #[inline(always)]
    pub const fn flash_rpmc_cfg_1(&self) -> &FlashRpmcCfg1 {
        &self.flash_rpmc_cfg_1
    }
    #[doc = "0x804 - Flash RPMC Configuration Register 2 (FLASH_RPMC_CFG_2)"]
    #[inline(always)]
    pub const fn flash_rpmc_cfg_2(&self) -> &FlashRpmcCfg2 {
        &self.flash_rpmc_cfg_2
    }
    #[doc = "0x808 - Remapping Flash Offset Register (RMAP_FLASH_OFFS)"]
    #[inline(always)]
    pub const fn rmap_flash_offs(&self) -> &RmapFlashOffs {
        &self.rmap_flash_offs
    }
    #[doc = "0x80c - Remapping Destination Base Register (RMAP_DST_BASE)"]
    #[inline(always)]
    pub const fn rmap_dst_base(&self) -> &RmapDstBase {
        &self.rmap_dst_base
    }
    #[doc = "0x810 - Remapping Window Size Register (RMAP_WIN_SIZE)"]
    #[inline(always)]
    pub const fn rmap_win_size(&self) -> &RmapWinSize {
        &self.rmap_win_size
    }
    #[doc = "0x814 - Flash Base Register (FLASHBASE)"]
    #[inline(always)]
    pub const fn flashbase(&self) -> &Flashbase {
        &self.flashbase
    }
    #[doc = "0x818 - Flash Reversible Lock Register (FLRLCK)"]
    #[inline(always)]
    pub const fn flrlck(&self) -> &Flrlck {
        &self.flrlck
    }
    #[doc = "0x900..0x948 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf(&self, n: usize) -> &Psmrxbuf {
        &self.psmrxbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x900..0x948 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub fn psmrxbuf_iter(&self) -> impl Iterator<Item = &Psmrxbuf> {
        self.psmrxbuf.iter()
    }
}
#[doc = "ESPIID (rw) register accessor: eSPI Identification Register (ESPIID)\n\nYou can [`read`](crate::Reg::read) this register and get [`espiid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espiid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espiid`]
module"]
#[doc(alias = "ESPIID")]
pub type Espiid = crate::Reg<espiid::EspiidSpec>;
#[doc = "eSPI Identification Register (ESPIID)"]
pub mod espiid;
#[doc = "ESPICFG (rw) register accessor: eSPI Configuration Register (ESPICFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`espicfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espicfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espicfg`]
module"]
#[doc(alias = "ESPICFG")]
pub type Espicfg = crate::Reg<espicfg::EspicfgSpec>;
#[doc = "eSPI Configuration Register (ESPICFG)"]
pub mod espicfg;
#[doc = "ESPISTS (rw) register accessor: eSPI Status Register (ESPISTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`espists::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espists::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espists`]
module"]
#[doc(alias = "ESPISTS")]
pub type Espists = crate::Reg<espists::EspistsSpec>;
#[doc = "eSPI Status Register (ESPISTS)"]
pub mod espists;
#[doc = "ESPIIE (rw) register accessor: eSPI Interrupt Enable Register (ESPIIE)\n\nYou can [`read`](crate::Reg::read) this register and get [`espiie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espiie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espiie`]
module"]
#[doc(alias = "ESPIIE")]
pub type Espiie = crate::Reg<espiie::EspiieSpec>;
#[doc = "eSPI Interrupt Enable Register (ESPIIE)"]
pub mod espiie;
#[doc = "ESPIWE (rw) register accessor: eSPI Wake-Up Enable Register (ESPIWE)\n\nYou can [`read`](crate::Reg::read) this register and get [`espiwe::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espiwe::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espiwe`]
module"]
#[doc(alias = "ESPIWE")]
pub type Espiwe = crate::Reg<espiwe::EspiweSpec>;
#[doc = "eSPI Wake-Up Enable Register (ESPIWE)"]
pub mod espiwe;
#[doc = "VWREGIDX (rw) register accessor: Virtual Wire Register Index Register (VWREGIDX)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwregidx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwregidx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwregidx`]
module"]
#[doc(alias = "VWREGIDX")]
pub type Vwregidx = crate::Reg<vwregidx::VwregidxSpec>;
#[doc = "Virtual Wire Register Index Register (VWREGIDX)"]
pub mod vwregidx;
#[doc = "VWREGDATA (rw) register accessor: Virtual Wire Register Data Register (VWREGDATA)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwregdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwregdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwregdata`]
module"]
#[doc(alias = "VWREGDATA")]
pub type Vwregdata = crate::Reg<vwregdata::VwregdataSpec>;
#[doc = "Virtual Wire Register Data Register (VWREGDATA)"]
pub mod vwregdata;
#[doc = "OOBRXRDHEAD (rw) register accessor: OOB Receive Buffer Read Head Register (OOBRXRDHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxrdhead::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxrdhead::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxrdhead`]
module"]
#[doc(alias = "OOBRXRDHEAD")]
pub type Oobrxrdhead = crate::Reg<oobrxrdhead::OobrxrdheadSpec>;
#[doc = "OOB Receive Buffer Read Head Register (OOBRXRDHEAD)"]
pub mod oobrxrdhead;
#[doc = "OOBTXWRHEAD (rw) register accessor: OOB Transmit Buffer Write Head Register (OOBTXWRHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxwrhead::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxwrhead::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxwrhead`]
module"]
#[doc(alias = "OOBTXWRHEAD")]
pub type Oobtxwrhead = crate::Reg<oobtxwrhead::OobtxwrheadSpec>;
#[doc = "OOB Transmit Buffer Write Head Register (OOBTXWRHEAD)"]
pub mod oobtxwrhead;
#[doc = "OOBCTL (rw) register accessor: OOB Channel Control Register (OOBCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobctl`]
module"]
#[doc(alias = "OOBCTL")]
pub type Oobctl = crate::Reg<oobctl::OobctlSpec>;
#[doc = "OOB Channel Control Register (OOBCTL)"]
pub mod oobctl;
#[doc = "FLASHRXRDHEAD (rw) register accessor: Flash Receive Buffer Read Head Register (FLASHRXRDHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxrdhead::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxrdhead::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxrdhead`]
module"]
#[doc(alias = "FLASHRXRDHEAD")]
pub type Flashrxrdhead = crate::Reg<flashrxrdhead::FlashrxrdheadSpec>;
#[doc = "Flash Receive Buffer Read Head Register (FLASHRXRDHEAD)"]
pub mod flashrxrdhead;
#[doc = "FLASHTXWRHEAD (rw) register accessor: Flash Transmit Buffer Write Head Register (FLASHTXWRHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxwrhead::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxwrhead::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxwrhead`]
module"]
#[doc(alias = "FLASHTXWRHEAD")]
pub type Flashtxwrhead = crate::Reg<flashtxwrhead::FlashtxwrheadSpec>;
#[doc = "Flash Transmit Buffer Write Head Register (FLASHTXWRHEAD)"]
pub mod flashtxwrhead;
#[doc = "FLASHCFG (rw) register accessor: Flash Channel Configuration Register (FLASHCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcfg`]
module"]
#[doc(alias = "FLASHCFG")]
pub type Flashcfg = crate::Reg<flashcfg::FlashcfgSpec>;
#[doc = "Flash Channel Configuration Register (FLASHCFG)"]
pub mod flashcfg;
#[doc = "FLASHCTL (rw) register accessor: Flash Channel Control Register (FLASHCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashctl`]
module"]
#[doc(alias = "FLASHCTL")]
pub type Flashctl = crate::Reg<flashctl::FlashctlSpec>;
#[doc = "Flash Channel Control Register (FLASHCTL)"]
pub mod flashctl;
#[doc = "ESPIERR (rw) register accessor: eSPI Error Status Register (ESPIERR)\n\nYou can [`read`](crate::Reg::read) this register and get [`espierr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`espierr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@espierr`]
module"]
#[doc(alias = "ESPIERR")]
pub type Espierr = crate::Reg<espierr::EspierrSpec>;
#[doc = "eSPI Error Status Register (ESPIERR)"]
pub mod espierr;
#[doc = "PBMRXRDHEAD (rw) register accessor: Peripheral Bus Master Receive Buffer Read Head Register (PBMRXRDHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxrdhead::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxrdhead::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxrdhead`]
module"]
#[doc(alias = "PBMRXRDHEAD")]
pub type Pbmrxrdhead = crate::Reg<pbmrxrdhead::PbmrxrdheadSpec>;
#[doc = "Peripheral Bus Master Receive Buffer Read Head Register (PBMRXRDHEAD)"]
pub mod pbmrxrdhead;
#[doc = "PBMTXWRHEAD (rw) register accessor: Peripheral Bus Master Transmit Buffer Write Head Register (PBMTXWRHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxwrhead::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxwrhead::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxwrhead`]
module"]
#[doc(alias = "PBMTXWRHEAD")]
pub type Pbmtxwrhead = crate::Reg<pbmtxwrhead::PbmtxwrheadSpec>;
#[doc = "Peripheral Bus Master Transmit Buffer Write Head Register (PBMTXWRHEAD)"]
pub mod pbmtxwrhead;
#[doc = "PERCFG (rw) register accessor: Peripheral Channel Configuration Register (PERCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`percfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`percfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@percfg`]
module"]
#[doc(alias = "PERCFG")]
pub type Percfg = crate::Reg<percfg::PercfgSpec>;
#[doc = "Peripheral Channel Configuration Register (PERCFG)"]
pub mod percfg;
#[doc = "PERCTL (rw) register accessor: Peripheral Channel Control Register (PERCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`perctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perctl`]
module"]
#[doc(alias = "PERCTL")]
pub type Perctl = crate::Reg<perctl::PerctlSpec>;
#[doc = "Peripheral Channel Control Register (PERCTL)"]
pub mod perctl;
#[doc = "STATUS_IMG (rw) register accessor: Status Image Register (STATUS_IMG)\n\nYou can [`read`](crate::Reg::read) this register and get [`status_img::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status_img::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status_img`]
module"]
#[doc(alias = "STATUS_IMG")]
pub type StatusImg = crate::Reg<status_img::StatusImgSpec>;
#[doc = "Status Image Register (STATUS_IMG)"]
pub mod status_img;
#[doc = "PERCTLBW (rw) register accessor: Peripheral Channel Control for Bus Master Write Register (PERCTLBW)\n\nYou can [`read`](crate::Reg::read) this register and get [`perctlbw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perctlbw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perctlbw`]
module"]
#[doc(alias = "PERCTLBW")]
pub type Perctlbw = crate::Reg<perctlbw::PerctlbwSpec>;
#[doc = "Peripheral Channel Control for Bus Master Write Register (PERCTLBW)"]
pub mod perctlbw;
#[doc = "PSMRXRDHEAD (rw) register accessor: Peripheral Target Message Receive Buffer Read Head Register (PSMRXRDHEAD)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxrdhead::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxrdhead::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxrdhead`]
module"]
#[doc(alias = "PSMRXRDHEAD")]
pub type Psmrxrdhead = crate::Reg<psmrxrdhead::PsmrxrdheadSpec>;
#[doc = "Peripheral Target Message Receive Buffer Read Head Register (PSMRXRDHEAD)"]
pub mod psmrxrdhead;
#[doc = "VWEVSM (rw) register accessor: Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevsm`]
module"]
#[doc(alias = "VWEVSM")]
pub type Vwevsm = crate::Reg<vwevsm::VwevsmSpec>;
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
pub mod vwevsm;
#[doc = "VWSWIRQ (rw) register accessor: Virtual Wire Software IRQ Register (VWSWIRQ)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwswirq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwswirq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwswirq`]
module"]
#[doc(alias = "VWSWIRQ")]
pub type Vwswirq = crate::Reg<vwswirq::VwswirqSpec>;
#[doc = "Virtual Wire Software IRQ Register (VWSWIRQ)"]
pub mod vwswirq;
#[doc = "VWEVMS (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms`]
module"]
#[doc(alias = "VWEVMS")]
pub type Vwevms = crate::Reg<vwevms::VwevmsSpec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms;
#[doc = "VWGPSM (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm`]
module"]
#[doc(alias = "VWGPSM")]
pub type Vwgpsm = crate::Reg<vwgpsm::VwgpsmSpec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm;
#[doc = "VWGPMS (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms`]
module"]
#[doc(alias = "VWGPMS")]
pub type Vwgpms = crate::Reg<vwgpms::VwgpmsSpec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms;
#[doc = "VWCTL (rw) register accessor: Virtual Wire Channel Control Register (VWCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwctl`]
module"]
#[doc(alias = "VWCTL")]
pub type Vwctl = crate::Reg<vwctl::VwctlSpec>;
#[doc = "Virtual Wire Channel Control Register (VWCTL)"]
pub mod vwctl;
#[doc = "OOBRXBUF (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf`]
module"]
#[doc(alias = "OOBRXBUF")]
pub type Oobrxbuf = crate::Reg<oobrxbuf::OobrxbufSpec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf;
#[doc = "OOBTXBUF (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf`]
module"]
#[doc(alias = "OOBTXBUF")]
pub type Oobtxbuf = crate::Reg<oobtxbuf::OobtxbufSpec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf;
#[doc = "FLASHRXBUF (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf`]
module"]
#[doc(alias = "FLASHRXBUF")]
pub type Flashrxbuf = crate::Reg<flashrxbuf::FlashrxbufSpec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf;
#[doc = "FLASHTXBUF (rw) register accessor: Flash transmit buffer\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf`]
module"]
#[doc(alias = "FLASHTXBUF")]
pub type Flashtxbuf = crate::Reg<flashtxbuf::FlashtxbufSpec>;
#[doc = "Flash transmit buffer"]
pub mod flashtxbuf;
#[doc = "PBMRXBUF (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf`]
module"]
#[doc(alias = "PBMRXBUF")]
pub type Pbmrxbuf = crate::Reg<pbmrxbuf::PbmrxbufSpec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf;
#[doc = "PBMTXBUF (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf`]
module"]
#[doc(alias = "PBMTXBUF")]
pub type Pbmtxbuf = crate::Reg<pbmtxbuf::PbmtxbufSpec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf;
#[doc = "FLASH_PRTR_BADDR (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR")]
pub type FlashPrtrBaddr = crate::Reg<flash_prtr_baddr::FlashPrtrBaddrSpec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr;
#[doc = "FLASH_PRTR_TADDR (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR")]
pub type FlashPrtrTaddr = crate::Reg<flash_prtr_taddr::FlashPrtrTaddrSpec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr;
#[doc = "FLASH_RNG_TAG_OVR (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR")]
pub type FlashRngTagOvr = crate::Reg<flash_rng_tag_ovr::FlashRngTagOvrSpec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr;
#[doc = "FLASH_RPMC_CFG_1 (rw) register accessor: Flash RPMC Configuration 1 Register (FLASH_RPMC_CFG_1)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rpmc_cfg_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rpmc_cfg_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rpmc_cfg_1`]
module"]
#[doc(alias = "FLASH_RPMC_CFG_1")]
pub type FlashRpmcCfg1 = crate::Reg<flash_rpmc_cfg_1::FlashRpmcCfg1Spec>;
#[doc = "Flash RPMC Configuration 1 Register (FLASH_RPMC_CFG_1)"]
pub mod flash_rpmc_cfg_1;
#[doc = "FLASH_RPMC_CFG_2 (rw) register accessor: Flash RPMC Configuration Register 2 (FLASH_RPMC_CFG_2)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rpmc_cfg_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rpmc_cfg_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rpmc_cfg_2`]
module"]
#[doc(alias = "FLASH_RPMC_CFG_2")]
pub type FlashRpmcCfg2 = crate::Reg<flash_rpmc_cfg_2::FlashRpmcCfg2Spec>;
#[doc = "Flash RPMC Configuration Register 2 (FLASH_RPMC_CFG_2)"]
pub mod flash_rpmc_cfg_2;
#[doc = "RMAP_FLASH_OFFS (rw) register accessor: Remapping Flash Offset Register (RMAP_FLASH_OFFS)\n\nYou can [`read`](crate::Reg::read) this register and get [`rmap_flash_offs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmap_flash_offs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmap_flash_offs`]
module"]
#[doc(alias = "RMAP_FLASH_OFFS")]
pub type RmapFlashOffs = crate::Reg<rmap_flash_offs::RmapFlashOffsSpec>;
#[doc = "Remapping Flash Offset Register (RMAP_FLASH_OFFS)"]
pub mod rmap_flash_offs;
#[doc = "RMAP_DST_BASE (rw) register accessor: Remapping Destination Base Register (RMAP_DST_BASE)\n\nYou can [`read`](crate::Reg::read) this register and get [`rmap_dst_base::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmap_dst_base::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmap_dst_base`]
module"]
#[doc(alias = "RMAP_DST_BASE")]
pub type RmapDstBase = crate::Reg<rmap_dst_base::RmapDstBaseSpec>;
#[doc = "Remapping Destination Base Register (RMAP_DST_BASE)"]
pub mod rmap_dst_base;
#[doc = "RMAP_WIN_SIZE (rw) register accessor: Remapping Window Size Register (RMAP_WIN_SIZE)\n\nYou can [`read`](crate::Reg::read) this register and get [`rmap_win_size::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmap_win_size::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmap_win_size`]
module"]
#[doc(alias = "RMAP_WIN_SIZE")]
pub type RmapWinSize = crate::Reg<rmap_win_size::RmapWinSizeSpec>;
#[doc = "Remapping Window Size Register (RMAP_WIN_SIZE)"]
pub mod rmap_win_size;
#[doc = "FLASHBASE (rw) register accessor: Flash Base Register (FLASHBASE)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashbase`]
module"]
#[doc(alias = "FLASHBASE")]
pub type Flashbase = crate::Reg<flashbase::FlashbaseSpec>;
#[doc = "Flash Base Register (FLASHBASE)"]
pub mod flashbase;
#[doc = "FLRLCK (rw) register accessor: Flash Reversible Lock Register (FLRLCK)\n\nYou can [`read`](crate::Reg::read) this register and get [`flrlck::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flrlck::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flrlck`]
module"]
#[doc(alias = "FLRLCK")]
pub type Flrlck = crate::Reg<flrlck::FlrlckSpec>;
#[doc = "Flash Reversible Lock Register (FLRLCK)"]
pub mod flrlck;
#[doc = "PSMRXBUF (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf`]
module"]
#[doc(alias = "PSMRXBUF")]
pub type Psmrxbuf = crate::Reg<psmrxbuf::PsmrxbufSpec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf;
