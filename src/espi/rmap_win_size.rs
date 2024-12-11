#[doc = "Register `RMAP_WIN_SIZE` reader"]
pub type R = crate::R<RmapWinSizeSpec>;
#[doc = "Register `RMAP_WIN_SIZE` writer"]
pub type W = crate::W<RmapWinSizeSpec>;
#[doc = "Field `SAF_RMAP_LCK` reader - Target-Attached Flash Remap Lock"]
pub type SafRmapLckR = crate::BitReader;
#[doc = "Field `SAF_RMAP_LCK` writer - Target-Attached Flash Remap Lock"]
pub type SafRmapLckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMAP_WIN_SZ` reader - Mapping Window Size"]
pub type RmapWinSzR = crate::FieldReader<u32>;
#[doc = "Field `RMAP_WIN_SZ` writer - Mapping Window Size"]
pub type RmapWinSzW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0 - Target-Attached Flash Remap Lock"]
    #[inline(always)]
    pub fn saf_rmap_lck(&self) -> SafRmapLckR {
        SafRmapLckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 11:28 - Mapping Window Size"]
    #[inline(always)]
    pub fn rmap_win_sz(&self) -> RmapWinSzR {
        RmapWinSzR::new((self.bits >> 11) & 0x0003_ffff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMAP_WIN_SIZE")
            .field("saf_rmap_lck", &self.saf_rmap_lck())
            .field("rmap_win_sz", &self.rmap_win_sz())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Target-Attached Flash Remap Lock"]
    #[inline(always)]
    pub fn saf_rmap_lck(&mut self) -> SafRmapLckW<RmapWinSizeSpec> {
        SafRmapLckW::new(self, 0)
    }
    #[doc = "Bits 11:28 - Mapping Window Size"]
    #[inline(always)]
    pub fn rmap_win_sz(&mut self) -> RmapWinSzW<RmapWinSizeSpec> {
        RmapWinSzW::new(self, 11)
    }
}
#[doc = "Remapping Window Size Register (RMAP_WIN_SIZE)\n\nYou can [`read`](crate::Reg::read) this register and get [`rmap_win_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmap_win_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmapWinSizeSpec;
impl crate::RegisterSpec for RmapWinSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmap_win_size::R`](R) reader structure"]
impl crate::Readable for RmapWinSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`rmap_win_size::W`](W) writer structure"]
impl crate::Writable for RmapWinSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMAP_WIN_SIZE to value 0"]
impl crate::Resettable for RmapWinSizeSpec {
    const RESET_VALUE: u32 = 0;
}
