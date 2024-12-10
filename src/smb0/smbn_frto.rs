#[doc = "Register `SMBn_FRTO` reader"]
pub type R = crate::R<SmbnFrtoSpec>;
#[doc = "Register `SMBn_FRTO` writer"]
pub type W = crate::W<SmbnFrtoSpec>;
#[doc = "Field `FR_LEN_TO` reader - Frame Length Timeout"]
pub type FrLenToR = crate::FieldReader;
#[doc = "Field `FR_LEN_TO` writer - Frame Length Timeout"]
pub type FrLenToW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FRTOIE` reader - Frame Timeout Interrupt Enable"]
pub type FrtoieR = crate::BitReader;
#[doc = "Field `FRTOIE` writer - Frame Timeout Interrupt Enable"]
pub type FrtoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRTOST` reader - Frame Timeout Status"]
pub type FrtostR = crate::BitReader;
#[doc = "Field `FRTOST` writer - Frame Timeout Status"]
pub type FrtostW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Frame Length Timeout"]
    #[inline(always)]
    pub fn fr_len_to(&self) -> FrLenToR {
        FrLenToR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Frame Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn frtoie(&self) -> FrtoieR {
        FrtoieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame Timeout Status"]
    #[inline(always)]
    pub fn frtost(&self) -> FrtostR {
        FrtostR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBn_FRTO")
            .field("fr_len_to", &self.fr_len_to())
            .field("frtoie", &self.frtoie())
            .field("frtost", &self.frtost())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Frame Length Timeout"]
    #[inline(always)]
    pub fn fr_len_to(&mut self) -> FrLenToW<SmbnFrtoSpec> {
        FrLenToW::new(self, 0)
    }
    #[doc = "Bit 6 - Frame Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn frtoie(&mut self) -> FrtoieW<SmbnFrtoSpec> {
        FrtoieW::new(self, 6)
    }
    #[doc = "Bit 7 - Frame Timeout Status"]
    #[inline(always)]
    pub fn frtost(&mut self) -> FrtostW<SmbnFrtoSpec> {
        FrtostW::new(self, 7)
    }
}
#[doc = "SMB Frame Timeout Register (SMBn_FRTO)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_frto::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_frto::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnFrtoSpec;
impl crate::RegisterSpec for SmbnFrtoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_frto::R`](R) reader structure"]
impl crate::Readable for SmbnFrtoSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_frto::W`](W) writer structure"]
impl crate::Writable for SmbnFrtoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBn_FRTO to value 0"]
impl crate::Resettable for SmbnFrtoSpec {
    const RESET_VALUE: u8 = 0;
}
