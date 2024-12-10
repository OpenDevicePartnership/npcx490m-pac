#[doc = "Register `HOFS_CTL` reader"]
pub type R = crate::R<HofsCtlSpec>;
#[doc = "Register `HOFS_CTL` writer"]
pub type W = crate::W<HofsCtlSpec>;
#[doc = "Field `HOFS1R_IE` reader - Enable Interrupt by Host Read to Offset in Window 1"]
pub type Hofs1rIeR = crate::BitReader;
#[doc = "Field `HOFS1R_IE` writer - Enable Interrupt by Host Read to Offset in Window 1"]
pub type Hofs1rIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS1W_IE` reader - Enable Interrupt by Host Write to Offset in Window 1"]
pub type Hofs1wIeR = crate::BitReader;
#[doc = "Field `HOFS1W_IE` writer - Enable Interrupt by Host Write to Offset in Window 1"]
pub type Hofs1wIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS2R_IE` reader - Enable Interrupt by Host Read to Offset in Window 2"]
pub type Hofs2rIeR = crate::BitReader;
#[doc = "Field `HOFS2R_IE` writer - Enable Interrupt by Host Read to Offset in Window 2"]
pub type Hofs2rIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOFS2W_IE` reader - Enable Interrupt by Host Write to Offset in Window 2"]
pub type Hofs2wIeR = crate::BitReader;
#[doc = "Field `HOFS2W_IE` writer - Enable Interrupt by Host Write to Offset in Window 2"]
pub type Hofs2wIeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Interrupt by Host Read to Offset in Window 1"]
    #[inline(always)]
    pub fn hofs1r_ie(&self) -> Hofs1rIeR {
        Hofs1rIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt by Host Write to Offset in Window 1"]
    #[inline(always)]
    pub fn hofs1w_ie(&self) -> Hofs1wIeR {
        Hofs1wIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Interrupt by Host Read to Offset in Window 2"]
    #[inline(always)]
    pub fn hofs2r_ie(&self) -> Hofs2rIeR {
        Hofs2rIeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Interrupt by Host Write to Offset in Window 2"]
    #[inline(always)]
    pub fn hofs2w_ie(&self) -> Hofs2wIeR {
        Hofs2wIeR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOFS_CTL")
            .field("hofs1r_ie", &self.hofs1r_ie())
            .field("hofs1w_ie", &self.hofs1w_ie())
            .field("hofs2r_ie", &self.hofs2r_ie())
            .field("hofs2w_ie", &self.hofs2w_ie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt by Host Read to Offset in Window 1"]
    #[inline(always)]
    pub fn hofs1r_ie(&mut self) -> Hofs1rIeW<HofsCtlSpec> {
        Hofs1rIeW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Interrupt by Host Write to Offset in Window 1"]
    #[inline(always)]
    pub fn hofs1w_ie(&mut self) -> Hofs1wIeW<HofsCtlSpec> {
        Hofs1wIeW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Interrupt by Host Read to Offset in Window 2"]
    #[inline(always)]
    pub fn hofs2r_ie(&mut self) -> Hofs2rIeW<HofsCtlSpec> {
        Hofs2rIeW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Interrupt by Host Write to Offset in Window 2"]
    #[inline(always)]
    pub fn hofs2w_ie(&mut self) -> Hofs2wIeW<HofsCtlSpec> {
        Hofs2wIeW::new(self, 3)
    }
}
#[doc = "Host_Offset in Windows 1,2 Control Register (HOFS_CTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`hofs_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hofs_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HofsCtlSpec;
impl crate::RegisterSpec for HofsCtlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hofs_ctl::R`](R) reader structure"]
impl crate::Readable for HofsCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`hofs_ctl::W`](W) writer structure"]
impl crate::Writable for HofsCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HOFS_CTL to value 0"]
impl crate::Resettable for HofsCtlSpec {
    const RESET_VALUE: u8 = 0;
}
