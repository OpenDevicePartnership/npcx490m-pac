#[doc = "Register `UCNTLn` reader"]
pub type R = crate::R<UcntlnSpec>;
#[doc = "Register `UCNTLn` writer"]
pub type W = crate::W<UcntlnSpec>;
#[doc = "Field `CR_SIN_INV` reader - CR_SIN Signal Invert"]
pub type CrSinInvR = crate::BitReader;
#[doc = "Field `CR_SIN_INV` writer - CR_SIN Signal Invert"]
pub type CrSinInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SOUT_INV` reader - CR_SOUT Signal Invert"]
pub type CrSoutInvR = crate::BitReader;
#[doc = "Field `CR_SOUT_INV` writer - CR_SOUT Signal Invert"]
pub type CrSoutInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SIN_COM` reader - CR_SIN Common Mode Select"]
pub type CrSinComR = crate::BitReader;
#[doc = "Field `CR_SIN_COM` writer - CR_SIN Common Mode Select"]
pub type CrSinComW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SIN_PP` reader - CR_SIN Common Push-Pull Select"]
pub type CrSinPpR = crate::BitReader;
#[doc = "Field `CR_SIN_PP` writer - CR_SIN Common Push-Pull Select"]
pub type CrSinPpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SOUT_COM` reader - CR_SOUT Common Mode Select"]
pub type CrSoutComR = crate::BitReader;
#[doc = "Field `CR_SOUT_COM` writer - CR_SOUT Common Mode Select"]
pub type CrSoutComW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR_SOUT_PP` reader - CR_SOUT Common Push-Pull Select"]
pub type CrSoutPpR = crate::BitReader;
#[doc = "Field `CR_SOUT_PP` writer - CR_SOUT Common Push-Pull Select"]
pub type CrSoutPpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COM_FDBK_EN` reader - Common Mode Feedback Enable"]
pub type ComFdbkEnR = crate::BitReader;
#[doc = "Field `COM_FDBK_EN` writer - Common Mode Feedback Enable"]
pub type ComFdbkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CR_SIN Signal Invert"]
    #[inline(always)]
    pub fn cr_sin_inv(&self) -> CrSinInvR {
        CrSinInvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CR_SOUT Signal Invert"]
    #[inline(always)]
    pub fn cr_sout_inv(&self) -> CrSoutInvR {
        CrSoutInvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CR_SIN Common Mode Select"]
    #[inline(always)]
    pub fn cr_sin_com(&self) -> CrSinComR {
        CrSinComR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CR_SIN Common Push-Pull Select"]
    #[inline(always)]
    pub fn cr_sin_pp(&self) -> CrSinPpR {
        CrSinPpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CR_SOUT Common Mode Select"]
    #[inline(always)]
    pub fn cr_sout_com(&self) -> CrSoutComR {
        CrSoutComR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CR_SOUT Common Push-Pull Select"]
    #[inline(always)]
    pub fn cr_sout_pp(&self) -> CrSoutPpR {
        CrSoutPpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Common Mode Feedback Enable"]
    #[inline(always)]
    pub fn com_fdbk_en(&self) -> ComFdbkEnR {
        ComFdbkEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UCNTLn")
            .field("cr_sin_inv", &self.cr_sin_inv())
            .field("cr_sout_inv", &self.cr_sout_inv())
            .field("cr_sin_com", &self.cr_sin_com())
            .field("cr_sin_pp", &self.cr_sin_pp())
            .field("cr_sout_com", &self.cr_sout_com())
            .field("cr_sout_pp", &self.cr_sout_pp())
            .field("com_fdbk_en", &self.com_fdbk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CR_SIN Signal Invert"]
    #[inline(always)]
    pub fn cr_sin_inv(&mut self) -> CrSinInvW<UcntlnSpec> {
        CrSinInvW::new(self, 0)
    }
    #[doc = "Bit 1 - CR_SOUT Signal Invert"]
    #[inline(always)]
    pub fn cr_sout_inv(&mut self) -> CrSoutInvW<UcntlnSpec> {
        CrSoutInvW::new(self, 1)
    }
    #[doc = "Bit 2 - CR_SIN Common Mode Select"]
    #[inline(always)]
    pub fn cr_sin_com(&mut self) -> CrSinComW<UcntlnSpec> {
        CrSinComW::new(self, 2)
    }
    #[doc = "Bit 3 - CR_SIN Common Push-Pull Select"]
    #[inline(always)]
    pub fn cr_sin_pp(&mut self) -> CrSinPpW<UcntlnSpec> {
        CrSinPpW::new(self, 3)
    }
    #[doc = "Bit 4 - CR_SOUT Common Mode Select"]
    #[inline(always)]
    pub fn cr_sout_com(&mut self) -> CrSoutComW<UcntlnSpec> {
        CrSoutComW::new(self, 4)
    }
    #[doc = "Bit 5 - CR_SOUT Common Push-Pull Select"]
    #[inline(always)]
    pub fn cr_sout_pp(&mut self) -> CrSoutPpW<UcntlnSpec> {
        CrSoutPpW::new(self, 5)
    }
    #[doc = "Bit 6 - Common Mode Feedback Enable"]
    #[inline(always)]
    pub fn com_fdbk_en(&mut self) -> ComFdbkEnW<UcntlnSpec> {
        ComFdbkEnW::new(self, 6)
    }
}
#[doc = "Control Register (UCNTLn)\n\nYou can [`read`](crate::Reg::read) this register and get [`ucntln::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucntln::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UcntlnSpec;
impl crate::RegisterSpec for UcntlnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ucntln::R`](R) reader structure"]
impl crate::Readable for UcntlnSpec {}
#[doc = "`write(|w| ..)` method takes [`ucntln::W`](W) writer structure"]
impl crate::Writable for UcntlnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UCNTLn to value 0"]
impl crate::Resettable for UcntlnSpec {
    const RESET_VALUE: u8 = 0;
}
