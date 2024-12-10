#[doc = "Register `DEVALTF_LK` reader"]
pub type R = crate::R<DevaltfLkSpec>;
#[doc = "Register `DEVALTF_LK` writer"]
pub type W = crate::W<DevaltfLkSpec>;
#[doc = "Field `AD5_SL_LK` reader - AD5 Select Lock"]
pub type Ad5SlLkR = crate::BitReader;
#[doc = "Field `AD5_SL_LK` writer - AD5 Select Lock"]
pub type Ad5SlLkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AD5 Select Lock"]
    #[inline(always)]
    pub fn ad5_sl_lk(&self) -> Ad5SlLkR {
        Ad5SlLkR::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEVALTF_LK")
            .field("ad5_sl_lk", &self.ad5_sl_lk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AD5 Select Lock"]
    #[inline(always)]
    pub fn ad5_sl_lk(&mut self) -> Ad5SlLkW<DevaltfLkSpec> {
        Ad5SlLkW::new(self, 0)
    }
}
#[doc = "Device Alternate Function F Lock Register (DEVALTF_LK)\n\nYou can [`read`](crate::Reg::read) this register and get [`devaltf_lk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devaltf_lk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevaltfLkSpec;
impl crate::RegisterSpec for DevaltfLkSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`devaltf_lk::R`](R) reader structure"]
impl crate::Readable for DevaltfLkSpec {}
#[doc = "`write(|w| ..)` method takes [`devaltf_lk::W`](W) writer structure"]
impl crate::Writable for DevaltfLkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DEVALTF_LK to value 0"]
impl crate::Resettable for DevaltfLkSpec {
    const RESET_VALUE: u8 = 0;
}
