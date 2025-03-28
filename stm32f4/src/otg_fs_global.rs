# [repr (C)] # [doc = "Register block"] pub struct RegisterBlock { gotgctl : Gotgctl , gotgint : Gotgint , gahbcfg : Gahbcfg , gusbcfg : Gusbcfg , grstctl : Grstctl , gintsts : Gintsts , gintmsk : Gintmsk , _reserved_7_grxstsr : [u8 ; 0x04] , _reserved_8_grxstsp : [u8 ; 0x04] , grxfsiz : Grxfsiz , _reserved_10_dieptxf0 : [u8 ; 0x04] , gnptxsts : Gnptxsts , _reserved12 : [u8 ; 0x08] , gccfg : Gccfg , cid : Cid , _reserved14 : [u8 ; 0xc0] , hptxfsiz : Hptxfsiz , dieptxf : [Dieptxf ; 5] , } impl RegisterBlock { # [doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)"] # [inline (always)] pub const fn gotgctl (& self) -> & Gotgctl { & self . gotgctl } # [doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)"] # [inline (always)] pub const fn gotgint (& self) -> & Gotgint { & self . gotgint } # [doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"] # [inline (always)] pub const fn gahbcfg (& self) -> & Gahbcfg { & self . gahbcfg } # [doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)"] # [inline (always)] pub const fn gusbcfg (& self) -> & Gusbcfg { & self . gusbcfg } # [doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)"] # [inline (always)] pub const fn grstctl (& self) -> & Grstctl { & self . grstctl } # [doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)"] # [inline (always)] pub const fn gintsts (& self) -> & Gintsts { & self . gintsts } # [doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)"] # [inline (always)] pub const fn gintmsk (& self) -> & Gintmsk { & self . gintmsk } # [doc = "0x1c - OTG status debug read (host mode)"] # [inline (always)] pub const fn grxstsr_host (& self) -> & GrxstsrHost { unsafe { & * core :: ptr :: from_ref (self) . cast :: < u8 > () . add (28) . cast () } } # [doc = "0x1c - OTG_FS Receive status debug read(Device mode)"] # [inline (always)] pub const fn grxstsr_device (& self) -> & GrxstsrDevice { unsafe { & * core :: ptr :: from_ref (self) . cast :: < u8 > () . add (28) . cast () } } # [doc = "0x20 - OTG status read and pop (host mode)"] # [inline (always)] pub const fn grxstsp_host (& self) -> & GrxstspHost { unsafe { & * core :: ptr :: from_ref (self) . cast :: < u8 > () . add (32) . cast () } } # [doc = "0x20 - OTG status read and pop (device mode)"] # [inline (always)] pub const fn grxstsp_device (& self) -> & GrxstspDevice { unsafe { & * core :: ptr :: from_ref (self) . cast :: < u8 > () . add (32) . cast () } } # [doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"] # [inline (always)] pub const fn grxfsiz (& self) -> & Grxfsiz { & self . grxfsiz } # [doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Host mode)"] # [inline (always)] pub const fn hnptxfsiz (& self) -> & Hnptxfsiz { unsafe { & * core :: ptr :: from_ref (self) . cast :: < u8 > () . add (40) . cast () } } # [doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)"] # [inline (always)] pub const fn dieptxf0 (& self) -> & Dieptxf0 { unsafe { & * core :: ptr :: from_ref (self) . cast :: < u8 > () . add (40) . cast () } } # [doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"] # [inline (always)] pub const fn gnptxsts (& self) -> & Gnptxsts { & self . gnptxsts } # [doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)"] # [inline (always)] pub const fn gccfg (& self) -> & Gccfg { & self . gccfg } # [doc = "0x3c - core ID register"] # [inline (always)] pub const fn cid (& self) -> & Cid { & self . cid } # [doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"] # [inline (always)] pub const fn hptxfsiz (& self) -> & Hptxfsiz { & self . hptxfsiz } # [doc = "0x104..0x118 - OTF_FS device IN endpoint transmit FIFO size register"] # [doc = ""] # [doc = "<div class=\"warning\">`n` is the index of register in the array. `n == 0` corresponds to `DIEPTXF1` register.</div>"] # [inline (always)] pub const fn dieptxf (& self , n : usize) -> & Dieptxf { & self . dieptxf [n] } # [doc = "Iterator for array of:"] # [doc = "0x104..0x118 - OTF_FS device IN endpoint transmit FIFO size register"] # [inline (always)] pub fn dieptxf_iter (& self) -> impl Iterator < Item = & Dieptxf > { self . dieptxf . iter () } # [doc = "0x104 - OTF_FS device IN endpoint transmit FIFO size register"] # [inline (always)] pub const fn dieptxf1 (& self) -> & Dieptxf { self . dieptxf (0) } # [doc = "0x108 - OTF_FS device IN endpoint transmit FIFO size register"] # [inline (always)] pub const fn dieptxf2 (& self) -> & Dieptxf { self . dieptxf (1) } # [doc = "0x10c - OTF_FS device IN endpoint transmit FIFO size register"] # [inline (always)] pub const fn dieptxf3 (& self) -> & Dieptxf { self . dieptxf (2) } # [doc = "0x110 - OTF_FS device IN endpoint transmit FIFO size register"] # [inline (always)] pub const fn dieptxf4 (& self) -> & Dieptxf { self . dieptxf (3) } # [doc = "0x114 - OTF_FS device IN endpoint transmit FIFO size register"] # [inline (always)] pub const fn dieptxf5 (& self) -> & Dieptxf { self . dieptxf (4) } } # [doc = "GOTGCTL (rw) register accessor: OTG_FS control and status register (OTG_FS_GOTGCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`] module"] # [doc (alias = "GOTGCTL")] pub type Gotgctl = crate :: Reg < gotgctl :: GotgctlSpec > ; # [doc = "OTG_FS control and status register (OTG_FS_GOTGCTL)"] pub mod gotgctl ; # [doc = "GOTGINT (rw) register accessor: OTG_FS interrupt register (OTG_FS_GOTGINT)\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`] module"] # [doc (alias = "GOTGINT")] pub type Gotgint = crate :: Reg < gotgint :: GotgintSpec > ; # [doc = "OTG_FS interrupt register (OTG_FS_GOTGINT)"] pub mod gotgint ; # [doc = "GAHBCFG (rw) register accessor: OTG_FS AHB configuration register (OTG_FS_GAHBCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`] module"] # [doc (alias = "GAHBCFG")] pub type Gahbcfg = crate :: Reg < gahbcfg :: GahbcfgSpec > ; # [doc = "OTG_FS AHB configuration register (OTG_FS_GAHBCFG)"] pub mod gahbcfg ; # [doc = "GUSBCFG (rw) register accessor: OTG_FS USB configuration register (OTG_FS_GUSBCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`] module"] # [doc (alias = "GUSBCFG")] pub type Gusbcfg = crate :: Reg < gusbcfg :: GusbcfgSpec > ; # [doc = "OTG_FS USB configuration register (OTG_FS_GUSBCFG)"] pub mod gusbcfg ; # [doc = "GRSTCTL (rw) register accessor: OTG_FS reset register (OTG_FS_GRSTCTL)\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`] module"] # [doc (alias = "GRSTCTL")] pub type Grstctl = crate :: Reg < grstctl :: GrstctlSpec > ; # [doc = "OTG_FS reset register (OTG_FS_GRSTCTL)"] pub mod grstctl ; # [doc = "GINTSTS (rw) register accessor: OTG_FS core interrupt register (OTG_FS_GINTSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`] module"] # [doc (alias = "GINTSTS")] pub type Gintsts = crate :: Reg < gintsts :: GintstsSpec > ; # [doc = "OTG_FS core interrupt register (OTG_FS_GINTSTS)"] pub mod gintsts ; # [doc = "GINTMSK (rw) register accessor: OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`] module"] # [doc (alias = "GINTMSK")] pub type Gintmsk = crate :: Reg < gintmsk :: GintmskSpec > ; # [doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)"] pub mod gintmsk ; # [doc = "GRXSTSR_Device (r) register accessor: OTG_FS Receive status debug read(Device mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_device`] module"] # [doc (alias = "GRXSTSR_Device")] pub type GrxstsrDevice = crate :: Reg < grxstsr_device :: GrxstsrDeviceSpec > ; # [doc = "OTG_FS Receive status debug read(Device mode)"] pub mod grxstsr_device ; # [doc = "GRXSTSR_Host (r) register accessor: OTG status debug read (host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr_host`] module"] # [doc (alias = "GRXSTSR_Host")] pub type GrxstsrHost = crate :: Reg < grxstsr_host :: GrxstsrHostSpec > ; # [doc = "OTG status debug read (host mode)"] pub mod grxstsr_host ; # [doc = "GRXSTSP_Device (r) register accessor: OTG status read and pop (device mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsp_device::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_device`] module"] # [doc (alias = "GRXSTSP_Device")] pub type GrxstspDevice = crate :: Reg < grxstsp_device :: GrxstspDeviceSpec > ; # [doc = "OTG status read and pop (device mode)"] pub mod grxstsp_device ; # [doc = "GRXSTSP_Host (r) register accessor: OTG status read and pop (host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsp_host::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp_host`] module"] # [doc (alias = "GRXSTSP_Host")] pub type GrxstspHost = crate :: Reg < grxstsp_host :: GrxstspHostSpec > ; # [doc = "OTG status read and pop (host mode)"] pub mod grxstsp_host ; # [doc = "GRXFSIZ (rw) register accessor: OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)\n\nYou can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`] module"] # [doc (alias = "GRXFSIZ")] pub type Grxfsiz = crate :: Reg < grxfsiz :: GrxfsizSpec > ; # [doc = "OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)"] pub mod grxfsiz ; # [doc = "DIEPTXF0 (rw) register accessor: OTG_FS non-periodic transmit FIFO size register (Device mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf0`] module"] # [doc (alias = "DIEPTXF0")] pub type Dieptxf0 = crate :: Reg < dieptxf0 :: Dieptxf0Spec > ; # [doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)"] pub mod dieptxf0 ; # [doc = "HNPTXFSIZ (rw) register accessor: OTG_FS non-periodic transmit FIFO size register (Host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`hnptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hnptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hnptxfsiz`] module"] # [doc (alias = "HNPTXFSIZ")] pub type Hnptxfsiz = crate :: Reg < hnptxfsiz :: HnptxfsizSpec > ; # [doc = "OTG_FS non-periodic transmit FIFO size register (Host mode)"] pub mod hnptxfsiz ; # [doc = "GNPTXSTS (r) register accessor: OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxsts`] module"] # [doc (alias = "GNPTXSTS")] pub type Gnptxsts = crate :: Reg < gnptxsts :: GnptxstsSpec > ; # [doc = "OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)"] pub mod gnptxsts ; # [doc = "GCCFG (rw) register accessor: OTG_FS general core configuration register (OTG_FS_GCCFG)\n\nYou can [`read`](crate::Reg::read) this register and get [`gccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gccfg`] module"] # [doc (alias = "GCCFG")] pub type Gccfg = crate :: Reg < gccfg :: GccfgSpec > ; # [doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)"] pub mod gccfg ; # [doc = "CID (rw) register accessor: core ID register\n\nYou can [`read`](crate::Reg::read) this register and get [`cid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cid`] module"] # [doc (alias = "CID")] pub type Cid = crate :: Reg < cid :: CidSpec > ; # [doc = "core ID register"] pub mod cid ; # [doc = "HPTXFSIZ (rw) register accessor: OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`] module"] # [doc (alias = "HPTXFSIZ")] pub type Hptxfsiz = crate :: Reg < hptxfsiz :: HptxfsizSpec > ; # [doc = "OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)"] pub mod hptxfsiz ; # [doc = "DIEPTXF (rw) register accessor: OTF_FS device IN endpoint transmit FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf`] module"] # [doc (alias = "DIEPTXF")] pub type Dieptxf = crate :: Reg < dieptxf :: DieptxfSpec > ; # [doc = "OTF_FS device IN endpoint transmit FIFO size register"] pub mod dieptxf ;