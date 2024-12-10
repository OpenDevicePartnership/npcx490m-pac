#[doc = "Register `HOFSE_CTL` reader"]
pub type R = crate::R<HofseCtlSpec>;
#[doc = "Register `HOFSE_CTL` writer"]
pub type W = crate::W<HofseCtlSpec>;
#[doc = "Field `HOFS3R_IE` reader - Enable Interrupt by Host Read to Offset in Window 3"]
pub type Hofs3rIeR = crate::BitReader;
#[doc = "Field `HOFS3R_IE` writer - Enable Interrupt by Host Read to Offset in Window 3"]
pub type Hofs3rIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS3W_IE` reader - Enable Interrupt by Host Write to Offset in Window 3"]
pub type Hofs3wIeR = crate::BitReader;
#[doc = "Field `HOFS3W_IE` writer - Enable Interrupt by Host Write to Offset in Window 3"]
pub type Hofs3wIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS4R_IE` reader - Enable Interrupt by Host Read to Offset in Window 4"]
pub type Hofs4rIeR = crate::BitReader;
#[doc = "Field `HOFS4R_IE` writer - Enable Interrupt by Host Read to Offset in Window 4"]
pub type Hofs4rIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS4W_IE` reader - Enable Interrupt by Host Write to Offset in Window 4"]
pub type Hofs4wIeR = crate::BitReader;
#[doc = "Field `HOFS4W_IE` writer - Enable Interrupt by Host Write to Offset in Window 4"]
pub type Hofs4wIeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Interrupt by Host Read to Offset in Window 3"]
    #[inline(always)]
    pub fn hofs3r_ie(&self) -> Hofs3rIeR {
        Hofs3rIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt by Host Write to Offset in Window 3"]
    #[inline(always)]
    pub fn hofs3w_ie(&self) -> Hofs3wIeR {
        Hofs3wIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Interrupt by Host Read to Offset in Window 4"]
    #[inline(always)]
    pub fn hofs4r_ie(&self) -> Hofs4rIeR {
        Hofs4rIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Interrupt by Host Write to Offset in Window 4"]
    #[inline(always)]
    pub fn hofs4w_ie(&self) -> Hofs4wIeR {
        Hofs4wIeR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOFSE_CTL")
            .field("hofs3r_ie", &self.hofs3r_ie())
            .field("hofs3w_ie", &self.hofs3w_ie())
            .field("hofs4r_ie", &self.hofs4r_ie())
            .field("hofs4w_ie", &self.hofs4w_ie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt by Host Read to Offset in Window 3"]
    #[inline(always)]
    pub fn hofs3r_ie(&mut self) -> Hofs3rIeW<HofseCtlSpec> {
        Hofs3rIeW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Interrupt by Host Write to Offset in Window 3"]
    #[inline(always)]
    pub fn hofs3w_ie(&mut self) -> Hofs3wIeW<HofseCtlSpec> {
        Hofs3wIeW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Interrupt by Host Read to Offset in Window 4"]
    #[inline(always)]
    pub fn hofs4r_ie(&mut self) -> Hofs4rIeW<HofseCtlSpec> {
        Hofs4rIeW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Interrupt by Host Write to Offset in Window 4"]
    #[inline(always)]
    pub fn hofs4w_ie(&mut self) -> Hofs4wIeW<HofseCtlSpec> {
        Hofs4wIeW::new(self, 3)
    }
}
#[doc = "Host_Offset in Windows 3,4 Control Register (HOFSE_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hofse_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hofse_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HofseCtlSpec;
impl crate::RegisterSpec for HofseCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hofse_ctl::R`](R) reader structure"]
impl crate::Readable for HofseCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`hofse_ctl::W`](W) writer structure"]
impl crate::Writable for HofseCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HOFSE_CTL to value 0"]
impl crate::Resettable for HofseCtlSpec {
    const RESET_VALUE: u8 = 0;
}
