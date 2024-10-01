#[doc = "Register `OTP_WR_PROT_14` reader"]
pub type R = crate::R<OtpWrProt14Spec>;
#[doc = "Register `OTP_WR_PROT_14` writer"]
pub type W = crate::W<OtpWrProt14Spec>;
#[doc = "Field `WRP8N_0` reader - Write Protect Block 8*n+0"]
pub type Wrp8n0R = crate::BitReader;
#[doc = "Field `WRP8N_0` writer - Write Protect Block 8*n+0"]
pub type Wrp8n0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRP8N_1` reader - Write Protect Block 8*n+1"]
pub type Wrp8n1R = crate::BitReader;
#[doc = "Field `WRP8N_1` writer - Write Protect Block 8*n+1"]
pub type Wrp8n1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRP8N_2` reader - Write Protect Block 8*n+2"]
pub type Wrp8n2R = crate::BitReader;
#[doc = "Field `WRP8N_2` writer - Write Protect Block 8*n+2"]
pub type Wrp8n2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRP8N_3` reader - Write Protect Block 8*n+3"]
pub type Wrp8n3R = crate::BitReader;
#[doc = "Field `WRP8N_3` writer - Write Protect Block 8*n+3"]
pub type Wrp8n3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRP8N_4` reader - Write Protect Block 8*n+4"]
pub type Wrp8n4R = crate::BitReader;
#[doc = "Field `WRP8N_4` writer - Write Protect Block 8*n+4"]
pub type Wrp8n4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRP8N_5` reader - Write Protect Block 8*n+5"]
pub type Wrp8n5R = crate::BitReader;
#[doc = "Field `WRP8N_5` writer - Write Protect Block 8*n+5"]
pub type Wrp8n5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRP8N_6` reader - Write Protect Block 8*n+6"]
pub type Wrp8n6R = crate::BitReader;
#[doc = "Field `WRP8N_6` writer - Write Protect Block 8*n+6"]
pub type Wrp8n6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRP8N_7` reader - Write Protect Block 8*n+7"]
pub type Wrp8n7R = crate::BitReader;
#[doc = "Field `WRP8N_7` writer - Write Protect Block 8*n+7"]
pub type Wrp8n7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Protect Block 8*n+0"]
    #[inline(always)]
    pub fn wrp8n_0(&self) -> Wrp8n0R {
        Wrp8n0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protect Block 8*n+1"]
    #[inline(always)]
    pub fn wrp8n_1(&self) -> Wrp8n1R {
        Wrp8n1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Protect Block 8*n+2"]
    #[inline(always)]
    pub fn wrp8n_2(&self) -> Wrp8n2R {
        Wrp8n2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write Protect Block 8*n+3"]
    #[inline(always)]
    pub fn wrp8n_3(&self) -> Wrp8n3R {
        Wrp8n3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write Protect Block 8*n+4"]
    #[inline(always)]
    pub fn wrp8n_4(&self) -> Wrp8n4R {
        Wrp8n4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protect Block 8*n+5"]
    #[inline(always)]
    pub fn wrp8n_5(&self) -> Wrp8n5R {
        Wrp8n5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Protect Block 8*n+6"]
    #[inline(always)]
    pub fn wrp8n_6(&self) -> Wrp8n6R {
        Wrp8n6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write Protect Block 8*n+7"]
    #[inline(always)]
    pub fn wrp8n_7(&self) -> Wrp8n7R {
        Wrp8n7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTP_WR_PROT_14")
            .field("wrp8n_0", &self.wrp8n_0())
            .field("wrp8n_1", &self.wrp8n_1())
            .field("wrp8n_2", &self.wrp8n_2())
            .field("wrp8n_3", &self.wrp8n_3())
            .field("wrp8n_4", &self.wrp8n_4())
            .field("wrp8n_5", &self.wrp8n_5())
            .field("wrp8n_6", &self.wrp8n_6())
            .field("wrp8n_7", &self.wrp8n_7())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect Block 8*n+0"]
    #[inline(always)]
    #[must_use]
    pub fn wrp8n_0(&mut self) -> Wrp8n0W<OtpWrProt14Spec> {
        Wrp8n0W::new(self, 0)
    }
    #[doc = "Bit 1 - Write Protect Block 8*n+1"]
    #[inline(always)]
    #[must_use]
    pub fn wrp8n_1(&mut self) -> Wrp8n1W<OtpWrProt14Spec> {
        Wrp8n1W::new(self, 1)
    }
    #[doc = "Bit 2 - Write Protect Block 8*n+2"]
    #[inline(always)]
    #[must_use]
    pub fn wrp8n_2(&mut self) -> Wrp8n2W<OtpWrProt14Spec> {
        Wrp8n2W::new(self, 2)
    }
    #[doc = "Bit 3 - Write Protect Block 8*n+3"]
    #[inline(always)]
    #[must_use]
    pub fn wrp8n_3(&mut self) -> Wrp8n3W<OtpWrProt14Spec> {
        Wrp8n3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write Protect Block 8*n+4"]
    #[inline(always)]
    #[must_use]
    pub fn wrp8n_4(&mut self) -> Wrp8n4W<OtpWrProt14Spec> {
        Wrp8n4W::new(self, 4)
    }
    #[doc = "Bit 5 - Write Protect Block 8*n+5"]
    #[inline(always)]
    #[must_use]
    pub fn wrp8n_5(&mut self) -> Wrp8n5W<OtpWrProt14Spec> {
        Wrp8n5W::new(self, 5)
    }
    #[doc = "Bit 6 - Write Protect Block 8*n+6"]
    #[inline(always)]
    #[must_use]
    pub fn wrp8n_6(&mut self) -> Wrp8n6W<OtpWrProt14Spec> {
        Wrp8n6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write Protect Block 8*n+7"]
    #[inline(always)]
    #[must_use]
    pub fn wrp8n_7(&mut self) -> Wrp8n7W<OtpWrProt14Spec> {
        Wrp8n7W::new(self, 7)
    }
}
#[doc = "OTP Write Protect 0-15 Register (OTP_WR_PROT_0-15)\n\nYou can [`read`](crate::Reg::read) this register and get [`otp_wr_prot_14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otp_wr_prot_14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtpWrProt14Spec;
impl crate::RegisterSpec for OtpWrProt14Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`otp_wr_prot_14::R`](R) reader structure"]
impl crate::Readable for OtpWrProt14Spec {}
#[doc = "`write(|w| ..)` method takes [`otp_wr_prot_14::W`](W) writer structure"]
impl crate::Writable for OtpWrProt14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OTP_WR_PROT_14 to value 0"]
impl crate::Resettable for OtpWrProt14Spec {
    const RESET_VALUE: u8 = 0;
}
