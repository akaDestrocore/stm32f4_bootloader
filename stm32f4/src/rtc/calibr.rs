# [doc = "Register `CALIBR` reader"] pub type R = crate :: R < CalibrSpec > ; # [doc = "Register `CALIBR` writer"] pub type W = crate :: W < CalibrSpec > ; # [doc = "Field `DC` reader - Digital calibration"] pub type DcR = crate :: FieldReader ; # [doc = "Field `DC` writer - Digital calibration"] pub type DcW < 'a , REG > = crate :: FieldWriter < 'a , REG , 5 > ; # [doc = "Field `DCS` reader - Digital calibration sign"] pub type DcsR = crate :: BitReader ; # [doc = "Field `DCS` writer - Digital calibration sign"] pub type DcsW < 'a , REG > = crate :: BitWriter < 'a , REG > ; impl R { # [doc = "Bits 0:4 - Digital calibration"] # [inline (always)] pub fn dc (& self) -> DcR { DcR :: new ((self . bits & 0x1f) as u8) } # [doc = "Bit 7 - Digital calibration sign"] # [inline (always)] pub fn dcs (& self) -> DcsR { DcsR :: new (((self . bits >> 7) & 1) != 0) } } impl W { # [doc = "Bits 0:4 - Digital calibration"] # [inline (always)] pub fn dc (& mut self) -> DcW < CalibrSpec > { DcW :: new (self , 0) } # [doc = "Bit 7 - Digital calibration sign"] # [inline (always)] pub fn dcs (& mut self) -> DcsW < CalibrSpec > { DcsW :: new (self , 7) } } # [doc = "calibration register\n\nYou can [`read`](crate::Reg::read) this register and get [`calibr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calibr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct CalibrSpec ; impl crate :: RegisterSpec for CalibrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`calibr::R`](R) reader structure"] impl crate :: Readable for CalibrSpec { } # [doc = "`write(|w| ..)` method takes [`calibr::W`](W) writer structure"] impl crate :: Writable for CalibrSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets CALIBR to value 0"] impl crate :: Resettable for CalibrSpec { }