#[doc = "Register `TnCPCFG` reader"]
pub type R = crate::R<TnCpcfgSpec>;
#[doc = "Register `TnCPCFG` writer"]
pub type W = crate::W<TnCpcfgSpec>;
#[doc = "Field `CPASEL` reader - Compare Function A Select"]
pub type CpaselR = crate::BitReader;
#[doc = "Field `CPASEL` writer - Compare Function A Select"]
pub type CpaselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOAEN` reader - Low Indication Enable for Comparator A"]
pub type LoaenR = crate::BitReader;
#[doc = "Field `LOAEN` writer - Low Indication Enable for Comparator A"]
pub type LoaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EQAEN` reader - Equal Indication Enable for Comparator A"]
pub type EqaenR = crate::BitReader;
#[doc = "Field `EQAEN` writer - Equal Indication Enable for Comparator A"]
pub type EqaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIAEN` reader - High Indication Enable for Comparator A"]
pub type HiaenR = crate::BitReader;
#[doc = "Field `HIAEN` writer - High Indication Enable for Comparator A"]
pub type HiaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPBSEL` reader - Compare Function B Select"]
pub type CpbselR = crate::BitReader;
#[doc = "Field `CPBSEL` writer - Compare Function B Select"]
pub type CpbselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOBEN` reader - Low Indication Enable for Comparator B"]
pub type LobenR = crate::BitReader;
#[doc = "Field `LOBEN` writer - Low Indication Enable for Comparator B"]
pub type LobenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EQBEN` reader - Equal Indication Enable for Comparator B"]
pub type EqbenR = crate::BitReader;
#[doc = "Field `EQBEN` writer - Equal Indication Enable for Comparator B"]
pub type EqbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIBEN` reader - High Indication Enable for Comparator B"]
pub type HibenR = crate::BitReader;
#[doc = "Field `HIBEN` writer - High Indication Enable for Comparator B"]
pub type HibenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Compare Function A Select"]
    #[inline(always)]
    pub fn cpasel(&self) -> CpaselR {
        CpaselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Indication Enable for Comparator A"]
    #[inline(always)]
    pub fn loaen(&self) -> LoaenR {
        LoaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Equal Indication Enable for Comparator A"]
    #[inline(always)]
    pub fn eqaen(&self) -> EqaenR {
        EqaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High Indication Enable for Comparator A"]
    #[inline(always)]
    pub fn hiaen(&self) -> HiaenR {
        HiaenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Function B Select"]
    #[inline(always)]
    pub fn cpbsel(&self) -> CpbselR {
        CpbselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low Indication Enable for Comparator B"]
    #[inline(always)]
    pub fn loben(&self) -> LobenR {
        LobenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Equal Indication Enable for Comparator B"]
    #[inline(always)]
    pub fn eqben(&self) -> EqbenR {
        EqbenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - High Indication Enable for Comparator B"]
    #[inline(always)]
    pub fn hiben(&self) -> HibenR {
        HibenR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TnCPCFG")
            .field("cpasel", &self.cpasel())
            .field("loaen", &self.loaen())
            .field("eqaen", &self.eqaen())
            .field("hiaen", &self.hiaen())
            .field("cpbsel", &self.cpbsel())
            .field("loben", &self.loben())
            .field("eqben", &self.eqben())
            .field("hiben", &self.hiben())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Compare Function A Select"]
    #[inline(always)]
    #[must_use]
    pub fn cpasel(&mut self) -> CpaselW<TnCpcfgSpec> {
        CpaselW::new(self, 0)
    }
    #[doc = "Bit 1 - Low Indication Enable for Comparator A"]
    #[inline(always)]
    #[must_use]
    pub fn loaen(&mut self) -> LoaenW<TnCpcfgSpec> {
        LoaenW::new(self, 1)
    }
    #[doc = "Bit 2 - Equal Indication Enable for Comparator A"]
    #[inline(always)]
    #[must_use]
    pub fn eqaen(&mut self) -> EqaenW<TnCpcfgSpec> {
        EqaenW::new(self, 2)
    }
    #[doc = "Bit 3 - High Indication Enable for Comparator A"]
    #[inline(always)]
    #[must_use]
    pub fn hiaen(&mut self) -> HiaenW<TnCpcfgSpec> {
        HiaenW::new(self, 3)
    }
    #[doc = "Bit 4 - Compare Function B Select"]
    #[inline(always)]
    #[must_use]
    pub fn cpbsel(&mut self) -> CpbselW<TnCpcfgSpec> {
        CpbselW::new(self, 4)
    }
    #[doc = "Bit 5 - Low Indication Enable for Comparator B"]
    #[inline(always)]
    #[must_use]
    pub fn loben(&mut self) -> LobenW<TnCpcfgSpec> {
        LobenW::new(self, 5)
    }
    #[doc = "Bit 6 - Equal Indication Enable for Comparator B"]
    #[inline(always)]
    #[must_use]
    pub fn eqben(&mut self) -> EqbenW<TnCpcfgSpec> {
        EqbenW::new(self, 6)
    }
    #[doc = "Bit 7 - High Indication Enable for Comparator B"]
    #[inline(always)]
    #[must_use]
    pub fn hiben(&mut self) -> HibenW<TnCpcfgSpec> {
        HibenW::new(self, 7)
    }
}
#[doc = "Compare Configuration Register (TnCPCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_cpcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_cpcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnCpcfgSpec;
impl crate::RegisterSpec for TnCpcfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tn_cpcfg::R`](R) reader structure"]
impl crate::Readable for TnCpcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_cpcfg::W`](W) writer structure"]
impl crate::Writable for TnCpcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TnCPCFG to value 0"]
impl crate::Resettable for TnCpcfgSpec {
    const RESET_VALUE: u8 = 0;
}
