#[doc = "Register `HFCBCD1` reader"]
pub type R = crate::R<Hfcbcd1Spec>;
#[doc = "Register `HFCBCD1` writer"]
pub type W = crate::W<Hfcbcd1Spec>;
#[doc = "Field `APB1DIV` reader - APB1 Clock Divider"]
pub type Apb1divR = crate::FieldReader;
#[doc = "Field `APB1DIV` writer - APB1 Clock Divider"]
pub type Apb1divW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `APB2DIV` reader - APB2 Clock Divider"]
pub type Apb2divR = crate::FieldReader;
#[doc = "Field `APB2DIV` writer - APB2 Clock Divider"]
pub type Apb2divW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - APB1 Clock Divider"]
    #[inline(always)]
    pub fn apb1div(&self) -> Apb1divR {
        Apb1divR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - APB2 Clock Divider"]
    #[inline(always)]
    pub fn apb2div(&self) -> Apb2divR {
        Apb2divR::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCBCD1")
            .field("apb1div", &self.apb1div())
            .field("apb2div", &self.apb2div())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - APB1 Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn apb1div(&mut self) -> Apb1divW<Hfcbcd1Spec> {
        Apb1divW::new(self, 0)
    }
    #[doc = "Bits 4:7 - APB2 Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn apb2div(&mut self) -> Apb2divW<Hfcbcd1Spec> {
        Apb2divW::new(self, 4)
    }
}
#[doc = "HFCG Bus Clock Dividers 1 Register (HFCBCD1)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcbcd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcbcd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfcbcd1Spec;
impl crate::RegisterSpec for Hfcbcd1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcbcd1::R`](R) reader structure"]
impl crate::Readable for Hfcbcd1Spec {}
#[doc = "`write(|w| ..)` method takes [`hfcbcd1::W`](W) writer structure"]
impl crate::Writable for Hfcbcd1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCBCD1 to value 0"]
impl crate::Resettable for Hfcbcd1Spec {
    const RESET_VALUE: u8 = 0;
}
