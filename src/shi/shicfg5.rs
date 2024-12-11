#[doc = "Register `SHICFG5` reader"]
pub type R = crate::R<Shicfg5Spec>;
#[doc = "Register `SHICFG5` writer"]
pub type W = crate::W<Shicfg5Spec>;
#[doc = "Field `IBUFLVL2` reader - Input Buffer Level 2"]
pub type Ibuflvl2R = crate::FieldReader;
#[doc = "Field `IBUFLVL2` writer - Input Buffer Level 2"]
pub type Ibuflvl2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `IBUFLVL2DIS` reader - Input Buffer Level 2 Disabled"]
pub type Ibuflvl2disR = crate::BitReader;
#[doc = "Field `IBUFLVL2DIS` writer - Input Buffer Level 2 Disabled"]
pub type Ibuflvl2disW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Input Buffer Level 2"]
    #[inline(always)]
    pub fn ibuflvl2(&self) -> Ibuflvl2R {
        Ibuflvl2R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Input Buffer Level 2 Disabled"]
    #[inline(always)]
    pub fn ibuflvl2dis(&self) -> Ibuflvl2disR {
        Ibuflvl2disR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHICFG5")
            .field("ibuflvl2", &self.ibuflvl2())
            .field("ibuflvl2dis", &self.ibuflvl2dis())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Input Buffer Level 2"]
    #[inline(always)]
    pub fn ibuflvl2(&mut self) -> Ibuflvl2W<Shicfg5Spec> {
        Ibuflvl2W::new(self, 0)
    }
    #[doc = "Bit 7 - Input Buffer Level 2 Disabled"]
    #[inline(always)]
    pub fn ibuflvl2dis(&mut self) -> Ibuflvl2disW<Shicfg5Spec> {
        Ibuflvl2disW::new(self, 7)
    }
}
#[doc = "SHI Configuration 5\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shicfg5Spec;
impl crate::RegisterSpec for Shicfg5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`shicfg5::R`](R) reader structure"]
impl crate::Readable for Shicfg5Spec {}
#[doc = "`write(|w| ..)` method takes [`shicfg5::W`](W) writer structure"]
impl crate::Writable for Shicfg5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHICFG5 to value 0"]
impl crate::Resettable for Shicfg5Spec {
    const RESET_VALUE: u8 = 0;
}
