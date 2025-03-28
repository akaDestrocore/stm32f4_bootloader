# [doc = "Register `HR%s` reader"] pub type R = crate :: R < HrSpec > ; # [doc = "Field `H` reader - H0"] pub type HR = crate :: FieldReader < u32 > ; impl R { # [doc = "Bits 0:31 - H0"] # [inline (always)] pub fn h (& self) -> HR { HR :: new (self . bits) } } # [doc = "digest registers\n\nYou can [`read`](crate::Reg::read) this register and get [`hr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct HrSpec ; impl crate :: RegisterSpec for HrSpec { type Ux = u32 ; } # [doc = "`read()` method returns [`hr::R`](R) reader structure"] impl crate :: Readable for HrSpec { } # [doc = "`reset()` method sets HR%s to value 0"] impl crate :: Resettable for HrSpec { }