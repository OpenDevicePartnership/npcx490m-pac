#[doc = "Register `OBUFSTAT` reader"]
pub type R = crate::R<ObufstatSpec>;
#[doc = "Register `OBUFSTAT` writer"]
pub type W = crate::W<ObufstatSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Status\n\nYou can [`read`](crate::Reg::read) this register and get [`obufstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obufstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ObufstatSpec;
impl crate::RegisterSpec for ObufstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obufstat::R`](R) reader structure"]
impl crate::Readable for ObufstatSpec {}
#[doc = "`write(|w| ..)` method takes [`obufstat::W`](W) writer structure"]
impl crate::Writable for ObufstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUFSTAT to value 0"]
impl crate::Resettable for ObufstatSpec {
    const RESET_VALUE: u8 = 0;
}
