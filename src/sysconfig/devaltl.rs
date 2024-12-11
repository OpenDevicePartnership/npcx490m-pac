#[doc = "Register `DEVALTL` reader"]
pub type R = crate::R<DevaltlSpec>;
#[doc = "Register `DEVALTL` writer"]
pub type W = crate::W<DevaltlSpec>;
#[doc = "Field `AD13_SL` reader - AD13 Select"]
pub type Ad13SlR = crate::BitReader;
#[doc = "Field `AD13_SL` writer - AD13 Select"]
pub type Ad13SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD14_SL` reader - AD14 Select"]
pub type Ad14SlR = crate::BitReader;
#[doc = "Field `AD14_SL` writer - AD14 Select"]
pub type Ad14SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD15_SL` reader - AD15 Select"]
pub type Ad15SlR = crate::BitReader;
#[doc = "Field `AD15_SL` writer - AD15 Select"]
pub type Ad15SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD16_SL` reader - AD16 Select"]
pub type Ad16SlR = crate::BitReader;
#[doc = "Field `AD16_SL` writer - AD16 Select"]
pub type Ad16SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD17_SL` reader - AD17 Select"]
pub type Ad17SlR = crate::BitReader;
#[doc = "Field `AD17_SL` writer - AD17 Select"]
pub type Ad17SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD18_SL` reader - AD18 Select"]
pub type Ad18SlR = crate::BitReader;
#[doc = "Field `AD18_SL` writer - AD18 Select"]
pub type Ad18SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD19_SL` reader - AD19 Select"]
pub type Ad19SlR = crate::BitReader;
#[doc = "Field `AD19_SL` writer - AD19 Select"]
pub type Ad19SlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD20_SL` reader - AD20 Select"]
pub type Ad20SlR = crate::BitReader;
#[doc = "Field `AD20_SL` writer - AD20 Select"]
pub type Ad20SlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AD13 Select"]
    #[inline(always)]
    pub fn ad13_sl(&self) -> Ad13SlR {
        Ad13SlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD14 Select"]
    #[inline(always)]
    pub fn ad14_sl(&self) -> Ad14SlR {
        Ad14SlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD15 Select"]
    #[inline(always)]
    pub fn ad15_sl(&self) -> Ad15SlR {
        Ad15SlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD16 Select"]
    #[inline(always)]
    pub fn ad16_sl(&self) -> Ad16SlR {
        Ad16SlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD17 Select"]
    #[inline(always)]
    pub fn ad17_sl(&self) -> Ad17SlR {
        Ad17SlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AD18 Select"]
    #[inline(always)]
    pub fn ad18_sl(&self) -> Ad18SlR {
        Ad18SlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AD19 Select"]
    #[inline(always)]
    pub fn ad19_sl(&self) -> Ad19SlR {
        Ad19SlR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AD20 Select"]
    #[inline(always)]
    pub fn ad20_sl(&self) -> Ad20SlR {
        Ad20SlR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTL")
            .field("ad13_sl", &self.ad13_sl())
            .field("ad14_sl", &self.ad14_sl())
            .field("ad15_sl", &self.ad15_sl())
            .field("ad16_sl", &self.ad16_sl())
            .field("ad17_sl", &self.ad17_sl())
            .field("ad18_sl", &self.ad18_sl())
            .field("ad19_sl", &self.ad19_sl())
            .field("ad20_sl", &self.ad20_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AD13 Select"]
    #[inline(always)]
    pub fn ad13_sl(&mut self) -> Ad13SlW<DevaltlSpec> {
        Ad13SlW::new(self, 0)
    }
    #[doc = "Bit 1 - AD14 Select"]
    #[inline(always)]
    pub fn ad14_sl(&mut self) -> Ad14SlW<DevaltlSpec> {
        Ad14SlW::new(self, 1)
    }
    #[doc = "Bit 2 - AD15 Select"]
    #[inline(always)]
    pub fn ad15_sl(&mut self) -> Ad15SlW<DevaltlSpec> {
        Ad15SlW::new(self, 2)
    }
    #[doc = "Bit 3 - AD16 Select"]
    #[inline(always)]
    pub fn ad16_sl(&mut self) -> Ad16SlW<DevaltlSpec> {
        Ad16SlW::new(self, 3)
    }
    #[doc = "Bit 4 - AD17 Select"]
    #[inline(always)]
    pub fn ad17_sl(&mut self) -> Ad17SlW<DevaltlSpec> {
        Ad17SlW::new(self, 4)
    }
    #[doc = "Bit 5 - AD18 Select"]
    #[inline(always)]
    pub fn ad18_sl(&mut self) -> Ad18SlW<DevaltlSpec> {
        Ad18SlW::new(self, 5)
    }
    #[doc = "Bit 6 - AD19 Select"]
    #[inline(always)]
    pub fn ad19_sl(&mut self) -> Ad19SlW<DevaltlSpec> {
        Ad19SlW::new(self, 6)
    }
    #[doc = "Bit 7 - AD20 Select"]
    #[inline(always)]
    pub fn ad20_sl(&mut self) -> Ad20SlW<DevaltlSpec> {
        Ad20SlW::new(self, 7)
    }
}
#[doc = "Device Alternate Function L Register (DEVALTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltlSpec;
impl crate::RegisterSpec for DevaltlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltl::R`](R) reader structure"]
impl crate::Readable for DevaltlSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltl::W`](W) writer structure"]
impl crate::Writable for DevaltlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTL to value 0"]
impl crate::Resettable for DevaltlSpec {
    const RESET_VALUE: u8 = 0;
}
