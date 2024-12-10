#[doc = "Register `UMDSLn` reader"]
pub type R = crate::R<UmdslnSpec>;
#[doc = "Register `UMDSLn` writer"]
pub type W = crate::W<UmdslnSpec>;
#[doc = "Field `BRK` reader - Break Transmit"]
pub type BrkR = crate::BitReader;
#[doc = "Field `BRK` writer - Break Transmit"]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETD` reader - Enable Transmit DMA"]
pub type EtdR = crate::BitReader;
#[doc = "Field `ETD` writer - Enable Transmit DMA"]
pub type EtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERD` reader - Enable Receive DMA"]
pub type ErdR = crate::BitReader;
#[doc = "Field `ERD` writer - Enable Receive DMA"]
pub type ErdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Break Transmit"]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Transmit DMA"]
    #[inline(always)]
    pub fn etd(&self) -> EtdR {
        EtdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Receive DMA"]
    #[inline(always)]
    pub fn erd(&self) -> ErdR {
        ErdR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UMDSLn")
            .field("brk", &self.brk())
            .field("etd", &self.etd())
            .field("erd", &self.erd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Break Transmit"]
    #[inline(always)]
    pub fn brk(&mut self) -> BrkW<UmdslnSpec> {
        BrkW::new(self, 2)
    }
    #[doc = "Bit 4 - Enable Transmit DMA"]
    #[inline(always)]
    pub fn etd(&mut self) -> EtdW<UmdslnSpec> {
        EtdW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Receive DMA"]
    #[inline(always)]
    pub fn erd(&mut self) -> ErdW<UmdslnSpec> {
        ErdW::new(self, 5)
    }
}
#[doc = "Mode Select Register (UMDSLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`umdsln::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`umdsln::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UmdslnSpec;
impl crate::RegisterSpec for UmdslnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`umdsln::R`](R) reader structure"]
impl crate::Readable for UmdslnSpec {}
#[doc = "`write(|w| ..)` method takes [`umdsln::W`](W) writer structure"]
impl crate::Writable for UmdslnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UMDSLn to value 0"]
impl crate::Resettable for UmdslnSpec {
    const RESET_VALUE: u8 = 0;
}
