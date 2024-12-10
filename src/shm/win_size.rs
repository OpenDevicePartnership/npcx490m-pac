#[doc = "Register `WIN_SIZE` reader"]
pub type R = crate::R<WinSizeSpec>;
#[doc = "Register `WIN_SIZE` writer"]
pub type W = crate::W<WinSizeSpec>;
#[doc = "Field `RWIN1_SIZE` reader - RAM Access Window 1, Size Select"]
pub type Rwin1SizeR = crate::FieldReader;
#[doc = "Field `RWIN1_SIZE` writer - RAM Access Window 1, Size Select"]
pub type Rwin1SizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RWIN2_SIZE` reader - RAM Access Window 2, Size Select"]
pub type Rwin2SizeR = crate::FieldReader;
#[doc = "Field `RWIN2_SIZE` writer - RAM Access Window 2, Size Select"]
pub type Rwin2SizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RAM Access Window 1, Size Select"]
    #[inline(always)]
    pub fn rwin1_size(&self) -> Rwin1SizeR {
        Rwin1SizeR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - RAM Access Window 2, Size Select"]
    #[inline(always)]
    pub fn rwin2_size(&self) -> Rwin2SizeR {
        Rwin2SizeR::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIN_SIZE")
            .field("rwin1_size", &self.rwin1_size())
            .field("rwin2_size", &self.rwin2_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM Access Window 1, Size Select"]
    #[inline(always)]
    pub fn rwin1_size(&mut self) -> Rwin1SizeW<WinSizeSpec> {
        Rwin1SizeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - RAM Access Window 2, Size Select"]
    #[inline(always)]
    pub fn rwin2_size(&mut self) -> Rwin2SizeW<WinSizeSpec> {
        Rwin2SizeW::new(self, 4)
    }
}
#[doc = "Shared Access Windows Size Register (WIN_SIZE)\n\nYou can [`read`](crate::Reg::read) this register and get [`win_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`win_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WinSizeSpec;
impl crate::RegisterSpec for WinSizeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`win_size::R`](R) reader structure"]
impl crate::Readable for WinSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`win_size::W`](W) writer structure"]
impl crate::Writable for WinSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WIN_SIZE to value 0"]
impl crate::Resettable for WinSizeSpec {
    const RESET_VALUE: u8 = 0;
}
