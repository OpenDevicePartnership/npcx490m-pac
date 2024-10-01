#[doc = "Register `USTATn` reader"]
pub type R = crate::R<UstatnSpec>;
#[doc = "Register `USTATn` writer"]
pub type W = crate::W<UstatnSpec>;
#[doc = "Field `PE` reader - Parity Error"]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - Parity Error"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FE` reader - Framing Error"]
pub type FeR = crate::BitReader;
#[doc = "Field `FE` writer - Framing Error"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOE` reader - Data Overrun Error"]
pub type DoeR = crate::BitReader;
#[doc = "Field `DOE` writer - Data Overrun Error"]
pub type DoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKD` reader - Break Detect"]
pub type BkdR = crate::BitReader;
#[doc = "Field `BKD` writer - Break Detect"]
pub type BkdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity Error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing Error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Overrun Error"]
    #[inline(always)]
    pub fn doe(&self) -> DoeR {
        DoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Detect"]
    #[inline(always)]
    pub fn bkd(&self) -> BkdR {
        BkdR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USTATn")
            .field("pe", &self.pe())
            .field("fe", &self.fe())
            .field("doe", &self.doe())
            .field("bkd", &self.bkd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<UstatnSpec> {
        PeW::new(self, 0)
    }
    #[doc = "Bit 1 - Framing Error"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<UstatnSpec> {
        FeW::new(self, 1)
    }
    #[doc = "Bit 2 - Data Overrun Error"]
    #[inline(always)]
    #[must_use]
    pub fn doe(&mut self) -> DoeW<UstatnSpec> {
        DoeW::new(self, 2)
    }
    #[doc = "Bit 4 - Break Detect"]
    #[inline(always)]
    #[must_use]
    pub fn bkd(&mut self) -> BkdW<UstatnSpec> {
        BkdW::new(self, 4)
    }
}
#[doc = "Status Register (USTATn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ustatn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ustatn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UstatnSpec;
impl crate::RegisterSpec for UstatnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ustatn::R`](R) reader structure"]
impl crate::Readable for UstatnSpec {}
#[doc = "`write(|w| ..)` method takes [`ustatn::W`](W) writer structure"]
impl crate::Writable for UstatnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets USTATn to value 0"]
impl crate::Resettable for UstatnSpec {
    const RESET_VALUE: u8 = 0;
}
