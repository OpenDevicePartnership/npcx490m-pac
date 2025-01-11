#[doc = "Register `WKINENn%s` reader"]
pub type R = crate::R<WkinennSpec>;
#[doc = "Register `WKINENn%s` writer"]
pub type W = crate::W<WkinennSpec>;
#[doc = "The associated input of the input group is enabled for MIWU event detection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WakeupInputEnable {
    #[doc = "0: Disable wake-up input"]
    Disabled = 0,
    #[doc = "1: Enable wake-up input"]
    Enabled = 1,
}
impl From<WakeupInputEnable> for bool {
    #[inline(always)]
    fn from(variant: WakeupInputEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT(0-7)` reader - The associated input of the input group is enabled for MIWU event detection"]
pub type InputR = crate::BitReader<WakeupInputEnable>;
impl InputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WakeupInputEnable {
        match self.bits {
            false => WakeupInputEnable::Disabled,
            true => WakeupInputEnable::Enabled,
        }
    }
    #[doc = "Disable wake-up input"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WakeupInputEnable::Disabled
    }
    #[doc = "Enable wake-up input"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WakeupInputEnable::Enabled
    }
}
#[doc = "Field `INPUT(0-7)` writer - The associated input of the input group is enabled for MIWU event detection"]
pub type InputW<'a, REG> = crate::BitWriter<'a, REG, WakeupInputEnable>;
impl<'a, REG> InputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable wake-up input"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupInputEnable::Disabled)
    }
    #[doc = "Enable wake-up input"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WakeupInputEnable::Enabled)
    }
}
impl R {
    #[doc = "The associated input of the input group is enabled for MIWU event detection"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&self, n: u8) -> InputR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input_iter(&self) -> impl Iterator<Item = InputR> + '_ {
        (0..8).map(move |n| InputR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input0(&self) -> InputR {
        InputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input1(&self) -> InputR {
        InputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input2(&self) -> InputR {
        InputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input3(&self) -> InputR {
        InputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input4(&self) -> InputR {
        InputR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input5(&self) -> InputR {
        InputR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input6(&self) -> InputR {
        InputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input7(&self) -> InputR {
        InputR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKINENn")
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
    #[doc = "The associated input of the input group is enabled for MIWU event detection"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&mut self, n: u8) -> InputW<WkinennSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputW::new(self, n)
    }
    #[doc = "Bit 0 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input0(&mut self) -> InputW<WkinennSpec> {
        InputW::new(self, 0)
    }
    #[doc = "Bit 1 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input1(&mut self) -> InputW<WkinennSpec> {
        InputW::new(self, 1)
    }
    #[doc = "Bit 2 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input2(&mut self) -> InputW<WkinennSpec> {
        InputW::new(self, 2)
    }
    #[doc = "Bit 3 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input3(&mut self) -> InputW<WkinennSpec> {
        InputW::new(self, 3)
    }
    #[doc = "Bit 4 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input4(&mut self) -> InputW<WkinennSpec> {
        InputW::new(self, 4)
    }
    #[doc = "Bit 5 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input5(&mut self) -> InputW<WkinennSpec> {
        InputW::new(self, 5)
    }
    #[doc = "Bit 6 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input6(&mut self) -> InputW<WkinennSpec> {
        InputW::new(self, 6)
    }
    #[doc = "Bit 7 - The associated input of the input group is enabled for MIWU event detection"]
    #[inline(always)]
    pub fn input7(&mut self) -> InputW<WkinennSpec> {
        InputW::new(self, 7)
    }
}
#[doc = "Wake-Up Input Enable nx Register (WKINENnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkinenn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkinenn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkinennSpec;
impl crate::RegisterSpec for WkinennSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkinenn::R`](R) reader structure"]
impl crate::Readable for WkinennSpec {}
#[doc = "`write(|w| ..)` method takes [`wkinenn::W`](W) writer structure"]
impl crate::Writable for WkinennSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKINENn%s to value 0"]
impl crate::Resettable for WkinennSpec {
    const RESET_VALUE: u8 = 0;
}
