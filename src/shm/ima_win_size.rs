#[doc = "Register `IMA_WIN_SIZE` reader"]
pub type R = crate::R<ImaWinSizeSpec>;
#[doc = "Register `IMA_WIN_SIZE` writer"]
pub type W = crate::W<ImaWinSizeSpec>;
#[doc = "Field `IMA_SIZE` reader - Indirect Memory Access Size Select"]
pub type ImaSizeR = crate::FieldReader;
#[doc = "Field `IMA_SIZE` writer - Indirect Memory Access Size Select"]
pub type ImaSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Indirect Memory Access Size Select"]
    #[inline(always)]
    pub fn ima_size(&self) -> ImaSizeR {
        ImaSizeR::new(self.bits & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IMA_WIN_SIZE")
            .field("ima_size", &self.ima_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Indirect Memory Access Size Select"]
    #[inline(always)]
    #[must_use]
    pub fn ima_size(&mut self) -> ImaSizeW<ImaWinSizeSpec> {
        ImaSizeW::new(self, 0)
    }
}
#[doc = "Indirect Memory Access Window Size Register (IMA_WIN_SIZE)\n\nYou can [`read`](crate::Reg::read) this register and get [`ima_win_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ima_win_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaWinSizeSpec;
impl crate::RegisterSpec for ImaWinSizeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ima_win_size::R`](R) reader structure"]
impl crate::Readable for ImaWinSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`ima_win_size::W`](W) writer structure"]
impl crate::Writable for ImaWinSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IMA_WIN_SIZE to value 0"]
impl crate::Resettable for ImaWinSizeSpec {
    const RESET_VALUE: u8 = 0;
}
