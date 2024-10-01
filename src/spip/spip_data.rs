#[doc = "Register `SPIP_DATA` reader"]
pub type R = crate::R<SpipDataSpec>;
#[doc = "Register `SPIP_DATA` writer"]
pub type W = crate::W<SpipDataSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "SPIP Data In Out Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spip_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spip_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpipDataSpec;
impl crate::RegisterSpec for SpipDataSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`spip_data::R`](R) reader structure"]
impl crate::Readable for SpipDataSpec {}
#[doc = "`write(|w| ..)` method takes [`spip_data::W`](W) writer structure"]
impl crate::Writable for SpipDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SPIP_DATA to value 0"]
impl crate::Resettable for SpipDataSpec {
    const RESET_VALUE: u16 = 0;
}
