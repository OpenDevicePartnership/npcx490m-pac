#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    adcsts: Adcsts,
    adccnf: Adccnf,
    atctl: Atctl,
    ascadd: Ascadd,
    adccs1: Adccs1,
    adccs2: Adccs2,
    _reserved6: [u8; 0x0e],
    thrcts: Thrcts,
    _reserved7: [u8; 0x24],
    chn0dat: Chn0dat,
    chn1dat: Chn1dat,
    chn2dat: Chn2dat,
    chn3dat: Chn3dat,
    chn4dat: Chn4dat,
    chn5dat: Chn5dat,
    chn6dat: Chn6dat,
    chn7dat: Chn7dat,
    chn8dat: Chn8dat,
    chn9dat: Chn9dat,
    chn10dat: Chn10dat,
    chn11dat: Chn11dat,
    chn12dat: Chn12dat,
    chn13dat: Chn13dat,
    chn14dat: Chn14dat,
    chn15dat: Chn15dat,
    chn16dat: Chn16dat,
    chn17dat: Chn17dat,
    chn18dat: Chn18dat,
    chn19dat: Chn19dat,
    chn20dat: Chn20dat,
    chn21dat: Chn21dat,
    chn22dat: Chn22dat,
    chn23dat: Chn23dat,
    chn24dat: Chn24dat,
    chn25dat: Chn25dat,
    _reserved33: [u8; 0x06],
    chn29dat: Chn29dat,
    chn30dat: Chn30dat,
    chn31dat: Chn31dat,
    thrctl1: Thrctl1,
    thrctl2: Thrctl2,
    thrctl3: Thrctl3,
    thrctl4: Thrctl4,
    thrctl5: Thrctl5,
    thrctl6: Thrctl6,
    _reserved42: [u8; 0x04],
    then: Then,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC Status Register (ADCSTS)"]
    #[inline(always)]
    pub const fn adcsts(&self) -> &Adcsts {
        &self.adcsts
    }
    #[doc = "0x02 - ADC Configuration Register (ADCCNF)"]
    #[inline(always)]
    pub const fn adccnf(&self) -> &Adccnf {
        &self.adccnf
    }
    #[doc = "0x04 - ADC Timing Control Register (ATCTL)"]
    #[inline(always)]
    pub const fn atctl(&self) -> &Atctl {
        &self.atctl
    }
    #[doc = "0x06 - ADC Single Channel Address Register (ASCADD)"]
    #[inline(always)]
    pub const fn ascadd(&self) -> &Ascadd {
        &self.ascadd
    }
    #[doc = "0x08 - ADC Scan Channels Select 1 Register (ADCCS1)"]
    #[inline(always)]
    pub const fn adccs1(&self) -> &Adccs1 {
        &self.adccs1
    }
    #[doc = "0x0a - ADC Scan Channels Select 2 Register (ADCCS2)"]
    #[inline(always)]
    pub const fn adccs2(&self) -> &Adccs2 {
        &self.adccs2
    }
    #[doc = "0x1a - Threshold Status Register (THRCTS)"]
    #[inline(always)]
    pub const fn thrcts(&self) -> &Thrcts {
        &self.thrcts
    }
    #[doc = "0x40 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn0dat(&self) -> &Chn0dat {
        &self.chn0dat
    }
    #[doc = "0x42 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn1dat(&self) -> &Chn1dat {
        &self.chn1dat
    }
    #[doc = "0x44 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn2dat(&self) -> &Chn2dat {
        &self.chn2dat
    }
    #[doc = "0x46 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn3dat(&self) -> &Chn3dat {
        &self.chn3dat
    }
    #[doc = "0x48 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn4dat(&self) -> &Chn4dat {
        &self.chn4dat
    }
    #[doc = "0x4a - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn5dat(&self) -> &Chn5dat {
        &self.chn5dat
    }
    #[doc = "0x4c - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn6dat(&self) -> &Chn6dat {
        &self.chn6dat
    }
    #[doc = "0x4e - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn7dat(&self) -> &Chn7dat {
        &self.chn7dat
    }
    #[doc = "0x50 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn8dat(&self) -> &Chn8dat {
        &self.chn8dat
    }
    #[doc = "0x52 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn9dat(&self) -> &Chn9dat {
        &self.chn9dat
    }
    #[doc = "0x54 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn10dat(&self) -> &Chn10dat {
        &self.chn10dat
    }
    #[doc = "0x56 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn11dat(&self) -> &Chn11dat {
        &self.chn11dat
    }
    #[doc = "0x58 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn12dat(&self) -> &Chn12dat {
        &self.chn12dat
    }
    #[doc = "0x5a - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn13dat(&self) -> &Chn13dat {
        &self.chn13dat
    }
    #[doc = "0x5c - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn14dat(&self) -> &Chn14dat {
        &self.chn14dat
    }
    #[doc = "0x5e - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn15dat(&self) -> &Chn15dat {
        &self.chn15dat
    }
    #[doc = "0x60 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn16dat(&self) -> &Chn16dat {
        &self.chn16dat
    }
    #[doc = "0x62 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn17dat(&self) -> &Chn17dat {
        &self.chn17dat
    }
    #[doc = "0x64 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn18dat(&self) -> &Chn18dat {
        &self.chn18dat
    }
    #[doc = "0x66 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn19dat(&self) -> &Chn19dat {
        &self.chn19dat
    }
    #[doc = "0x68 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn20dat(&self) -> &Chn20dat {
        &self.chn20dat
    }
    #[doc = "0x6a - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn21dat(&self) -> &Chn21dat {
        &self.chn21dat
    }
    #[doc = "0x6c - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn22dat(&self) -> &Chn22dat {
        &self.chn22dat
    }
    #[doc = "0x6e - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn23dat(&self) -> &Chn23dat {
        &self.chn23dat
    }
    #[doc = "0x70 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn24dat(&self) -> &Chn24dat {
        &self.chn24dat
    }
    #[doc = "0x72 - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn25dat(&self) -> &Chn25dat {
        &self.chn25dat
    }
    #[doc = "0x7a - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn29dat(&self) -> &Chn29dat {
        &self.chn29dat
    }
    #[doc = "0x7c - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn30dat(&self) -> &Chn30dat {
        &self.chn30dat
    }
    #[doc = "0x7e - Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
    #[inline(always)]
    pub const fn chn31dat(&self) -> &Chn31dat {
        &self.chn31dat
    }
    #[doc = "0x80 - Threshold Control 1-6 Register (THRCTL1-6)"]
    #[inline(always)]
    pub const fn thrctl1(&self) -> &Thrctl1 {
        &self.thrctl1
    }
    #[doc = "0x82 - Threshold Control 1-6 Register (THRCTL1-6)"]
    #[inline(always)]
    pub const fn thrctl2(&self) -> &Thrctl2 {
        &self.thrctl2
    }
    #[doc = "0x84 - Threshold Control 1-6 Register (THRCTL1-6)"]
    #[inline(always)]
    pub const fn thrctl3(&self) -> &Thrctl3 {
        &self.thrctl3
    }
    #[doc = "0x86 - Threshold Control 1-6 Register (THRCTL1-6)"]
    #[inline(always)]
    pub const fn thrctl4(&self) -> &Thrctl4 {
        &self.thrctl4
    }
    #[doc = "0x88 - Threshold Control 1-6 Register (THRCTL1-6)"]
    #[inline(always)]
    pub const fn thrctl5(&self) -> &Thrctl5 {
        &self.thrctl5
    }
    #[doc = "0x8a - Threshold Control 1-6 Register (THRCTL1-6)"]
    #[inline(always)]
    pub const fn thrctl6(&self) -> &Thrctl6 {
        &self.thrctl6
    }
    #[doc = "0x90 - Threshold Enable Register (THEN)"]
    #[inline(always)]
    pub const fn then(&self) -> &Then {
        &self.then
    }
}
#[doc = "ADCSTS (rw) register accessor: ADC Status Register (ADCSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`adcsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcsts`]
module"]
#[doc(alias = "ADCSTS")]
pub type Adcsts = crate::Reg<adcsts::AdcstsSpec>;
#[doc = "ADC Status Register (ADCSTS)"]
pub mod adcsts;
#[doc = "ADCCNF (rw) register accessor: ADC Configuration Register (ADCCNF)\n\nYou can [`read`](crate::Reg::read) this register and get [`adccnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccnf`]
module"]
#[doc(alias = "ADCCNF")]
pub type Adccnf = crate::Reg<adccnf::AdccnfSpec>;
#[doc = "ADC Configuration Register (ADCCNF)"]
pub mod adccnf;
#[doc = "ATCTL (rw) register accessor: ADC Timing Control Register (ATCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`atctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`atctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atctl`]
module"]
#[doc(alias = "ATCTL")]
pub type Atctl = crate::Reg<atctl::AtctlSpec>;
#[doc = "ADC Timing Control Register (ATCTL)"]
pub mod atctl;
#[doc = "ASCADD (rw) register accessor: ADC Single Channel Address Register (ASCADD)\n\nYou can [`read`](crate::Reg::read) this register and get [`ascadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ascadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ascadd`]
module"]
#[doc(alias = "ASCADD")]
pub type Ascadd = crate::Reg<ascadd::AscaddSpec>;
#[doc = "ADC Single Channel Address Register (ASCADD)"]
pub mod ascadd;
#[doc = "ADCCS1 (rw) register accessor: ADC Scan Channels Select 1 Register (ADCCS1)\n\nYou can [`read`](crate::Reg::read) this register and get [`adccs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccs1`]
module"]
#[doc(alias = "ADCCS1")]
pub type Adccs1 = crate::Reg<adccs1::Adccs1Spec>;
#[doc = "ADC Scan Channels Select 1 Register (ADCCS1)"]
pub mod adccs1;
#[doc = "ADCCS2 (rw) register accessor: ADC Scan Channels Select 2 Register (ADCCS2)\n\nYou can [`read`](crate::Reg::read) this register and get [`adccs2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccs2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccs2`]
module"]
#[doc(alias = "ADCCS2")]
pub type Adccs2 = crate::Reg<adccs2::Adccs2Spec>;
#[doc = "ADC Scan Channels Select 2 Register (ADCCS2)"]
pub mod adccs2;
#[doc = "THRCTS (rw) register accessor: Threshold Status Register (THRCTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`thrcts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thrcts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thrcts`]
module"]
#[doc(alias = "THRCTS")]
pub type Thrcts = crate::Reg<thrcts::ThrctsSpec>;
#[doc = "Threshold Status Register (THRCTS)"]
pub mod thrcts;
#[doc = "CHN0DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn0dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn0dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn0dat`]
module"]
#[doc(alias = "CHN0DAT")]
pub type Chn0dat = crate::Reg<chn0dat::Chn0datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn0dat;
#[doc = "CHN1DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn1dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn1dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn1dat`]
module"]
#[doc(alias = "CHN1DAT")]
pub type Chn1dat = crate::Reg<chn1dat::Chn1datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn1dat;
#[doc = "CHN2DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn2dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn2dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn2dat`]
module"]
#[doc(alias = "CHN2DAT")]
pub type Chn2dat = crate::Reg<chn2dat::Chn2datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn2dat;
#[doc = "CHN3DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn3dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn3dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn3dat`]
module"]
#[doc(alias = "CHN3DAT")]
pub type Chn3dat = crate::Reg<chn3dat::Chn3datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn3dat;
#[doc = "CHN4DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn4dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn4dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn4dat`]
module"]
#[doc(alias = "CHN4DAT")]
pub type Chn4dat = crate::Reg<chn4dat::Chn4datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn4dat;
#[doc = "CHN5DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn5dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn5dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn5dat`]
module"]
#[doc(alias = "CHN5DAT")]
pub type Chn5dat = crate::Reg<chn5dat::Chn5datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn5dat;
#[doc = "CHN6DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn6dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn6dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn6dat`]
module"]
#[doc(alias = "CHN6DAT")]
pub type Chn6dat = crate::Reg<chn6dat::Chn6datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn6dat;
#[doc = "CHN7DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn7dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn7dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn7dat`]
module"]
#[doc(alias = "CHN7DAT")]
pub type Chn7dat = crate::Reg<chn7dat::Chn7datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn7dat;
#[doc = "CHN8DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn8dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn8dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn8dat`]
module"]
#[doc(alias = "CHN8DAT")]
pub type Chn8dat = crate::Reg<chn8dat::Chn8datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn8dat;
#[doc = "CHN9DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn9dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn9dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn9dat`]
module"]
#[doc(alias = "CHN9DAT")]
pub type Chn9dat = crate::Reg<chn9dat::Chn9datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn9dat;
#[doc = "CHN10DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn10dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn10dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn10dat`]
module"]
#[doc(alias = "CHN10DAT")]
pub type Chn10dat = crate::Reg<chn10dat::Chn10datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn10dat;
#[doc = "CHN11DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn11dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn11dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn11dat`]
module"]
#[doc(alias = "CHN11DAT")]
pub type Chn11dat = crate::Reg<chn11dat::Chn11datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn11dat;
#[doc = "CHN12DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn12dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn12dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn12dat`]
module"]
#[doc(alias = "CHN12DAT")]
pub type Chn12dat = crate::Reg<chn12dat::Chn12datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn12dat;
#[doc = "CHN13DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn13dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn13dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn13dat`]
module"]
#[doc(alias = "CHN13DAT")]
pub type Chn13dat = crate::Reg<chn13dat::Chn13datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn13dat;
#[doc = "CHN14DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn14dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn14dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn14dat`]
module"]
#[doc(alias = "CHN14DAT")]
pub type Chn14dat = crate::Reg<chn14dat::Chn14datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn14dat;
#[doc = "CHN15DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn15dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn15dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn15dat`]
module"]
#[doc(alias = "CHN15DAT")]
pub type Chn15dat = crate::Reg<chn15dat::Chn15datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn15dat;
#[doc = "CHN16DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn16dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn16dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn16dat`]
module"]
#[doc(alias = "CHN16DAT")]
pub type Chn16dat = crate::Reg<chn16dat::Chn16datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn16dat;
#[doc = "CHN17DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn17dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn17dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn17dat`]
module"]
#[doc(alias = "CHN17DAT")]
pub type Chn17dat = crate::Reg<chn17dat::Chn17datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn17dat;
#[doc = "CHN18DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn18dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn18dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn18dat`]
module"]
#[doc(alias = "CHN18DAT")]
pub type Chn18dat = crate::Reg<chn18dat::Chn18datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn18dat;
#[doc = "CHN19DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn19dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn19dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn19dat`]
module"]
#[doc(alias = "CHN19DAT")]
pub type Chn19dat = crate::Reg<chn19dat::Chn19datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn19dat;
#[doc = "CHN20DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn20dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn20dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn20dat`]
module"]
#[doc(alias = "CHN20DAT")]
pub type Chn20dat = crate::Reg<chn20dat::Chn20datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn20dat;
#[doc = "CHN21DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn21dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn21dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn21dat`]
module"]
#[doc(alias = "CHN21DAT")]
pub type Chn21dat = crate::Reg<chn21dat::Chn21datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn21dat;
#[doc = "CHN22DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn22dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn22dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn22dat`]
module"]
#[doc(alias = "CHN22DAT")]
pub type Chn22dat = crate::Reg<chn22dat::Chn22datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn22dat;
#[doc = "CHN23DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn23dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn23dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn23dat`]
module"]
#[doc(alias = "CHN23DAT")]
pub type Chn23dat = crate::Reg<chn23dat::Chn23datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn23dat;
#[doc = "CHN24DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn24dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn24dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn24dat`]
module"]
#[doc(alias = "CHN24DAT")]
pub type Chn24dat = crate::Reg<chn24dat::Chn24datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn24dat;
#[doc = "CHN25DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn25dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn25dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn25dat`]
module"]
#[doc(alias = "CHN25DAT")]
pub type Chn25dat = crate::Reg<chn25dat::Chn25datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn25dat;
#[doc = "CHN29DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn29dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn29dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn29dat`]
module"]
#[doc(alias = "CHN29DAT")]
pub type Chn29dat = crate::Reg<chn29dat::Chn29datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn29dat;
#[doc = "CHN30DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn30dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn30dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn30dat`]
module"]
#[doc(alias = "CHN30DAT")]
pub type Chn30dat = crate::Reg<chn30dat::Chn30datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn30dat;
#[doc = "CHN31DAT (rw) register accessor: Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn31dat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn31dat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chn31dat`]
module"]
#[doc(alias = "CHN31DAT")]
pub type Chn31dat = crate::Reg<chn31dat::Chn31datSpec>;
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})"]
pub mod chn31dat;
#[doc = "THRCTL1 (rw) register accessor: Threshold Control 1-6 Register (THRCTL1-6)\n\nYou can [`read`](crate::Reg::read) this register and get [`thrctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thrctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thrctl1`]
module"]
#[doc(alias = "THRCTL1")]
pub type Thrctl1 = crate::Reg<thrctl1::Thrctl1Spec>;
#[doc = "Threshold Control 1-6 Register (THRCTL1-6)"]
pub mod thrctl1;
#[doc = "THRCTL2 (rw) register accessor: Threshold Control 1-6 Register (THRCTL1-6)\n\nYou can [`read`](crate::Reg::read) this register and get [`thrctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thrctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thrctl2`]
module"]
#[doc(alias = "THRCTL2")]
pub type Thrctl2 = crate::Reg<thrctl2::Thrctl2Spec>;
#[doc = "Threshold Control 1-6 Register (THRCTL1-6)"]
pub mod thrctl2;
#[doc = "THRCTL3 (rw) register accessor: Threshold Control 1-6 Register (THRCTL1-6)\n\nYou can [`read`](crate::Reg::read) this register and get [`thrctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thrctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thrctl3`]
module"]
#[doc(alias = "THRCTL3")]
pub type Thrctl3 = crate::Reg<thrctl3::Thrctl3Spec>;
#[doc = "Threshold Control 1-6 Register (THRCTL1-6)"]
pub mod thrctl3;
#[doc = "THRCTL4 (rw) register accessor: Threshold Control 1-6 Register (THRCTL1-6)\n\nYou can [`read`](crate::Reg::read) this register and get [`thrctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thrctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thrctl4`]
module"]
#[doc(alias = "THRCTL4")]
pub type Thrctl4 = crate::Reg<thrctl4::Thrctl4Spec>;
#[doc = "Threshold Control 1-6 Register (THRCTL1-6)"]
pub mod thrctl4;
#[doc = "THRCTL5 (rw) register accessor: Threshold Control 1-6 Register (THRCTL1-6)\n\nYou can [`read`](crate::Reg::read) this register and get [`thrctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thrctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thrctl5`]
module"]
#[doc(alias = "THRCTL5")]
pub type Thrctl5 = crate::Reg<thrctl5::Thrctl5Spec>;
#[doc = "Threshold Control 1-6 Register (THRCTL1-6)"]
pub mod thrctl5;
#[doc = "THRCTL6 (rw) register accessor: Threshold Control 1-6 Register (THRCTL1-6)\n\nYou can [`read`](crate::Reg::read) this register and get [`thrctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thrctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thrctl6`]
module"]
#[doc(alias = "THRCTL6")]
pub type Thrctl6 = crate::Reg<thrctl6::Thrctl6Spec>;
#[doc = "Threshold Control 1-6 Register (THRCTL1-6)"]
pub mod thrctl6;
#[doc = "THEN (rw) register accessor: Threshold Enable Register (THEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`then::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`then::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@then`]
module"]
#[doc(alias = "THEN")]
pub type Then = crate::Reg<then::ThenSpec>;
#[doc = "Threshold Enable Register (THEN)"]
pub mod then;
