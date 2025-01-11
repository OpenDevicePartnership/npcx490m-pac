#[doc = "Register `WKAEDGn%s` reader"]
pub type R = crate::R<WkaedgnSpec>;
#[doc = "Register `WKAEDGn%s` writer"]
pub type W = crate::W<WkaedgnSpec>;
#[doc = "Selects \"any edge\" for input\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnyEdge {
    #[doc = "0: Edge detection is defined by the respective bit in WKEDGnx"]
    Edge = 0,
    #[doc = "1: Any transition is detected"]
    Any = 1,
}
impl From<AnyEdge> for bool {
    #[inline(always)]
    fn from(variant: AnyEdge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT(0-7)` reader - Selects \"any edge\" for input"]
pub type InputR = crate::BitReader<AnyEdge>;
impl InputR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AnyEdge {
        match self.bits {
            false => AnyEdge::Edge,
            true => AnyEdge::Any,
        }
    }
    #[doc = "Edge detection is defined by the respective bit in WKEDGnx"]
    #[inline(always)]
    pub fn is_edge(&self) -> bool {
        *self == AnyEdge::Edge
    }
    #[doc = "Any transition is detected"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        *self == AnyEdge::Any
    }
}
#[doc = "Field `INPUT(0-7)` writer - Selects \"any edge\" for input"]
pub type InputW<'a, REG> = crate::BitWriter<'a, REG, AnyEdge>;
impl<'a, REG> InputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge detection is defined by the respective bit in WKEDGnx"]
    #[inline(always)]
    pub fn edge(self) -> &'a mut crate::W<REG> {
        self.variant(AnyEdge::Edge)
    }
    #[doc = "Any transition is detected"]
    #[inline(always)]
    pub fn any(self) -> &'a mut crate::W<REG> {
        self.variant(AnyEdge::Any)
    }
}
impl R {
    #[doc = "Selects \"any edge\" for input"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&self, n: u8) -> InputR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input_iter(&self) -> impl Iterator<Item = InputR> + '_ {
        (0..8).map(move |n| InputR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input0(&self) -> InputR {
        InputR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input1(&self) -> InputR {
        InputR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input2(&self) -> InputR {
        InputR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input3(&self) -> InputR {
        InputR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input4(&self) -> InputR {
        InputR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input5(&self) -> InputR {
        InputR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input6(&self) -> InputR {
        InputR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input7(&self) -> InputR {
        InputR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WKAEDGn")
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
    #[doc = "Selects \"any edge\" for input"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&mut self, n: u8) -> InputW<WkaedgnSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputW::new(self, n)
    }
    #[doc = "Bit 0 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input0(&mut self) -> InputW<WkaedgnSpec> {
        InputW::new(self, 0)
    }
    #[doc = "Bit 1 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input1(&mut self) -> InputW<WkaedgnSpec> {
        InputW::new(self, 1)
    }
    #[doc = "Bit 2 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input2(&mut self) -> InputW<WkaedgnSpec> {
        InputW::new(self, 2)
    }
    #[doc = "Bit 3 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input3(&mut self) -> InputW<WkaedgnSpec> {
        InputW::new(self, 3)
    }
    #[doc = "Bit 4 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input4(&mut self) -> InputW<WkaedgnSpec> {
        InputW::new(self, 4)
    }
    #[doc = "Bit 5 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input5(&mut self) -> InputW<WkaedgnSpec> {
        InputW::new(self, 5)
    }
    #[doc = "Bit 6 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input6(&mut self) -> InputW<WkaedgnSpec> {
        InputW::new(self, 6)
    }
    #[doc = "Bit 7 - Selects \"any edge\" for input"]
    #[inline(always)]
    pub fn input7(&mut self) -> InputW<WkaedgnSpec> {
        InputW::new(self, 7)
    }
}
#[doc = "Any Edge Detection nx Register (WKAEDGnx)\n\nYou can [`read`](crate::Reg::read) this register and get [`wkaedgn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkaedgn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkaedgnSpec;
impl crate::RegisterSpec for WkaedgnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wkaedgn::R`](R) reader structure"]
impl crate::Readable for WkaedgnSpec {}
#[doc = "`write(|w| ..)` method takes [`wkaedgn::W`](W) writer structure"]
impl crate::Writable for WkaedgnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKAEDGn%s to value 0"]
impl crate::Resettable for WkaedgnSpec {
    const RESET_VALUE: u8 = 0;
}
