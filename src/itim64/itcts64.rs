#[doc = "Register `ITCTS64` reader"]
pub type R = crate::R<Itcts64Spec>;
#[doc = "Register `ITCTS64` writer"]
pub type W = crate::W<Itcts64Spec>;
#[doc = "Field `TO_STS` reader - Timeout Status"]
pub type ToStsR = crate::BitReader;
#[doc = "Field `TO_STS` writer - Timeout Status"]
pub type ToStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO_IE` reader - Timeout Interrupt Enable"]
pub type ToIeR = crate::BitReader;
#[doc = "Field `TO_IE` writer - Timeout Interrupt Enable"]
pub type ToIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO_WUE` reader - Timeout Wake-up Enable"]
pub type ToWueR = crate::BitReader;
#[doc = "Field `TO_WUE` writer - Timeout Wake-up Enable"]
pub type ToWueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSEL` reader - Input Clock Select"]
pub type CkselR = crate::BitReader;
#[doc = "Field `CKSEL` writer - Input Clock Select"]
pub type CkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEN` reader - ITIM64 Module Enable"]
pub type ItenR = crate::BitReader;
#[doc = "Field `ITEN` writer - ITIM64 Module Enable"]
pub type ItenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timeout Status"]
    #[inline(always)]
    pub fn to_sts(&self) -> ToStsR {
        ToStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn to_ie(&self) -> ToIeR {
        ToIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout Wake-up Enable"]
    #[inline(always)]
    pub fn to_wue(&self) -> ToWueR {
        ToWueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input Clock Select"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ITIM64 Module Enable"]
    #[inline(always)]
    pub fn iten(&self) -> ItenR {
        ItenR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITCTS64")
            .field("to_sts", &self.to_sts())
            .field("to_ie", &self.to_ie())
            .field("to_wue", &self.to_wue())
            .field("cksel", &self.cksel())
            .field("iten", &self.iten())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timeout Status"]
    #[inline(always)]
    #[must_use]
    pub fn to_sts(&mut self) -> ToStsW<Itcts64Spec> {
        ToStsW::new(self, 0)
    }
    #[doc = "Bit 2 - Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn to_ie(&mut self) -> ToIeW<Itcts64Spec> {
        ToIeW::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Wake-up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn to_wue(&mut self) -> ToWueW<Itcts64Spec> {
        ToWueW::new(self, 3)
    }
    #[doc = "Bit 4 - Input Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn cksel(&mut self) -> CkselW<Itcts64Spec> {
        CkselW::new(self, 4)
    }
    #[doc = "Bit 7 - ITIM64 Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iten(&mut self) -> ItenW<Itcts64Spec> {
        ItenW::new(self, 7)
    }
}
#[doc = "Internal Timer Control and Status Register (ITCTS64)\n\nYou can [`read`](crate::Reg::read) this register and get [`itcts64::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcts64::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itcts64Spec;
impl crate::RegisterSpec for Itcts64Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itcts64::R`](R) reader structure"]
impl crate::Readable for Itcts64Spec {}
#[doc = "`write(|w| ..)` method takes [`itcts64::W`](W) writer structure"]
impl crate::Writable for Itcts64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ITCTS64 to value 0"]
impl crate::Resettable for Itcts64Spec {
    const RESET_VALUE: u8 = 0;
}
