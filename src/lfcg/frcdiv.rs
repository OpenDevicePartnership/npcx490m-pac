#[doc = "Register `FRCDIV` reader"]
pub type R = crate::R<FrcdivSpec>;
#[doc = "Register `FRCDIV` writer"]
pub type W = crate::W<FrcdivSpec>;
#[doc = "Field `FRCDIV` reader - FRCLK Clock Divisor"]
pub type FrcdivR = crate::FieldReader<u16>;
#[doc = "Field `FRCDIV` writer - FRCLK Clock Divisor"]
pub type FrcdivW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - FRCLK Clock Divisor"]
    #[inline(always)]
    pub fn frcdiv(&self) -> FrcdivR {
        FrcdivR::new(self.bits & 0x01ff)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRCDIV")
            .field("frcdiv", &self.frcdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - FRCLK Clock Divisor"]
    #[inline(always)]
    pub fn frcdiv(&mut self) -> FrcdivW<FrcdivSpec> {
        FrcdivW::new(self, 0)
    }
}
#[doc = "FRCLK Clock Divisor Register (FRCDIV)\n\nYou can [`read`](crate::Reg::read) this register and get [`frcdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frcdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrcdivSpec;
impl crate::RegisterSpec for FrcdivSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frcdiv::R`](R) reader structure"]
impl crate::Readable for FrcdivSpec {}
#[doc = "`write(|w| ..)` method takes [`frcdiv::W`](W) writer structure"]
impl crate::Writable for FrcdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FRCDIV to value 0xe3"]
impl crate::Resettable for FrcdivSpec {
    const RESET_VALUE: u16 = 0xe3;
}
