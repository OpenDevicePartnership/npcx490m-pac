#[doc = "Register `WTC` reader"]
pub type R = crate::R<WtcSpec>;
#[doc = "Register `WTC` writer"]
pub type W = crate::W<WtcSpec>;
#[doc = "Field `PT` reader - Predefined Time"]
pub type PtR = crate::FieldReader<u32>;
#[doc = "Field `PT` writer - Predefined Time"]
pub type PtW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
#[doc = "Field `PTO` reader - Predefined Time Occurred"]
pub type PtoR = crate::BitReader;
#[doc = "Field `PTO` writer - Predefined Time Occurred"]
pub type PtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIE` reader - Wake-Up/Interrupt Enabled"]
pub type WieR = crate::BitReader;
#[doc = "Field `WIE` writer - Wake-Up/Interrupt Enabled"]
pub type WieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:24 - Predefined Time"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bit 30 - Predefined Time Occurred"]
    #[inline(always)]
    pub fn pto(&self) -> PtoR {
        PtoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Wake-Up/Interrupt Enabled"]
    #[inline(always)]
    pub fn wie(&self) -> WieR {
        WieR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WTC")
            .field("pt", &self.pt())
            .field("pto", &self.pto())
            .field("wie", &self.wie())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:24 - Predefined Time"]
    #[inline(always)]
    pub fn pt(&mut self) -> PtW<WtcSpec> {
        PtW::new(self, 0)
    }
    #[doc = "Bit 30 - Predefined Time Occurred"]
    #[inline(always)]
    pub fn pto(&mut self) -> PtoW<WtcSpec> {
        PtoW::new(self, 30)
    }
    #[doc = "Bit 31 - Wake-Up/Interrupt Enabled"]
    #[inline(always)]
    pub fn wie(&mut self) -> WieW<WtcSpec> {
        WieW::new(self, 31)
    }
}
#[doc = "Wake-Up Ticks Count Register (WTC)\n\nYou can [`read`](crate::Reg::read) this register and get [`wtc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WtcSpec;
impl crate::RegisterSpec for WtcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtc::R`](R) reader structure"]
impl crate::Readable for WtcSpec {}
#[doc = "`write(|w| ..)` method takes [`wtc::W`](W) writer structure"]
impl crate::Writable for WtcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTC to value 0"]
impl crate::Resettable for WtcSpec {
    const RESET_VALUE: u32 = 0;
}
