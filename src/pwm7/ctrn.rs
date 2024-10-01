#[doc = "Register `CTRn` reader"]
pub type R = crate::R<CtrnSpec>;
#[doc = "Register `CTRn` writer"]
pub type W = crate::W<CtrnSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Cycle Time Register (CTRn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrnSpec;
impl crate::RegisterSpec for CtrnSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrn::R`](R) reader structure"]
impl crate::Readable for CtrnSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrn::W`](W) writer structure"]
impl crate::Writable for CtrnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRn to value 0"]
impl crate::Resettable for CtrnSpec {
    const RESET_VALUE: u16 = 0;
}
