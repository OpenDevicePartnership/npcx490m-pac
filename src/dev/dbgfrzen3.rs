#[doc = "Register `DBGFRZEN3` reader"]
pub type R = crate::R<Dbgfrzen3Spec>;
#[doc = "Register `DBGFRZEN3` writer"]
pub type W = crate::W<Dbgfrzen3Spec>;
#[doc = "Field `SHMFEN` reader - Shared Memory Freeze Enable"]
pub type ShmfenR = crate::BitReader;
#[doc = "Field `SHMFEN` writer - Shared Memory Freeze Enable"]
pub type ShmfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB4FEN` reader - SMB4 Freeze Enable"]
pub type Smb4fenR = crate::BitReader;
#[doc = "Field `SMB4FEN` writer - SMB4 Freeze Enable"]
pub type Smb4fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM1FEN` reader - ITIM32-1 Freeze Enable"]
pub type Itim1fenR = crate::BitReader;
#[doc = "Field `ITIM1FEN` writer - ITIM32-1 Freeze Enable"]
pub type Itim1fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM2FEN` reader - ITIM32-2 Freeze Enable"]
pub type Itim2fenR = crate::BitReader;
#[doc = "Field `ITIM2FEN` writer - ITIM32-2 Freeze Enable"]
pub type Itim2fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM3FEN` reader - ITIM32-3 Freeze Enable"]
pub type Itim3fenR = crate::BitReader;
#[doc = "Field `ITIM3FEN` writer - ITIM32-3 Freeze Enable"]
pub type Itim3fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLBL_FRZ_DIS` reader - Global Freeze Disable"]
pub type GlblFrzDisR = crate::BitReader;
#[doc = "Field `GLBL_FRZ_DIS` writer - Global Freeze Disable"]
pub type GlblFrzDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shared Memory Freeze Enable"]
    #[inline(always)]
    pub fn shmfen(&self) -> ShmfenR {
        ShmfenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMB4 Freeze Enable"]
    #[inline(always)]
    pub fn smb4fen(&self) -> Smb4fenR {
        Smb4fenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - ITIM32-1 Freeze Enable"]
    #[inline(always)]
    pub fn itim1fen(&self) -> Itim1fenR {
        Itim1fenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ITIM32-2 Freeze Enable"]
    #[inline(always)]
    pub fn itim2fen(&self) -> Itim2fenR {
        Itim2fenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ITIM32-3 Freeze Enable"]
    #[inline(always)]
    pub fn itim3fen(&self) -> Itim3fenR {
        Itim3fenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global Freeze Disable"]
    #[inline(always)]
    pub fn glbl_frz_dis(&self) -> GlblFrzDisR {
        GlblFrzDisR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGFRZEN3")
            .field("shmfen", &self.shmfen())
            .field("smb4fen", &self.smb4fen())
            .field("itim1fen", &self.itim1fen())
            .field("itim2fen", &self.itim2fen())
            .field("itim3fen", &self.itim3fen())
            .field("glbl_frz_dis", &self.glbl_frz_dis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Shared Memory Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shmfen(&mut self) -> ShmfenW<Dbgfrzen3Spec> {
        ShmfenW::new(self, 0)
    }
    #[doc = "Bit 1 - SMB4 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb4fen(&mut self) -> Smb4fenW<Dbgfrzen3Spec> {
        Smb4fenW::new(self, 1)
    }
    #[doc = "Bit 4 - ITIM32-1 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn itim1fen(&mut self) -> Itim1fenW<Dbgfrzen3Spec> {
        Itim1fenW::new(self, 4)
    }
    #[doc = "Bit 5 - ITIM32-2 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn itim2fen(&mut self) -> Itim2fenW<Dbgfrzen3Spec> {
        Itim2fenW::new(self, 5)
    }
    #[doc = "Bit 6 - ITIM32-3 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn itim3fen(&mut self) -> Itim3fenW<Dbgfrzen3Spec> {
        Itim3fenW::new(self, 6)
    }
    #[doc = "Bit 7 - Global Freeze Disable"]
    #[inline(always)]
    #[must_use]
    pub fn glbl_frz_dis(&mut self) -> GlblFrzDisW<Dbgfrzen3Spec> {
        GlblFrzDisW::new(self, 7)
    }
}
#[doc = "Debug Freeze Enable 3 Register (DBGFRZEN3)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgfrzen3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgfrzen3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbgfrzen3Spec;
impl crate::RegisterSpec for Dbgfrzen3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbgfrzen3::R`](R) reader structure"]
impl crate::Readable for Dbgfrzen3Spec {}
#[doc = "`write(|w| ..)` method takes [`dbgfrzen3::W`](W) writer structure"]
impl crate::Writable for Dbgfrzen3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DBGFRZEN3 to value 0"]
impl crate::Resettable for Dbgfrzen3Spec {
    const RESET_VALUE: u8 = 0;
}
