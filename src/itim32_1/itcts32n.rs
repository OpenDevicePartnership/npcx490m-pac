#[doc = "Register `ITCTS32n` reader"]
pub type R = crate::R<Itcts32nSpec>;
#[doc = "Register `ITCTS32n` writer"]
pub type W = crate::W<Itcts32nSpec>;
#[doc = "Field `TO_STS` reader - Timeout Status"]
pub type ToStsR = crate::BitReader;
#[doc = "Field `TO_STS` writer - Timeout Status"]
pub type ToStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO_IE` reader - Timeout Interrupt Enable"]
pub type ToIeR = crate::BitReader;
#[doc = "Field `TO_IE` writer - Timeout Interrupt Enable"]
pub type ToIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TO_WUE` reader - Timeout Wake-Up Enable"]
pub type ToWueR = crate::BitReader;
#[doc = "Field `TO_WUE` writer - Timeout Wake-Up Enable"]
pub type ToWueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSEL` reader - Input Clock Select"]
pub type CkselR = crate::BitReader;
#[doc = "Field `CKSEL` writer - Input Clock Select"]
pub type CkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEN` reader - ITIM32 Module Enable"]
pub type ItenR = crate::BitReader;
#[doc = "Field `ITEN` writer - ITIM32 Module Enable"]
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
    #[doc = "Bit 3 - Timeout Wake-Up Enable"]
    #[inline(always)]
    pub fn to_wue(&self) -> ToWueR {
        ToWueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input Clock Select"]
    #[inline(always)]
    pub fn cksel(&self) -> CkselR {
        CkselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ITIM32 Module Enable"]
    #[inline(always)]
    pub fn iten(&self) -> ItenR {
        ItenR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITCTS32n")
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
    pub fn to_sts(&mut self) -> ToStsW<Itcts32nSpec> {
        ToStsW::new(self, 0)
    }
    #[doc = "Bit 2 - Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn to_ie(&mut self) -> ToIeW<Itcts32nSpec> {
        ToIeW::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout Wake-Up Enable"]
    #[inline(always)]
    pub fn to_wue(&mut self) -> ToWueW<Itcts32nSpec> {
        ToWueW::new(self, 3)
    }
    #[doc = "Bit 4 - Input Clock Select"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CkselW<Itcts32nSpec> {
        CkselW::new(self, 4)
    }
    #[doc = "Bit 7 - ITIM32 Module Enable"]
    #[inline(always)]
    pub fn iten(&mut self) -> ItenW<Itcts32nSpec> {
        ItenW::new(self, 7)
    }
}
#[doc = "Internal Timer Control and Status Register (ITCTS32n)\n\nYou can [`read`](crate::Reg::read) this register and get [`itcts32n::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcts32n::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itcts32nSpec;
impl crate::RegisterSpec for Itcts32nSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`itcts32n::R`](R) reader structure"]
impl crate::Readable for Itcts32nSpec {}
#[doc = "`write(|w| ..)` method takes [`itcts32n::W`](W) writer structure"]
impl crate::Writable for Itcts32nSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ITCTS32n to value 0"]
impl crate::Resettable for Itcts32nSpec {
    const RESET_VALUE: u8 = 0;
}
