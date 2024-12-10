#[doc = "Register `MAX_DC_FLn` reader"]
pub type R = crate::R<MaxDcFlnSpec>;
#[doc = "Register `MAX_DC_FLn` writer"]
pub type W = crate::W<MaxDcFlnSpec>;
#[doc = "Field `MAX_DC_FL11_0` reader - Maximum Duty Cycle Fall Value"]
pub type MaxDcFl11_0R = crate::FieldReader<u16>;
#[doc = "Field `MAX_DC_FL11_0` writer - Maximum Duty Cycle Fall Value"]
pub type MaxDcFl11_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Maximum Duty Cycle Fall Value"]
    #[inline(always)]
    pub fn max_dc_fl11_0(&self) -> MaxDcFl11_0R {
        MaxDcFl11_0R::new(self.bits & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAX_DC_FLn")
            .field("max_dc_fl11_0", &self.max_dc_fl11_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Maximum Duty Cycle Fall Value"]
    #[inline(always)]
    pub fn max_dc_fl11_0(&mut self) -> MaxDcFl11_0W<MaxDcFlnSpec> {
        MaxDcFl11_0W::new(self, 0)
    }
}
#[doc = "Maximum Duty Cycle Fall Register (MAX_DC_FLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`max_dc_fln::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`max_dc_fln::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxDcFlnSpec;
impl crate::RegisterSpec for MaxDcFlnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`max_dc_fln::R`](R) reader structure"]
impl crate::Readable for MaxDcFlnSpec {}
#[doc = "`write(|w| ..)` method takes [`max_dc_fln::W`](W) writer structure"]
impl crate::Writable for MaxDcFlnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets MAX_DC_FLn to value 0"]
impl crate::Resettable for MaxDcFlnSpec {
    const RESET_VALUE: u16 = 0;
}
