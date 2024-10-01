#[doc = "Register `WKAEDGn3` reader"]
pub type R = crate::R<Wkaedgn3Spec>;
#[doc = "Register `WKAEDGn3` writer"]
pub type W = crate::W<Wkaedgn3Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wkaedgn3Spec;
impl crate::RegisterSpec for Wkaedgn3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkaedgn3::R`](R) reader structure"]
impl crate::Readable for Wkaedgn3Spec {}
#[doc = "`write(|w| ..)` method takes [`wkaedgn3::W`](W) writer structure"]
impl crate::Writable for Wkaedgn3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKAEDGn3 to value 0"]
impl crate::Resettable for Wkaedgn3Spec {
    const RESET_VALUE: u8 = 0;
}
