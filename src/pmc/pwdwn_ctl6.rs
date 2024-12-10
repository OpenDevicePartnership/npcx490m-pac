#[doc = "Register `PWDWN_CTL6` reader"]
pub type R = crate::R<PwdwnCtl6Spec>;
#[doc = "Register `PWDWN_CTL6` writer"]
pub type W = crate::W<PwdwnCtl6Spec>;
#[doc = "Field `ITIM4_PD` reader - ITIM32-4 Power-Down"]
pub type Itim4PdR = crate::BitReader;
#[doc = "Field `ITIM4_PD` writer - ITIM32-4 Power-Down"]
pub type Itim4PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM5_PD` reader - ITIM32-5 Power-Down"]
pub type Itim5PdR = crate::BitReader;
#[doc = "Field `ITIM5_PD` writer - ITIM32-5 Power-Down"]
pub type Itim5PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM6_PD` reader - ITIM32-6 Power-Down"]
pub type Itim6PdR = crate::BitReader;
#[doc = "Field `ITIM6_PD` writer - ITIM32-6 Power-Down"]
pub type Itim6PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AES_PD` reader - AES Power-Down"]
pub type AesPdR = crate::BitReader;
#[doc = "Field `AES_PD` writer - AES Power-Down"]
pub type AesPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHA_PD` reader - SHA Power-Down"]
pub type ShaPdR = crate::BitReader;
#[doc = "Field `SHA_PD` writer - SHA Power-Down"]
pub type ShaPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PKA_PD` reader - PKA Power-Down"]
pub type PkaPdR = crate::BitReader;
#[doc = "Field `PKA_PD` writer - PKA Power-Down"]
pub type PkaPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESPI_PD` reader - eSPI_SIF Power-Down"]
pub type EspiPdR = crate::BitReader;
#[doc = "Field `ESPI_PD` writer - eSPI_SIF Power-Down"]
pub type EspiPdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ITIM32-4 Power-Down"]
    #[inline(always)]
    pub fn itim4_pd(&self) -> Itim4PdR {
        Itim4PdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ITIM32-5 Power-Down"]
    #[inline(always)]
    pub fn itim5_pd(&self) -> Itim5PdR {
        Itim5PdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ITIM32-6 Power-Down"]
    #[inline(always)]
    pub fn itim6_pd(&self) -> Itim6PdR {
        Itim6PdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - AES Power-Down"]
    #[inline(always)]
    pub fn aes_pd(&self) -> AesPdR {
        AesPdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SHA Power-Down"]
    #[inline(always)]
    pub fn sha_pd(&self) -> ShaPdR {
        ShaPdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PKA Power-Down"]
    #[inline(always)]
    pub fn pka_pd(&self) -> PkaPdR {
        PkaPdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - eSPI_SIF Power-Down"]
    #[inline(always)]
    pub fn espi_pd(&self) -> EspiPdR {
        EspiPdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWDWN_CTL6")
            .field("itim4_pd", &self.itim4_pd())
            .field("itim5_pd", &self.itim5_pd())
            .field("itim6_pd", &self.itim6_pd())
            .field("aes_pd", &self.aes_pd())
            .field("sha_pd", &self.sha_pd())
            .field("pka_pd", &self.pka_pd())
            .field("espi_pd", &self.espi_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - ITIM32-4 Power-Down"]
    #[inline(always)]
    pub fn itim4_pd(&mut self) -> Itim4PdW<PwdwnCtl6Spec> {
        Itim4PdW::new(self, 0)
    }
    #[doc = "Bit 1 - ITIM32-5 Power-Down"]
    #[inline(always)]
    pub fn itim5_pd(&mut self) -> Itim5PdW<PwdwnCtl6Spec> {
        Itim5PdW::new(self, 1)
    }
    #[doc = "Bit 2 - ITIM32-6 Power-Down"]
    #[inline(always)]
    pub fn itim6_pd(&mut self) -> Itim6PdW<PwdwnCtl6Spec> {
        Itim6PdW::new(self, 2)
    }
    #[doc = "Bit 4 - AES Power-Down"]
    #[inline(always)]
    pub fn aes_pd(&mut self) -> AesPdW<PwdwnCtl6Spec> {
        AesPdW::new(self, 4)
    }
    #[doc = "Bit 5 - SHA Power-Down"]
    #[inline(always)]
    pub fn sha_pd(&mut self) -> ShaPdW<PwdwnCtl6Spec> {
        ShaPdW::new(self, 5)
    }
    #[doc = "Bit 6 - PKA Power-Down"]
    #[inline(always)]
    pub fn pka_pd(&mut self) -> PkaPdW<PwdwnCtl6Spec> {
        PkaPdW::new(self, 6)
    }
    #[doc = "Bit 7 - eSPI_SIF Power-Down"]
    #[inline(always)]
    pub fn espi_pd(&mut self) -> EspiPdW<PwdwnCtl6Spec> {
        EspiPdW::new(self, 7)
    }
}
#[doc = "Power-Down Control 6 Register (PWDWN_CTL6)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdwnCtl6Spec;
impl crate::RegisterSpec for PwdwnCtl6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwdwn_ctl6::R`](R) reader structure"]
impl crate::Readable for PwdwnCtl6Spec {}
#[doc = "`write(|w| ..)` method takes [`pwdwn_ctl6::W`](W) writer structure"]
impl crate::Writable for PwdwnCtl6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWDWN_CTL6 to value 0"]
impl crate::Resettable for PwdwnCtl6Spec {
    const RESET_VALUE: u8 = 0;
}
