#[doc = "Register `DP80STS` reader"]
pub type R = crate::R<Dp80stsSpec>;
#[doc = "Register `DP80STS` writer"]
pub type W = crate::W<Dp80stsSpec>;
#[doc = "Field `FWR` reader - FIFO Was Written"]
pub type FwrR = crate::BitReader;
#[doc = "Field `FWR` writer - FIFO Was Written"]
pub type FwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FNE` reader - FIFO Not Empty"]
pub type FneR = crate::BitReader;
#[doc = "Field `FNE` writer - FIFO Not Empty"]
pub type FneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOR` reader - FIFO Overrun"]
pub type ForR = crate::BitReader;
#[doc = "Field `FOR` writer - FIFO Overrun"]
pub type ForW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - FIFO Was Written"]
    #[inline(always)]
    pub fn fwr(&self) -> FwrR {
        FwrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO Not Empty"]
    #[inline(always)]
    pub fn fne(&self) -> FneR {
        FneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FIFO Overrun"]
    #[inline(always)]
    pub fn for_(&self) -> ForR {
        ForR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DP80STS")
            .field("fwr", &self.fwr())
            .field("fne", &self.fne())
            .field("for_", &self.for_())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - FIFO Was Written"]
    #[inline(always)]
    #[must_use]
    pub fn fwr(&mut self) -> FwrW<Dp80stsSpec> {
        FwrW::new(self, 5)
    }
    #[doc = "Bit 6 - FIFO Not Empty"]
    #[inline(always)]
    #[must_use]
    pub fn fne(&mut self) -> FneW<Dp80stsSpec> {
        FneW::new(self, 6)
    }
    #[doc = "Bit 7 - FIFO Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn for_(&mut self) -> ForW<Dp80stsSpec> {
        ForW::new(self, 7)
    }
}
#[doc = "Debug Port 80 Status Register (DP80STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`dp80sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dp80sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dp80stsSpec;
impl crate::RegisterSpec for Dp80stsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dp80sts::R`](R) reader structure"]
impl crate::Readable for Dp80stsSpec {}
#[doc = "`write(|w| ..)` method takes [`dp80sts::W`](W) writer structure"]
impl crate::Writable for Dp80stsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DP80STS to value 0"]
impl crate::Resettable for Dp80stsSpec {
    const RESET_VALUE: u8 = 0;
}
