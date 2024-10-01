#[doc = "Register `FLM_CQ0-30` reader"]
pub type R = crate::R<FlmCq030Spec>;
#[doc = "Register `FLM_CQ0-30` writer"]
pub type W = crate::W<FlmCq030Spec>;
#[doc = "Field `QMASK` reader - Qualifier Compare Mask"]
pub type QmaskR = crate::FieldReader;
#[doc = "Field `QMASK` writer - Qualifier Compare Mask"]
pub type QmaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `QVAL` reader - Qualifier Compare Value"]
pub type QvalR = crate::FieldReader;
#[doc = "Field `QVAL` writer - Qualifier Compare Value"]
pub type QvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `QBYTE` reader - Qualifier Compare Byte Index"]
pub type QbyteR = crate::FieldReader;
#[doc = "Field `QBYTE` writer - Qualifier Compare Byte Index"]
pub type QbyteW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `QPOL` reader - Qualifier Compare Polarity"]
pub type QpolR = crate::BitReader;
#[doc = "Field `QPOL` writer - Qualifier Compare Polarity"]
pub type QpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QEN` reader - Qualifier Enable"]
pub type QenR = crate::FieldReader;
#[doc = "Field `QEN` writer - Qualifier Enable"]
pub type QenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Qualifier Compare Mask"]
    #[inline(always)]
    pub fn qmask(&self) -> QmaskR {
        QmaskR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Qualifier Compare Value"]
    #[inline(always)]
    pub fn qval(&self) -> QvalR {
        QvalR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Qualifier Compare Byte Index"]
    #[inline(always)]
    pub fn qbyte(&self) -> QbyteR {
        QbyteR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Qualifier Compare Polarity"]
    #[inline(always)]
    pub fn qpol(&self) -> QpolR {
        QpolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Qualifier Enable"]
    #[inline(always)]
    pub fn qen(&self) -> QenR {
        QenR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_CQ0-30")
            .field("qmask", &self.qmask())
            .field("qval", &self.qval())
            .field("qbyte", &self.qbyte())
            .field("qpol", &self.qpol())
            .field("qen", &self.qen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Qualifier Compare Mask"]
    #[inline(always)]
    #[must_use]
    pub fn qmask(&mut self) -> QmaskW<FlmCq030Spec> {
        QmaskW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Qualifier Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn qval(&mut self) -> QvalW<FlmCq030Spec> {
        QvalW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Qualifier Compare Byte Index"]
    #[inline(always)]
    #[must_use]
    pub fn qbyte(&mut self) -> QbyteW<FlmCq030Spec> {
        QbyteW::new(self, 16)
    }
    #[doc = "Bit 20 - Qualifier Compare Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn qpol(&mut self) -> QpolW<FlmCq030Spec> {
        QpolW::new(self, 20)
    }
    #[doc = "Bits 24:31 - Qualifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn qen(&mut self) -> QenW<FlmCq030Spec> {
        QenW::new(self, 24)
    }
}
#[doc = "FLM Command Qualifier Register 0-3 (FLM_CQ0-3)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cq030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flm_cq030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCq030Spec;
impl crate::RegisterSpec for FlmCq030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cq030::R`](R) reader structure"]
impl crate::Readable for FlmCq030Spec {}
#[doc = "`write(|w| ..)` method takes [`flm_cq030::W`](W) writer structure"]
impl crate::Writable for FlmCq030Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLM_CQ0-30 to value 0"]
impl crate::Resettable for FlmCq030Spec {
    const RESET_VALUE: u32 = 0;
}
