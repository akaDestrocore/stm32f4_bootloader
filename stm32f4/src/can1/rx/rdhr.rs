# [doc = "Register `RDHR` reader"] pub type R = crate :: R < RdhrSpec > ; # [doc = "Field `DATA(4-7)` reader - DATA%s"] pub type DataR = crate :: FieldReader ; impl R { # [doc = "DATA(4-7)"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DATA4` field.</div>"] # [inline (always)] pub fn data (& self , n : u8) -> DataR { # [allow (clippy :: no_effect)] [() ; 4] [n as usize] ; DataR :: new (((self . bits >> (n * 8)) & 0xff) as u8) } # [doc = "Iterator for array of:"] # [doc = "DATA(4-7)"] # [inline (always)] pub fn data_iter (& self) -> impl Iterator < Item = DataR > + '_ { (0 .. 4) . map (move | n | DataR :: new (((self . bits >> (n * 8)) & 0xff) as u8)) } # [doc = "Bits 0:7 - DATA4"] # [inline (always)] pub fn data4 (& self) -> DataR { DataR :: new ((self . bits & 0xff) as u8) } # [doc = "Bits 8:15 - DATA5"] # [inline (always)] pub fn data5 (& self) -> DataR { DataR :: new (((self . bits >> 8) & 0xff) as u8) } # [doc = "Bits 16:23 - DATA6"] # [inline (always)] pub fn data6 (& self) -> DataR { DataR :: new (((self . bits >> 16) & 0xff) as u8) } # [doc = "Bits 24:31 - DATA7"] # [inline (always)] pub fn data7 (& self) -> DataR { DataR :: new (((self . bits >> 24) & 0xff) as u8) } } # [doc = "receive FIFO mailbox data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdhr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct RdhrSpec ; impl crate :: RegisterSpec for RdhrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`rdhr::R`](R) reader structure"] impl crate :: Readable for RdhrSpec { } # [doc = "`reset()` method sets RDHR to value 0"] impl crate :: Resettable for RdhrSpec { }