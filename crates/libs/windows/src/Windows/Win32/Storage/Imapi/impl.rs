#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2DataEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: Option<&super::super::System::Com::IDispatch>, progress: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DDiscFormat2DataEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2DataEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2DataEvents_Impl, const OFFSET: isize>() -> DDiscFormat2DataEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: DDiscFormat2DataEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DDiscFormat2DataEvents_Impl::Update(this, windows_core::from_raw_borrowed(&object), windows_core::from_raw_borrowed(&progress)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2DataEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2EraseEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: Option<&super::super::System::Com::IDispatch>, elapsedseconds: i32, estimatedtotalseconds: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DDiscFormat2EraseEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2EraseEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>() -> DDiscFormat2EraseEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, elapsedseconds: i32, estimatedtotalseconds: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DDiscFormat2EraseEvents_Impl::Update(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&elapsedseconds), core::mem::transmute_copy(&estimatedtotalseconds)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2EraseEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2RawCDEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: Option<&super::super::System::Com::IDispatch>, progress: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DDiscFormat2RawCDEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2RawCDEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>() -> DDiscFormat2RawCDEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DDiscFormat2RawCDEvents_Impl::Update(this, windows_core::from_raw_borrowed(&object), windows_core::from_raw_borrowed(&progress)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2RawCDEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2TrackAtOnceEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: Option<&super::super::System::Com::IDispatch>, progress: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DDiscFormat2TrackAtOnceEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>() -> DDiscFormat2TrackAtOnceEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DDiscFormat2TrackAtOnceEvents_Impl::Update(this, windows_core::from_raw_borrowed(&object), windows_core::from_raw_borrowed(&progress)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2TrackAtOnceEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscMaster2Events_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn NotifyDeviceAdded(&self, object: Option<&super::super::System::Com::IDispatch>, uniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NotifyDeviceRemoved(&self, object: Option<&super::super::System::Com::IDispatch>, uniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DDiscMaster2Events {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscMaster2Events_Vtbl {
    pub const fn new<Identity: DDiscMaster2Events_Impl, const OFFSET: isize>() -> DDiscMaster2Events_Vtbl {
        unsafe extern "system" fn NotifyDeviceAdded<Identity: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, uniqueid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DDiscMaster2Events_Impl::NotifyDeviceAdded(this, windows_core::from_raw_borrowed(&object), core::mem::transmute(&uniqueid)).into()
        }
        unsafe extern "system" fn NotifyDeviceRemoved<Identity: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, uniqueid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DDiscMaster2Events_Impl::NotifyDeviceRemoved(this, windows_core::from_raw_borrowed(&object), core::mem::transmute(&uniqueid)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            NotifyDeviceAdded: NotifyDeviceAdded::<Identity, OFFSET>,
            NotifyDeviceRemoved: NotifyDeviceRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscMaster2Events as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DFileSystemImageEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: Option<&super::super::System::Com::IDispatch>, currentfile: &windows_core::BSTR, copiedsectors: i32, totalsectors: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DFileSystemImageEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DFileSystemImageEvents_Vtbl {
    pub const fn new<Identity: DFileSystemImageEvents_Impl, const OFFSET: isize>() -> DFileSystemImageEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: DFileSystemImageEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, currentfile: core::mem::MaybeUninit<windows_core::BSTR>, copiedsectors: i32, totalsectors: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DFileSystemImageEvents_Impl::Update(this, windows_core::from_raw_borrowed(&object), core::mem::transmute(&currentfile), core::mem::transmute_copy(&copiedsectors), core::mem::transmute_copy(&totalsectors)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DFileSystemImageEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DFileSystemImageImportEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UpdateImport(&self, object: Option<&super::super::System::Com::IDispatch>, filesystem: FsiFileSystems, currentitem: &windows_core::BSTR, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DFileSystemImageImportEvents {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DFileSystemImageImportEvents_Vtbl {
    pub const fn new<Identity: DFileSystemImageImportEvents_Impl, const OFFSET: isize>() -> DFileSystemImageImportEvents_Vtbl {
        unsafe extern "system" fn UpdateImport<Identity: DFileSystemImageImportEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, filesystem: FsiFileSystems, currentitem: core::mem::MaybeUninit<windows_core::BSTR>, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DFileSystemImageImportEvents_Impl::UpdateImport(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&filesystem), core::mem::transmute(&currentitem), core::mem::transmute_copy(&importeddirectoryitems), core::mem::transmute_copy(&totaldirectoryitems), core::mem::transmute_copy(&importedfileitems), core::mem::transmute_copy(&totalfileitems)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), UpdateImport: UpdateImport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DFileSystemImageImportEvents as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DWriteEngine2Events_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: Option<&super::super::System::Com::IDispatch>, progress: Option<&super::super::System::Com::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for DWriteEngine2Events {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DWriteEngine2Events_Vtbl {
    pub const fn new<Identity: DWriteEngine2Events_Impl, const OFFSET: isize>() -> DWriteEngine2Events_Vtbl {
        unsafe extern "system" fn Update<Identity: DWriteEngine2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            DWriteEngine2Events_Impl::Update(this, windows_core::from_raw_borrowed(&object), windows_core::from_raw_borrowed(&progress)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DWriteEngine2Events as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBlockRange_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartLba(&self) -> windows_core::Result<i32>;
    fn EndLba(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IBlockRange {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBlockRange_Vtbl {
    pub const fn new<Identity: IBlockRange_Impl, const OFFSET: isize>() -> IBlockRange_Vtbl {
        unsafe extern "system" fn StartLba<Identity: IBlockRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBlockRange_Impl::StartLba(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndLba<Identity: IBlockRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBlockRange_Impl::EndLba(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartLba: StartLba::<Identity, OFFSET>,
            EndLba: EndLba::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBlockRange as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBlockRangeList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BlockRanges(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IBlockRangeList {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBlockRangeList_Vtbl {
    pub const fn new<Identity: IBlockRangeList_Impl, const OFFSET: isize>() -> IBlockRangeList_Vtbl {
        unsafe extern "system" fn BlockRanges<Identity: IBlockRangeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBlockRangeList_Impl::BlockRanges(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), BlockRanges: BlockRanges::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBlockRangeList as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBootOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BootImage(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn Manufacturer(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetManufacturer(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PlatformId(&self) -> windows_core::Result<PlatformId>;
    fn SetPlatformId(&self, newval: PlatformId) -> windows_core::Result<()>;
    fn Emulation(&self) -> windows_core::Result<EmulationType>;
    fn SetEmulation(&self, newval: EmulationType) -> windows_core::Result<()>;
    fn ImageSize(&self) -> windows_core::Result<u32>;
    fn AssignBootImage(&self, newval: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IBootOptions {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBootOptions_Vtbl {
    pub const fn new<Identity: IBootOptions_Impl, const OFFSET: isize>() -> IBootOptions_Vtbl {
        unsafe extern "system" fn BootImage<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBootOptions_Impl::BootImage(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Manufacturer<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBootOptions_Impl::Manufacturer(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManufacturer<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBootOptions_Impl::SetManufacturer(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn PlatformId<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut PlatformId) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBootOptions_Impl::PlatformId(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlatformId<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: PlatformId) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBootOptions_Impl::SetPlatformId(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Emulation<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut EmulationType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBootOptions_Impl::Emulation(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmulation<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: EmulationType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBootOptions_Impl::SetEmulation(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ImageSize<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBootOptions_Impl::ImageSize(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssignBootImage<Identity: IBootOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBootOptions_Impl::AssignBootImage(this, windows_core::from_raw_borrowed(&newval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            BootImage: BootImage::<Identity, OFFSET>,
            Manufacturer: Manufacturer::<Identity, OFFSET>,
            SetManufacturer: SetManufacturer::<Identity, OFFSET>,
            PlatformId: PlatformId::<Identity, OFFSET>,
            SetPlatformId: SetPlatformId::<Identity, OFFSET>,
            Emulation: Emulation::<Identity, OFFSET>,
            SetEmulation: SetEmulation::<Identity, OFFSET>,
            ImageSize: ImageSize::<Identity, OFFSET>,
            AssignBootImage: AssignBootImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBootOptions as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IBurnVerification_Impl: Sized + windows_core::IUnknownImpl {
    fn SetBurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::Result<()>;
    fn BurnVerificationLevel(&self) -> windows_core::Result<IMAPI_BURN_VERIFICATION_LEVEL>;
}
impl windows_core::RuntimeName for IBurnVerification {}
impl IBurnVerification_Vtbl {
    pub const fn new<Identity: IBurnVerification_Impl, const OFFSET: isize>() -> IBurnVerification_Vtbl {
        unsafe extern "system" fn SetBurnVerificationLevel<Identity: IBurnVerification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBurnVerification_Impl::SetBurnVerificationLevel(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BurnVerificationLevel<Identity: IBurnVerification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBurnVerification_Impl::BurnVerificationLevel(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBurnVerificationLevel: SetBurnVerificationLevel::<Identity, OFFSET>,
            BurnVerificationLevel: BurnVerificationLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBurnVerification as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsRecorderSupported(&self, recorder: Option<&IDiscRecorder2>) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsCurrentMediaSupported(&self, recorder: Option<&IDiscRecorder2>) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MediaPhysicallyBlank(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MediaHeuristicallyBlank(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedMediaTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2_Vtbl {
    pub const fn new<Identity: IDiscFormat2_Impl, const OFFSET: isize>() -> IDiscFormat2_Vtbl {
        unsafe extern "system" fn IsRecorderSupported<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recorder: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2_Impl::IsRecorderSupported(this, windows_core::from_raw_borrowed(&recorder)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentMediaSupported<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recorder: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2_Impl::IsCurrentMediaSupported(this, windows_core::from_raw_borrowed(&recorder)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaPhysicallyBlank<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2_Impl::MediaPhysicallyBlank(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaHeuristicallyBlank<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2_Impl::MediaHeuristicallyBlank(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedMediaTypes<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2_Impl::SupportedMediaTypes(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsRecorderSupported: IsRecorderSupported::<Identity, OFFSET>,
            IsCurrentMediaSupported: IsCurrentMediaSupported::<Identity, OFFSET>,
            MediaPhysicallyBlank: MediaPhysicallyBlank::<Identity, OFFSET>,
            MediaHeuristicallyBlank: MediaHeuristicallyBlank::<Identity, OFFSET>,
            SupportedMediaTypes: SupportedMediaTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2Data_Impl: Sized + IDiscFormat2_Impl {
    fn SetRecorder(&self, value: Option<&IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPostgapAlreadyInImage(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn PostgapAlreadyInImage(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentMediaStatus(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE>;
    fn WriteProtectStatus(&self) -> windows_core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE>;
    fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn NextWritableAddress(&self) -> windows_core::Result<i32>;
    fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn SetForceMediaToBeClosed(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ForceMediaToBeClosed(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisableConsumerDvdCompatibilityMode(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DisableConsumerDvdCompatibilityMode(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetForceOverwrite(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ForceOverwrite(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Write(&self, data: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2Data {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2Data_Vtbl {
    pub const fn new<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>() -> IDiscFormat2Data_Vtbl {
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Data_Impl::SetRecorder(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::Recorder(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Data_Impl::SetBufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::BufferUnderrunFreeDisabled(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostgapAlreadyInImage<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Data_Impl::SetPostgapAlreadyInImage(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn PostgapAlreadyInImage<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::PostgapAlreadyInImage(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMediaStatus<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::CurrentMediaStatus(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProtectStatus<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::WriteProtectStatus(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::TotalSectorsOnMedia(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::FreeSectorsOnMedia(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextWritableAddress<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::NextWritableAddress(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::StartAddressOfPreviousSession(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::LastWrittenAddressOfPreviousSession(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceMediaToBeClosed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Data_Impl::SetForceMediaToBeClosed(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ForceMediaToBeClosed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::ForceMediaToBeClosed(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableConsumerDvdCompatibilityMode<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Data_Impl::SetDisableConsumerDvdCompatibilityMode(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DisableConsumerDvdCompatibilityMode<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::DisableConsumerDvdCompatibilityMode(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::CurrentPhysicalMediaType(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Data_Impl::SetClientName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::ClientName(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::RequestedWriteSpeed(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::RequestedRotationTypeIsPureCAV(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::CurrentWriteSpeed(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::CurrentRotationTypeIsPureCAV(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::SupportedWriteSpeeds(this) {
                Ok(ok__) => {
                    supportedspeeds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::SupportedWriteSpeedDescriptors(this) {
                Ok(ok__) => {
                    supportedspeeddescriptors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceOverwrite<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Data_Impl::SetForceOverwrite(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ForceOverwrite<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::ForceOverwrite(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Data_Impl::MultisessionInterfaces(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Data_Impl::Write(this, windows_core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn CancelWrite<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Data_Impl::CancelWrite(this).into()
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Data_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            SetPostgapAlreadyInImage: SetPostgapAlreadyInImage::<Identity, OFFSET>,
            PostgapAlreadyInImage: PostgapAlreadyInImage::<Identity, OFFSET>,
            CurrentMediaStatus: CurrentMediaStatus::<Identity, OFFSET>,
            WriteProtectStatus: WriteProtectStatus::<Identity, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, OFFSET>,
            SetForceMediaToBeClosed: SetForceMediaToBeClosed::<Identity, OFFSET>,
            ForceMediaToBeClosed: ForceMediaToBeClosed::<Identity, OFFSET>,
            SetDisableConsumerDvdCompatibilityMode: SetDisableConsumerDvdCompatibilityMode::<Identity, OFFSET>,
            DisableConsumerDvdCompatibilityMode: DisableConsumerDvdCompatibilityMode::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
            SetForceOverwrite: SetForceOverwrite::<Identity, OFFSET>,
            ForceOverwrite: ForceOverwrite::<Identity, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            CancelWrite: CancelWrite::<Identity, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2Data as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2DataEventArgs_Impl: Sized + IWriteEngine2EventArgs_Impl {
    fn ElapsedTime(&self) -> windows_core::Result<i32>;
    fn RemainingTime(&self) -> windows_core::Result<i32>;
    fn TotalTime(&self) -> windows_core::Result<i32>;
    fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2DataEventArgs {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2DataEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>() -> IDiscFormat2DataEventArgs_Vtbl {
        unsafe extern "system" fn ElapsedTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2DataEventArgs_Impl::ElapsedTime(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2DataEventArgs_Impl::RemainingTime(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2DataEventArgs_Impl::TotalTime(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2DataEventArgs_Impl::CurrentAction(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, OFFSET>(),
            ElapsedTime: ElapsedTime::<Identity, OFFSET>,
            RemainingTime: RemainingTime::<Identity, OFFSET>,
            TotalTime: TotalTime::<Identity, OFFSET>,
            CurrentAction: CurrentAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2DataEventArgs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2Erase_Impl: Sized + IDiscFormat2_Impl {
    fn SetRecorder(&self, value: Option<&IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetFullErase(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FullErase(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EraseMedia(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2Erase {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2Erase_Vtbl {
    pub const fn new<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>() -> IDiscFormat2Erase_Vtbl {
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Erase_Impl::SetRecorder(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Erase_Impl::Recorder(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullErase<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Erase_Impl::SetFullErase(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn FullErase<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Erase_Impl::FullErase(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Erase_Impl::CurrentPhysicalMediaType(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Erase_Impl::SetClientName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2Erase_Impl::ClientName(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EraseMedia<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2Erase_Impl::EraseMedia(this).into()
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetFullErase: SetFullErase::<Identity, OFFSET>,
            FullErase: FullErase::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            EraseMedia: EraseMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2Erase as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2RawCD_Impl: Sized + IDiscFormat2_Impl {
    fn PrepareMedia(&self) -> windows_core::Result<()>;
    fn WriteMedia(&self, data: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn WriteMedia2(&self, data: Option<&super::super::System::Com::IStream>, streamleadinsectors: i32) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn ReleaseMedia(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRecorder(&self, value: Option<&IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn StartOfNextSession(&self) -> windows_core::Result<i32>;
    fn LastPossibleStartOfLeadout(&self) -> windows_core::Result<i32>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SupportedSectorTypes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetRequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()>;
    fn RequestedSectorType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2RawCD {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2RawCD_Vtbl {
    pub const fn new<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>() -> IDiscFormat2RawCD_Vtbl {
        unsafe extern "system" fn PrepareMedia<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2RawCD_Impl::PrepareMedia(this).into()
        }
        unsafe extern "system" fn WriteMedia<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2RawCD_Impl::WriteMedia(this, windows_core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn WriteMedia2<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, streamleadinsectors: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2RawCD_Impl::WriteMedia2(this, windows_core::from_raw_borrowed(&data), core::mem::transmute_copy(&streamleadinsectors)).into()
        }
        unsafe extern "system" fn CancelWrite<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2RawCD_Impl::CancelWrite(this).into()
        }
        unsafe extern "system" fn ReleaseMedia<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2RawCD_Impl::ReleaseMedia(this).into()
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2RawCD_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2RawCD_Impl::SetRecorder(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::Recorder(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2RawCD_Impl::SetBufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::BufferUnderrunFreeDisabled(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOfNextSession<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::StartOfNextSession(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastPossibleStartOfLeadout<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::LastPossibleStartOfLeadout(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::CurrentPhysicalMediaType(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedSectorTypes<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::SupportedSectorTypes(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedSectorType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2RawCD_Impl::SetRequestedSectorType(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RequestedSectorType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::RequestedSectorType(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2RawCD_Impl::SetClientName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::ClientName(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::RequestedWriteSpeed(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::RequestedRotationTypeIsPureCAV(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::CurrentWriteSpeed(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::CurrentRotationTypeIsPureCAV(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::SupportedWriteSpeeds(this) {
                Ok(ok__) => {
                    supportedspeeds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCD_Impl::SupportedWriteSpeedDescriptors(this) {
                Ok(ok__) => {
                    supportedspeeddescriptors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            PrepareMedia: PrepareMedia::<Identity, OFFSET>,
            WriteMedia: WriteMedia::<Identity, OFFSET>,
            WriteMedia2: WriteMedia2::<Identity, OFFSET>,
            CancelWrite: CancelWrite::<Identity, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, OFFSET>,
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            StartOfNextSession: StartOfNextSession::<Identity, OFFSET>,
            LastPossibleStartOfLeadout: LastPossibleStartOfLeadout::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SupportedSectorTypes: SupportedSectorTypes::<Identity, OFFSET>,
            SetRequestedSectorType: SetRequestedSectorType::<Identity, OFFSET>,
            RequestedSectorType: RequestedSectorType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2RawCD as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2RawCDEventArgs_Impl: Sized + IWriteEngine2EventArgs_Impl {
    fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION>;
    fn ElapsedTime(&self) -> windows_core::Result<i32>;
    fn RemainingTime(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2RawCDEventArgs {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2RawCDEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>() -> IDiscFormat2RawCDEventArgs_Vtbl {
        unsafe extern "system" fn CurrentAction<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCDEventArgs_Impl::CurrentAction(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedTime<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCDEventArgs_Impl::ElapsedTime(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2RawCDEventArgs_Impl::RemainingTime(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, OFFSET>(),
            CurrentAction: CurrentAction::<Identity, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, OFFSET>,
            RemainingTime: RemainingTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2RawCDEventArgs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2TrackAtOnce_Impl: Sized + IDiscFormat2_Impl {
    fn PrepareMedia(&self) -> windows_core::Result<()>;
    fn AddAudioTrack(&self, data: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn CancelAddTrack(&self) -> windows_core::Result<()>;
    fn ReleaseMedia(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRecorder(&self, value: Option<&IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn NumberOfExistingTracks(&self) -> windows_core::Result<i32>;
    fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn UsedSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn SetDoNotFinalizeMedia(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DoNotFinalizeMedia(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2TrackAtOnce {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2TrackAtOnce_Vtbl {
    pub const fn new<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>() -> IDiscFormat2TrackAtOnce_Vtbl {
        unsafe extern "system" fn PrepareMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2TrackAtOnce_Impl::PrepareMedia(this).into()
        }
        unsafe extern "system" fn AddAudioTrack<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2TrackAtOnce_Impl::AddAudioTrack(this, windows_core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn CancelAddTrack<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2TrackAtOnce_Impl::CancelAddTrack(this).into()
        }
        unsafe extern "system" fn ReleaseMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2TrackAtOnce_Impl::ReleaseMedia(this).into()
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2TrackAtOnce_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2TrackAtOnce_Impl::SetRecorder(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::Recorder(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2TrackAtOnce_Impl::SetBufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::BufferUnderrunFreeDisabled(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::NumberOfExistingTracks(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::TotalSectorsOnMedia(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::FreeSectorsOnMedia(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::UsedSectorsOnMedia(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoNotFinalizeMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2TrackAtOnce_Impl::SetDoNotFinalizeMedia(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DoNotFinalizeMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::DoNotFinalizeMedia(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::ExpectedTableOfContents(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::CurrentPhysicalMediaType(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscFormat2TrackAtOnce_Impl::SetClientName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::ClientName(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::RequestedWriteSpeed(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::RequestedRotationTypeIsPureCAV(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::CurrentWriteSpeed(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::CurrentRotationTypeIsPureCAV(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::SupportedWriteSpeeds(this) {
                Ok(ok__) => {
                    supportedspeeds.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnce_Impl::SupportedWriteSpeedDescriptors(this) {
                Ok(ok__) => {
                    supportedspeeddescriptors.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            PrepareMedia: PrepareMedia::<Identity, OFFSET>,
            AddAudioTrack: AddAudioTrack::<Identity, OFFSET>,
            CancelAddTrack: CancelAddTrack::<Identity, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, OFFSET>,
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, OFFSET>,
            UsedSectorsOnMedia: UsedSectorsOnMedia::<Identity, OFFSET>,
            SetDoNotFinalizeMedia: SetDoNotFinalizeMedia::<Identity, OFFSET>,
            DoNotFinalizeMedia: DoNotFinalizeMedia::<Identity, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnce as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2TrackAtOnceEventArgs_Impl: Sized + IWriteEngine2EventArgs_Impl {
    fn CurrentTrackNumber(&self) -> windows_core::Result<i32>;
    fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION>;
    fn ElapsedTime(&self) -> windows_core::Result<i32>;
    fn RemainingTime(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscFormat2TrackAtOnceEventArgs {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>() -> IDiscFormat2TrackAtOnceEventArgs_Vtbl {
        unsafe extern "system" fn CurrentTrackNumber<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnceEventArgs_Impl::CurrentTrackNumber(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnceEventArgs_Impl::CurrentAction(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedTime<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnceEventArgs_Impl::ElapsedTime(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscFormat2TrackAtOnceEventArgs_Impl::RemainingTime(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, OFFSET>(),
            CurrentTrackNumber: CurrentTrackNumber::<Identity, OFFSET>,
            CurrentAction: CurrentAction::<Identity, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, OFFSET>,
            RemainingTime: RemainingTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnceEventArgs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
pub trait IDiscMaster_Impl: Sized + windows_core::IUnknownImpl {
    fn Open(&self) -> windows_core::Result<()>;
    fn EnumDiscMasterFormats(&self) -> windows_core::Result<IEnumDiscMasterFormats>;
    fn GetActiveDiscMasterFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetActiveDiscMasterFormat(&self, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn EnumDiscRecorders(&self) -> windows_core::Result<IEnumDiscRecorders>;
    fn GetActiveDiscRecorder(&self) -> windows_core::Result<IDiscRecorder>;
    fn SetActiveDiscRecorder(&self, precorder: Option<&IDiscRecorder>) -> windows_core::Result<()>;
    fn ClearFormatContent(&self) -> windows_core::Result<()>;
    fn ProgressAdvise(&self, pevents: Option<&IDiscMasterProgressEvents>) -> windows_core::Result<usize>;
    fn ProgressUnadvise(&self, vcookie: usize) -> windows_core::Result<()>;
    fn RecordDisc(&self, bsimulate: u8, bejectafterburn: u8) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDiscMaster {}
impl IDiscMaster_Vtbl {
    pub const fn new<Identity: IDiscMaster_Impl, const OFFSET: isize>() -> IDiscMaster_Vtbl {
        unsafe extern "system" fn Open<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMaster_Impl::Open(this).into()
        }
        unsafe extern "system" fn EnumDiscMasterFormats<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscMaster_Impl::EnumDiscMasterFormats(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveDiscMasterFormat<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpiid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscMaster_Impl::GetActiveDiscMasterFormat(this) {
                Ok(ok__) => {
                    lpiid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveDiscMasterFormat<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMaster_Impl::SetActiveDiscMasterFormat(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn EnumDiscRecorders<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscMaster_Impl::EnumDiscRecorders(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveDiscRecorder<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprecorder: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscMaster_Impl::GetActiveDiscRecorder(this) {
                Ok(ok__) => {
                    pprecorder.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveDiscRecorder<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, precorder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMaster_Impl::SetActiveDiscRecorder(this, windows_core::from_raw_borrowed(&precorder)).into()
        }
        unsafe extern "system" fn ClearFormatContent<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMaster_Impl::ClearFormatContent(this).into()
        }
        unsafe extern "system" fn ProgressAdvise<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevents: *mut core::ffi::c_void, pvcookie: *mut usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscMaster_Impl::ProgressAdvise(this, windows_core::from_raw_borrowed(&pevents)) {
                Ok(ok__) => {
                    pvcookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressUnadvise<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, vcookie: usize) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMaster_Impl::ProgressUnadvise(this, core::mem::transmute_copy(&vcookie)).into()
        }
        unsafe extern "system" fn RecordDisc<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bsimulate: u8, bejectafterburn: u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMaster_Impl::RecordDisc(this, core::mem::transmute_copy(&bsimulate), core::mem::transmute_copy(&bejectafterburn)).into()
        }
        unsafe extern "system" fn Close<Identity: IDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMaster_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            EnumDiscMasterFormats: EnumDiscMasterFormats::<Identity, OFFSET>,
            GetActiveDiscMasterFormat: GetActiveDiscMasterFormat::<Identity, OFFSET>,
            SetActiveDiscMasterFormat: SetActiveDiscMasterFormat::<Identity, OFFSET>,
            EnumDiscRecorders: EnumDiscRecorders::<Identity, OFFSET>,
            GetActiveDiscRecorder: GetActiveDiscRecorder::<Identity, OFFSET>,
            SetActiveDiscRecorder: SetActiveDiscRecorder::<Identity, OFFSET>,
            ClearFormatContent: ClearFormatContent::<Identity, OFFSET>,
            ProgressAdvise: ProgressAdvise::<Identity, OFFSET>,
            ProgressUnadvise: ProgressUnadvise::<Identity, OFFSET>,
            RecordDisc: RecordDisc::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscMaster as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscMaster2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn IsSupportedEnvironment(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscMaster2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscMaster2_Vtbl {
    pub const fn new<Identity: IDiscMaster2_Impl, const OFFSET: isize>() -> IDiscMaster2_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscMaster2_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppunk.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscMaster2_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscMaster2_Impl::Count(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupportedEnvironment<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscMaster2_Impl::IsSupportedEnvironment(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            IsSupportedEnvironment: IsSupportedEnvironment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscMaster2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IDiscMasterProgressEvents_Impl: Sized + windows_core::IUnknownImpl {
    fn QueryCancel(&self) -> windows_core::Result<u8>;
    fn NotifyPnPActivity(&self) -> windows_core::Result<()>;
    fn NotifyAddProgress(&self, ncompletedsteps: i32, ntotalsteps: i32) -> windows_core::Result<()>;
    fn NotifyBlockProgress(&self, ncompleted: i32, ntotal: i32) -> windows_core::Result<()>;
    fn NotifyTrackProgress(&self, ncurrenttrack: i32, ntotaltracks: i32) -> windows_core::Result<()>;
    fn NotifyPreparingBurn(&self, nestimatedseconds: i32) -> windows_core::Result<()>;
    fn NotifyClosingDisc(&self, nestimatedseconds: i32) -> windows_core::Result<()>;
    fn NotifyBurnComplete(&self, status: windows_core::HRESULT) -> windows_core::Result<()>;
    fn NotifyEraseComplete(&self, status: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDiscMasterProgressEvents {}
impl IDiscMasterProgressEvents_Vtbl {
    pub const fn new<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>() -> IDiscMasterProgressEvents_Vtbl {
        unsafe extern "system" fn QueryCancel<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbcancel: *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscMasterProgressEvents_Impl::QueryCancel(this) {
                Ok(ok__) => {
                    pbcancel.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyPnPActivity<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMasterProgressEvents_Impl::NotifyPnPActivity(this).into()
        }
        unsafe extern "system" fn NotifyAddProgress<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncompletedsteps: i32, ntotalsteps: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMasterProgressEvents_Impl::NotifyAddProgress(this, core::mem::transmute_copy(&ncompletedsteps), core::mem::transmute_copy(&ntotalsteps)).into()
        }
        unsafe extern "system" fn NotifyBlockProgress<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncompleted: i32, ntotal: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMasterProgressEvents_Impl::NotifyBlockProgress(this, core::mem::transmute_copy(&ncompleted), core::mem::transmute_copy(&ntotal)).into()
        }
        unsafe extern "system" fn NotifyTrackProgress<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ncurrenttrack: i32, ntotaltracks: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMasterProgressEvents_Impl::NotifyTrackProgress(this, core::mem::transmute_copy(&ncurrenttrack), core::mem::transmute_copy(&ntotaltracks)).into()
        }
        unsafe extern "system" fn NotifyPreparingBurn<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nestimatedseconds: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMasterProgressEvents_Impl::NotifyPreparingBurn(this, core::mem::transmute_copy(&nestimatedseconds)).into()
        }
        unsafe extern "system" fn NotifyClosingDisc<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nestimatedseconds: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMasterProgressEvents_Impl::NotifyClosingDisc(this, core::mem::transmute_copy(&nestimatedseconds)).into()
        }
        unsafe extern "system" fn NotifyBurnComplete<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMasterProgressEvents_Impl::NotifyBurnComplete(this, core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn NotifyEraseComplete<Identity: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscMasterProgressEvents_Impl::NotifyEraseComplete(this, core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryCancel: QueryCancel::<Identity, OFFSET>,
            NotifyPnPActivity: NotifyPnPActivity::<Identity, OFFSET>,
            NotifyAddProgress: NotifyAddProgress::<Identity, OFFSET>,
            NotifyBlockProgress: NotifyBlockProgress::<Identity, OFFSET>,
            NotifyTrackProgress: NotifyTrackProgress::<Identity, OFFSET>,
            NotifyPreparingBurn: NotifyPreparingBurn::<Identity, OFFSET>,
            NotifyClosingDisc: NotifyClosingDisc::<Identity, OFFSET>,
            NotifyBurnComplete: NotifyBurnComplete::<Identity, OFFSET>,
            NotifyEraseComplete: NotifyEraseComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscMasterProgressEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IDiscRecorder_Impl: Sized + windows_core::IUnknownImpl {
    fn Init(&self, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> windows_core::Result<()>;
    fn GetRecorderGUID(&self, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> windows_core::Result<()>;
    fn GetRecorderType(&self) -> windows_core::Result<RECORDER_TYPES>;
    fn GetDisplayNames(&self, pbstrvendorid: *mut windows_core::BSTR, pbstrproductid: *mut windows_core::BSTR, pbstrrevision: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBasePnPID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRecorderProperties(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetRecorderProperties(&self, ppropstg: Option<&super::super::System::Com::StructuredStorage::IPropertyStorage>) -> windows_core::Result<()>;
    fn GetRecorderState(&self) -> windows_core::Result<DISC_RECORDER_STATE_FLAGS>;
    fn OpenExclusive(&self) -> windows_core::Result<()>;
    fn QueryMediaType(&self, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> windows_core::Result<()>;
    fn QueryMediaInfo(&self, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> windows_core::Result<()>;
    fn Eject(&self) -> windows_core::Result<()>;
    fn Erase(&self, bfullerase: u8) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IDiscRecorder {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IDiscRecorder_Vtbl {
    pub const fn new<Identity: IDiscRecorder_Impl, const OFFSET: isize>() -> IDiscRecorder_Vtbl {
        unsafe extern "system" fn Init<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder_Impl::Init(this, core::mem::transmute_copy(&pbyuniqueid), core::mem::transmute_copy(&nulidsize), core::mem::transmute_copy(&nuldrivenumber)).into()
        }
        unsafe extern "system" fn GetRecorderGUID<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder_Impl::GetRecorderGUID(this, core::mem::transmute_copy(&pbyuniqueid), core::mem::transmute_copy(&ulbuffersize), core::mem::transmute_copy(&pulreturnsizerequired)).into()
        }
        unsafe extern "system" fn GetRecorderType<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ftypecode: *mut RECORDER_TYPES) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder_Impl::GetRecorderType(this) {
                Ok(ok__) => {
                    ftypecode.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayNames<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrvendorid: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrproductid: *mut core::mem::MaybeUninit<windows_core::BSTR>, pbstrrevision: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder_Impl::GetDisplayNames(this, core::mem::transmute_copy(&pbstrvendorid), core::mem::transmute_copy(&pbstrproductid), core::mem::transmute_copy(&pbstrrevision)).into()
        }
        unsafe extern "system" fn GetBasePnPID<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbasepnpid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder_Impl::GetBasePnPID(this) {
                Ok(ok__) => {
                    pbstrbasepnpid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder_Impl::GetPath(this) {
                Ok(ok__) => {
                    pbstrpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecorderProperties<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder_Impl::GetRecorderProperties(this) {
                Ok(ok__) => {
                    pppropstg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecorderProperties<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder_Impl::SetRecorderProperties(this, windows_core::from_raw_borrowed(&ppropstg)).into()
        }
        unsafe extern "system" fn GetRecorderState<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder_Impl::GetRecorderState(this) {
                Ok(ok__) => {
                    puldevstateflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenExclusive<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder_Impl::OpenExclusive(this).into()
        }
        unsafe extern "system" fn QueryMediaType<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder_Impl::QueryMediaType(this, core::mem::transmute_copy(&fmediatype), core::mem::transmute_copy(&fmediaflags)).into()
        }
        unsafe extern "system" fn QueryMediaInfo<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder_Impl::QueryMediaInfo(this, core::mem::transmute_copy(&pbsessions), core::mem::transmute_copy(&pblasttrack), core::mem::transmute_copy(&ulstartaddress), core::mem::transmute_copy(&ulnextwritable), core::mem::transmute_copy(&ulfreeblocks)).into()
        }
        unsafe extern "system" fn Eject<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder_Impl::Eject(this).into()
        }
        unsafe extern "system" fn Erase<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bfullerase: u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder_Impl::Erase(this, core::mem::transmute_copy(&bfullerase)).into()
        }
        unsafe extern "system" fn Close<Identity: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder_Impl::Close(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GetRecorderGUID: GetRecorderGUID::<Identity, OFFSET>,
            GetRecorderType: GetRecorderType::<Identity, OFFSET>,
            GetDisplayNames: GetDisplayNames::<Identity, OFFSET>,
            GetBasePnPID: GetBasePnPID::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            GetRecorderProperties: GetRecorderProperties::<Identity, OFFSET>,
            SetRecorderProperties: SetRecorderProperties::<Identity, OFFSET>,
            GetRecorderState: GetRecorderState::<Identity, OFFSET>,
            OpenExclusive: OpenExclusive::<Identity, OFFSET>,
            QueryMediaType: QueryMediaType::<Identity, OFFSET>,
            QueryMediaInfo: QueryMediaInfo::<Identity, OFFSET>,
            Eject: Eject::<Identity, OFFSET>,
            Erase: Erase::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscRecorder as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscRecorder2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EjectMedia(&self) -> windows_core::Result<()>;
    fn CloseTray(&self) -> windows_core::Result<()>;
    fn AcquireExclusiveAccess(&self, force: super::super::Foundation::VARIANT_BOOL, __midl__idiscrecorder20000: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReleaseExclusiveAccess(&self) -> windows_core::Result<()>;
    fn DisableMcn(&self) -> windows_core::Result<()>;
    fn EnableMcn(&self) -> windows_core::Result<()>;
    fn InitializeDiscRecorder(&self, recorderuniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ActiveDiscRecorder(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VendorId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProductId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProductRevision(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumePathNames(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn DeviceCanLoadMedia(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn LegacyDeviceNumber(&self) -> windows_core::Result<i32>;
    fn SupportedFeaturePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentFeaturePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedProfiles(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentProfiles(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedModePages(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ExclusiveAccessOwner(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IDiscRecorder2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscRecorder2_Vtbl {
    pub const fn new<Identity: IDiscRecorder2_Impl, const OFFSET: isize>() -> IDiscRecorder2_Vtbl {
        unsafe extern "system" fn EjectMedia<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2_Impl::EjectMedia(this).into()
        }
        unsafe extern "system" fn CloseTray<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2_Impl::CloseTray(this).into()
        }
        unsafe extern "system" fn AcquireExclusiveAccess<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, force: super::super::Foundation::VARIANT_BOOL, __midl__idiscrecorder20000: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2_Impl::AcquireExclusiveAccess(this, core::mem::transmute_copy(&force), core::mem::transmute(&__midl__idiscrecorder20000)).into()
        }
        unsafe extern "system" fn ReleaseExclusiveAccess<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2_Impl::ReleaseExclusiveAccess(this).into()
        }
        unsafe extern "system" fn DisableMcn<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2_Impl::DisableMcn(this).into()
        }
        unsafe extern "system" fn EnableMcn<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2_Impl::EnableMcn(this).into()
        }
        unsafe extern "system" fn InitializeDiscRecorder<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recorderuniqueid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2_Impl::InitializeDiscRecorder(this, core::mem::transmute(&recorderuniqueid)).into()
        }
        unsafe extern "system" fn ActiveDiscRecorder<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::ActiveDiscRecorder(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VendorId<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::VendorId(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductId<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::ProductId(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductRevision<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::ProductRevision(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::VolumeName(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumePathNames<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::VolumePathNames(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceCanLoadMedia<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::DeviceCanLoadMedia(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LegacyDeviceNumber<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, legacydevicenumber: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::LegacyDeviceNumber(this) {
                Ok(ok__) => {
                    legacydevicenumber.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFeaturePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::SupportedFeaturePages(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFeaturePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::CurrentFeaturePages(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedProfiles<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::SupportedProfiles(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProfiles<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::CurrentProfiles(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::SupportedModePages(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExclusiveAccessOwner<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2_Impl::ExclusiveAccessOwner(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EjectMedia: EjectMedia::<Identity, OFFSET>,
            CloseTray: CloseTray::<Identity, OFFSET>,
            AcquireExclusiveAccess: AcquireExclusiveAccess::<Identity, OFFSET>,
            ReleaseExclusiveAccess: ReleaseExclusiveAccess::<Identity, OFFSET>,
            DisableMcn: DisableMcn::<Identity, OFFSET>,
            EnableMcn: EnableMcn::<Identity, OFFSET>,
            InitializeDiscRecorder: InitializeDiscRecorder::<Identity, OFFSET>,
            ActiveDiscRecorder: ActiveDiscRecorder::<Identity, OFFSET>,
            VendorId: VendorId::<Identity, OFFSET>,
            ProductId: ProductId::<Identity, OFFSET>,
            ProductRevision: ProductRevision::<Identity, OFFSET>,
            VolumeName: VolumeName::<Identity, OFFSET>,
            VolumePathNames: VolumePathNames::<Identity, OFFSET>,
            DeviceCanLoadMedia: DeviceCanLoadMedia::<Identity, OFFSET>,
            LegacyDeviceNumber: LegacyDeviceNumber::<Identity, OFFSET>,
            SupportedFeaturePages: SupportedFeaturePages::<Identity, OFFSET>,
            CurrentFeaturePages: CurrentFeaturePages::<Identity, OFFSET>,
            SupportedProfiles: SupportedProfiles::<Identity, OFFSET>,
            CurrentProfiles: CurrentProfiles::<Identity, OFFSET>,
            SupportedModePages: SupportedModePages::<Identity, OFFSET>,
            ExclusiveAccessOwner: ExclusiveAccessOwner::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscRecorder2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IDiscRecorder2Ex_Impl: Sized + windows_core::IUnknownImpl {
    fn SendCommandNoData(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> windows_core::Result<()>;
    fn SendCommandSendDataToDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> windows_core::Result<()>;
    fn SendCommandGetDataFromDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> windows_core::Result<()>;
    fn ReadDvdStructure(&self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> windows_core::Result<()>;
    fn SendDvdStructure(&self, format: u32, data: *const u8, count: u32) -> windows_core::Result<()>;
    fn GetAdapterDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetDeviceDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetDiscInformation(&self, discinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetTrackInformation(&self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetFeaturePage(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> windows_core::Result<()>;
    fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> windows_core::Result<()>;
    fn GetSupportedFeaturePages(&self, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> windows_core::Result<()>;
    fn GetSupportedProfiles(&self, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> windows_core::Result<()>;
    fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> windows_core::Result<()>;
    fn GetByteAlignmentMask(&self) -> windows_core::Result<u32>;
    fn GetMaximumNonPageAlignedTransferSize(&self) -> windows_core::Result<u32>;
    fn GetMaximumPageAlignedTransferSize(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IDiscRecorder2Ex {}
impl IDiscRecorder2Ex_Vtbl {
    pub const fn new<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>() -> IDiscRecorder2Ex_Vtbl {
        unsafe extern "system" fn SendCommandNoData<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::SendCommandNoData(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn SendCommandSendDataToDevice<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::SendCommandSendDataToDevice(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout), core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn SendCommandGetDataFromDevice<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::SendCommandGetDataFromDevice(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout), core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&buffersize), core::mem::transmute_copy(&bufferfetched)).into()
        }
        unsafe extern "system" fn ReadDvdStructure<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::ReadDvdStructure(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&address), core::mem::transmute_copy(&layer), core::mem::transmute_copy(&agid), core::mem::transmute_copy(&data), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SendDvdStructure<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: u32, data: *const u8, count: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::SendDvdStructure(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&data), core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetAdapterDescriptor<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::GetAdapterDescriptor(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetDeviceDescriptor<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::GetDeviceDescriptor(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetDiscInformation<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::GetDiscInformation(this, core::mem::transmute_copy(&discinformation), core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetTrackInformation<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::GetTrackInformation(this, core::mem::transmute_copy(&address), core::mem::transmute_copy(&addresstype), core::mem::transmute_copy(&trackinformation), core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetFeaturePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::GetFeaturePage(this, core::mem::transmute_copy(&requestedfeature), core::mem::transmute_copy(&currentfeatureonly), core::mem::transmute_copy(&featuredata), core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetModePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::GetModePage(this, core::mem::transmute_copy(&requestedmodepage), core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&modepagedata), core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn SetModePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::SetModePage(this, core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetSupportedFeaturePages<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::GetSupportedFeaturePages(this, core::mem::transmute_copy(&currentfeatureonly), core::mem::transmute_copy(&featuredata), core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetSupportedProfiles<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::GetSupportedProfiles(this, core::mem::transmute_copy(&currentonly), core::mem::transmute_copy(&profiletypes), core::mem::transmute_copy(&validprofiles)).into()
        }
        unsafe extern "system" fn GetSupportedModePages<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDiscRecorder2Ex_Impl::GetSupportedModePages(this, core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&modepagetypes), core::mem::transmute_copy(&validpages)).into()
        }
        unsafe extern "system" fn GetByteAlignmentMask<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2Ex_Impl::GetByteAlignmentMask(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumNonPageAlignedTransferSize<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2Ex_Impl::GetMaximumNonPageAlignedTransferSize(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumPageAlignedTransferSize<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDiscRecorder2Ex_Impl::GetMaximumPageAlignedTransferSize(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendCommandNoData: SendCommandNoData::<Identity, OFFSET>,
            SendCommandSendDataToDevice: SendCommandSendDataToDevice::<Identity, OFFSET>,
            SendCommandGetDataFromDevice: SendCommandGetDataFromDevice::<Identity, OFFSET>,
            ReadDvdStructure: ReadDvdStructure::<Identity, OFFSET>,
            SendDvdStructure: SendDvdStructure::<Identity, OFFSET>,
            GetAdapterDescriptor: GetAdapterDescriptor::<Identity, OFFSET>,
            GetDeviceDescriptor: GetDeviceDescriptor::<Identity, OFFSET>,
            GetDiscInformation: GetDiscInformation::<Identity, OFFSET>,
            GetTrackInformation: GetTrackInformation::<Identity, OFFSET>,
            GetFeaturePage: GetFeaturePage::<Identity, OFFSET>,
            GetModePage: GetModePage::<Identity, OFFSET>,
            SetModePage: SetModePage::<Identity, OFFSET>,
            GetSupportedFeaturePages: GetSupportedFeaturePages::<Identity, OFFSET>,
            GetSupportedProfiles: GetSupportedProfiles::<Identity, OFFSET>,
            GetSupportedModePages: GetSupportedModePages::<Identity, OFFSET>,
            GetByteAlignmentMask: GetByteAlignmentMask::<Identity, OFFSET>,
            GetMaximumNonPageAlignedTransferSize: GetMaximumNonPageAlignedTransferSize::<Identity, OFFSET>,
            GetMaximumPageAlignedTransferSize: GetMaximumPageAlignedTransferSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscRecorder2Ex as windows_core::Interface>::IID
    }
}
pub trait IEnumDiscMasterFormats_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, cformats: u32, lpiidformatid: *mut windows_core::GUID, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, cformats: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDiscMasterFormats>;
}
impl windows_core::RuntimeName for IEnumDiscMasterFormats {}
impl IEnumDiscMasterFormats_Vtbl {
    pub const fn new<Identity: IEnumDiscMasterFormats_Impl, const OFFSET: isize>() -> IEnumDiscMasterFormats_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cformats: u32, lpiidformatid: *mut windows_core::GUID, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDiscMasterFormats_Impl::Next(this, core::mem::transmute_copy(&cformats), core::mem::transmute_copy(&lpiidformatid), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cformats: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDiscMasterFormats_Impl::Skip(this, core::mem::transmute_copy(&cformats)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDiscMasterFormats_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumDiscMasterFormats_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDiscMasterFormats as windows_core::Interface>::IID
    }
}
pub trait IEnumDiscRecorders_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, crecorders: u32, pprecorder: *mut Option<IDiscRecorder>, pcfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, crecorders: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumDiscRecorders>;
}
impl windows_core::RuntimeName for IEnumDiscRecorders {}
impl IEnumDiscRecorders_Vtbl {
    pub const fn new<Identity: IEnumDiscRecorders_Impl, const OFFSET: isize>() -> IEnumDiscRecorders_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crecorders: u32, pprecorder: *mut *mut core::ffi::c_void, pcfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDiscRecorders_Impl::Next(this, core::mem::transmute_copy(&crecorders), core::mem::transmute_copy(&pprecorder), core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crecorders: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDiscRecorders_Impl::Skip(this, core::mem::transmute_copy(&crecorders)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumDiscRecorders_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumDiscRecorders_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumDiscRecorders as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumFsiItems_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<IFsiItem>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumFsiItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumFsiItems {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumFsiItems_Vtbl {
    pub const fn new<Identity: IEnumFsiItems_Impl, const OFFSET: isize>() -> IEnumFsiItems_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFsiItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFsiItems_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumFsiItems_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumFsiItems_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumFsiItems as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumProgressItems_Impl: Sized + windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<IProgressItem>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumProgressItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IEnumProgressItems {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumProgressItems_Vtbl {
    pub const fn new<Identity: IEnumProgressItems_Impl, const OFFSET: isize>() -> IEnumProgressItems_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumProgressItems_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumProgressItems_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumProgressItems_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumProgressItems_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumProgressItems as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Root(&self) -> windows_core::Result<IFsiDirectoryItem>;
    fn SessionStartBlock(&self) -> windows_core::Result<i32>;
    fn SetSessionStartBlock(&self, newval: i32) -> windows_core::Result<()>;
    fn FreeMediaBlocks(&self) -> windows_core::Result<i32>;
    fn SetFreeMediaBlocks(&self, newval: i32) -> windows_core::Result<()>;
    fn SetMaxMediaBlocksFromDevice(&self, discrecorder: Option<&IDiscRecorder2>) -> windows_core::Result<()>;
    fn UsedBlocks(&self) -> windows_core::Result<i32>;
    fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetVolumeName(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ImportedVolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn BootImageOptions(&self) -> windows_core::Result<IBootOptions>;
    fn SetBootImageOptions(&self, newval: Option<&IBootOptions>) -> windows_core::Result<()>;
    fn FileCount(&self) -> windows_core::Result<i32>;
    fn DirectoryCount(&self) -> windows_core::Result<i32>;
    fn WorkingDirectory(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetWorkingDirectory(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ChangePoint(&self) -> windows_core::Result<i32>;
    fn StrictFileSystemCompliance(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStrictFileSystemCompliance(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseRestrictedCharacterSet(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseRestrictedCharacterSet(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FileSystemsToCreate(&self) -> windows_core::Result<FsiFileSystems>;
    fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> windows_core::Result<()>;
    fn FileSystemsSupported(&self) -> windows_core::Result<FsiFileSystems>;
    fn SetUDFRevision(&self, newval: i32) -> windows_core::Result<()>;
    fn UDFRevision(&self) -> windows_core::Result<i32>;
    fn UDFRevisionsSupported(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ChooseImageDefaults(&self, discrecorder: Option<&IDiscRecorder2>) -> windows_core::Result<()>;
    fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::Result<()>;
    fn SetISO9660InterchangeLevel(&self, newval: i32) -> windows_core::Result<()>;
    fn ISO9660InterchangeLevel(&self) -> windows_core::Result<i32>;
    fn ISO9660InterchangeLevelsSupported(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateResultImage(&self) -> windows_core::Result<IFileSystemImageResult>;
    fn Exists(&self, fullpath: &windows_core::BSTR) -> windows_core::Result<FsiItemType>;
    fn CalculateDiscIdentifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IdentifyFileSystemsOnDisc(&self, discrecorder: Option<&IDiscRecorder2>) -> windows_core::Result<FsiFileSystems>;
    fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> windows_core::Result<FsiFileSystems>;
    fn ImportFileSystem(&self) -> windows_core::Result<FsiFileSystems>;
    fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> windows_core::Result<()>;
    fn RollbackToChangePoint(&self, changepoint: i32) -> windows_core::Result<()>;
    fn LockInChangePoint(&self) -> windows_core::Result<()>;
    fn CreateDirectoryItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiDirectoryItem>;
    fn CreateFileItem(&self, name: &windows_core::BSTR) -> windows_core::Result<IFsiFileItem>;
    fn VolumeNameUDF(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeNameJoliet(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeNameISO9660(&self) -> windows_core::Result<windows_core::BSTR>;
    fn StageFiles(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStageFiles(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFileSystemImage {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImage_Vtbl {
    pub const fn new<Identity: IFileSystemImage_Impl, const OFFSET: isize>() -> IFileSystemImage_Vtbl {
        unsafe extern "system" fn Root<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::Root(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStartBlock<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::SessionStartBlock(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionStartBlock<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetSessionStartBlock(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FreeMediaBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::FreeMediaBlocks(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFreeMediaBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetFreeMediaBlocks(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SetMaxMediaBlocksFromDevice<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discrecorder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetMaxMediaBlocksFromDevice(this, windows_core::from_raw_borrowed(&discrecorder)).into()
        }
        unsafe extern "system" fn UsedBlocks<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::UsedBlocks(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::VolumeName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetVolumeName(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn ImportedVolumeName<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::ImportedVolumeName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BootImageOptions<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::BootImageOptions(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBootImageOptions<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetBootImageOptions(this, windows_core::from_raw_borrowed(&newval)).into()
        }
        unsafe extern "system" fn FileCount<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::FileCount(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryCount<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::DirectoryCount(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkingDirectory<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::WorkingDirectory(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetWorkingDirectory(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn ChangePoint<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::ChangePoint(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrictFileSystemCompliance<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::StrictFileSystemCompliance(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrictFileSystemCompliance<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetStrictFileSystemCompliance(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn UseRestrictedCharacterSet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::UseRestrictedCharacterSet(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseRestrictedCharacterSet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetUseRestrictedCharacterSet(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemsToCreate<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsiFileSystems) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::FileSystemsToCreate(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileSystemsToCreate<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: FsiFileSystems) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetFileSystemsToCreate(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut FsiFileSystems) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::FileSystemsSupported(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUDFRevision<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetUDFRevision(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn UDFRevision<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::UDFRevision(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UDFRevisionsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::UDFRevisionsSupported(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChooseImageDefaults<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discrecorder: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::ChooseImageDefaults(this, windows_core::from_raw_borrowed(&discrecorder)).into()
        }
        unsafe extern "system" fn ChooseImageDefaultsForMediaType<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::ChooseImageDefaultsForMediaType(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetISO9660InterchangeLevel<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetISO9660InterchangeLevel(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ISO9660InterchangeLevel<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::ISO9660InterchangeLevel(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ISO9660InterchangeLevelsSupported<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::ISO9660InterchangeLevelsSupported(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResultImage<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::CreateResultImage(this) {
                Ok(ok__) => {
                    resultstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Exists<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fullpath: core::mem::MaybeUninit<windows_core::BSTR>, itemtype: *mut FsiItemType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::Exists(this, core::mem::transmute(&fullpath)) {
                Ok(ok__) => {
                    itemtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalculateDiscIdentifier<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discidentifier: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::CalculateDiscIdentifier(this) {
                Ok(ok__) => {
                    discidentifier.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdentifyFileSystemsOnDisc<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discrecorder: *mut core::ffi::c_void, filesystems: *mut FsiFileSystems) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::IdentifyFileSystemsOnDisc(this, windows_core::from_raw_borrowed(&discrecorder)) {
                Ok(ok__) => {
                    filesystems.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFileSystemForImport<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::GetDefaultFileSystemForImport(this, core::mem::transmute_copy(&filesystems)) {
                Ok(ok__) => {
                    importdefault.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportFileSystem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, importedfilesystem: *mut FsiFileSystems) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::ImportFileSystem(this) {
                Ok(ok__) => {
                    importedfilesystem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportSpecificFileSystem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystemtouse: FsiFileSystems) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::ImportSpecificFileSystem(this, core::mem::transmute_copy(&filesystemtouse)).into()
        }
        unsafe extern "system" fn RollbackToChangePoint<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, changepoint: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::RollbackToChangePoint(this, core::mem::transmute_copy(&changepoint)).into()
        }
        unsafe extern "system" fn LockInChangePoint<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::LockInChangePoint(this).into()
        }
        unsafe extern "system" fn CreateDirectoryItem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, newitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::CreateDirectoryItem(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    newitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileItem<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, newitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::CreateFileItem(this, core::mem::transmute(&name)) {
                Ok(ok__) => {
                    newitem.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameUDF<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::VolumeNameUDF(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameJoliet<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::VolumeNameJoliet(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameISO9660<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::VolumeNameISO9660(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StageFiles<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::StageFiles(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStageFiles<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetStageFiles(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage_Impl::MultisessionInterfaces(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultisessionInterfaces<Identity: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage_Impl::SetMultisessionInterfaces(this, core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Root: Root::<Identity, OFFSET>,
            SessionStartBlock: SessionStartBlock::<Identity, OFFSET>,
            SetSessionStartBlock: SetSessionStartBlock::<Identity, OFFSET>,
            FreeMediaBlocks: FreeMediaBlocks::<Identity, OFFSET>,
            SetFreeMediaBlocks: SetFreeMediaBlocks::<Identity, OFFSET>,
            SetMaxMediaBlocksFromDevice: SetMaxMediaBlocksFromDevice::<Identity, OFFSET>,
            UsedBlocks: UsedBlocks::<Identity, OFFSET>,
            VolumeName: VolumeName::<Identity, OFFSET>,
            SetVolumeName: SetVolumeName::<Identity, OFFSET>,
            ImportedVolumeName: ImportedVolumeName::<Identity, OFFSET>,
            BootImageOptions: BootImageOptions::<Identity, OFFSET>,
            SetBootImageOptions: SetBootImageOptions::<Identity, OFFSET>,
            FileCount: FileCount::<Identity, OFFSET>,
            DirectoryCount: DirectoryCount::<Identity, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, OFFSET>,
            ChangePoint: ChangePoint::<Identity, OFFSET>,
            StrictFileSystemCompliance: StrictFileSystemCompliance::<Identity, OFFSET>,
            SetStrictFileSystemCompliance: SetStrictFileSystemCompliance::<Identity, OFFSET>,
            UseRestrictedCharacterSet: UseRestrictedCharacterSet::<Identity, OFFSET>,
            SetUseRestrictedCharacterSet: SetUseRestrictedCharacterSet::<Identity, OFFSET>,
            FileSystemsToCreate: FileSystemsToCreate::<Identity, OFFSET>,
            SetFileSystemsToCreate: SetFileSystemsToCreate::<Identity, OFFSET>,
            FileSystemsSupported: FileSystemsSupported::<Identity, OFFSET>,
            SetUDFRevision: SetUDFRevision::<Identity, OFFSET>,
            UDFRevision: UDFRevision::<Identity, OFFSET>,
            UDFRevisionsSupported: UDFRevisionsSupported::<Identity, OFFSET>,
            ChooseImageDefaults: ChooseImageDefaults::<Identity, OFFSET>,
            ChooseImageDefaultsForMediaType: ChooseImageDefaultsForMediaType::<Identity, OFFSET>,
            SetISO9660InterchangeLevel: SetISO9660InterchangeLevel::<Identity, OFFSET>,
            ISO9660InterchangeLevel: ISO9660InterchangeLevel::<Identity, OFFSET>,
            ISO9660InterchangeLevelsSupported: ISO9660InterchangeLevelsSupported::<Identity, OFFSET>,
            CreateResultImage: CreateResultImage::<Identity, OFFSET>,
            Exists: Exists::<Identity, OFFSET>,
            CalculateDiscIdentifier: CalculateDiscIdentifier::<Identity, OFFSET>,
            IdentifyFileSystemsOnDisc: IdentifyFileSystemsOnDisc::<Identity, OFFSET>,
            GetDefaultFileSystemForImport: GetDefaultFileSystemForImport::<Identity, OFFSET>,
            ImportFileSystem: ImportFileSystem::<Identity, OFFSET>,
            ImportSpecificFileSystem: ImportSpecificFileSystem::<Identity, OFFSET>,
            RollbackToChangePoint: RollbackToChangePoint::<Identity, OFFSET>,
            LockInChangePoint: LockInChangePoint::<Identity, OFFSET>,
            CreateDirectoryItem: CreateDirectoryItem::<Identity, OFFSET>,
            CreateFileItem: CreateFileItem::<Identity, OFFSET>,
            VolumeNameUDF: VolumeNameUDF::<Identity, OFFSET>,
            VolumeNameJoliet: VolumeNameJoliet::<Identity, OFFSET>,
            VolumeNameISO9660: VolumeNameISO9660::<Identity, OFFSET>,
            StageFiles: StageFiles::<Identity, OFFSET>,
            SetStageFiles: SetStageFiles::<Identity, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, OFFSET>,
            SetMultisessionInterfaces: SetMultisessionInterfaces::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage2_Impl: Sized + IFileSystemImage_Impl {
    fn BootImageOptionsArray(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetBootImageOptionsArray(&self, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFileSystemImage2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImage2_Vtbl {
    pub const fn new<Identity: IFileSystemImage2_Impl, const OFFSET: isize>() -> IFileSystemImage2_Vtbl {
        unsafe extern "system" fn BootImageOptionsArray<Identity: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage2_Impl::BootImageOptionsArray(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBootImageOptionsArray<Identity: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage2_Impl::SetBootImageOptionsArray(this, core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base__: IFileSystemImage_Vtbl::new::<Identity, OFFSET>(),
            BootImageOptionsArray: BootImageOptionsArray::<Identity, OFFSET>,
            SetBootImageOptionsArray: SetBootImageOptionsArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage3_Impl: Sized + IFileSystemImage2_Impl {
    fn CreateRedundantUdfMetadataFiles(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetCreateRedundantUdfMetadataFiles(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFileSystemImage3 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImage3_Vtbl {
    pub const fn new<Identity: IFileSystemImage3_Impl, const OFFSET: isize>() -> IFileSystemImage3_Vtbl {
        unsafe extern "system" fn CreateRedundantUdfMetadataFiles<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage3_Impl::CreateRedundantUdfMetadataFiles(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateRedundantUdfMetadataFiles<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFileSystemImage3_Impl::SetCreateRedundantUdfMetadataFiles(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ProbeSpecificFileSystem<Identity: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImage3_Impl::ProbeSpecificFileSystem(this, core::mem::transmute_copy(&filesystemtoprobe)) {
                Ok(ok__) => {
                    isappendable.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IFileSystemImage2_Vtbl::new::<Identity, OFFSET>(),
            CreateRedundantUdfMetadataFiles: CreateRedundantUdfMetadataFiles::<Identity, OFFSET>,
            SetCreateRedundantUdfMetadataFiles: SetCreateRedundantUdfMetadataFiles::<Identity, OFFSET>,
            ProbeSpecificFileSystem: ProbeSpecificFileSystem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImage3 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImage as windows_core::Interface>::IID || iid == &<IFileSystemImage2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImageResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ImageStream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn ProgressItems(&self) -> windows_core::Result<IProgressItems>;
    fn TotalBlocks(&self) -> windows_core::Result<i32>;
    fn BlockSize(&self) -> windows_core::Result<i32>;
    fn DiscId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFileSystemImageResult {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImageResult_Vtbl {
    pub const fn new<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>() -> IFileSystemImageResult_Vtbl {
        unsafe extern "system" fn ImageStream<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImageResult_Impl::ImageStream(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItems<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImageResult_Impl::ProgressItems(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBlocks<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImageResult_Impl::TotalBlocks(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockSize<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImageResult_Impl::BlockSize(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscId<Identity: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImageResult_Impl::DiscId(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ImageStream: ImageStream::<Identity, OFFSET>,
            ProgressItems: ProgressItems::<Identity, OFFSET>,
            TotalBlocks: TotalBlocks::<Identity, OFFSET>,
            BlockSize: BlockSize::<Identity, OFFSET>,
            DiscId: DiscId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImageResult as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImageResult2_Impl: Sized + IFileSystemImageResult_Impl {
    fn ModifiedBlocks(&self) -> windows_core::Result<IBlockRangeList>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFileSystemImageResult2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImageResult2_Vtbl {
    pub const fn new<Identity: IFileSystemImageResult2_Impl, const OFFSET: isize>() -> IFileSystemImageResult2_Vtbl {
        unsafe extern "system" fn ModifiedBlocks<Identity: IFileSystemImageResult2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFileSystemImageResult2_Impl::ModifiedBlocks(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IFileSystemImageResult_Vtbl::new::<Identity, OFFSET>(), ModifiedBlocks: ModifiedBlocks::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileSystemImageResult2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFileSystemImageResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiDirectoryItem_Impl: Sized + IFsiItem_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, path: &windows_core::BSTR) -> windows_core::Result<IFsiItem>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn EnumFsiItems(&self) -> windows_core::Result<IEnumFsiItems>;
    fn AddDirectory(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddFile(&self, path: &windows_core::BSTR, filedata: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn AddTree(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Add(&self, item: Option<&IFsiItem>) -> windows_core::Result<()>;
    fn Remove(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveTree(&self, path: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiDirectoryItem {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiDirectoryItem_Vtbl {
    pub const fn new<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>() -> IFsiDirectoryItem_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiDirectoryItem_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiDirectoryItem_Impl::get_Item(this, core::mem::transmute(&path)) {
                Ok(ok__) => {
                    item.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiDirectoryItem_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFsiItems<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiDirectoryItem_Impl::EnumFsiItems(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirectory<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiDirectoryItem_Impl::AddDirectory(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn AddFile<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>, filedata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiDirectoryItem_Impl::AddFile(this, core::mem::transmute(&path), windows_core::from_raw_borrowed(&filedata)).into()
        }
        unsafe extern "system" fn AddTree<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcedirectory: core::mem::MaybeUninit<windows_core::BSTR>, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiDirectoryItem_Impl::AddTree(this, core::mem::transmute(&sourcedirectory), core::mem::transmute_copy(&includebasedirectory)).into()
        }
        unsafe extern "system" fn Add<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, item: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiDirectoryItem_Impl::Add(this, windows_core::from_raw_borrowed(&item)).into()
        }
        unsafe extern "system" fn Remove<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiDirectoryItem_Impl::Remove(this, core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn RemoveTree<Identity: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiDirectoryItem_Impl::RemoveTree(this, core::mem::transmute(&path)).into()
        }
        Self {
            base__: IFsiItem_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            EnumFsiItems: EnumFsiItems::<Identity, OFFSET>,
            AddDirectory: AddDirectory::<Identity, OFFSET>,
            AddFile: AddFile::<Identity, OFFSET>,
            AddTree: AddTree::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveTree: RemoveTree::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiDirectoryItem as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiDirectoryItem2_Impl: Sized + IFsiDirectoryItem_Impl {
    fn AddTreeWithNamedStreams(&self, sourcedirectory: &windows_core::BSTR, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiDirectoryItem2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiDirectoryItem2_Vtbl {
    pub const fn new<Identity: IFsiDirectoryItem2_Impl, const OFFSET: isize>() -> IFsiDirectoryItem2_Vtbl {
        unsafe extern "system" fn AddTreeWithNamedStreams<Identity: IFsiDirectoryItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourcedirectory: core::mem::MaybeUninit<windows_core::BSTR>, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiDirectoryItem2_Impl::AddTreeWithNamedStreams(this, core::mem::transmute(&sourcedirectory), core::mem::transmute_copy(&includebasedirectory)).into()
        }
        Self { base__: IFsiDirectoryItem_Vtbl::new::<Identity, OFFSET>(), AddTreeWithNamedStreams: AddTreeWithNamedStreams::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiDirectoryItem2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<IFsiDirectoryItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiFileItem_Impl: Sized + IFsiItem_Impl {
    fn DataSize(&self) -> windows_core::Result<i64>;
    fn DataSize32BitLow(&self) -> windows_core::Result<i32>;
    fn DataSize32BitHigh(&self) -> windows_core::Result<i32>;
    fn Data(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetData(&self, newval: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiFileItem {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiFileItem_Vtbl {
    pub const fn new<Identity: IFsiFileItem_Impl, const OFFSET: isize>() -> IFsiFileItem_Vtbl {
        unsafe extern "system" fn DataSize<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiFileItem_Impl::DataSize(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSize32BitLow<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiFileItem_Impl::DataSize32BitLow(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSize32BitHigh<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiFileItem_Impl::DataSize32BitHigh(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiFileItem_Impl::Data(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiFileItem_Impl::SetData(this, windows_core::from_raw_borrowed(&newval)).into()
        }
        Self {
            base__: IFsiItem_Vtbl::new::<Identity, OFFSET>(),
            DataSize: DataSize::<Identity, OFFSET>,
            DataSize32BitLow: DataSize32BitLow::<Identity, OFFSET>,
            DataSize32BitHigh: DataSize32BitHigh::<Identity, OFFSET>,
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiFileItem as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiFileItem2_Impl: Sized + IFsiFileItem_Impl {
    fn FsiNamedStreams(&self) -> windows_core::Result<IFsiNamedStreams>;
    fn IsNamedStream(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AddStream(&self, name: &windows_core::BSTR, streamdata: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn RemoveStream(&self, name: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsRealTime(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsRealTime(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiFileItem2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiFileItem2_Vtbl {
    pub const fn new<Identity: IFsiFileItem2_Impl, const OFFSET: isize>() -> IFsiFileItem2_Vtbl {
        unsafe extern "system" fn FsiNamedStreams<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiFileItem2_Impl::FsiNamedStreams(this) {
                Ok(ok__) => {
                    streams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNamedStream<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiFileItem2_Impl::IsNamedStream(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStream<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, streamdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiFileItem2_Impl::AddStream(this, core::mem::transmute(&name), windows_core::from_raw_borrowed(&streamdata)).into()
        }
        unsafe extern "system" fn RemoveStream<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiFileItem2_Impl::RemoveStream(this, core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn IsRealTime<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiFileItem2_Impl::IsRealTime(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRealTime<Identity: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiFileItem2_Impl::SetIsRealTime(this, core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base__: IFsiFileItem_Vtbl::new::<Identity, OFFSET>(),
            FsiNamedStreams: FsiNamedStreams::<Identity, OFFSET>,
            IsNamedStream: IsNamedStream::<Identity, OFFSET>,
            AddStream: AddStream::<Identity, OFFSET>,
            RemoveStream: RemoveStream::<Identity, OFFSET>,
            IsRealTime: IsRealTime::<Identity, OFFSET>,
            SetIsRealTime: SetIsRealTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiFileItem2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<IFsiFileItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FullPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CreationTime(&self) -> windows_core::Result<f64>;
    fn SetCreationTime(&self, newval: f64) -> windows_core::Result<()>;
    fn LastAccessedTime(&self) -> windows_core::Result<f64>;
    fn SetLastAccessedTime(&self, newval: f64) -> windows_core::Result<()>;
    fn LastModifiedTime(&self) -> windows_core::Result<f64>;
    fn SetLastModifiedTime(&self, newval: f64) -> windows_core::Result<()>;
    fn IsHidden(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsHidden(&self, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FileSystemName(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR>;
    fn FileSystemPath(&self, filesystem: FsiFileSystems) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiItem {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiItem_Vtbl {
    pub const fn new<Identity: IFsiItem_Impl, const OFFSET: isize>() -> IFsiItem_Vtbl {
        unsafe extern "system" fn Name<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiItem_Impl::Name(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullPath<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiItem_Impl::FullPath(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiItem_Impl::CreationTime(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreationTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiItem_Impl::SetCreationTime(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn LastAccessedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiItem_Impl::LastAccessedTime(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastAccessedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiItem_Impl::SetLastAccessedTime(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn LastModifiedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiItem_Impl::LastModifiedTime(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModifiedTime<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiItem_Impl::SetLastModifiedTime(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn IsHidden<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiItem_Impl::IsHidden(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHidden<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IFsiItem_Impl::SetIsHidden(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemName<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiItem_Impl::FileSystemName(this, core::mem::transmute_copy(&filesystem)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemPath<Identity: IFsiItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiItem_Impl::FileSystemPath(this, core::mem::transmute_copy(&filesystem)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            FullPath: FullPath::<Identity, OFFSET>,
            CreationTime: CreationTime::<Identity, OFFSET>,
            SetCreationTime: SetCreationTime::<Identity, OFFSET>,
            LastAccessedTime: LastAccessedTime::<Identity, OFFSET>,
            SetLastAccessedTime: SetLastAccessedTime::<Identity, OFFSET>,
            LastModifiedTime: LastModifiedTime::<Identity, OFFSET>,
            SetLastModifiedTime: SetLastModifiedTime::<Identity, OFFSET>,
            IsHidden: IsHidden::<Identity, OFFSET>,
            SetIsHidden: SetIsHidden::<Identity, OFFSET>,
            FileSystemName: FileSystemName::<Identity, OFFSET>,
            FileSystemPath: FileSystemPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiItem as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiNamedStreams_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, index: i32) -> windows_core::Result<IFsiFileItem2>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn EnumNamedStreams(&self) -> windows_core::Result<IEnumFsiItems>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IFsiNamedStreams {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiNamedStreams_Vtbl {
    pub const fn new<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>() -> IFsiNamedStreams_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiNamedStreams_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiNamedStreams_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    item.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiNamedStreams_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumNamedStreams<Identity: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IFsiNamedStreams_Impl::EnumNamedStreams(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            EnumNamedStreams: EnumNamedStreams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFsiNamedStreams as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIsoImageManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Path(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Stream(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn SetPath(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetStream(&self, data: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Validate(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IIsoImageManager {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IIsoImageManager_Vtbl {
    pub const fn new<Identity: IIsoImageManager_Impl, const OFFSET: isize>() -> IIsoImageManager_Vtbl {
        unsafe extern "system" fn Path<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIsoImageManager_Impl::Path(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IIsoImageManager_Impl::Stream(this) {
                Ok(ok__) => {
                    data.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIsoImageManager_Impl::SetPath(this, core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn SetStream<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIsoImageManager_Impl::SetStream(this, windows_core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn Validate<Identity: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IIsoImageManager_Impl::Validate(this).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Path: Path::<Identity, OFFSET>,
            Stream: Stream::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
            SetStream: SetStream::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IIsoImageManager as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IJolietDiscMaster_Impl: Sized + windows_core::IUnknownImpl {
    fn GetTotalDataBlocks(&self) -> windows_core::Result<i32>;
    fn GetUsedDataBlocks(&self) -> windows_core::Result<i32>;
    fn GetDataBlockSize(&self) -> windows_core::Result<i32>;
    fn AddData(&self, pstorage: Option<&super::super::System::Com::StructuredStorage::IStorage>, lfileoverwrite: i32) -> windows_core::Result<()>;
    fn GetJolietProperties(&self) -> windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetJolietProperties(&self, ppropstg: Option<&super::super::System::Com::StructuredStorage::IPropertyStorage>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl windows_core::RuntimeName for IJolietDiscMaster {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IJolietDiscMaster_Vtbl {
    pub const fn new<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>() -> IJolietDiscMaster_Vtbl {
        unsafe extern "system" fn GetTotalDataBlocks<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblocks: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IJolietDiscMaster_Impl::GetTotalDataBlocks(this) {
                Ok(ok__) => {
                    pnblocks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsedDataBlocks<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblocks: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IJolietDiscMaster_Impl::GetUsedDataBlocks(this) {
                Ok(ok__) => {
                    pnblocks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataBlockSize<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblockbytes: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IJolietDiscMaster_Impl::GetDataBlockSize(this) {
                Ok(ok__) => {
                    pnblockbytes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddData<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstorage: *mut core::ffi::c_void, lfileoverwrite: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IJolietDiscMaster_Impl::AddData(this, windows_core::from_raw_borrowed(&pstorage), core::mem::transmute_copy(&lfileoverwrite)).into()
        }
        unsafe extern "system" fn GetJolietProperties<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IJolietDiscMaster_Impl::GetJolietProperties(this) {
                Ok(ok__) => {
                    pppropstg.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJolietProperties<Identity: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IJolietDiscMaster_Impl::SetJolietProperties(this, windows_core::from_raw_borrowed(&ppropstg)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTotalDataBlocks: GetTotalDataBlocks::<Identity, OFFSET>,
            GetUsedDataBlocks: GetUsedDataBlocks::<Identity, OFFSET>,
            GetDataBlockSize: GetDataBlockSize::<Identity, OFFSET>,
            AddData: AddData::<Identity, OFFSET>,
            GetJolietProperties: GetJolietProperties::<Identity, OFFSET>,
            SetJolietProperties: SetJolietProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IJolietDiscMaster as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisession_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsSupportedOnCurrentMediaState(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetInUse(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn InUse(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ImportRecorder(&self) -> windows_core::Result<IDiscRecorder2>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMultisession {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisession_Vtbl {
    pub const fn new<Identity: IMultisession_Impl, const OFFSET: isize>() -> IMultisession_Vtbl {
        unsafe extern "system" fn IsSupportedOnCurrentMediaState<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisession_Impl::IsSupportedOnCurrentMediaState(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInUse<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IMultisession_Impl::SetInUse(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn InUse<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisession_Impl::InUse(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportRecorder<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisession_Impl::ImportRecorder(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsSupportedOnCurrentMediaState: IsSupportedOnCurrentMediaState::<Identity, OFFSET>,
            SetInUse: SetInUse::<Identity, OFFSET>,
            InUse: InUse::<Identity, OFFSET>,
            ImportRecorder: ImportRecorder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisession as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionRandomWrite_Impl: Sized + IMultisession_Impl {
    fn WriteUnitSize(&self) -> windows_core::Result<i32>;
    fn LastWrittenAddress(&self) -> windows_core::Result<i32>;
    fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMultisessionRandomWrite {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisessionRandomWrite_Vtbl {
    pub const fn new<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>() -> IMultisessionRandomWrite_Vtbl {
        unsafe extern "system" fn WriteUnitSize<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisessionRandomWrite_Impl::WriteUnitSize(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddress<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisessionRandomWrite_Impl::LastWrittenAddress(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisessionRandomWrite_Impl::TotalSectorsOnMedia(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IMultisession_Vtbl::new::<Identity, OFFSET>(),
            WriteUnitSize: WriteUnitSize::<Identity, OFFSET>,
            LastWrittenAddress: LastWrittenAddress::<Identity, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionRandomWrite as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionSequential_Impl: Sized + IMultisession_Impl {
    fn IsFirstDataSession(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn NextWritableAddress(&self) -> windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMultisessionSequential {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisessionSequential_Vtbl {
    pub const fn new<Identity: IMultisessionSequential_Impl, const OFFSET: isize>() -> IMultisessionSequential_Vtbl {
        unsafe extern "system" fn IsFirstDataSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisessionSequential_Impl::IsFirstDataSession(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisessionSequential_Impl::StartAddressOfPreviousSession(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisessionSequential_Impl::LastWrittenAddressOfPreviousSession(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextWritableAddress<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisessionSequential_Impl::NextWritableAddress(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisessionSequential_Impl::FreeSectorsOnMedia(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IMultisession_Vtbl::new::<Identity, OFFSET>(),
            IsFirstDataSession: IsFirstDataSession::<Identity, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionSequential as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionSequential2_Impl: Sized + IMultisessionSequential_Impl {
    fn WriteUnitSize(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IMultisessionSequential2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisessionSequential2_Vtbl {
    pub const fn new<Identity: IMultisessionSequential2_Impl, const OFFSET: isize>() -> IMultisessionSequential2_Vtbl {
        unsafe extern "system" fn WriteUnitSize<Identity: IMultisessionSequential2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IMultisessionSequential2_Impl::WriteUnitSize(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: IMultisessionSequential_Vtbl::new::<Identity, OFFSET>(), WriteUnitSize: WriteUnitSize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionSequential2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID || iid == &<IMultisessionSequential as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProgressItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FirstBlock(&self) -> windows_core::Result<u32>;
    fn LastBlock(&self) -> windows_core::Result<u32>;
    fn BlockCount(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IProgressItem {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IProgressItem_Vtbl {
    pub const fn new<Identity: IProgressItem_Impl, const OFFSET: isize>() -> IProgressItem_Vtbl {
        unsafe extern "system" fn Description<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desc: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProgressItem_Impl::Description(this) {
                Ok(ok__) => {
                    desc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstBlock<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProgressItem_Impl::FirstBlock(this) {
                Ok(ok__) => {
                    block.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBlock<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProgressItem_Impl::LastBlock(this) {
                Ok(ok__) => {
                    block.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockCount<Identity: IProgressItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, blocks: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProgressItem_Impl::BlockCount(this) {
                Ok(ok__) => {
                    blocks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Description: Description::<Identity, OFFSET>,
            FirstBlock: FirstBlock::<Identity, OFFSET>,
            LastBlock: LastBlock::<Identity, OFFSET>,
            BlockCount: BlockCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressItem as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProgressItems_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, index: i32) -> windows_core::Result<IProgressItem>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn ProgressItemFromBlock(&self, block: u32) -> windows_core::Result<IProgressItem>;
    fn ProgressItemFromDescription(&self, description: &windows_core::BSTR) -> windows_core::Result<IProgressItem>;
    fn EnumProgressItems(&self) -> windows_core::Result<IEnumProgressItems>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IProgressItems {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IProgressItems_Vtbl {
    pub const fn new<Identity: IProgressItems_Impl, const OFFSET: isize>() -> IProgressItems_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProgressItems_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProgressItems_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    item.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProgressItems_Impl::Count(this) {
                Ok(ok__) => {
                    count.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItemFromBlock<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, block: u32, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProgressItems_Impl::ProgressItemFromBlock(this, core::mem::transmute_copy(&block)) {
                Ok(ok__) => {
                    item.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItemFromDescription<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: core::mem::MaybeUninit<windows_core::BSTR>, item: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProgressItems_Impl::ProgressItemFromDescription(this, core::mem::transmute(&description)) {
                Ok(ok__) => {
                    item.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProgressItems<Identity: IProgressItems_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IProgressItems_Impl::EnumProgressItems(this) {
                Ok(ok__) => {
                    newenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            ProgressItemFromBlock: ProgressItemFromBlock::<Identity, OFFSET>,
            ProgressItemFromDescription: ProgressItemFromDescription::<Identity, OFFSET>,
            EnumProgressItems: EnumProgressItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressItems as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawCDImageCreator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateResultImage(&self) -> windows_core::Result<super::super::System::Com::IStream>;
    fn AddTrack(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: Option<&super::super::System::Com::IStream>) -> windows_core::Result<i32>;
    fn AddSpecialPregap(&self, data: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn AddSubcodeRWGenerator(&self, subcode: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn SetResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()>;
    fn ResultingImageType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn StartOfLeadout(&self) -> windows_core::Result<i32>;
    fn SetStartOfLeadoutLimit(&self, value: i32) -> windows_core::Result<()>;
    fn StartOfLeadoutLimit(&self) -> windows_core::Result<i32>;
    fn SetDisableGaplessAudio(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DisableGaplessAudio(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMediaCatalogNumber(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MediaCatalogNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStartingTrackNumber(&self, value: i32) -> windows_core::Result<()>;
    fn StartingTrackNumber(&self) -> windows_core::Result<i32>;
    fn get_TrackInfo(&self, trackindex: i32) -> windows_core::Result<IRawCDImageTrackInfo>;
    fn NumberOfExistingTracks(&self) -> windows_core::Result<i32>;
    fn LastUsedUserSectorInImage(&self) -> windows_core::Result<i32>;
    fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRawCDImageCreator {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRawCDImageCreator_Vtbl {
    pub const fn new<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>() -> IRawCDImageCreator_Vtbl {
        unsafe extern "system" fn CreateResultImage<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::CreateResultImage(this) {
                Ok(ok__) => {
                    resultstream.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrack<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: *mut core::ffi::c_void, trackindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::AddTrack(this, core::mem::transmute_copy(&datatype), windows_core::from_raw_borrowed(&data)) {
                Ok(ok__) => {
                    trackindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSpecialPregap<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageCreator_Impl::AddSpecialPregap(this, windows_core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn AddSubcodeRWGenerator<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subcode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageCreator_Impl::AddSubcodeRWGenerator(this, windows_core::from_raw_borrowed(&subcode)).into()
        }
        unsafe extern "system" fn SetResultingImageType<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageCreator_Impl::SetResultingImageType(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ResultingImageType<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::ResultingImageType(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOfLeadout<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::StartOfLeadout(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartOfLeadoutLimit<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageCreator_Impl::SetStartOfLeadoutLimit(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartOfLeadoutLimit<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::StartOfLeadoutLimit(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableGaplessAudio<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageCreator_Impl::SetDisableGaplessAudio(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DisableGaplessAudio<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::DisableGaplessAudio(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaCatalogNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageCreator_Impl::SetMediaCatalogNumber(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn MediaCatalogNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::MediaCatalogNumber(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartingTrackNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageCreator_Impl::SetStartingTrackNumber(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartingTrackNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::StartingTrackNumber(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_TrackInfo<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackindex: i32, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::get_TrackInfo(this, core::mem::transmute_copy(&trackindex)) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::NumberOfExistingTracks(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastUsedUserSectorInImage<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::LastUsedUserSectorInImage(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageCreator_Impl::ExpectedTableOfContents(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateResultImage: CreateResultImage::<Identity, OFFSET>,
            AddTrack: AddTrack::<Identity, OFFSET>,
            AddSpecialPregap: AddSpecialPregap::<Identity, OFFSET>,
            AddSubcodeRWGenerator: AddSubcodeRWGenerator::<Identity, OFFSET>,
            SetResultingImageType: SetResultingImageType::<Identity, OFFSET>,
            ResultingImageType: ResultingImageType::<Identity, OFFSET>,
            StartOfLeadout: StartOfLeadout::<Identity, OFFSET>,
            SetStartOfLeadoutLimit: SetStartOfLeadoutLimit::<Identity, OFFSET>,
            StartOfLeadoutLimit: StartOfLeadoutLimit::<Identity, OFFSET>,
            SetDisableGaplessAudio: SetDisableGaplessAudio::<Identity, OFFSET>,
            DisableGaplessAudio: DisableGaplessAudio::<Identity, OFFSET>,
            SetMediaCatalogNumber: SetMediaCatalogNumber::<Identity, OFFSET>,
            MediaCatalogNumber: MediaCatalogNumber::<Identity, OFFSET>,
            SetStartingTrackNumber: SetStartingTrackNumber::<Identity, OFFSET>,
            StartingTrackNumber: StartingTrackNumber::<Identity, OFFSET>,
            get_TrackInfo: get_TrackInfo::<Identity, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, OFFSET>,
            LastUsedUserSectorInImage: LastUsedUserSectorInImage::<Identity, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawCDImageCreator as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawCDImageTrackInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartingLba(&self) -> windows_core::Result<i32>;
    fn SectorCount(&self) -> windows_core::Result<i32>;
    fn TrackNumber(&self) -> windows_core::Result<i32>;
    fn SectorType(&self) -> windows_core::Result<IMAPI_CD_SECTOR_TYPE>;
    fn ISRC(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetISRC(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DigitalAudioCopySetting(&self) -> windows_core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING>;
    fn SetDigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::Result<()>;
    fn AudioHasPreemphasis(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAudioHasPreemphasis(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TrackIndexes(&self) -> windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn AddTrackIndex(&self, lbaoffset: i32) -> windows_core::Result<()>;
    fn ClearTrackIndex(&self, lbaoffset: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRawCDImageTrackInfo {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRawCDImageTrackInfo_Vtbl {
    pub const fn new<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>() -> IRawCDImageTrackInfo_Vtbl {
        unsafe extern "system" fn StartingLba<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageTrackInfo_Impl::StartingLba(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorCount<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageTrackInfo_Impl::SectorCount(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackNumber<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageTrackInfo_Impl::TrackNumber(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorType<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageTrackInfo_Impl::SectorType(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ISRC<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageTrackInfo_Impl::ISRC(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetISRC<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageTrackInfo_Impl::SetISRC(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn DigitalAudioCopySetting<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageTrackInfo_Impl::DigitalAudioCopySetting(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigitalAudioCopySetting<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageTrackInfo_Impl::SetDigitalAudioCopySetting(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AudioHasPreemphasis<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageTrackInfo_Impl::AudioHasPreemphasis(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioHasPreemphasis<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageTrackInfo_Impl::SetAudioHasPreemphasis(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn TrackIndexes<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRawCDImageTrackInfo_Impl::TrackIndexes(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrackIndex<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbaoffset: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageTrackInfo_Impl::AddTrackIndex(this, core::mem::transmute_copy(&lbaoffset)).into()
        }
        unsafe extern "system" fn ClearTrackIndex<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbaoffset: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRawCDImageTrackInfo_Impl::ClearTrackIndex(this, core::mem::transmute_copy(&lbaoffset)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartingLba: StartingLba::<Identity, OFFSET>,
            SectorCount: SectorCount::<Identity, OFFSET>,
            TrackNumber: TrackNumber::<Identity, OFFSET>,
            SectorType: SectorType::<Identity, OFFSET>,
            ISRC: ISRC::<Identity, OFFSET>,
            SetISRC: SetISRC::<Identity, OFFSET>,
            DigitalAudioCopySetting: DigitalAudioCopySetting::<Identity, OFFSET>,
            SetDigitalAudioCopySetting: SetDigitalAudioCopySetting::<Identity, OFFSET>,
            AudioHasPreemphasis: AudioHasPreemphasis::<Identity, OFFSET>,
            SetAudioHasPreemphasis: SetAudioHasPreemphasis::<Identity, OFFSET>,
            TrackIndexes: TrackIndexes::<Identity, OFFSET>,
            AddTrackIndex: AddTrackIndex::<Identity, OFFSET>,
            ClearTrackIndex: ClearTrackIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawCDImageTrackInfo as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IRedbookDiscMaster_Impl: Sized + windows_core::IUnknownImpl {
    fn GetTotalAudioTracks(&self) -> windows_core::Result<i32>;
    fn GetTotalAudioBlocks(&self) -> windows_core::Result<i32>;
    fn GetUsedAudioBlocks(&self) -> windows_core::Result<i32>;
    fn GetAvailableAudioTrackBlocks(&self) -> windows_core::Result<i32>;
    fn GetAudioBlockSize(&self) -> windows_core::Result<i32>;
    fn CreateAudioTrack(&self, nblocks: i32) -> windows_core::Result<()>;
    fn AddAudioTrackBlocks(&self, pby: *const u8, cb: i32) -> windows_core::Result<()>;
    fn CloseAudioTrack(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IRedbookDiscMaster {}
impl IRedbookDiscMaster_Vtbl {
    pub const fn new<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>() -> IRedbookDiscMaster_Vtbl {
        unsafe extern "system" fn GetTotalAudioTracks<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pntracks: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRedbookDiscMaster_Impl::GetTotalAudioTracks(this) {
                Ok(ok__) => {
                    pntracks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalAudioBlocks<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblocks: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRedbookDiscMaster_Impl::GetTotalAudioBlocks(this) {
                Ok(ok__) => {
                    pnblocks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsedAudioBlocks<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblocks: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRedbookDiscMaster_Impl::GetUsedAudioBlocks(this) {
                Ok(ok__) => {
                    pnblocks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableAudioTrackBlocks<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblocks: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRedbookDiscMaster_Impl::GetAvailableAudioTrackBlocks(this) {
                Ok(ok__) => {
                    pnblocks.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioBlockSize<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnblockbytes: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRedbookDiscMaster_Impl::GetAudioBlockSize(this) {
                Ok(ok__) => {
                    pnblockbytes.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioTrack<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nblocks: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRedbookDiscMaster_Impl::CreateAudioTrack(this, core::mem::transmute_copy(&nblocks)).into()
        }
        unsafe extern "system" fn AddAudioTrackBlocks<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pby: *const u8, cb: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRedbookDiscMaster_Impl::AddAudioTrackBlocks(this, core::mem::transmute_copy(&pby), core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn CloseAudioTrack<Identity: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRedbookDiscMaster_Impl::CloseAudioTrack(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTotalAudioTracks: GetTotalAudioTracks::<Identity, OFFSET>,
            GetTotalAudioBlocks: GetTotalAudioBlocks::<Identity, OFFSET>,
            GetUsedAudioBlocks: GetUsedAudioBlocks::<Identity, OFFSET>,
            GetAvailableAudioTrackBlocks: GetAvailableAudioTrackBlocks::<Identity, OFFSET>,
            GetAudioBlockSize: GetAudioBlockSize::<Identity, OFFSET>,
            CreateAudioTrack: CreateAudioTrack::<Identity, OFFSET>,
            AddAudioTrackBlocks: AddAudioTrackBlocks::<Identity, OFFSET>,
            CloseAudioTrack: CloseAudioTrack::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRedbookDiscMaster as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamConcatenate_Impl: Sized + super::super::System::Com::IStream_Impl {
    fn Initialize(&self, stream1: Option<&super::super::System::Com::IStream>, stream2: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Initialize2(&self, streams: *const Option<super::super::System::Com::IStream>, streamcount: u32) -> windows_core::Result<()>;
    fn Append(&self, stream: Option<&super::super::System::Com::IStream>) -> windows_core::Result<()>;
    fn Append2(&self, streams: *const Option<super::super::System::Com::IStream>, streamcount: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStreamConcatenate {}
#[cfg(feature = "Win32_System_Com")]
impl IStreamConcatenate_Vtbl {
    pub const fn new<Identity: IStreamConcatenate_Impl, const OFFSET: isize>() -> IStreamConcatenate_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream1: *mut core::ffi::c_void, stream2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStreamConcatenate_Impl::Initialize(this, windows_core::from_raw_borrowed(&stream1), windows_core::from_raw_borrowed(&stream2)).into()
        }
        unsafe extern "system" fn Initialize2<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, streamcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStreamConcatenate_Impl::Initialize2(this, core::mem::transmute_copy(&streams), core::mem::transmute_copy(&streamcount)).into()
        }
        unsafe extern "system" fn Append<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStreamConcatenate_Impl::Append(this, windows_core::from_raw_borrowed(&stream)).into()
        }
        unsafe extern "system" fn Append2<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, streamcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStreamConcatenate_Impl::Append2(this, core::mem::transmute_copy(&streams), core::mem::transmute_copy(&streamcount)).into()
        }
        Self {
            base__: super::super::System::Com::IStream_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Initialize2: Initialize2::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            Append2: Append2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamConcatenate as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamInterleave_Impl: Sized + super::super::System::Com::IStream_Impl {
    fn Initialize(&self, streams: *const Option<super::super::System::Com::IStream>, interleavesizes: *const u32, streamcount: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStreamInterleave {}
#[cfg(feature = "Win32_System_Com")]
impl IStreamInterleave_Vtbl {
    pub const fn new<Identity: IStreamInterleave_Impl, const OFFSET: isize>() -> IStreamInterleave_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IStreamInterleave_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, interleavesizes: *const u32, streamcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStreamInterleave_Impl::Initialize(this, core::mem::transmute_copy(&streams), core::mem::transmute_copy(&interleavesizes), core::mem::transmute_copy(&streamcount)).into()
        }
        Self { base__: super::super::System::Com::IStream_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamInterleave as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamPseudoRandomBased_Impl: Sized + super::super::System::Com::IStream_Impl {
    fn SetSeed(&self, value: u32) -> windows_core::Result<()>;
    fn Seed(&self) -> windows_core::Result<u32>;
    fn put_ExtendedSeed(&self, values: *const u32, ecount: u32) -> windows_core::Result<()>;
    fn get_ExtendedSeed(&self, values: *mut *mut u32, ecount: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IStreamPseudoRandomBased {}
#[cfg(feature = "Win32_System_Com")]
impl IStreamPseudoRandomBased_Vtbl {
    pub const fn new<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>() -> IStreamPseudoRandomBased_Vtbl {
        unsafe extern "system" fn SetSeed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStreamPseudoRandomBased_Impl::SetSeed(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Seed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IStreamPseudoRandomBased_Impl::Seed(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_ExtendedSeed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, values: *const u32, ecount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStreamPseudoRandomBased_Impl::put_ExtendedSeed(this, core::mem::transmute_copy(&values), core::mem::transmute_copy(&ecount)).into()
        }
        unsafe extern "system" fn get_ExtendedSeed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, values: *mut *mut u32, ecount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IStreamPseudoRandomBased_Impl::get_ExtendedSeed(this, core::mem::transmute_copy(&values), core::mem::transmute_copy(&ecount)).into()
        }
        Self {
            base__: super::super::System::Com::IStream_Vtbl::new::<Identity, OFFSET>(),
            SetSeed: SetSeed::<Identity, OFFSET>,
            Seed: Seed::<Identity, OFFSET>,
            put_ExtendedSeed: put_ExtendedSeed::<Identity, OFFSET>,
            get_ExtendedSeed: get_ExtendedSeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamPseudoRandomBased as windows_core::Interface>::IID || iid == &<super::super::System::Com::ISequentialStream as windows_core::Interface>::IID || iid == &<super::super::System::Com::IStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteEngine2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WriteSection(&self, data: Option<&super::super::System::Com::IStream>, startingblockaddress: i32, numberofblocks: i32) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn SetRecorder(&self, value: Option<&IDiscRecorder2Ex>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2Ex>;
    fn SetUseStreamingWrite12(&self, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseStreamingWrite12(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStartingSectorsPerSecond(&self, value: i32) -> windows_core::Result<()>;
    fn StartingSectorsPerSecond(&self) -> windows_core::Result<i32>;
    fn SetEndingSectorsPerSecond(&self, value: i32) -> windows_core::Result<()>;
    fn EndingSectorsPerSecond(&self) -> windows_core::Result<i32>;
    fn SetBytesPerSector(&self, value: i32) -> windows_core::Result<()>;
    fn BytesPerSector(&self) -> windows_core::Result<i32>;
    fn WriteInProgress(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWriteEngine2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWriteEngine2_Vtbl {
    pub const fn new<Identity: IWriteEngine2_Impl, const OFFSET: isize>() -> IWriteEngine2_Vtbl {
        unsafe extern "system" fn WriteSection<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, startingblockaddress: i32, numberofblocks: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWriteEngine2_Impl::WriteSection(this, windows_core::from_raw_borrowed(&data), core::mem::transmute_copy(&startingblockaddress), core::mem::transmute_copy(&numberofblocks)).into()
        }
        unsafe extern "system" fn CancelWrite<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWriteEngine2_Impl::CancelWrite(this).into()
        }
        unsafe extern "system" fn SetRecorder<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWriteEngine2_Impl::SetRecorder(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2_Impl::Recorder(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseStreamingWrite12<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWriteEngine2_Impl::SetUseStreamingWrite12(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UseStreamingWrite12<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2_Impl::UseStreamingWrite12(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWriteEngine2_Impl::SetStartingSectorsPerSecond(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2_Impl::StartingSectorsPerSecond(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWriteEngine2_Impl::SetEndingSectorsPerSecond(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn EndingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2_Impl::EndingSectorsPerSecond(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBytesPerSector<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWriteEngine2_Impl::SetBytesPerSector(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BytesPerSector<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2_Impl::BytesPerSector(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteInProgress<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2_Impl::WriteInProgress(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            WriteSection: WriteSection::<Identity, OFFSET>,
            CancelWrite: CancelWrite::<Identity, OFFSET>,
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetUseStreamingWrite12: SetUseStreamingWrite12::<Identity, OFFSET>,
            UseStreamingWrite12: UseStreamingWrite12::<Identity, OFFSET>,
            SetStartingSectorsPerSecond: SetStartingSectorsPerSecond::<Identity, OFFSET>,
            StartingSectorsPerSecond: StartingSectorsPerSecond::<Identity, OFFSET>,
            SetEndingSectorsPerSecond: SetEndingSectorsPerSecond::<Identity, OFFSET>,
            EndingSectorsPerSecond: EndingSectorsPerSecond::<Identity, OFFSET>,
            SetBytesPerSector: SetBytesPerSector::<Identity, OFFSET>,
            BytesPerSector: BytesPerSector::<Identity, OFFSET>,
            WriteInProgress: WriteInProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteEngine2 as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteEngine2EventArgs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartLba(&self) -> windows_core::Result<i32>;
    fn SectorCount(&self) -> windows_core::Result<i32>;
    fn LastReadLba(&self) -> windows_core::Result<i32>;
    fn LastWrittenLba(&self) -> windows_core::Result<i32>;
    fn TotalSystemBuffer(&self) -> windows_core::Result<i32>;
    fn UsedSystemBuffer(&self) -> windows_core::Result<i32>;
    fn FreeSystemBuffer(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWriteEngine2EventArgs {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWriteEngine2EventArgs_Vtbl {
    pub const fn new<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>() -> IWriteEngine2EventArgs_Vtbl {
        unsafe extern "system" fn StartLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2EventArgs_Impl::StartLba(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorCount<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2EventArgs_Impl::SectorCount(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastReadLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2EventArgs_Impl::LastReadLba(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2EventArgs_Impl::LastWrittenLba(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2EventArgs_Impl::TotalSystemBuffer(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2EventArgs_Impl::UsedSystemBuffer(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteEngine2EventArgs_Impl::FreeSystemBuffer(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartLba: StartLba::<Identity, OFFSET>,
            SectorCount: SectorCount::<Identity, OFFSET>,
            LastReadLba: LastReadLba::<Identity, OFFSET>,
            LastWrittenLba: LastWrittenLba::<Identity, OFFSET>,
            TotalSystemBuffer: TotalSystemBuffer::<Identity, OFFSET>,
            UsedSystemBuffer: UsedSystemBuffer::<Identity, OFFSET>,
            FreeSystemBuffer: FreeSystemBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteSpeedDescriptor_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn RotationTypeIsPureCAV(&self) -> windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn WriteSpeed(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IWriteSpeedDescriptor {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWriteSpeedDescriptor_Vtbl {
    pub const fn new<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>() -> IWriteSpeedDescriptor_Vtbl {
        unsafe extern "system" fn MediaType<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteSpeedDescriptor_Impl::MediaType(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationTypeIsPureCAV<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteSpeedDescriptor_Impl::RotationTypeIsPureCAV(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSpeed<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWriteSpeedDescriptor_Impl::WriteSpeed(this) {
                Ok(ok__) => {
                    value.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            MediaType: MediaType::<Identity, OFFSET>,
            RotationTypeIsPureCAV: RotationTypeIsPureCAV::<Identity, OFFSET>,
            WriteSpeed: WriteSpeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteSpeedDescriptor as windows_core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
