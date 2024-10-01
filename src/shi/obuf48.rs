#[doc = "Register `OBUF48` reader"]
pub type R = crate::R<Obuf48Spec>;
#[doc = "Register `OBUF48` writer"]
pub type W = crate::W<Obuf48Spec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Output Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`obuf48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`obuf48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Obuf48Spec;
impl crate::RegisterSpec for Obuf48Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`obuf48::R`](R) reader structure"]
impl crate::Readable for Obuf48Spec {}
#[doc = "`write(|w| ..)` method takes [`obuf48::W`](W) writer structure"]
impl crate::Writable for Obuf48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OBUF48 to value 0"]
impl crate::Resettable for Obuf48Spec {
    const RESET_VALUE: u8 = 0;
}
