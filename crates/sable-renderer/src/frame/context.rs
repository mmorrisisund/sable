use crate::gpu::context::WgpuContext;

pub struct FrameContext<'a> {
    pub gpu_ctx: &'a WgpuContext,
}
