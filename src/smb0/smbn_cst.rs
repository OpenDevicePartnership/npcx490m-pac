#[doc = "Register `SMBnCST` reader"]
pub type R = crate::R<SmbnCstSpec>;
#[doc = "Register `SMBnCST` writer"]
pub type W = crate::W<SmbnCstSpec>;
#[doc = "Field `BUSY` reader - Module Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - Module Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BB` reader - Bus Busy"]
pub type BbR = crate::BitReader;
#[doc = "Field `BB` writer - Bus Busy"]
pub type BbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCH` reader - Address Match"]
pub type MatchR = crate::BitReader;
#[doc = "Field `MATCH` writer - Address Match"]
pub type MatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCMATCH` reader - Global Call Match"]
pub type GcmatchR = crate::BitReader;
#[doc = "Field `GCMATCH` writer - Global Call Match"]
pub type GcmatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSDA` reader - Test SDA Line"]
pub type TsdaR = crate::BitReader;
#[doc = "Field `TSDA` writer - Test SDA Line"]
pub type TsdaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGSCL` reader - Toggle SCL Line"]
pub type TgsclR = crate::BitReader;
#[doc = "Field `TGSCL` writer - Toggle SCL Line"]
pub type TgsclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHAF` reader - Match Address Field"]
pub type MatchafR = crate::BitReader;
#[doc = "Field `MATCHAF` writer - Match Address Field"]
pub type MatchafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARPMATCH` reader - ARP Address Match"]
pub type ArpmatchR = crate::BitReader;
#[doc = "Field `ARPMATCH` writer - ARP Address Match"]
pub type ArpmatchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Module Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus Busy"]
    #[inline(always)]
    pub fn bb(&self) -> BbR {
        BbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Address Match"]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global Call Match"]
    #[inline(always)]
    pub fn gcmatch(&self) -> GcmatchR {
        GcmatchR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Test SDA Line"]
    #[inline(always)]
    pub fn tsda(&self) -> TsdaR {
        TsdaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Toggle SCL Line"]
    #[inline(always)]
    pub fn tgscl(&self) -> TgsclR {
        TgsclR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Match Address Field"]
    #[inline(always)]
    pub fn matchaf(&self) -> MatchafR {
        MatchafR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ARP Address Match"]
    #[inline(always)]
    pub fn arpmatch(&self) -> ArpmatchR {
        ArpmatchR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMBnCST")
            .field("busy", &self.busy())
            .field("bb", &self.bb())
            .field("match_", &self.match_())
            .field("gcmatch", &self.gcmatch())
            .field("tsda", &self.tsda())
            .field("tgscl", &self.tgscl())
            .field("matchaf", &self.matchaf())
            .field("arpmatch", &self.arpmatch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Module Busy"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<SmbnCstSpec> {
        BusyW::new(self, 0)
    }
    #[doc = "Bit 1 - Bus Busy"]
    #[inline(always)]
    pub fn bb(&mut self) -> BbW<SmbnCstSpec> {
        BbW::new(self, 1)
    }
    #[doc = "Bit 2 - Address Match"]
    #[inline(always)]
    pub fn match_(&mut self) -> MatchW<SmbnCstSpec> {
        MatchW::new(self, 2)
    }
    #[doc = "Bit 3 - Global Call Match"]
    #[inline(always)]
    pub fn gcmatch(&mut self) -> GcmatchW<SmbnCstSpec> {
        GcmatchW::new(self, 3)
    }
    #[doc = "Bit 4 - Test SDA Line"]
    #[inline(always)]
    pub fn tsda(&mut self) -> TsdaW<SmbnCstSpec> {
        TsdaW::new(self, 4)
    }
    #[doc = "Bit 5 - Toggle SCL Line"]
    #[inline(always)]
    pub fn tgscl(&mut self) -> TgsclW<SmbnCstSpec> {
        TgsclW::new(self, 5)
    }
    #[doc = "Bit 6 - Match Address Field"]
    #[inline(always)]
    pub fn matchaf(&mut self) -> MatchafW<SmbnCstSpec> {
        MatchafW::new(self, 6)
    }
    #[doc = "Bit 7 - ARP Address Match"]
    #[inline(always)]
    pub fn arpmatch(&mut self) -> ArpmatchW<SmbnCstSpec> {
        ArpmatchW::new(self, 7)
    }
}
#[doc = "SMB Control Status Register (SMBnCST)\n\nYou can [`read`](crate::Reg::read) this register and get [`smbn_cst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smbn_cst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbnCstSpec;
impl crate::RegisterSpec for SmbnCstSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smbn_cst::R`](R) reader structure"]
impl crate::Readable for SmbnCstSpec {}
#[doc = "`write(|w| ..)` method takes [`smbn_cst::W`](W) writer structure"]
impl crate::Writable for SmbnCstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMBnCST to value 0"]
impl crate::Resettable for SmbnCstSpec {
    const RESET_VALUE: u8 = 0;
}
