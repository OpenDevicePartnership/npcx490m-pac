#[doc = "Register `SHICFG4` reader"]
pub type R = crate::R<Shicfg4Spec>;
#[doc = "Register `SHICFG4` writer"]
pub type W = crate::W<Shicfg4Spec>;
#[doc = "Field `IBUFLVL` reader - Input Buffer Level"]
pub type IbuflvlR = crate::FieldReader;
#[doc = "Field `IBUFLVL` writer - Input Buffer Level"]
pub type IbuflvlW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `IBUFLVLDIS` reader - Input Buffer Level Disabled"]
pub type IbuflvldisR = crate::BitReader;
#[doc = "Field `IBUFLVLDIS` writer - Input Buffer Level Disabled"]
pub type IbuflvldisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Input Buffer Level"]
    #[inline(always)]
    pub fn ibuflvl(&self) -> IbuflvlR {
        IbuflvlR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Input Buffer Level Disabled"]
    #[inline(always)]
    pub fn ibuflvldis(&self) -> IbuflvldisR {
        IbuflvldisR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHICFG4")
            .field("ibuflvl", &self.ibuflvl())
            .field("ibuflvldis", &self.ibuflvldis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Input Buffer Level"]
    #[inline(always)]
    #[must_use]
    pub fn ibuflvl(&mut self) -> IbuflvlW<Shicfg4Spec> {
        IbuflvlW::new(self, 0)
    }
    #[doc = "Bit 7 - Input Buffer Level Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn ibuflvldis(&mut self) -> IbuflvldisW<Shicfg4Spec> {
        IbuflvldisW::new(self, 7)
    }
}
#[doc = "SHI Configuration 4\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shicfg4Spec;
impl crate::RegisterSpec for Shicfg4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`shicfg4::R`](R) reader structure"]
impl crate::Readable for Shicfg4Spec {}
#[doc = "`write(|w| ..)` method takes [`shicfg4::W`](W) writer structure"]
impl crate::Writable for Shicfg4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHICFG4 to value 0"]
impl crate::Resettable for Shicfg4Spec {
    const RESET_VALUE: u8 = 0;
}
