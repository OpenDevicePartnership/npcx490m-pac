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
    vwevsm090: Vwevsm090,
    vwevsm091: Vwevsm091,
    vwevsm092: Vwevsm092,
    _reserved25: [u8; 0x06],
    vwevsm093: Vwevsm093,
    vwevsm094: Vwevsm094,
    _reserved27: [u8; 0x06],
    vwevsm095: Vwevsm095,
    vwevsm096: Vwevsm096,
    vwevsm097: Vwevsm097,
    _reserved30: [u8; 0x06],
    vwevsm098: Vwevsm098,
    vwevsm099: Vwevsm099,
    _reserved32: [u8; 0x02],
    vwswirq: Vwswirq,
    vwevms0110: Vwevms0110,
    vwevms0111: Vwevms0111,
    vwevms0112: Vwevms0112,
    _reserved36: [u8; 0x06],
    vwevms0113: Vwevms0113,
    vwevms0114: Vwevms0114,
    _reserved38: [u8; 0x06],
    vwevms0115: Vwevms0115,
    vwevms0116: Vwevms0116,
    vwevms0117: Vwevms0117,
    _reserved41: [u8; 0x06],
    vwevms0118: Vwevms0118,
    vwevms0119: Vwevms0119,
    _reserved43: [u8; 0x06],
    _reserved_43_vwgpsm0150: [u8; 0x04],
    _reserved_44_vwgpsm0151: [u8; 0x04],
    vwgpsm0152: Vwgpsm0152,
    _reserved46: [u8; 0x04],
    _reserved_46_vwgpms0150: [u8; 0x0c],
    _reserved47: [u8; 0x64],
    _reserved_47_vwgpms0153: [u8; 0x0c],
    _reserved48: [u8; 0x04],
    _reserved_48_vwgpms0155: [u8; 0x0c],
    _reserved49: [u8; 0x04],
    _reserved_49_vwgpms0158: [u8; 0x0c],
    _reserved50: [u8; 0x04],
    _reserved_50_vwgpms01510: [u8; 0x0c],
    _reserved51: [u8; 0x04],
    _reserved_51_vwgpms01513: [u8; 0x06],
    vwgpms01514: Vwgpms01514,
    _reserved53: [u8; 0x06],
    vwgpms01515: Vwgpms01515,
    _reserved54: [u8; 0xa8],
    vwctl: Vwctl,
    oobrxbuf0190: Oobrxbuf0190,
    oobrxbuf0191: Oobrxbuf0191,
    oobrxbuf0192: Oobrxbuf0192,
    _reserved58: [u8; 0x06],
    oobrxbuf0193: Oobrxbuf0193,
    oobrxbuf0194: Oobrxbuf0194,
    _reserved60: [u8; 0x06],
    oobrxbuf0195: Oobrxbuf0195,
    oobrxbuf0196: Oobrxbuf0196,
    oobrxbuf0197: Oobrxbuf0197,
    _reserved63: [u8; 0x06],
    oobrxbuf0198: Oobrxbuf0198,
    oobrxbuf0199: Oobrxbuf0199,
    _reserved65: [u8; 0x06],
    oobrxbuf01910: Oobrxbuf01910,
    oobrxbuf01911: Oobrxbuf01911,
    oobrxbuf01912: Oobrxbuf01912,
    _reserved68: [u8; 0x06],
    oobrxbuf01913: Oobrxbuf01913,
    oobrxbuf01914: Oobrxbuf01914,
    _reserved70: [u8; 0x06],
    oobrxbuf01915: Oobrxbuf01915,
    oobrxbuf01916: Oobrxbuf01916,
    oobrxbuf01917: Oobrxbuf01917,
    _reserved73: [u8; 0x06],
    oobrxbuf01918: Oobrxbuf01918,
    oobrxbuf01919: Oobrxbuf01919,
    _reserved75: [u8; 0x06],
    oobtxbuf0190: Oobtxbuf0190,
    oobtxbuf0191: Oobtxbuf0191,
    oobtxbuf0192: Oobtxbuf0192,
    _reserved78: [u8; 0x06],
    oobtxbuf0193: Oobtxbuf0193,
    oobtxbuf0194: Oobtxbuf0194,
    _reserved80: [u8; 0x66],
    _reserved_80_oobtxbuf0195: [u8; 0x04],
    _reserved_81_oobtxbuf0196: [u8; 0x04],
    _reserved_82_oobtxbuf0197: [u8; 0x04],
    _reserved83: [u8; 0x06],
    _reserved_83_oobtxbuf0198: [u8; 0x04],
    _reserved_84_oobtxbuf0199: [u8; 0x04],
    _reserved85: [u8; 0x06],
    _reserved_85_oobtxbuf01910: [u8; 0x04],
    _reserved_86_oobtxbuf01911: [u8; 0x04],
    _reserved_87_oobtxbuf01912: [u8; 0x04],
    _reserved88: [u8; 0x06],
    _reserved_88_oobtxbuf01913: [u8; 0x04],
    _reserved_89_oobtxbuf01914: [u8; 0x04],
    _reserved90: [u8; 0x06],
    _reserved_90_oobtxbuf01915: [u8; 0x04],
    _reserved_91_oobtxbuf01916: [u8; 0x04],
    _reserved_92_oobtxbuf01917: [u8; 0x04],
    _reserved93: [u8; 0x06],
    _reserved_93_oobtxbuf01918: [u8; 0x04],
    _reserved_94_oobtxbuf01919: [u8; 0x04],
    _reserved95: [u8; 0x06],
    flashrxbuf01715: Flashrxbuf01715,
    flashrxbuf01716: Flashrxbuf01716,
    flashrxbuf01717: Flashrxbuf01717,
    _reserved98: [u8; 0x14],
    flashtxbuf0160: Flashtxbuf0160,
    flashtxbuf0161: Flashtxbuf0161,
    flashtxbuf0162: Flashtxbuf0162,
    _reserved101: [u8; 0x06],
    flashtxbuf0163: Flashtxbuf0163,
    flashtxbuf0164: Flashtxbuf0164,
    _reserved103: [u8; 0x66],
    _reserved_103_pbmrxbuf0170: [u8; 0x04],
    _reserved_104_pbmrxbuf0171: [u8; 0x04],
    _reserved_105_pbmrxbuf0172: [u8; 0x04],
    _reserved106: [u8; 0x06],
    _reserved_106_pbmrxbuf0173: [u8; 0x04],
    _reserved_107_pbmrxbuf0174: [u8; 0x04],
    _reserved108: [u8; 0x06],
    _reserved_108_pbmrxbuf0175: [u8; 0x04],
    _reserved_109_pbmrxbuf0176: [u8; 0x04],
    _reserved_110_pbmrxbuf0177: [u8; 0x04],
    _reserved111: [u8; 0x06],
    _reserved_111_pbmrxbuf0178: [u8; 0x04],
    _reserved_112_pbmrxbuf0179: [u8; 0x04],
    _reserved113: [u8; 0x06],
    _reserved_113_pbmrxbuf01710: [u8; 0x04],
    _reserved_114_pbmrxbuf01711: [u8; 0x04],
    pbmrxbuf01712: Pbmrxbuf01712,
    _reserved116: [u8; 0x06],
    pbmrxbuf01713: Pbmrxbuf01713,
    pbmrxbuf01714: Pbmrxbuf01714,
    _reserved118: [u8; 0x06],
    pbmrxbuf01715: Pbmrxbuf01715,
    pbmrxbuf01716: Pbmrxbuf01716,
    pbmrxbuf01717: Pbmrxbuf01717,
    _reserved121: [u8; 0x14],
    pbmtxbuf0180: Pbmtxbuf0180,
    pbmtxbuf0181: Pbmtxbuf0181,
    pbmtxbuf0182: Pbmtxbuf0182,
    _reserved124: [u8; 0x06],
    pbmtxbuf0183: Pbmtxbuf0183,
    pbmtxbuf0184: Pbmtxbuf0184,
    _reserved126: [u8; 0x66],
    _reserved_126_pbmtxbuf0185: [u8; 0x04],
    _reserved_127_pbmtxbuf0186: [u8; 0x04],
    _reserved_128_pbmtxbuf0187: [u8; 0x04],
    _reserved129: [u8; 0x06],
    _reserved_129_pbmtxbuf0188: [u8; 0x04],
    _reserved_130_pbmtxbuf0189: [u8; 0x04],
    _reserved131: [u8; 0x06],
    _reserved_131_pbmtxbuf01810: [u8; 0x04],
    _reserved_132_pbmtxbuf01811: [u8; 0x04],
    _reserved_133_pbmtxbuf01812: [u8; 0x04],
    _reserved134: [u8; 0x06],
    _reserved_134_pbmtxbuf01813: [u8; 0x04],
    _reserved_135_pbmtxbuf01814: [u8; 0x04],
    _reserved136: [u8; 0x06],
    _reserved_136_pbmtxbuf01815: [u8; 0x04],
    _reserved_137_pbmtxbuf01816: [u8; 0x04],
    _reserved_138_pbmtxbuf01817: [u8; 0x04],
    _reserved139: [u8; 0x06],
    _reserved_139_pbmtxbuf01818: [u8; 0x04],
    _reserved_140_flash_prtr: [u8; 0x04],
    _reserved141: [u8; 0x06],
    _reserved_141_flash_prtr: [u8; 0x04],
    flash_prtr_taddr0156: FlashPrtrTaddr0156,
    flash_prtr_taddr0157: FlashPrtrTaddr0157,
    _reserved144: [u8; 0x06],
    flash_prtr_taddr0158: FlashPrtrTaddr0158,
    flash_prtr_taddr0159: FlashPrtrTaddr0159,
    _reserved146: [u8; 0x06],
    _reserved_146_flash: [u8; 0x04],
    _reserved_147_flash: [u8; 0x04],
    _reserved_148_flash: [u8; 0x04],
    _reserved149: [u8; 0x06],
    _reserved_149_flash: [u8; 0x04],
    _reserved_150_flash: [u8; 0x04],
    _reserved151: [u8; 0x66],
    _reserved_151_flash: [u8; 0x04],
    flash_rng_tag_ovr0156: FlashRngTagOvr0156,
    flash_rng_tag_ovr0157: FlashRngTagOvr0157,
    _reserved154: [u8; 0x06],
    flash_rng_tag_ovr0158: FlashRngTagOvr0158,
    flash_rng_tag_ovr0159: FlashRngTagOvr0159,
    _reserved156: [u8; 0x06],
    flash_rng_tag_ovr01510: FlashRngTagOvr01510,
    flash_rng_tag_ovr01511: FlashRngTagOvr01511,
    flash_rng_tag_ovr01512: FlashRngTagOvr01512,
    _reserved159: [u8; 0x06],
    flash_rng_tag_ovr01513: FlashRngTagOvr01513,
    flash_rng_tag_ovr01514: FlashRngTagOvr01514,
    _reserved161: [u8; 0x06],
    flash_rng_tag_ovr01515: FlashRngTagOvr01515,
    _reserved162: [u8; 0xbc],
    flash_rpmc_cfg_1: FlashRpmcCfg1,
    flash_rpmc_cfg_2: FlashRpmcCfg2,
    rmap_flash_offs: RmapFlashOffs,
    rmap_dst_base: RmapDstBase,
    rmap_win_size: RmapWinSize,
    flashbase: Flashbase,
    flrlck: Flrlck,
    _reserved169: [u8; 0xe4],
    psmrxbuf0170: Psmrxbuf0170,
    psmrxbuf0171: Psmrxbuf0171,
    psmrxbuf0172: Psmrxbuf0172,
    _reserved172: [u8; 0x06],
    psmrxbuf0173: Psmrxbuf0173,
    psmrxbuf0174: Psmrxbuf0174,
    _reserved174: [u8; 0x06],
    psmrxbuf0175: Psmrxbuf0175,
    psmrxbuf0176: Psmrxbuf0176,
    psmrxbuf0177: Psmrxbuf0177,
    _reserved177: [u8; 0x06],
    psmrxbuf0178: Psmrxbuf0178,
    psmrxbuf0179: Psmrxbuf0179,
    _reserved179: [u8; 0x06],
    psmrxbuf01710: Psmrxbuf01710,
    psmrxbuf01711: Psmrxbuf01711,
    psmrxbuf01712: Psmrxbuf01712,
    _reserved182: [u8; 0x06],
    psmrxbuf01713: Psmrxbuf01713,
    psmrxbuf01714: Psmrxbuf01714,
    _reserved184: [u8; 0x06],
    psmrxbuf01715: Psmrxbuf01715,
    psmrxbuf01716: Psmrxbuf01716,
    psmrxbuf01717: Psmrxbuf01717,
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
    #[doc = "0x100 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub const fn vwevsm090(&self) -> &Vwevsm090 {
        &self.vwevsm090
    }
    #[doc = "0x104 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub const fn vwevsm091(&self) -> &Vwevsm091 {
        &self.vwevsm091
    }
    #[doc = "0x108 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub const fn vwevsm092(&self) -> &Vwevsm092 {
        &self.vwevsm092
    }
    #[doc = "0x112 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub const fn vwevsm093(&self) -> &Vwevsm093 {
        &self.vwevsm093
    }
    #[doc = "0x116 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub const fn vwevsm094(&self) -> &Vwevsm094 {
        &self.vwevsm094
    }
    #[doc = "0x120 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub const fn vwevsm095(&self) -> &Vwevsm095 {
        &self.vwevsm095
    }
    #[doc = "0x124 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub const fn vwevsm096(&self) -> &Vwevsm096 {
        &self.vwevsm096
    }
    #[doc = "0x128 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub const fn vwevsm097(&self) -> &Vwevsm097 {
        &self.vwevsm097
    }
    #[doc = "0x132 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub const fn vwevsm098(&self) -> &Vwevsm098 {
        &self.vwevsm098
    }
    #[doc = "0x136 - Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
    #[inline(always)]
    pub const fn vwevsm099(&self) -> &Vwevsm099 {
        &self.vwevsm099
    }
    #[doc = "0x13c - Virtual Wire Software IRQ Register (VWSWIRQ)"]
    #[inline(always)]
    pub const fn vwswirq(&self) -> &Vwswirq {
        &self.vwswirq
    }
    #[doc = "0x140 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms0110(&self) -> &Vwevms0110 {
        &self.vwevms0110
    }
    #[doc = "0x144 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms0111(&self) -> &Vwevms0111 {
        &self.vwevms0111
    }
    #[doc = "0x148 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms0112(&self) -> &Vwevms0112 {
        &self.vwevms0112
    }
    #[doc = "0x152 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms0113(&self) -> &Vwevms0113 {
        &self.vwevms0113
    }
    #[doc = "0x156 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms0114(&self) -> &Vwevms0114 {
        &self.vwevms0114
    }
    #[doc = "0x160 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms0115(&self) -> &Vwevms0115 {
        &self.vwevms0115
    }
    #[doc = "0x164 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms0116(&self) -> &Vwevms0116 {
        &self.vwevms0116
    }
    #[doc = "0x168 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms0117(&self) -> &Vwevms0117 {
        &self.vwevms0117
    }
    #[doc = "0x172 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms0118(&self) -> &Vwevms0118 {
        &self.vwevms0118
    }
    #[doc = "0x176 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms0119(&self) -> &Vwevms0119 {
        &self.vwevms0119
    }
    #[doc = "0x180 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm0150(&self) -> &Vwgpsm0150 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    #[doc = "0x180 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms01110(&self) -> &Vwevms01110 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(384).cast() }
    }
    #[doc = "0x184 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm0151(&self) -> &Vwgpsm0151 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(388).cast() }
    }
    #[doc = "0x184 - Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
    #[inline(always)]
    pub const fn vwevms01111(&self) -> &Vwevms01111 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(388).cast() }
    }
    #[doc = "0x188 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm0152(&self) -> &Vwgpsm0152 {
        &self.vwgpsm0152
    }
    #[doc = "0x190 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms0150(&self) -> &Vwgpms0150 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(400).cast() }
    }
    #[doc = "0x192 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm0153(&self) -> &Vwgpsm0153 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(402).cast() }
    }
    #[doc = "0x194 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms0151(&self) -> &Vwgpms0151 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(404).cast() }
    }
    #[doc = "0x196 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm0154(&self) -> &Vwgpsm0154 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(406).cast() }
    }
    #[doc = "0x198 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms0152(&self) -> &Vwgpms0152 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(408).cast() }
    }
    #[doc = "0x200 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm0155(&self) -> &Vwgpsm0155 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).cast() }
    }
    #[doc = "0x202 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms0153(&self) -> &Vwgpms0153 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(514).cast() }
    }
    #[doc = "0x204 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm0156(&self) -> &Vwgpsm0156 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(516).cast() }
    }
    #[doc = "0x206 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms0154(&self) -> &Vwgpms0154 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(518).cast() }
    }
    #[doc = "0x208 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm0157(&self) -> &Vwgpsm0157 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).cast() }
    }
    #[doc = "0x210 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms0155(&self) -> &Vwgpms0155 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(528).cast() }
    }
    #[doc = "0x212 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm0158(&self) -> &Vwgpsm0158 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(530).cast() }
    }
    #[doc = "0x214 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms0156(&self) -> &Vwgpms0156 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(532).cast() }
    }
    #[doc = "0x216 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm0159(&self) -> &Vwgpsm0159 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(534).cast() }
    }
    #[doc = "0x218 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms0157(&self) -> &Vwgpms0157 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(536).cast() }
    }
    #[doc = "0x220 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm01510(&self) -> &Vwgpsm01510 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(544).cast() }
    }
    #[doc = "0x222 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms0158(&self) -> &Vwgpms0158 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(546).cast() }
    }
    #[doc = "0x224 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm01511(&self) -> &Vwgpsm01511 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(548).cast() }
    }
    #[doc = "0x226 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms0159(&self) -> &Vwgpms0159 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(550).cast() }
    }
    #[doc = "0x228 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm01512(&self) -> &Vwgpsm01512 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(552).cast() }
    }
    #[doc = "0x230 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms01510(&self) -> &Vwgpms01510 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(560).cast() }
    }
    #[doc = "0x232 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm01513(&self) -> &Vwgpsm01513 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(562).cast() }
    }
    #[doc = "0x234 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms01511(&self) -> &Vwgpms01511 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(564).cast() }
    }
    #[doc = "0x236 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm01514(&self) -> &Vwgpsm01514 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(566).cast() }
    }
    #[doc = "0x238 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms01512(&self) -> &Vwgpms01512 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(568).cast() }
    }
    #[doc = "0x240 - Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
    #[inline(always)]
    pub const fn vwgpsm01515(&self) -> &Vwgpsm01515 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(576).cast() }
    }
    #[doc = "0x242 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms01513(&self) -> &Vwgpms01513 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(578).cast() }
    }
    #[doc = "0x246 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms01514(&self) -> &Vwgpms01514 {
        &self.vwgpms01514
    }
    #[doc = "0x250 - Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
    #[inline(always)]
    pub const fn vwgpms01515(&self) -> &Vwgpms01515 {
        &self.vwgpms01515
    }
    #[doc = "0x2fc - Virtual Wire Channel Control Register (VWCTL)"]
    #[inline(always)]
    pub const fn vwctl(&self) -> &Vwctl {
        &self.vwctl
    }
    #[doc = "0x300 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf0190(&self) -> &Oobrxbuf0190 {
        &self.oobrxbuf0190
    }
    #[doc = "0x304 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf0191(&self) -> &Oobrxbuf0191 {
        &self.oobrxbuf0191
    }
    #[doc = "0x308 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf0192(&self) -> &Oobrxbuf0192 {
        &self.oobrxbuf0192
    }
    #[doc = "0x312 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf0193(&self) -> &Oobrxbuf0193 {
        &self.oobrxbuf0193
    }
    #[doc = "0x316 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf0194(&self) -> &Oobrxbuf0194 {
        &self.oobrxbuf0194
    }
    #[doc = "0x320 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf0195(&self) -> &Oobrxbuf0195 {
        &self.oobrxbuf0195
    }
    #[doc = "0x324 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf0196(&self) -> &Oobrxbuf0196 {
        &self.oobrxbuf0196
    }
    #[doc = "0x328 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf0197(&self) -> &Oobrxbuf0197 {
        &self.oobrxbuf0197
    }
    #[doc = "0x332 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf0198(&self) -> &Oobrxbuf0198 {
        &self.oobrxbuf0198
    }
    #[doc = "0x336 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf0199(&self) -> &Oobrxbuf0199 {
        &self.oobrxbuf0199
    }
    #[doc = "0x340 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf01910(&self) -> &Oobrxbuf01910 {
        &self.oobrxbuf01910
    }
    #[doc = "0x344 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf01911(&self) -> &Oobrxbuf01911 {
        &self.oobrxbuf01911
    }
    #[doc = "0x348 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf01912(&self) -> &Oobrxbuf01912 {
        &self.oobrxbuf01912
    }
    #[doc = "0x352 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf01913(&self) -> &Oobrxbuf01913 {
        &self.oobrxbuf01913
    }
    #[doc = "0x356 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf01914(&self) -> &Oobrxbuf01914 {
        &self.oobrxbuf01914
    }
    #[doc = "0x360 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf01915(&self) -> &Oobrxbuf01915 {
        &self.oobrxbuf01915
    }
    #[doc = "0x364 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf01916(&self) -> &Oobrxbuf01916 {
        &self.oobrxbuf01916
    }
    #[doc = "0x368 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf01917(&self) -> &Oobrxbuf01917 {
        &self.oobrxbuf01917
    }
    #[doc = "0x372 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf01918(&self) -> &Oobrxbuf01918 {
        &self.oobrxbuf01918
    }
    #[doc = "0x376 - OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
    #[inline(always)]
    pub const fn oobrxbuf01919(&self) -> &Oobrxbuf01919 {
        &self.oobrxbuf01919
    }
    #[doc = "0x380 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf0190(&self) -> &Oobtxbuf0190 {
        &self.oobtxbuf0190
    }
    #[doc = "0x384 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf0191(&self) -> &Oobtxbuf0191 {
        &self.oobtxbuf0191
    }
    #[doc = "0x388 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf0192(&self) -> &Oobtxbuf0192 {
        &self.oobtxbuf0192
    }
    #[doc = "0x392 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf0193(&self) -> &Oobtxbuf0193 {
        &self.oobtxbuf0193
    }
    #[doc = "0x396 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf0194(&self) -> &Oobtxbuf0194 {
        &self.oobtxbuf0194
    }
    #[doc = "0x400 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf0170(&self) -> &Flashrxbuf0170 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1024).cast() }
    }
    #[doc = "0x400 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf0195(&self) -> &Oobtxbuf0195 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1024).cast() }
    }
    #[doc = "0x404 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf0171(&self) -> &Flashrxbuf0171 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1028).cast() }
    }
    #[doc = "0x404 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf0196(&self) -> &Oobtxbuf0196 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1028).cast() }
    }
    #[doc = "0x408 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf0172(&self) -> &Flashrxbuf0172 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1032).cast() }
    }
    #[doc = "0x408 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf0197(&self) -> &Oobtxbuf0197 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1032).cast() }
    }
    #[doc = "0x412 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf0173(&self) -> &Flashrxbuf0173 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1042).cast() }
    }
    #[doc = "0x412 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf0198(&self) -> &Oobtxbuf0198 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1042).cast() }
    }
    #[doc = "0x416 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf0174(&self) -> &Flashrxbuf0174 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1046).cast() }
    }
    #[doc = "0x416 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf0199(&self) -> &Oobtxbuf0199 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1046).cast() }
    }
    #[doc = "0x420 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf0175(&self) -> &Flashrxbuf0175 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1056).cast() }
    }
    #[doc = "0x420 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf01910(&self) -> &Oobtxbuf01910 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1056).cast() }
    }
    #[doc = "0x424 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf0176(&self) -> &Flashrxbuf0176 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1060).cast() }
    }
    #[doc = "0x424 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf01911(&self) -> &Oobtxbuf01911 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1060).cast() }
    }
    #[doc = "0x428 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf0177(&self) -> &Flashrxbuf0177 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1064).cast() }
    }
    #[doc = "0x428 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf01912(&self) -> &Oobtxbuf01912 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1064).cast() }
    }
    #[doc = "0x432 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf0178(&self) -> &Flashrxbuf0178 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1074).cast() }
    }
    #[doc = "0x432 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf01913(&self) -> &Oobtxbuf01913 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1074).cast() }
    }
    #[doc = "0x436 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf0179(&self) -> &Flashrxbuf0179 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1078).cast() }
    }
    #[doc = "0x436 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf01914(&self) -> &Oobtxbuf01914 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1078).cast() }
    }
    #[doc = "0x440 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf01710(&self) -> &Flashrxbuf01710 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1088).cast() }
    }
    #[doc = "0x440 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf01915(&self) -> &Oobtxbuf01915 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1088).cast() }
    }
    #[doc = "0x444 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf01711(&self) -> &Flashrxbuf01711 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1092).cast() }
    }
    #[doc = "0x444 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf01916(&self) -> &Oobtxbuf01916 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1092).cast() }
    }
    #[doc = "0x448 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf01712(&self) -> &Flashrxbuf01712 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1096).cast() }
    }
    #[doc = "0x448 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf01917(&self) -> &Oobtxbuf01917 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1096).cast() }
    }
    #[doc = "0x452 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf01713(&self) -> &Flashrxbuf01713 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1106).cast() }
    }
    #[doc = "0x452 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf01918(&self) -> &Oobtxbuf01918 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1106).cast() }
    }
    #[doc = "0x456 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf01714(&self) -> &Flashrxbuf01714 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1110).cast() }
    }
    #[doc = "0x456 - OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
    #[inline(always)]
    pub const fn oobtxbuf01919(&self) -> &Oobtxbuf01919 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1110).cast() }
    }
    #[doc = "0x460 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf01715(&self) -> &Flashrxbuf01715 {
        &self.flashrxbuf01715
    }
    #[doc = "0x464 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf01716(&self) -> &Flashrxbuf01716 {
        &self.flashrxbuf01716
    }
    #[doc = "0x468 - Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
    #[inline(always)]
    pub const fn flashrxbuf01717(&self) -> &Flashrxbuf01717 {
        &self.flashrxbuf01717
    }
    #[doc = "0x480 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf0160(&self) -> &Flashtxbuf0160 {
        &self.flashtxbuf0160
    }
    #[doc = "0x484 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf0161(&self) -> &Flashtxbuf0161 {
        &self.flashtxbuf0161
    }
    #[doc = "0x488 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf0162(&self) -> &Flashtxbuf0162 {
        &self.flashtxbuf0162
    }
    #[doc = "0x492 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf0163(&self) -> &Flashtxbuf0163 {
        &self.flashtxbuf0163
    }
    #[doc = "0x496 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf0164(&self) -> &Flashtxbuf0164 {
        &self.flashtxbuf0164
    }
    #[doc = "0x500 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf0170(&self) -> &Pbmrxbuf0170 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1280).cast() }
    }
    #[doc = "0x500 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf0165(&self) -> &Flashtxbuf0165 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1280).cast() }
    }
    #[doc = "0x504 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf0171(&self) -> &Pbmrxbuf0171 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1284).cast() }
    }
    #[doc = "0x504 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf0166(&self) -> &Flashtxbuf0166 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1284).cast() }
    }
    #[doc = "0x508 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf0172(&self) -> &Pbmrxbuf0172 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1288).cast() }
    }
    #[doc = "0x508 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf0167(&self) -> &Flashtxbuf0167 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1288).cast() }
    }
    #[doc = "0x512 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf0173(&self) -> &Pbmrxbuf0173 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1298).cast() }
    }
    #[doc = "0x512 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf0168(&self) -> &Flashtxbuf0168 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1298).cast() }
    }
    #[doc = "0x516 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf0174(&self) -> &Pbmrxbuf0174 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1302).cast() }
    }
    #[doc = "0x516 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf0169(&self) -> &Flashtxbuf0169 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1302).cast() }
    }
    #[doc = "0x520 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf0175(&self) -> &Pbmrxbuf0175 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1312).cast() }
    }
    #[doc = "0x520 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf01610(&self) -> &Flashtxbuf01610 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1312).cast() }
    }
    #[doc = "0x524 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf0176(&self) -> &Pbmrxbuf0176 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1316).cast() }
    }
    #[doc = "0x524 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf01611(&self) -> &Flashtxbuf01611 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1316).cast() }
    }
    #[doc = "0x528 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf0177(&self) -> &Pbmrxbuf0177 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1320).cast() }
    }
    #[doc = "0x528 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf01612(&self) -> &Flashtxbuf01612 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1320).cast() }
    }
    #[doc = "0x532 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf0178(&self) -> &Pbmrxbuf0178 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1330).cast() }
    }
    #[doc = "0x532 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf01613(&self) -> &Flashtxbuf01613 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1330).cast() }
    }
    #[doc = "0x536 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf0179(&self) -> &Pbmrxbuf0179 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1334).cast() }
    }
    #[doc = "0x536 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf01614(&self) -> &Flashtxbuf01614 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1334).cast() }
    }
    #[doc = "0x540 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf01710(&self) -> &Pbmrxbuf01710 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1344).cast() }
    }
    #[doc = "0x540 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf01615(&self) -> &Flashtxbuf01615 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1344).cast() }
    }
    #[doc = "0x544 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf01711(&self) -> &Pbmrxbuf01711 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1348).cast() }
    }
    #[doc = "0x544 - Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
    #[inline(always)]
    pub const fn flashtxbuf01616(&self) -> &Flashtxbuf01616 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1348).cast() }
    }
    #[doc = "0x548 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf01712(&self) -> &Pbmrxbuf01712 {
        &self.pbmrxbuf01712
    }
    #[doc = "0x552 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf01713(&self) -> &Pbmrxbuf01713 {
        &self.pbmrxbuf01713
    }
    #[doc = "0x556 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf01714(&self) -> &Pbmrxbuf01714 {
        &self.pbmrxbuf01714
    }
    #[doc = "0x560 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf01715(&self) -> &Pbmrxbuf01715 {
        &self.pbmrxbuf01715
    }
    #[doc = "0x564 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf01716(&self) -> &Pbmrxbuf01716 {
        &self.pbmrxbuf01716
    }
    #[doc = "0x568 - Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
    #[inline(always)]
    pub const fn pbmrxbuf01717(&self) -> &Pbmrxbuf01717 {
        &self.pbmrxbuf01717
    }
    #[doc = "0x580 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf0180(&self) -> &Pbmtxbuf0180 {
        &self.pbmtxbuf0180
    }
    #[doc = "0x584 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf0181(&self) -> &Pbmtxbuf0181 {
        &self.pbmtxbuf0181
    }
    #[doc = "0x588 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf0182(&self) -> &Pbmtxbuf0182 {
        &self.pbmtxbuf0182
    }
    #[doc = "0x592 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf0183(&self) -> &Pbmtxbuf0183 {
        &self.pbmtxbuf0183
    }
    #[doc = "0x596 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf0184(&self) -> &Pbmtxbuf0184 {
        &self.pbmtxbuf0184
    }
    #[doc = "0x600 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr0150(&self) -> &FlashPrtrBaddr0150 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1536).cast() }
    }
    #[doc = "0x600 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf0185(&self) -> &Pbmtxbuf0185 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1536).cast() }
    }
    #[doc = "0x604 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr0151(&self) -> &FlashPrtrBaddr0151 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1540).cast() }
    }
    #[doc = "0x604 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf0186(&self) -> &Pbmtxbuf0186 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1540).cast() }
    }
    #[doc = "0x608 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr0152(&self) -> &FlashPrtrBaddr0152 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1544).cast() }
    }
    #[doc = "0x608 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf0187(&self) -> &Pbmtxbuf0187 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1544).cast() }
    }
    #[doc = "0x612 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr0153(&self) -> &FlashPrtrBaddr0153 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1554).cast() }
    }
    #[doc = "0x612 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf0188(&self) -> &Pbmtxbuf0188 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1554).cast() }
    }
    #[doc = "0x616 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr0154(&self) -> &FlashPrtrBaddr0154 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1558).cast() }
    }
    #[doc = "0x616 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf0189(&self) -> &Pbmtxbuf0189 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1558).cast() }
    }
    #[doc = "0x620 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr0155(&self) -> &FlashPrtrBaddr0155 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1568).cast() }
    }
    #[doc = "0x620 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf01810(&self) -> &Pbmtxbuf01810 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1568).cast() }
    }
    #[doc = "0x624 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr0156(&self) -> &FlashPrtrBaddr0156 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1572).cast() }
    }
    #[doc = "0x624 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf01811(&self) -> &Pbmtxbuf01811 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1572).cast() }
    }
    #[doc = "0x628 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr0157(&self) -> &FlashPrtrBaddr0157 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1576).cast() }
    }
    #[doc = "0x628 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf01812(&self) -> &Pbmtxbuf01812 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1576).cast() }
    }
    #[doc = "0x632 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr0158(&self) -> &FlashPrtrBaddr0158 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1586).cast() }
    }
    #[doc = "0x632 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf01813(&self) -> &Pbmtxbuf01813 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1586).cast() }
    }
    #[doc = "0x636 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr0159(&self) -> &FlashPrtrBaddr0159 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1590).cast() }
    }
    #[doc = "0x636 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf01814(&self) -> &Pbmtxbuf01814 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1590).cast() }
    }
    #[doc = "0x640 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr0150(&self) -> &FlashPrtrTaddr0150 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1600).cast() }
    }
    #[doc = "0x640 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr01510(&self) -> &FlashPrtrBaddr01510 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1600).cast() }
    }
    #[doc = "0x640 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf01815(&self) -> &Pbmtxbuf01815 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1600).cast() }
    }
    #[doc = "0x644 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr0151(&self) -> &FlashPrtrTaddr0151 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1604).cast() }
    }
    #[doc = "0x644 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr01511(&self) -> &FlashPrtrBaddr01511 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1604).cast() }
    }
    #[doc = "0x644 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf01816(&self) -> &Pbmtxbuf01816 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1604).cast() }
    }
    #[doc = "0x648 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr0152(&self) -> &FlashPrtrTaddr0152 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1608).cast() }
    }
    #[doc = "0x648 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr01512(&self) -> &FlashPrtrBaddr01512 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1608).cast() }
    }
    #[doc = "0x648 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf01817(&self) -> &Pbmtxbuf01817 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1608).cast() }
    }
    #[doc = "0x652 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr0153(&self) -> &FlashPrtrTaddr0153 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1618).cast() }
    }
    #[doc = "0x652 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr01513(&self) -> &FlashPrtrBaddr01513 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1618).cast() }
    }
    #[doc = "0x652 - Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
    #[inline(always)]
    pub const fn pbmtxbuf01818(&self) -> &Pbmtxbuf01818 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1618).cast() }
    }
    #[doc = "0x656 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr0154(&self) -> &FlashPrtrTaddr0154 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1622).cast() }
    }
    #[doc = "0x656 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr01514(&self) -> &FlashPrtrBaddr01514 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1622).cast() }
    }
    #[doc = "0x660 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr0155(&self) -> &FlashPrtrTaddr0155 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1632).cast() }
    }
    #[doc = "0x660 - Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_baddr01515(&self) -> &FlashPrtrBaddr01515 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1632).cast() }
    }
    #[doc = "0x664 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr0156(&self) -> &FlashPrtrTaddr0156 {
        &self.flash_prtr_taddr0156
    }
    #[doc = "0x668 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr0157(&self) -> &FlashPrtrTaddr0157 {
        &self.flash_prtr_taddr0157
    }
    #[doc = "0x672 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr0158(&self) -> &FlashPrtrTaddr0158 {
        &self.flash_prtr_taddr0158
    }
    #[doc = "0x676 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr0159(&self) -> &FlashPrtrTaddr0159 {
        &self.flash_prtr_taddr0159
    }
    #[doc = "0x680 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr0150(&self) -> &FlashRngTagOvr0150 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1664).cast() }
    }
    #[doc = "0x680 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr01510(&self) -> &FlashPrtrTaddr01510 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1664).cast() }
    }
    #[doc = "0x684 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr0151(&self) -> &FlashRngTagOvr0151 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1668).cast() }
    }
    #[doc = "0x684 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr01511(&self) -> &FlashPrtrTaddr01511 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1668).cast() }
    }
    #[doc = "0x688 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr0152(&self) -> &FlashRngTagOvr0152 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1672).cast() }
    }
    #[doc = "0x688 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr01512(&self) -> &FlashPrtrTaddr01512 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1672).cast() }
    }
    #[doc = "0x692 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr0153(&self) -> &FlashRngTagOvr0153 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1682).cast() }
    }
    #[doc = "0x692 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr01513(&self) -> &FlashPrtrTaddr01513 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1682).cast() }
    }
    #[doc = "0x696 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr0154(&self) -> &FlashRngTagOvr0154 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1686).cast() }
    }
    #[doc = "0x696 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr01514(&self) -> &FlashPrtrTaddr01514 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1686).cast() }
    }
    #[doc = "0x700 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr0155(&self) -> &FlashRngTagOvr0155 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1792).cast() }
    }
    #[doc = "0x700 - Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
    #[inline(always)]
    pub const fn flash_prtr_taddr01515(&self) -> &FlashPrtrTaddr01515 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1792).cast() }
    }
    #[doc = "0x704 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr0156(&self) -> &FlashRngTagOvr0156 {
        &self.flash_rng_tag_ovr0156
    }
    #[doc = "0x708 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr0157(&self) -> &FlashRngTagOvr0157 {
        &self.flash_rng_tag_ovr0157
    }
    #[doc = "0x712 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr0158(&self) -> &FlashRngTagOvr0158 {
        &self.flash_rng_tag_ovr0158
    }
    #[doc = "0x716 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr0159(&self) -> &FlashRngTagOvr0159 {
        &self.flash_rng_tag_ovr0159
    }
    #[doc = "0x720 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr01510(&self) -> &FlashRngTagOvr01510 {
        &self.flash_rng_tag_ovr01510
    }
    #[doc = "0x724 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr01511(&self) -> &FlashRngTagOvr01511 {
        &self.flash_rng_tag_ovr01511
    }
    #[doc = "0x728 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr01512(&self) -> &FlashRngTagOvr01512 {
        &self.flash_rng_tag_ovr01512
    }
    #[doc = "0x732 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr01513(&self) -> &FlashRngTagOvr01513 {
        &self.flash_rng_tag_ovr01513
    }
    #[doc = "0x736 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr01514(&self) -> &FlashRngTagOvr01514 {
        &self.flash_rng_tag_ovr01514
    }
    #[doc = "0x740 - Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
    #[inline(always)]
    pub const fn flash_rng_tag_ovr01515(&self) -> &FlashRngTagOvr01515 {
        &self.flash_rng_tag_ovr01515
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
    #[doc = "0x900 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf0170(&self) -> &Psmrxbuf0170 {
        &self.psmrxbuf0170
    }
    #[doc = "0x904 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf0171(&self) -> &Psmrxbuf0171 {
        &self.psmrxbuf0171
    }
    #[doc = "0x908 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf0172(&self) -> &Psmrxbuf0172 {
        &self.psmrxbuf0172
    }
    #[doc = "0x912 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf0173(&self) -> &Psmrxbuf0173 {
        &self.psmrxbuf0173
    }
    #[doc = "0x916 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf0174(&self) -> &Psmrxbuf0174 {
        &self.psmrxbuf0174
    }
    #[doc = "0x920 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf0175(&self) -> &Psmrxbuf0175 {
        &self.psmrxbuf0175
    }
    #[doc = "0x924 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf0176(&self) -> &Psmrxbuf0176 {
        &self.psmrxbuf0176
    }
    #[doc = "0x928 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf0177(&self) -> &Psmrxbuf0177 {
        &self.psmrxbuf0177
    }
    #[doc = "0x932 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf0178(&self) -> &Psmrxbuf0178 {
        &self.psmrxbuf0178
    }
    #[doc = "0x936 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf0179(&self) -> &Psmrxbuf0179 {
        &self.psmrxbuf0179
    }
    #[doc = "0x940 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf01710(&self) -> &Psmrxbuf01710 {
        &self.psmrxbuf01710
    }
    #[doc = "0x944 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf01711(&self) -> &Psmrxbuf01711 {
        &self.psmrxbuf01711
    }
    #[doc = "0x948 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf01712(&self) -> &Psmrxbuf01712 {
        &self.psmrxbuf01712
    }
    #[doc = "0x952 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf01713(&self) -> &Psmrxbuf01713 {
        &self.psmrxbuf01713
    }
    #[doc = "0x956 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf01714(&self) -> &Psmrxbuf01714 {
        &self.psmrxbuf01714
    }
    #[doc = "0x960 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf01715(&self) -> &Psmrxbuf01715 {
        &self.psmrxbuf01715
    }
    #[doc = "0x964 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf01716(&self) -> &Psmrxbuf01716 {
        &self.psmrxbuf01716
    }
    #[doc = "0x968 - Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
    #[inline(always)]
    pub const fn psmrxbuf01717(&self) -> &Psmrxbuf01717 {
        &self.psmrxbuf01717
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
#[doc = "VWEVSM0-90 (rw) register accessor: Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevsm090`]
module"]
#[doc(alias = "VWEVSM0-90")]
pub type Vwevsm090 = crate::Reg<vwevsm090::Vwevsm090Spec>;
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
pub mod vwevsm090;
#[doc = "VWEVSM0-91 (rw) register accessor: Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm091::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm091::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevsm091`]
module"]
#[doc(alias = "VWEVSM0-91")]
pub type Vwevsm091 = crate::Reg<vwevsm091::Vwevsm091Spec>;
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
pub mod vwevsm091;
#[doc = "VWEVSM0-92 (rw) register accessor: Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm092::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm092::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevsm092`]
module"]
#[doc(alias = "VWEVSM0-92")]
pub type Vwevsm092 = crate::Reg<vwevsm092::Vwevsm092Spec>;
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
pub mod vwevsm092;
#[doc = "VWEVSM0-93 (rw) register accessor: Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm093::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm093::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevsm093`]
module"]
#[doc(alias = "VWEVSM0-93")]
pub type Vwevsm093 = crate::Reg<vwevsm093::Vwevsm093Spec>;
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
pub mod vwevsm093;
#[doc = "VWEVSM0-94 (rw) register accessor: Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevsm094`]
module"]
#[doc(alias = "VWEVSM0-94")]
pub type Vwevsm094 = crate::Reg<vwevsm094::Vwevsm094Spec>;
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
pub mod vwevsm094;
#[doc = "VWEVSM0-95 (rw) register accessor: Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm095::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm095::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevsm095`]
module"]
#[doc(alias = "VWEVSM0-95")]
pub type Vwevsm095 = crate::Reg<vwevsm095::Vwevsm095Spec>;
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
pub mod vwevsm095;
#[doc = "VWEVSM0-96 (rw) register accessor: Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm096::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm096::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevsm096`]
module"]
#[doc(alias = "VWEVSM0-96")]
pub type Vwevsm096 = crate::Reg<vwevsm096::Vwevsm096Spec>;
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
pub mod vwevsm096;
#[doc = "VWEVSM0-97 (rw) register accessor: Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm097::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm097::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevsm097`]
module"]
#[doc(alias = "VWEVSM0-97")]
pub type Vwevsm097 = crate::Reg<vwevsm097::Vwevsm097Spec>;
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
pub mod vwevsm097;
#[doc = "VWEVSM0-98 (rw) register accessor: Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevsm098`]
module"]
#[doc(alias = "VWEVSM0-98")]
pub type Vwevsm098 = crate::Reg<vwevsm098::Vwevsm098Spec>;
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
pub mod vwevsm098;
#[doc = "VWEVSM0-99 (rw) register accessor: Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevsm099::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevsm099::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevsm099`]
module"]
#[doc(alias = "VWEVSM0-99")]
pub type Vwevsm099 = crate::Reg<vwevsm099::Vwevsm099Spec>;
#[doc = "Virtual Wire Event Target-to-Controller Register 0-9 (VWEVSM0-9)"]
pub mod vwevsm099;
#[doc = "VWSWIRQ (rw) register accessor: Virtual Wire Software IRQ Register (VWSWIRQ)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwswirq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwswirq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwswirq`]
module"]
#[doc(alias = "VWSWIRQ")]
pub type Vwswirq = crate::Reg<vwswirq::VwswirqSpec>;
#[doc = "Virtual Wire Software IRQ Register (VWSWIRQ)"]
pub mod vwswirq;
#[doc = "VWEVMS0-110 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms0110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms0110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms0110`]
module"]
#[doc(alias = "VWEVMS0-110")]
pub type Vwevms0110 = crate::Reg<vwevms0110::Vwevms0110Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms0110;
#[doc = "VWEVMS0-111 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms0111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms0111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms0111`]
module"]
#[doc(alias = "VWEVMS0-111")]
pub type Vwevms0111 = crate::Reg<vwevms0111::Vwevms0111Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms0111;
#[doc = "VWEVMS0-112 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms0112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms0112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms0112`]
module"]
#[doc(alias = "VWEVMS0-112")]
pub type Vwevms0112 = crate::Reg<vwevms0112::Vwevms0112Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms0112;
#[doc = "VWEVMS0-113 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms0113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms0113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms0113`]
module"]
#[doc(alias = "VWEVMS0-113")]
pub type Vwevms0113 = crate::Reg<vwevms0113::Vwevms0113Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms0113;
#[doc = "VWEVMS0-114 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms0114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms0114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms0114`]
module"]
#[doc(alias = "VWEVMS0-114")]
pub type Vwevms0114 = crate::Reg<vwevms0114::Vwevms0114Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms0114;
#[doc = "VWEVMS0-115 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms0115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms0115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms0115`]
module"]
#[doc(alias = "VWEVMS0-115")]
pub type Vwevms0115 = crate::Reg<vwevms0115::Vwevms0115Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms0115;
#[doc = "VWEVMS0-116 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms0116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms0116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms0116`]
module"]
#[doc(alias = "VWEVMS0-116")]
pub type Vwevms0116 = crate::Reg<vwevms0116::Vwevms0116Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms0116;
#[doc = "VWEVMS0-117 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms0117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms0117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms0117`]
module"]
#[doc(alias = "VWEVMS0-117")]
pub type Vwevms0117 = crate::Reg<vwevms0117::Vwevms0117Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms0117;
#[doc = "VWEVMS0-118 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms0118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms0118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms0118`]
module"]
#[doc(alias = "VWEVMS0-118")]
pub type Vwevms0118 = crate::Reg<vwevms0118::Vwevms0118Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms0118;
#[doc = "VWEVMS0-119 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms0119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms0119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms0119`]
module"]
#[doc(alias = "VWEVMS0-119")]
pub type Vwevms0119 = crate::Reg<vwevms0119::Vwevms0119Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms0119;
#[doc = "VWEVMS0-1110 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms01110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms01110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms01110`]
module"]
#[doc(alias = "VWEVMS0-1110")]
pub type Vwevms01110 = crate::Reg<vwevms01110::Vwevms01110Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms01110;
#[doc = "VWEVMS0-1111 (rw) register accessor: Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwevms01111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwevms01111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwevms01111`]
module"]
#[doc(alias = "VWEVMS0-1111")]
pub type Vwevms01111 = crate::Reg<vwevms01111::Vwevms01111Spec>;
#[doc = "Virtual Wire Event Controller-to-Target Register 0-11 (VWEVMS0-11)"]
pub mod vwevms01111;
#[doc = "VWGPSM0-150 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm0150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm0150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm0150`]
module"]
#[doc(alias = "VWGPSM0-150")]
pub type Vwgpsm0150 = crate::Reg<vwgpsm0150::Vwgpsm0150Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm0150;
#[doc = "VWGPSM0-151 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm0151::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm0151::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm0151`]
module"]
#[doc(alias = "VWGPSM0-151")]
pub type Vwgpsm0151 = crate::Reg<vwgpsm0151::Vwgpsm0151Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm0151;
#[doc = "VWGPSM0-152 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm0152::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm0152::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm0152`]
module"]
#[doc(alias = "VWGPSM0-152")]
pub type Vwgpsm0152 = crate::Reg<vwgpsm0152::Vwgpsm0152Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm0152;
#[doc = "VWGPSM0-153 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm0153::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm0153::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm0153`]
module"]
#[doc(alias = "VWGPSM0-153")]
pub type Vwgpsm0153 = crate::Reg<vwgpsm0153::Vwgpsm0153Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm0153;
#[doc = "VWGPSM0-154 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm0154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm0154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm0154`]
module"]
#[doc(alias = "VWGPSM0-154")]
pub type Vwgpsm0154 = crate::Reg<vwgpsm0154::Vwgpsm0154Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm0154;
#[doc = "VWGPSM0-155 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm0155::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm0155::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm0155`]
module"]
#[doc(alias = "VWGPSM0-155")]
pub type Vwgpsm0155 = crate::Reg<vwgpsm0155::Vwgpsm0155Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm0155;
#[doc = "VWGPSM0-156 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm0156::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm0156::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm0156`]
module"]
#[doc(alias = "VWGPSM0-156")]
pub type Vwgpsm0156 = crate::Reg<vwgpsm0156::Vwgpsm0156Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm0156;
#[doc = "VWGPSM0-157 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm0157::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm0157::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm0157`]
module"]
#[doc(alias = "VWGPSM0-157")]
pub type Vwgpsm0157 = crate::Reg<vwgpsm0157::Vwgpsm0157Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm0157;
#[doc = "VWGPSM0-158 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm0158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm0158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm0158`]
module"]
#[doc(alias = "VWGPSM0-158")]
pub type Vwgpsm0158 = crate::Reg<vwgpsm0158::Vwgpsm0158Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm0158;
#[doc = "VWGPSM0-159 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm0159::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm0159::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm0159`]
module"]
#[doc(alias = "VWGPSM0-159")]
pub type Vwgpsm0159 = crate::Reg<vwgpsm0159::Vwgpsm0159Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm0159;
#[doc = "VWGPSM0-1510 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm01510::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm01510::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm01510`]
module"]
#[doc(alias = "VWGPSM0-1510")]
pub type Vwgpsm01510 = crate::Reg<vwgpsm01510::Vwgpsm01510Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm01510;
#[doc = "VWGPSM0-1511 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm01511::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm01511::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm01511`]
module"]
#[doc(alias = "VWGPSM0-1511")]
pub type Vwgpsm01511 = crate::Reg<vwgpsm01511::Vwgpsm01511Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm01511;
#[doc = "VWGPSM0-1512 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm01512::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm01512::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm01512`]
module"]
#[doc(alias = "VWGPSM0-1512")]
pub type Vwgpsm01512 = crate::Reg<vwgpsm01512::Vwgpsm01512Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm01512;
#[doc = "VWGPSM0-1513 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm01513::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm01513::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm01513`]
module"]
#[doc(alias = "VWGPSM0-1513")]
pub type Vwgpsm01513 = crate::Reg<vwgpsm01513::Vwgpsm01513Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm01513;
#[doc = "VWGPSM0-1514 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm01514::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm01514::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm01514`]
module"]
#[doc(alias = "VWGPSM0-1514")]
pub type Vwgpsm01514 = crate::Reg<vwgpsm01514::Vwgpsm01514Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm01514;
#[doc = "VWGPSM0-1515 (rw) register accessor: Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpsm01515::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpsm01515::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpsm01515`]
module"]
#[doc(alias = "VWGPSM0-1515")]
pub type Vwgpsm01515 = crate::Reg<vwgpsm01515::Vwgpsm01515Spec>;
#[doc = "Virtual Wire GPIO Target-to-Controller Register 0-15 (VWGPSM0-15)"]
pub mod vwgpsm01515;
#[doc = "VWGPMS0-150 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms0150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms0150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms0150`]
module"]
#[doc(alias = "VWGPMS0-150")]
pub type Vwgpms0150 = crate::Reg<vwgpms0150::Vwgpms0150Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms0150;
#[doc = "VWGPMS0-151 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms0151::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms0151::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms0151`]
module"]
#[doc(alias = "VWGPMS0-151")]
pub type Vwgpms0151 = crate::Reg<vwgpms0151::Vwgpms0151Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms0151;
#[doc = "VWGPMS0-152 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms0152::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms0152::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms0152`]
module"]
#[doc(alias = "VWGPMS0-152")]
pub type Vwgpms0152 = crate::Reg<vwgpms0152::Vwgpms0152Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms0152;
#[doc = "VWGPMS0-153 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms0153::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms0153::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms0153`]
module"]
#[doc(alias = "VWGPMS0-153")]
pub type Vwgpms0153 = crate::Reg<vwgpms0153::Vwgpms0153Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms0153;
#[doc = "VWGPMS0-154 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms0154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms0154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms0154`]
module"]
#[doc(alias = "VWGPMS0-154")]
pub type Vwgpms0154 = crate::Reg<vwgpms0154::Vwgpms0154Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms0154;
#[doc = "VWGPMS0-155 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms0155::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms0155::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms0155`]
module"]
#[doc(alias = "VWGPMS0-155")]
pub type Vwgpms0155 = crate::Reg<vwgpms0155::Vwgpms0155Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms0155;
#[doc = "VWGPMS0-156 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms0156::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms0156::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms0156`]
module"]
#[doc(alias = "VWGPMS0-156")]
pub type Vwgpms0156 = crate::Reg<vwgpms0156::Vwgpms0156Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms0156;
#[doc = "VWGPMS0-157 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms0157::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms0157::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms0157`]
module"]
#[doc(alias = "VWGPMS0-157")]
pub type Vwgpms0157 = crate::Reg<vwgpms0157::Vwgpms0157Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms0157;
#[doc = "VWGPMS0-158 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms0158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms0158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms0158`]
module"]
#[doc(alias = "VWGPMS0-158")]
pub type Vwgpms0158 = crate::Reg<vwgpms0158::Vwgpms0158Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms0158;
#[doc = "VWGPMS0-159 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms0159::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms0159::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms0159`]
module"]
#[doc(alias = "VWGPMS0-159")]
pub type Vwgpms0159 = crate::Reg<vwgpms0159::Vwgpms0159Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms0159;
#[doc = "VWGPMS0-1510 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms01510::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms01510::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms01510`]
module"]
#[doc(alias = "VWGPMS0-1510")]
pub type Vwgpms01510 = crate::Reg<vwgpms01510::Vwgpms01510Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms01510;
#[doc = "VWGPMS0-1511 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms01511::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms01511::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms01511`]
module"]
#[doc(alias = "VWGPMS0-1511")]
pub type Vwgpms01511 = crate::Reg<vwgpms01511::Vwgpms01511Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms01511;
#[doc = "VWGPMS0-1512 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms01512::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms01512::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms01512`]
module"]
#[doc(alias = "VWGPMS0-1512")]
pub type Vwgpms01512 = crate::Reg<vwgpms01512::Vwgpms01512Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms01512;
#[doc = "VWGPMS0-1513 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms01513::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms01513::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms01513`]
module"]
#[doc(alias = "VWGPMS0-1513")]
pub type Vwgpms01513 = crate::Reg<vwgpms01513::Vwgpms01513Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms01513;
#[doc = "VWGPMS0-1514 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms01514::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms01514::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms01514`]
module"]
#[doc(alias = "VWGPMS0-1514")]
pub type Vwgpms01514 = crate::Reg<vwgpms01514::Vwgpms01514Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms01514;
#[doc = "VWGPMS0-1515 (rw) register accessor: Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwgpms01515::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwgpms01515::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwgpms01515`]
module"]
#[doc(alias = "VWGPMS0-1515")]
pub type Vwgpms01515 = crate::Reg<vwgpms01515::Vwgpms01515Spec>;
#[doc = "Virtual Wire GPIO Controller-to-Target Register 0-15 (VWGPMS0-15)"]
pub mod vwgpms01515;
#[doc = "VWCTL (rw) register accessor: Virtual Wire Channel Control Register (VWCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwctl`]
module"]
#[doc(alias = "VWCTL")]
pub type Vwctl = crate::Reg<vwctl::VwctlSpec>;
#[doc = "Virtual Wire Channel Control Register (VWCTL)"]
pub mod vwctl;
#[doc = "OOBRXBUF0-190 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf0190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf0190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf0190`]
module"]
#[doc(alias = "OOBRXBUF0-190")]
pub type Oobrxbuf0190 = crate::Reg<oobrxbuf0190::Oobrxbuf0190Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf0190;
#[doc = "OOBRXBUF0-191 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf0191::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf0191::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf0191`]
module"]
#[doc(alias = "OOBRXBUF0-191")]
pub type Oobrxbuf0191 = crate::Reg<oobrxbuf0191::Oobrxbuf0191Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf0191;
#[doc = "OOBRXBUF0-192 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf0192::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf0192::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf0192`]
module"]
#[doc(alias = "OOBRXBUF0-192")]
pub type Oobrxbuf0192 = crate::Reg<oobrxbuf0192::Oobrxbuf0192Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf0192;
#[doc = "OOBRXBUF0-193 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf0193::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf0193::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf0193`]
module"]
#[doc(alias = "OOBRXBUF0-193")]
pub type Oobrxbuf0193 = crate::Reg<oobrxbuf0193::Oobrxbuf0193Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf0193;
#[doc = "OOBRXBUF0-194 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf0194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf0194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf0194`]
module"]
#[doc(alias = "OOBRXBUF0-194")]
pub type Oobrxbuf0194 = crate::Reg<oobrxbuf0194::Oobrxbuf0194Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf0194;
#[doc = "OOBRXBUF0-195 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf0195::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf0195::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf0195`]
module"]
#[doc(alias = "OOBRXBUF0-195")]
pub type Oobrxbuf0195 = crate::Reg<oobrxbuf0195::Oobrxbuf0195Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf0195;
#[doc = "OOBRXBUF0-196 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf0196::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf0196::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf0196`]
module"]
#[doc(alias = "OOBRXBUF0-196")]
pub type Oobrxbuf0196 = crate::Reg<oobrxbuf0196::Oobrxbuf0196Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf0196;
#[doc = "OOBRXBUF0-197 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf0197::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf0197::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf0197`]
module"]
#[doc(alias = "OOBRXBUF0-197")]
pub type Oobrxbuf0197 = crate::Reg<oobrxbuf0197::Oobrxbuf0197Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf0197;
#[doc = "OOBRXBUF0-198 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf0198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf0198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf0198`]
module"]
#[doc(alias = "OOBRXBUF0-198")]
pub type Oobrxbuf0198 = crate::Reg<oobrxbuf0198::Oobrxbuf0198Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf0198;
#[doc = "OOBRXBUF0-199 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf0199::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf0199::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf0199`]
module"]
#[doc(alias = "OOBRXBUF0-199")]
pub type Oobrxbuf0199 = crate::Reg<oobrxbuf0199::Oobrxbuf0199Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf0199;
#[doc = "OOBRXBUF0-1910 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf01910::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf01910::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf01910`]
module"]
#[doc(alias = "OOBRXBUF0-1910")]
pub type Oobrxbuf01910 = crate::Reg<oobrxbuf01910::Oobrxbuf01910Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf01910;
#[doc = "OOBRXBUF0-1911 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf01911::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf01911::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf01911`]
module"]
#[doc(alias = "OOBRXBUF0-1911")]
pub type Oobrxbuf01911 = crate::Reg<oobrxbuf01911::Oobrxbuf01911Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf01911;
#[doc = "OOBRXBUF0-1912 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf01912::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf01912::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf01912`]
module"]
#[doc(alias = "OOBRXBUF0-1912")]
pub type Oobrxbuf01912 = crate::Reg<oobrxbuf01912::Oobrxbuf01912Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf01912;
#[doc = "OOBRXBUF0-1913 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf01913::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf01913::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf01913`]
module"]
#[doc(alias = "OOBRXBUF0-1913")]
pub type Oobrxbuf01913 = crate::Reg<oobrxbuf01913::Oobrxbuf01913Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf01913;
#[doc = "OOBRXBUF0-1914 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf01914::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf01914::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf01914`]
module"]
#[doc(alias = "OOBRXBUF0-1914")]
pub type Oobrxbuf01914 = crate::Reg<oobrxbuf01914::Oobrxbuf01914Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf01914;
#[doc = "OOBRXBUF0-1915 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf01915::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf01915::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf01915`]
module"]
#[doc(alias = "OOBRXBUF0-1915")]
pub type Oobrxbuf01915 = crate::Reg<oobrxbuf01915::Oobrxbuf01915Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf01915;
#[doc = "OOBRXBUF0-1916 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf01916::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf01916::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf01916`]
module"]
#[doc(alias = "OOBRXBUF0-1916")]
pub type Oobrxbuf01916 = crate::Reg<oobrxbuf01916::Oobrxbuf01916Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf01916;
#[doc = "OOBRXBUF0-1917 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf01917::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf01917::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf01917`]
module"]
#[doc(alias = "OOBRXBUF0-1917")]
pub type Oobrxbuf01917 = crate::Reg<oobrxbuf01917::Oobrxbuf01917Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf01917;
#[doc = "OOBRXBUF0-1918 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf01918::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf01918::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf01918`]
module"]
#[doc(alias = "OOBRXBUF0-1918")]
pub type Oobrxbuf01918 = crate::Reg<oobrxbuf01918::Oobrxbuf01918Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf01918;
#[doc = "OOBRXBUF0-1919 (rw) register accessor: OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobrxbuf01919::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobrxbuf01919::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobrxbuf01919`]
module"]
#[doc(alias = "OOBRXBUF0-1919")]
pub type Oobrxbuf01919 = crate::Reg<oobrxbuf01919::Oobrxbuf01919Spec>;
#[doc = "OOB Receive Buffer Register 0-19 (OOBRXBUF0-19)"]
pub mod oobrxbuf01919;
#[doc = "OOBTXBUF0-190 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf0190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf0190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf0190`]
module"]
#[doc(alias = "OOBTXBUF0-190")]
pub type Oobtxbuf0190 = crate::Reg<oobtxbuf0190::Oobtxbuf0190Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf0190;
#[doc = "OOBTXBUF0-191 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf0191::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf0191::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf0191`]
module"]
#[doc(alias = "OOBTXBUF0-191")]
pub type Oobtxbuf0191 = crate::Reg<oobtxbuf0191::Oobtxbuf0191Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf0191;
#[doc = "OOBTXBUF0-192 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf0192::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf0192::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf0192`]
module"]
#[doc(alias = "OOBTXBUF0-192")]
pub type Oobtxbuf0192 = crate::Reg<oobtxbuf0192::Oobtxbuf0192Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf0192;
#[doc = "OOBTXBUF0-193 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf0193::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf0193::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf0193`]
module"]
#[doc(alias = "OOBTXBUF0-193")]
pub type Oobtxbuf0193 = crate::Reg<oobtxbuf0193::Oobtxbuf0193Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf0193;
#[doc = "OOBTXBUF0-194 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf0194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf0194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf0194`]
module"]
#[doc(alias = "OOBTXBUF0-194")]
pub type Oobtxbuf0194 = crate::Reg<oobtxbuf0194::Oobtxbuf0194Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf0194;
#[doc = "OOBTXBUF0-195 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf0195::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf0195::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf0195`]
module"]
#[doc(alias = "OOBTXBUF0-195")]
pub type Oobtxbuf0195 = crate::Reg<oobtxbuf0195::Oobtxbuf0195Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf0195;
#[doc = "OOBTXBUF0-196 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf0196::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf0196::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf0196`]
module"]
#[doc(alias = "OOBTXBUF0-196")]
pub type Oobtxbuf0196 = crate::Reg<oobtxbuf0196::Oobtxbuf0196Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf0196;
#[doc = "OOBTXBUF0-197 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf0197::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf0197::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf0197`]
module"]
#[doc(alias = "OOBTXBUF0-197")]
pub type Oobtxbuf0197 = crate::Reg<oobtxbuf0197::Oobtxbuf0197Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf0197;
#[doc = "OOBTXBUF0-198 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf0198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf0198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf0198`]
module"]
#[doc(alias = "OOBTXBUF0-198")]
pub type Oobtxbuf0198 = crate::Reg<oobtxbuf0198::Oobtxbuf0198Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf0198;
#[doc = "OOBTXBUF0-199 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf0199::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf0199::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf0199`]
module"]
#[doc(alias = "OOBTXBUF0-199")]
pub type Oobtxbuf0199 = crate::Reg<oobtxbuf0199::Oobtxbuf0199Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf0199;
#[doc = "OOBTXBUF0-1910 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf01910::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf01910::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf01910`]
module"]
#[doc(alias = "OOBTXBUF0-1910")]
pub type Oobtxbuf01910 = crate::Reg<oobtxbuf01910::Oobtxbuf01910Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf01910;
#[doc = "OOBTXBUF0-1911 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf01911::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf01911::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf01911`]
module"]
#[doc(alias = "OOBTXBUF0-1911")]
pub type Oobtxbuf01911 = crate::Reg<oobtxbuf01911::Oobtxbuf01911Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf01911;
#[doc = "OOBTXBUF0-1912 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf01912::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf01912::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf01912`]
module"]
#[doc(alias = "OOBTXBUF0-1912")]
pub type Oobtxbuf01912 = crate::Reg<oobtxbuf01912::Oobtxbuf01912Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf01912;
#[doc = "OOBTXBUF0-1913 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf01913::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf01913::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf01913`]
module"]
#[doc(alias = "OOBTXBUF0-1913")]
pub type Oobtxbuf01913 = crate::Reg<oobtxbuf01913::Oobtxbuf01913Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf01913;
#[doc = "OOBTXBUF0-1914 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf01914::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf01914::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf01914`]
module"]
#[doc(alias = "OOBTXBUF0-1914")]
pub type Oobtxbuf01914 = crate::Reg<oobtxbuf01914::Oobtxbuf01914Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf01914;
#[doc = "OOBTXBUF0-1915 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf01915::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf01915::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf01915`]
module"]
#[doc(alias = "OOBTXBUF0-1915")]
pub type Oobtxbuf01915 = crate::Reg<oobtxbuf01915::Oobtxbuf01915Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf01915;
#[doc = "OOBTXBUF0-1916 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf01916::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf01916::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf01916`]
module"]
#[doc(alias = "OOBTXBUF0-1916")]
pub type Oobtxbuf01916 = crate::Reg<oobtxbuf01916::Oobtxbuf01916Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf01916;
#[doc = "OOBTXBUF0-1917 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf01917::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf01917::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf01917`]
module"]
#[doc(alias = "OOBTXBUF0-1917")]
pub type Oobtxbuf01917 = crate::Reg<oobtxbuf01917::Oobtxbuf01917Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf01917;
#[doc = "OOBTXBUF0-1918 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf01918::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf01918::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf01918`]
module"]
#[doc(alias = "OOBTXBUF0-1918")]
pub type Oobtxbuf01918 = crate::Reg<oobtxbuf01918::Oobtxbuf01918Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf01918;
#[doc = "OOBTXBUF0-1919 (rw) register accessor: OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)\n\nYou can [`read`](crate::Reg::read) this register and get [`oobtxbuf01919::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oobtxbuf01919::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oobtxbuf01919`]
module"]
#[doc(alias = "OOBTXBUF0-1919")]
pub type Oobtxbuf01919 = crate::Reg<oobtxbuf01919::Oobtxbuf01919Spec>;
#[doc = "OOB Transmit Buffer Register 0-19 (OOBTXBUF0-19)"]
pub mod oobtxbuf01919;
#[doc = "FLASHRXBUF0-170 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf0170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf0170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf0170`]
module"]
#[doc(alias = "FLASHRXBUF0-170")]
pub type Flashrxbuf0170 = crate::Reg<flashrxbuf0170::Flashrxbuf0170Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf0170;
#[doc = "FLASHRXBUF0-171 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf0171::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf0171::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf0171`]
module"]
#[doc(alias = "FLASHRXBUF0-171")]
pub type Flashrxbuf0171 = crate::Reg<flashrxbuf0171::Flashrxbuf0171Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf0171;
#[doc = "FLASHRXBUF0-172 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf0172::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf0172::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf0172`]
module"]
#[doc(alias = "FLASHRXBUF0-172")]
pub type Flashrxbuf0172 = crate::Reg<flashrxbuf0172::Flashrxbuf0172Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf0172;
#[doc = "FLASHRXBUF0-173 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf0173::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf0173::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf0173`]
module"]
#[doc(alias = "FLASHRXBUF0-173")]
pub type Flashrxbuf0173 = crate::Reg<flashrxbuf0173::Flashrxbuf0173Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf0173;
#[doc = "FLASHRXBUF0-174 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf0174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf0174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf0174`]
module"]
#[doc(alias = "FLASHRXBUF0-174")]
pub type Flashrxbuf0174 = crate::Reg<flashrxbuf0174::Flashrxbuf0174Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf0174;
#[doc = "FLASHRXBUF0-175 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf0175::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf0175::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf0175`]
module"]
#[doc(alias = "FLASHRXBUF0-175")]
pub type Flashrxbuf0175 = crate::Reg<flashrxbuf0175::Flashrxbuf0175Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf0175;
#[doc = "FLASHRXBUF0-176 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf0176::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf0176::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf0176`]
module"]
#[doc(alias = "FLASHRXBUF0-176")]
pub type Flashrxbuf0176 = crate::Reg<flashrxbuf0176::Flashrxbuf0176Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf0176;
#[doc = "FLASHRXBUF0-177 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf0177::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf0177::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf0177`]
module"]
#[doc(alias = "FLASHRXBUF0-177")]
pub type Flashrxbuf0177 = crate::Reg<flashrxbuf0177::Flashrxbuf0177Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf0177;
#[doc = "FLASHRXBUF0-178 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf0178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf0178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf0178`]
module"]
#[doc(alias = "FLASHRXBUF0-178")]
pub type Flashrxbuf0178 = crate::Reg<flashrxbuf0178::Flashrxbuf0178Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf0178;
#[doc = "FLASHRXBUF0-179 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf0179::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf0179::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf0179`]
module"]
#[doc(alias = "FLASHRXBUF0-179")]
pub type Flashrxbuf0179 = crate::Reg<flashrxbuf0179::Flashrxbuf0179Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf0179;
#[doc = "FLASHRXBUF0-1710 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf01710::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf01710::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf01710`]
module"]
#[doc(alias = "FLASHRXBUF0-1710")]
pub type Flashrxbuf01710 = crate::Reg<flashrxbuf01710::Flashrxbuf01710Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf01710;
#[doc = "FLASHRXBUF0-1711 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf01711::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf01711::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf01711`]
module"]
#[doc(alias = "FLASHRXBUF0-1711")]
pub type Flashrxbuf01711 = crate::Reg<flashrxbuf01711::Flashrxbuf01711Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf01711;
#[doc = "FLASHRXBUF0-1712 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf01712::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf01712::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf01712`]
module"]
#[doc(alias = "FLASHRXBUF0-1712")]
pub type Flashrxbuf01712 = crate::Reg<flashrxbuf01712::Flashrxbuf01712Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf01712;
#[doc = "FLASHRXBUF0-1713 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf01713::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf01713::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf01713`]
module"]
#[doc(alias = "FLASHRXBUF0-1713")]
pub type Flashrxbuf01713 = crate::Reg<flashrxbuf01713::Flashrxbuf01713Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf01713;
#[doc = "FLASHRXBUF0-1714 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf01714::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf01714::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf01714`]
module"]
#[doc(alias = "FLASHRXBUF0-1714")]
pub type Flashrxbuf01714 = crate::Reg<flashrxbuf01714::Flashrxbuf01714Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf01714;
#[doc = "FLASHRXBUF0-1715 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf01715::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf01715::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf01715`]
module"]
#[doc(alias = "FLASHRXBUF0-1715")]
pub type Flashrxbuf01715 = crate::Reg<flashrxbuf01715::Flashrxbuf01715Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf01715;
#[doc = "FLASHRXBUF0-1716 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf01716::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf01716::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf01716`]
module"]
#[doc(alias = "FLASHRXBUF0-1716")]
pub type Flashrxbuf01716 = crate::Reg<flashrxbuf01716::Flashrxbuf01716Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf01716;
#[doc = "FLASHRXBUF0-1717 (rw) register accessor: Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashrxbuf01717::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashrxbuf01717::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashrxbuf01717`]
module"]
#[doc(alias = "FLASHRXBUF0-1717")]
pub type Flashrxbuf01717 = crate::Reg<flashrxbuf01717::Flashrxbuf01717Spec>;
#[doc = "Flash Receive Buffer Register 0-17 (FLASHRXBUF0-17)"]
pub mod flashrxbuf01717;
#[doc = "FLASHTXBUF0-160 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf0160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf0160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf0160`]
module"]
#[doc(alias = "FLASHTXBUF0-160")]
pub type Flashtxbuf0160 = crate::Reg<flashtxbuf0160::Flashtxbuf0160Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf0160;
#[doc = "FLASHTXBUF0-161 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf0161::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf0161::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf0161`]
module"]
#[doc(alias = "FLASHTXBUF0-161")]
pub type Flashtxbuf0161 = crate::Reg<flashtxbuf0161::Flashtxbuf0161Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf0161;
#[doc = "FLASHTXBUF0-162 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf0162::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf0162::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf0162`]
module"]
#[doc(alias = "FLASHTXBUF0-162")]
pub type Flashtxbuf0162 = crate::Reg<flashtxbuf0162::Flashtxbuf0162Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf0162;
#[doc = "FLASHTXBUF0-163 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf0163::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf0163::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf0163`]
module"]
#[doc(alias = "FLASHTXBUF0-163")]
pub type Flashtxbuf0163 = crate::Reg<flashtxbuf0163::Flashtxbuf0163Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf0163;
#[doc = "FLASHTXBUF0-164 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf0164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf0164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf0164`]
module"]
#[doc(alias = "FLASHTXBUF0-164")]
pub type Flashtxbuf0164 = crate::Reg<flashtxbuf0164::Flashtxbuf0164Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf0164;
#[doc = "FLASHTXBUF0-165 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf0165::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf0165::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf0165`]
module"]
#[doc(alias = "FLASHTXBUF0-165")]
pub type Flashtxbuf0165 = crate::Reg<flashtxbuf0165::Flashtxbuf0165Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf0165;
#[doc = "FLASHTXBUF0-166 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf0166::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf0166::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf0166`]
module"]
#[doc(alias = "FLASHTXBUF0-166")]
pub type Flashtxbuf0166 = crate::Reg<flashtxbuf0166::Flashtxbuf0166Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf0166;
#[doc = "FLASHTXBUF0-167 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf0167::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf0167::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf0167`]
module"]
#[doc(alias = "FLASHTXBUF0-167")]
pub type Flashtxbuf0167 = crate::Reg<flashtxbuf0167::Flashtxbuf0167Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf0167;
#[doc = "FLASHTXBUF0-168 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf0168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf0168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf0168`]
module"]
#[doc(alias = "FLASHTXBUF0-168")]
pub type Flashtxbuf0168 = crate::Reg<flashtxbuf0168::Flashtxbuf0168Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf0168;
#[doc = "FLASHTXBUF0-169 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf0169::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf0169::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf0169`]
module"]
#[doc(alias = "FLASHTXBUF0-169")]
pub type Flashtxbuf0169 = crate::Reg<flashtxbuf0169::Flashtxbuf0169Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf0169;
#[doc = "FLASHTXBUF0-1610 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf01610::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf01610::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf01610`]
module"]
#[doc(alias = "FLASHTXBUF0-1610")]
pub type Flashtxbuf01610 = crate::Reg<flashtxbuf01610::Flashtxbuf01610Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf01610;
#[doc = "FLASHTXBUF0-1611 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf01611::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf01611::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf01611`]
module"]
#[doc(alias = "FLASHTXBUF0-1611")]
pub type Flashtxbuf01611 = crate::Reg<flashtxbuf01611::Flashtxbuf01611Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf01611;
#[doc = "FLASHTXBUF0-1612 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf01612::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf01612::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf01612`]
module"]
#[doc(alias = "FLASHTXBUF0-1612")]
pub type Flashtxbuf01612 = crate::Reg<flashtxbuf01612::Flashtxbuf01612Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf01612;
#[doc = "FLASHTXBUF0-1613 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf01613::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf01613::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf01613`]
module"]
#[doc(alias = "FLASHTXBUF0-1613")]
pub type Flashtxbuf01613 = crate::Reg<flashtxbuf01613::Flashtxbuf01613Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf01613;
#[doc = "FLASHTXBUF0-1614 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf01614::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf01614::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf01614`]
module"]
#[doc(alias = "FLASHTXBUF0-1614")]
pub type Flashtxbuf01614 = crate::Reg<flashtxbuf01614::Flashtxbuf01614Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf01614;
#[doc = "FLASHTXBUF0-1615 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf01615::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf01615::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf01615`]
module"]
#[doc(alias = "FLASHTXBUF0-1615")]
pub type Flashtxbuf01615 = crate::Reg<flashtxbuf01615::Flashtxbuf01615Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf01615;
#[doc = "FLASHTXBUF0-1616 (rw) register accessor: Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)\n\nYou can [`read`](crate::Reg::read) this register and get [`flashtxbuf01616::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashtxbuf01616::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashtxbuf01616`]
module"]
#[doc(alias = "FLASHTXBUF0-1616")]
pub type Flashtxbuf01616 = crate::Reg<flashtxbuf01616::Flashtxbuf01616Spec>;
#[doc = "Flash Transmit Buffer Register 0-16 (FLASHTXBUF0-16)"]
pub mod flashtxbuf01616;
#[doc = "PBMRXBUF0-170 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf0170`]
module"]
#[doc(alias = "PBMRXBUF0-170")]
pub type Pbmrxbuf0170 = crate::Reg<pbmrxbuf0170::Pbmrxbuf0170Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf0170;
#[doc = "PBMRXBUF0-171 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0171::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0171::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf0171`]
module"]
#[doc(alias = "PBMRXBUF0-171")]
pub type Pbmrxbuf0171 = crate::Reg<pbmrxbuf0171::Pbmrxbuf0171Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf0171;
#[doc = "PBMRXBUF0-172 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0172::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0172::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf0172`]
module"]
#[doc(alias = "PBMRXBUF0-172")]
pub type Pbmrxbuf0172 = crate::Reg<pbmrxbuf0172::Pbmrxbuf0172Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf0172;
#[doc = "PBMRXBUF0-173 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0173::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0173::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf0173`]
module"]
#[doc(alias = "PBMRXBUF0-173")]
pub type Pbmrxbuf0173 = crate::Reg<pbmrxbuf0173::Pbmrxbuf0173Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf0173;
#[doc = "PBMRXBUF0-174 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf0174`]
module"]
#[doc(alias = "PBMRXBUF0-174")]
pub type Pbmrxbuf0174 = crate::Reg<pbmrxbuf0174::Pbmrxbuf0174Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf0174;
#[doc = "PBMRXBUF0-175 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0175::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0175::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf0175`]
module"]
#[doc(alias = "PBMRXBUF0-175")]
pub type Pbmrxbuf0175 = crate::Reg<pbmrxbuf0175::Pbmrxbuf0175Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf0175;
#[doc = "PBMRXBUF0-176 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0176::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0176::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf0176`]
module"]
#[doc(alias = "PBMRXBUF0-176")]
pub type Pbmrxbuf0176 = crate::Reg<pbmrxbuf0176::Pbmrxbuf0176Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf0176;
#[doc = "PBMRXBUF0-177 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0177::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0177::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf0177`]
module"]
#[doc(alias = "PBMRXBUF0-177")]
pub type Pbmrxbuf0177 = crate::Reg<pbmrxbuf0177::Pbmrxbuf0177Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf0177;
#[doc = "PBMRXBUF0-178 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf0178`]
module"]
#[doc(alias = "PBMRXBUF0-178")]
pub type Pbmrxbuf0178 = crate::Reg<pbmrxbuf0178::Pbmrxbuf0178Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf0178;
#[doc = "PBMRXBUF0-179 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf0179::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf0179::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf0179`]
module"]
#[doc(alias = "PBMRXBUF0-179")]
pub type Pbmrxbuf0179 = crate::Reg<pbmrxbuf0179::Pbmrxbuf0179Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf0179;
#[doc = "PBMRXBUF0-1710 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf01710::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf01710::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf01710`]
module"]
#[doc(alias = "PBMRXBUF0-1710")]
pub type Pbmrxbuf01710 = crate::Reg<pbmrxbuf01710::Pbmrxbuf01710Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf01710;
#[doc = "PBMRXBUF0-1711 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf01711::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf01711::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf01711`]
module"]
#[doc(alias = "PBMRXBUF0-1711")]
pub type Pbmrxbuf01711 = crate::Reg<pbmrxbuf01711::Pbmrxbuf01711Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf01711;
#[doc = "PBMRXBUF0-1712 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf01712::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf01712::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf01712`]
module"]
#[doc(alias = "PBMRXBUF0-1712")]
pub type Pbmrxbuf01712 = crate::Reg<pbmrxbuf01712::Pbmrxbuf01712Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf01712;
#[doc = "PBMRXBUF0-1713 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf01713::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf01713::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf01713`]
module"]
#[doc(alias = "PBMRXBUF0-1713")]
pub type Pbmrxbuf01713 = crate::Reg<pbmrxbuf01713::Pbmrxbuf01713Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf01713;
#[doc = "PBMRXBUF0-1714 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf01714::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf01714::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf01714`]
module"]
#[doc(alias = "PBMRXBUF0-1714")]
pub type Pbmrxbuf01714 = crate::Reg<pbmrxbuf01714::Pbmrxbuf01714Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf01714;
#[doc = "PBMRXBUF0-1715 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf01715::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf01715::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf01715`]
module"]
#[doc(alias = "PBMRXBUF0-1715")]
pub type Pbmrxbuf01715 = crate::Reg<pbmrxbuf01715::Pbmrxbuf01715Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf01715;
#[doc = "PBMRXBUF0-1716 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf01716::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf01716::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf01716`]
module"]
#[doc(alias = "PBMRXBUF0-1716")]
pub type Pbmrxbuf01716 = crate::Reg<pbmrxbuf01716::Pbmrxbuf01716Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf01716;
#[doc = "PBMRXBUF0-1717 (rw) register accessor: Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmrxbuf01717::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmrxbuf01717::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmrxbuf01717`]
module"]
#[doc(alias = "PBMRXBUF0-1717")]
pub type Pbmrxbuf01717 = crate::Reg<pbmrxbuf01717::Pbmrxbuf01717Spec>;
#[doc = "Peripheral Bus Master Receive Buffer Register 0-17 (PBMRXBUF0-17)"]
pub mod pbmrxbuf01717;
#[doc = "PBMTXBUF0-180 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf0180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf0180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf0180`]
module"]
#[doc(alias = "PBMTXBUF0-180")]
pub type Pbmtxbuf0180 = crate::Reg<pbmtxbuf0180::Pbmtxbuf0180Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf0180;
#[doc = "PBMTXBUF0-181 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf0181::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf0181::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf0181`]
module"]
#[doc(alias = "PBMTXBUF0-181")]
pub type Pbmtxbuf0181 = crate::Reg<pbmtxbuf0181::Pbmtxbuf0181Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf0181;
#[doc = "PBMTXBUF0-182 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf0182::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf0182::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf0182`]
module"]
#[doc(alias = "PBMTXBUF0-182")]
pub type Pbmtxbuf0182 = crate::Reg<pbmtxbuf0182::Pbmtxbuf0182Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf0182;
#[doc = "PBMTXBUF0-183 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf0183::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf0183::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf0183`]
module"]
#[doc(alias = "PBMTXBUF0-183")]
pub type Pbmtxbuf0183 = crate::Reg<pbmtxbuf0183::Pbmtxbuf0183Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf0183;
#[doc = "PBMTXBUF0-184 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf0184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf0184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf0184`]
module"]
#[doc(alias = "PBMTXBUF0-184")]
pub type Pbmtxbuf0184 = crate::Reg<pbmtxbuf0184::Pbmtxbuf0184Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf0184;
#[doc = "PBMTXBUF0-185 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf0185::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf0185::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf0185`]
module"]
#[doc(alias = "PBMTXBUF0-185")]
pub type Pbmtxbuf0185 = crate::Reg<pbmtxbuf0185::Pbmtxbuf0185Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf0185;
#[doc = "PBMTXBUF0-186 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf0186::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf0186::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf0186`]
module"]
#[doc(alias = "PBMTXBUF0-186")]
pub type Pbmtxbuf0186 = crate::Reg<pbmtxbuf0186::Pbmtxbuf0186Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf0186;
#[doc = "PBMTXBUF0-187 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf0187::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf0187::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf0187`]
module"]
#[doc(alias = "PBMTXBUF0-187")]
pub type Pbmtxbuf0187 = crate::Reg<pbmtxbuf0187::Pbmtxbuf0187Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf0187;
#[doc = "PBMTXBUF0-188 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf0188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf0188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf0188`]
module"]
#[doc(alias = "PBMTXBUF0-188")]
pub type Pbmtxbuf0188 = crate::Reg<pbmtxbuf0188::Pbmtxbuf0188Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf0188;
#[doc = "PBMTXBUF0-189 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf0189::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf0189::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf0189`]
module"]
#[doc(alias = "PBMTXBUF0-189")]
pub type Pbmtxbuf0189 = crate::Reg<pbmtxbuf0189::Pbmtxbuf0189Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf0189;
#[doc = "PBMTXBUF0-1810 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf01810::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf01810::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf01810`]
module"]
#[doc(alias = "PBMTXBUF0-1810")]
pub type Pbmtxbuf01810 = crate::Reg<pbmtxbuf01810::Pbmtxbuf01810Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf01810;
#[doc = "PBMTXBUF0-1811 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf01811::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf01811::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf01811`]
module"]
#[doc(alias = "PBMTXBUF0-1811")]
pub type Pbmtxbuf01811 = crate::Reg<pbmtxbuf01811::Pbmtxbuf01811Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf01811;
#[doc = "PBMTXBUF0-1812 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf01812::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf01812::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf01812`]
module"]
#[doc(alias = "PBMTXBUF0-1812")]
pub type Pbmtxbuf01812 = crate::Reg<pbmtxbuf01812::Pbmtxbuf01812Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf01812;
#[doc = "PBMTXBUF0-1813 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf01813::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf01813::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf01813`]
module"]
#[doc(alias = "PBMTXBUF0-1813")]
pub type Pbmtxbuf01813 = crate::Reg<pbmtxbuf01813::Pbmtxbuf01813Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf01813;
#[doc = "PBMTXBUF0-1814 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf01814::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf01814::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf01814`]
module"]
#[doc(alias = "PBMTXBUF0-1814")]
pub type Pbmtxbuf01814 = crate::Reg<pbmtxbuf01814::Pbmtxbuf01814Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf01814;
#[doc = "PBMTXBUF0-1815 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf01815::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf01815::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf01815`]
module"]
#[doc(alias = "PBMTXBUF0-1815")]
pub type Pbmtxbuf01815 = crate::Reg<pbmtxbuf01815::Pbmtxbuf01815Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf01815;
#[doc = "PBMTXBUF0-1816 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf01816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf01816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf01816`]
module"]
#[doc(alias = "PBMTXBUF0-1816")]
pub type Pbmtxbuf01816 = crate::Reg<pbmtxbuf01816::Pbmtxbuf01816Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf01816;
#[doc = "PBMTXBUF0-1817 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf01817::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf01817::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf01817`]
module"]
#[doc(alias = "PBMTXBUF0-1817")]
pub type Pbmtxbuf01817 = crate::Reg<pbmtxbuf01817::Pbmtxbuf01817Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf01817;
#[doc = "PBMTXBUF0-1818 (rw) register accessor: Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)\n\nYou can [`read`](crate::Reg::read) this register and get [`pbmtxbuf01818::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbmtxbuf01818::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pbmtxbuf01818`]
module"]
#[doc(alias = "PBMTXBUF0-1818")]
pub type Pbmtxbuf01818 = crate::Reg<pbmtxbuf01818::Pbmtxbuf01818Spec>;
#[doc = "Peripheral Bus Master Transmit Buffer Register 0-18 (PBMTXBUF0-18)"]
pub mod pbmtxbuf01818;
#[doc = "FLASH_PRTR_BADDR0-150 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr0150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr0150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr0150`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-150")]
pub type FlashPrtrBaddr0150 = crate::Reg<flash_prtr_baddr0150::FlashPrtrBaddr0150Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr0150;
#[doc = "FLASH_PRTR_BADDR0-151 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr0151::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr0151::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr0151`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-151")]
pub type FlashPrtrBaddr0151 = crate::Reg<flash_prtr_baddr0151::FlashPrtrBaddr0151Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr0151;
#[doc = "FLASH_PRTR_BADDR0-152 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr0152::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr0152::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr0152`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-152")]
pub type FlashPrtrBaddr0152 = crate::Reg<flash_prtr_baddr0152::FlashPrtrBaddr0152Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr0152;
#[doc = "FLASH_PRTR_BADDR0-153 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr0153::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr0153::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr0153`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-153")]
pub type FlashPrtrBaddr0153 = crate::Reg<flash_prtr_baddr0153::FlashPrtrBaddr0153Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr0153;
#[doc = "FLASH_PRTR_BADDR0-154 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr0154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr0154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr0154`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-154")]
pub type FlashPrtrBaddr0154 = crate::Reg<flash_prtr_baddr0154::FlashPrtrBaddr0154Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr0154;
#[doc = "FLASH_PRTR_BADDR0-155 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr0155::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr0155::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr0155`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-155")]
pub type FlashPrtrBaddr0155 = crate::Reg<flash_prtr_baddr0155::FlashPrtrBaddr0155Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr0155;
#[doc = "FLASH_PRTR_BADDR0-156 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr0156::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr0156::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr0156`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-156")]
pub type FlashPrtrBaddr0156 = crate::Reg<flash_prtr_baddr0156::FlashPrtrBaddr0156Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr0156;
#[doc = "FLASH_PRTR_BADDR0-157 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr0157::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr0157::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr0157`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-157")]
pub type FlashPrtrBaddr0157 = crate::Reg<flash_prtr_baddr0157::FlashPrtrBaddr0157Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr0157;
#[doc = "FLASH_PRTR_BADDR0-158 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr0158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr0158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr0158`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-158")]
pub type FlashPrtrBaddr0158 = crate::Reg<flash_prtr_baddr0158::FlashPrtrBaddr0158Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr0158;
#[doc = "FLASH_PRTR_BADDR0-159 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr0159::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr0159::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr0159`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-159")]
pub type FlashPrtrBaddr0159 = crate::Reg<flash_prtr_baddr0159::FlashPrtrBaddr0159Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr0159;
#[doc = "FLASH_PRTR_BADDR0-1510 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr01510::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr01510::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr01510`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-1510")]
pub type FlashPrtrBaddr01510 = crate::Reg<flash_prtr_baddr01510::FlashPrtrBaddr01510Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr01510;
#[doc = "FLASH_PRTR_BADDR0-1511 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr01511::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr01511::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr01511`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-1511")]
pub type FlashPrtrBaddr01511 = crate::Reg<flash_prtr_baddr01511::FlashPrtrBaddr01511Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr01511;
#[doc = "FLASH_PRTR_BADDR0-1512 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr01512::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr01512::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr01512`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-1512")]
pub type FlashPrtrBaddr01512 = crate::Reg<flash_prtr_baddr01512::FlashPrtrBaddr01512Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr01512;
#[doc = "FLASH_PRTR_BADDR0-1513 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr01513::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr01513::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr01513`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-1513")]
pub type FlashPrtrBaddr01513 = crate::Reg<flash_prtr_baddr01513::FlashPrtrBaddr01513Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr01513;
#[doc = "FLASH_PRTR_BADDR0-1514 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr01514::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr01514::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr01514`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-1514")]
pub type FlashPrtrBaddr01514 = crate::Reg<flash_prtr_baddr01514::FlashPrtrBaddr01514Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr01514;
#[doc = "FLASH_PRTR_BADDR0-1515 (rw) register accessor: Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_baddr01515::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_baddr01515::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_baddr01515`]
module"]
#[doc(alias = "FLASH_PRTR_BADDR0-1515")]
pub type FlashPrtrBaddr01515 = crate::Reg<flash_prtr_baddr01515::FlashPrtrBaddr01515Spec>;
#[doc = "Flash Protection Range Base Address Register 0-15 (FLASH_PRTR_BADDR0-15)"]
pub mod flash_prtr_baddr01515;
#[doc = "FLASH_PRTR_TADDR0-150 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr0150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr0150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr0150`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-150")]
pub type FlashPrtrTaddr0150 = crate::Reg<flash_prtr_taddr0150::FlashPrtrTaddr0150Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr0150;
#[doc = "FLASH_PRTR_TADDR0-151 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr0151::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr0151::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr0151`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-151")]
pub type FlashPrtrTaddr0151 = crate::Reg<flash_prtr_taddr0151::FlashPrtrTaddr0151Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr0151;
#[doc = "FLASH_PRTR_TADDR0-152 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr0152::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr0152::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr0152`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-152")]
pub type FlashPrtrTaddr0152 = crate::Reg<flash_prtr_taddr0152::FlashPrtrTaddr0152Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr0152;
#[doc = "FLASH_PRTR_TADDR0-153 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr0153::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr0153::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr0153`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-153")]
pub type FlashPrtrTaddr0153 = crate::Reg<flash_prtr_taddr0153::FlashPrtrTaddr0153Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr0153;
#[doc = "FLASH_PRTR_TADDR0-154 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr0154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr0154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr0154`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-154")]
pub type FlashPrtrTaddr0154 = crate::Reg<flash_prtr_taddr0154::FlashPrtrTaddr0154Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr0154;
#[doc = "FLASH_PRTR_TADDR0-155 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr0155::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr0155::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr0155`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-155")]
pub type FlashPrtrTaddr0155 = crate::Reg<flash_prtr_taddr0155::FlashPrtrTaddr0155Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr0155;
#[doc = "FLASH_PRTR_TADDR0-156 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr0156::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr0156::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr0156`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-156")]
pub type FlashPrtrTaddr0156 = crate::Reg<flash_prtr_taddr0156::FlashPrtrTaddr0156Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr0156;
#[doc = "FLASH_PRTR_TADDR0-157 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr0157::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr0157::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr0157`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-157")]
pub type FlashPrtrTaddr0157 = crate::Reg<flash_prtr_taddr0157::FlashPrtrTaddr0157Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr0157;
#[doc = "FLASH_PRTR_TADDR0-158 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr0158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr0158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr0158`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-158")]
pub type FlashPrtrTaddr0158 = crate::Reg<flash_prtr_taddr0158::FlashPrtrTaddr0158Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr0158;
#[doc = "FLASH_PRTR_TADDR0-159 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr0159::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr0159::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr0159`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-159")]
pub type FlashPrtrTaddr0159 = crate::Reg<flash_prtr_taddr0159::FlashPrtrTaddr0159Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr0159;
#[doc = "FLASH_PRTR_TADDR0-1510 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr01510::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr01510::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr01510`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-1510")]
pub type FlashPrtrTaddr01510 = crate::Reg<flash_prtr_taddr01510::FlashPrtrTaddr01510Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr01510;
#[doc = "FLASH_PRTR_TADDR0-1511 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr01511::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr01511::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr01511`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-1511")]
pub type FlashPrtrTaddr01511 = crate::Reg<flash_prtr_taddr01511::FlashPrtrTaddr01511Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr01511;
#[doc = "FLASH_PRTR_TADDR0-1512 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr01512::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr01512::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr01512`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-1512")]
pub type FlashPrtrTaddr01512 = crate::Reg<flash_prtr_taddr01512::FlashPrtrTaddr01512Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr01512;
#[doc = "FLASH_PRTR_TADDR0-1513 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr01513::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr01513::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr01513`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-1513")]
pub type FlashPrtrTaddr01513 = crate::Reg<flash_prtr_taddr01513::FlashPrtrTaddr01513Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr01513;
#[doc = "FLASH_PRTR_TADDR0-1514 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr01514::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr01514::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr01514`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-1514")]
pub type FlashPrtrTaddr01514 = crate::Reg<flash_prtr_taddr01514::FlashPrtrTaddr01514Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr01514;
#[doc = "FLASH_PRTR_TADDR0-1515 (rw) register accessor: Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_prtr_taddr01515::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_prtr_taddr01515::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_prtr_taddr01515`]
module"]
#[doc(alias = "FLASH_PRTR_TADDR0-1515")]
pub type FlashPrtrTaddr01515 = crate::Reg<flash_prtr_taddr01515::FlashPrtrTaddr01515Spec>;
#[doc = "Flash Protection Range Top Address Register 0-15 (FLASH_PRTR_TADDR0-15)"]
pub mod flash_prtr_taddr01515;
#[doc = "FLASH_RNG_TAG_OVR0-150 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr0150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr0150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr0150`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-150")]
pub type FlashRngTagOvr0150 = crate::Reg<flash_rng_tag_ovr0150::FlashRngTagOvr0150Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr0150;
#[doc = "FLASH_RNG_TAG_OVR0-151 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr0151::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr0151::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr0151`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-151")]
pub type FlashRngTagOvr0151 = crate::Reg<flash_rng_tag_ovr0151::FlashRngTagOvr0151Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr0151;
#[doc = "FLASH_RNG_TAG_OVR0-152 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr0152::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr0152::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr0152`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-152")]
pub type FlashRngTagOvr0152 = crate::Reg<flash_rng_tag_ovr0152::FlashRngTagOvr0152Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr0152;
#[doc = "FLASH_RNG_TAG_OVR0-153 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr0153::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr0153::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr0153`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-153")]
pub type FlashRngTagOvr0153 = crate::Reg<flash_rng_tag_ovr0153::FlashRngTagOvr0153Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr0153;
#[doc = "FLASH_RNG_TAG_OVR0-154 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr0154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr0154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr0154`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-154")]
pub type FlashRngTagOvr0154 = crate::Reg<flash_rng_tag_ovr0154::FlashRngTagOvr0154Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr0154;
#[doc = "FLASH_RNG_TAG_OVR0-155 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr0155::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr0155::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr0155`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-155")]
pub type FlashRngTagOvr0155 = crate::Reg<flash_rng_tag_ovr0155::FlashRngTagOvr0155Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr0155;
#[doc = "FLASH_RNG_TAG_OVR0-156 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr0156::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr0156::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr0156`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-156")]
pub type FlashRngTagOvr0156 = crate::Reg<flash_rng_tag_ovr0156::FlashRngTagOvr0156Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr0156;
#[doc = "FLASH_RNG_TAG_OVR0-157 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr0157::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr0157::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr0157`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-157")]
pub type FlashRngTagOvr0157 = crate::Reg<flash_rng_tag_ovr0157::FlashRngTagOvr0157Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr0157;
#[doc = "FLASH_RNG_TAG_OVR0-158 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr0158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr0158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr0158`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-158")]
pub type FlashRngTagOvr0158 = crate::Reg<flash_rng_tag_ovr0158::FlashRngTagOvr0158Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr0158;
#[doc = "FLASH_RNG_TAG_OVR0-159 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr0159::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr0159::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr0159`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-159")]
pub type FlashRngTagOvr0159 = crate::Reg<flash_rng_tag_ovr0159::FlashRngTagOvr0159Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr0159;
#[doc = "FLASH_RNG_TAG_OVR0-1510 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr01510::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr01510::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr01510`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-1510")]
pub type FlashRngTagOvr01510 = crate::Reg<flash_rng_tag_ovr01510::FlashRngTagOvr01510Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr01510;
#[doc = "FLASH_RNG_TAG_OVR0-1511 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr01511::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr01511::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr01511`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-1511")]
pub type FlashRngTagOvr01511 = crate::Reg<flash_rng_tag_ovr01511::FlashRngTagOvr01511Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr01511;
#[doc = "FLASH_RNG_TAG_OVR0-1512 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr01512::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr01512::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr01512`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-1512")]
pub type FlashRngTagOvr01512 = crate::Reg<flash_rng_tag_ovr01512::FlashRngTagOvr01512Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr01512;
#[doc = "FLASH_RNG_TAG_OVR0-1513 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr01513::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr01513::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr01513`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-1513")]
pub type FlashRngTagOvr01513 = crate::Reg<flash_rng_tag_ovr01513::FlashRngTagOvr01513Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr01513;
#[doc = "FLASH_RNG_TAG_OVR0-1514 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr01514::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr01514::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr01514`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-1514")]
pub type FlashRngTagOvr01514 = crate::Reg<flash_rng_tag_ovr01514::FlashRngTagOvr01514Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr01514;
#[doc = "FLASH_RNG_TAG_OVR0-1515 (rw) register accessor: Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_rng_tag_ovr01515::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_rng_tag_ovr01515::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flash_rng_tag_ovr01515`]
module"]
#[doc(alias = "FLASH_RNG_TAG_OVR0-1515")]
pub type FlashRngTagOvr01515 = crate::Reg<flash_rng_tag_ovr01515::FlashRngTagOvr01515Spec>;
#[doc = "Flash Range Tag Override Register 0-15 (FLASH_RNG_TAG_OVR0-15)"]
pub mod flash_rng_tag_ovr01515;
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
#[doc = "PSMRXBUF0-170 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf0170`]
module"]
#[doc(alias = "PSMRXBUF0-170")]
pub type Psmrxbuf0170 = crate::Reg<psmrxbuf0170::Psmrxbuf0170Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf0170;
#[doc = "PSMRXBUF0-171 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0171::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0171::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf0171`]
module"]
#[doc(alias = "PSMRXBUF0-171")]
pub type Psmrxbuf0171 = crate::Reg<psmrxbuf0171::Psmrxbuf0171Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf0171;
#[doc = "PSMRXBUF0-172 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0172::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0172::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf0172`]
module"]
#[doc(alias = "PSMRXBUF0-172")]
pub type Psmrxbuf0172 = crate::Reg<psmrxbuf0172::Psmrxbuf0172Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf0172;
#[doc = "PSMRXBUF0-173 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0173::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0173::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf0173`]
module"]
#[doc(alias = "PSMRXBUF0-173")]
pub type Psmrxbuf0173 = crate::Reg<psmrxbuf0173::Psmrxbuf0173Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf0173;
#[doc = "PSMRXBUF0-174 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf0174`]
module"]
#[doc(alias = "PSMRXBUF0-174")]
pub type Psmrxbuf0174 = crate::Reg<psmrxbuf0174::Psmrxbuf0174Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf0174;
#[doc = "PSMRXBUF0-175 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0175::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0175::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf0175`]
module"]
#[doc(alias = "PSMRXBUF0-175")]
pub type Psmrxbuf0175 = crate::Reg<psmrxbuf0175::Psmrxbuf0175Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf0175;
#[doc = "PSMRXBUF0-176 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0176::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0176::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf0176`]
module"]
#[doc(alias = "PSMRXBUF0-176")]
pub type Psmrxbuf0176 = crate::Reg<psmrxbuf0176::Psmrxbuf0176Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf0176;
#[doc = "PSMRXBUF0-177 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0177::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0177::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf0177`]
module"]
#[doc(alias = "PSMRXBUF0-177")]
pub type Psmrxbuf0177 = crate::Reg<psmrxbuf0177::Psmrxbuf0177Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf0177;
#[doc = "PSMRXBUF0-178 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf0178`]
module"]
#[doc(alias = "PSMRXBUF0-178")]
pub type Psmrxbuf0178 = crate::Reg<psmrxbuf0178::Psmrxbuf0178Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf0178;
#[doc = "PSMRXBUF0-179 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf0179::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf0179::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf0179`]
module"]
#[doc(alias = "PSMRXBUF0-179")]
pub type Psmrxbuf0179 = crate::Reg<psmrxbuf0179::Psmrxbuf0179Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf0179;
#[doc = "PSMRXBUF0-1710 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf01710::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf01710::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf01710`]
module"]
#[doc(alias = "PSMRXBUF0-1710")]
pub type Psmrxbuf01710 = crate::Reg<psmrxbuf01710::Psmrxbuf01710Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf01710;
#[doc = "PSMRXBUF0-1711 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf01711::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf01711::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf01711`]
module"]
#[doc(alias = "PSMRXBUF0-1711")]
pub type Psmrxbuf01711 = crate::Reg<psmrxbuf01711::Psmrxbuf01711Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf01711;
#[doc = "PSMRXBUF0-1712 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf01712::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf01712::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf01712`]
module"]
#[doc(alias = "PSMRXBUF0-1712")]
pub type Psmrxbuf01712 = crate::Reg<psmrxbuf01712::Psmrxbuf01712Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf01712;
#[doc = "PSMRXBUF0-1713 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf01713::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf01713::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf01713`]
module"]
#[doc(alias = "PSMRXBUF0-1713")]
pub type Psmrxbuf01713 = crate::Reg<psmrxbuf01713::Psmrxbuf01713Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf01713;
#[doc = "PSMRXBUF0-1714 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf01714::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf01714::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf01714`]
module"]
#[doc(alias = "PSMRXBUF0-1714")]
pub type Psmrxbuf01714 = crate::Reg<psmrxbuf01714::Psmrxbuf01714Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf01714;
#[doc = "PSMRXBUF0-1715 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf01715::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf01715::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf01715`]
module"]
#[doc(alias = "PSMRXBUF0-1715")]
pub type Psmrxbuf01715 = crate::Reg<psmrxbuf01715::Psmrxbuf01715Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf01715;
#[doc = "PSMRXBUF0-1716 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf01716::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf01716::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf01716`]
module"]
#[doc(alias = "PSMRXBUF0-1716")]
pub type Psmrxbuf01716 = crate::Reg<psmrxbuf01716::Psmrxbuf01716Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf01716;
#[doc = "PSMRXBUF0-1717 (rw) register accessor: Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)\n\nYou can [`read`](crate::Reg::read) this register and get [`psmrxbuf01717::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmrxbuf01717::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmrxbuf01717`]
module"]
#[doc(alias = "PSMRXBUF0-1717")]
pub type Psmrxbuf01717 = crate::Reg<psmrxbuf01717::Psmrxbuf01717Spec>;
#[doc = "Peripheral Target Message Receive Buffer Register 0-17 (PSMRXBUF0-17)"]
pub mod psmrxbuf01717;
