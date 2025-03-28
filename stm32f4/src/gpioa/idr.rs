# [doc = "Register `IDR` reader"] pub type R = crate :: R < IdrSpec > ; # [doc = "Port input data pin %s\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum InputData { # [doc = "0: Input is logic low"] Low = 0 , # [doc = "1: Input is logic high"] High = 1 , } impl From < InputData > for bool { # [inline (always)] fn from (variant : InputData) -> Self { variant as u8 != 0 } } # [doc = "Field `IDR(0-15)` reader - Port input data pin %s"] pub type IdrR = crate :: BitReader < InputData > ; impl IdrR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> InputData { match self . bits { false => InputData :: Low , true => InputData :: High , } } # [doc = "Input is logic low"] # [inline (always)] pub fn is_low (& self) -> bool { * self == InputData :: Low } # [doc = "Input is logic high"] # [inline (always)] pub fn is_high (& self) -> bool { * self == InputData :: High } } impl R { # [doc = "Port input data pin (0-15)"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `IDR0` field.</div>"] # [inline (always)] pub fn idr (& self , n : u8) -> IdrR { # [allow (clippy :: no_effect)] [() ; 16] [n as usize] ; IdrR :: new (((self . bits >> n) & 1) != 0) } # [doc = "Iterator for array of:"] # [doc = "Port input data pin (0-15)"] # [inline (always)] pub fn idr_iter (& self) -> impl Iterator < Item = IdrR > + '_ { (0 .. 16) . map (move | n | IdrR :: new (((self . bits >> n) & 1) != 0)) } # [doc = "Bit 0 - Port input data pin 0"] # [inline (always)] pub fn idr0 (& self) -> IdrR { IdrR :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - Port input data pin 1"] # [inline (always)] pub fn idr1 (& self) -> IdrR { IdrR :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - Port input data pin 2"] # [inline (always)] pub fn idr2 (& self) -> IdrR { IdrR :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - Port input data pin 3"] # [inline (always)] pub fn idr3 (& self) -> IdrR { IdrR :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - Port input data pin 4"] # [inline (always)] pub fn idr4 (& self) -> IdrR { IdrR :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - Port input data pin 5"] # [inline (always)] pub fn idr5 (& self) -> IdrR { IdrR :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - Port input data pin 6"] # [inline (always)] pub fn idr6 (& self) -> IdrR { IdrR :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 7 - Port input data pin 7"] # [inline (always)] pub fn idr7 (& self) -> IdrR { IdrR :: new (((self . bits >> 7) & 1) != 0) } # [doc = "Bit 8 - Port input data pin 8"] # [inline (always)] pub fn idr8 (& self) -> IdrR { IdrR :: new (((self . bits >> 8) & 1) != 0) } # [doc = "Bit 9 - Port input data pin 9"] # [inline (always)] pub fn idr9 (& self) -> IdrR { IdrR :: new (((self . bits >> 9) & 1) != 0) } # [doc = "Bit 10 - Port input data pin 10"] # [inline (always)] pub fn idr10 (& self) -> IdrR { IdrR :: new (((self . bits >> 10) & 1) != 0) } # [doc = "Bit 11 - Port input data pin 11"] # [inline (always)] pub fn idr11 (& self) -> IdrR { IdrR :: new (((self . bits >> 11) & 1) != 0) } # [doc = "Bit 12 - Port input data pin 12"] # [inline (always)] pub fn idr12 (& self) -> IdrR { IdrR :: new (((self . bits >> 12) & 1) != 0) } # [doc = "Bit 13 - Port input data pin 13"] # [inline (always)] pub fn idr13 (& self) -> IdrR { IdrR :: new (((self . bits >> 13) & 1) != 0) } # [doc = "Bit 14 - Port input data pin 14"] # [inline (always)] pub fn idr14 (& self) -> IdrR { IdrR :: new (((self . bits >> 14) & 1) != 0) } # [doc = "Bit 15 - Port input data pin 15"] # [inline (always)] pub fn idr15 (& self) -> IdrR { IdrR :: new (((self . bits >> 15) & 1) != 0) } } # [doc = "GPIO port input data register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct IdrSpec ; impl crate :: RegisterSpec for IdrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`idr::R`](R) reader structure"] impl crate :: Readable for IdrSpec { } # [doc = "`reset()` method sets IDR to value 0"] impl crate :: Resettable for IdrSpec { }