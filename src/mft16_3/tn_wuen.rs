#[doc = "Register `TnWUEN` reader"]
pub type R = crate::R<TnWuenSpec>;
#[doc = "Register `TnWUEN` writer"]
pub type W = crate::W<TnWuenSpec>;
#[doc = "Field `TAWEN` reader - Timer Wake-Up A Enable"]
pub type TawenR = crate::BitReader;
#[doc = "Field `TAWEN` writer - Timer Wake-Up A Enable"]
pub type TawenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBWEN` reader - Timer Wake-Up B Enable"]
pub type TbwenR = crate::BitReader;
#[doc = "Field `TBWEN` writer - Timer Wake-Up B Enable"]
pub type TbwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCWEN` reader - Timer Wake-Up C Enable"]
pub type TcwenR = crate::BitReader;
#[doc = "Field `TCWEN` writer - Timer Wake-Up C Enable"]
pub type TcwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDWEN` reader - Timer Wake-Up D Enable"]
pub type TdwenR = crate::BitReader;
#[doc = "Field `TDWEN` writer - Timer Wake-Up D Enable"]
pub type TdwenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEWEN` reader - Timer Wake-Up E Enable"]
pub type TewenR = crate::BitReader;
#[doc = "Field `TEWEN` writer - Timer Wake-Up E Enable"]
pub type TewenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFWEN` reader - Timer Wake-Up F Enable"]
pub type TfwenR = crate::BitReader;
#[doc = "Field `TFWEN` writer - Timer Wake-Up F Enable"]
pub type TfwenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timer Wake-Up A Enable"]
    #[inline(always)]
    pub fn tawen(&self) -> TawenR {
        TawenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Wake-Up B Enable"]
    #[inline(always)]
    pub fn tbwen(&self) -> TbwenR {
        TbwenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer Wake-Up C Enable"]
    #[inline(always)]
    pub fn tcwen(&self) -> TcwenR {
        TcwenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer Wake-Up D Enable"]
    #[inline(always)]
    pub fn tdwen(&self) -> TdwenR {
        TdwenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer Wake-Up E Enable"]
    #[inline(always)]
    pub fn tewen(&self) -> TewenR {
        TewenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer Wake-Up F Enable"]
    #[inline(always)]
    pub fn tfwen(&self) -> TfwenR {
        TfwenR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TnWUEN")
            .field("tawen", &self.tawen())
            .field("tbwen", &self.tbwen())
            .field("tcwen", &self.tcwen())
            .field("tdwen", &self.tdwen())
            .field("tewen", &self.tewen())
            .field("tfwen", &self.tfwen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timer Wake-Up A Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tawen(&mut self) -> TawenW<TnWuenSpec> {
        TawenW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Wake-Up B Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tbwen(&mut self) -> TbwenW<TnWuenSpec> {
        TbwenW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer Wake-Up C Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcwen(&mut self) -> TcwenW<TnWuenSpec> {
        TcwenW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer Wake-Up D Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdwen(&mut self) -> TdwenW<TnWuenSpec> {
        TdwenW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer Wake-Up E Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tewen(&mut self) -> TewenW<TnWuenSpec> {
        TewenW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer Wake-Up F Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfwen(&mut self) -> TfwenW<TnWuenSpec> {
        TfwenW::new(self, 5)
    }
}
#[doc = "Timer Wake-Up Enable Register (TnWUEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`tn_wuen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tn_wuen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TnWuenSpec;
impl crate::RegisterSpec for TnWuenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tn_wuen::R`](R) reader structure"]
impl crate::Readable for TnWuenSpec {}
#[doc = "`write(|w| ..)` method takes [`tn_wuen::W`](W) writer structure"]
impl crate::Writable for TnWuenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TnWUEN to value 0"]
impl crate::Resettable for TnWuenSpec {
    const RESET_VALUE: u8 = 0;
}
