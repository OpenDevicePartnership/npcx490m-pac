#[doc = "Register `USTATn` reader"]
pub type R = crate::R<UstatnSpec>;
#[doc = "Field `PE` reader - Parity Error"]
pub type PeR = crate::BitReader;
#[doc = "Field `FE` reader - Framing Error"]
pub type FeR = crate::BitReader;
#[doc = "Field `DOE` reader - Data Overrun Error"]
pub type DoeR = crate::BitReader;
#[doc = "Field `BKD` reader - Break Detect"]
pub type BkdR = crate::BitReader;
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
#[doc = "Status Register (USTATn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ustatn::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UstatnSpec;
impl crate::RegisterSpec for UstatnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ustatn::R`](R) reader structure"]
impl crate::Readable for UstatnSpec {}
#[doc = "`reset()` method sets USTATn to value 0"]
impl crate::Resettable for UstatnSpec {
    const RESET_VALUE: u8 = 0;
}
