#[doc = "Register `WKSTn%s` reader"]
pub type R = crate::R<WkstnSpec>;
#[doc = "Register `WKSTn%s` writer"]
pub type W = crate::W<WkstnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Wake-Up Status nx Register (WKSTnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkstn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkstn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkstnSpec;
impl crate::RegisterSpec for WkstnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkstn::R`](R) reader structure"]
impl crate::Readable for WkstnSpec {}
#[doc = "`write(|w| ..)` method takes [`wkstn::W`](W) writer structure"]
impl crate::Writable for WkstnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKSTn%s to value 0"]
impl crate::Resettable for WkstnSpec {
    const RESET_VALUE: u8 = 0;
}
