#[doc = "Register `DBGFRZEN2` reader"]
pub type R = crate::R<Dbgfrzen2Spec>;
#[doc = "Register `DBGFRZEN2` writer"]
pub type W = crate::W<Dbgfrzen2Spec>;
#[doc = "Field `MFT3FEN` reader - MFT16-3 Freeze Enable"]
pub type Mft3fenR = crate::BitReader;
#[doc = "Field `MFT3FEN` writer - MFT16-3 Freeze Enable"]
pub type Mft3fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB0FEN` reader - SMB0 Freeze Enable"]
pub type Smb0fenR = crate::BitReader;
#[doc = "Field `SMB0FEN` writer - SMB0 Freeze Enable"]
pub type Smb0fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB1FEN` reader - SMB1 Freeze Enable"]
pub type Smb1fenR = crate::BitReader;
#[doc = "Field `SMB1FEN` writer - SMB1 Freeze Enable"]
pub type Smb1fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM64FEN` reader - ITIM64 Freeze Enable"]
pub type Itim64fenR = crate::BitReader;
#[doc = "Field `ITIM64FEN` writer - ITIM64 Freeze Enable"]
pub type Itim64fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM4FEN` reader - ITIM32-4 Freeze Enable"]
pub type Itim4fenR = crate::BitReader;
#[doc = "Field `ITIM4FEN` writer - ITIM32-4 Freeze Enable"]
pub type Itim4fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM5FEN` reader - ITIM32-5 Freeze Enable"]
pub type Itim5fenR = crate::BitReader;
#[doc = "Field `ITIM5FEN` writer - ITIM32-5 Freeze Enable"]
pub type Itim5fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM6FEN` reader - ITIM32-6 Freeze Enable"]
pub type Itim6fenR = crate::BitReader;
#[doc = "Field `ITIM6FEN` writer - ITIM32-6 Freeze Enable"]
pub type Itim6fenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MFT16-3 Freeze Enable"]
    #[inline(always)]
    pub fn mft3fen(&self) -> Mft3fenR {
        Mft3fenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMB0 Freeze Enable"]
    #[inline(always)]
    pub fn smb0fen(&self) -> Smb0fenR {
        Smb0fenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMB1 Freeze Enable"]
    #[inline(always)]
    pub fn smb1fen(&self) -> Smb1fenR {
        Smb1fenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ITIM64 Freeze Enable"]
    #[inline(always)]
    pub fn itim64fen(&self) -> Itim64fenR {
        Itim64fenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - ITIM32-4 Freeze Enable"]
    #[inline(always)]
    pub fn itim4fen(&self) -> Itim4fenR {
        Itim4fenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ITIM32-5 Freeze Enable"]
    #[inline(always)]
    pub fn itim5fen(&self) -> Itim5fenR {
        Itim5fenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ITIM32-6 Freeze Enable"]
    #[inline(always)]
    pub fn itim6fen(&self) -> Itim6fenR {
        Itim6fenR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGFRZEN2")
            .field("mft3fen", &self.mft3fen())
            .field("smb0fen", &self.smb0fen())
            .field("smb1fen", &self.smb1fen())
            .field("itim64fen", &self.itim64fen())
            .field("itim4fen", &self.itim4fen())
            .field("itim5fen", &self.itim5fen())
            .field("itim6fen", &self.itim6fen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MFT16-3 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mft3fen(&mut self) -> Mft3fenW<Dbgfrzen2Spec> {
        Mft3fenW::new(self, 0)
    }
    #[doc = "Bit 1 - SMB0 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb0fen(&mut self) -> Smb0fenW<Dbgfrzen2Spec> {
        Smb0fenW::new(self, 1)
    }
    #[doc = "Bit 2 - SMB1 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb1fen(&mut self) -> Smb1fenW<Dbgfrzen2Spec> {
        Smb1fenW::new(self, 2)
    }
    #[doc = "Bit 3 - ITIM64 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn itim64fen(&mut self) -> Itim64fenW<Dbgfrzen2Spec> {
        Itim64fenW::new(self, 3)
    }
    #[doc = "Bit 5 - ITIM32-4 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn itim4fen(&mut self) -> Itim4fenW<Dbgfrzen2Spec> {
        Itim4fenW::new(self, 5)
    }
    #[doc = "Bit 6 - ITIM32-5 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn itim5fen(&mut self) -> Itim5fenW<Dbgfrzen2Spec> {
        Itim5fenW::new(self, 6)
    }
    #[doc = "Bit 7 - ITIM32-6 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn itim6fen(&mut self) -> Itim6fenW<Dbgfrzen2Spec> {
        Itim6fenW::new(self, 7)
    }
}
#[doc = "Debug Freeze Enable 2 Register (DBGFRZEN2)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgfrzen2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgfrzen2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbgfrzen2Spec;
impl crate::RegisterSpec for Dbgfrzen2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbgfrzen2::R`](R) reader structure"]
impl crate::Readable for Dbgfrzen2Spec {}
#[doc = "`write(|w| ..)` method takes [`dbgfrzen2::W`](W) writer structure"]
impl crate::Writable for Dbgfrzen2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DBGFRZEN2 to value 0"]
impl crate::Resettable for Dbgfrzen2Spec {
    const RESET_VALUE: u8 = 0;
}
