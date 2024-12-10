#[doc = "Register `DBGFRZEN5` reader"]
pub type R = crate::R<Dbgfrzen5Spec>;
#[doc = "Register `DBGFRZEN5` writer"]
pub type W = crate::W<Dbgfrzen5Spec>;
#[doc = "Field `I3C1FEN` reader - I3CI1 Freeze Enable"]
pub type I3c1fenR = crate::BitReader;
#[doc = "Field `I3C1FEN` writer - I3CI1 Freeze Enable"]
pub type I3c1fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C2FEN` reader - I3CI2 Freeze Enable"]
pub type I3c2fenR = crate::BitReader;
#[doc = "Field `I3C2FEN` writer - I3CI2 Freeze Enable"]
pub type I3c2fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I3C3FEN` reader - I3CI3 Freeze Enable"]
pub type I3c3fenR = crate::BitReader;
#[doc = "Field `I3C3FEN` writer - I3CI3 Freeze Enable"]
pub type I3c3fenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I3CI1 Freeze Enable"]
    #[inline(always)]
    pub fn i3c1fen(&self) -> I3c1fenR {
        I3c1fenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I3CI2 Freeze Enable"]
    #[inline(always)]
    pub fn i3c2fen(&self) -> I3c2fenR {
        I3c2fenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I3CI3 Freeze Enable"]
    #[inline(always)]
    pub fn i3c3fen(&self) -> I3c3fenR {
        I3c3fenR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBGFRZEN5")
            .field("i3c1fen", &self.i3c1fen())
            .field("i3c2fen", &self.i3c2fen())
            .field("i3c3fen", &self.i3c3fen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - I3CI1 Freeze Enable"]
    #[inline(always)]
    pub fn i3c1fen(&mut self) -> I3c1fenW<Dbgfrzen5Spec> {
        I3c1fenW::new(self, 0)
    }
    #[doc = "Bit 1 - I3CI2 Freeze Enable"]
    #[inline(always)]
    pub fn i3c2fen(&mut self) -> I3c2fenW<Dbgfrzen5Spec> {
        I3c2fenW::new(self, 1)
    }
    #[doc = "Bit 2 - I3CI3 Freeze Enable"]
    #[inline(always)]
    pub fn i3c3fen(&mut self) -> I3c3fenW<Dbgfrzen5Spec> {
        I3c3fenW::new(self, 2)
    }
}
#[doc = "Debug Freeze Enable 5 Register (DBGFRZEN5)\n\nYou can [`read`](crate::Reg::read) this register and get [`dbgfrzen5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgfrzen5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dbgfrzen5Spec;
impl crate::RegisterSpec for Dbgfrzen5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dbgfrzen5::R`](R) reader structure"]
impl crate::Readable for Dbgfrzen5Spec {}
#[doc = "`write(|w| ..)` method takes [`dbgfrzen5::W`](W) writer structure"]
impl crate::Writable for Dbgfrzen5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DBGFRZEN5 to value 0"]
impl crate::Resettable for Dbgfrzen5Spec {
    const RESET_VALUE: u8 = 0;
}
