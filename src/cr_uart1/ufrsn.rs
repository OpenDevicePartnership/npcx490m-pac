#[doc = "Register `UFRSn` reader"]
pub type R = crate::R<UfrsnSpec>;
#[doc = "Register `UFRSn` writer"]
pub type W = crate::W<UfrsnSpec>;
#[doc = "Field `STP` reader - Stop Bits"]
pub type StpR = crate::BitReader;
#[doc = "Field `STP` writer - Stop Bits"]
pub type StpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSEL` reader - Parity Select"]
pub type PselR = crate::FieldReader;
#[doc = "Field `PSEL` writer - Parity Select"]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PEN` reader - Parity Enable"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Parity Enable"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Stop Bits"]
    #[inline(always)]
    pub fn stp(&self) -> StpR {
        StpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UFRSn")
            .field("stp", &self.stp())
            .field("psel", &self.psel())
            .field("pen", &self.pen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Stop Bits"]
    #[inline(always)]
    pub fn stp(&mut self) -> StpW<UfrsnSpec> {
        StpW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    pub fn psel(&mut self) -> PselW<UfrsnSpec> {
        PselW::new(self, 4)
    }
    #[doc = "Bit 6 - Parity Enable"]
    #[inline(always)]
    pub fn pen(&mut self) -> PenW<UfrsnSpec> {
        PenW::new(self, 6)
    }
}
#[doc = "Frame Select Register (UFRSn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ufrsn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ufrsn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UfrsnSpec;
impl crate::RegisterSpec for UfrsnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ufrsn::R`](R) reader structure"]
impl crate::Readable for UfrsnSpec {}
#[doc = "`write(|w| ..)` method takes [`ufrsn::W`](W) writer structure"]
impl crate::Writable for UfrsnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UFRSn to value 0"]
impl crate::Resettable for UfrsnSpec {
    const RESET_VALUE: u8 = 0;
}
