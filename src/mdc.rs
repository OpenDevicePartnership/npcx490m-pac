#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0a],
    head_basel: HeadBasel,
    head_baseh: HeadBaseh,
    _reserved2: [u8; 0x12],
    mem_par_en: MemParEn,
    _reserved3: [u8; 0x01],
    fwsc: Fwsc,
    _reserved4: [u8; 0x0b],
    ram_lk_sts: RamLkSts,
    ram_lk_ctl: RamLkCtl,
    _reserved6: [u8; 0x19],
    ram_wpr_adr_20: RamWprAdr20,
    ram_wpr_adr_28: RamWprAdr28,
    ram_wpr_adr_30: RamWprAdr30,
    ram_wpr_adr_38: RamWprAdr38,
    ram_wpr_adr_40: RamWprAdr40,
    ram_wpr_adr_48: RamWprAdr48,
    ram_wpr_adr_50: RamWprAdr50,
    ram_wpr_adr_58: RamWprAdr58,
    ram_wpr_adr_60: RamWprAdr60,
    ram_wpr_adr_68: RamWprAdr68,
    ram_wpr_adr_70: RamWprAdr70,
    ram_wpr_adr_78: RamWprAdr78,
    ram_wpr_adr_80: RamWprAdr80,
    ram_wpr_adr_88: RamWprAdr88,
    ram_wpr_adr_90: RamWprAdr90,
    ram_wpr_adr_98: RamWprAdr98,
    ram_wpr_adr_a0: RamWprAdrA0,
    ram_wpr_adr_a8: RamWprAdrA8,
    ram_wpr_adr_b0: RamWprAdrB0,
    ram_wpr_adr_b8: RamWprAdrB8,
    ram_wpr_adr_c0: RamWprAdrC0,
    ram_wpr_adr_c8: RamWprAdrC8,
    ram_wpr_adr_d0: RamWprAdrD0,
    ram_wpr_adr_d8: RamWprAdrD8,
    _reserved30: [u8; 0x08],
    ram_fpr_adr_20: RamFprAdr20,
    ram_fpr_adr_28: RamFprAdr28,
    ram_fpr_adr_30: RamFprAdr30,
    ram_fpr_adr_38: RamFprAdr38,
    ram_fpr_adr_40: RamFprAdr40,
    ram_fpr_adr_48: RamFprAdr48,
    ram_fpr_adr_50: RamFprAdr50,
    ram_fpr_adr_58: RamFprAdr58,
    ram_fpr_adr_60: RamFprAdr60,
    ram_fpr_adr_68: RamFprAdr68,
    ram_fpr_adr_70: RamFprAdr70,
    ram_fpr_adr_78: RamFprAdr78,
    ram_fpr_adr_80: RamFprAdr80,
    ram_fpr_adr_88: RamFprAdr88,
    ram_fpr_adr_90: RamFprAdr90,
    ram_fpr_adr_98: RamFprAdr98,
    ram_fpr_adr_a0: RamFprAdrA0,
    ram_fpr_adr_a8: RamFprAdrA8,
    ram_fpr_adr_b0: RamFprAdrB0,
    ram_fpr_adr_b8: RamFprAdrB8,
    ram_fpr_adr_c0: RamFprAdrC0,
    ram_fpr_adr_c8: RamFprAdrC8,
    ram_fpr_adr_d0: RamFprAdrD0,
    ram_fpr_adr_d8: RamFprAdrD8,
    _reserved54: [u8; 0xc8],
    ram_wplk_adr_20: RamWplkAdr20,
    ram_wplk_adr_28: RamWplkAdr28,
    ram_wplk_adr_30: RamWplkAdr30,
    ram_wplk_adr_38: RamWplkAdr38,
    ram_wplk_adr_40: RamWplkAdr40,
    ram_wplk_adr_48: RamWplkAdr48,
    ram_wplk_adr_50: RamWplkAdr50,
    ram_wplk_adr_58: RamWplkAdr58,
    ram_wplk_adr_60: RamWplkAdr60,
    ram_wplk_adr_68: RamWplkAdr68,
    ram_wplk_adr_70: RamWplkAdr70,
    ram_wplk_adr_78: RamWplkAdr78,
    ram_wplk_adr_80: RamWplkAdr80,
    ram_wplk_adr_88: RamWplkAdr88,
    ram_wplk_adr_90: RamWplkAdr90,
    ram_wplk_adr_98: RamWplkAdr98,
    ram_wplk_adr_a0: RamWplkAdrA0,
    ram_wplk_adr_a8: RamWplkAdrA8,
    ram_wplk_adr_b0: RamWplkAdrB0,
    ram_wplk_adr_b8: RamWplkAdrB8,
    ram_wplk_adr_c0: RamWplkAdrC0,
    ram_wplk_adr_c8: RamWplkAdrC8,
    ram_wplk_adr_d0: RamWplkAdrD0,
    ram_wplk_adr_d8: RamWplkAdrD8,
    _reserved78: [u8; 0x08],
    ram_fplk_adr_20: RamFplkAdr20,
    ram_fplk_adr_28: RamFplkAdr28,
    ram_fplk_adr_30: RamFplkAdr30,
    ram_fplk_adr_38: RamFplkAdr38,
    ram_fplk_adr_40: RamFplkAdr40,
    ram_fplk_adr_48: RamFplkAdr48,
    ram_fplk_adr_50: RamFplkAdr50,
    ram_fplk_adr_58: RamFplkAdr58,
    ram_fplk_adr_60: RamFplkAdr60,
    ram_fplk_adr_68: RamFplkAdr68,
    ram_fplk_adr_70: RamFplkAdr70,
    ram_fplk_adr_78: RamFplkAdr78,
    ram_fplk_adr_80: RamFplkAdr80,
    ram_fplk_adr_88: RamFplkAdr88,
    ram_fplk_adr_90: RamFplkAdr90,
    ram_fplk_adr_98: RamFplkAdr98,
    ram_fplk_adr_a0: RamFplkAdrA0,
    ram_fplk_adr_a8: RamFplkAdrA8,
    ram_fplk_adr_b0: RamFplkAdrB0,
    ram_fplk_adr_b8: RamFplkAdrB8,
    ram_fplk_adr_c0: RamFplkAdrC0,
    ram_fplk_adr_c8: RamFplkAdrC8,
    ram_fplk_adr_d0: RamFplkAdrD0,
    ram_fplk_adr_d8: RamFplkAdrD8,
}
impl RegisterBlock {
    #[doc = "0x0a - Header Base Low Register (HEAD_BASEL)"]
    #[inline(always)]
    pub const fn head_basel(&self) -> &HeadBasel {
        &self.head_basel
    }
    #[doc = "0x0c - Header Base High Register (HEAD_BASEH)"]
    #[inline(always)]
    pub const fn head_baseh(&self) -> &HeadBaseh {
        &self.head_baseh
    }
    #[doc = "0x20 - Memory Parity Check Enable Register (MEM_PAR_EN)"]
    #[inline(always)]
    pub const fn mem_par_en(&self) -> &MemParEn {
        &self.mem_par_en
    }
    #[doc = "0x22 - Firmware Scratch Register (FWSC)"]
    #[inline(always)]
    pub const fn fwsc(&self) -> &Fwsc {
        &self.fwsc
    }
    #[doc = "0x2e - RAM Lock Status Register (RAM_LK_STS)"]
    #[inline(always)]
    pub const fn ram_lk_sts(&self) -> &RamLkSts {
        &self.ram_lk_sts
    }
    #[doc = "0x2f - RAM Lock Control Register (RAM_LK_CTL)"]
    #[inline(always)]
    pub const fn ram_lk_ctl(&self) -> &RamLkCtl {
        &self.ram_lk_ctl
    }
    #[doc = "0x49 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_20(&self) -> &RamWprAdr20 {
        &self.ram_wpr_adr_20
    }
    #[doc = "0x4a - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_28(&self) -> &RamWprAdr28 {
        &self.ram_wpr_adr_28
    }
    #[doc = "0x4b - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_30(&self) -> &RamWprAdr30 {
        &self.ram_wpr_adr_30
    }
    #[doc = "0x4c - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_38(&self) -> &RamWprAdr38 {
        &self.ram_wpr_adr_38
    }
    #[doc = "0x4d - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_40(&self) -> &RamWprAdr40 {
        &self.ram_wpr_adr_40
    }
    #[doc = "0x4e - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_48(&self) -> &RamWprAdr48 {
        &self.ram_wpr_adr_48
    }
    #[doc = "0x4f - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_50(&self) -> &RamWprAdr50 {
        &self.ram_wpr_adr_50
    }
    #[doc = "0x50 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_58(&self) -> &RamWprAdr58 {
        &self.ram_wpr_adr_58
    }
    #[doc = "0x51 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_60(&self) -> &RamWprAdr60 {
        &self.ram_wpr_adr_60
    }
    #[doc = "0x52 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_68(&self) -> &RamWprAdr68 {
        &self.ram_wpr_adr_68
    }
    #[doc = "0x53 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_70(&self) -> &RamWprAdr70 {
        &self.ram_wpr_adr_70
    }
    #[doc = "0x54 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_78(&self) -> &RamWprAdr78 {
        &self.ram_wpr_adr_78
    }
    #[doc = "0x55 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_80(&self) -> &RamWprAdr80 {
        &self.ram_wpr_adr_80
    }
    #[doc = "0x56 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_88(&self) -> &RamWprAdr88 {
        &self.ram_wpr_adr_88
    }
    #[doc = "0x57 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_90(&self) -> &RamWprAdr90 {
        &self.ram_wpr_adr_90
    }
    #[doc = "0x58 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_98(&self) -> &RamWprAdr98 {
        &self.ram_wpr_adr_98
    }
    #[doc = "0x59 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_a0(&self) -> &RamWprAdrA0 {
        &self.ram_wpr_adr_a0
    }
    #[doc = "0x5a - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_a8(&self) -> &RamWprAdrA8 {
        &self.ram_wpr_adr_a8
    }
    #[doc = "0x5b - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_b0(&self) -> &RamWprAdrB0 {
        &self.ram_wpr_adr_b0
    }
    #[doc = "0x5c - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_b8(&self) -> &RamWprAdrB8 {
        &self.ram_wpr_adr_b8
    }
    #[doc = "0x5d - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_c0(&self) -> &RamWprAdrC0 {
        &self.ram_wpr_adr_c0
    }
    #[doc = "0x5e - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_c8(&self) -> &RamWprAdrC8 {
        &self.ram_wpr_adr_c8
    }
    #[doc = "0x5f - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_d0(&self) -> &RamWprAdrD0 {
        &self.ram_wpr_adr_d0
    }
    #[doc = "0x60 - RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wpr_adr_d8(&self) -> &RamWprAdrD8 {
        &self.ram_wpr_adr_d8
    }
    #[doc = "0x69 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_20(&self) -> &RamFprAdr20 {
        &self.ram_fpr_adr_20
    }
    #[doc = "0x6a - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_28(&self) -> &RamFprAdr28 {
        &self.ram_fpr_adr_28
    }
    #[doc = "0x6b - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_30(&self) -> &RamFprAdr30 {
        &self.ram_fpr_adr_30
    }
    #[doc = "0x6c - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_38(&self) -> &RamFprAdr38 {
        &self.ram_fpr_adr_38
    }
    #[doc = "0x6d - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_40(&self) -> &RamFprAdr40 {
        &self.ram_fpr_adr_40
    }
    #[doc = "0x6e - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_48(&self) -> &RamFprAdr48 {
        &self.ram_fpr_adr_48
    }
    #[doc = "0x6f - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_50(&self) -> &RamFprAdr50 {
        &self.ram_fpr_adr_50
    }
    #[doc = "0x70 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_58(&self) -> &RamFprAdr58 {
        &self.ram_fpr_adr_58
    }
    #[doc = "0x71 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_60(&self) -> &RamFprAdr60 {
        &self.ram_fpr_adr_60
    }
    #[doc = "0x72 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_68(&self) -> &RamFprAdr68 {
        &self.ram_fpr_adr_68
    }
    #[doc = "0x73 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_70(&self) -> &RamFprAdr70 {
        &self.ram_fpr_adr_70
    }
    #[doc = "0x74 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_78(&self) -> &RamFprAdr78 {
        &self.ram_fpr_adr_78
    }
    #[doc = "0x75 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_80(&self) -> &RamFprAdr80 {
        &self.ram_fpr_adr_80
    }
    #[doc = "0x76 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_88(&self) -> &RamFprAdr88 {
        &self.ram_fpr_adr_88
    }
    #[doc = "0x77 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_90(&self) -> &RamFprAdr90 {
        &self.ram_fpr_adr_90
    }
    #[doc = "0x78 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_98(&self) -> &RamFprAdr98 {
        &self.ram_fpr_adr_98
    }
    #[doc = "0x79 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_a0(&self) -> &RamFprAdrA0 {
        &self.ram_fpr_adr_a0
    }
    #[doc = "0x7a - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_a8(&self) -> &RamFprAdrA8 {
        &self.ram_fpr_adr_a8
    }
    #[doc = "0x7b - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_b0(&self) -> &RamFprAdrB0 {
        &self.ram_fpr_adr_b0
    }
    #[doc = "0x7c - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_b8(&self) -> &RamFprAdrB8 {
        &self.ram_fpr_adr_b8
    }
    #[doc = "0x7d - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_c0(&self) -> &RamFprAdrC0 {
        &self.ram_fpr_adr_c0
    }
    #[doc = "0x7e - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_c8(&self) -> &RamFprAdrC8 {
        &self.ram_fpr_adr_c8
    }
    #[doc = "0x7f - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_d0(&self) -> &RamFprAdrD0 {
        &self.ram_fpr_adr_d0
    }
    #[doc = "0x80 - RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fpr_adr_d8(&self) -> &RamFprAdrD8 {
        &self.ram_fpr_adr_d8
    }
    #[doc = "0x149 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_20(&self) -> &RamWplkAdr20 {
        &self.ram_wplk_adr_20
    }
    #[doc = "0x14a - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_28(&self) -> &RamWplkAdr28 {
        &self.ram_wplk_adr_28
    }
    #[doc = "0x14b - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_30(&self) -> &RamWplkAdr30 {
        &self.ram_wplk_adr_30
    }
    #[doc = "0x14c - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_38(&self) -> &RamWplkAdr38 {
        &self.ram_wplk_adr_38
    }
    #[doc = "0x14d - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_40(&self) -> &RamWplkAdr40 {
        &self.ram_wplk_adr_40
    }
    #[doc = "0x14e - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_48(&self) -> &RamWplkAdr48 {
        &self.ram_wplk_adr_48
    }
    #[doc = "0x14f - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_50(&self) -> &RamWplkAdr50 {
        &self.ram_wplk_adr_50
    }
    #[doc = "0x150 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_58(&self) -> &RamWplkAdr58 {
        &self.ram_wplk_adr_58
    }
    #[doc = "0x151 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_60(&self) -> &RamWplkAdr60 {
        &self.ram_wplk_adr_60
    }
    #[doc = "0x152 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_68(&self) -> &RamWplkAdr68 {
        &self.ram_wplk_adr_68
    }
    #[doc = "0x153 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_70(&self) -> &RamWplkAdr70 {
        &self.ram_wplk_adr_70
    }
    #[doc = "0x154 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_78(&self) -> &RamWplkAdr78 {
        &self.ram_wplk_adr_78
    }
    #[doc = "0x155 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_80(&self) -> &RamWplkAdr80 {
        &self.ram_wplk_adr_80
    }
    #[doc = "0x156 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_88(&self) -> &RamWplkAdr88 {
        &self.ram_wplk_adr_88
    }
    #[doc = "0x157 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_90(&self) -> &RamWplkAdr90 {
        &self.ram_wplk_adr_90
    }
    #[doc = "0x158 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_98(&self) -> &RamWplkAdr98 {
        &self.ram_wplk_adr_98
    }
    #[doc = "0x159 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_a0(&self) -> &RamWplkAdrA0 {
        &self.ram_wplk_adr_a0
    }
    #[doc = "0x15a - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_a8(&self) -> &RamWplkAdrA8 {
        &self.ram_wplk_adr_a8
    }
    #[doc = "0x15b - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_b0(&self) -> &RamWplkAdrB0 {
        &self.ram_wplk_adr_b0
    }
    #[doc = "0x15c - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_b8(&self) -> &RamWplkAdrB8 {
        &self.ram_wplk_adr_b8
    }
    #[doc = "0x15d - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_c0(&self) -> &RamWplkAdrC0 {
        &self.ram_wplk_adr_c0
    }
    #[doc = "0x15e - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_c8(&self) -> &RamWplkAdrC8 {
        &self.ram_wplk_adr_c8
    }
    #[doc = "0x15f - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_d0(&self) -> &RamWplkAdrD0 {
        &self.ram_wplk_adr_d0
    }
    #[doc = "0x160 - RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_wplk_adr_d8(&self) -> &RamWplkAdrD8 {
        &self.ram_wplk_adr_d8
    }
    #[doc = "0x169 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_20(&self) -> &RamFplkAdr20 {
        &self.ram_fplk_adr_20
    }
    #[doc = "0x16a - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_28(&self) -> &RamFplkAdr28 {
        &self.ram_fplk_adr_28
    }
    #[doc = "0x16b - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_30(&self) -> &RamFplkAdr30 {
        &self.ram_fplk_adr_30
    }
    #[doc = "0x16c - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_38(&self) -> &RamFplkAdr38 {
        &self.ram_fplk_adr_38
    }
    #[doc = "0x16d - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_40(&self) -> &RamFplkAdr40 {
        &self.ram_fplk_adr_40
    }
    #[doc = "0x16e - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_48(&self) -> &RamFplkAdr48 {
        &self.ram_fplk_adr_48
    }
    #[doc = "0x16f - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_50(&self) -> &RamFplkAdr50 {
        &self.ram_fplk_adr_50
    }
    #[doc = "0x170 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_58(&self) -> &RamFplkAdr58 {
        &self.ram_fplk_adr_58
    }
    #[doc = "0x171 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_60(&self) -> &RamFplkAdr60 {
        &self.ram_fplk_adr_60
    }
    #[doc = "0x172 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_68(&self) -> &RamFplkAdr68 {
        &self.ram_fplk_adr_68
    }
    #[doc = "0x173 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_70(&self) -> &RamFplkAdr70 {
        &self.ram_fplk_adr_70
    }
    #[doc = "0x174 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_78(&self) -> &RamFplkAdr78 {
        &self.ram_fplk_adr_78
    }
    #[doc = "0x175 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_80(&self) -> &RamFplkAdr80 {
        &self.ram_fplk_adr_80
    }
    #[doc = "0x176 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_88(&self) -> &RamFplkAdr88 {
        &self.ram_fplk_adr_88
    }
    #[doc = "0x177 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_90(&self) -> &RamFplkAdr90 {
        &self.ram_fplk_adr_90
    }
    #[doc = "0x178 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_98(&self) -> &RamFplkAdr98 {
        &self.ram_fplk_adr_98
    }
    #[doc = "0x179 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_a0(&self) -> &RamFplkAdrA0 {
        &self.ram_fplk_adr_a0
    }
    #[doc = "0x17a - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_a8(&self) -> &RamFplkAdrA8 {
        &self.ram_fplk_adr_a8
    }
    #[doc = "0x17b - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_b0(&self) -> &RamFplkAdrB0 {
        &self.ram_fplk_adr_b0
    }
    #[doc = "0x17c - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_b8(&self) -> &RamFplkAdrB8 {
        &self.ram_fplk_adr_b8
    }
    #[doc = "0x17d - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_c0(&self) -> &RamFplkAdrC0 {
        &self.ram_fplk_adr_c0
    }
    #[doc = "0x17e - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_c8(&self) -> &RamFplkAdrC8 {
        &self.ram_fplk_adr_c8
    }
    #[doc = "0x17f - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_d0(&self) -> &RamFplkAdrD0 {
        &self.ram_fplk_adr_d0
    }
    #[doc = "0x180 - RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
    #[inline(always)]
    pub const fn ram_fplk_adr_d8(&self) -> &RamFplkAdrD8 {
        &self.ram_fplk_adr_d8
    }
}
#[doc = "HEAD_BASEL (rw) register accessor: Header Base Low Register (HEAD_BASEL)\n\nYou can [`read`](crate::Reg::read) this register and get [`head_basel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`head_basel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@head_basel`]
module"]
#[doc(alias = "HEAD_BASEL")]
pub type HeadBasel = crate::Reg<head_basel::HeadBaselSpec>;
#[doc = "Header Base Low Register (HEAD_BASEL)"]
pub mod head_basel;
#[doc = "HEAD_BASEH (rw) register accessor: Header Base High Register (HEAD_BASEH)\n\nYou can [`read`](crate::Reg::read) this register and get [`head_baseh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`head_baseh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@head_baseh`]
module"]
#[doc(alias = "HEAD_BASEH")]
pub type HeadBaseh = crate::Reg<head_baseh::HeadBasehSpec>;
#[doc = "Header Base High Register (HEAD_BASEH)"]
pub mod head_baseh;
#[doc = "MEM_PAR_EN (rw) register accessor: Memory Parity Check Enable Register (MEM_PAR_EN)\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_par_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_par_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_par_en`]
module"]
#[doc(alias = "MEM_PAR_EN")]
pub type MemParEn = crate::Reg<mem_par_en::MemParEnSpec>;
#[doc = "Memory Parity Check Enable Register (MEM_PAR_EN)"]
pub mod mem_par_en;
#[doc = "FWSC (rw) register accessor: Firmware Scratch Register (FWSC)\n\nYou can [`read`](crate::Reg::read) this register and get [`fwsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwsc`]
module"]
#[doc(alias = "FWSC")]
pub type Fwsc = crate::Reg<fwsc::FwscSpec>;
#[doc = "Firmware Scratch Register (FWSC)"]
pub mod fwsc;
#[doc = "RAM_LK_STS (rw) register accessor: RAM Lock Status Register (RAM_LK_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_lk_sts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_lk_sts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_lk_sts`]
module"]
#[doc(alias = "RAM_LK_STS")]
pub type RamLkSts = crate::Reg<ram_lk_sts::RamLkStsSpec>;
#[doc = "RAM Lock Status Register (RAM_LK_STS)"]
pub mod ram_lk_sts;
#[doc = "RAM_LK_CTL (rw) register accessor: RAM Lock Control Register (RAM_LK_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_lk_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_lk_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_lk_ctl`]
module"]
#[doc(alias = "RAM_LK_CTL")]
pub type RamLkCtl = crate::Reg<ram_lk_ctl::RamLkCtlSpec>;
#[doc = "RAM Lock Control Register (RAM_LK_CTL)"]
pub mod ram_lk_ctl;
#[doc = "RAM_WPR_ADR_20 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_20`]
module"]
#[doc(alias = "RAM_WPR_ADR_20")]
pub type RamWprAdr20 = crate::Reg<ram_wpr_adr_20::RamWprAdr20Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_20;
#[doc = "RAM_WPR_ADR_28 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_28`]
module"]
#[doc(alias = "RAM_WPR_ADR_28")]
pub type RamWprAdr28 = crate::Reg<ram_wpr_adr_28::RamWprAdr28Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_28;
#[doc = "RAM_WPR_ADR_30 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_30`]
module"]
#[doc(alias = "RAM_WPR_ADR_30")]
pub type RamWprAdr30 = crate::Reg<ram_wpr_adr_30::RamWprAdr30Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_30;
#[doc = "RAM_WPR_ADR_38 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_38`]
module"]
#[doc(alias = "RAM_WPR_ADR_38")]
pub type RamWprAdr38 = crate::Reg<ram_wpr_adr_38::RamWprAdr38Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_38;
#[doc = "RAM_WPR_ADR_40 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_40`]
module"]
#[doc(alias = "RAM_WPR_ADR_40")]
pub type RamWprAdr40 = crate::Reg<ram_wpr_adr_40::RamWprAdr40Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_40;
#[doc = "RAM_WPR_ADR_48 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_48`]
module"]
#[doc(alias = "RAM_WPR_ADR_48")]
pub type RamWprAdr48 = crate::Reg<ram_wpr_adr_48::RamWprAdr48Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_48;
#[doc = "RAM_WPR_ADR_50 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_50`]
module"]
#[doc(alias = "RAM_WPR_ADR_50")]
pub type RamWprAdr50 = crate::Reg<ram_wpr_adr_50::RamWprAdr50Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_50;
#[doc = "RAM_WPR_ADR_58 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_58`]
module"]
#[doc(alias = "RAM_WPR_ADR_58")]
pub type RamWprAdr58 = crate::Reg<ram_wpr_adr_58::RamWprAdr58Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_58;
#[doc = "RAM_WPR_ADR_60 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_60`]
module"]
#[doc(alias = "RAM_WPR_ADR_60")]
pub type RamWprAdr60 = crate::Reg<ram_wpr_adr_60::RamWprAdr60Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_60;
#[doc = "RAM_WPR_ADR_68 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_68`]
module"]
#[doc(alias = "RAM_WPR_ADR_68")]
pub type RamWprAdr68 = crate::Reg<ram_wpr_adr_68::RamWprAdr68Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_68;
#[doc = "RAM_WPR_ADR_70 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_70`]
module"]
#[doc(alias = "RAM_WPR_ADR_70")]
pub type RamWprAdr70 = crate::Reg<ram_wpr_adr_70::RamWprAdr70Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_70;
#[doc = "RAM_WPR_ADR_78 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_78`]
module"]
#[doc(alias = "RAM_WPR_ADR_78")]
pub type RamWprAdr78 = crate::Reg<ram_wpr_adr_78::RamWprAdr78Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_78;
#[doc = "RAM_WPR_ADR_80 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_80`]
module"]
#[doc(alias = "RAM_WPR_ADR_80")]
pub type RamWprAdr80 = crate::Reg<ram_wpr_adr_80::RamWprAdr80Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_80;
#[doc = "RAM_WPR_ADR_88 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_88`]
module"]
#[doc(alias = "RAM_WPR_ADR_88")]
pub type RamWprAdr88 = crate::Reg<ram_wpr_adr_88::RamWprAdr88Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_88;
#[doc = "RAM_WPR_ADR_90 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_90`]
module"]
#[doc(alias = "RAM_WPR_ADR_90")]
pub type RamWprAdr90 = crate::Reg<ram_wpr_adr_90::RamWprAdr90Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_90;
#[doc = "RAM_WPR_ADR_98 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_98`]
module"]
#[doc(alias = "RAM_WPR_ADR_98")]
pub type RamWprAdr98 = crate::Reg<ram_wpr_adr_98::RamWprAdr98Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_98;
#[doc = "RAM_WPR_ADR_A0 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_a0`]
module"]
#[doc(alias = "RAM_WPR_ADR_A0")]
pub type RamWprAdrA0 = crate::Reg<ram_wpr_adr_a0::RamWprAdrA0Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_a0;
#[doc = "RAM_WPR_ADR_A8 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_a8`]
module"]
#[doc(alias = "RAM_WPR_ADR_A8")]
pub type RamWprAdrA8 = crate::Reg<ram_wpr_adr_a8::RamWprAdrA8Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_a8;
#[doc = "RAM_WPR_ADR_B0 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_b0`]
module"]
#[doc(alias = "RAM_WPR_ADR_B0")]
pub type RamWprAdrB0 = crate::Reg<ram_wpr_adr_b0::RamWprAdrB0Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_b0;
#[doc = "RAM_WPR_ADR_B8 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_b8`]
module"]
#[doc(alias = "RAM_WPR_ADR_B8")]
pub type RamWprAdrB8 = crate::Reg<ram_wpr_adr_b8::RamWprAdrB8Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_b8;
#[doc = "RAM_WPR_ADR_C0 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_c0`]
module"]
#[doc(alias = "RAM_WPR_ADR_C0")]
pub type RamWprAdrC0 = crate::Reg<ram_wpr_adr_c0::RamWprAdrC0Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_c0;
#[doc = "RAM_WPR_ADR_C8 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_c8`]
module"]
#[doc(alias = "RAM_WPR_ADR_C8")]
pub type RamWprAdrC8 = crate::Reg<ram_wpr_adr_c8::RamWprAdrC8Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_c8;
#[doc = "RAM_WPR_ADR_D0 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_d0`]
module"]
#[doc(alias = "RAM_WPR_ADR_D0")]
pub type RamWprAdrD0 = crate::Reg<ram_wpr_adr_d0::RamWprAdrD0Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_d0;
#[doc = "RAM_WPR_ADR_D8 (rw) register accessor: RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wpr_adr_d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wpr_adr_d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wpr_adr_d8`]
module"]
#[doc(alias = "RAM_WPR_ADR_D8")]
pub type RamWprAdrD8 = crate::Reg<ram_wpr_adr_d8::RamWprAdrD8Spec>;
#[doc = "RAM Write Protect Block Base Address nn Register (RAM_WPR_ADR_nn)"]
pub mod ram_wpr_adr_d8;
#[doc = "RAM_FPR_ADR_20 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_20`]
module"]
#[doc(alias = "RAM_FPR_ADR_20")]
pub type RamFprAdr20 = crate::Reg<ram_fpr_adr_20::RamFprAdr20Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_20;
#[doc = "RAM_FPR_ADR_28 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_28`]
module"]
#[doc(alias = "RAM_FPR_ADR_28")]
pub type RamFprAdr28 = crate::Reg<ram_fpr_adr_28::RamFprAdr28Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_28;
#[doc = "RAM_FPR_ADR_30 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_30`]
module"]
#[doc(alias = "RAM_FPR_ADR_30")]
pub type RamFprAdr30 = crate::Reg<ram_fpr_adr_30::RamFprAdr30Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_30;
#[doc = "RAM_FPR_ADR_38 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_38`]
module"]
#[doc(alias = "RAM_FPR_ADR_38")]
pub type RamFprAdr38 = crate::Reg<ram_fpr_adr_38::RamFprAdr38Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_38;
#[doc = "RAM_FPR_ADR_40 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_40`]
module"]
#[doc(alias = "RAM_FPR_ADR_40")]
pub type RamFprAdr40 = crate::Reg<ram_fpr_adr_40::RamFprAdr40Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_40;
#[doc = "RAM_FPR_ADR_48 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_48`]
module"]
#[doc(alias = "RAM_FPR_ADR_48")]
pub type RamFprAdr48 = crate::Reg<ram_fpr_adr_48::RamFprAdr48Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_48;
#[doc = "RAM_FPR_ADR_50 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_50`]
module"]
#[doc(alias = "RAM_FPR_ADR_50")]
pub type RamFprAdr50 = crate::Reg<ram_fpr_adr_50::RamFprAdr50Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_50;
#[doc = "RAM_FPR_ADR_58 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_58`]
module"]
#[doc(alias = "RAM_FPR_ADR_58")]
pub type RamFprAdr58 = crate::Reg<ram_fpr_adr_58::RamFprAdr58Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_58;
#[doc = "RAM_FPR_ADR_60 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_60`]
module"]
#[doc(alias = "RAM_FPR_ADR_60")]
pub type RamFprAdr60 = crate::Reg<ram_fpr_adr_60::RamFprAdr60Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_60;
#[doc = "RAM_FPR_ADR_68 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_68`]
module"]
#[doc(alias = "RAM_FPR_ADR_68")]
pub type RamFprAdr68 = crate::Reg<ram_fpr_adr_68::RamFprAdr68Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_68;
#[doc = "RAM_FPR_ADR_70 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_70`]
module"]
#[doc(alias = "RAM_FPR_ADR_70")]
pub type RamFprAdr70 = crate::Reg<ram_fpr_adr_70::RamFprAdr70Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_70;
#[doc = "RAM_FPR_ADR_78 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_78`]
module"]
#[doc(alias = "RAM_FPR_ADR_78")]
pub type RamFprAdr78 = crate::Reg<ram_fpr_adr_78::RamFprAdr78Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_78;
#[doc = "RAM_FPR_ADR_80 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_80`]
module"]
#[doc(alias = "RAM_FPR_ADR_80")]
pub type RamFprAdr80 = crate::Reg<ram_fpr_adr_80::RamFprAdr80Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_80;
#[doc = "RAM_FPR_ADR_88 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_88`]
module"]
#[doc(alias = "RAM_FPR_ADR_88")]
pub type RamFprAdr88 = crate::Reg<ram_fpr_adr_88::RamFprAdr88Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_88;
#[doc = "RAM_FPR_ADR_90 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_90`]
module"]
#[doc(alias = "RAM_FPR_ADR_90")]
pub type RamFprAdr90 = crate::Reg<ram_fpr_adr_90::RamFprAdr90Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_90;
#[doc = "RAM_FPR_ADR_98 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_98`]
module"]
#[doc(alias = "RAM_FPR_ADR_98")]
pub type RamFprAdr98 = crate::Reg<ram_fpr_adr_98::RamFprAdr98Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_98;
#[doc = "RAM_FPR_ADR_A0 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_a0`]
module"]
#[doc(alias = "RAM_FPR_ADR_A0")]
pub type RamFprAdrA0 = crate::Reg<ram_fpr_adr_a0::RamFprAdrA0Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_a0;
#[doc = "RAM_FPR_ADR_A8 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_a8`]
module"]
#[doc(alias = "RAM_FPR_ADR_A8")]
pub type RamFprAdrA8 = crate::Reg<ram_fpr_adr_a8::RamFprAdrA8Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_a8;
#[doc = "RAM_FPR_ADR_B0 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_b0`]
module"]
#[doc(alias = "RAM_FPR_ADR_B0")]
pub type RamFprAdrB0 = crate::Reg<ram_fpr_adr_b0::RamFprAdrB0Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_b0;
#[doc = "RAM_FPR_ADR_B8 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_b8`]
module"]
#[doc(alias = "RAM_FPR_ADR_B8")]
pub type RamFprAdrB8 = crate::Reg<ram_fpr_adr_b8::RamFprAdrB8Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_b8;
#[doc = "RAM_FPR_ADR_C0 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_c0`]
module"]
#[doc(alias = "RAM_FPR_ADR_C0")]
pub type RamFprAdrC0 = crate::Reg<ram_fpr_adr_c0::RamFprAdrC0Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_c0;
#[doc = "RAM_FPR_ADR_C8 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_c8`]
module"]
#[doc(alias = "RAM_FPR_ADR_C8")]
pub type RamFprAdrC8 = crate::Reg<ram_fpr_adr_c8::RamFprAdrC8Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_c8;
#[doc = "RAM_FPR_ADR_D0 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_d0`]
module"]
#[doc(alias = "RAM_FPR_ADR_D0")]
pub type RamFprAdrD0 = crate::Reg<ram_fpr_adr_d0::RamFprAdrD0Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_d0;
#[doc = "RAM_FPR_ADR_D8 (rw) register accessor: RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fpr_adr_d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fpr_adr_d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fpr_adr_d8`]
module"]
#[doc(alias = "RAM_FPR_ADR_D8")]
pub type RamFprAdrD8 = crate::Reg<ram_fpr_adr_d8::RamFprAdrD8Spec>;
#[doc = "RAM Fetch Protect Block Base Address nn Register (RAM_FPR_ADR_nn)"]
pub mod ram_fpr_adr_d8;
#[doc = "RAM_WPLK_ADR_20 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_20`]
module"]
#[doc(alias = "RAM_WPLK_ADR_20")]
pub type RamWplkAdr20 = crate::Reg<ram_wplk_adr_20::RamWplkAdr20Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_20;
#[doc = "RAM_WPLK_ADR_28 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_28`]
module"]
#[doc(alias = "RAM_WPLK_ADR_28")]
pub type RamWplkAdr28 = crate::Reg<ram_wplk_adr_28::RamWplkAdr28Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_28;
#[doc = "RAM_WPLK_ADR_30 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_30`]
module"]
#[doc(alias = "RAM_WPLK_ADR_30")]
pub type RamWplkAdr30 = crate::Reg<ram_wplk_adr_30::RamWplkAdr30Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_30;
#[doc = "RAM_WPLK_ADR_38 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_38`]
module"]
#[doc(alias = "RAM_WPLK_ADR_38")]
pub type RamWplkAdr38 = crate::Reg<ram_wplk_adr_38::RamWplkAdr38Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_38;
#[doc = "RAM_WPLK_ADR_40 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_40`]
module"]
#[doc(alias = "RAM_WPLK_ADR_40")]
pub type RamWplkAdr40 = crate::Reg<ram_wplk_adr_40::RamWplkAdr40Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_40;
#[doc = "RAM_WPLK_ADR_48 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_48`]
module"]
#[doc(alias = "RAM_WPLK_ADR_48")]
pub type RamWplkAdr48 = crate::Reg<ram_wplk_adr_48::RamWplkAdr48Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_48;
#[doc = "RAM_WPLK_ADR_50 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_50`]
module"]
#[doc(alias = "RAM_WPLK_ADR_50")]
pub type RamWplkAdr50 = crate::Reg<ram_wplk_adr_50::RamWplkAdr50Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_50;
#[doc = "RAM_WPLK_ADR_58 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_58`]
module"]
#[doc(alias = "RAM_WPLK_ADR_58")]
pub type RamWplkAdr58 = crate::Reg<ram_wplk_adr_58::RamWplkAdr58Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_58;
#[doc = "RAM_WPLK_ADR_60 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_60`]
module"]
#[doc(alias = "RAM_WPLK_ADR_60")]
pub type RamWplkAdr60 = crate::Reg<ram_wplk_adr_60::RamWplkAdr60Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_60;
#[doc = "RAM_WPLK_ADR_68 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_68`]
module"]
#[doc(alias = "RAM_WPLK_ADR_68")]
pub type RamWplkAdr68 = crate::Reg<ram_wplk_adr_68::RamWplkAdr68Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_68;
#[doc = "RAM_WPLK_ADR_70 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_70`]
module"]
#[doc(alias = "RAM_WPLK_ADR_70")]
pub type RamWplkAdr70 = crate::Reg<ram_wplk_adr_70::RamWplkAdr70Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_70;
#[doc = "RAM_WPLK_ADR_78 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_78`]
module"]
#[doc(alias = "RAM_WPLK_ADR_78")]
pub type RamWplkAdr78 = crate::Reg<ram_wplk_adr_78::RamWplkAdr78Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_78;
#[doc = "RAM_WPLK_ADR_80 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_80`]
module"]
#[doc(alias = "RAM_WPLK_ADR_80")]
pub type RamWplkAdr80 = crate::Reg<ram_wplk_adr_80::RamWplkAdr80Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_80;
#[doc = "RAM_WPLK_ADR_88 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_88`]
module"]
#[doc(alias = "RAM_WPLK_ADR_88")]
pub type RamWplkAdr88 = crate::Reg<ram_wplk_adr_88::RamWplkAdr88Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_88;
#[doc = "RAM_WPLK_ADR_90 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_90`]
module"]
#[doc(alias = "RAM_WPLK_ADR_90")]
pub type RamWplkAdr90 = crate::Reg<ram_wplk_adr_90::RamWplkAdr90Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_90;
#[doc = "RAM_WPLK_ADR_98 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_98`]
module"]
#[doc(alias = "RAM_WPLK_ADR_98")]
pub type RamWplkAdr98 = crate::Reg<ram_wplk_adr_98::RamWplkAdr98Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_98;
#[doc = "RAM_WPLK_ADR_A0 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_a0`]
module"]
#[doc(alias = "RAM_WPLK_ADR_A0")]
pub type RamWplkAdrA0 = crate::Reg<ram_wplk_adr_a0::RamWplkAdrA0Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_a0;
#[doc = "RAM_WPLK_ADR_A8 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_a8`]
module"]
#[doc(alias = "RAM_WPLK_ADR_A8")]
pub type RamWplkAdrA8 = crate::Reg<ram_wplk_adr_a8::RamWplkAdrA8Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_a8;
#[doc = "RAM_WPLK_ADR_B0 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_b0`]
module"]
#[doc(alias = "RAM_WPLK_ADR_B0")]
pub type RamWplkAdrB0 = crate::Reg<ram_wplk_adr_b0::RamWplkAdrB0Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_b0;
#[doc = "RAM_WPLK_ADR_B8 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_b8`]
module"]
#[doc(alias = "RAM_WPLK_ADR_B8")]
pub type RamWplkAdrB8 = crate::Reg<ram_wplk_adr_b8::RamWplkAdrB8Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_b8;
#[doc = "RAM_WPLK_ADR_C0 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_c0`]
module"]
#[doc(alias = "RAM_WPLK_ADR_C0")]
pub type RamWplkAdrC0 = crate::Reg<ram_wplk_adr_c0::RamWplkAdrC0Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_c0;
#[doc = "RAM_WPLK_ADR_C8 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_c8`]
module"]
#[doc(alias = "RAM_WPLK_ADR_C8")]
pub type RamWplkAdrC8 = crate::Reg<ram_wplk_adr_c8::RamWplkAdrC8Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_c8;
#[doc = "RAM_WPLK_ADR_D0 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_d0`]
module"]
#[doc(alias = "RAM_WPLK_ADR_D0")]
pub type RamWplkAdrD0 = crate::Reg<ram_wplk_adr_d0::RamWplkAdrD0Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_d0;
#[doc = "RAM_WPLK_ADR_D8 (rw) register accessor: RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_wplk_adr_d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_wplk_adr_d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_wplk_adr_d8`]
module"]
#[doc(alias = "RAM_WPLK_ADR_D8")]
pub type RamWplkAdrD8 = crate::Reg<ram_wplk_adr_d8::RamWplkAdrD8Spec>;
#[doc = "RAM Write Protect Lock Block Base Address nn Register (RAM_WPLK_ADR_nn)"]
pub mod ram_wplk_adr_d8;
#[doc = "RAM_FPLK_ADR_20 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_20`]
module"]
#[doc(alias = "RAM_FPLK_ADR_20")]
pub type RamFplkAdr20 = crate::Reg<ram_fplk_adr_20::RamFplkAdr20Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_20;
#[doc = "RAM_FPLK_ADR_28 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_28`]
module"]
#[doc(alias = "RAM_FPLK_ADR_28")]
pub type RamFplkAdr28 = crate::Reg<ram_fplk_adr_28::RamFplkAdr28Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_28;
#[doc = "RAM_FPLK_ADR_30 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_30`]
module"]
#[doc(alias = "RAM_FPLK_ADR_30")]
pub type RamFplkAdr30 = crate::Reg<ram_fplk_adr_30::RamFplkAdr30Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_30;
#[doc = "RAM_FPLK_ADR_38 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_38`]
module"]
#[doc(alias = "RAM_FPLK_ADR_38")]
pub type RamFplkAdr38 = crate::Reg<ram_fplk_adr_38::RamFplkAdr38Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_38;
#[doc = "RAM_FPLK_ADR_40 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_40`]
module"]
#[doc(alias = "RAM_FPLK_ADR_40")]
pub type RamFplkAdr40 = crate::Reg<ram_fplk_adr_40::RamFplkAdr40Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_40;
#[doc = "RAM_FPLK_ADR_48 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_48`]
module"]
#[doc(alias = "RAM_FPLK_ADR_48")]
pub type RamFplkAdr48 = crate::Reg<ram_fplk_adr_48::RamFplkAdr48Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_48;
#[doc = "RAM_FPLK_ADR_50 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_50`]
module"]
#[doc(alias = "RAM_FPLK_ADR_50")]
pub type RamFplkAdr50 = crate::Reg<ram_fplk_adr_50::RamFplkAdr50Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_50;
#[doc = "RAM_FPLK_ADR_58 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_58`]
module"]
#[doc(alias = "RAM_FPLK_ADR_58")]
pub type RamFplkAdr58 = crate::Reg<ram_fplk_adr_58::RamFplkAdr58Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_58;
#[doc = "RAM_FPLK_ADR_60 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_60`]
module"]
#[doc(alias = "RAM_FPLK_ADR_60")]
pub type RamFplkAdr60 = crate::Reg<ram_fplk_adr_60::RamFplkAdr60Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_60;
#[doc = "RAM_FPLK_ADR_68 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_68`]
module"]
#[doc(alias = "RAM_FPLK_ADR_68")]
pub type RamFplkAdr68 = crate::Reg<ram_fplk_adr_68::RamFplkAdr68Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_68;
#[doc = "RAM_FPLK_ADR_70 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_70`]
module"]
#[doc(alias = "RAM_FPLK_ADR_70")]
pub type RamFplkAdr70 = crate::Reg<ram_fplk_adr_70::RamFplkAdr70Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_70;
#[doc = "RAM_FPLK_ADR_78 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_78`]
module"]
#[doc(alias = "RAM_FPLK_ADR_78")]
pub type RamFplkAdr78 = crate::Reg<ram_fplk_adr_78::RamFplkAdr78Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_78;
#[doc = "RAM_FPLK_ADR_80 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_80`]
module"]
#[doc(alias = "RAM_FPLK_ADR_80")]
pub type RamFplkAdr80 = crate::Reg<ram_fplk_adr_80::RamFplkAdr80Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_80;
#[doc = "RAM_FPLK_ADR_88 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_88`]
module"]
#[doc(alias = "RAM_FPLK_ADR_88")]
pub type RamFplkAdr88 = crate::Reg<ram_fplk_adr_88::RamFplkAdr88Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_88;
#[doc = "RAM_FPLK_ADR_90 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_90`]
module"]
#[doc(alias = "RAM_FPLK_ADR_90")]
pub type RamFplkAdr90 = crate::Reg<ram_fplk_adr_90::RamFplkAdr90Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_90;
#[doc = "RAM_FPLK_ADR_98 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_98`]
module"]
#[doc(alias = "RAM_FPLK_ADR_98")]
pub type RamFplkAdr98 = crate::Reg<ram_fplk_adr_98::RamFplkAdr98Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_98;
#[doc = "RAM_FPLK_ADR_A0 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_a0`]
module"]
#[doc(alias = "RAM_FPLK_ADR_A0")]
pub type RamFplkAdrA0 = crate::Reg<ram_fplk_adr_a0::RamFplkAdrA0Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_a0;
#[doc = "RAM_FPLK_ADR_A8 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_a8`]
module"]
#[doc(alias = "RAM_FPLK_ADR_A8")]
pub type RamFplkAdrA8 = crate::Reg<ram_fplk_adr_a8::RamFplkAdrA8Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_a8;
#[doc = "RAM_FPLK_ADR_B0 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_b0`]
module"]
#[doc(alias = "RAM_FPLK_ADR_B0")]
pub type RamFplkAdrB0 = crate::Reg<ram_fplk_adr_b0::RamFplkAdrB0Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_b0;
#[doc = "RAM_FPLK_ADR_B8 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_b8`]
module"]
#[doc(alias = "RAM_FPLK_ADR_B8")]
pub type RamFplkAdrB8 = crate::Reg<ram_fplk_adr_b8::RamFplkAdrB8Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_b8;
#[doc = "RAM_FPLK_ADR_C0 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_c0`]
module"]
#[doc(alias = "RAM_FPLK_ADR_C0")]
pub type RamFplkAdrC0 = crate::Reg<ram_fplk_adr_c0::RamFplkAdrC0Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_c0;
#[doc = "RAM_FPLK_ADR_C8 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_c8`]
module"]
#[doc(alias = "RAM_FPLK_ADR_C8")]
pub type RamFplkAdrC8 = crate::Reg<ram_fplk_adr_c8::RamFplkAdrC8Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_c8;
#[doc = "RAM_FPLK_ADR_D0 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_d0`]
module"]
#[doc(alias = "RAM_FPLK_ADR_D0")]
pub type RamFplkAdrD0 = crate::Reg<ram_fplk_adr_d0::RamFplkAdrD0Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_d0;
#[doc = "RAM_FPLK_ADR_D8 (rw) register accessor: RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ram_fplk_adr_d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram_fplk_adr_d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ram_fplk_adr_d8`]
module"]
#[doc(alias = "RAM_FPLK_ADR_D8")]
pub type RamFplkAdrD8 = crate::Reg<ram_fplk_adr_d8::RamFplkAdrD8Spec>;
#[doc = "RAM Fetch Protect Lock Block Base Address nn Register (RAM_FPLK_ADR_nn)"]
pub mod ram_fplk_adr_d8;
