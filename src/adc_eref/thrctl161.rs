#[doc = "Register `THRCTL1-61` reader"]
pub type R = crate::R<Thrctl161Spec>;
#[doc = "Register `THRCTL1-61` writer"]
pub type W = crate::W<Thrctl161Spec>;
#[doc = "Field `THRVAL` reader - Threshold Value"]
pub type ThrvalR = crate::FieldReader<u16>;
#[doc = "Field `THRVAL` writer - Threshold Value"]
pub type ThrvalW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CHNSEL` reader - Channel Select"]
pub type ChnselR = crate::FieldReader;
#[doc = "Field `CHNSEL` writer - Channel Select"]
pub type ChnselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `L_H` reader - Lower or Higher"]
pub type LHR = crate::BitReader;
#[doc = "Field `L_H` writer - Lower or Higher"]
pub type LHW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Threshold Value"]
    #[inline(always)]
    pub fn thrval(&self) -> ThrvalR {
        ThrvalR::new(self.bits & 0x03ff)
    }
    #[doc = "Bits 10:14 - Channel Select"]
    #[inline(always)]
    pub fn chnsel(&self) -> ChnselR {
        ChnselR::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Lower or Higher"]
    #[inline(always)]
    pub fn l_h(&self) -> LHR {
        LHR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRCTL1-61")
            .field("thrval", &self.thrval())
            .field("chnsel", &self.chnsel())
            .field("l_h", &self.l_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn thrval(&mut self) -> ThrvalW<Thrctl161Spec> {
        ThrvalW::new(self, 0)
    }
    #[doc = "Bits 10:14 - Channel Select"]
    #[inline(always)]
    #[must_use]
    pub fn chnsel(&mut self) -> ChnselW<Thrctl161Spec> {
        ChnselW::new(self, 10)
    }
    #[doc = "Bit 15 - Lower or Higher"]
    #[inline(always)]
    #[must_use]
    pub fn l_h(&mut self) -> LHW<Thrctl161Spec> {
        LHW::new(self, 15)
    }
}
#[doc = "Threshold Control 1-6 Register (THRCTL1-6)\n\nYou can [`read`](crate::Reg::read) this register and get [`thrctl161::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thrctl161::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Thrctl161Spec;
impl crate::RegisterSpec for Thrctl161Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`thrctl161::R`](R) reader structure"]
impl crate::Readable for Thrctl161Spec {}
#[doc = "`write(|w| ..)` method takes [`thrctl161::W`](W) writer structure"]
impl crate::Writable for Thrctl161Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets THRCTL1-61 to value 0"]
impl crate::Resettable for Thrctl161Spec {
    const RESET_VALUE: u16 = 0;
}
