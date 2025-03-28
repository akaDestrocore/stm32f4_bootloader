# [repr (C)] # [doc = "Register block"] pub struct RegisterBlock { dcfg : Dcfg , dctl : Dctl , dsts : Dsts , _reserved3 : [u8 ; 0x04] , diepmsk : Diepmsk , doepmsk : Doepmsk , daint : Daint , daintmsk : Daintmsk , _reserved7 : [u8 ; 0x08] , dvbusdis : Dvbusdis , dvbuspulse : Dvbuspulse , _reserved9 : [u8 ; 0x04] , diepempmsk : Diepempmsk , _reserved10 : [u8 ; 0xc8] , diep0 : Diep0 , _reserved11 : [u8 ; 0x04] , diep : () , _reserved12 : [u8 ; 0x01e0] , doep0 : Doep0 , _reserved13 : [u8 ; 0x0c] , doep : () , } impl RegisterBlock { # [doc = "0x00 - OTG_FS device configuration register (OTG_FS_DCFG)"] # [inline (always)] pub const fn dcfg (& self) -> & Dcfg { & self . dcfg } # [doc = "0x04 - OTG_FS device control register (OTG_FS_DCTL)"] # [inline (always)] pub const fn dctl (& self) -> & Dctl { & self . dctl } # [doc = "0x08 - OTG_FS device status register (OTG_FS_DSTS)"] # [inline (always)] pub const fn dsts (& self) -> & Dsts { & self . dsts } # [doc = "0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"] # [inline (always)] pub const fn diepmsk (& self) -> & Diepmsk { & self . diepmsk } # [doc = "0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"] # [inline (always)] pub const fn doepmsk (& self) -> & Doepmsk { & self . doepmsk } # [doc = "0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"] # [inline (always)] pub const fn daint (& self) -> & Daint { & self . daint } # [doc = "0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"] # [inline (always)] pub const fn daintmsk (& self) -> & Daintmsk { & self . daintmsk } # [doc = "0x28 - OTG_FS device VBUS discharge time register"] # [inline (always)] pub const fn dvbusdis (& self) -> & Dvbusdis { & self . dvbusdis } # [doc = "0x2c - OTG_FS device VBUS pulsing time register"] # [inline (always)] pub const fn dvbuspulse (& self) -> & Dvbuspulse { & self . dvbuspulse } # [doc = "0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register"] # [inline (always)] pub const fn diepempmsk (& self) -> & Diepempmsk { & self . diepempmsk } # [doc = "0x100..0x11c - Device IN endpoint 0"] # [inline (always)] pub const fn diep0 (& self) -> & Diep0 { & self . diep0 } # [doc = "0x120..0x1ac - Device IN endpoint X"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is the index of cluster in the array. `n == 0` corresponds to `DIEP1` cluster.</div>"] # [inline (always)] pub const fn diep (& self , n : usize) -> & Diep { # [allow (clippy :: no_effect)] [() ; 5] [n] ; unsafe { & * core :: ptr :: from_ref (self) . cast :: < u8 > () . add (288) . add (32 * n) . cast () } } # [doc = "Iterator for array of:"] # [doc = "0x120..0x1ac - Device IN endpoint X"] # [inline (always)] pub fn diep_iter (& self) -> impl Iterator < Item = & Diep > { (0 .. 5) . map (move | n | unsafe { & * core :: ptr :: from_ref (self) . cast :: < u8 > () . add (288) . add (32 * n) . cast () }) } # [doc = "0x120..0x13c - Device IN endpoint X"] # [inline (always)] pub const fn diep1 (& self) -> & Diep { self . diep (0) } # [doc = "0x140..0x15c - Device IN endpoint X"] # [inline (always)] pub const fn diep2 (& self) -> & Diep { self . diep (1) } # [doc = "0x160..0x17c - Device IN endpoint X"] # [inline (always)] pub const fn diep3 (& self) -> & Diep { self . diep (2) } # [doc = "0x180..0x19c - Device IN endpoint X"] # [inline (always)] pub const fn diep4 (& self) -> & Diep { self . diep (3) } # [doc = "0x1a0..0x1bc - Device IN endpoint X"] # [inline (always)] pub const fn diep5 (& self) -> & Diep { self . diep (4) } # [doc = "0x300..0x314 - Device OUT endpoint 0"] # [inline (always)] pub const fn doep0 (& self) -> & Doep0 { & self . doep0 } # [doc = "0x320..0x384 - Device IN endpoint X"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is the index of cluster in the array. `n == 0` corresponds to `DOEP1` cluster.</div>"] # [inline (always)] pub const fn doep (& self , n : usize) -> & Doep { # [allow (clippy :: no_effect)] [() ; 5] [n] ; unsafe { & * core :: ptr :: from_ref (self) . cast :: < u8 > () . add (800) . add (32 * n) . cast () } } # [doc = "Iterator for array of:"] # [doc = "0x320..0x384 - Device IN endpoint X"] # [inline (always)] pub fn doep_iter (& self) -> impl Iterator < Item = & Doep > { (0 .. 5) . map (move | n | unsafe { & * core :: ptr :: from_ref (self) . cast :: < u8 > () . add (800) . add (32 * n) . cast () }) } # [doc = "0x320..0x334 - Device IN endpoint X"] # [inline (always)] pub const fn doep1 (& self) -> & Doep { self . doep (0) } # [doc = "0x340..0x354 - Device IN endpoint X"] # [inline (always)] pub const fn doep2 (& self) -> & Doep { self . doep (1) } # [doc = "0x360..0x374 - Device IN endpoint X"] # [inline (always)] pub const fn doep3 (& self) -> & Doep { self . doep (2) } # [doc = "0x380..0x394 - Device IN endpoint X"] # [inline (always)] pub const fn doep4 (& self) -> & Doep { self . doep (3) } # [doc = "0x3a0..0x3b4 - Device IN endpoint X"] # [inline (always)] pub const fn doep5 (& self) -> & Doep { self . doep (4) } } # [doc = "DCFG (rw) register accessor: OTG_FS device configuration register (OTG_FS_DCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`] module"] # [doc (alias = "DCFG")] pub type Dcfg = crate :: Reg < dcfg :: DcfgSpec > ; # [doc = "OTG_FS device configuration register (OTG_FS_DCFG)"] pub mod dcfg ; # [doc = "DCTL (rw) register accessor: OTG_FS device control register (OTG_FS_DCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`] module"] # [doc (alias = "DCTL")] pub type Dctl = crate :: Reg < dctl :: DctlSpec > ; # [doc = "OTG_FS device control register (OTG_FS_DCTL)"] pub mod dctl ; # [doc = "DSTS (r) register accessor: OTG_FS device status register (OTG_FS_DSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`] module"] # [doc (alias = "DSTS")] pub type Dsts = crate :: Reg < dsts :: DstsSpec > ; # [doc = "OTG_FS device status register (OTG_FS_DSTS)"] pub mod dsts ; # [doc = "DIEPMSK (rw) register accessor: OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`diepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`] module"] # [doc (alias = "DIEPMSK")] pub type Diepmsk = crate :: Reg < diepmsk :: DiepmskSpec > ; # [doc = "OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)"] pub mod diepmsk ; # [doc = "DOEPMSK (rw) register accessor: OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`] module"] # [doc (alias = "DOEPMSK")] pub type Doepmsk = crate :: Reg < doepmsk :: DoepmskSpec > ; # [doc = "OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)"] pub mod doepmsk ; # [doc = "DAINT (r) register accessor: OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`] module"] # [doc (alias = "DAINT")] pub type Daint = crate :: Reg < daint :: DaintSpec > ; # [doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)"] pub mod daint ; # [doc = "DAINTMSK (rw) register accessor: OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`daintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`] module"] # [doc (alias = "DAINTMSK")] pub type Daintmsk = crate :: Reg < daintmsk :: DaintmskSpec > ; # [doc = "OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)"] pub mod daintmsk ; # [doc = "DVBUSDIS (rw) register accessor: OTG_FS device VBUS discharge time register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbusdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`] module"] # [doc (alias = "DVBUSDIS")] pub type Dvbusdis = crate :: Reg < dvbusdis :: DvbusdisSpec > ; # [doc = "OTG_FS device VBUS discharge time register"] pub mod dvbusdis ; # [doc = "DVBUSPULSE (rw) register accessor: OTG_FS device VBUS pulsing time register\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbuspulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`] module"] # [doc (alias = "DVBUSPULSE")] pub type Dvbuspulse = crate :: Reg < dvbuspulse :: DvbuspulseSpec > ; # [doc = "OTG_FS device VBUS pulsing time register"] pub mod dvbuspulse ; # [doc = "DIEPEMPMSK (rw) register accessor: OTG_FS device IN endpoint FIFO empty interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`] module"] # [doc (alias = "DIEPEMPMSK")] pub type Diepempmsk = crate :: Reg < diepempmsk :: DiepempmskSpec > ; # [doc = "OTG_FS device IN endpoint FIFO empty interrupt mask register"] pub mod diepempmsk ; # [doc = "Device IN endpoint 0"] pub use self :: diep0 :: Diep0 ; # [doc = r"Cluster"] # [doc = "Device IN endpoint 0"] pub mod diep0 ; # [doc = "Device IN endpoint X"] pub use self :: diep :: Diep ; # [doc = r"Cluster"] # [doc = "Device IN endpoint X"] pub mod diep ; # [doc = "Device OUT endpoint 0"] pub use self :: doep0 :: Doep0 ; # [doc = r"Cluster"] # [doc = "Device OUT endpoint 0"] pub mod doep0 ; # [doc = "Device IN endpoint X"] pub use self :: doep :: Doep ; # [doc = r"Cluster"] # [doc = "Device IN endpoint X"] pub mod doep ;