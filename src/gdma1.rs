#[repr(C)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    channel: [Channel; 2],
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - Cluster CHANNEL%s, containing GDMA_CTL*, GDMA_SRCB*, GDMA_DSTB*, GDMA_TCNT*, GDMA_CSRC*, GDMA_CDST*, GDMA_CTCNT*, GDMA_REQ_SL*"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        &self.channel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - Cluster CHANNEL%s, containing GDMA_CTL*, GDMA_SRCB*, GDMA_DSTB*, GDMA_TCNT*, GDMA_CSRC*, GDMA_CDST*, GDMA_CTCNT*, GDMA_REQ_SL*"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        self.channel.iter()
    }
}
#[doc = "Cluster CHANNEL%s, containing GDMA_CTL*, GDMA_SRCB*, GDMA_DSTB*, GDMA_TCNT*, GDMA_CSRC*, GDMA_CDST*, GDMA_CTCNT*, GDMA_REQ_SL*"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "Cluster CHANNEL%s, containing GDMA_CTL*, GDMA_SRCB*, GDMA_DSTB*, GDMA_TCNT*, GDMA_CSRC*, GDMA_CDST*, GDMA_CTCNT*, GDMA_REQ_SL*"]
pub mod channel;
