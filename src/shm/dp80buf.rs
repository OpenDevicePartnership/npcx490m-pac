#[doc = "Register `DP80BUF` reader"]
pub type R = crate::R<Dp80bufSpec>;
#[doc = "Register `DP80BUF` writer"]
pub type W = crate::W<Dp80bufSpec>;
#[doc = "Field `DP80BUF` reader - Debug Port 80 Buffered Data"]
pub type Dp80bufR = crate::FieldReader;
#[doc = "Field `DP80BUF` writer - Debug Port 80 Buffered Data"]
pub type Dp80bufW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OFFS` reader - Offset Address"]
pub type OffsR = crate::FieldReader;
#[doc = "Field `OFFS` writer - Offset Address"]
pub type OffsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RNG` reader - Address Range"]
pub type RngR = crate::BitReader;
#[doc = "Field `RNG` writer - Address Range"]
pub type RngW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Debug Port 80 Buffered Data"]
    #[inline(always)]
    pub fn dp80buf(&self) -> Dp80bufR {
        Dp80bufR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Offset Address"]
    #[inline(always)]
    pub fn offs(&self) -> OffsR {
        OffsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Address Range"]
    #[inline(always)]
    pub fn rng(&self) -> RngR {
        RngR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DP80BUF")
            .field("dp80buf", &self.dp80buf())
            .field("offs", &self.offs())
            .field("rng", &self.rng())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Debug Port 80 Buffered Data"]
    #[inline(always)]
    #[must_use]
    pub fn dp80buf(&mut self) -> Dp80bufW<Dp80bufSpec> {
        Dp80bufW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Offset Address"]
    #[inline(always)]
    #[must_use]
    pub fn offs(&mut self) -> OffsW<Dp80bufSpec> {
        OffsW::new(self, 8)
    }
    #[doc = "Bit 11 - Address Range"]
    #[inline(always)]
    #[must_use]
    pub fn rng(&mut self) -> RngW<Dp80bufSpec> {
        RngW::new(self, 11)
    }
}
#[doc = "Debug Port 80 Buffered Data Register (DP80BUF)\n\nYou can [`read`](crate::Reg::read) this register and get [`dp80buf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dp80buf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dp80bufSpec;
impl crate::RegisterSpec for Dp80bufSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dp80buf::R`](R) reader structure"]
impl crate::Readable for Dp80bufSpec {}
#[doc = "`write(|w| ..)` method takes [`dp80buf::W`](W) writer structure"]
impl crate::Writable for Dp80bufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DP80BUF to value 0"]
impl crate::Resettable for Dp80bufSpec {
    const RESET_VALUE: u16 = 0;
}
