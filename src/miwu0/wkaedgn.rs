#[doc = "Register `WKAEDGn%s` reader"]
pub type R = crate::R<WkaedgnSpec>;
#[doc = "Register `WKAEDGn%s` writer"]
pub type W = crate::W<WkaedgnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkaedgnSpec;
impl crate::RegisterSpec for WkaedgnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkaedgn::R`](R) reader structure"]
impl crate::Readable for WkaedgnSpec {}
#[doc = "`write(|w| ..)` method takes [`wkaedgn::W`](W) writer structure"]
impl crate::Writable for WkaedgnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKAEDGn%s to value 0"]
impl crate::Resettable for WkaedgnSpec {
    const RESET_VALUE: u8 = 0;
}
