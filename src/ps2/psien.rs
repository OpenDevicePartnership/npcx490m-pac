#[doc = "Register `PSIEN` reader"]
pub type R = crate::R<PsienSpec>;
#[doc = "Register `PSIEN` writer"]
pub type W = crate::W<PsienSpec>;
#[doc = "Field `SOTIE` reader - Start Of Transaction Interrupt Enable"]
pub type SotieR = crate::BitReader;
#[doc = "Field `SOTIE` writer - Start Of Transaction Interrupt Enable"]
pub type SotieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOTIE` reader - End Of Transaction Interrupt Enable"]
pub type EotieR = crate::BitReader;
#[doc = "Field `EOTIE` writer - End Of Transaction Interrupt Enable"]
pub type EotieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2_WUE` reader - PS/2 Wake-Up Enable"]
pub type Ps2WueR = crate::BitReader;
#[doc = "Field `PS2_WUE` writer - PS/2 Wake-Up Enable"]
pub type Ps2WueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2_CLK_SEL` reader - PS/2 Interface Clock Select"]
pub type Ps2ClkSelR = crate::BitReader;
#[doc = "Field `PS2_CLK_SEL` writer - PS/2 Interface Clock Select"]
pub type Ps2ClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Of Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn sotie(&self) -> SotieR {
        SotieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End Of Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn eotie(&self) -> EotieR {
        EotieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - PS/2 Wake-Up Enable"]
    #[inline(always)]
    pub fn ps2_wue(&self) -> Ps2WueR {
        Ps2WueR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - PS/2 Interface Clock Select"]
    #[inline(always)]
    pub fn ps2_clk_sel(&self) -> Ps2ClkSelR {
        Ps2ClkSelR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSIEN")
            .field("sotie", &self.sotie())
            .field("eotie", &self.eotie())
            .field("ps2_wue", &self.ps2_wue())
            .field("ps2_clk_sel", &self.ps2_clk_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Start Of Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn sotie(&mut self) -> SotieW<PsienSpec> {
        SotieW::new(self, 0)
    }
    #[doc = "Bit 1 - End Of Transaction Interrupt Enable"]
    #[inline(always)]
    pub fn eotie(&mut self) -> EotieW<PsienSpec> {
        EotieW::new(self, 1)
    }
    #[doc = "Bit 4 - PS/2 Wake-Up Enable"]
    #[inline(always)]
    pub fn ps2_wue(&mut self) -> Ps2WueW<PsienSpec> {
        Ps2WueW::new(self, 4)
    }
    #[doc = "Bit 7 - PS/2 Interface Clock Select"]
    #[inline(always)]
    pub fn ps2_clk_sel(&mut self) -> Ps2ClkSelW<PsienSpec> {
        Ps2ClkSelW::new(self, 7)
    }
}
#[doc = "PS/2 Interrupt Enable Register (PSIEN)\n\nYou can [`read`](crate::Reg::read) this register and get [`psien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsienSpec;
impl crate::RegisterSpec for PsienSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`psien::R`](R) reader structure"]
impl crate::Readable for PsienSpec {}
#[doc = "`write(|w| ..)` method takes [`psien::W`](W) writer structure"]
impl crate::Writable for PsienSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PSIEN to value 0"]
impl crate::Resettable for PsienSpec {
    const RESET_VALUE: u8 = 0;
}
