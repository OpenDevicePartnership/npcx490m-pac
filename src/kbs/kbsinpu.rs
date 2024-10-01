#[doc = "Register `KBSINPU` reader"]
pub type R = crate::R<KbsinpuSpec>;
#[doc = "Register `KBSINPU` writer"]
pub type W = crate::W<KbsinpuSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Keyboard Scan In Pull-Up Enable Register (KBSINPU)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsinpu::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsinpu::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsinpuSpec;
impl crate::RegisterSpec for KbsinpuSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbsinpu::R`](R) reader structure"]
impl crate::Readable for KbsinpuSpec {}
#[doc = "`write(|w| ..)` method takes [`kbsinpu::W`](W) writer structure"]
impl crate::Writable for KbsinpuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBSINPU to value 0"]
impl crate::Resettable for KbsinpuSpec {
    const RESET_VALUE: u8 = 0;
}
