#[doc = "Register `CTR_FLn` reader"]
pub type R = crate::R<CtrFlnSpec>;
#[doc = "Register `CTR_FLn` writer"]
pub type W = crate::W<CtrFlnSpec>;
#[doc = "Field `CTR_FL11_0` reader - Cycle Time Value for Fall Duty Cycle"]
pub type CtrFl11_0R = crate::FieldReader<u16>;
#[doc = "Field `CTR_FL11_0` writer - Cycle Time Value for Fall Duty Cycle"]
pub type CtrFl11_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Cycle Time Value for Fall Duty Cycle"]
    #[inline(always)]
    pub fn ctr_fl11_0(&self) -> CtrFl11_0R {
        CtrFl11_0R::new(self.bits & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTR_FLn")
            .field("ctr_fl11_0", &self.ctr_fl11_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Cycle Time Value for Fall Duty Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn ctr_fl11_0(&mut self) -> CtrFl11_0W<CtrFlnSpec> {
        CtrFl11_0W::new(self, 0)
    }
}
#[doc = "Cycle Time for Fall Duty Cycle Register (CTR_FLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr_fln::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr_fln::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrFlnSpec;
impl crate::RegisterSpec for CtrFlnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctr_fln::R`](R) reader structure"]
impl crate::Readable for CtrFlnSpec {}
#[doc = "`write(|w| ..)` method takes [`ctr_fln::W`](W) writer structure"]
impl crate::Writable for CtrFlnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTR_FLn to value 0"]
impl crate::Resettable for CtrFlnSpec {
    const RESET_VALUE: u16 = 0;
}
