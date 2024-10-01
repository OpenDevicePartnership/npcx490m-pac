#[doc = "Register `ASCADD` reader"]
pub type R = crate::R<AscaddSpec>;
#[doc = "Register `ASCADD` writer"]
pub type W = crate::W<AscaddSpec>;
#[doc = "Field `SADDR` reader - Software-Triggered Address"]
pub type SaddrR = crate::FieldReader;
#[doc = "Field `SADDR` writer - Software-Triggered Address"]
pub type SaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TTADDR` reader - Timer-Triggered Address"]
pub type TtaddrR = crate::FieldReader;
#[doc = "Field `TTADDR` writer - Timer-Triggered Address"]
pub type TtaddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Software-Triggered Address"]
    #[inline(always)]
    pub fn saddr(&self) -> SaddrR {
        SaddrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Timer-Triggered Address"]
    #[inline(always)]
    pub fn ttaddr(&self) -> TtaddrR {
        TtaddrR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ASCADD")
            .field("saddr", &self.saddr())
            .field("ttaddr", &self.ttaddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Software-Triggered Address"]
    #[inline(always)]
    #[must_use]
    pub fn saddr(&mut self) -> SaddrW<AscaddSpec> {
        SaddrW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Timer-Triggered Address"]
    #[inline(always)]
    #[must_use]
    pub fn ttaddr(&mut self) -> TtaddrW<AscaddSpec> {
        TtaddrW::new(self, 8)
    }
}
#[doc = "ADC Single Channel Address Register (ASCADD)\n\nYou can [`read`](crate::Reg::read) this register and get [`ascadd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ascadd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AscaddSpec;
impl crate::RegisterSpec for AscaddSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ascadd::R`](R) reader structure"]
impl crate::Readable for AscaddSpec {}
#[doc = "`write(|w| ..)` method takes [`ascadd::W`](W) writer structure"]
impl crate::Writable for AscaddSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ASCADD to value 0"]
impl crate::Resettable for AscaddSpec {
    const RESET_VALUE: u16 = 0;
}
