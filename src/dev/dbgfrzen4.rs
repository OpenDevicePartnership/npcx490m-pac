#[doc = "Register `DBGFRZEN4` reader"]
pub type R = crate::R<Dbgfrzen4Spec>;
#[doc = "Register `DBGFRZEN4` writer"]
pub type W = crate::W<Dbgfrzen4Spec>;
#[doc = "Field `SMB5FEN` reader - SMB5 Freeze Enable"]
pub type Smb5fenR = crate::BitReader;
#[doc = "Field `SMB5FEN` writer - SMB5 Freeze Enable"]
pub type Smb5fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB6FEN` reader - SMB6 Freeze Enable"]
pub type Smb6fenR = crate::BitReader;
#[doc = "Field `SMB6FEN` writer - SMB6 Freeze Enable"]
pub type Smb6fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB7FEN` reader - SMB7 Freeze Enable"]
pub type Smb7fenR = crate::BitReader;
#[doc = "Field `SMB7FEN` writer - SMB7 Freeze Enable"]
pub type Smb7fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCTFEN` reader - LCT Freeze Enable"]
pub type LctfenR = crate::BitReader;
#[doc = "Field `LCTFEN` writer - LCT Freeze Enable"]
pub type LctfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4FEN` reader - CR_UART4 Freeze Enable"]
pub type Uart4fenR = crate::BitReader;
#[doc = "Field `UART4FEN` writer - CR_UART4 Freeze Enable"]
pub type Uart4fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3FEN` reader - CR_UART3 Freeze Enable"]
pub type Uart3fenR = crate::BitReader;
#[doc = "Field `UART3FEN` writer - CR_UART3 Freeze Enable"]
pub type Uart3fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2FEN` reader - CR_UART2 Freeze Enable"]
pub type Uart2fenR = crate::BitReader;
#[doc = "Field `UART2FEN` writer - CR_UART2 Freeze Enable"]
pub type Uart2fenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SMB5 Freeze Enable"]
    #[inline(always)]
    pub fn smb5fen(&self) -> Smb5fenR {
        Smb5fenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMB6 Freeze Enable"]
    #[inline(always)]
    pub fn smb6fen(&self) -> Smb6fenR {
        Smb6fenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMB7 Freeze Enable"]
    #[inline(always)]
    pub fn smb7fen(&self) -> Smb7fenR {
        Smb7fenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LCT Freeze Enable"]
    #[inline(always)]
    pub fn lctfen(&self) -> LctfenR {
        LctfenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CR_UART4 Freeze Enable"]
    #[inline(always)]
    pub fn uart4fen(&self) -> Uart4fenR {
        Uart4fenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CR_UART3 Freeze Enable"]
    #[inline(always)]
    pub fn uart3fen(&self) -> Uart3fenR {
        Uart3fenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CR_UART2 Freeze Enable"]
    #[inline(always)]
    pub fn uart2fen(&self) -> Uart2fenR {
        Uart2fenR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGFRZEN4")
            .field("smb5fen", &self.smb5fen())
            .field("smb6fen", &self.smb6fen())
            .field("smb7fen", &self.smb7fen())
            .field("lctfen", &self.lctfen())
            .field("uart4fen", &self.uart4fen())
            .field("uart3fen", &self.uart3fen())
            .field("uart2fen", &self.uart2fen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SMB5 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb5fen(&mut self) -> Smb5fenW<Dbgfrzen4Spec> {
        Smb5fenW::new(self, 0)
    }
    #[doc = "Bit 1 - SMB6 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb6fen(&mut self) -> Smb6fenW<Dbgfrzen4Spec> {
        Smb6fenW::new(self, 1)
    }
    #[doc = "Bit 2 - SMB7 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn smb7fen(&mut self) -> Smb7fenW<Dbgfrzen4Spec> {
        Smb7fenW::new(self, 2)
    }
    #[doc = "Bit 3 - LCT Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lctfen(&mut self) -> LctfenW<Dbgfrzen4Spec> {
        LctfenW::new(self, 3)
    }
    #[doc = "Bit 4 - CR_UART4 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart4fen(&mut self) -> Uart4fenW<Dbgfrzen4Spec> {
        Uart4fenW::new(self, 4)
    }
    #[doc = "Bit 5 - CR_UART3 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart3fen(&mut self) -> Uart3fenW<Dbgfrzen4Spec> {
        Uart3fenW::new(self, 5)
    }
    #[doc = "Bit 6 - CR_UART2 Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart2fen(&mut self) -> Uart2fenW<Dbgfrzen4Spec> {
        Uart2fenW::new(self, 6)
    }
}
#[doc = "Debug Freeze Enable 4 Register (DBGFRZEN4)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgfrzen4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgfrzen4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbgfrzen4Spec;
impl crate::RegisterSpec for Dbgfrzen4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbgfrzen4::R`](R) reader structure"]
impl crate::Readable for Dbgfrzen4Spec {}
#[doc = "`write(|w| ..)` method takes [`dbgfrzen4::W`](W) writer structure"]
impl crate::Writable for Dbgfrzen4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DBGFRZEN4 to value 0"]
impl crate::Resettable for Dbgfrzen4Spec {
    const RESET_VALUE: u8 = 0;
}
