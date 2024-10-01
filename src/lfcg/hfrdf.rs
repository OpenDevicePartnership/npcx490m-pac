#[doc = "Register `HFRDF` reader"]
pub type R = crate::R<HfrdfSpec>;
#[doc = "Register `HFRDF` writer"]
pub type W = crate::W<HfrdfSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "High-Frequency Reference Divisor F Register (HFRDF)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrdf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfrdf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfrdfSpec;
impl crate::RegisterSpec for HfrdfSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hfrdf::R`](R) reader structure"]
impl crate::Readable for HfrdfSpec {}
#[doc = "`write(|w| ..)` method takes [`hfrdf::W`](W) writer structure"]
impl crate::Writable for HfrdfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HFRDF to value 0"]
impl crate::Resettable for HfrdfSpec {
    const RESET_VALUE: u16 = 0;
}
