#[doc = "Register `SHICFG2` reader"]
pub type R = crate::R<Shicfg2Spec>;
#[doc = "Register `SHICFG2` writer"]
pub type W = crate::W<Shicfg2Spec>;
#[doc = "Field `SIMUL` reader - Simultaneous Read/Write"]
pub type SimulR = crate::BitReader;
#[doc = "Field `SIMUL` writer - Simultaneous Read/Write"]
pub type SimulW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - SHI Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - SHI Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONESHOT` reader - Write One Shot"]
pub type OneshotR = crate::BitReader;
#[doc = "Field `ONESHOT` writer - Write One Shot"]
pub type OneshotW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Simultaneous Read/Write"]
    #[inline(always)]
    pub fn simul(&self) -> SimulR {
        SimulR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SHI Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write One Shot"]
    #[inline(always)]
    pub fn oneshot(&self) -> OneshotR {
        OneshotR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHICFG2")
            .field("simul", &self.simul())
            .field("busy", &self.busy())
            .field("oneshot", &self.oneshot())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Simultaneous Read/Write"]
    #[inline(always)]
    pub fn simul(&mut self) -> SimulW<Shicfg2Spec> {
        SimulW::new(self, 0)
    }
    #[doc = "Bit 1 - SHI Busy"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<Shicfg2Spec> {
        BusyW::new(self, 1)
    }
    #[doc = "Bit 2 - Write One Shot"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> OneshotW<Shicfg2Spec> {
        OneshotW::new(self, 2)
    }
}
#[doc = "SHI Configuration 2\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shicfg2Spec;
impl crate::RegisterSpec for Shicfg2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`shicfg2::R`](R) reader structure"]
impl crate::Readable for Shicfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`shicfg2::W`](W) writer structure"]
impl crate::Writable for Shicfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHICFG2 to value 0"]
impl crate::Resettable for Shicfg2Spec {
    const RESET_VALUE: u8 = 0;
}
