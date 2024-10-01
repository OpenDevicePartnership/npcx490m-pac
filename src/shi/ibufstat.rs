#[doc = "Register `IBUFSTAT` reader"]
pub type R = crate::R<IbufstatSpec>;
#[doc = "Register `IBUFSTAT` writer"]
pub type W = crate::W<IbufstatSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Input Buffer Status\n\nYou can [`read`](crate::Reg::read) this register and get [`ibufstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibufstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IbufstatSpec;
impl crate::RegisterSpec for IbufstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ibufstat::R`](R) reader structure"]
impl crate::Readable for IbufstatSpec {}
#[doc = "`write(|w| ..)` method takes [`ibufstat::W`](W) writer structure"]
impl crate::Writable for IbufstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IBUFSTAT to value 0"]
impl crate::Resettable for IbufstatSpec {
    const RESET_VALUE: u8 = 0;
}
