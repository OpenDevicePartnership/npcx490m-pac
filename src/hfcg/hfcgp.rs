#[doc = "Register `HFCGP` reader"]
pub type R = crate::R<HfcgpSpec>;
#[doc = "Register `HFCGP` writer"]
pub type W = crate::W<HfcgpSpec>;
#[doc = "Field `AHB6DIV` reader - AHB6 Clock Divider"]
pub type Ahb6divR = crate::FieldReader;
#[doc = "Field `AHB6DIV` writer - AHB6 Clock Divider"]
pub type Ahb6divW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FPRED` reader - Core Clock Prescaler Divider Value"]
pub type FpredR = crate::FieldReader;
#[doc = "Field `FPRED` writer - Core Clock Prescaler Divider Value"]
pub type FpredW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - AHB6 Clock Divider"]
    #[inline(always)]
    pub fn ahb6div(&self) -> Ahb6divR {
        Ahb6divR::new(self.bits & 3)
    }
    #[doc = "Bits 4:7 - Core Clock Prescaler Divider Value"]
    #[inline(always)]
    pub fn fpred(&self) -> FpredR {
        FpredR::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCGP")
            .field("ahb6div", &self.ahb6div())
            .field("fpred", &self.fpred())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - AHB6 Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn ahb6div(&mut self) -> Ahb6divW<HfcgpSpec> {
        Ahb6divW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Core Clock Prescaler Divider Value"]
    #[inline(always)]
    #[must_use]
    pub fn fpred(&mut self) -> FpredW<HfcgpSpec> {
        FpredW::new(self, 4)
    }
}
#[doc = "HFCG Prescaler Register (HFCGP)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcgp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcgp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfcgpSpec;
impl crate::RegisterSpec for HfcgpSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcgp::R`](R) reader structure"]
impl crate::Readable for HfcgpSpec {}
#[doc = "`write(|w| ..)` method takes [`hfcgp::W`](W) writer structure"]
impl crate::Writable for HfcgpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCGP to value 0"]
impl crate::Resettable for HfcgpSpec {
    const RESET_VALUE: u8 = 0;
}
