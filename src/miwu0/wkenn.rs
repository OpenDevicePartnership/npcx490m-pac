#[doc = "Register `WKENn%s` reader"]
pub type R = crate::R<WkennSpec>;
#[doc = "Register `WKENn%s` writer"]
pub type W = crate::W<WkennSpec>;
#[doc = "If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupEnable {
    #[doc = "0: Disable wake-up"]
    Disabled = 0,
    #[doc = "1: Enable wake-up and interrupt request"]
    Enabled = 1,
}
impl From<WakeupEnable> for bool {
    #[inline(always)]
    fn from(variant: WakeupEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT(0-7)` reader - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
pub type InputR = crate::BitReader<WakeupEnable>;
impl InputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupEnable {
        match self.bits {
            false => WakeupEnable::Disabled,
            true => WakeupEnable::Enabled,
        }
    }
    #[doc = "Disable wake-up"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WakeupEnable::Disabled
    }
    #[doc = "Enable wake-up and interrupt request"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WakeupEnable::Enabled
    }
}
#[doc = "Field `INPUT(0-7)` writer - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
pub type InputW<'a, REG> = crate::BitWriter<'a, REG, WakeupEnable>;
impl<'a, REG> InputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable wake-up"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupEnable::Disabled)
    }
    #[doc = "Enable wake-up and interrupt request"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupEnable::Enabled)
    }
}
impl R {
    #[doc = "If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&self, n: u8) -> InputR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input_iter(&self) -> impl Iterator<Item = InputR> + '_ {
        (0..8).map(move |n| InputR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input0(&self) -> InputR {
        InputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input1(&self) -> InputR {
        InputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input2(&self) -> InputR {
        InputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input3(&self) -> InputR {
        InputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input4(&self) -> InputR {
        InputR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input5(&self) -> InputR {
        InputR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input6(&self) -> InputR {
        InputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input7(&self) -> InputR {
        InputR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKENn")
            .field("input0", &self.input0())
            .field("input1", &self.input1())
            .field("input2", &self.input2())
            .field("input3", &self.input3())
            .field("input4", &self.input4())
            .field("input5", &self.input5())
            .field("input6", &self.input6())
            .field("input7", &self.input7())
            .finish()
    }
}
impl W {
    #[doc = "If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&mut self, n: u8) -> InputW<WkennSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputW::new(self, n)
    }
    #[doc = "Bit 0 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input0(&mut self) -> InputW<WkennSpec> {
        InputW::new(self, 0)
    }
    #[doc = "Bit 1 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input1(&mut self) -> InputW<WkennSpec> {
        InputW::new(self, 1)
    }
    #[doc = "Bit 2 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input2(&mut self) -> InputW<WkennSpec> {
        InputW::new(self, 2)
    }
    #[doc = "Bit 3 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input3(&mut self) -> InputW<WkennSpec> {
        InputW::new(self, 3)
    }
    #[doc = "Bit 4 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input4(&mut self) -> InputW<WkennSpec> {
        InputW::new(self, 4)
    }
    #[doc = "Bit 5 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input5(&mut self) -> InputW<WkennSpec> {
        InputW::new(self, 5)
    }
    #[doc = "Bit 6 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input6(&mut self) -> InputW<WkennSpec> {
        InputW::new(self, 6)
    }
    #[doc = "Bit 7 - If enabled, a valid trigger event on the associated input of the input group generates a wake-up signal or interrupt request"]
    #[inline(always)]
    pub fn input7(&mut self) -> InputW<WkennSpec> {
        InputW::new(self, 7)
    }
}
#[doc = "Enable nx Register (WKENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkenn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkenn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkennSpec;
impl crate::RegisterSpec for WkennSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkenn::R`](R) reader structure"]
impl crate::Readable for WkennSpec {}
#[doc = "`write(|w| ..)` method takes [`wkenn::W`](W) writer structure"]
impl crate::Writable for WkennSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKENn%s to value 0"]
impl crate::Resettable for WkennSpec {
    const RESET_VALUE: u8 = 0;
}
