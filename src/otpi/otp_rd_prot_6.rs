#[doc = "Register `OTP_RD_PROT_6` reader"]
pub type R = crate::R<OtpRdProt6Spec>;
#[doc = "Register `OTP_RD_PROT_6` writer"]
pub type W = crate::W<OtpRdProt6Spec>;
#[doc = "Field `RDP8N_0` reader - Read Protect Block 8*n+0"]
pub type Rdp8n0R = crate::BitReader;
#[doc = "Field `RDP8N_0` writer - Read Protect Block 8*n+0"]
pub type Rdp8n0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP8N_1` reader - Read Protect Block 8*n+1"]
pub type Rdp8n1R = crate::BitReader;
#[doc = "Field `RDP8N_1` writer - Read Protect Block 8*n+1"]
pub type Rdp8n1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP8N_2` reader - Read Protect Block 8*n+2"]
pub type Rdp8n2R = crate::BitReader;
#[doc = "Field `RDP8N_2` writer - Read Protect Block 8*n+2"]
pub type Rdp8n2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP8N_3` reader - Read Protect Block 8*n+3"]
pub type Rdp8n3R = crate::BitReader;
#[doc = "Field `RDP8N_3` writer - Read Protect Block 8*n+3"]
pub type Rdp8n3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP8N_4` reader - Read Protect Block 8*n+4"]
pub type Rdp8n4R = crate::BitReader;
#[doc = "Field `RDP8N_4` writer - Read Protect Block 8*n+4"]
pub type Rdp8n4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP8N_5` reader - Read Protect Block 8*n+5"]
pub type Rdp8n5R = crate::BitReader;
#[doc = "Field `RDP8N_5` writer - Read Protect Block 8*n+5"]
pub type Rdp8n5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP8N_6` reader - Read Protect Block 8*n+6"]
pub type Rdp8n6R = crate::BitReader;
#[doc = "Field `RDP8N_6` writer - Read Protect Block 8*n+6"]
pub type Rdp8n6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP8N_7` reader - Read Protect Block 8*n+7"]
pub type Rdp8n7R = crate::BitReader;
#[doc = "Field `RDP8N_7` writer - Read Protect Block 8*n+7"]
pub type Rdp8n7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read Protect Block 8*n+0"]
    #[inline(always)]
    pub fn rdp8n_0(&self) -> Rdp8n0R {
        Rdp8n0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Protect Block 8*n+1"]
    #[inline(always)]
    pub fn rdp8n_1(&self) -> Rdp8n1R {
        Rdp8n1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read Protect Block 8*n+2"]
    #[inline(always)]
    pub fn rdp8n_2(&self) -> Rdp8n2R {
        Rdp8n2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read Protect Block 8*n+3"]
    #[inline(always)]
    pub fn rdp8n_3(&self) -> Rdp8n3R {
        Rdp8n3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read Protect Block 8*n+4"]
    #[inline(always)]
    pub fn rdp8n_4(&self) -> Rdp8n4R {
        Rdp8n4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read Protect Block 8*n+5"]
    #[inline(always)]
    pub fn rdp8n_5(&self) -> Rdp8n5R {
        Rdp8n5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read Protect Block 8*n+6"]
    #[inline(always)]
    pub fn rdp8n_6(&self) -> Rdp8n6R {
        Rdp8n6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read Protect Block 8*n+7"]
    #[inline(always)]
    pub fn rdp8n_7(&self) -> Rdp8n7R {
        Rdp8n7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_RD_PROT_6")
            .field("rdp8n_0", &self.rdp8n_0())
            .field("rdp8n_1", &self.rdp8n_1())
            .field("rdp8n_2", &self.rdp8n_2())
            .field("rdp8n_3", &self.rdp8n_3())
            .field("rdp8n_4", &self.rdp8n_4())
            .field("rdp8n_5", &self.rdp8n_5())
            .field("rdp8n_6", &self.rdp8n_6())
            .field("rdp8n_7", &self.rdp8n_7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Read Protect Block 8*n+0"]
    #[inline(always)]
    pub fn rdp8n_0(&mut self) -> Rdp8n0W<OtpRdProt6Spec> {
        Rdp8n0W::new(self, 0)
    }
    #[doc = "Bit 1 - Read Protect Block 8*n+1"]
    #[inline(always)]
    pub fn rdp8n_1(&mut self) -> Rdp8n1W<OtpRdProt6Spec> {
        Rdp8n1W::new(self, 1)
    }
    #[doc = "Bit 2 - Read Protect Block 8*n+2"]
    #[inline(always)]
    pub fn rdp8n_2(&mut self) -> Rdp8n2W<OtpRdProt6Spec> {
        Rdp8n2W::new(self, 2)
    }
    #[doc = "Bit 3 - Read Protect Block 8*n+3"]
    #[inline(always)]
    pub fn rdp8n_3(&mut self) -> Rdp8n3W<OtpRdProt6Spec> {
        Rdp8n3W::new(self, 3)
    }
    #[doc = "Bit 4 - Read Protect Block 8*n+4"]
    #[inline(always)]
    pub fn rdp8n_4(&mut self) -> Rdp8n4W<OtpRdProt6Spec> {
        Rdp8n4W::new(self, 4)
    }
    #[doc = "Bit 5 - Read Protect Block 8*n+5"]
    #[inline(always)]
    pub fn rdp8n_5(&mut self) -> Rdp8n5W<OtpRdProt6Spec> {
        Rdp8n5W::new(self, 5)
    }
    #[doc = "Bit 6 - Read Protect Block 8*n+6"]
    #[inline(always)]
    pub fn rdp8n_6(&mut self) -> Rdp8n6W<OtpRdProt6Spec> {
        Rdp8n6W::new(self, 6)
    }
    #[doc = "Bit 7 - Read Protect Block 8*n+7"]
    #[inline(always)]
    pub fn rdp8n_7(&mut self) -> Rdp8n7W<OtpRdProt6Spec> {
        Rdp8n7W::new(self, 7)
    }
}
#[doc = "OTP Read Protect 0-15 Register (OTP_RD_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_rd_prot_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_rd_prot_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpRdProt6Spec;
impl crate::RegisterSpec for OtpRdProt6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`otp_rd_prot_6::R`](R) reader structure"]
impl crate::Readable for OtpRdProt6Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_rd_prot_6::W`](W) writer structure"]
impl crate::Writable for OtpRdProt6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OTP_RD_PROT_6 to value 0"]
impl crate::Resettable for OtpRdProt6Spec {
    const RESET_VALUE: u8 = 0;
}
