#[doc = "Register `HIKDO` reader"]
pub type R = crate::R<HikdoSpec>;
#[doc = "Register `HIKDO` writer"]
pub type W = crate::W<HikdoSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Interface Keyboard Data Out Buffer Register (HIKDO)\n\nYou can [`read`](crate::Reg::read) this register and get [`hikdo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hikdo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HikdoSpec;
impl crate::RegisterSpec for HikdoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hikdo::R`](R) reader structure"]
impl crate::Readable for HikdoSpec {}
#[doc = "`write(|w| ..)` method takes [`hikdo::W`](W) writer structure"]
impl crate::Writable for HikdoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIKDO to value 0"]
impl crate::Resettable for HikdoSpec {
    const RESET_VALUE: u8 = 0;
}
