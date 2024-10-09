#[doc = "Register `FLM_CMBEV` reader"]
pub type R = crate::R<FlmCmbevSpec>;
#[doc = "Field `CMBEV(0-31)` reader - CMB Event %s"]
pub type CmbevR = crate::BitReader;
impl R {
    #[doc = "CMB Event (0-31)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CMBEV0` field.</div>"]
    #[inline(always)]
    pub fn cmbev(&self, n: u8) -> CmbevR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        CmbevR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "CMB Event (0-31)"]
    #[inline(always)]
    pub fn cmbev_iter(&self) -> impl Iterator<Item = CmbevR> + '_ {
        (0..32).map(move |n| CmbevR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - CMB Event 0"]
    #[inline(always)]
    pub fn cmbev0(&self) -> CmbevR {
        CmbevR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMB Event 1"]
    #[inline(always)]
    pub fn cmbev1(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMB Event 2"]
    #[inline(always)]
    pub fn cmbev2(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMB Event 3"]
    #[inline(always)]
    pub fn cmbev3(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CMB Event 4"]
    #[inline(always)]
    pub fn cmbev4(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CMB Event 5"]
    #[inline(always)]
    pub fn cmbev5(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMB Event 6"]
    #[inline(always)]
    pub fn cmbev6(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMB Event 7"]
    #[inline(always)]
    pub fn cmbev7(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CMB Event 8"]
    #[inline(always)]
    pub fn cmbev8(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CMB Event 9"]
    #[inline(always)]
    pub fn cmbev9(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CMB Event 10"]
    #[inline(always)]
    pub fn cmbev10(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CMB Event 11"]
    #[inline(always)]
    pub fn cmbev11(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CMB Event 12"]
    #[inline(always)]
    pub fn cmbev12(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CMB Event 13"]
    #[inline(always)]
    pub fn cmbev13(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CMB Event 14"]
    #[inline(always)]
    pub fn cmbev14(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CMB Event 15"]
    #[inline(always)]
    pub fn cmbev15(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CMB Event 16"]
    #[inline(always)]
    pub fn cmbev16(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CMB Event 17"]
    #[inline(always)]
    pub fn cmbev17(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CMB Event 18"]
    #[inline(always)]
    pub fn cmbev18(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CMB Event 19"]
    #[inline(always)]
    pub fn cmbev19(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CMB Event 20"]
    #[inline(always)]
    pub fn cmbev20(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CMB Event 21"]
    #[inline(always)]
    pub fn cmbev21(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CMB Event 22"]
    #[inline(always)]
    pub fn cmbev22(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CMB Event 23"]
    #[inline(always)]
    pub fn cmbev23(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CMB Event 24"]
    #[inline(always)]
    pub fn cmbev24(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CMB Event 25"]
    #[inline(always)]
    pub fn cmbev25(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CMB Event 26"]
    #[inline(always)]
    pub fn cmbev26(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - CMB Event 27"]
    #[inline(always)]
    pub fn cmbev27(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CMB Event 28"]
    #[inline(always)]
    pub fn cmbev28(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - CMB Event 29"]
    #[inline(always)]
    pub fn cmbev29(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - CMB Event 30"]
    #[inline(always)]
    pub fn cmbev30(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CMB Event 31"]
    #[inline(always)]
    pub fn cmbev31(&self) -> CmbevR {
        CmbevR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLM_CMBEV")
            .field("cmbev0", &self.cmbev0())
            .field("cmbev1", &self.cmbev1())
            .field("cmbev2", &self.cmbev2())
            .field("cmbev3", &self.cmbev3())
            .field("cmbev4", &self.cmbev4())
            .field("cmbev5", &self.cmbev5())
            .field("cmbev6", &self.cmbev6())
            .field("cmbev7", &self.cmbev7())
            .field("cmbev8", &self.cmbev8())
            .field("cmbev9", &self.cmbev9())
            .field("cmbev10", &self.cmbev10())
            .field("cmbev11", &self.cmbev11())
            .field("cmbev12", &self.cmbev12())
            .field("cmbev13", &self.cmbev13())
            .field("cmbev14", &self.cmbev14())
            .field("cmbev15", &self.cmbev15())
            .field("cmbev16", &self.cmbev16())
            .field("cmbev17", &self.cmbev17())
            .field("cmbev18", &self.cmbev18())
            .field("cmbev19", &self.cmbev19())
            .field("cmbev20", &self.cmbev20())
            .field("cmbev21", &self.cmbev21())
            .field("cmbev22", &self.cmbev22())
            .field("cmbev23", &self.cmbev23())
            .field("cmbev24", &self.cmbev24())
            .field("cmbev25", &self.cmbev25())
            .field("cmbev26", &self.cmbev26())
            .field("cmbev27", &self.cmbev27())
            .field("cmbev28", &self.cmbev28())
            .field("cmbev29", &self.cmbev29())
            .field("cmbev30", &self.cmbev30())
            .field("cmbev31", &self.cmbev31())
            .finish()
    }
}
#[doc = "FLM CMB Event Register (FLM_CMBEV)\n\nYou can [`read`](crate::Reg::read) this register and get [`flm_cmbev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlmCmbevSpec;
impl crate::RegisterSpec for FlmCmbevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flm_cmbev::R`](R) reader structure"]
impl crate::Readable for FlmCmbevSpec {}
#[doc = "`reset()` method sets FLM_CMBEV to value 0"]
impl crate::Resettable for FlmCmbevSpec {
    const RESET_VALUE: u32 = 0;
}
