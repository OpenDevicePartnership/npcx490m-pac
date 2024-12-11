#[doc = "Register `RMAP_FLASH_OFFS` reader"]
pub type R = crate::R<RmapFlashOffsSpec>;
#[doc = "Register `RMAP_FLASH_OFFS` writer"]
pub type W = crate::W<RmapFlashOffsSpec>;
#[doc = "Field `RMAP_FLASH_OFFS` reader - Remapping Flash Offset"]
pub type RmapFlashOffsR = crate::FieldReader<u32>;
#[doc = "Field `RMAP_FLASH_OFFS` writer - Remapping Flash Offset"]
pub type RmapFlashOffsW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:26 - Remapping Flash Offset"]
    #[inline(always)]
    pub fn rmap_flash_offs(&self) -> RmapFlashOffsR {
        RmapFlashOffsR::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMAP_FLASH_OFFS")
            .field("rmap_flash_offs", &self.rmap_flash_offs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:26 - Remapping Flash Offset"]
    #[inline(always)]
    pub fn rmap_flash_offs(&mut self) -> RmapFlashOffsW<RmapFlashOffsSpec> {
        RmapFlashOffsW::new(self, 0)
    }
}
#[doc = "Remapping Flash Offset Register (RMAP_FLASH_OFFS)\n\nYou can [`read`](crate::Reg::read) this register and get [`rmap_flash_offs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmap_flash_offs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmapFlashOffsSpec;
impl crate::RegisterSpec for RmapFlashOffsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmap_flash_offs::R`](R) reader structure"]
impl crate::Readable for RmapFlashOffsSpec {}
#[doc = "`write(|w| ..)` method takes [`rmap_flash_offs::W`](W) writer structure"]
impl crate::Writable for RmapFlashOffsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMAP_FLASH_OFFS to value 0"]
impl crate::Resettable for RmapFlashOffsSpec {
    const RESET_VALUE: u32 = 0;
}
