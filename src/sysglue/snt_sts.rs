#[doc = "Register `SNT_STS` reader"]
pub type R = crate::R<SntStsSpec>;
#[doc = "Register `SNT_STS` writer"]
pub type W = crate::W<SntStsSpec>;
#[doc = "Field `SNT_END` reader - Snooze Timer Count Ended"]
pub type SntEndR = crate::FieldReader;
#[doc = "Field `SNT_END` writer - Snooze Timer Count Ended"]
pub type SntEndW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Snooze Timer Count Ended"]
    #[inline(always)]
    pub fn snt_end(&self) -> SntEndR {
        SntEndR::new(self.bits & 3)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SNT_STS")
            .field("snt_end", &self.snt_end())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Snooze Timer Count Ended"]
    #[inline(always)]
    pub fn snt_end(&mut self) -> SntEndW<SntStsSpec> {
        SntEndW::new(self, 0)
    }
}
#[doc = "Snooze Timer Status Register (SNT_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`snt_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snt_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SntStsSpec;
impl crate::RegisterSpec for SntStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`snt_sts::R`](R) reader structure"]
impl crate::Readable for SntStsSpec {}
#[doc = "`write(|w| ..)` method takes [`snt_sts::W`](W) writer structure"]
impl crate::Writable for SntStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SNT_STS to value 0"]
impl crate::Resettable for SntStsSpec {
    const RESET_VALUE: u8 = 0;
}
