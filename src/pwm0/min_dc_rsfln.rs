#[doc = "Register `MIN_DC_RSFLn` reader"]
pub type R = crate::R<MinDcRsflnSpec>;
#[doc = "Register `MIN_DC_RSFLn` writer"]
pub type W = crate::W<MinDcRsflnSpec>;
#[doc = "Field `MIN_DC_RSFL11_0` reader - Minimum Duty Cycle Rise/Fall Value"]
pub type MinDcRsfl11_0R = crate::FieldReader<u16>;
#[doc = "Field `MIN_DC_RSFL11_0` writer - Minimum Duty Cycle Rise/Fall Value"]
pub type MinDcRsfl11_0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Minimum Duty Cycle Rise/Fall Value"]
    #[inline(always)]
    pub fn min_dc_rsfl11_0(&self) -> MinDcRsfl11_0R {
        MinDcRsfl11_0R::new(self.bits & 0x0fff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIN_DC_RSFLn")
            .field("min_dc_rsfl11_0", &self.min_dc_rsfl11_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Minimum Duty Cycle Rise/Fall Value"]
    #[inline(always)]
    pub fn min_dc_rsfl11_0(&mut self) -> MinDcRsfl11_0W<MinDcRsflnSpec> {
        MinDcRsfl11_0W::new(self, 0)
    }
}
#[doc = "Minimum Duty Cycle Rise/Fall Register (MIN_DC_RSFLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`min_dc_rsfln::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`min_dc_rsfln::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MinDcRsflnSpec;
impl crate::RegisterSpec for MinDcRsflnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`min_dc_rsfln::R`](R) reader structure"]
impl crate::Readable for MinDcRsflnSpec {}
#[doc = "`write(|w| ..)` method takes [`min_dc_rsfln::W`](W) writer structure"]
impl crate::Writable for MinDcRsflnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets MIN_DC_RSFLn to value 0"]
impl crate::Resettable for MinDcRsflnSpec {
    const RESET_VALUE: u16 = 0;
}
