# [doc = "Register `MIS` reader"] pub type R = crate :: R < MisSpec > ; # [doc = "Field `FRAME_MIS` reader - Capture complete masked interrupt status"] pub type FrameMisR = crate :: BitReader ; # [doc = "Field `OVR_MIS` reader - Overrun masked interrupt status"] pub type OvrMisR = crate :: BitReader ; # [doc = "Field `ERR_MIS` reader - Synchronization error masked interrupt status"] pub type ErrMisR = crate :: BitReader ; # [doc = "Field `VSYNC_MIS` reader - VSYNC masked interrupt status"] pub type VsyncMisR = crate :: BitReader ; # [doc = "Field `LINE_MIS` reader - Line masked interrupt status"] pub type LineMisR = crate :: BitReader ; impl R { # [doc = "Bit 0 - Capture complete masked interrupt status"] # [inline (always)] pub fn frame_mis (& self) -> FrameMisR { FrameMisR :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Overrun masked interrupt status"] # [inline (always)] pub fn ovr_mis (& self) -> OvrMisR { OvrMisR :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Synchronization error masked interrupt status"] # [inline (always)] pub fn err_mis (& self) -> ErrMisR { ErrMisR :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - VSYNC masked interrupt status"] # [inline (always)] pub fn vsync_mis (& self) -> VsyncMisR { VsyncMisR :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - Line masked interrupt status"] # [inline (always)] pub fn line_mis (& self) -> LineMisR { LineMisR :: new (((self . bits >> 4) & 1) != 0) } } # [doc = "masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct MisSpec ; impl crate :: RegisterSpec for MisSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`mis::R`](R) reader structure"] impl crate :: Readable for MisSpec { } # [doc = "`reset()` method sets MIS to value 0"] impl crate :: Resettable for MisSpec { }