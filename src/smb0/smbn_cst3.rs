#[doc = "Register `SMBnCST3` reader"]
pub type R = crate::R<SmbnCst3Spec>;
#[doc = "Register `SMBnCST3` writer"]
pub type W = crate::W<SmbnCst3Spec>;
#[doc = "Field `MATCHA8F` reader - Match Address 8 Field"]
pub type Matcha8fR = crate::BitReader;
#[doc = "Field `MATCHA8F` writer - Match Address 8 Field"]
pub type Matcha8fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EO_BUSY` reader - End of Busy"]
pub type EoBusyR = crate::BitReader;
#[doc = "Field `EO_BUSY` writer - End of Busy"]
pub type EoBusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Match Address 8 Field"]
    #[inline(always)]
    pub fn matcha8f(&self) -> Matcha8fR {
        Matcha8fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - End of Busy"]
    #[inline(always)]
    pub fn eo_busy(&self) -> EoBusyR {
        EoBusyR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnCST3")
            .field("matcha8f", &self.matcha8f())
            .field("eo_busy", &self.eo_busy())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Match Address 8 Field"]
    #[inline(always)]
    pub fn matcha8f(&mut self) -> Matcha8fW<SmbnCst3Spec> {
        Matcha8fW::new(self, 0)
    }
    #[doc = "Bit 7 - End of Busy"]
    #[inline(always)]
    pub fn eo_busy(&mut self) -> EoBusyW<SmbnCst3Spec> {
        EoBusyW::new(self, 7)
    }
}
#[doc = "SMB Control Status 3 Register (SMBnCST3)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_cst3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_cst3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnCst3Spec;
impl crate::RegisterSpec for SmbnCst3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_cst3::R`](R) reader structure"]
impl crate::Readable for SmbnCst3Spec {}
#[doc = "`write(|w| ..)` method takes [`smbn_cst3::W`](W) writer structure"]
impl crate::Writable for SmbnCst3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnCST3 to value 0"]
impl crate::Resettable for SmbnCst3Spec {
    const RESET_VALUE: u8 = 0;
}
