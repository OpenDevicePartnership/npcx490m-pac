#[doc = "Register `SHICFG3` reader"]
pub type R = crate::R<Shicfg3Spec>;
#[doc = "Register `SHICFG3` writer"]
pub type W = crate::W<Shicfg3Spec>;
#[doc = "Field `OBUFLVL` reader - 128-Byte Payload Buffer Level"]
pub type ObuflvlR = crate::FieldReader;
#[doc = "Field `OBUFLVL` writer - 128-Byte Payload Buffer Level"]
pub type ObuflvlW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OBUFLVLDIS` reader - 128-Byte Payload Buffer Level Disabled"]
pub type ObuflvldisR = crate::BitReader;
#[doc = "Field `OBUFLVLDIS` writer - 128-Byte Payload Buffer Level Disabled"]
pub type ObuflvldisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 128-Byte Payload Buffer Level"]
    #[inline(always)]
    pub fn obuflvl(&self) -> ObuflvlR {
        ObuflvlR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - 128-Byte Payload Buffer Level Disabled"]
    #[inline(always)]
    pub fn obuflvldis(&self) -> ObuflvldisR {
        ObuflvldisR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHICFG3")
            .field("obuflvl", &self.obuflvl())
            .field("obuflvldis", &self.obuflvldis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - 128-Byte Payload Buffer Level"]
    #[inline(always)]
    pub fn obuflvl(&mut self) -> ObuflvlW<Shicfg3Spec> {
        ObuflvlW::new(self, 0)
    }
    #[doc = "Bit 7 - 128-Byte Payload Buffer Level Disabled"]
    #[inline(always)]
    pub fn obuflvldis(&mut self) -> ObuflvldisW<Shicfg3Spec> {
        ObuflvldisW::new(self, 7)
    }
}
#[doc = "SHI Configuration 3\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shicfg3Spec;
impl crate::RegisterSpec for Shicfg3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`shicfg3::R`](R) reader structure"]
impl crate::Readable for Shicfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`shicfg3::W`](W) writer structure"]
impl crate::Writable for Shicfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHICFG3 to value 0"]
impl crate::Resettable for Shicfg3Spec {
    const RESET_VALUE: u8 = 0;
}
