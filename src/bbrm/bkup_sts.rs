#[doc = "Register `BKUP_STS` reader"]
pub type R = crate::R<BkupStsSpec>;
#[doc = "Register `BKUP_STS` writer"]
pub type W = crate::W<BkupStsSpec>;
#[doc = "Field `VCC1_STS` reader - VCC1 Power-Fail Status"]
pub type Vcc1StsR = crate::BitReader;
#[doc = "Field `VCC1_STS` writer - VCC1 Power-Fail Status"]
pub type Vcc1StsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRUD1_ALARM` reader - Intruder1 Alarm"]
pub type Intrud1AlarmR = crate::BitReader;
#[doc = "Field `INTRUD1_ALARM` writer - Intruder1 Alarm"]
pub type Intrud1AlarmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRUD2_ALARM` reader - Intruder2 Alarm"]
pub type Intrud2AlarmR = crate::BitReader;
#[doc = "Field `INTRUD2_ALARM` writer - Intruder2 Alarm"]
pub type Intrud2AlarmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBBR` reader - Invalid Battery-Backed RAM"]
pub type IbbrR = crate::BitReader;
#[doc = "Field `IBBR` writer - Invalid Battery-Backed RAM"]
pub type IbbrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VCC1 Power-Fail Status"]
    #[inline(always)]
    pub fn vcc1_sts(&self) -> Vcc1StsR {
        Vcc1StsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Intruder1 Alarm"]
    #[inline(always)]
    pub fn intrud1_alarm(&self) -> Intrud1AlarmR {
        Intrud1AlarmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Intruder2 Alarm"]
    #[inline(always)]
    pub fn intrud2_alarm(&self) -> Intrud2AlarmR {
        Intrud2AlarmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Invalid Battery-Backed RAM"]
    #[inline(always)]
    pub fn ibbr(&self) -> IbbrR {
        IbbrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKUP_STS")
            .field("vcc1_sts", &self.vcc1_sts())
            .field("intrud1_alarm", &self.intrud1_alarm())
            .field("intrud2_alarm", &self.intrud2_alarm())
            .field("ibbr", &self.ibbr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - VCC1 Power-Fail Status"]
    #[inline(always)]
    #[must_use]
    pub fn vcc1_sts(&mut self) -> Vcc1StsW<BkupStsSpec> {
        Vcc1StsW::new(self, 0)
    }
    #[doc = "Bit 5 - Intruder1 Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn intrud1_alarm(&mut self) -> Intrud1AlarmW<BkupStsSpec> {
        Intrud1AlarmW::new(self, 5)
    }
    #[doc = "Bit 6 - Intruder2 Alarm"]
    #[inline(always)]
    #[must_use]
    pub fn intrud2_alarm(&mut self) -> Intrud2AlarmW<BkupStsSpec> {
        Intrud2AlarmW::new(self, 6)
    }
    #[doc = "Bit 7 - Invalid Battery-Backed RAM"]
    #[inline(always)]
    #[must_use]
    pub fn ibbr(&mut self) -> IbbrW<BkupStsSpec> {
        IbbrW::new(self, 7)
    }
}
#[doc = "BBRM Status Register (BKUP_STS)\n\nYou can [`read`](crate::Reg::read) this register and get [`bkup_sts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bkup_sts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BkupStsSpec;
impl crate::RegisterSpec for BkupStsSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bkup_sts::R`](R) reader structure"]
impl crate::Readable for BkupStsSpec {}
#[doc = "`write(|w| ..)` method takes [`bkup_sts::W`](W) writer structure"]
impl crate::Writable for BkupStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BKUP_STS to value 0"]
impl crate::Resettable for BkupStsSpec {
    const RESET_VALUE: u8 = 0;
}
