#[doc = "Register `SPIP_STAT` reader"]
pub type R = crate::R<SpipStatSpec>;
#[doc = "Register `SPIP_STAT` writer"]
pub type W = crate::W<SpipStatSpec>;
#[doc = "Field `BSY` reader - Shift Register Busy"]
pub type BsyR = crate::BitReader;
#[doc = "Field `BSY` writer - Shift Register Busy"]
pub type BsyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBF` reader - Read Buffer Full"]
pub type RbfR = crate::BitReader;
#[doc = "Field `RBF` writer - Read Buffer Full"]
pub type RbfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shift Register Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read Buffer Full"]
    #[inline(always)]
    pub fn rbf(&self) -> RbfR {
        RbfR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPIP_STAT")
            .field("bsy", &self.bsy())
            .field("rbf", &self.rbf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Shift Register Busy"]
    #[inline(always)]
    pub fn bsy(&mut self) -> BsyW<SpipStatSpec> {
        BsyW::new(self, 0)
    }
    #[doc = "Bit 1 - Read Buffer Full"]
    #[inline(always)]
    pub fn rbf(&mut self) -> RbfW<SpipStatSpec> {
        RbfW::new(self, 1)
    }
}
#[doc = "SPIP Status Register (SPIP_STAT)\n\nYou can [`read`](crate::Reg::read) this register and get [`spip_stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spip_stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpipStatSpec;
impl crate::RegisterSpec for SpipStatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spip_stat::R`](R) reader structure"]
impl crate::Readable for SpipStatSpec {}
#[doc = "`write(|w| ..)` method takes [`spip_stat::W`](W) writer structure"]
impl crate::Writable for SpipStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SPIP_STAT to value 0"]
impl crate::Resettable for SpipStatSpec {
    const RESET_VALUE: u8 = 0;
}
