#[doc = "Register `SHIKMDI` reader"]
pub type R = crate::R<ShikmdiSpec>;
#[doc = "Register `SHIKMDI` writer"]
pub type W = crate::W<ShikmdiSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Interface Keyboard/Mouse Shadow Data In Buffer Register (SHIKMDI)\n\nYou can [`read`](crate::Reg::read) this register and get [`shikmdi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shikmdi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShikmdiSpec;
impl crate::RegisterSpec for ShikmdiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`shikmdi::R`](R) reader structure"]
impl crate::Readable for ShikmdiSpec {}
#[doc = "`write(|w| ..)` method takes [`shikmdi::W`](W) writer structure"]
impl crate::Writable for ShikmdiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHIKMDI to value 0"]
impl crate::Resettable for ShikmdiSpec {
    const RESET_VALUE: u8 = 0;
}
