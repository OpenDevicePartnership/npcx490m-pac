#[doc = "Register `SHICFG6` reader"]
pub type R = crate::R<Shicfg6Spec>;
#[doc = "Register `SHICFG6` writer"]
pub type W = crate::W<Shicfg6Spec>;
#[doc = "Field `EBUFMD` reader - Enhanced Buffer Mode"]
pub type EbufmdR = crate::BitReader;
#[doc = "Field `EBUFMD` writer - Enhanced Buffer Mode"]
pub type EbufmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OBUF_SL` reader - OBUF Select"]
pub type ObufSlR = crate::BitReader;
#[doc = "Field `OBUF_SL` writer - OBUF Select"]
pub type ObufSlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enhanced Buffer Mode"]
    #[inline(always)]
    pub fn ebufmd(&self) -> EbufmdR {
        EbufmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OBUF Select"]
    #[inline(always)]
    pub fn obuf_sl(&self) -> ObufSlR {
        ObufSlR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHICFG6")
            .field("ebufmd", &self.ebufmd())
            .field("obuf_sl", &self.obuf_sl())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enhanced Buffer Mode"]
    #[inline(always)]
    pub fn ebufmd(&mut self) -> EbufmdW<Shicfg6Spec> {
        EbufmdW::new(self, 0)
    }
    #[doc = "Bit 1 - OBUF Select"]
    #[inline(always)]
    pub fn obuf_sl(&mut self) -> ObufSlW<Shicfg6Spec> {
        ObufSlW::new(self, 1)
    }
}
#[doc = "SHI Configuration 6\n\nYou can [`read`](crate::Reg::read) this register and get [`shicfg6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shicfg6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shicfg6Spec;
impl crate::RegisterSpec for Shicfg6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`shicfg6::R`](R) reader structure"]
impl crate::Readable for Shicfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`shicfg6::W`](W) writer structure"]
impl crate::Writable for Shicfg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHICFG6 to value 0"]
impl crate::Resettable for Shicfg6Spec {
    const RESET_VALUE: u8 = 0;
}
