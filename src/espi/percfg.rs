#[doc = "Register `PERCFG` reader"]
pub type R = crate::R<PercfgSpec>;
#[doc = "Register `PERCFG` writer"]
pub type W = crate::W<PercfgSpec>;
#[doc = "Field `BMMEN` reader - Bus Master Enable"]
pub type BmmenR = crate::BitReader;
#[doc = "Field `BMMEN` writer - Bus Master Enable"]
pub type BmmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERPLSIZE` reader - Peripheral Channel Maximum Payload Size"]
pub type PerplsizeR = crate::FieldReader;
#[doc = "Field `PERPLSIZE` writer - Peripheral Channel Maximum Payload Size"]
pub type PerplsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BMREQSIZE` reader - Peripheral Channel Maximum Read Request Size"]
pub type BmreqsizeR = crate::FieldReader;
#[doc = "Field `BMREQSIZE` writer - Peripheral Channel Maximum Read Request Size"]
pub type BmreqsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 2 - Bus Master Enable"]
    #[inline(always)]
    pub fn bmmen(&self) -> BmmenR {
        BmmenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Peripheral Channel Maximum Payload Size"]
    #[inline(always)]
    pub fn perplsize(&self) -> PerplsizeR {
        PerplsizeR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Peripheral Channel Maximum Read Request Size"]
    #[inline(always)]
    pub fn bmreqsize(&self) -> BmreqsizeR {
        BmreqsizeR::new(((self.bits >> 13) & 7) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERCFG")
            .field("bmmen", &self.bmmen())
            .field("perplsize", &self.perplsize())
            .field("bmreqsize", &self.bmreqsize())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Bus Master Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bmmen(&mut self) -> BmmenW<PercfgSpec> {
        BmmenW::new(self, 2)
    }
    #[doc = "Bits 10:12 - Peripheral Channel Maximum Payload Size"]
    #[inline(always)]
    #[must_use]
    pub fn perplsize(&mut self) -> PerplsizeW<PercfgSpec> {
        PerplsizeW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Peripheral Channel Maximum Read Request Size"]
    #[inline(always)]
    #[must_use]
    pub fn bmreqsize(&mut self) -> BmreqsizeW<PercfgSpec> {
        BmreqsizeW::new(self, 13)
    }
}
#[doc = "Peripheral Channel Configuration Register (PERCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`percfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`percfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PercfgSpec;
impl crate::RegisterSpec for PercfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`percfg::R`](R) reader structure"]
impl crate::Readable for PercfgSpec {}
#[doc = "`write(|w| ..)` method takes [`percfg::W`](W) writer structure"]
impl crate::Writable for PercfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERCFG to value 0"]
impl crate::Resettable for PercfgSpec {
    const RESET_VALUE: u32 = 0;
}
