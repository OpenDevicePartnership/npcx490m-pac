#[doc = "Register `HFCGML` reader"]
pub type R = crate::R<HfcgmlSpec>;
#[doc = "Register `HFCGML` writer"]
pub type W = crate::W<HfcgmlSpec>;
#[doc = "Field `HFCGM7-0` reader - When written, holds the lower eight bits of the M value to be loaded into the frequency multiplier. When read, this field returns the value currently loaded in the frequency multiplier. HFCGM7-0 is updated by the BootLoader based on the data in the Firmware Header after reset."]
pub type Hfcgm70R = crate::FieldReader;
#[doc = "Field `HFCGM7-0` writer - When written, holds the lower eight bits of the M value to be loaded into the frequency multiplier. When read, this field returns the value currently loaded in the frequency multiplier. HFCGM7-0 is updated by the BootLoader based on the data in the Firmware Header after reset."]
pub type Hfcgm70W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - When written, holds the lower eight bits of the M value to be loaded into the frequency multiplier. When read, this field returns the value currently loaded in the frequency multiplier. HFCGM7-0 is updated by the BootLoader based on the data in the Firmware Header after reset."]
    #[inline(always)]
    pub fn hfcgm70(&self) -> Hfcgm70R {
        Hfcgm70R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCGML")
            .field("hfcgm70", &self.hfcgm70())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - When written, holds the lower eight bits of the M value to be loaded into the frequency multiplier. When read, this field returns the value currently loaded in the frequency multiplier. HFCGM7-0 is updated by the BootLoader based on the data in the Firmware Header after reset."]
    #[inline(always)]
    pub fn hfcgm70(&mut self) -> Hfcgm70W<HfcgmlSpec> {
        Hfcgm70W::new(self, 0)
    }
}
#[doc = "HFCG M Low Value Register (HFCGML)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgml::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgml::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfcgmlSpec;
impl crate::RegisterSpec for HfcgmlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcgml::R`](R) reader structure"]
impl crate::Readable for HfcgmlSpec {}
#[doc = "`write(|w| ..)` method takes [`hfcgml::W`](W) writer structure"]
impl crate::Writable for HfcgmlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCGML to value 0xca"]
impl crate::Resettable for HfcgmlSpec {
    const RESET_VALUE: u8 = 0xca;
}
