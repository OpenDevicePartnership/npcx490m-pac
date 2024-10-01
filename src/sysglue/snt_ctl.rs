#[doc = "Register `SNT_CTL` reader"]
pub type R = crate::R<SntCtlSpec>;
#[doc = "Register `SNT_CTL` writer"]
pub type W = crate::W<SntCtlSpec>;
#[doc = "Field `SNT_GO` reader - Snooze Timer Start"]
pub type SntGoR = crate::BitReader;
#[doc = "Field `SNT_GO` writer - Snooze Timer Start"]
pub type SntGoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNT_CNT` reader - Snooze Timer Count"]
pub type SntCntR = crate::FieldReader<u16>;
#[doc = "Field `SNT_CNT` writer - Snooze Timer Count"]
pub type SntCntW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - Snooze Timer Start"]
    #[inline(always)]
    pub fn snt_go(&self) -> SntGoR {
        SntGoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:14 - Snooze Timer Count"]
    #[inline(always)]
    pub fn snt_cnt(&self) -> SntCntR {
        SntCntR::new((self.bits >> 3) & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNT_CTL")
            .field("snt_go", &self.snt_go())
            .field("snt_cnt", &self.snt_cnt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Snooze Timer Start"]
    #[inline(always)]
    #[must_use]
    pub fn snt_go(&mut self) -> SntGoW<SntCtlSpec> {
        SntGoW::new(self, 0)
    }
    #[doc = "Bits 3:14 - Snooze Timer Count"]
    #[inline(always)]
    #[must_use]
    pub fn snt_cnt(&mut self) -> SntCntW<SntCtlSpec> {
        SntCntW::new(self, 3)
    }
}
#[doc = "Snooze Timer Control Register (SNT_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`snt_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snt_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SntCtlSpec;
impl crate::RegisterSpec for SntCtlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`snt_ctl::R`](R) reader structure"]
impl crate::Readable for SntCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`snt_ctl::W`](W) writer structure"]
impl crate::Writable for SntCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SNT_CTL to value 0"]
impl crate::Resettable for SntCtlSpec {
    const RESET_VALUE: u16 = 0;
}
