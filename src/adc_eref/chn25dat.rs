#[doc = "Register `CHN25DAT` reader"]
pub type R = crate::R<Chn25datSpec>;
#[doc = "Register `CHN25DAT` writer"]
pub type W = crate::W<Chn25datSpec>;
#[doc = "Field `CHDAT` reader - Channel Data"]
pub type ChdatR = crate::FieldReader<u16>;
#[doc = "Field `CHDAT` writer - Channel Data"]
pub type ChdatW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `NEW` reader - New Conversion Result"]
pub type NewR = crate::BitReader;
#[doc = "Field `NEW` writer - New Conversion Result"]
pub type NewW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Channel Data"]
    #[inline(always)]
    pub fn chdat(&self) -> ChdatR {
        ChdatR::new(self.bits & 0x03ff)
    }
    #[doc = "Bit 15 - New Conversion Result"]
    #[inline(always)]
    pub fn new(&self) -> NewR {
        NewR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHN25DAT")
            .field("chdat", &self.chdat())
            .field("new", &self.new())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Channel Data"]
    #[inline(always)]
    pub fn chdat(&mut self) -> ChdatW<Chn25datSpec> {
        ChdatW::new(self, 0)
    }
    #[doc = "Bit 15 - New Conversion Result"]
    #[inline(always)]
    pub fn new(&mut self) -> NewW<Chn25datSpec> {
        NewW::new(self, 15)
    }
}
#[doc = "Channel n Data Buffer Register (CHNnDAT{, n is 0-25,29-31)})\n\nYou can [`read`](crate::Reg::read) this register and get [`chn25dat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chn25dat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chn25datSpec;
impl crate::RegisterSpec for Chn25datSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chn25dat::R`](R) reader structure"]
impl crate::Readable for Chn25datSpec {}
#[doc = "`write(|w| ..)` method takes [`chn25dat::W`](W) writer structure"]
impl crate::Writable for Chn25datSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CHN25DAT to value 0"]
impl crate::Resettable for Chn25datSpec {
    const RESET_VALUE: u16 = 0;
}
