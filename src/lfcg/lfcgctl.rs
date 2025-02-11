#[doc = "Register `LFCGCTL` reader"]
pub type R = crate::R<LfcgctlSpec>;
#[doc = "Register `LFCGCTL` writer"]
pub type W = crate::W<LfcgctlSpec>;
#[doc = "Field `LREFEN` reader - LPC Clock Reference Enable"]
pub type LrefenR = crate::BitReader;
#[doc = "Field `LREFEN` writer - LPC Clock Reference Enable"]
pub type LrefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFLER` reader - LFCG Locked on External Reference"]
pub type LflerR = crate::BitReader;
#[doc = "Field `LFLER` writer - LFCG Locked on External Reference"]
pub type LflerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDCP` reader - Update Calibration Parameters"]
pub type UdcpR = crate::BitReader;
#[doc = "Field `UDCP` writer - Update Calibration Parameters"]
pub type UdcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFLOC` reader - LFCG Register Write Lock"]
pub type LflocR = crate::BitReader;
#[doc = "Field `LFLOC` writer - LFCG Register Write Lock"]
pub type LflocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "When set to 1, this bit indicates that the XTOSC clock (XTCLK) is valid (i.e., toggling and stable).\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XtoscClockValid {
    #[doc = "0: Not yet stabilized"]
    NotYetStabilized = 0,
    #[doc = "1: Stabilized"]
    Stabilized = 1,
}
impl From<XtoscClockValid> for bool {
    #[inline(always)]
    fn from(variant: XtoscClockValid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XTCLK_VAL` reader - When set to 1, this bit indicates that the XTOSC clock (XTCLK) is valid (i.e., toggling and stable)."]
pub type XtclkValR = crate::BitReader<XtoscClockValid>;
impl XtclkValR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XtoscClockValid {
        match self.bits {
            false => XtoscClockValid::NotYetStabilized,
            true => XtoscClockValid::Stabilized,
        }
    }
    #[doc = "Not yet stabilized"]
    #[inline(always)]
    pub fn is_not_yet_stabilized(&self) -> bool {
        *self == XtoscClockValid::NotYetStabilized
    }
    #[doc = "Stabilized"]
    #[inline(always)]
    pub fn is_stabilized(&self) -> bool {
        *self == XtoscClockValid::Stabilized
    }
}
impl R {
    #[doc = "Bit 2 - LPC Clock Reference Enable"]
    #[inline(always)]
    pub fn lrefen(&self) -> LrefenR {
        LrefenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFCG Locked on External Reference"]
    #[inline(always)]
    pub fn lfler(&self) -> LflerR {
        LflerR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Update Calibration Parameters"]
    #[inline(always)]
    pub fn udcp(&self) -> UdcpR {
        UdcpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LFCG Register Write Lock"]
    #[inline(always)]
    pub fn lfloc(&self) -> LflocR {
        LflocR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - When set to 1, this bit indicates that the XTOSC clock (XTCLK) is valid (i.e., toggling and stable)."]
    #[inline(always)]
    pub fn xtclk_val(&self) -> XtclkValR {
        XtclkValR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LFCGCTL")
            .field("lrefen", &self.lrefen())
            .field("lfler", &self.lfler())
            .field("udcp", &self.udcp())
            .field("lfloc", &self.lfloc())
            .field("xtclk_val", &self.xtclk_val())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - LPC Clock Reference Enable"]
    #[inline(always)]
    pub fn lrefen(&mut self) -> LrefenW<LfcgctlSpec> {
        LrefenW::new(self, 2)
    }
    #[doc = "Bit 3 - LFCG Locked on External Reference"]
    #[inline(always)]
    pub fn lfler(&mut self) -> LflerW<LfcgctlSpec> {
        LflerW::new(self, 3)
    }
    #[doc = "Bit 4 - Update Calibration Parameters"]
    #[inline(always)]
    pub fn udcp(&mut self) -> UdcpW<LfcgctlSpec> {
        UdcpW::new(self, 4)
    }
    #[doc = "Bit 5 - LFCG Register Write Lock"]
    #[inline(always)]
    pub fn lfloc(&mut self) -> LflocW<LfcgctlSpec> {
        LflocW::new(self, 5)
    }
}
#[doc = "LFCG Control Register (LFCGCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfcgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfcgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfcgctlSpec;
impl crate::RegisterSpec for LfcgctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lfcgctl::R`](R) reader structure"]
impl crate::Readable for LfcgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfcgctl::W`](W) writer structure"]
impl crate::Writable for LfcgctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LFCGCTL to value 0x81"]
impl crate::Resettable for LfcgctlSpec {
    const RESET_VALUE: u8 = 0x81;
}
