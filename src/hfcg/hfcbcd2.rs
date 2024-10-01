#[doc = "Register `HFCBCD2` reader"]
pub type R = crate::R<Hfcbcd2Spec>;
#[doc = "Register `HFCBCD2` writer"]
pub type W = crate::W<Hfcbcd2Spec>;
#[doc = "Field `APB3DIV` reader - APB3 Clock Divider"]
pub type Apb3divR = crate::FieldReader;
#[doc = "Field `APB3DIV` writer - APB3 Clock Divider"]
pub type Apb3divW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `APB4DIV` reader - APB4 Clock Divider"]
pub type Apb4divR = crate::FieldReader;
#[doc = "Field `APB4DIV` writer - APB4 Clock Divider"]
pub type Apb4divW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - APB3 Clock Divider"]
    #[inline(always)]
    pub fn apb3div(&self) -> Apb3divR {
        Apb3divR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - APB4 Clock Divider"]
    #[inline(always)]
    pub fn apb4div(&self) -> Apb4divR {
        Apb4divR::new((self.bits >> 4) & 0x0f)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFCBCD2")
            .field("apb3div", &self.apb3div())
            .field("apb4div", &self.apb4div())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - APB3 Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn apb3div(&mut self) -> Apb3divW<Hfcbcd2Spec> {
        Apb3divW::new(self, 0)
    }
    #[doc = "Bits 4:7 - APB4 Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn apb4div(&mut self) -> Apb4divW<Hfcbcd2Spec> {
        Apb4divW::new(self, 4)
    }
}
#[doc = "HFCG Bus Clock Dividers 2 Register (HFCBCD2)\n\nYou can [`read`](crate::Reg::read) this register and get [`hfcbcd2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfcbcd2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hfcbcd2Spec;
impl crate::RegisterSpec for Hfcbcd2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hfcbcd2::R`](R) reader structure"]
impl crate::Readable for Hfcbcd2Spec {}
#[doc = "`write(|w| ..)` method takes [`hfcbcd2::W`](W) writer structure"]
impl crate::Writable for Hfcbcd2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HFCBCD2 to value 0"]
impl crate::Resettable for Hfcbcd2Spec {
    const RESET_VALUE: u8 = 0;
}
