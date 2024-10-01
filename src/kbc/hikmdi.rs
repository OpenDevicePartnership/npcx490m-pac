#[doc = "Register `HIKMDI` reader"]
pub type R = crate::R<HikmdiSpec>;
#[doc = "Register `HIKMDI` writer"]
pub type W = crate::W<HikmdiSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Interface Keyboard/Mouse Data In Buffer Register (HIKMDI)\n\nYou can [`read`](crate::Reg::read) this register and get [`hikmdi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hikmdi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HikmdiSpec;
impl crate::RegisterSpec for HikmdiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hikmdi::R`](R) reader structure"]
impl crate::Readable for HikmdiSpec {}
#[doc = "`write(|w| ..)` method takes [`hikmdi::W`](W) writer structure"]
impl crate::Writable for HikmdiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HIKMDI to value 0"]
impl crate::Resettable for HikmdiSpec {
    const RESET_VALUE: u8 = 0;
}
