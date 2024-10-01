#[doc = "Register `SHIPMnDI` reader"]
pub type R = crate::R<ShipmnDiSpec>;
#[doc = "Register `SHIPMnDI` writer"]
pub type W = crate::W<ShipmnDiSpec>;
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Host Interface PM n Shadow Data In Buffer Register (SHIPMnDI)\n\nYou can [`read`](crate::Reg::read) this register and get [`shipmn_di::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shipmn_di::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShipmnDiSpec;
impl crate::RegisterSpec for ShipmnDiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`shipmn_di::R`](R) reader structure"]
impl crate::Readable for ShipmnDiSpec {}
#[doc = "`write(|w| ..)` method takes [`shipmn_di::W`](W) writer structure"]
impl crate::Writable for ShipmnDiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHIPMnDI to value 0"]
impl crate::Resettable for ShipmnDiSpec {
    const RESET_VALUE: u8 = 0;
}
