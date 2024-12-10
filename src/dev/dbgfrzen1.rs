#[doc = "Register `DBGFRZEN1` reader"]
pub type R = crate::R<Dbgfrzen1Spec>;
#[doc = "Register `DBGFRZEN1` writer"]
pub type W = crate::W<Dbgfrzen1Spec>;
#[doc = "Field `MFT1FEN` reader - MFT16-1 Freeze Enable"]
pub type Mft1fenR = crate::BitReader;
#[doc = "Field `MFT1FEN` writer - MFT16-1 Freeze Enable"]
pub type Mft1fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MFT2FEN` reader - MFT16-2 Freeze Enable"]
pub type Mft2fenR = crate::BitReader;
#[doc = "Field `MFT2FEN` writer - MFT16-2 Freeze Enable"]
pub type Mft2fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB2FEN` reader - SMB2 Freeze Enable"]
pub type Smb2fenR = crate::BitReader;
#[doc = "Field `SMB2FEN` writer - SMB2 Freeze Enable"]
pub type Smb2fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB3FEN` reader - SMB3 Freeze Enable"]
pub type Smb3fenR = crate::BitReader;
#[doc = "Field `SMB3FEN` writer - SMB3 Freeze Enable"]
pub type Smb3fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART1FEN` reader - CR_UART1 Freeze Enable"]
pub type Uart1fenR = crate::BitReader;
#[doc = "Field `UART1FEN` writer - CR_UART1 Freeze Enable"]
pub type Uart1fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESPISEN` reader - eSPI Target Interface Freeze Enable"]
pub type EspisenR = crate::BitReader;
#[doc = "Field `ESPISEN` writer - eSPI Target Interface Freeze Enable"]
pub type EspisenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIFEN` reader - Host Interface Freeze Enable"]
pub type HifenR = crate::BitReader;
#[doc = "Field `HIFEN` writer - Host Interface Freeze Enable"]
pub type HifenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIFEN` reader - SPI Peripheral Freeze Enable"]
pub type SpifenR = crate::BitReader;
#[doc = "Field `SPIFEN` writer - SPI Peripheral Freeze Enable"]
pub type SpifenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MFT16-1 Freeze Enable"]
    #[inline(always)]
    pub fn mft1fen(&self) -> Mft1fenR {
        Mft1fenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MFT16-2 Freeze Enable"]
    #[inline(always)]
    pub fn mft2fen(&self) -> Mft2fenR {
        Mft2fenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMB2 Freeze Enable"]
    #[inline(always)]
    pub fn smb2fen(&self) -> Smb2fenR {
        Smb2fenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SMB3 Freeze Enable"]
    #[inline(always)]
    pub fn smb3fen(&self) -> Smb3fenR {
        Smb3fenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CR_UART1 Freeze Enable"]
    #[inline(always)]
    pub fn uart1fen(&self) -> Uart1fenR {
        Uart1fenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - eSPI Target Interface Freeze Enable"]
    #[inline(always)]
    pub fn espisen(&self) -> EspisenR {
        EspisenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Host Interface Freeze Enable"]
    #[inline(always)]
    pub fn hifen(&self) -> HifenR {
        HifenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Peripheral Freeze Enable"]
    #[inline(always)]
    pub fn spifen(&self) -> SpifenR {
        SpifenR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGFRZEN1")
            .field("mft1fen", &self.mft1fen())
            .field("mft2fen", &self.mft2fen())
            .field("smb2fen", &self.smb2fen())
            .field("smb3fen", &self.smb3fen())
            .field("uart1fen", &self.uart1fen())
            .field("espisen", &self.espisen())
            .field("hifen", &self.hifen())
            .field("spifen", &self.spifen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MFT16-1 Freeze Enable"]
    #[inline(always)]
    pub fn mft1fen(&mut self) -> Mft1fenW<Dbgfrzen1Spec> {
        Mft1fenW::new(self, 0)
    }
    #[doc = "Bit 1 - MFT16-2 Freeze Enable"]
    #[inline(always)]
    pub fn mft2fen(&mut self) -> Mft2fenW<Dbgfrzen1Spec> {
        Mft2fenW::new(self, 1)
    }
    #[doc = "Bit 2 - SMB2 Freeze Enable"]
    #[inline(always)]
    pub fn smb2fen(&mut self) -> Smb2fenW<Dbgfrzen1Spec> {
        Smb2fenW::new(self, 2)
    }
    #[doc = "Bit 3 - SMB3 Freeze Enable"]
    #[inline(always)]
    pub fn smb3fen(&mut self) -> Smb3fenW<Dbgfrzen1Spec> {
        Smb3fenW::new(self, 3)
    }
    #[doc = "Bit 4 - CR_UART1 Freeze Enable"]
    #[inline(always)]
    pub fn uart1fen(&mut self) -> Uart1fenW<Dbgfrzen1Spec> {
        Uart1fenW::new(self, 4)
    }
    #[doc = "Bit 5 - eSPI Target Interface Freeze Enable"]
    #[inline(always)]
    pub fn espisen(&mut self) -> EspisenW<Dbgfrzen1Spec> {
        EspisenW::new(self, 5)
    }
    #[doc = "Bit 6 - Host Interface Freeze Enable"]
    #[inline(always)]
    pub fn hifen(&mut self) -> HifenW<Dbgfrzen1Spec> {
        HifenW::new(self, 6)
    }
    #[doc = "Bit 7 - SPI Peripheral Freeze Enable"]
    #[inline(always)]
    pub fn spifen(&mut self) -> SpifenW<Dbgfrzen1Spec> {
        SpifenW::new(self, 7)
    }
}
#[doc = "Debug Freeze Enable 1 Register (DBGFRZEN1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgfrzen1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgfrzen1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbgfrzen1Spec;
impl crate::RegisterSpec for Dbgfrzen1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbgfrzen1::R`](R) reader structure"]
impl crate::Readable for Dbgfrzen1Spec {}
#[doc = "`write(|w| ..)` method takes [`dbgfrzen1::W`](W) writer structure"]
impl crate::Writable for Dbgfrzen1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DBGFRZEN1 to value 0"]
impl crate::Resettable for Dbgfrzen1Spec {
    const RESET_VALUE: u8 = 0;
}
