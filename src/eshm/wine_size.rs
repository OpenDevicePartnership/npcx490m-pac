#[doc = "Register `WINE_SIZE` reader"]
pub type R = crate::R<WineSizeSpec>;
#[doc = "Register `WINE_SIZE` writer"]
pub type W = crate::W<WineSizeSpec>;
#[doc = "Field `RWIN3_SIZE` reader - RAM Access Window 3, Size Select"]
pub type Rwin3SizeR = crate::FieldReader;
#[doc = "Field `RWIN3_SIZE` writer - RAM Access Window 3, Size Select"]
pub type Rwin3SizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RWIN4_SIZE` reader - RAM Access Window 4, Size Select"]
pub type Rwin4SizeR = crate::FieldReader;
#[doc = "Field `RWIN4_SIZE` writer - RAM Access Window 4, Size Select"]
pub type Rwin4SizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RAM Access Window 3, Size Select"]
    #[inline(always)]
    pub fn rwin3_size(&self) -> Rwin3SizeR {
        Rwin3SizeR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - RAM Access Window 4, Size Select"]
    #[inline(always)]
    pub fn rwin4_size(&self) -> Rwin4SizeR {
        Rwin4SizeR::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WINE_SIZE")
            .field("rwin3_size", &self.rwin3_size())
            .field("rwin4_size", &self.rwin4_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - RAM Access Window 3, Size Select"]
    #[inline(always)]
    pub fn rwin3_size(&mut self) -> Rwin3SizeW<WineSizeSpec> {
        Rwin3SizeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - RAM Access Window 4, Size Select"]
    #[inline(always)]
    pub fn rwin4_size(&mut self) -> Rwin4SizeW<WineSizeSpec> {
        Rwin4SizeW::new(self, 4)
    }
}
#[doc = "Extended Shared Access Windows Size Register (WINE_SIZE)\n\nYou can [`read`](crate::Reg::read) this register and get [`wine_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wine_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WineSizeSpec;
impl crate::RegisterSpec for WineSizeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wine_size::R`](R) reader structure"]
impl crate::Readable for WineSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`wine_size::W`](W) writer structure"]
impl crate::Writable for WineSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WINE_SIZE to value 0"]
impl crate::Resettable for WineSizeSpec {
    const RESET_VALUE: u8 = 0;
}
