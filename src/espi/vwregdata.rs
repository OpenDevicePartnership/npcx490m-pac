#[doc = "Register `VWREGDATA` reader"]
pub type R = crate::R<VwregdataSpec>;
#[doc = "Register `VWREGDATA` writer"]
pub type W = crate::W<VwregdataSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Virtual Wire Register Data Register (VWREGDATA)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwregdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwregdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VwregdataSpec;
impl crate::RegisterSpec for VwregdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vwregdata::R`](R) reader structure"]
impl crate::Readable for VwregdataSpec {}
#[doc = "`write(|w| ..)` method takes [`vwregdata::W`](W) writer structure"]
impl crate::Writable for VwregdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VWREGDATA to value 0"]
impl crate::Resettable for VwregdataSpec {
    const RESET_VALUE: u32 = 0;
}
