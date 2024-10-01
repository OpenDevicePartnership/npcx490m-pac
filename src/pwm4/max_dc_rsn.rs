#[doc = "Register `MAX_DC_RSn` reader"]
pub type R = crate::R<MaxDcRsnSpec>;
#[doc = "Register `MAX_DC_RSn` writer"]
pub type W = crate::W<MaxDcRsnSpec>;
#[doc = "Field `MAX_DC_RS11_0` reader - Maximum Duty Cycle Rise Value"]
pub type MaxDcRs11_0R = crate::FieldReader<u16>;
#[doc = "Field `MAX_DC_RS11_0` writer - Maximum Duty Cycle Rise Value"]
pub type MaxDcRs11_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Maximum Duty Cycle Rise Value"]
    #[inline(always)]
    pub fn max_dc_rs11_0(&self) -> MaxDcRs11_0R {
        MaxDcRs11_0R::new(self.bits & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAX_DC_RSn")
            .field("max_dc_rs11_0", &self.max_dc_rs11_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Maximum Duty Cycle Rise Value"]
    #[inline(always)]
    #[must_use]
    pub fn max_dc_rs11_0(&mut self) -> MaxDcRs11_0W<MaxDcRsnSpec> {
        MaxDcRs11_0W::new(self, 0)
    }
}
#[doc = "Maximum Duty Cycle Rise Register (MAX_DC_RSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`max_dc_rsn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max_dc_rsn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxDcRsnSpec;
impl crate::RegisterSpec for MaxDcRsnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`max_dc_rsn::R`](R) reader structure"]
impl crate::Readable for MaxDcRsnSpec {}
#[doc = "`write(|w| ..)` method takes [`max_dc_rsn::W`](W) writer structure"]
impl crate::Writable for MaxDcRsnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets MAX_DC_RSn to value 0"]
impl crate::Resettable for MaxDcRsnSpec {
    const RESET_VALUE: u16 = 0;
}
