#[doc = "Register `DBGCTRL` reader"]
pub type R = crate::R<DbgctrlSpec>;
#[doc = "Register `DBGCTRL` writer"]
pub type W = crate::W<DbgctrlSpec>;
#[doc = "Field `SEQ_WK_EN` reader - Wake-Up by Detection Sequence, Enable"]
pub type SeqWkEnR = crate::BitReader;
#[doc = "Field `SEQ_WK_EN` writer - Wake-Up by Detection Sequence, Enable"]
pub type SeqWkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Wake-Up by Detection Sequence, Enable"]
    #[inline(always)]
    pub fn seq_wk_en(&self) -> SeqWkEnR {
        SeqWkEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGCTRL")
            .field("seq_wk_en", &self.seq_wk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 4 - Wake-Up by Detection Sequence, Enable"]
    #[inline(always)]
    pub fn seq_wk_en(&mut self) -> SeqWkEnW<DbgctrlSpec> {
        SeqWkEnW::new(self, 4)
    }
}
#[doc = "Debug Control Register (DBGCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbgctrlSpec;
impl crate::RegisterSpec for DbgctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbgctrl::R`](R) reader structure"]
impl crate::Readable for DbgctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dbgctrl::W`](W) writer structure"]
impl crate::Writable for DbgctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DBGCTRL to value 0"]
impl crate::Resettable for DbgctrlSpec {
    const RESET_VALUE: u8 = 0;
}
