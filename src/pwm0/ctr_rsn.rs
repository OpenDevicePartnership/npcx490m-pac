#[doc = "Register `CTR_RSn` reader"]
pub type R = crate::R<CtrRsnSpec>;
#[doc = "Register `CTR_RSn` writer"]
pub type W = crate::W<CtrRsnSpec>;
#[doc = "Field `CTR_RS11_0` reader - Cycle Time Value for Rise Duty Cycle"]
pub type CtrRs11_0R = crate::FieldReader<u16>;
#[doc = "Field `CTR_RS11_0` writer - Cycle Time Value for Rise Duty Cycle"]
pub type CtrRs11_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Cycle Time Value for Rise Duty Cycle"]
    #[inline(always)]
    pub fn ctr_rs11_0(&self) -> CtrRs11_0R {
        CtrRs11_0R::new(self.bits & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTR_RSn")
            .field("ctr_rs11_0", &self.ctr_rs11_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Cycle Time Value for Rise Duty Cycle"]
    #[inline(always)]
    pub fn ctr_rs11_0(&mut self) -> CtrRs11_0W<CtrRsnSpec> {
        CtrRs11_0W::new(self, 0)
    }
}
#[doc = "Cycle Time for Rise Duty Cycle Register (CTR_RSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctr_rsn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctr_rsn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrRsnSpec;
impl crate::RegisterSpec for CtrRsnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctr_rsn::R`](R) reader structure"]
impl crate::Readable for CtrRsnSpec {}
#[doc = "`write(|w| ..)` method takes [`ctr_rsn::W`](W) writer structure"]
impl crate::Writable for CtrRsnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTR_RSn to value 0"]
impl crate::Resettable for CtrRsnSpec {
    const RESET_VALUE: u16 = 0;
}
