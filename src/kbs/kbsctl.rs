#[doc = "Register `KBSCTL` reader"]
pub type R = crate::R<KbsctlSpec>;
#[doc = "Register `KBSCTL` writer"]
pub type W = crate::W<KbsctlSpec>;
#[doc = "Field `START` writer - Automatic Keyboard Scan Start"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KBSMODE` reader - Keyboard Scan Mode"]
pub type KbsmodeR = crate::BitReader;
#[doc = "Field `KBSMODE` writer - Keyboard Scan Mode"]
pub type KbsmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KBSIEN` reader - Keyboard Scan Interrupt Enable"]
pub type KbsienR = crate::BitReader;
#[doc = "Field `KBSIEN` writer - Keyboard Scan Interrupt Enable"]
pub type KbsienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KBSINC` reader - Keyboard Scan Index Increment"]
pub type KbsincR = crate::BitReader;
#[doc = "Field `KBSINC` writer - Keyboard Scan Index Increment"]
pub type KbsincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KBHDRV` reader - Keyboard Outputs High Drive"]
pub type KbhdrvR = crate::FieldReader;
#[doc = "Field `KBHDRV` writer - Keyboard Outputs High Drive"]
pub type KbhdrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 1 - Keyboard Scan Mode"]
    #[inline(always)]
    pub fn kbsmode(&self) -> KbsmodeR {
        KbsmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Keyboard Scan Interrupt Enable"]
    #[inline(always)]
    pub fn kbsien(&self) -> KbsienR {
        KbsienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Keyboard Scan Index Increment"]
    #[inline(always)]
    pub fn kbsinc(&self) -> KbsincR {
        KbsincR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Keyboard Outputs High Drive"]
    #[inline(always)]
    pub fn kbhdrv(&self) -> KbhdrvR {
        KbhdrvR::new((self.bits >> 6) & 3)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KBSCTL")
            .field("kbsmode", &self.kbsmode())
            .field("kbsien", &self.kbsien())
            .field("kbsinc", &self.kbsinc())
            .field("kbhdrv", &self.kbhdrv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Automatic Keyboard Scan Start"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<KbsctlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Keyboard Scan Mode"]
    #[inline(always)]
    pub fn kbsmode(&mut self) -> KbsmodeW<KbsctlSpec> {
        KbsmodeW::new(self, 1)
    }
    #[doc = "Bit 2 - Keyboard Scan Interrupt Enable"]
    #[inline(always)]
    pub fn kbsien(&mut self) -> KbsienW<KbsctlSpec> {
        KbsienW::new(self, 2)
    }
    #[doc = "Bit 3 - Keyboard Scan Index Increment"]
    #[inline(always)]
    pub fn kbsinc(&mut self) -> KbsincW<KbsctlSpec> {
        KbsincW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Keyboard Outputs High Drive"]
    #[inline(always)]
    pub fn kbhdrv(&mut self) -> KbhdrvW<KbsctlSpec> {
        KbhdrvW::new(self, 6)
    }
}
#[doc = "Keyboard Scan Control Register (KBSCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`kbsctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`kbsctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KbsctlSpec;
impl crate::RegisterSpec for KbsctlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`kbsctl::R`](R) reader structure"]
impl crate::Readable for KbsctlSpec {}
#[doc = "`write(|w| ..)` method takes [`kbsctl::W`](W) writer structure"]
impl crate::Writable for KbsctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets KBSCTL to value 0"]
impl crate::Resettable for KbsctlSpec {
    const RESET_VALUE: u8 = 0;
}
