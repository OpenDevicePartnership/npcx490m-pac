#[doc = "Register `PxDIR` reader"]
pub type R = crate::R<PxDirSpec>;
#[doc = "Register `PxDIR` writer"]
pub type W = crate::W<PxDirSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Port GPIOx Direction Register (PxDIR)\n\nYou can [`read`](crate::Reg::read) this register and get [`px_dir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`px_dir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PxDirSpec;
impl crate::RegisterSpec for PxDirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`px_dir::R`](R) reader structure"]
impl crate::Readable for PxDirSpec {}
#[doc = "`write(|w| ..)` method takes [`px_dir::W`](W) writer structure"]
impl crate::Writable for PxDirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PxDIR to value 0"]
impl crate::Resettable for PxDirSpec {
    const RESET_VALUE: u8 = 0;
}
