#[doc = "Register `SMBnST` reader"]
pub type R = crate::R<SmbnStSpec>;
#[doc = "Register `SMBnST` writer"]
pub type W = crate::W<SmbnStSpec>;
#[doc = "Field `XMIT` reader - Transmit Mode"]
pub type XmitR = crate::BitReader;
#[doc = "Field `XMIT` writer - Transmit Mode"]
pub type XmitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER` reader - Master Mode"]
pub type MasterR = crate::BitReader;
#[doc = "Field `MASTER` writer - Master Mode"]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NMATCH` reader - New Match"]
pub type NmatchR = crate::BitReader;
#[doc = "Field `NMATCH` writer - New Match"]
pub type NmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STASTR` reader - Stall After Start"]
pub type StastrR = crate::BitReader;
#[doc = "Field `STASTR` writer - Stall After Start"]
pub type StastrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEGACK` reader - Negative Acknowledge"]
pub type NegackR = crate::BitReader;
#[doc = "Field `NEGACK` writer - Negative Acknowledge"]
pub type NegackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BER` reader - Bus Error"]
pub type BerR = crate::BitReader;
#[doc = "Field `BER` writer - Bus Error"]
pub type BerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDAST` reader - SDA Status"]
pub type SdastR = crate::BitReader;
#[doc = "Field `SDAST` writer - SDA Status"]
pub type SdastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLVSTP` reader - Slave Stop"]
pub type SlvstpR = crate::BitReader;
#[doc = "Field `SLVSTP` writer - Slave Stop"]
pub type SlvstpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Mode"]
    #[inline(always)]
    pub fn xmit(&self) -> XmitR {
        XmitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Mode"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - New Match"]
    #[inline(always)]
    pub fn nmatch(&self) -> NmatchR {
        NmatchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stall After Start"]
    #[inline(always)]
    pub fn stastr(&self) -> StastrR {
        StastrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Negative Acknowledge"]
    #[inline(always)]
    pub fn negack(&self) -> NegackR {
        NegackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Error"]
    #[inline(always)]
    pub fn ber(&self) -> BerR {
        BerR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SDA Status"]
    #[inline(always)]
    pub fn sdast(&self) -> SdastR {
        SdastR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave Stop"]
    #[inline(always)]
    pub fn slvstp(&self) -> SlvstpR {
        SlvstpR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnST")
            .field("xmit", &self.xmit())
            .field("master", &self.master())
            .field("nmatch", &self.nmatch())
            .field("stastr", &self.stastr())
            .field("negack", &self.negack())
            .field("ber", &self.ber())
            .field("sdast", &self.sdast())
            .field("slvstp", &self.slvstp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Mode"]
    #[inline(always)]
    #[must_use]
    pub fn xmit(&mut self) -> XmitW<SmbnStSpec> {
        XmitW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Mode"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MasterW<SmbnStSpec> {
        MasterW::new(self, 1)
    }
    #[doc = "Bit 2 - New Match"]
    #[inline(always)]
    #[must_use]
    pub fn nmatch(&mut self) -> NmatchW<SmbnStSpec> {
        NmatchW::new(self, 2)
    }
    #[doc = "Bit 3 - Stall After Start"]
    #[inline(always)]
    #[must_use]
    pub fn stastr(&mut self) -> StastrW<SmbnStSpec> {
        StastrW::new(self, 3)
    }
    #[doc = "Bit 4 - Negative Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn negack(&mut self) -> NegackW<SmbnStSpec> {
        NegackW::new(self, 4)
    }
    #[doc = "Bit 5 - Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn ber(&mut self) -> BerW<SmbnStSpec> {
        BerW::new(self, 5)
    }
    #[doc = "Bit 6 - SDA Status"]
    #[inline(always)]
    #[must_use]
    pub fn sdast(&mut self) -> SdastW<SmbnStSpec> {
        SdastW::new(self, 6)
    }
    #[doc = "Bit 7 - Slave Stop"]
    #[inline(always)]
    #[must_use]
    pub fn slvstp(&mut self) -> SlvstpW<SmbnStSpec> {
        SlvstpW::new(self, 7)
    }
}
#[doc = "SMB Status Register (SMBnST)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnStSpec;
impl crate::RegisterSpec for SmbnStSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_st::R`](R) reader structure"]
impl crate::Readable for SmbnStSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_st::W`](W) writer structure"]
impl crate::Writable for SmbnStSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnST to value 0"]
impl crate::Resettable for SmbnStSpec {
    const RESET_VALUE: u8 = 0;
}
