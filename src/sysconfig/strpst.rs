#[doc = "Register `STRPST` reader"]
pub type R = crate::R<StrpstSpec>;
#[doc = "Register `STRPST` writer"]
pub type W = crate::W<StrpstSpec>;
#[doc = "Field `NTRIS` reader - TRI-STATE Test Mode"]
pub type NtrisR = crate::BitReader;
#[doc = "Field `NTRIS` writer - TRI-STATE Test Mode"]
pub type NtrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NTEST` reader - XOR-Tree Test Mode"]
pub type NtestR = crate::BitReader;
#[doc = "Field `NTEST` writer - XOR-Tree Test Mode"]
pub type NtestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NJEN` reader - JTAG Strap"]
pub type NjenR = crate::BitReader;
#[doc = "Field `NJEN` writer - JTAG Strap"]
pub type NjenW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - JTAG Strap"]
    #[inline(always)]
    pub fn njen(&self) -> NjenR {
        NjenR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STRPST")
            .field("ntris", &self.ntris())
            .field("ntest", &self.ntest())
            .field("njen", &self.njen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - TRI-STATE Test Mode"]
    #[inline(always)]
    pub fn ntris(&mut self) -> NtrisW<StrpstSpec> {
        NtrisW::new(self, 1)
    }
    #[doc = "Bit 2 - XOR-Tree Test Mode"]
    #[inline(always)]
    pub fn ntest(&mut self) -> NtestW<StrpstSpec> {
        NtestW::new(self, 2)
    }
    #[doc = "Bit 5 - JTAG Strap"]
    #[inline(always)]
    pub fn njen(&mut self) -> NjenW<StrpstSpec> {
        NjenW::new(self, 5)
    }
}
#[doc = "Straps Status Register (STRPST)\n\nYou can [`read`](crate::Reg::read) this register and get [`strpst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`strpst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StrpstSpec;
impl crate::RegisterSpec for StrpstSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`strpst::R`](R) reader structure"]
impl crate::Readable for StrpstSpec {}
#[doc = "`write(|w| ..)` method takes [`strpst::W`](W) writer structure"]
impl crate::Writable for StrpstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets STRPST to value 0"]
impl crate::Resettable for StrpstSpec {
    const RESET_VALUE: u8 = 0;
}
