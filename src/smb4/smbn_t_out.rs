#[doc = "Register `SMBnT_OUT` reader"]
pub type R = crate::R<SmbnTOutSpec>;
#[doc = "Register `SMBnT_OUT` writer"]
pub type W = crate::W<SmbnTOutSpec>;
#[doc = "Field `TO_CKDIV` reader - Timeout Clock Divisor"]
pub type ToCkdivR = crate::FieldReader;
#[doc = "Field `TO_CKDIV` writer - Timeout Clock Divisor"]
pub type ToCkdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `T_OUTIE` reader - Timeout Interrupt Enable"]
pub type TOutieR = crate::BitReader;
#[doc = "Field `T_OUTIE` writer - Timeout Interrupt Enable"]
pub type TOutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T_OUTST` reader - Timeout Status"]
pub type TOutstR = crate::BitReader;
#[doc = "Field `T_OUTST` writer - Timeout Status"]
pub type TOutstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Timeout Clock Divisor"]
    #[inline(always)]
    pub fn to_ckdiv(&self) -> ToCkdivR {
        ToCkdivR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn t_outie(&self) -> TOutieR {
        TOutieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timeout Status"]
    #[inline(always)]
    pub fn t_outst(&self) -> TOutstR {
        TOutstR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnT_OUT")
            .field("to_ckdiv", &self.to_ckdiv())
            .field("t_outie", &self.t_outie())
            .field("t_outst", &self.t_outst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Timeout Clock Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn to_ckdiv(&mut self) -> ToCkdivW<SmbnTOutSpec> {
        ToCkdivW::new(self, 0)
    }
    #[doc = "Bit 6 - Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn t_outie(&mut self) -> TOutieW<SmbnTOutSpec> {
        TOutieW::new(self, 6)
    }
    #[doc = "Bit 7 - Timeout Status"]
    #[inline(always)]
    #[must_use]
    pub fn t_outst(&mut self) -> TOutstW<SmbnTOutSpec> {
        TOutstW::new(self, 7)
    }
}
#[doc = "SMB Bus Timeout Register (SMBnT_OUT)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_t_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_t_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnTOutSpec;
impl crate::RegisterSpec for SmbnTOutSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_t_out::R`](R) reader structure"]
impl crate::Readable for SmbnTOutSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_t_out::W`](W) writer structure"]
impl crate::Writable for SmbnTOutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnT_OUT to value 0"]
impl crate::Resettable for SmbnTOutSpec {
    const RESET_VALUE: u8 = 0;
}
