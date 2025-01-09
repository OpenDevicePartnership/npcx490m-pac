#[doc = "Register `HFRDF` reader"]
pub type R = crate::R<HfrdfSpec>;
#[doc = "Register `HFRDF` writer"]
pub type W = crate::W<HfrdfSpec>;
#[doc = "High-Frequency Reference Divisor F. The contents of this field should be changed only when the high-frequency reference clock is not selected. HFRDF is locked for writes (RO) when LFLOC bit is set to 1.\n\nValue on reset: 41045"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum ReferenceClock {
    #[doc = "13824: A 24 MHz PCI_CLK clock is used as the reference"]
    Mhz24 = 13824,
    #[doc = "30784: A 25 MHz PCI_CLK clock is used as the reference"]
    Mhz25 = 30784,
    #[doc = "41045: A 33.333 MHz PCI_CLK clock is used as the reference (default)"]
    Mhz33_333 = 41045,
    #[doc = "63488: A 19.2 MHz PCI_CLK clock is used as the reference"]
    Mhz19_2 = 63488,
}
impl From<ReferenceClock> for u16 {
    #[inline(always)]
    fn from(variant: ReferenceClock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ReferenceClock {
    type Ux = u16;
}
impl crate::IsEnum for ReferenceClock {}
#[doc = "Field `HFRDF` reader - High-Frequency Reference Divisor F. The contents of this field should be changed only when the high-frequency reference clock is not selected. HFRDF is locked for writes (RO) when LFLOC bit is set to 1."]
pub type HfrdfR = crate::FieldReader<ReferenceClock>;
impl HfrdfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ReferenceClock> {
        match self.bits {
            13824 => Some(ReferenceClock::Mhz24),
            30784 => Some(ReferenceClock::Mhz25),
            41045 => Some(ReferenceClock::Mhz33_333),
            63488 => Some(ReferenceClock::Mhz19_2),
            _ => None,
        }
    }
    #[doc = "A 24 MHz PCI_CLK clock is used as the reference"]
    #[inline(always)]
    pub fn is_mhz24(&self) -> bool {
        *self == ReferenceClock::Mhz24
    }
    #[doc = "A 25 MHz PCI_CLK clock is used as the reference"]
    #[inline(always)]
    pub fn is_mhz25(&self) -> bool {
        *self == ReferenceClock::Mhz25
    }
    #[doc = "A 33.333 MHz PCI_CLK clock is used as the reference (default)"]
    #[inline(always)]
    pub fn is_mhz33_333(&self) -> bool {
        *self == ReferenceClock::Mhz33_333
    }
    #[doc = "A 19.2 MHz PCI_CLK clock is used as the reference"]
    #[inline(always)]
    pub fn is_mhz19_2(&self) -> bool {
        *self == ReferenceClock::Mhz19_2
    }
}
#[doc = "Field `HFRDF` writer - High-Frequency Reference Divisor F. The contents of this field should be changed only when the high-frequency reference clock is not selected. HFRDF is locked for writes (RO) when LFLOC bit is set to 1."]
pub type HfrdfW<'a, REG> = crate::FieldWriter<'a, REG, 16, ReferenceClock>;
impl<'a, REG> HfrdfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "A 24 MHz PCI_CLK clock is used as the reference"]
    #[inline(always)]
    pub fn mhz24(self) -> &'a mut crate::W<REG> {
        self.variant(ReferenceClock::Mhz24)
    }
    #[doc = "A 25 MHz PCI_CLK clock is used as the reference"]
    #[inline(always)]
    pub fn mhz25(self) -> &'a mut crate::W<REG> {
        self.variant(ReferenceClock::Mhz25)
    }
    #[doc = "A 33.333 MHz PCI_CLK clock is used as the reference (default)"]
    #[inline(always)]
    pub fn mhz33_333(self) -> &'a mut crate::W<REG> {
        self.variant(ReferenceClock::Mhz33_333)
    }
    #[doc = "A 19.2 MHz PCI_CLK clock is used as the reference"]
    #[inline(always)]
    pub fn mhz19_2(self) -> &'a mut crate::W<REG> {
        self.variant(ReferenceClock::Mhz19_2)
    }
}
impl R {
    #[doc = "Bits 0:15 - High-Frequency Reference Divisor F. The contents of this field should be changed only when the high-frequency reference clock is not selected. HFRDF is locked for writes (RO) when LFLOC bit is set to 1."]
    #[inline(always)]
    pub fn hfrdf(&self) -> HfrdfR {
        HfrdfR::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFRDF")
            .field("hfrdf", &self.hfrdf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - High-Frequency Reference Divisor F. The contents of this field should be changed only when the high-frequency reference clock is not selected. HFRDF is locked for writes (RO) when LFLOC bit is set to 1."]
    #[inline(always)]
    pub fn hfrdf(&mut self) -> HfrdfW<HfrdfSpec> {
        HfrdfW::new(self, 0)
    }
}
#[doc = "High-Frequency Reference Divisor F Register (HFRDF)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrdf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrdf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfrdfSpec;
impl crate::RegisterSpec for HfrdfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hfrdf::R`](R) reader structure"]
impl crate::Readable for HfrdfSpec {}
#[doc = "`write(|w| ..)` method takes [`hfrdf::W`](W) writer structure"]
impl crate::Writable for HfrdfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HFRDF to value 0xa055"]
impl crate::Resettable for HfrdfSpec {
    const RESET_VALUE: u16 = 0xa055;
}
