# [doc = "Register `AHB2LPENR` reader"] pub type R = crate :: R < Ahb2lpenrSpec > ; # [doc = "Register `AHB2LPENR` writer"] pub type W = crate :: W < Ahb2lpenrSpec > ; # [doc = "Camera interface enable during Sleep mode\n\nValue on reset: 1"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Dcmilpen { # [doc = "0: Selected module is disabled during Sleep mode"] DisabledInSleep = 0 , # [doc = "1: Selected module is enabled during Sleep mode"] EnabledInSleep = 1 , } impl From < Dcmilpen > for bool { # [inline (always)] fn from (variant : Dcmilpen) -> Self { variant as u8 != 0 } } # [doc = "Field `DCMILPEN` reader - Camera interface enable during Sleep mode"] pub type DcmilpenR = crate :: BitReader < Dcmilpen > ; impl DcmilpenR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Dcmilpen { match self . bits { false => Dcmilpen :: DisabledInSleep , true => Dcmilpen :: EnabledInSleep , } } # [doc = "Selected module is disabled during Sleep mode"] # [inline (always)] pub fn is_disabled_in_sleep (& self) -> bool { * self == Dcmilpen :: DisabledInSleep } # [doc = "Selected module is enabled during Sleep mode"] # [inline (always)] pub fn is_enabled_in_sleep (& self) -> bool { * self == Dcmilpen :: EnabledInSleep } } # [doc = "Field `DCMILPEN` writer - Camera interface enable during Sleep mode"] pub type DcmilpenW < 'a , REG > = crate :: BitWriter < 'a , REG , Dcmilpen > ; impl < 'a , REG > DcmilpenW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "Selected module is disabled during Sleep mode"] # [inline (always)] pub fn disabled_in_sleep (self) -> & 'a mut crate :: W < REG > { self . variant (Dcmilpen :: DisabledInSleep) } # [doc = "Selected module is enabled during Sleep mode"] # [inline (always)] pub fn enabled_in_sleep (self) -> & 'a mut crate :: W < REG > { self . variant (Dcmilpen :: EnabledInSleep) } } # [doc = "Field `RNGLPEN` reader - Random number generator clock enable during Sleep mode"] pub use DcmilpenR as RnglpenR ; # [doc = "Field `RNGLPEN` writer - Random number generator clock enable during Sleep mode"] pub use DcmilpenW as RnglpenW ; # [doc = "Field `OTGFSLPEN` reader - USB OTG FS clock enable during Sleep mode"] pub use DcmilpenR as OtgfslpenR ; # [doc = "Field `OTGFSLPEN` writer - USB OTG FS clock enable during Sleep mode"] pub use DcmilpenW as OtgfslpenW ; impl R { # [doc = "Bit 0 - Camera interface enable during Sleep mode"] # [inline (always)] pub fn dcmilpen (& self) -> DcmilpenR { DcmilpenR :: new ((self . bits & 1) != 0) } # [doc = "Bit 6 - Random number generator clock enable during Sleep mode"] # [inline (always)] pub fn rnglpen (& self) -> RnglpenR { RnglpenR :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"] # [inline (always)] pub fn otgfslpen (& self) -> OtgfslpenR { OtgfslpenR :: new (((self . bits >> 7) & 1) != 0) } } impl W { # [doc = "Bit 0 - Camera interface enable during Sleep mode"] # [inline (always)] pub fn dcmilpen (& mut self) -> DcmilpenW < Ahb2lpenrSpec > { DcmilpenW :: new (self , 0) } # [doc = "Bit 6 - Random number generator clock enable during Sleep mode"] # [inline (always)] pub fn rnglpen (& mut self) -> RnglpenW < Ahb2lpenrSpec > { RnglpenW :: new (self , 6) } # [doc = "Bit 7 - USB OTG FS clock enable during Sleep mode"] # [inline (always)] pub fn otgfslpen (& mut self) -> OtgfslpenW < Ahb2lpenrSpec > { OtgfslpenW :: new (self , 7) } } # [doc = "AHB2 peripheral clock enable in low power mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct Ahb2lpenrSpec ; impl crate :: RegisterSpec for Ahb2lpenrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`ahb2lpenr::R`](R) reader structure"] impl crate :: Readable for Ahb2lpenrSpec { } # [doc = "`write(|w| ..)` method takes [`ahb2lpenr::W`](W) writer structure"] impl crate :: Writable for Ahb2lpenrSpec { type Safety = crate :: Unsafe ; } # [doc = "`reset()` method sets AHB2LPENR to value 0xf1"] impl crate :: Resettable for Ahb2lpenrSpec { const RESET_VALUE : u32 = 0xf1 ; }