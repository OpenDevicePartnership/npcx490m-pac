#[doc = "Register `UFTCTLn` reader"]
pub type R = crate::R<UftctlnSpec>;
#[doc = "Register `UFTCTLn` writer"]
pub type W = crate::W<UftctlnSpec>;
#[doc = "Field `TEMPTY_LEVEL_SEL` reader - Transmit FIFO Empty Level Select"]
pub type TemptyLevelSelR = crate::FieldReader;
#[doc = "Field `TEMPTY_LEVEL_SEL` writer - Transmit FIFO Empty Level Select"]
pub type TemptyLevelSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TEMPTY_LEVEL_EN` reader - Transmit FIFO Empty Level Interrupt Enable"]
pub type TemptyLevelEnR = crate::BitReader;
#[doc = "Field `TEMPTY_LEVEL_EN` writer - Transmit FIFO Empty Level Interrupt Enable"]
pub type TemptyLevelEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIFO_EMPTY_EN` reader - Transmit FIFO Empty Interrupt Enable"]
pub type TfifoEmptyEnR = crate::BitReader;
#[doc = "Field `TFIFO_EMPTY_EN` writer - Transmit FIFO Empty Interrupt Enable"]
pub type TfifoEmptyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NXMIP_EN` reader - No Transmit in Progress Interrupt Enable"]
pub type NxmipEnR = crate::BitReader;
#[doc = "Field `NXMIP_EN` writer - No Transmit in Progress Interrupt Enable"]
pub type NxmipEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO Empty Level Select"]
    #[inline(always)]
    pub fn tempty_level_sel(&self) -> TemptyLevelSelR {
        TemptyLevelSelR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty Level Interrupt Enable"]
    #[inline(always)]
    pub fn tempty_level_en(&self) -> TemptyLevelEnR {
        TemptyLevelEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tfifo_empty_en(&self) -> TfifoEmptyEnR {
        TfifoEmptyEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Transmit in Progress Interrupt Enable"]
    #[inline(always)]
    pub fn nxmip_en(&self) -> NxmipEnR {
        NxmipEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UFTCTLn")
            .field("tempty_level_sel", &self.tempty_level_sel())
            .field("tempty_level_en", &self.tempty_level_en())
            .field("tfifo_empty_en", &self.tfifo_empty_en())
            .field("nxmip_en", &self.nxmip_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit FIFO Empty Level Select"]
    #[inline(always)]
    pub fn tempty_level_sel(&mut self) -> TemptyLevelSelW<UftctlnSpec> {
        TemptyLevelSelW::new(self, 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty Level Interrupt Enable"]
    #[inline(always)]
    pub fn tempty_level_en(&mut self) -> TemptyLevelEnW<UftctlnSpec> {
        TemptyLevelEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn tfifo_empty_en(&mut self) -> TfifoEmptyEnW<UftctlnSpec> {
        TfifoEmptyEnW::new(self, 6)
    }
    #[doc = "Bit 7 - No Transmit in Progress Interrupt Enable"]
    #[inline(always)]
    pub fn nxmip_en(&mut self) -> NxmipEnW<UftctlnSpec> {
        NxmipEnW::new(self, 7)
    }
}
#[doc = "FIFO Mode Transmit Control Register (UFTCTLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`uftctln::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uftctln::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UftctlnSpec;
impl crate::RegisterSpec for UftctlnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uftctln::R`](R) reader structure"]
impl crate::Readable for UftctlnSpec {}
#[doc = "`write(|w| ..)` method takes [`uftctln::W`](W) writer structure"]
impl crate::Writable for UftctlnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UFTCTLn to value 0"]
impl crate::Resettable for UftctlnSpec {
    const RESET_VALUE: u8 = 0;
}
