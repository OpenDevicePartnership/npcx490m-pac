#[doc = "Register `WKMODn%s` reader"]
pub type R = crate::R<WkmodnSpec>;
#[doc = "Register `WKMODn%s` writer"]
pub type W = crate::W<WkmodnSpec>;
#[doc = "Selects the detection mode for input\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMode {
    #[doc = "0: Edge detection"]
    Edge = 0,
    #[doc = "1: Level detection"]
    Level = 1,
}
impl From<InputMode> for bool {
    #[inline(always)]
    fn from(variant: InputMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT(0-7)` reader - Selects the detection mode for input"]
pub type InputR = crate::BitReader<InputMode>;
impl InputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InputMode {
        match self.bits {
            false => InputMode::Edge,
            true => InputMode::Level,
        }
    }
    #[doc = "Edge detection"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == InputMode::Edge
    }
    #[doc = "Level detection"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == InputMode::Level
    }
}
#[doc = "Field `INPUT(0-7)` writer - Selects the detection mode for input"]
pub type InputW<'a, REG> = crate::BitWriter<'a, REG, InputMode>;
impl<'a, REG> InputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge detection"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(InputMode::Edge)
    }
    #[doc = "Level detection"]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(InputMode::Level)
    }
}
impl R {
    #[doc = "Selects the detection mode for input"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&self, n: u8) -> InputR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Selects the detection mode for input"]
    #[inline(always)]
    pub fn input_iter(&self) -> impl Iterator<Item = InputR> + '_ {
        (0..8).map(move |n| InputR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input0(&self) -> InputR {
        InputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input1(&self) -> InputR {
        InputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input2(&self) -> InputR {
        InputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input3(&self) -> InputR {
        InputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input4(&self) -> InputR {
        InputR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input5(&self) -> InputR {
        InputR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input6(&self) -> InputR {
        InputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input7(&self) -> InputR {
        InputR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKMODn")
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
    #[doc = "Selects the detection mode for input"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&mut self, n: u8) -> InputW<WkmodnSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputW::new(self, n)
    }
    #[doc = "Bit 0 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input0(&mut self) -> InputW<WkmodnSpec> {
        InputW::new(self, 0)
    }
    #[doc = "Bit 1 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input1(&mut self) -> InputW<WkmodnSpec> {
        InputW::new(self, 1)
    }
    #[doc = "Bit 2 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input2(&mut self) -> InputW<WkmodnSpec> {
        InputW::new(self, 2)
    }
    #[doc = "Bit 3 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input3(&mut self) -> InputW<WkmodnSpec> {
        InputW::new(self, 3)
    }
    #[doc = "Bit 4 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input4(&mut self) -> InputW<WkmodnSpec> {
        InputW::new(self, 4)
    }
    #[doc = "Bit 5 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input5(&mut self) -> InputW<WkmodnSpec> {
        InputW::new(self, 5)
    }
    #[doc = "Bit 6 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input6(&mut self) -> InputW<WkmodnSpec> {
        InputW::new(self, 6)
    }
    #[doc = "Bit 7 - Selects the detection mode for input"]
    #[inline(always)]
    pub fn input7(&mut self) -> InputW<WkmodnSpec> {
        InputW::new(self, 7)
    }
}
#[doc = "Wake-Up Detection Mode nx Register (WKMODnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkmodn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkmodn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkmodnSpec;
impl crate::RegisterSpec for WkmodnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkmodn::R`](R) reader structure"]
impl crate::Readable for WkmodnSpec {}
#[doc = "`write(|w| ..)` method takes [`wkmodn::W`](W) writer structure"]
impl crate::Writable for WkmodnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKMODn%s to value 0"]
impl crate::Resettable for WkmodnSpec {
    const RESET_VALUE: u8 = 0;
}
