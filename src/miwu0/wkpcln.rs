#[doc = "Register `WKPCLn%s` writer"]
pub type W = crate::W<WkpclnSpec>;
#[doc = "Clear a pending flag of an input event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClearEvent {
    #[doc = "0: Ignored (the flag is not changed)"]
    Ignored = 0,
    #[doc = "1: Clear the associated pending flag"]
    Clear = 1,
}
impl From<ClearEvent> for bool {
    #[inline(always)]
    fn from(variant: ClearEvent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INPUT(0-7)` writer - Clear a pending flag of an input event"]
pub type InputW<'a, REG> = crate::BitWriter<'a, REG, ClearEvent>;
impl<'a, REG> InputW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ignored (the flag is not changed)"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut crate::W<REG> {
        self.variant(ClearEvent::Ignored)
    }
    #[doc = "Clear the associated pending flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ClearEvent::Clear)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for crate::generic::Reg<WkpclnSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Clear a pending flag of an input event"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `INPUT0` field.</div>"]
    #[inline(always)]
    pub fn input(&mut self, n: u8) -> InputW<WkpclnSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        InputW::new(self, n)
    }
    #[doc = "Bit 0 - Clear a pending flag of an input event"]
    #[inline(always)]
    pub fn input0(&mut self) -> InputW<WkpclnSpec> {
        InputW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear a pending flag of an input event"]
    #[inline(always)]
    pub fn input1(&mut self) -> InputW<WkpclnSpec> {
        InputW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear a pending flag of an input event"]
    #[inline(always)]
    pub fn input2(&mut self) -> InputW<WkpclnSpec> {
        InputW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear a pending flag of an input event"]
    #[inline(always)]
    pub fn input3(&mut self) -> InputW<WkpclnSpec> {
        InputW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear a pending flag of an input event"]
    #[inline(always)]
    pub fn input4(&mut self) -> InputW<WkpclnSpec> {
        InputW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear a pending flag of an input event"]
    #[inline(always)]
    pub fn input5(&mut self) -> InputW<WkpclnSpec> {
        InputW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear a pending flag of an input event"]
    #[inline(always)]
    pub fn input6(&mut self) -> InputW<WkpclnSpec> {
        InputW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear a pending flag of an input event"]
    #[inline(always)]
    pub fn input7(&mut self) -> InputW<WkpclnSpec> {
        InputW::new(self, 7)
    }
}
#[doc = "Pending Clear nx Register (WKPCLnx)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkpcln::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkpclnSpec;
impl crate::RegisterSpec for WkpclnSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`wkpcln::W`](W) writer structure"]
impl crate::Writable for WkpclnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WKPCLn%s to value 0"]
impl crate::Resettable for WkpclnSpec {
    const RESET_VALUE: u8 = 0;
}
