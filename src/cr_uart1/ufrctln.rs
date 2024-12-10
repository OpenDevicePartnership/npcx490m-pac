#[doc = "Register `UFRCTLn` reader"]
pub type R = crate::R<UfrctlnSpec>;
#[doc = "Register `UFRCTLn` writer"]
pub type W = crate::W<UfrctlnSpec>;
#[doc = "Field `RFULL_LEVEL_SEL` reader - Receive FIFO Full Level Select"]
pub type RfullLevelSelR = crate::FieldReader;
#[doc = "Field `RFULL_LEVEL_SEL` writer - Receive FIFO Full Level Select"]
pub type RfullLevelSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RFULL_LEVEL_EN` reader - Receive FIFO Full Level Interrupt Enable"]
pub type RfullLevelEnR = crate::BitReader;
#[doc = "Field `RFULL_LEVEL_EN` writer - Receive FIFO Full Level Interrupt Enable"]
pub type RfullLevelEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFO_NEMPTY_EN` reader - Receive FIFO Not Empty Interrupt Enable"]
pub type RfifoNemptyEnR = crate::BitReader;
#[doc = "Field `RFIFO_NEMPTY_EN` writer - Receive FIFO Not Empty Interrupt Enable"]
pub type RfifoNemptyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_EN` reader - Receive Error Interrupt Enable"]
pub type ErrEnR = crate::BitReader;
#[doc = "Field `ERR_EN` writer - Receive Error Interrupt Enable"]
pub type ErrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Full Level Select"]
    #[inline(always)]
    pub fn rfull_level_sel(&self) -> RfullLevelSelR {
        RfullLevelSelR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Receive FIFO Full Level Interrupt Enable"]
    #[inline(always)]
    pub fn rfull_level_en(&self) -> RfullLevelEnR {
        RfullLevelEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rfifo_nempty_en(&self) -> RfifoNemptyEnR {
        RfifoNemptyEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn err_en(&self) -> ErrEnR {
        ErrEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UFRCTLn")
            .field("rfull_level_sel", &self.rfull_level_sel())
            .field("rfull_level_en", &self.rfull_level_en())
            .field("rfifo_nempty_en", &self.rfifo_nempty_en())
            .field("err_en", &self.err_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Receive FIFO Full Level Select"]
    #[inline(always)]
    pub fn rfull_level_sel(&mut self) -> RfullLevelSelW<UfrctlnSpec> {
        RfullLevelSelW::new(self, 0)
    }
    #[doc = "Bit 5 - Receive FIFO Full Level Interrupt Enable"]
    #[inline(always)]
    pub fn rfull_level_en(&mut self) -> RfullLevelEnW<UfrctlnSpec> {
        RfullLevelEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive FIFO Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn rfifo_nempty_en(&mut self) -> RfifoNemptyEnW<UfrctlnSpec> {
        RfifoNemptyEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Error Interrupt Enable"]
    #[inline(always)]
    pub fn err_en(&mut self) -> ErrEnW<UfrctlnSpec> {
        ErrEnW::new(self, 7)
    }
}
#[doc = "FIFO Mode Receive Control Register (UFRCTLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ufrctln::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ufrctln::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UfrctlnSpec;
impl crate::RegisterSpec for UfrctlnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ufrctln::R`](R) reader structure"]
impl crate::Readable for UfrctlnSpec {}
#[doc = "`write(|w| ..)` method takes [`ufrctln::W`](W) writer structure"]
impl crate::Writable for UfrctlnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UFRCTLn to value 0"]
impl crate::Resettable for UfrctlnSpec {
    const RESET_VALUE: u8 = 0;
}
