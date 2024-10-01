#[doc = "Register `PWDWN_CTL7` reader"]
pub type R = crate::R<PwdwnCtl7Spec>;
#[doc = "Register `PWDWN_CTL7` writer"]
pub type W = crate::W<PwdwnCtl7Spec>;
#[doc = "Field `SMB5_PD` reader - SMB5 Power-Down"]
pub type Smb5PdR = crate::BitReader;
#[doc = "Field `SMB5_PD` writer - SMB5 Power-Down"]
pub type Smb5PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB6_PD` reader - SMB6 Power-Down"]
pub type Smb6PdR = crate::BitReader;
#[doc = "Field `SMB6_PD` writer - SMB6 Power-Down"]
pub type Smb6PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMB7_PD` reader - SMB7 Power-Down"]
pub type Smb7PdR = crate::BitReader;
#[doc = "Field `SMB7_PD` writer - SMB7 Power-Down"]
pub type Smb7PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4_PD` reader - CR_UART4 Power-Down"]
pub type Uart4PdR = crate::BitReader;
#[doc = "Field `UART4_PD` writer - CR_UART4 Power-Down"]
pub type Uart4PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART3_PD` reader - CR_UART3 Power-Down"]
pub type Uart3PdR = crate::BitReader;
#[doc = "Field `UART3_PD` writer - CR_UART3 Power-Down"]
pub type Uart3PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITIM64_PD` reader - ITIM64 Power-Down"]
pub type Itim64PdR = crate::BitReader;
#[doc = "Field `ITIM64_PD` writer - ITIM64 Power-Down"]
pub type Itim64PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2_PD` reader - CR_UART2 Power-Down"]
pub type Uart2PdR = crate::BitReader;
#[doc = "Field `UART2_PD` writer - CR_UART2 Power-Down"]
pub type Uart2PdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SMB5 Power-Down"]
    #[inline(always)]
    pub fn smb5_pd(&self) -> Smb5PdR {
        Smb5PdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMB6 Power-Down"]
    #[inline(always)]
    pub fn smb6_pd(&self) -> Smb6PdR {
        Smb6PdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SMB7 Power-Down"]
    #[inline(always)]
    pub fn smb7_pd(&self) -> Smb7PdR {
        Smb7PdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CR_UART4 Power-Down"]
    #[inline(always)]
    pub fn uart4_pd(&self) -> Uart4PdR {
        Uart4PdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CR_UART3 Power-Down"]
    #[inline(always)]
    pub fn uart3_pd(&self) -> Uart3PdR {
        Uart3PdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ITIM64 Power-Down"]
    #[inline(always)]
    pub fn itim64_pd(&self) -> Itim64PdR {
        Itim64PdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CR_UART2 Power-Down"]
    #[inline(always)]
    pub fn uart2_pd(&self) -> Uart2PdR {
        Uart2PdR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWDWN_CTL7")
            .field("smb5_pd", &self.smb5_pd())
            .field("smb6_pd", &self.smb6_pd())
            .field("smb7_pd", &self.smb7_pd())
            .field("uart4_pd", &self.uart4_pd())
            .field("uart3_pd", &self.uart3_pd())
            .field("itim64_pd", &self.itim64_pd())
            .field("uart2_pd", &self.uart2_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - SMB5 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn smb5_pd(&mut self) -> Smb5PdW<PwdwnCtl7Spec> {
        Smb5PdW::new(self, 0)
    }
    #[doc = "Bit 1 - SMB6 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn smb6_pd(&mut self) -> Smb6PdW<PwdwnCtl7Spec> {
        Smb6PdW::new(self, 1)
    }
    #[doc = "Bit 2 - SMB7 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn smb7_pd(&mut self) -> Smb7PdW<PwdwnCtl7Spec> {
        Smb7PdW::new(self, 2)
    }
    #[doc = "Bit 3 - CR_UART4 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn uart4_pd(&mut self) -> Uart4PdW<PwdwnCtl7Spec> {
        Uart4PdW::new(self, 3)
    }
    #[doc = "Bit 4 - CR_UART3 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn uart3_pd(&mut self) -> Uart3PdW<PwdwnCtl7Spec> {
        Uart3PdW::new(self, 4)
    }
    #[doc = "Bit 5 - ITIM64 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn itim64_pd(&mut self) -> Itim64PdW<PwdwnCtl7Spec> {
        Itim64PdW::new(self, 5)
    }
    #[doc = "Bit 6 - CR_UART2 Power-Down"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_pd(&mut self) -> Uart2PdW<PwdwnCtl7Spec> {
        Uart2PdW::new(self, 6)
    }
}
#[doc = "Power-Down Control 7 Register (PWDWN_CTL7)\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdwn_ctl7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdwn_ctl7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwdwnCtl7Spec;
impl crate::RegisterSpec for PwdwnCtl7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pwdwn_ctl7::R`](R) reader structure"]
impl crate::Readable for PwdwnCtl7Spec {}
#[doc = "`write(|w| ..)` method takes [`pwdwn_ctl7::W`](W) writer structure"]
impl crate::Writable for PwdwnCtl7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PWDWN_CTL7 to value 0"]
impl crate::Resettable for PwdwnCtl7Spec {
    const RESET_VALUE: u8 = 0;
}
