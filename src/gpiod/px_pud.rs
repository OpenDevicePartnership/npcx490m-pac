#[doc = "Register `PxPUD` reader"]
pub type R = crate::R<PxPudSpec>;
#[doc = "Register `PxPUD` writer"]
pub type W = crate::W<PxPudSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port GPIOx Pull-Up/Down Selection Register (PxPUD)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_pud::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_pud::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PxPudSpec;
impl crate::RegisterSpec for PxPudSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`px_pud::R`](R) reader structure"]
impl crate::Readable for PxPudSpec {}
#[doc = "`write(|w| ..)` method takes [`px_pud::W`](W) writer structure"]
impl crate::Writable for PxPudSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PxPUD to value 0"]
impl crate::Resettable for PxPudSpec {
    const RESET_VALUE: u8 = 0;
}
