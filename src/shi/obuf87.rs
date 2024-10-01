#[doc = "Register `OBUF87` reader"]
pub type R = crate::R<Obuf87Spec>;
#[doc = "Register `OBUF87` writer"]
pub type W = crate::W<Obuf87Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf87::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf87::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obuf87Spec;
impl crate::RegisterSpec for Obuf87Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obuf87::R`](R) reader structure"]
impl crate::Readable for Obuf87Spec {}
#[doc = "`write(|w| ..)` method takes [`obuf87::W`](W) writer structure"]
impl crate::Writable for Obuf87Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUF87 to value 0"]
impl crate::Resettable for Obuf87Spec {
    const RESET_VALUE: u8 = 0;
}
