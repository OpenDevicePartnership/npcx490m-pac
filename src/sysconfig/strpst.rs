#[doc = "Register `STRPST` reader"]
pub type R = crate::R<StrpstSpec>;
#[doc = "Field `NTRIS` reader - TRI-STATE Test Mode"]
pub type NtrisR = crate::BitReader;
#[doc = "Field `NTEST` reader - XOR-Tree Test Mode"]
pub type NtestR = crate::BitReader;
#[doc = "Field `nGP_SEL` reader - Gang Programmer Mode Select"]
pub type NGpSelR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - TRI-STATE Test Mode"]
    #[inline(always)]
    pub fn ntris(&self) -> NtrisR {
        NtrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOR-Tree Test Mode"]
    #[inline(always)]
    pub fn ntest(&self) -> NtestR {
        NtestR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Gang Programmer Mode Select"]
    #[inline(always)]
    pub fn n_gp_sel(&self) -> NGpSelR {
        NGpSelR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STRPST")
            .field("ntris", &self.ntris())
            .field("ntest", &self.ntest())
            .field("n_gp_sel", &self.n_gp_sel())
            .finish()
    }
}
#[doc = "Straps Status Register (STRPST)\n\nYou can [`read`](crate::Reg::read) this register and get [`strpst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrpstSpec;
impl crate::RegisterSpec for StrpstSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`strpst::R`](R) reader structure"]
impl crate::Readable for StrpstSpec {}
#[doc = "`reset()` method sets STRPST to value 0"]
impl crate::Resettable for StrpstSpec {
    const RESET_VALUE: u8 = 0;
}
