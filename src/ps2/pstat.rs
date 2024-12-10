#[doc = "Register `PSTAT` reader"]
pub type R = crate::R<PstatSpec>;
#[doc = "Register `PSTAT` writer"]
pub type W = crate::W<PstatSpec>;
#[doc = "Field `SOT` reader - Start Of Transaction"]
pub type SotR = crate::BitReader;
#[doc = "Field `SOT` writer - Start Of Transaction"]
pub type SotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOT` reader - End Of Transaction"]
pub type EotR = crate::BitReader;
#[doc = "Field `EOT` writer - End Of Transaction"]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Parity Error"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Parity Error"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACH` reader - Active Channel"]
pub type AchR = crate::FieldReader;
#[doc = "Field `ACH` writer - Active Channel"]
pub type AchW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RFERR` reader - Receive Frame Error"]
pub type RferrR = crate::BitReader;
#[doc = "Field `RFERR` writer - Receive Frame Error"]
pub type RferrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Of Transaction"]
    #[inline(always)]
    pub fn sot(&self) -> SotR {
        SotR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End Of Transaction"]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Active Channel"]
    #[inline(always)]
    pub fn ach(&self) -> AchR {
        AchR::new((self.bits >> 3) & 7)
    }
    #[doc = "Bit 6 - Receive Frame Error"]
    #[inline(always)]
    pub fn rferr(&self) -> RferrR {
        RferrR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSTAT")
            .field("sot", &self.sot())
            .field("eot", &self.eot())
            .field("perr", &self.perr())
            .field("ach", &self.ach())
            .field("rferr", &self.rferr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Start Of Transaction"]
    #[inline(always)]
    pub fn sot(&mut self) -> SotW<PstatSpec> {
        SotW::new(self, 0)
    }
    #[doc = "Bit 1 - End Of Transaction"]
    #[inline(always)]
    pub fn eot(&mut self) -> EotW<PstatSpec> {
        EotW::new(self, 1)
    }
    #[doc = "Bit 2 - Parity Error"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<PstatSpec> {
        PerrW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Active Channel"]
    #[inline(always)]
    pub fn ach(&mut self) -> AchW<PstatSpec> {
        AchW::new(self, 3)
    }
    #[doc = "Bit 6 - Receive Frame Error"]
    #[inline(always)]
    pub fn rferr(&mut self) -> RferrW<PstatSpec> {
        RferrW::new(self, 6)
    }
}
#[doc = "PS/2 Status Register (PSTAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`pstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PstatSpec;
impl crate::RegisterSpec for PstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pstat::R`](R) reader structure"]
impl crate::Readable for PstatSpec {}
#[doc = "`write(|w| ..)` method takes [`pstat::W`](W) writer structure"]
impl crate::Writable for PstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSTAT to value 0"]
impl crate::Resettable for PstatSpec {
    const RESET_VALUE: u8 = 0;
}
