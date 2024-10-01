#[doc = "Register `EXT_OFFn` reader"]
pub type R = crate::R<ExtOffnSpec>;
#[doc = "Register `EXT_OFFn` writer"]
pub type W = crate::W<ExtOffnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Extend OFF Time Register (EXT_OFFn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_offn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_offn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtOffnSpec;
impl crate::RegisterSpec for ExtOffnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ext_offn::R`](R) reader structure"]
impl crate::Readable for ExtOffnSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_offn::W`](W) writer structure"]
impl crate::Writable for ExtOffnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EXT_OFFn to value 0"]
impl crate::Resettable for ExtOffnSpec {
    const RESET_VALUE: u8 = 0;
}
