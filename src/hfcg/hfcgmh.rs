#[doc = "Register `HFCGMH` reader"]
pub type R = crate::R<HfcgmhSpec>;
#[doc = "Register `HFCGMH` writer"]
pub type W = crate::W<HfcgmhSpec>;
#[doc = "Field `HFCGM15-8` reader - When written, holds the upper eight bits of the M value to be loaded into the frequency multiplier. When read, this field returns the value currently loaded in the frequency multiplier. HFCGM15-8 is updated by the BootLoader based on the data in the Firmware Header after reset."]
pub type Hfcgm158R = crate::FieldReader;
#[doc = "Field `HFCGM15-8` writer - When written, holds the upper eight bits of the M value to be loaded into the frequency multiplier. When read, this field returns the value currently loaded in the frequency multiplier. HFCGM15-8 is updated by the BootLoader based on the data in the Firmware Header after reset."]
pub type Hfcgm158W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - When written, holds the upper eight bits of the M value to be loaded into the frequency multiplier. When read, this field returns the value currently loaded in the frequency multiplier. HFCGM15-8 is updated by the BootLoader based on the data in the Firmware Header after reset."]
    #[inline(always)]
    pub fn hfcgm158(&self) -> Hfcgm158R {
        Hfcgm158R::new(self.bits)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCGMH")
            .field("hfcgm158", &self.hfcgm158())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - When written, holds the upper eight bits of the M value to be loaded into the frequency multiplier. When read, this field returns the value currently loaded in the frequency multiplier. HFCGM15-8 is updated by the BootLoader based on the data in the Firmware Header after reset."]
    #[inline(always)]
    pub fn hfcgm158(&mut self) -> Hfcgm158W<HfcgmhSpec> {
        Hfcgm158W::new(self, 0)
    }
}
#[doc = "HFCG M High Value Register (HFCGMH)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgmh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgmh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfcgmhSpec;
impl crate::RegisterSpec for HfcgmhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcgmh::R`](R) reader structure"]
impl crate::Readable for HfcgmhSpec {}
#[doc = "`write(|w| ..)` method takes [`hfcgmh::W`](W) writer structure"]
impl crate::Writable for HfcgmhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCGMH to value 0x0a"]
impl crate::Resettable for HfcgmhSpec {
    const RESET_VALUE: u8 = 0x0a;
}
