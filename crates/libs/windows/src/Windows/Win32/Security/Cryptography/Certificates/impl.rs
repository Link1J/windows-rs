#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAlternativeName_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn InitializeFromString(&self, r#type: AlternativeNameType, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromRawData(&self, r#type: AlternativeNameType, encoding: EncodingType, strrawdata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromOtherName(&self, pobjectid: Option<&IObjectId>, encoding: EncodingType, strrawdata: &windows_core::BSTR, tobewrapped: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Type(&self) -> windows_core::Result<AlternativeNameType>;
    fn StrValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ObjectId(&self) -> windows_core::Result<IObjectId>;
    fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAlternativeName {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAlternativeName_Vtbl {
    pub const fn new<Identity: IAlternativeName_Impl, const OFFSET: isize>() -> IAlternativeName_Vtbl {
        unsafe extern "system" fn InitializeFromString<Identity: IAlternativeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AlternativeNameType, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlternativeName_Impl::InitializeFromString(this, core::mem::transmute_copy(&r#type), core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn InitializeFromRawData<Identity: IAlternativeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AlternativeNameType, encoding: EncodingType, strrawdata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlternativeName_Impl::InitializeFromRawData(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&encoding), core::mem::transmute(&strrawdata)).into()
        }
        unsafe extern "system" fn InitializeFromOtherName<Identity: IAlternativeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectid: *mut core::ffi::c_void, encoding: EncodingType, strrawdata: core::mem::MaybeUninit<windows_core::BSTR>, tobewrapped: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlternativeName_Impl::InitializeFromOtherName(this, windows_core::from_raw_borrowed(&pobjectid), core::mem::transmute_copy(&encoding), core::mem::transmute(&strrawdata), core::mem::transmute_copy(&tobewrapped)).into()
        }
        unsafe extern "system" fn Type<Identity: IAlternativeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut AlternativeNameType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlternativeName_Impl::Type(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrValue<Identity: IAlternativeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlternativeName_Impl::StrValue(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectId<Identity: IAlternativeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlternativeName_Impl::ObjectId(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_RawData<Identity: IAlternativeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlternativeName_Impl::get_RawData(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromString: InitializeFromString::<Identity, OFFSET>,
            InitializeFromRawData: InitializeFromRawData::<Identity, OFFSET>,
            InitializeFromOtherName: InitializeFromOtherName::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            StrValue: StrValue::<Identity, OFFSET>,
            ObjectId: ObjectId::<Identity, OFFSET>,
            get_RawData: get_RawData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAlternativeName as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAlternativeNames_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IAlternativeName>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&IAlternativeName>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IAlternativeNames {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAlternativeNames_Vtbl {
    pub const fn new<Identity: IAlternativeNames_Impl, const OFFSET: isize>() -> IAlternativeNames_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: IAlternativeNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlternativeNames_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IAlternativeNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlternativeNames_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IAlternativeNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IAlternativeNames_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IAlternativeNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlternativeNames_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: IAlternativeNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlternativeNames_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: IAlternativeNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IAlternativeNames_Impl::Clear(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAlternativeNames as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBinaryConverter_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn StringToString(&self, strencodedin: &windows_core::BSTR, encodingin: EncodingType, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn VariantByteArrayToString(&self, pvarbytearray: *const super::super::super::System::Variant::VARIANT, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn StringToVariantByteArray(&self, strencoded: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IBinaryConverter {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBinaryConverter_Vtbl {
    pub const fn new<Identity: IBinaryConverter_Impl, const OFFSET: isize>() -> IBinaryConverter_Vtbl {
        unsafe extern "system" fn StringToString<Identity: IBinaryConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodedin: core::mem::MaybeUninit<windows_core::BSTR>, encodingin: EncodingType, encoding: EncodingType, pstrencoded: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBinaryConverter_Impl::StringToString(this, core::mem::transmute(&strencodedin), core::mem::transmute_copy(&encodingin), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrencoded.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VariantByteArrayToString<Identity: IBinaryConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarbytearray: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>, encoding: EncodingType, pstrencoded: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBinaryConverter_Impl::VariantByteArrayToString(this, core::mem::transmute_copy(&pvarbytearray), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrencoded.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StringToVariantByteArray<Identity: IBinaryConverter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencoded: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType, pvarbytearray: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBinaryConverter_Impl::StringToVariantByteArray(this, core::mem::transmute(&strencoded), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvarbytearray.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StringToString: StringToString::<Identity, OFFSET>,
            VariantByteArrayToString: VariantByteArrayToString::<Identity, OFFSET>,
            StringToVariantByteArray: StringToVariantByteArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBinaryConverter as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBinaryConverter2_Impl: Sized + IBinaryConverter_Impl {
    fn StringArrayToVariantArray(&self, pvarstringarray: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn VariantArrayToStringArray(&self, pvarvariantarray: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IBinaryConverter2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBinaryConverter2_Vtbl {
    pub const fn new<Identity: IBinaryConverter2_Impl, const OFFSET: isize>() -> IBinaryConverter2_Vtbl {
        unsafe extern "system" fn StringArrayToVariantArray<Identity: IBinaryConverter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarstringarray: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>, pvarvariantarray: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBinaryConverter2_Impl::StringArrayToVariantArray(this, core::mem::transmute_copy(&pvarstringarray)) {
                Ok(ok__) => {
                    pvarvariantarray.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VariantArrayToStringArray<Identity: IBinaryConverter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarvariantarray: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>, pvarstringarray: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBinaryConverter2_Impl::VariantArrayToStringArray(this, core::mem::transmute_copy(&pvarvariantarray)) {
                Ok(ok__) => {
                    pvarstringarray.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IBinaryConverter_Vtbl::new::<Identity, OFFSET>(),
            StringArrayToVariantArray: StringArrayToVariantArray::<Identity, OFFSET>,
            VariantArrayToStringArray: VariantArrayToStringArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBinaryConverter2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IBinaryConverter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICEnroll_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn createFilePKCS10(&self, dnname: &windows_core::BSTR, usage: &windows_core::BSTR, wszpkcs10filename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn acceptFilePKCS7(&self, wszpkcs7filename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn createPKCS10(&self, dnname: &windows_core::BSTR, usage: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn acceptPKCS7(&self, pkcs7: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getCertFromPKCS7(&self, wszpkcs7: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn enumProviders(&self, dwindex: i32, dwflags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn enumContainers(&self, dwindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn freeRequestInfo(&self, pkcs7orpkcs10: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MyStoreName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMyStoreName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MyStoreType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetMyStoreType(&self, bstrtype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MyStoreFlags(&self) -> windows_core::Result<i32>;
    fn SetMyStoreFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn CAStoreName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCAStoreName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CAStoreType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCAStoreType(&self, bstrtype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CAStoreFlags(&self) -> windows_core::Result<i32>;
    fn SetCAStoreFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn RootStoreName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRootStoreName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RootStoreType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRootStoreType(&self, bstrtype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RootStoreFlags(&self) -> windows_core::Result<i32>;
    fn SetRootStoreFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn RequestStoreName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRequestStoreName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RequestStoreType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRequestStoreType(&self, bstrtype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RequestStoreFlags(&self) -> windows_core::Result<i32>;
    fn SetRequestStoreFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn ContainerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetContainerName(&self, bstrcontainer: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProviderName(&self, bstrprovider: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProviderType(&self) -> windows_core::Result<i32>;
    fn SetProviderType(&self, dwtype: i32) -> windows_core::Result<()>;
    fn KeySpec(&self) -> windows_core::Result<i32>;
    fn SetKeySpec(&self, dw: i32) -> windows_core::Result<()>;
    fn ProviderFlags(&self) -> windows_core::Result<i32>;
    fn SetProviderFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn UseExistingKeySet(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetUseExistingKeySet(&self, fuseexistingkeys: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GenKeyFlags(&self) -> windows_core::Result<i32>;
    fn SetGenKeyFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn DeleteRequestCert(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetDeleteRequestCert(&self, fdelete: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn WriteCertToCSP(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetWriteCertToCSP(&self, fbool: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SPCFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSPCFileName(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PVKFileName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPVKFileName(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHashAlgorithm(&self, bstr: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICEnroll {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICEnroll_Vtbl {
    pub const fn new<Identity: ICEnroll_Impl, const OFFSET: isize>() -> ICEnroll_Vtbl {
        unsafe extern "system" fn createFilePKCS10<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dnname: core::mem::MaybeUninit<windows_core::BSTR>, usage: core::mem::MaybeUninit<windows_core::BSTR>, wszpkcs10filename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::createFilePKCS10(this, core::mem::transmute(&dnname), core::mem::transmute(&usage), core::mem::transmute(&wszpkcs10filename)).into()
        }
        unsafe extern "system" fn acceptFilePKCS7<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpkcs7filename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::acceptFilePKCS7(this, core::mem::transmute(&wszpkcs7filename)).into()
        }
        unsafe extern "system" fn createPKCS10<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dnname: core::mem::MaybeUninit<windows_core::BSTR>, usage: core::mem::MaybeUninit<windows_core::BSTR>, ppkcs10: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::createPKCS10(this, core::mem::transmute(&dnname), core::mem::transmute(&usage)) {
                Ok(ok__) => {
                    ppkcs10.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn acceptPKCS7<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkcs7: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::acceptPKCS7(this, core::mem::transmute(&pkcs7)).into()
        }
        unsafe extern "system" fn getCertFromPKCS7<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpkcs7: core::mem::MaybeUninit<windows_core::BSTR>, pbstrcert: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::getCertFromPKCS7(this, core::mem::transmute(&wszpkcs7)) {
                Ok(ok__) => {
                    pbstrcert.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enumProviders<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: i32, dwflags: i32, pbstrprovname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::enumProviders(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwflags)) {
                Ok(ok__) => {
                    pbstrprovname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enumContainers<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: i32, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::enumContainers(this, core::mem::transmute_copy(&dwindex)) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn freeRequestInfo<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkcs7orpkcs10: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::freeRequestInfo(this, core::mem::transmute(&pkcs7orpkcs10)).into()
        }
        unsafe extern "system" fn MyStoreName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::MyStoreName(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMyStoreName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetMyStoreName(this, core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn MyStoreType<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::MyStoreType(this) {
                Ok(ok__) => {
                    pbstrtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMyStoreType<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetMyStoreType(this, core::mem::transmute(&bstrtype)).into()
        }
        unsafe extern "system" fn MyStoreFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::MyStoreFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMyStoreFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetMyStoreFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CAStoreName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::CAStoreName(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAStoreName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetCAStoreName(this, core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn CAStoreType<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::CAStoreType(this) {
                Ok(ok__) => {
                    pbstrtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAStoreType<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetCAStoreType(this, core::mem::transmute(&bstrtype)).into()
        }
        unsafe extern "system" fn CAStoreFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::CAStoreFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAStoreFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetCAStoreFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RootStoreName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::RootStoreName(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootStoreName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetRootStoreName(this, core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn RootStoreType<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::RootStoreType(this) {
                Ok(ok__) => {
                    pbstrtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootStoreType<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetRootStoreType(this, core::mem::transmute(&bstrtype)).into()
        }
        unsafe extern "system" fn RootStoreFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::RootStoreFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRootStoreFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetRootStoreFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestStoreName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::RequestStoreName(this) {
                Ok(ok__) => {
                    pbstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestStoreName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetRequestStoreName(this, core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn RequestStoreType<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrtype: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::RequestStoreType(this) {
                Ok(ok__) => {
                    pbstrtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestStoreType<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrtype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetRequestStoreType(this, core::mem::transmute(&bstrtype)).into()
        }
        unsafe extern "system" fn RequestStoreFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::RequestStoreFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestStoreFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetRequestStoreFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ContainerName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcontainer: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::ContainerName(this) {
                Ok(ok__) => {
                    pbstrcontainer.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcontainer: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetContainerName(this, core::mem::transmute(&bstrcontainer)).into()
        }
        unsafe extern "system" fn ProviderName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprovider: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::ProviderName(this) {
                Ok(ok__) => {
                    pbstrprovider.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrprovider: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetProviderName(this, core::mem::transmute(&bstrprovider)).into()
        }
        unsafe extern "system" fn ProviderType<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtype: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::ProviderType(this) {
                Ok(ok__) => {
                    pdwtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderType<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtype: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetProviderType(this, core::mem::transmute_copy(&dwtype)).into()
        }
        unsafe extern "system" fn KeySpec<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdw: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::KeySpec(this) {
                Ok(ok__) => {
                    pdw.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeySpec<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dw: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetKeySpec(this, core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn ProviderFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::ProviderFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetProviderFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn UseExistingKeySet<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::UseExistingKeySet(this) {
                Ok(ok__) => {
                    fuseexistingkeys.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseExistingKeySet<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuseexistingkeys: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetUseExistingKeySet(this, core::mem::transmute_copy(&fuseexistingkeys)).into()
        }
        unsafe extern "system" fn GenKeyFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::GenKeyFlags(this) {
                Ok(ok__) => {
                    pdwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenKeyFlags<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetGenKeyFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteRequestCert<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdelete: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::DeleteRequestCert(this) {
                Ok(ok__) => {
                    fdelete.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeleteRequestCert<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdelete: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetDeleteRequestCert(this, core::mem::transmute_copy(&fdelete)).into()
        }
        unsafe extern "system" fn WriteCertToCSP<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::WriteCertToCSP(this) {
                Ok(ok__) => {
                    fbool.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteCertToCSP<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetWriteCertToCSP(this, core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn SPCFileName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::SPCFileName(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSPCFileName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetSPCFileName(this, core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn PVKFileName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::PVKFileName(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPVKFileName<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetPVKFileName(this, core::mem::transmute(&bstr)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll_Impl::HashAlgorithm(this) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: ICEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstr: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll_Impl::SetHashAlgorithm(this, core::mem::transmute(&bstr)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            createFilePKCS10: createFilePKCS10::<Identity, OFFSET>,
            acceptFilePKCS7: acceptFilePKCS7::<Identity, OFFSET>,
            createPKCS10: createPKCS10::<Identity, OFFSET>,
            acceptPKCS7: acceptPKCS7::<Identity, OFFSET>,
            getCertFromPKCS7: getCertFromPKCS7::<Identity, OFFSET>,
            enumProviders: enumProviders::<Identity, OFFSET>,
            enumContainers: enumContainers::<Identity, OFFSET>,
            freeRequestInfo: freeRequestInfo::<Identity, OFFSET>,
            MyStoreName: MyStoreName::<Identity, OFFSET>,
            SetMyStoreName: SetMyStoreName::<Identity, OFFSET>,
            MyStoreType: MyStoreType::<Identity, OFFSET>,
            SetMyStoreType: SetMyStoreType::<Identity, OFFSET>,
            MyStoreFlags: MyStoreFlags::<Identity, OFFSET>,
            SetMyStoreFlags: SetMyStoreFlags::<Identity, OFFSET>,
            CAStoreName: CAStoreName::<Identity, OFFSET>,
            SetCAStoreName: SetCAStoreName::<Identity, OFFSET>,
            CAStoreType: CAStoreType::<Identity, OFFSET>,
            SetCAStoreType: SetCAStoreType::<Identity, OFFSET>,
            CAStoreFlags: CAStoreFlags::<Identity, OFFSET>,
            SetCAStoreFlags: SetCAStoreFlags::<Identity, OFFSET>,
            RootStoreName: RootStoreName::<Identity, OFFSET>,
            SetRootStoreName: SetRootStoreName::<Identity, OFFSET>,
            RootStoreType: RootStoreType::<Identity, OFFSET>,
            SetRootStoreType: SetRootStoreType::<Identity, OFFSET>,
            RootStoreFlags: RootStoreFlags::<Identity, OFFSET>,
            SetRootStoreFlags: SetRootStoreFlags::<Identity, OFFSET>,
            RequestStoreName: RequestStoreName::<Identity, OFFSET>,
            SetRequestStoreName: SetRequestStoreName::<Identity, OFFSET>,
            RequestStoreType: RequestStoreType::<Identity, OFFSET>,
            SetRequestStoreType: SetRequestStoreType::<Identity, OFFSET>,
            RequestStoreFlags: RequestStoreFlags::<Identity, OFFSET>,
            SetRequestStoreFlags: SetRequestStoreFlags::<Identity, OFFSET>,
            ContainerName: ContainerName::<Identity, OFFSET>,
            SetContainerName: SetContainerName::<Identity, OFFSET>,
            ProviderName: ProviderName::<Identity, OFFSET>,
            SetProviderName: SetProviderName::<Identity, OFFSET>,
            ProviderType: ProviderType::<Identity, OFFSET>,
            SetProviderType: SetProviderType::<Identity, OFFSET>,
            KeySpec: KeySpec::<Identity, OFFSET>,
            SetKeySpec: SetKeySpec::<Identity, OFFSET>,
            ProviderFlags: ProviderFlags::<Identity, OFFSET>,
            SetProviderFlags: SetProviderFlags::<Identity, OFFSET>,
            UseExistingKeySet: UseExistingKeySet::<Identity, OFFSET>,
            SetUseExistingKeySet: SetUseExistingKeySet::<Identity, OFFSET>,
            GenKeyFlags: GenKeyFlags::<Identity, OFFSET>,
            SetGenKeyFlags: SetGenKeyFlags::<Identity, OFFSET>,
            DeleteRequestCert: DeleteRequestCert::<Identity, OFFSET>,
            SetDeleteRequestCert: SetDeleteRequestCert::<Identity, OFFSET>,
            WriteCertToCSP: WriteCertToCSP::<Identity, OFFSET>,
            SetWriteCertToCSP: SetWriteCertToCSP::<Identity, OFFSET>,
            SPCFileName: SPCFileName::<Identity, OFFSET>,
            SetSPCFileName: SetSPCFileName::<Identity, OFFSET>,
            PVKFileName: PVKFileName::<Identity, OFFSET>,
            SetPVKFileName: SetPVKFileName::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICEnroll as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICEnroll2_Impl: Sized + ICEnroll_Impl {
    fn addCertTypeToRequest(&self, certtype: &windows_core::BSTR) -> windows_core::Result<()>;
    fn addNameValuePairToSignature(&self, name: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn WriteCertToUserDS(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetWriteCertToUserDS(&self, fbool: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EnableT61DNEncoding(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetEnableT61DNEncoding(&self, fbool: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICEnroll2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICEnroll2_Vtbl {
    pub const fn new<Identity: ICEnroll2_Impl, const OFFSET: isize>() -> ICEnroll2_Vtbl {
        unsafe extern "system" fn addCertTypeToRequest<Identity: ICEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certtype: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll2_Impl::addCertTypeToRequest(this, core::mem::transmute(&certtype)).into()
        }
        unsafe extern "system" fn addNameValuePairToSignature<Identity: ICEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: core::mem::MaybeUninit<windows_core::BSTR>, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll2_Impl::addNameValuePairToSignature(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn WriteCertToUserDS<Identity: ICEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll2_Impl::WriteCertToUserDS(this) {
                Ok(ok__) => {
                    fbool.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteCertToUserDS<Identity: ICEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll2_Impl::SetWriteCertToUserDS(this, core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn EnableT61DNEncoding<Identity: ICEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll2_Impl::EnableT61DNEncoding(this) {
                Ok(ok__) => {
                    fbool.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableT61DNEncoding<Identity: ICEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll2_Impl::SetEnableT61DNEncoding(this, core::mem::transmute_copy(&fbool)).into()
        }
        Self {
            base__: ICEnroll_Vtbl::new::<Identity, OFFSET>(),
            addCertTypeToRequest: addCertTypeToRequest::<Identity, OFFSET>,
            addNameValuePairToSignature: addNameValuePairToSignature::<Identity, OFFSET>,
            WriteCertToUserDS: WriteCertToUserDS::<Identity, OFFSET>,
            SetWriteCertToUserDS: SetWriteCertToUserDS::<Identity, OFFSET>,
            EnableT61DNEncoding: EnableT61DNEncoding::<Identity, OFFSET>,
            SetEnableT61DNEncoding: SetEnableT61DNEncoding::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICEnroll2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICEnroll as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICEnroll3_Impl: Sized + ICEnroll2_Impl {
    fn InstallPKCS7(&self, pkcs7: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn GetSupportedKeySpec(&self) -> windows_core::Result<i32>;
    fn GetKeyLen(&self, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL) -> windows_core::Result<i32>;
    fn EnumAlgs(&self, dwindex: i32, algclass: i32) -> windows_core::Result<i32>;
    fn GetAlgName(&self, algid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetReuseHardwareKeyIfUnableToGenNew(&self, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ReuseHardwareKeyIfUnableToGenNew(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetHashAlgID(&self, hashalgid: i32) -> windows_core::Result<()>;
    fn HashAlgID(&self) -> windows_core::Result<i32>;
    fn SetLimitExchangeKeyToEncipherment(&self, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn LimitExchangeKeyToEncipherment(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetEnableSMIMECapabilities(&self, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EnableSMIMECapabilities(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICEnroll3 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICEnroll3_Vtbl {
    pub const fn new<Identity: ICEnroll3_Impl, const OFFSET: isize>() -> ICEnroll3_Vtbl {
        unsafe extern "system" fn InstallPKCS7<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkcs7: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll3_Impl::InstallPKCS7(this, core::mem::transmute(&pkcs7)).into()
        }
        unsafe extern "system" fn Reset<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll3_Impl::Reset(this).into()
        }
        unsafe extern "system" fn GetSupportedKeySpec<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwkeyspec: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll3_Impl::GetSupportedKeySpec(this) {
                Ok(ok__) => {
                    pdwkeyspec.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyLen<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL, pdwkeysize: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll3_Impl::GetKeyLen(this, core::mem::transmute_copy(&fmin), core::mem::transmute_copy(&fexchange)) {
                Ok(ok__) => {
                    pdwkeysize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAlgs<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll3_Impl::EnumAlgs(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&algclass)) {
                Ok(ok__) => {
                    pdwalgid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlgName<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, algid: i32, pbstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll3_Impl::GetAlgName(this, core::mem::transmute_copy(&algid)) {
                Ok(ok__) => {
                    pbstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReuseHardwareKeyIfUnableToGenNew<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll3_Impl::SetReuseHardwareKeyIfUnableToGenNew(this, core::mem::transmute_copy(&freusehardwarekeyifunabletogennew)).into()
        }
        unsafe extern "system" fn ReuseHardwareKeyIfUnableToGenNew<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll3_Impl::ReuseHardwareKeyIfUnableToGenNew(this) {
                Ok(ok__) => {
                    freusehardwarekeyifunabletogennew.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgID<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashalgid: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll3_Impl::SetHashAlgID(this, core::mem::transmute_copy(&hashalgid)).into()
        }
        unsafe extern "system" fn HashAlgID<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashalgid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll3_Impl::HashAlgID(this) {
                Ok(ok__) => {
                    hashalgid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLimitExchangeKeyToEncipherment<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll3_Impl::SetLimitExchangeKeyToEncipherment(this, core::mem::transmute_copy(&flimitexchangekeytoencipherment)).into()
        }
        unsafe extern "system" fn LimitExchangeKeyToEncipherment<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll3_Impl::LimitExchangeKeyToEncipherment(this) {
                Ok(ok__) => {
                    flimitexchangekeytoencipherment.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableSMIMECapabilities<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll3_Impl::SetEnableSMIMECapabilities(this, core::mem::transmute_copy(&fenablesmimecapabilities)).into()
        }
        unsafe extern "system" fn EnableSMIMECapabilities<Identity: ICEnroll3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll3_Impl::EnableSMIMECapabilities(this) {
                Ok(ok__) => {
                    fenablesmimecapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICEnroll2_Vtbl::new::<Identity, OFFSET>(),
            InstallPKCS7: InstallPKCS7::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            GetSupportedKeySpec: GetSupportedKeySpec::<Identity, OFFSET>,
            GetKeyLen: GetKeyLen::<Identity, OFFSET>,
            EnumAlgs: EnumAlgs::<Identity, OFFSET>,
            GetAlgName: GetAlgName::<Identity, OFFSET>,
            SetReuseHardwareKeyIfUnableToGenNew: SetReuseHardwareKeyIfUnableToGenNew::<Identity, OFFSET>,
            ReuseHardwareKeyIfUnableToGenNew: ReuseHardwareKeyIfUnableToGenNew::<Identity, OFFSET>,
            SetHashAlgID: SetHashAlgID::<Identity, OFFSET>,
            HashAlgID: HashAlgID::<Identity, OFFSET>,
            SetLimitExchangeKeyToEncipherment: SetLimitExchangeKeyToEncipherment::<Identity, OFFSET>,
            LimitExchangeKeyToEncipherment: LimitExchangeKeyToEncipherment::<Identity, OFFSET>,
            SetEnableSMIMECapabilities: SetEnableSMIMECapabilities::<Identity, OFFSET>,
            EnableSMIMECapabilities: EnableSMIMECapabilities::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICEnroll3 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICEnroll as windows_core::Interface>::IID || iid == &<ICEnroll2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICEnroll4_Impl: Sized + ICEnroll3_Impl {
    fn SetPrivateKeyArchiveCertificate(&self, bstrcert: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PrivateKeyArchiveCertificate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetThumbPrint(&self, bstrthumbprint: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ThumbPrint(&self) -> windows_core::Result<windows_core::BSTR>;
    fn binaryToString(&self, flags: i32, strbinary: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn stringToBinary(&self, flags: i32, strencoded: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn addExtensionToRequest(&self, flags: i32, strname: &windows_core::BSTR, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn addAttributeToRequest(&self, flags: i32, strname: &windows_core::BSTR, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn addNameValuePairToRequest(&self, flags: i32, strname: &windows_core::BSTR, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn resetExtensions(&self) -> windows_core::Result<()>;
    fn resetAttributes(&self) -> windows_core::Result<()>;
    fn createRequest(&self, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: &windows_core::BSTR, usage: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn createFileRequest(&self, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: &windows_core::BSTR, strusage: &windows_core::BSTR, strrequestfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn acceptResponse(&self, strresponse: &windows_core::BSTR) -> windows_core::Result<()>;
    fn acceptFileResponse(&self, strresponsefilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn getCertFromResponse(&self, strresponse: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn getCertFromFileResponse(&self, strresponsefilename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn createPFX(&self, strpassword: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn createFilePFX(&self, strpassword: &windows_core::BSTR, strpfxfilename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn setPendingRequestInfo(&self, lrequestid: i32, strcadns: &windows_core::BSTR, strcaname: &windows_core::BSTR, strfriendlyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn enumPendingRequest(&self, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn removePendingRequest(&self, strthumbprint: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetKeyLenEx(&self, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC) -> windows_core::Result<i32>;
    fn InstallPKCS7Ex(&self, pkcs7: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn addCertTypeToRequestEx(&self, ltype: ADDED_CERT_TYPE, bstroidorname: &windows_core::BSTR, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> windows_core::Result<()>;
    fn getProviderType(&self, strprovname: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn SetSignerCertificate(&self, bstrcert: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetClientId(&self, lclientid: i32) -> windows_core::Result<()>;
    fn ClientId(&self) -> windows_core::Result<i32>;
    fn addBlobPropertyToCertificate(&self, lpropertyid: i32, lreserved: i32, bstrproperty: &windows_core::BSTR) -> windows_core::Result<()>;
    fn resetBlobProperties(&self) -> windows_core::Result<()>;
    fn SetIncludeSubjectKeyID(&self, finclude: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IncludeSubjectKeyID(&self) -> windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICEnroll4 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICEnroll4_Vtbl {
    pub const fn new<Identity: ICEnroll4_Impl, const OFFSET: isize>() -> ICEnroll4_Vtbl {
        unsafe extern "system" fn SetPrivateKeyArchiveCertificate<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcert: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::SetPrivateKeyArchiveCertificate(this, core::mem::transmute(&bstrcert)).into()
        }
        unsafe extern "system" fn PrivateKeyArchiveCertificate<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcert: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::PrivateKeyArchiveCertificate(this) {
                Ok(ok__) => {
                    pbstrcert.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThumbPrint<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrthumbprint: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::SetThumbPrint(this, core::mem::transmute(&bstrthumbprint)).into()
        }
        unsafe extern "system" fn ThumbPrint<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrthumbprint: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::ThumbPrint(this) {
                Ok(ok__) => {
                    pbstrthumbprint.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn binaryToString<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, strbinary: core::mem::MaybeUninit<windows_core::BSTR>, pstrencoded: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::binaryToString(this, core::mem::transmute_copy(&flags), core::mem::transmute(&strbinary)) {
                Ok(ok__) => {
                    pstrencoded.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stringToBinary<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, strencoded: core::mem::MaybeUninit<windows_core::BSTR>, pstrbinary: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::stringToBinary(this, core::mem::transmute_copy(&flags), core::mem::transmute(&strencoded)) {
                Ok(ok__) => {
                    pstrbinary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addExtensionToRequest<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, strname: core::mem::MaybeUninit<windows_core::BSTR>, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::addExtensionToRequest(this, core::mem::transmute_copy(&flags), core::mem::transmute(&strname), core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn addAttributeToRequest<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, strname: core::mem::MaybeUninit<windows_core::BSTR>, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::addAttributeToRequest(this, core::mem::transmute_copy(&flags), core::mem::transmute(&strname), core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn addNameValuePairToRequest<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, strname: core::mem::MaybeUninit<windows_core::BSTR>, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::addNameValuePairToRequest(this, core::mem::transmute_copy(&flags), core::mem::transmute(&strname), core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn resetExtensions<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::resetExtensions(this).into()
        }
        unsafe extern "system" fn resetAttributes<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::resetAttributes(this).into()
        }
        unsafe extern "system" fn createRequest<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: core::mem::MaybeUninit<windows_core::BSTR>, usage: core::mem::MaybeUninit<windows_core::BSTR>, pstrrequest: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::createRequest(this, core::mem::transmute_copy(&flags), core::mem::transmute(&strdnname), core::mem::transmute(&usage)) {
                Ok(ok__) => {
                    pstrrequest.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createFileRequest<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, strdnname: core::mem::MaybeUninit<windows_core::BSTR>, strusage: core::mem::MaybeUninit<windows_core::BSTR>, strrequestfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::createFileRequest(this, core::mem::transmute_copy(&flags), core::mem::transmute(&strdnname), core::mem::transmute(&strusage), core::mem::transmute(&strrequestfilename)).into()
        }
        unsafe extern "system" fn acceptResponse<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strresponse: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::acceptResponse(this, core::mem::transmute(&strresponse)).into()
        }
        unsafe extern "system" fn acceptFileResponse<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strresponsefilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::acceptFileResponse(this, core::mem::transmute(&strresponsefilename)).into()
        }
        unsafe extern "system" fn getCertFromResponse<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strresponse: core::mem::MaybeUninit<windows_core::BSTR>, pstrcert: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::getCertFromResponse(this, core::mem::transmute(&strresponse)) {
                Ok(ok__) => {
                    pstrcert.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getCertFromFileResponse<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strresponsefilename: core::mem::MaybeUninit<windows_core::BSTR>, pstrcert: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::getCertFromFileResponse(this, core::mem::transmute(&strresponsefilename)) {
                Ok(ok__) => {
                    pstrcert.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createPFX<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, pstrpfx: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::createPFX(this, core::mem::transmute(&strpassword)) {
                Ok(ok__) => {
                    pstrpfx.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createFilePFX<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strpfxfilename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::createFilePFX(this, core::mem::transmute(&strpassword), core::mem::transmute(&strpfxfilename)).into()
        }
        unsafe extern "system" fn setPendingRequestInfo<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrequestid: i32, strcadns: core::mem::MaybeUninit<windows_core::BSTR>, strcaname: core::mem::MaybeUninit<windows_core::BSTR>, strfriendlyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::setPendingRequestInfo(this, core::mem::transmute_copy(&lrequestid), core::mem::transmute(&strcadns), core::mem::transmute(&strcaname), core::mem::transmute(&strfriendlyname)).into()
        }
        unsafe extern "system" fn enumPendingRequest<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY, pvarproperty: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::enumPendingRequest(this, core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&ldesiredproperty)) {
                Ok(ok__) => {
                    pvarproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removePendingRequest<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strthumbprint: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::removePendingRequest(this, core::mem::transmute(&strthumbprint)).into()
        }
        unsafe extern "system" fn GetKeyLenEx<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC, pdwkeysize: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::GetKeyLenEx(this, core::mem::transmute_copy(&lsizespec), core::mem::transmute_copy(&lkeyspec)) {
                Ok(ok__) => {
                    pdwkeysize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallPKCS7Ex<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkcs7: core::mem::MaybeUninit<windows_core::BSTR>, plcertinstalled: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::InstallPKCS7Ex(this, core::mem::transmute(&pkcs7)) {
                Ok(ok__) => {
                    plcertinstalled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addCertTypeToRequestEx<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltype: ADDED_CERT_TYPE, bstroidorname: core::mem::MaybeUninit<windows_core::BSTR>, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::addCertTypeToRequestEx(this, core::mem::transmute_copy(&ltype), core::mem::transmute(&bstroidorname), core::mem::transmute_copy(&lmajorversion), core::mem::transmute_copy(&fminorversion), core::mem::transmute_copy(&lminorversion)).into()
        }
        unsafe extern "system" fn getProviderType<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprovname: core::mem::MaybeUninit<windows_core::BSTR>, plprovtype: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::getProviderType(this, core::mem::transmute(&strprovname)) {
                Ok(ok__) => {
                    plprovtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignerCertificate<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrcert: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::SetSignerCertificate(this, core::mem::transmute(&bstrcert)).into()
        }
        unsafe extern "system" fn SetClientId<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lclientid: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::SetClientId(this, core::mem::transmute_copy(&lclientid)).into()
        }
        unsafe extern "system" fn ClientId<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plclientid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::ClientId(this) {
                Ok(ok__) => {
                    plclientid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addBlobPropertyToCertificate<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropertyid: i32, lreserved: i32, bstrproperty: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::addBlobPropertyToCertificate(this, core::mem::transmute_copy(&lpropertyid), core::mem::transmute_copy(&lreserved), core::mem::transmute(&bstrproperty)).into()
        }
        unsafe extern "system" fn resetBlobProperties<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::resetBlobProperties(this).into()
        }
        unsafe extern "system" fn SetIncludeSubjectKeyID<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finclude: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICEnroll4_Impl::SetIncludeSubjectKeyID(this, core::mem::transmute_copy(&finclude)).into()
        }
        unsafe extern "system" fn IncludeSubjectKeyID<Identity: ICEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfinclude: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICEnroll4_Impl::IncludeSubjectKeyID(this) {
                Ok(ok__) => {
                    pfinclude.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICEnroll3_Vtbl::new::<Identity, OFFSET>(),
            SetPrivateKeyArchiveCertificate: SetPrivateKeyArchiveCertificate::<Identity, OFFSET>,
            PrivateKeyArchiveCertificate: PrivateKeyArchiveCertificate::<Identity, OFFSET>,
            SetThumbPrint: SetThumbPrint::<Identity, OFFSET>,
            ThumbPrint: ThumbPrint::<Identity, OFFSET>,
            binaryToString: binaryToString::<Identity, OFFSET>,
            stringToBinary: stringToBinary::<Identity, OFFSET>,
            addExtensionToRequest: addExtensionToRequest::<Identity, OFFSET>,
            addAttributeToRequest: addAttributeToRequest::<Identity, OFFSET>,
            addNameValuePairToRequest: addNameValuePairToRequest::<Identity, OFFSET>,
            resetExtensions: resetExtensions::<Identity, OFFSET>,
            resetAttributes: resetAttributes::<Identity, OFFSET>,
            createRequest: createRequest::<Identity, OFFSET>,
            createFileRequest: createFileRequest::<Identity, OFFSET>,
            acceptResponse: acceptResponse::<Identity, OFFSET>,
            acceptFileResponse: acceptFileResponse::<Identity, OFFSET>,
            getCertFromResponse: getCertFromResponse::<Identity, OFFSET>,
            getCertFromFileResponse: getCertFromFileResponse::<Identity, OFFSET>,
            createPFX: createPFX::<Identity, OFFSET>,
            createFilePFX: createFilePFX::<Identity, OFFSET>,
            setPendingRequestInfo: setPendingRequestInfo::<Identity, OFFSET>,
            enumPendingRequest: enumPendingRequest::<Identity, OFFSET>,
            removePendingRequest: removePendingRequest::<Identity, OFFSET>,
            GetKeyLenEx: GetKeyLenEx::<Identity, OFFSET>,
            InstallPKCS7Ex: InstallPKCS7Ex::<Identity, OFFSET>,
            addCertTypeToRequestEx: addCertTypeToRequestEx::<Identity, OFFSET>,
            getProviderType: getProviderType::<Identity, OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Identity, OFFSET>,
            SetClientId: SetClientId::<Identity, OFFSET>,
            ClientId: ClientId::<Identity, OFFSET>,
            addBlobPropertyToCertificate: addBlobPropertyToCertificate::<Identity, OFFSET>,
            resetBlobProperties: resetBlobProperties::<Identity, OFFSET>,
            SetIncludeSubjectKeyID: SetIncludeSubjectKeyID::<Identity, OFFSET>,
            IncludeSubjectKeyID: IncludeSubjectKeyID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICEnroll4 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICEnroll as windows_core::Interface>::IID || iid == &<ICEnroll2 as windows_core::Interface>::IID || iid == &<ICEnroll3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertAdmin_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn IsValidCertificate(&self, strconfig: &windows_core::BSTR, strserialnumber: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn GetRevocationReason(&self) -> windows_core::Result<i32>;
    fn RevokeCertificate(&self, strconfig: &windows_core::BSTR, strserialnumber: &windows_core::BSTR, reason: i32, date: f64) -> windows_core::Result<()>;
    fn SetRequestAttributes(&self, strconfig: &windows_core::BSTR, requestid: i32, strattributes: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetCertificateExtension(&self, strconfig: &windows_core::BSTR, requestid: i32, strextensionname: &windows_core::BSTR, r#type: CERT_PROPERTY_TYPE, flags: i32, pvarvalue: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn DenyRequest(&self, strconfig: &windows_core::BSTR, requestid: i32) -> windows_core::Result<()>;
    fn ResubmitRequest(&self, strconfig: &windows_core::BSTR, requestid: i32) -> windows_core::Result<i32>;
    fn PublishCRL(&self, strconfig: &windows_core::BSTR, date: f64) -> windows_core::Result<()>;
    fn GetCRL(&self, strconfig: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn ImportCertificate(&self, strconfig: &windows_core::BSTR, strcertificate: &windows_core::BSTR, flags: CERT_IMPORT_FLAGS) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertAdmin {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertAdmin_Vtbl {
    pub const fn new<Identity: ICertAdmin_Impl, const OFFSET: isize>() -> ICertAdmin_Vtbl {
        unsafe extern "system" fn IsValidCertificate<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, strserialnumber: core::mem::MaybeUninit<windows_core::BSTR>, pdisposition: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin_Impl::IsValidCertificate(this, core::mem::transmute(&strconfig), core::mem::transmute(&strserialnumber)) {
                Ok(ok__) => {
                    pdisposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRevocationReason<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preason: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin_Impl::GetRevocationReason(this) {
                Ok(ok__) => {
                    preason.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeCertificate<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, strserialnumber: core::mem::MaybeUninit<windows_core::BSTR>, reason: i32, date: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertAdmin_Impl::RevokeCertificate(this, core::mem::transmute(&strconfig), core::mem::transmute(&strserialnumber), core::mem::transmute_copy(&reason), core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn SetRequestAttributes<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, requestid: i32, strattributes: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertAdmin_Impl::SetRequestAttributes(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute(&strattributes)).into()
        }
        unsafe extern "system" fn SetCertificateExtension<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, requestid: i32, strextensionname: core::mem::MaybeUninit<windows_core::BSTR>, r#type: CERT_PROPERTY_TYPE, flags: i32, pvarvalue: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertAdmin_Impl::SetCertificateExtension(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute(&strextensionname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn DenyRequest<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, requestid: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertAdmin_Impl::DenyRequest(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid)).into()
        }
        unsafe extern "system" fn ResubmitRequest<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, requestid: i32, pdisposition: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin_Impl::ResubmitRequest(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid)) {
                Ok(ok__) => {
                    pdisposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublishCRL<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, date: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertAdmin_Impl::PublishCRL(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn GetCRL<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32, pstrcrl: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin_Impl::GetCRL(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pstrcrl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportCertificate<Identity: ICertAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, strcertificate: core::mem::MaybeUninit<windows_core::BSTR>, flags: CERT_IMPORT_FLAGS, prequestid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin_Impl::ImportCertificate(this, core::mem::transmute(&strconfig), core::mem::transmute(&strcertificate), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    prequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsValidCertificate: IsValidCertificate::<Identity, OFFSET>,
            GetRevocationReason: GetRevocationReason::<Identity, OFFSET>,
            RevokeCertificate: RevokeCertificate::<Identity, OFFSET>,
            SetRequestAttributes: SetRequestAttributes::<Identity, OFFSET>,
            SetCertificateExtension: SetCertificateExtension::<Identity, OFFSET>,
            DenyRequest: DenyRequest::<Identity, OFFSET>,
            ResubmitRequest: ResubmitRequest::<Identity, OFFSET>,
            PublishCRL: PublishCRL::<Identity, OFFSET>,
            GetCRL: GetCRL::<Identity, OFFSET>,
            ImportCertificate: ImportCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertAdmin as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertAdmin2_Impl: Sized + ICertAdmin_Impl {
    fn PublishCRLs(&self, strconfig: &windows_core::BSTR, date: f64, crlflags: i32) -> windows_core::Result<()>;
    fn GetCAProperty(&self, strconfig: &windows_core::BSTR, propid: i32, propindex: i32, proptype: i32, flags: i32) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetCAProperty(&self, strconfig: &windows_core::BSTR, propid: i32, propindex: i32, proptype: CERT_PROPERTY_TYPE, pvarpropertyvalue: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetCAPropertyFlags(&self, strconfig: &windows_core::BSTR, propid: i32) -> windows_core::Result<i32>;
    fn GetCAPropertyDisplayName(&self, strconfig: &windows_core::BSTR, propid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetArchivedKey(&self, strconfig: &windows_core::BSTR, requestid: i32, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetConfigEntry(&self, strconfig: &windows_core::BSTR, strnodepath: &windows_core::BSTR, strentryname: &windows_core::BSTR) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetConfigEntry(&self, strconfig: &windows_core::BSTR, strnodepath: &windows_core::BSTR, strentryname: &windows_core::BSTR, pvarentry: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn ImportKey(&self, strconfig: &windows_core::BSTR, requestid: i32, strcerthash: &windows_core::BSTR, flags: CERT_IMPORT_FLAGS, strkey: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetMyRoles(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<CERTADMIN_GET_ROLES_FLAGS>;
    fn DeleteRow(&self, strconfig: &windows_core::BSTR, flags: CERT_DELETE_ROW_FLAGS, date: f64, table: CVRC_TABLE, rowid: i32) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertAdmin2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertAdmin2_Vtbl {
    pub const fn new<Identity: ICertAdmin2_Impl, const OFFSET: isize>() -> ICertAdmin2_Vtbl {
        unsafe extern "system" fn PublishCRLs<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, date: f64, crlflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertAdmin2_Impl::PublishCRLs(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&date), core::mem::transmute_copy(&crlflags)).into()
        }
        unsafe extern "system" fn GetCAProperty<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, propid: i32, propindex: i32, proptype: i32, flags: i32, pvarpropertyvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin2_Impl::GetCAProperty(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid), core::mem::transmute_copy(&propindex), core::mem::transmute_copy(&proptype), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pvarpropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAProperty<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, propid: i32, propindex: i32, proptype: CERT_PROPERTY_TYPE, pvarpropertyvalue: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertAdmin2_Impl::SetCAProperty(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid), core::mem::transmute_copy(&propindex), core::mem::transmute_copy(&proptype), core::mem::transmute_copy(&pvarpropertyvalue)).into()
        }
        unsafe extern "system" fn GetCAPropertyFlags<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, propid: i32, ppropflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin2_Impl::GetCAPropertyFlags(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid)) {
                Ok(ok__) => {
                    ppropflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAPropertyDisplayName<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, propid: i32, pstrdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin2_Impl::GetCAPropertyDisplayName(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid)) {
                Ok(ok__) => {
                    pstrdisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetArchivedKey<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, requestid: i32, flags: i32, pstrarchivedkey: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin2_Impl::GetArchivedKey(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pstrarchivedkey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfigEntry<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, strnodepath: core::mem::MaybeUninit<windows_core::BSTR>, strentryname: core::mem::MaybeUninit<windows_core::BSTR>, pvarentry: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin2_Impl::GetConfigEntry(this, core::mem::transmute(&strconfig), core::mem::transmute(&strnodepath), core::mem::transmute(&strentryname)) {
                Ok(ok__) => {
                    pvarentry.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConfigEntry<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, strnodepath: core::mem::MaybeUninit<windows_core::BSTR>, strentryname: core::mem::MaybeUninit<windows_core::BSTR>, pvarentry: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertAdmin2_Impl::SetConfigEntry(this, core::mem::transmute(&strconfig), core::mem::transmute(&strnodepath), core::mem::transmute(&strentryname), core::mem::transmute_copy(&pvarentry)).into()
        }
        unsafe extern "system" fn ImportKey<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, requestid: i32, strcerthash: core::mem::MaybeUninit<windows_core::BSTR>, flags: CERT_IMPORT_FLAGS, strkey: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertAdmin2_Impl::ImportKey(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute(&strcerthash), core::mem::transmute_copy(&flags), core::mem::transmute(&strkey)).into()
        }
        unsafe extern "system" fn GetMyRoles<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, proles: *mut CERTADMIN_GET_ROLES_FLAGS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin2_Impl::GetMyRoles(this, core::mem::transmute(&strconfig)) {
                Ok(ok__) => {
                    proles.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRow<Identity: ICertAdmin2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, flags: CERT_DELETE_ROW_FLAGS, date: f64, table: CVRC_TABLE, rowid: i32, pcdeleted: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertAdmin2_Impl::DeleteRow(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&date), core::mem::transmute_copy(&table), core::mem::transmute_copy(&rowid)) {
                Ok(ok__) => {
                    pcdeleted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertAdmin_Vtbl::new::<Identity, OFFSET>(),
            PublishCRLs: PublishCRLs::<Identity, OFFSET>,
            GetCAProperty: GetCAProperty::<Identity, OFFSET>,
            SetCAProperty: SetCAProperty::<Identity, OFFSET>,
            GetCAPropertyFlags: GetCAPropertyFlags::<Identity, OFFSET>,
            GetCAPropertyDisplayName: GetCAPropertyDisplayName::<Identity, OFFSET>,
            GetArchivedKey: GetArchivedKey::<Identity, OFFSET>,
            GetConfigEntry: GetConfigEntry::<Identity, OFFSET>,
            SetConfigEntry: SetConfigEntry::<Identity, OFFSET>,
            ImportKey: ImportKey::<Identity, OFFSET>,
            GetMyRoles: GetMyRoles::<Identity, OFFSET>,
            DeleteRow: DeleteRow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertAdmin2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertAdmin as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertConfig_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Reset(&self, index: i32) -> windows_core::Result<i32>;
    fn Next(&self) -> windows_core::Result<i32>;
    fn GetField(&self, strfieldname: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetConfig(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertConfig {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertConfig_Vtbl {
    pub const fn new<Identity: ICertConfig_Impl, const OFFSET: isize>() -> ICertConfig_Vtbl {
        unsafe extern "system" fn Reset<Identity: ICertConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pcount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertConfig_Impl::Reset(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ICertConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertConfig_Impl::Next(this) {
                Ok(ok__) => {
                    pindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetField<Identity: ICertConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfieldname: core::mem::MaybeUninit<windows_core::BSTR>, pstrout: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertConfig_Impl::GetField(this, core::mem::transmute(&strfieldname)) {
                Ok(ok__) => {
                    pstrout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfig<Identity: ICertConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pstrout: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertConfig_Impl::GetConfig(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pstrout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            GetField: GetField::<Identity, OFFSET>,
            GetConfig: GetConfig::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertConfig as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertConfig2_Impl: Sized + ICertConfig_Impl {
    fn SetSharedFolder(&self, strsharedfolder: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertConfig2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertConfig2_Vtbl {
    pub const fn new<Identity: ICertConfig2_Impl, const OFFSET: isize>() -> ICertConfig2_Vtbl {
        unsafe extern "system" fn SetSharedFolder<Identity: ICertConfig2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strsharedfolder: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertConfig2_Impl::SetSharedFolder(this, core::mem::transmute(&strsharedfolder)).into()
        }
        Self { base__: ICertConfig_Vtbl::new::<Identity, OFFSET>(), SetSharedFolder: SetSharedFolder::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertConfig2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertConfig as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeAltName_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetNameCount(&self) -> windows_core::Result<i32>;
    fn GetNameChoice(&self, nameindex: i32) -> windows_core::Result<i32>;
    fn GetName(&self, nameindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Reset(&self, namecount: i32) -> windows_core::Result<()>;
    fn SetNameEntry(&self, nameindex: i32, namechoice: CERT_ALT_NAME, strname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeAltName {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeAltName_Vtbl {
    pub const fn new<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>() -> ICertEncodeAltName_Vtbl {
        unsafe extern "system" fn Decode<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeAltName_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
        }
        unsafe extern "system" fn GetNameCount<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamecount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeAltName_Impl::GetNameCount(this) {
                Ok(ok__) => {
                    pnamecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameChoice<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nameindex: i32, pnamechoice: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeAltName_Impl::GetNameChoice(this, core::mem::transmute_copy(&nameindex)) {
                Ok(ok__) => {
                    pnamechoice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nameindex: i32, pstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeAltName_Impl::GetName(this, core::mem::transmute_copy(&nameindex)) {
                Ok(ok__) => {
                    pstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namecount: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeAltName_Impl::Reset(this, core::mem::transmute_copy(&namecount)).into()
        }
        unsafe extern "system" fn SetNameEntry<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nameindex: i32, namechoice: CERT_ALT_NAME, strname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeAltName_Impl::SetNameEntry(this, core::mem::transmute_copy(&nameindex), core::mem::transmute_copy(&namechoice), core::mem::transmute(&strname)).into()
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeAltName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbinary: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeAltName_Impl::Encode(this) {
                Ok(ok__) => {
                    pstrbinary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetNameCount: GetNameCount::<Identity, OFFSET>,
            GetNameChoice: GetNameChoice::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetNameEntry: SetNameEntry::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeAltName as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeAltName2_Impl: Sized + ICertEncodeAltName_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn GetNameBlob(&self, nameindex: i32, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn SetNameEntryBlob(&self, nameindex: i32, namechoice: i32, strname: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeAltName2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeAltName2_Vtbl {
    pub const fn new<Identity: ICertEncodeAltName2_Impl, const OFFSET: isize>() -> ICertEncodeAltName2_Vtbl {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeAltName2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeAltName2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeAltName2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeAltName2_Impl::EncodeBlob(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrencodeddata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameBlob<Identity: ICertEncodeAltName2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nameindex: i32, encoding: EncodingType, pstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeAltName2_Impl::GetNameBlob(this, core::mem::transmute_copy(&nameindex), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNameEntryBlob<Identity: ICertEncodeAltName2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nameindex: i32, namechoice: i32, strname: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeAltName2_Impl::SetNameEntryBlob(this, core::mem::transmute_copy(&nameindex), core::mem::transmute_copy(&namechoice), core::mem::transmute(&strname), core::mem::transmute_copy(&encoding)).into()
        }
        Self {
            base__: ICertEncodeAltName_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
            GetNameBlob: GetNameBlob::<Identity, OFFSET>,
            SetNameEntryBlob: SetNameEntryBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeAltName2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeAltName as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeBitString_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetBitCount(&self) -> windows_core::Result<i32>;
    fn GetBitString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Encode(&self, bitcount: i32, strbitstring: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeBitString {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeBitString_Vtbl {
    pub const fn new<Identity: ICertEncodeBitString_Impl, const OFFSET: isize>() -> ICertEncodeBitString_Vtbl {
        unsafe extern "system" fn Decode<Identity: ICertEncodeBitString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeBitString_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
        }
        unsafe extern "system" fn GetBitCount<Identity: ICertEncodeBitString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitcount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeBitString_Impl::GetBitCount(this) {
                Ok(ok__) => {
                    pbitcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitString<Identity: ICertEncodeBitString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbitstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeBitString_Impl::GetBitString(this) {
                Ok(ok__) => {
                    pstrbitstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeBitString_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitcount: i32, strbitstring: core::mem::MaybeUninit<windows_core::BSTR>, pstrbinary: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeBitString_Impl::Encode(this, core::mem::transmute_copy(&bitcount), core::mem::transmute(&strbitstring)) {
                Ok(ok__) => {
                    pstrbinary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetBitCount: GetBitCount::<Identity, OFFSET>,
            GetBitString: GetBitString::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeBitString as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeBitString2_Impl: Sized + ICertEncodeBitString_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, bitcount: i32, strbitstring: &windows_core::BSTR, encodingin: EncodingType, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn GetBitStringBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeBitString2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeBitString2_Vtbl {
    pub const fn new<Identity: ICertEncodeBitString2_Impl, const OFFSET: isize>() -> ICertEncodeBitString2_Vtbl {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeBitString2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeBitString2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeBitString2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bitcount: i32, strbitstring: core::mem::MaybeUninit<windows_core::BSTR>, encodingin: EncodingType, encoding: EncodingType, pstrencodeddata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeBitString2_Impl::EncodeBlob(this, core::mem::transmute_copy(&bitcount), core::mem::transmute(&strbitstring), core::mem::transmute_copy(&encodingin), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrencodeddata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBitStringBlob<Identity: ICertEncodeBitString2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pstrbitstring: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeBitString2_Impl::GetBitStringBlob(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrbitstring.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertEncodeBitString_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
            GetBitStringBlob: GetBitStringBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeBitString2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeBitString as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeCRLDistInfo_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDistPointCount(&self) -> windows_core::Result<i32>;
    fn GetNameCount(&self, distpointindex: i32) -> windows_core::Result<i32>;
    fn GetNameChoice(&self, distpointindex: i32, nameindex: i32) -> windows_core::Result<i32>;
    fn GetName(&self, distpointindex: i32, nameindex: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Reset(&self, distpointcount: i32) -> windows_core::Result<()>;
    fn SetNameCount(&self, distpointindex: i32, namecount: i32) -> windows_core::Result<()>;
    fn SetNameEntry(&self, distpointindex: i32, nameindex: i32, namechoice: CERT_ALT_NAME, strname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeCRLDistInfo {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeCRLDistInfo_Vtbl {
    pub const fn new<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>() -> ICertEncodeCRLDistInfo_Vtbl {
        unsafe extern "system" fn Decode<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeCRLDistInfo_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
        }
        unsafe extern "system" fn GetDistPointCount<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdistpointcount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeCRLDistInfo_Impl::GetDistPointCount(this) {
                Ok(ok__) => {
                    pdistpointcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameCount<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointindex: i32, pnamecount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeCRLDistInfo_Impl::GetNameCount(this, core::mem::transmute_copy(&distpointindex)) {
                Ok(ok__) => {
                    pnamecount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNameChoice<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointindex: i32, nameindex: i32, pnamechoice: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeCRLDistInfo_Impl::GetNameChoice(this, core::mem::transmute_copy(&distpointindex), core::mem::transmute_copy(&nameindex)) {
                Ok(ok__) => {
                    pnamechoice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointindex: i32, nameindex: i32, pstrname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeCRLDistInfo_Impl::GetName(this, core::mem::transmute_copy(&distpointindex), core::mem::transmute_copy(&nameindex)) {
                Ok(ok__) => {
                    pstrname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointcount: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeCRLDistInfo_Impl::Reset(this, core::mem::transmute_copy(&distpointcount)).into()
        }
        unsafe extern "system" fn SetNameCount<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointindex: i32, namecount: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeCRLDistInfo_Impl::SetNameCount(this, core::mem::transmute_copy(&distpointindex), core::mem::transmute_copy(&namecount)).into()
        }
        unsafe extern "system" fn SetNameEntry<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, distpointindex: i32, nameindex: i32, namechoice: CERT_ALT_NAME, strname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeCRLDistInfo_Impl::SetNameEntry(this, core::mem::transmute_copy(&distpointindex), core::mem::transmute_copy(&nameindex), core::mem::transmute_copy(&namechoice), core::mem::transmute(&strname)).into()
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeCRLDistInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbinary: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeCRLDistInfo_Impl::Encode(this) {
                Ok(ok__) => {
                    pstrbinary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetDistPointCount: GetDistPointCount::<Identity, OFFSET>,
            GetNameCount: GetNameCount::<Identity, OFFSET>,
            GetNameChoice: GetNameChoice::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetNameCount: SetNameCount::<Identity, OFFSET>,
            SetNameEntry: SetNameEntry::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeCRLDistInfo as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeCRLDistInfo2_Impl: Sized + ICertEncodeCRLDistInfo_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeCRLDistInfo2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeCRLDistInfo2_Vtbl {
    pub const fn new<Identity: ICertEncodeCRLDistInfo2_Impl, const OFFSET: isize>() -> ICertEncodeCRLDistInfo2_Vtbl {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeCRLDistInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeCRLDistInfo2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeCRLDistInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeCRLDistInfo2_Impl::EncodeBlob(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrencodeddata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertEncodeCRLDistInfo_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeCRLDistInfo2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeCRLDistInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeDateArray_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetValue(&self, index: i32) -> windows_core::Result<f64>;
    fn Reset(&self, count: i32) -> windows_core::Result<()>;
    fn SetValue(&self, index: i32, value: f64) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeDateArray {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeDateArray_Vtbl {
    pub const fn new<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>() -> ICertEncodeDateArray_Vtbl {
        unsafe extern "system" fn Decode<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeDateArray_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
        }
        unsafe extern "system" fn GetCount<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeDateArray_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeDateArray_Impl::GetValue(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeDateArray_Impl::Reset(this, core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetValue<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeDateArray_Impl::SetValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeDateArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbinary: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeDateArray_Impl::Encode(this) {
                Ok(ok__) => {
                    pstrbinary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeDateArray as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeDateArray2_Impl: Sized + ICertEncodeDateArray_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeDateArray2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeDateArray2_Vtbl {
    pub const fn new<Identity: ICertEncodeDateArray2_Impl, const OFFSET: isize>() -> ICertEncodeDateArray2_Vtbl {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeDateArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeDateArray2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeDateArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeDateArray2_Impl::EncodeBlob(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrencodeddata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertEncodeDateArray_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeDateArray2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeDateArray as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeLongArray_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetValue(&self, index: i32) -> windows_core::Result<i32>;
    fn Reset(&self, count: i32) -> windows_core::Result<()>;
    fn SetValue(&self, index: i32, value: i32) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeLongArray {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeLongArray_Vtbl {
    pub const fn new<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>() -> ICertEncodeLongArray_Vtbl {
        unsafe extern "system" fn Decode<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeLongArray_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
        }
        unsafe extern "system" fn GetCount<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeLongArray_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeLongArray_Impl::GetValue(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeLongArray_Impl::Reset(this, core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetValue<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeLongArray_Impl::SetValue(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeLongArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbinary: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeLongArray_Impl::Encode(this) {
                Ok(ok__) => {
                    pstrbinary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeLongArray as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeLongArray2_Impl: Sized + ICertEncodeLongArray_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeLongArray2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeLongArray2_Vtbl {
    pub const fn new<Identity: ICertEncodeLongArray2_Impl, const OFFSET: isize>() -> ICertEncodeLongArray2_Vtbl {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeLongArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeLongArray2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeLongArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeLongArray2_Impl::EncodeBlob(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrencodeddata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertEncodeLongArray_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeLongArray2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeLongArray as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeStringArray_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Decode(&self, strbinary: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetStringType(&self) -> windows_core::Result<i32>;
    fn GetCount(&self) -> windows_core::Result<i32>;
    fn GetValue(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Reset(&self, count: i32, stringtype: super::CERT_RDN_ATTR_VALUE_TYPE) -> windows_core::Result<()>;
    fn SetValue(&self, index: i32, str: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeStringArray {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeStringArray_Vtbl {
    pub const fn new<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>() -> ICertEncodeStringArray_Vtbl {
        unsafe extern "system" fn Decode<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strbinary: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeStringArray_Impl::Decode(this, core::mem::transmute(&strbinary)).into()
        }
        unsafe extern "system" fn GetStringType<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstringtype: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeStringArray_Impl::GetStringType(this) {
                Ok(ok__) => {
                    pstringtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeStringArray_Impl::GetCount(this) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pstr: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeStringArray_Impl::GetValue(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pstr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: i32, stringtype: super::CERT_RDN_ATTR_VALUE_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeStringArray_Impl::Reset(this, core::mem::transmute_copy(&count), core::mem::transmute_copy(&stringtype)).into()
        }
        unsafe extern "system" fn SetValue<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, str: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeStringArray_Impl::SetValue(this, core::mem::transmute_copy(&index), core::mem::transmute(&str)).into()
        }
        unsafe extern "system" fn Encode<Identity: ICertEncodeStringArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrbinary: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeStringArray_Impl::Encode(this) {
                Ok(ok__) => {
                    pstrbinary.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            GetStringType: GetStringType::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeStringArray as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertEncodeStringArray2_Impl: Sized + ICertEncodeStringArray_Impl {
    fn DecodeBlob(&self, strencodeddata: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn EncodeBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertEncodeStringArray2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertEncodeStringArray2_Vtbl {
    pub const fn new<Identity: ICertEncodeStringArray2_Impl, const OFFSET: isize>() -> ICertEncodeStringArray2_Vtbl {
        unsafe extern "system" fn DecodeBlob<Identity: ICertEncodeStringArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertEncodeStringArray2_Impl::DecodeBlob(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn EncodeBlob<Identity: ICertEncodeStringArray2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pstrencodeddata: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertEncodeStringArray2_Impl::EncodeBlob(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrencodeddata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertEncodeStringArray_Vtbl::new::<Identity, OFFSET>(),
            DecodeBlob: DecodeBlob::<Identity, OFFSET>,
            EncodeBlob: EncodeBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertEncodeStringArray2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertEncodeStringArray as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertExit_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<CERT_EXIT_EVENT_MASK>;
    fn Notify(&self, exitevent: i32, context: i32) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertExit {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertExit_Vtbl {
    pub const fn new<Identity: ICertExit_Impl, const OFFSET: isize>() -> ICertExit_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, peventmask: *mut CERT_EXIT_EVENT_MASK) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertExit_Impl::Initialize(this, core::mem::transmute(&strconfig)) {
                Ok(ok__) => {
                    peventmask.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Identity: ICertExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, exitevent: i32, context: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertExit_Impl::Notify(this, core::mem::transmute_copy(&exitevent), core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: ICertExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertExit_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertExit as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertExit2_Impl: Sized + ICertExit_Impl {
    fn GetManageModule(&self) -> windows_core::Result<ICertManageModule>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertExit2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertExit2_Vtbl {
    pub const fn new<Identity: ICertExit2_Impl, const OFFSET: isize>() -> ICertExit2_Vtbl {
        unsafe extern "system" fn GetManageModule<Identity: ICertExit2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmanagemodule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertExit2_Impl::GetManageModule(this) {
                Ok(ok__) => {
                    ppmanagemodule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ICertExit_Vtbl::new::<Identity, OFFSET>(), GetManageModule: GetManageModule::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertExit2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertExit as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertGetConfig_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetConfig(&self, flags: CERT_GET_CONFIG_FLAGS) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertGetConfig {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertGetConfig_Vtbl {
    pub const fn new<Identity: ICertGetConfig_Impl, const OFFSET: isize>() -> ICertGetConfig_Vtbl {
        unsafe extern "system" fn GetConfig<Identity: ICertGetConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: CERT_GET_CONFIG_FLAGS, pstrout: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertGetConfig_Impl::GetConfig(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pstrout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), GetConfig: GetConfig::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertGetConfig as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertManageModule_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetProperty(&self, strconfig: &windows_core::BSTR, strstoragelocation: &windows_core::BSTR, strpropertyname: &windows_core::BSTR, flags: i32) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetProperty(&self, strconfig: &windows_core::BSTR, strstoragelocation: &windows_core::BSTR, strpropertyname: &windows_core::BSTR, flags: i32, pvarproperty: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Configure(&self, strconfig: &windows_core::BSTR, strstoragelocation: &windows_core::BSTR, flags: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertManageModule {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertManageModule_Vtbl {
    pub const fn new<Identity: ICertManageModule_Impl, const OFFSET: isize>() -> ICertManageModule_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: ICertManageModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, strstoragelocation: core::mem::MaybeUninit<windows_core::BSTR>, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32, pvarproperty: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertManageModule_Impl::GetProperty(this, core::mem::transmute(&strconfig), core::mem::transmute(&strstoragelocation), core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pvarproperty.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ICertManageModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, strstoragelocation: core::mem::MaybeUninit<windows_core::BSTR>, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32, pvarproperty: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertManageModule_Impl::SetProperty(this, core::mem::transmute(&strconfig), core::mem::transmute(&strstoragelocation), core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pvarproperty)).into()
        }
        unsafe extern "system" fn Configure<Identity: ICertManageModule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, strstoragelocation: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertManageModule_Impl::Configure(this, core::mem::transmute(&strconfig), core::mem::transmute(&strstoragelocation), core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            Configure: Configure::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertManageModule as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPolicy_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<()>;
    fn VerifyRequest(&self, strconfig: &windows_core::BSTR, context: i32, bnewrequest: i32, flags: i32) -> windows_core::Result<i32>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ShutDown(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPolicy {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPolicy_Vtbl {
    pub const fn new<Identity: ICertPolicy_Impl, const OFFSET: isize>() -> ICertPolicy_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPolicy_Impl::Initialize(this, core::mem::transmute(&strconfig)).into()
        }
        unsafe extern "system" fn VerifyRequest<Identity: ICertPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, context: i32, bnewrequest: i32, flags: i32, pdisposition: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPolicy_Impl::VerifyRequest(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&context), core::mem::transmute_copy(&bnewrequest), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pdisposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ICertPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdescription: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPolicy_Impl::GetDescription(this) {
                Ok(ok__) => {
                    pstrdescription.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutDown<Identity: ICertPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPolicy_Impl::ShutDown(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            VerifyRequest: VerifyRequest::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
            ShutDown: ShutDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPolicy as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPolicy2_Impl: Sized + ICertPolicy_Impl {
    fn GetManageModule(&self) -> windows_core::Result<ICertManageModule>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPolicy2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPolicy2_Vtbl {
    pub const fn new<Identity: ICertPolicy2_Impl, const OFFSET: isize>() -> ICertPolicy2_Vtbl {
        unsafe extern "system" fn GetManageModule<Identity: ICertPolicy2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmanagemodule: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPolicy2_Impl::GetManageModule(this) {
                Ok(ok__) => {
                    ppmanagemodule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ICertPolicy_Vtbl::new::<Identity, OFFSET>(), GetManageModule: GetManageModule::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPolicy2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertPolicy as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertProperties_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICertProperty>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&ICertProperty>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn InitializeFromCertificate(&self, machinecontext: super::super::super::Foundation::VARIANT_BOOL, encoding: EncodingType, strcertificate: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertProperties {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertProperties_Vtbl {
    pub const fn new<Identity: ICertProperties_Impl, const OFFSET: isize>() -> ICertProperties_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: ICertProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertProperties_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ICertProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertProperties_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ICertProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertProperties_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ICertProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertProperties_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: ICertProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertProperties_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ICertProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertProperties_Impl::Clear(this).into()
        }
        unsafe extern "system" fn InitializeFromCertificate<Identity: ICertProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, machinecontext: super::super::super::Foundation::VARIANT_BOOL, encoding: EncodingType, strcertificate: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertProperties_Impl::InitializeFromCertificate(this, core::mem::transmute_copy(&machinecontext), core::mem::transmute_copy(&encoding), core::mem::transmute(&strcertificate)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            InitializeFromCertificate: InitializeFromCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertProperties as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertProperty_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn InitializeFromCertificate(&self, machinecontext: super::super::super::Foundation::VARIANT_BOOL, encoding: EncodingType, strcertificate: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn PropertyId(&self) -> windows_core::Result<CERTENROLL_PROPERTYID>;
    fn SetPropertyId(&self, value: CERTENROLL_PROPERTYID) -> windows_core::Result<()>;
    fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn RemoveFromCertificate(&self, machinecontext: super::super::super::Foundation::VARIANT_BOOL, encoding: EncodingType, strcertificate: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetValueOnCertificate(&self, machinecontext: super::super::super::Foundation::VARIANT_BOOL, encoding: EncodingType, strcertificate: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertProperty {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertProperty_Vtbl {
    pub const fn new<Identity: ICertProperty_Impl, const OFFSET: isize>() -> ICertProperty_Vtbl {
        unsafe extern "system" fn InitializeFromCertificate<Identity: ICertProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, machinecontext: super::super::super::Foundation::VARIANT_BOOL, encoding: EncodingType, strcertificate: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertProperty_Impl::InitializeFromCertificate(this, core::mem::transmute_copy(&machinecontext), core::mem::transmute_copy(&encoding), core::mem::transmute(&strcertificate)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: ICertProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertProperty_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn PropertyId<Identity: ICertProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut CERTENROLL_PROPERTYID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertProperty_Impl::PropertyId(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyId<Identity: ICertProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: CERTENROLL_PROPERTYID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertProperty_Impl::SetPropertyId(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn get_RawData<Identity: ICertProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertProperty_Impl::get_RawData(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveFromCertificate<Identity: ICertProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, machinecontext: super::super::super::Foundation::VARIANT_BOOL, encoding: EncodingType, strcertificate: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertProperty_Impl::RemoveFromCertificate(this, core::mem::transmute_copy(&machinecontext), core::mem::transmute_copy(&encoding), core::mem::transmute(&strcertificate)).into()
        }
        unsafe extern "system" fn SetValueOnCertificate<Identity: ICertProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, machinecontext: super::super::super::Foundation::VARIANT_BOOL, encoding: EncodingType, strcertificate: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertProperty_Impl::SetValueOnCertificate(this, core::mem::transmute_copy(&machinecontext), core::mem::transmute_copy(&encoding), core::mem::transmute(&strcertificate)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromCertificate: InitializeFromCertificate::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            PropertyId: PropertyId::<Identity, OFFSET>,
            SetPropertyId: SetPropertyId::<Identity, OFFSET>,
            get_RawData: get_RawData::<Identity, OFFSET>,
            RemoveFromCertificate: RemoveFromCertificate::<Identity, OFFSET>,
            SetValueOnCertificate: SetValueOnCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertProperty as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertyArchived_Impl: Sized + ICertProperty_Impl {
    fn Initialize(&self, archivedvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Archived(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertyArchived {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertyArchived_Vtbl {
    pub const fn new<Identity: ICertPropertyArchived_Impl, const OFFSET: isize>() -> ICertPropertyArchived_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPropertyArchived_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, archivedvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyArchived_Impl::Initialize(this, core::mem::transmute_copy(&archivedvalue)).into()
        }
        unsafe extern "system" fn Archived<Identity: ICertPropertyArchived_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyArchived_Impl::Archived(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET>, Archived: Archived::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertyArchived as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertyArchivedKeyHash_Impl: Sized + ICertProperty_Impl {
    fn Initialize(&self, encoding: EncodingType, strarchivedkeyhashvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ArchivedKeyHash(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertyArchivedKeyHash {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertyArchivedKeyHash_Vtbl {
    pub const fn new<Identity: ICertPropertyArchivedKeyHash_Impl, const OFFSET: isize>() -> ICertPropertyArchivedKeyHash_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPropertyArchivedKeyHash_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strarchivedkeyhashvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyArchivedKeyHash_Impl::Initialize(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strarchivedkeyhashvalue)).into()
        }
        unsafe extern "system" fn get_ArchivedKeyHash<Identity: ICertPropertyArchivedKeyHash_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyArchivedKeyHash_Impl::get_ArchivedKeyHash(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            get_ArchivedKeyHash: get_ArchivedKeyHash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertyArchivedKeyHash as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertyAutoEnroll_Impl: Sized + ICertProperty_Impl {
    fn Initialize(&self, strtemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TemplateName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertyAutoEnroll {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertyAutoEnroll_Vtbl {
    pub const fn new<Identity: ICertPropertyAutoEnroll_Impl, const OFFSET: isize>() -> ICertPropertyAutoEnroll_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPropertyAutoEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strtemplatename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyAutoEnroll_Impl::Initialize(this, core::mem::transmute(&strtemplatename)).into()
        }
        unsafe extern "system" fn TemplateName<Identity: ICertPropertyAutoEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyAutoEnroll_Impl::TemplateName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            TemplateName: TemplateName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertyAutoEnroll as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertyBackedUp_Impl: Sized + ICertProperty_Impl {
    fn InitializeFromCurrentTime(&self, backedupvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Initialize(&self, backedupvalue: super::super::super::Foundation::VARIANT_BOOL, date: f64) -> windows_core::Result<()>;
    fn BackedUpValue(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn BackedUpTime(&self) -> windows_core::Result<f64>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertyBackedUp {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertyBackedUp_Vtbl {
    pub const fn new<Identity: ICertPropertyBackedUp_Impl, const OFFSET: isize>() -> ICertPropertyBackedUp_Vtbl {
        unsafe extern "system" fn InitializeFromCurrentTime<Identity: ICertPropertyBackedUp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, backedupvalue: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyBackedUp_Impl::InitializeFromCurrentTime(this, core::mem::transmute_copy(&backedupvalue)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ICertPropertyBackedUp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, backedupvalue: super::super::super::Foundation::VARIANT_BOOL, date: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyBackedUp_Impl::Initialize(this, core::mem::transmute_copy(&backedupvalue), core::mem::transmute_copy(&date)).into()
        }
        unsafe extern "system" fn BackedUpValue<Identity: ICertPropertyBackedUp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyBackedUp_Impl::BackedUpValue(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackedUpTime<Identity: ICertPropertyBackedUp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyBackedUp_Impl::BackedUpTime(this) {
                Ok(ok__) => {
                    pdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromCurrentTime: InitializeFromCurrentTime::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            BackedUpValue: BackedUpValue::<Identity, OFFSET>,
            BackedUpTime: BackedUpTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertyBackedUp as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertyDescription_Impl: Sized + ICertProperty_Impl {
    fn Initialize(&self, strdescription: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertyDescription {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertyDescription_Vtbl {
    pub const fn new<Identity: ICertPropertyDescription_Impl, const OFFSET: isize>() -> ICertPropertyDescription_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strdescription: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyDescription_Impl::Initialize(this, core::mem::transmute(&strdescription)).into()
        }
        unsafe extern "system" fn Description<Identity: ICertPropertyDescription_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyDescription_Impl::Description(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET>, Description: Description::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertyDescription as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertyEnrollment_Impl: Sized + ICertProperty_Impl {
    fn Initialize(&self, requestid: i32, strcadnsname: &windows_core::BSTR, strcaname: &windows_core::BSTR, strfriendlyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RequestId(&self) -> windows_core::Result<i32>;
    fn CADnsName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CAName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertyEnrollment {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertyEnrollment_Vtbl {
    pub const fn new<Identity: ICertPropertyEnrollment_Impl, const OFFSET: isize>() -> ICertPropertyEnrollment_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPropertyEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: i32, strcadnsname: core::mem::MaybeUninit<windows_core::BSTR>, strcaname: core::mem::MaybeUninit<windows_core::BSTR>, strfriendlyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyEnrollment_Impl::Initialize(this, core::mem::transmute_copy(&requestid), core::mem::transmute(&strcadnsname), core::mem::transmute(&strcaname), core::mem::transmute(&strfriendlyname)).into()
        }
        unsafe extern "system" fn RequestId<Identity: ICertPropertyEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollment_Impl::RequestId(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CADnsName<Identity: ICertPropertyEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollment_Impl::CADnsName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAName<Identity: ICertPropertyEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollment_Impl::CAName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: ICertPropertyEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollment_Impl::FriendlyName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            RequestId: RequestId::<Identity, OFFSET>,
            CADnsName: CADnsName::<Identity, OFFSET>,
            CAName: CAName::<Identity, OFFSET>,
            FriendlyName: FriendlyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertyEnrollment as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertyEnrollmentPolicyServer_Impl: Sized + ICertProperty_Impl {
    fn Initialize(&self, propertyflags: EnrollmentPolicyServerPropertyFlags, authflags: X509EnrollmentAuthFlags, enrollmentserverauthflags: X509EnrollmentAuthFlags, urlflags: PolicyServerUrlFlags, strrequestid: &windows_core::BSTR, strurl: &windows_core::BSTR, strid: &windows_core::BSTR, strenrollmentserverurl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetPolicyServerUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPolicyServerId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetEnrollmentServerUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRequestIdString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPropertyFlags(&self) -> windows_core::Result<EnrollmentPolicyServerPropertyFlags>;
    fn GetUrlFlags(&self) -> windows_core::Result<PolicyServerUrlFlags>;
    fn GetAuthentication(&self) -> windows_core::Result<X509EnrollmentAuthFlags>;
    fn GetEnrollmentServerAuthentication(&self) -> windows_core::Result<X509EnrollmentAuthFlags>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertyEnrollmentPolicyServer {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertyEnrollmentPolicyServer_Vtbl {
    pub const fn new<Identity: ICertPropertyEnrollmentPolicyServer_Impl, const OFFSET: isize>() -> ICertPropertyEnrollmentPolicyServer_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPropertyEnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyflags: EnrollmentPolicyServerPropertyFlags, authflags: X509EnrollmentAuthFlags, enrollmentserverauthflags: X509EnrollmentAuthFlags, urlflags: PolicyServerUrlFlags, strrequestid: core::mem::MaybeUninit<windows_core::BSTR>, strurl: core::mem::MaybeUninit<windows_core::BSTR>, strid: core::mem::MaybeUninit<windows_core::BSTR>, strenrollmentserverurl: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyEnrollmentPolicyServer_Impl::Initialize(this, core::mem::transmute_copy(&propertyflags), core::mem::transmute_copy(&authflags), core::mem::transmute_copy(&enrollmentserverauthflags), core::mem::transmute_copy(&urlflags), core::mem::transmute(&strrequestid), core::mem::transmute(&strurl), core::mem::transmute(&strid), core::mem::transmute(&strenrollmentserverurl)).into()
        }
        unsafe extern "system" fn GetPolicyServerUrl<Identity: ICertPropertyEnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollmentPolicyServer_Impl::GetPolicyServerUrl(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicyServerId<Identity: ICertPropertyEnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollmentPolicyServer_Impl::GetPolicyServerId(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnrollmentServerUrl<Identity: ICertPropertyEnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollmentPolicyServer_Impl::GetEnrollmentServerUrl(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestIdString<Identity: ICertPropertyEnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollmentPolicyServer_Impl::GetRequestIdString(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyFlags<Identity: ICertPropertyEnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut EnrollmentPolicyServerPropertyFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollmentPolicyServer_Impl::GetPropertyFlags(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUrlFlags<Identity: ICertPropertyEnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut PolicyServerUrlFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollmentPolicyServer_Impl::GetUrlFlags(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthentication<Identity: ICertPropertyEnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollmentPolicyServer_Impl::GetAuthentication(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEnrollmentServerAuthentication<Identity: ICertPropertyEnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyEnrollmentPolicyServer_Impl::GetEnrollmentServerAuthentication(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetPolicyServerUrl: GetPolicyServerUrl::<Identity, OFFSET>,
            GetPolicyServerId: GetPolicyServerId::<Identity, OFFSET>,
            GetEnrollmentServerUrl: GetEnrollmentServerUrl::<Identity, OFFSET>,
            GetRequestIdString: GetRequestIdString::<Identity, OFFSET>,
            GetPropertyFlags: GetPropertyFlags::<Identity, OFFSET>,
            GetUrlFlags: GetUrlFlags::<Identity, OFFSET>,
            GetAuthentication: GetAuthentication::<Identity, OFFSET>,
            GetEnrollmentServerAuthentication: GetEnrollmentServerAuthentication::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertyEnrollmentPolicyServer as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertyFriendlyName_Impl: Sized + ICertProperty_Impl {
    fn Initialize(&self, strfriendlyname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertyFriendlyName {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertyFriendlyName_Vtbl {
    pub const fn new<Identity: ICertPropertyFriendlyName_Impl, const OFFSET: isize>() -> ICertPropertyFriendlyName_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPropertyFriendlyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strfriendlyname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyFriendlyName_Impl::Initialize(this, core::mem::transmute(&strfriendlyname)).into()
        }
        unsafe extern "system" fn FriendlyName<Identity: ICertPropertyFriendlyName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyFriendlyName_Impl::FriendlyName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            FriendlyName: FriendlyName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertyFriendlyName as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertyKeyProvInfo_Impl: Sized + ICertProperty_Impl {
    fn Initialize(&self, pvalue: Option<&IX509PrivateKey>) -> windows_core::Result<()>;
    fn PrivateKey(&self) -> windows_core::Result<IX509PrivateKey>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertyKeyProvInfo {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertyKeyProvInfo_Vtbl {
    pub const fn new<Identity: ICertPropertyKeyProvInfo_Impl, const OFFSET: isize>() -> ICertPropertyKeyProvInfo_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPropertyKeyProvInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyKeyProvInfo_Impl::Initialize(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn PrivateKey<Identity: ICertPropertyKeyProvInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyKeyProvInfo_Impl::PrivateKey(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET>, PrivateKey: PrivateKey::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertyKeyProvInfo as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertyRenewal_Impl: Sized + ICertProperty_Impl {
    fn Initialize(&self, encoding: EncodingType, strrenewalvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromCertificateHash(&self, machinecontext: super::super::super::Foundation::VARIANT_BOOL, encoding: EncodingType, strcertificate: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Renewal(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertyRenewal {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertyRenewal_Vtbl {
    pub const fn new<Identity: ICertPropertyRenewal_Impl, const OFFSET: isize>() -> ICertPropertyRenewal_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPropertyRenewal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strrenewalvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyRenewal_Impl::Initialize(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strrenewalvalue)).into()
        }
        unsafe extern "system" fn InitializeFromCertificateHash<Identity: ICertPropertyRenewal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, machinecontext: super::super::super::Foundation::VARIANT_BOOL, encoding: EncodingType, strcertificate: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyRenewal_Impl::InitializeFromCertificateHash(this, core::mem::transmute_copy(&machinecontext), core::mem::transmute_copy(&encoding), core::mem::transmute(&strcertificate)).into()
        }
        unsafe extern "system" fn get_Renewal<Identity: ICertPropertyRenewal_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyRenewal_Impl::get_Renewal(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            InitializeFromCertificateHash: InitializeFromCertificateHash::<Identity, OFFSET>,
            get_Renewal: get_Renewal::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertyRenewal as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertyRequestOriginator_Impl: Sized + ICertProperty_Impl {
    fn Initialize(&self, strrequestoriginator: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromLocalRequestOriginator(&self) -> windows_core::Result<()>;
    fn RequestOriginator(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertyRequestOriginator {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertyRequestOriginator_Vtbl {
    pub const fn new<Identity: ICertPropertyRequestOriginator_Impl, const OFFSET: isize>() -> ICertPropertyRequestOriginator_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPropertyRequestOriginator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strrequestoriginator: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyRequestOriginator_Impl::Initialize(this, core::mem::transmute(&strrequestoriginator)).into()
        }
        unsafe extern "system" fn InitializeFromLocalRequestOriginator<Identity: ICertPropertyRequestOriginator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertyRequestOriginator_Impl::InitializeFromLocalRequestOriginator(this).into()
        }
        unsafe extern "system" fn RequestOriginator<Identity: ICertPropertyRequestOriginator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertyRequestOriginator_Impl::RequestOriginator(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            InitializeFromLocalRequestOriginator: InitializeFromLocalRequestOriginator::<Identity, OFFSET>,
            RequestOriginator: RequestOriginator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertyRequestOriginator as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertPropertySHA1Hash_Impl: Sized + ICertProperty_Impl {
    fn Initialize(&self, encoding: EncodingType, strrenewalvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SHA1Hash(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertPropertySHA1Hash {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertPropertySHA1Hash_Vtbl {
    pub const fn new<Identity: ICertPropertySHA1Hash_Impl, const OFFSET: isize>() -> ICertPropertySHA1Hash_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertPropertySHA1Hash_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strrenewalvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertPropertySHA1Hash_Impl::Initialize(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strrenewalvalue)).into()
        }
        unsafe extern "system" fn get_SHA1Hash<Identity: ICertPropertySHA1Hash_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertPropertySHA1Hash_Impl::get_SHA1Hash(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertProperty_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            get_SHA1Hash: get_SHA1Hash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertPropertySHA1Hash as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertProperty as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertRequest_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Submit(&self, flags: i32, strrequest: &windows_core::BSTR, strattributes: &windows_core::BSTR, strconfig: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn RetrievePending(&self, requestid: i32, strconfig: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn GetLastStatus(&self) -> windows_core::Result<i32>;
    fn GetRequestId(&self) -> windows_core::Result<i32>;
    fn GetDispositionMessage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCACertificate(&self, fexchangecertificate: i32, strconfig: &windows_core::BSTR, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetCertificate(&self, flags: i32) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertRequest {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertRequest_Vtbl {
    pub const fn new<Identity: ICertRequest_Impl, const OFFSET: isize>() -> ICertRequest_Vtbl {
        unsafe extern "system" fn Submit<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, strrequest: core::mem::MaybeUninit<windows_core::BSTR>, strattributes: core::mem::MaybeUninit<windows_core::BSTR>, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, pdisposition: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest_Impl::Submit(this, core::mem::transmute_copy(&flags), core::mem::transmute(&strrequest), core::mem::transmute(&strattributes), core::mem::transmute(&strconfig)) {
                Ok(ok__) => {
                    pdisposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetrievePending<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestid: i32, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, pdisposition: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest_Impl::RetrievePending(this, core::mem::transmute_copy(&requestid), core::mem::transmute(&strconfig)) {
                Ok(ok__) => {
                    pdisposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastStatus<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest_Impl::GetLastStatus(this) {
                Ok(ok__) => {
                    pstatus.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestId<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequestid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest_Impl::GetRequestId(this) {
                Ok(ok__) => {
                    prequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDispositionMessage<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrdispositionmessage: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest_Impl::GetDispositionMessage(this) {
                Ok(ok__) => {
                    pstrdispositionmessage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCACertificate<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fexchangecertificate: i32, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, flags: i32, pstrcertificate: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest_Impl::GetCACertificate(this, core::mem::transmute_copy(&fexchangecertificate), core::mem::transmute(&strconfig), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pstrcertificate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificate<Identity: ICertRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pstrcertificate: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest_Impl::GetCertificate(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pstrcertificate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Submit: Submit::<Identity, OFFSET>,
            RetrievePending: RetrievePending::<Identity, OFFSET>,
            GetLastStatus: GetLastStatus::<Identity, OFFSET>,
            GetRequestId: GetRequestId::<Identity, OFFSET>,
            GetDispositionMessage: GetDispositionMessage::<Identity, OFFSET>,
            GetCACertificate: GetCACertificate::<Identity, OFFSET>,
            GetCertificate: GetCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertRequest as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertRequest2_Impl: Sized + ICertRequest_Impl {
    fn GetIssuedCertificate(&self, strconfig: &windows_core::BSTR, requestid: i32, strserialnumber: &windows_core::BSTR) -> windows_core::Result<CR_DISP>;
    fn GetErrorMessageText(&self, hrmessage: i32, flags: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetCAProperty(&self, strconfig: &windows_core::BSTR, propid: i32, propindex: i32, proptype: i32, flags: i32) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn GetCAPropertyFlags(&self, strconfig: &windows_core::BSTR, propid: i32) -> windows_core::Result<i32>;
    fn GetCAPropertyDisplayName(&self, strconfig: &windows_core::BSTR, propid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn GetFullResponseProperty(&self, propid: FULL_RESPONSE_PROPERTY_ID, propindex: i32, proptype: CERT_PROPERTY_TYPE, flags: CERT_REQUEST_OUT_TYPE) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertRequest2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertRequest2_Vtbl {
    pub const fn new<Identity: ICertRequest2_Impl, const OFFSET: isize>() -> ICertRequest2_Vtbl {
        unsafe extern "system" fn GetIssuedCertificate<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, requestid: i32, strserialnumber: core::mem::MaybeUninit<windows_core::BSTR>, pdisposition: *mut CR_DISP) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest2_Impl::GetIssuedCertificate(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&requestid), core::mem::transmute(&strserialnumber)) {
                Ok(ok__) => {
                    pdisposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorMessageText<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrmessage: i32, flags: i32, pstrerrormessagetext: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest2_Impl::GetErrorMessageText(this, core::mem::transmute_copy(&hrmessage), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pstrerrormessagetext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAProperty<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, propid: i32, propindex: i32, proptype: i32, flags: i32, pvarpropertyvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest2_Impl::GetCAProperty(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid), core::mem::transmute_copy(&propindex), core::mem::transmute_copy(&proptype), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pvarpropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAPropertyFlags<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, propid: i32, ppropflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest2_Impl::GetCAPropertyFlags(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid)) {
                Ok(ok__) => {
                    ppropflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAPropertyDisplayName<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, propid: i32, pstrdisplayname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest2_Impl::GetCAPropertyDisplayName(this, core::mem::transmute(&strconfig), core::mem::transmute_copy(&propid)) {
                Ok(ok__) => {
                    pstrdisplayname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullResponseProperty<Identity: ICertRequest2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propid: FULL_RESPONSE_PROPERTY_ID, propindex: i32, proptype: CERT_PROPERTY_TYPE, flags: CERT_REQUEST_OUT_TYPE, pvarpropertyvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest2_Impl::GetFullResponseProperty(this, core::mem::transmute_copy(&propid), core::mem::transmute_copy(&propindex), core::mem::transmute_copy(&proptype), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    pvarpropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertRequest_Vtbl::new::<Identity, OFFSET>(),
            GetIssuedCertificate: GetIssuedCertificate::<Identity, OFFSET>,
            GetErrorMessageText: GetErrorMessageText::<Identity, OFFSET>,
            GetCAProperty: GetCAProperty::<Identity, OFFSET>,
            GetCAPropertyFlags: GetCAPropertyFlags::<Identity, OFFSET>,
            GetCAPropertyDisplayName: GetCAPropertyDisplayName::<Identity, OFFSET>,
            GetFullResponseProperty: GetFullResponseProperty::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertRequest2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertRequest as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertRequest3_Impl: Sized + ICertRequest2_Impl {
    fn SetCredential(&self, hwnd: i32, authtype: X509EnrollmentAuthFlags, strcredential: &windows_core::BSTR, strpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetRequestIdString(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetIssuedCertificate2(&self, strconfig: &windows_core::BSTR, strrequestid: &windows_core::BSTR, strserialnumber: &windows_core::BSTR) -> windows_core::Result<CR_DISP>;
    fn GetRefreshPolicy(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertRequest3 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertRequest3_Vtbl {
    pub const fn new<Identity: ICertRequest3_Impl, const OFFSET: isize>() -> ICertRequest3_Vtbl {
        unsafe extern "system" fn SetCredential<Identity: ICertRequest3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: i32, authtype: X509EnrollmentAuthFlags, strcredential: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertRequest3_Impl::SetCredential(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&authtype), core::mem::transmute(&strcredential), core::mem::transmute(&strpassword)).into()
        }
        unsafe extern "system" fn GetRequestIdString<Identity: ICertRequest3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrrequestid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest3_Impl::GetRequestIdString(this) {
                Ok(ok__) => {
                    pstrrequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIssuedCertificate2<Identity: ICertRequest3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>, strrequestid: core::mem::MaybeUninit<windows_core::BSTR>, strserialnumber: core::mem::MaybeUninit<windows_core::BSTR>, pdisposition: *mut CR_DISP) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest3_Impl::GetIssuedCertificate2(this, core::mem::transmute(&strconfig), core::mem::transmute(&strrequestid), core::mem::transmute(&strserialnumber)) {
                Ok(ok__) => {
                    pdisposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRefreshPolicy<Identity: ICertRequest3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequest3_Impl::GetRefreshPolicy(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: ICertRequest2_Vtbl::new::<Identity, OFFSET>(),
            SetCredential: SetCredential::<Identity, OFFSET>,
            GetRequestIdString: GetRequestIdString::<Identity, OFFSET>,
            GetIssuedCertificate2: GetIssuedCertificate2::<Identity, OFFSET>,
            GetRefreshPolicy: GetRefreshPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertRequest3 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertRequest as windows_core::Interface>::IID || iid == &<ICertRequest2 as windows_core::Interface>::IID
    }
}
pub trait ICertRequestD_Impl: Sized + windows_core::IUnknownImpl {
    fn Request(&self, dwflags: u32, pwszauthority: &windows_core::PCWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: &windows_core::PCWSTR, pctbrequest: *const CERTTRANSBLOB, pctbcertchain: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> windows_core::Result<()>;
    fn GetCACert(&self, fchain: u32, pwszauthority: &windows_core::PCWSTR) -> windows_core::Result<CERTTRANSBLOB>;
    fn Ping(&self, pwszauthority: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICertRequestD {}
impl ICertRequestD_Vtbl {
    pub const fn new<Identity: ICertRequestD_Impl, const OFFSET: isize>() -> ICertRequestD_Vtbl {
        unsafe extern "system" fn Request<Identity: ICertRequestD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pwszauthority: windows_core::PCWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: windows_core::PCWSTR, pctbrequest: *const CERTTRANSBLOB, pctbcertchain: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertRequestD_Impl::Request(this, core::mem::transmute_copy(&dwflags), core::mem::transmute(&pwszauthority), core::mem::transmute_copy(&pdwrequestid), core::mem::transmute_copy(&pdwdisposition), core::mem::transmute(&pwszattributes), core::mem::transmute_copy(&pctbrequest), core::mem::transmute_copy(&pctbcertchain), core::mem::transmute_copy(&pctbencodedcert), core::mem::transmute_copy(&pctbdispositionmessage)).into()
        }
        unsafe extern "system" fn GetCACert<Identity: ICertRequestD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fchain: u32, pwszauthority: windows_core::PCWSTR, pctbout: *mut CERTTRANSBLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequestD_Impl::GetCACert(this, core::mem::transmute_copy(&fchain), core::mem::transmute(&pwszauthority)) {
                Ok(ok__) => {
                    pctbout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ping<Identity: ICertRequestD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszauthority: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertRequestD_Impl::Ping(this, core::mem::transmute(&pwszauthority)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Request: Request::<Identity, OFFSET>,
            GetCACert: GetCACert::<Identity, OFFSET>,
            Ping: Ping::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertRequestD as windows_core::Interface>::IID
    }
}
pub trait ICertRequestD2_Impl: Sized + ICertRequestD_Impl {
    fn Request2(&self, pwszauthority: &windows_core::PCWSTR, dwflags: u32, pwszserialnumber: &windows_core::PCWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: &windows_core::PCWSTR, pctbrequest: *const CERTTRANSBLOB, pctbfullresponse: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> windows_core::Result<()>;
    fn GetCAProperty(&self, pwszauthority: &windows_core::PCWSTR, propid: i32, propindex: i32, proptype: i32) -> windows_core::Result<CERTTRANSBLOB>;
    fn GetCAPropertyInfo(&self, pwszauthority: &windows_core::PCWSTR, pcproperty: *mut i32, pctbpropinfo: *mut CERTTRANSBLOB) -> windows_core::Result<()>;
    fn Ping2(&self, pwszauthority: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ICertRequestD2 {}
impl ICertRequestD2_Vtbl {
    pub const fn new<Identity: ICertRequestD2_Impl, const OFFSET: isize>() -> ICertRequestD2_Vtbl {
        unsafe extern "system" fn Request2<Identity: ICertRequestD2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszauthority: windows_core::PCWSTR, dwflags: u32, pwszserialnumber: windows_core::PCWSTR, pdwrequestid: *mut u32, pdwdisposition: *mut u32, pwszattributes: windows_core::PCWSTR, pctbrequest: *const CERTTRANSBLOB, pctbfullresponse: *mut CERTTRANSBLOB, pctbencodedcert: *mut CERTTRANSBLOB, pctbdispositionmessage: *mut CERTTRANSBLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertRequestD2_Impl::Request2(this, core::mem::transmute(&pwszauthority), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pwszserialnumber), core::mem::transmute_copy(&pdwrequestid), core::mem::transmute_copy(&pdwdisposition), core::mem::transmute(&pwszattributes), core::mem::transmute_copy(&pctbrequest), core::mem::transmute_copy(&pctbfullresponse), core::mem::transmute_copy(&pctbencodedcert), core::mem::transmute_copy(&pctbdispositionmessage)).into()
        }
        unsafe extern "system" fn GetCAProperty<Identity: ICertRequestD2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszauthority: windows_core::PCWSTR, propid: i32, propindex: i32, proptype: i32, pctbpropertyvalue: *mut CERTTRANSBLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertRequestD2_Impl::GetCAProperty(this, core::mem::transmute(&pwszauthority), core::mem::transmute_copy(&propid), core::mem::transmute_copy(&propindex), core::mem::transmute_copy(&proptype)) {
                Ok(ok__) => {
                    pctbpropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAPropertyInfo<Identity: ICertRequestD2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszauthority: windows_core::PCWSTR, pcproperty: *mut i32, pctbpropinfo: *mut CERTTRANSBLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertRequestD2_Impl::GetCAPropertyInfo(this, core::mem::transmute(&pwszauthority), core::mem::transmute_copy(&pcproperty), core::mem::transmute_copy(&pctbpropinfo)).into()
        }
        unsafe extern "system" fn Ping2<Identity: ICertRequestD2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszauthority: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertRequestD2_Impl::Ping2(this, core::mem::transmute(&pwszauthority)).into()
        }
        Self {
            base__: ICertRequestD_Vtbl::new::<Identity, OFFSET>(),
            Request2: Request2::<Identity, OFFSET>,
            GetCAProperty: GetCAProperty::<Identity, OFFSET>,
            GetCAPropertyInfo: GetCAPropertyInfo::<Identity, OFFSET>,
            Ping2: Ping2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertRequestD2 as windows_core::Interface>::IID || iid == &<ICertRequestD as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertServerExit_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn SetContext(&self, context: i32) -> windows_core::Result<()>;
    fn GetRequestProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn GetRequestAttribute(&self, strattributename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetCertificateProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn GetCertificateExtension(&self, strextensionname: &windows_core::BSTR, r#type: i32) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn GetCertificateExtensionFlags(&self) -> windows_core::Result<i32>;
    fn EnumerateExtensionsSetup(&self, flags: i32) -> windows_core::Result<()>;
    fn EnumerateExtensions(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnumerateExtensionsClose(&self) -> windows_core::Result<()>;
    fn EnumerateAttributesSetup(&self, flags: i32) -> windows_core::Result<()>;
    fn EnumerateAttributes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnumerateAttributesClose(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertServerExit {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertServerExit_Vtbl {
    pub const fn new<Identity: ICertServerExit_Impl, const OFFSET: isize>() -> ICertServerExit_Vtbl {
        unsafe extern "system" fn SetContext<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerExit_Impl::SetContext(this, core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn GetRequestProperty<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertytype: i32, pvarpropertyvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerExit_Impl::GetRequestProperty(this, core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&propertytype)) {
                Ok(ok__) => {
                    pvarpropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestAttribute<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strattributename: core::mem::MaybeUninit<windows_core::BSTR>, pstrattributevalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerExit_Impl::GetRequestAttribute(this, core::mem::transmute(&strattributename)) {
                Ok(ok__) => {
                    pstrattributevalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateProperty<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertytype: i32, pvarpropertyvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerExit_Impl::GetCertificateProperty(this, core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&propertytype)) {
                Ok(ok__) => {
                    pvarpropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateExtension<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strextensionname: core::mem::MaybeUninit<windows_core::BSTR>, r#type: i32, pvarvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerExit_Impl::GetCertificateExtension(this, core::mem::transmute(&strextensionname), core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateExtensionFlags<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerExit_Impl::GetCertificateExtensionFlags(this) {
                Ok(ok__) => {
                    pextflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateExtensionsSetup<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerExit_Impl::EnumerateExtensionsSetup(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumerateExtensions<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrextensionname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerExit_Impl::EnumerateExtensions(this) {
                Ok(ok__) => {
                    pstrextensionname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateExtensionsClose<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerExit_Impl::EnumerateExtensionsClose(this).into()
        }
        unsafe extern "system" fn EnumerateAttributesSetup<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerExit_Impl::EnumerateAttributesSetup(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumerateAttributes<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrattributename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerExit_Impl::EnumerateAttributes(this) {
                Ok(ok__) => {
                    pstrattributename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAttributesClose<Identity: ICertServerExit_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerExit_Impl::EnumerateAttributesClose(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, OFFSET>,
            GetRequestProperty: GetRequestProperty::<Identity, OFFSET>,
            GetRequestAttribute: GetRequestAttribute::<Identity, OFFSET>,
            GetCertificateProperty: GetCertificateProperty::<Identity, OFFSET>,
            GetCertificateExtension: GetCertificateExtension::<Identity, OFFSET>,
            GetCertificateExtensionFlags: GetCertificateExtensionFlags::<Identity, OFFSET>,
            EnumerateExtensionsSetup: EnumerateExtensionsSetup::<Identity, OFFSET>,
            EnumerateExtensions: EnumerateExtensions::<Identity, OFFSET>,
            EnumerateExtensionsClose: EnumerateExtensionsClose::<Identity, OFFSET>,
            EnumerateAttributesSetup: EnumerateAttributesSetup::<Identity, OFFSET>,
            EnumerateAttributes: EnumerateAttributes::<Identity, OFFSET>,
            EnumerateAttributesClose: EnumerateAttributesClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertServerExit as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertServerPolicy_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn SetContext(&self, context: i32) -> windows_core::Result<()>;
    fn GetRequestProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn GetRequestAttribute(&self, strattributename: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetCertificateProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: CERT_PROPERTY_TYPE) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetCertificateProperty(&self, strpropertyname: &windows_core::BSTR, propertytype: i32, pvarpropertyvalue: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetCertificateExtension(&self, strextensionname: &windows_core::BSTR, r#type: CERT_PROPERTY_TYPE) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn GetCertificateExtensionFlags(&self) -> windows_core::Result<i32>;
    fn SetCertificateExtension(&self, strextensionname: &windows_core::BSTR, r#type: i32, extflags: i32, pvarvalue: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn EnumerateExtensionsSetup(&self, flags: i32) -> windows_core::Result<()>;
    fn EnumerateExtensions(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnumerateExtensionsClose(&self) -> windows_core::Result<()>;
    fn EnumerateAttributesSetup(&self, flags: i32) -> windows_core::Result<()>;
    fn EnumerateAttributes(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EnumerateAttributesClose(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertServerPolicy {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertServerPolicy_Vtbl {
    pub const fn new<Identity: ICertServerPolicy_Impl, const OFFSET: isize>() -> ICertServerPolicy_Vtbl {
        unsafe extern "system" fn SetContext<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerPolicy_Impl::SetContext(this, core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn GetRequestProperty<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertytype: i32, pvarpropertyvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerPolicy_Impl::GetRequestProperty(this, core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&propertytype)) {
                Ok(ok__) => {
                    pvarpropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRequestAttribute<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strattributename: core::mem::MaybeUninit<windows_core::BSTR>, pstrattributevalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerPolicy_Impl::GetRequestAttribute(this, core::mem::transmute(&strattributename)) {
                Ok(ok__) => {
                    pstrattributevalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateProperty<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertytype: CERT_PROPERTY_TYPE, pvarpropertyvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerPolicy_Impl::GetCertificateProperty(this, core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&propertytype)) {
                Ok(ok__) => {
                    pvarpropertyvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateProperty<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpropertyname: core::mem::MaybeUninit<windows_core::BSTR>, propertytype: i32, pvarpropertyvalue: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerPolicy_Impl::SetCertificateProperty(this, core::mem::transmute(&strpropertyname), core::mem::transmute_copy(&propertytype), core::mem::transmute_copy(&pvarpropertyvalue)).into()
        }
        unsafe extern "system" fn GetCertificateExtension<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strextensionname: core::mem::MaybeUninit<windows_core::BSTR>, r#type: CERT_PROPERTY_TYPE, pvarvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerPolicy_Impl::GetCertificateExtension(this, core::mem::transmute(&strextensionname), core::mem::transmute_copy(&r#type)) {
                Ok(ok__) => {
                    pvarvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateExtensionFlags<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerPolicy_Impl::GetCertificateExtensionFlags(this) {
                Ok(ok__) => {
                    pextflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateExtension<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strextensionname: core::mem::MaybeUninit<windows_core::BSTR>, r#type: i32, extflags: i32, pvarvalue: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerPolicy_Impl::SetCertificateExtension(this, core::mem::transmute(&strextensionname), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&extflags), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn EnumerateExtensionsSetup<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerPolicy_Impl::EnumerateExtensionsSetup(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumerateExtensions<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrextensionname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerPolicy_Impl::EnumerateExtensions(this) {
                Ok(ok__) => {
                    pstrextensionname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateExtensionsClose<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerPolicy_Impl::EnumerateExtensionsClose(this).into()
        }
        unsafe extern "system" fn EnumerateAttributesSetup<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerPolicy_Impl::EnumerateAttributesSetup(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn EnumerateAttributes<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrattributename: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertServerPolicy_Impl::EnumerateAttributes(this) {
                Ok(ok__) => {
                    pstrattributename.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerateAttributesClose<Identity: ICertServerPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertServerPolicy_Impl::EnumerateAttributesClose(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            SetContext: SetContext::<Identity, OFFSET>,
            GetRequestProperty: GetRequestProperty::<Identity, OFFSET>,
            GetRequestAttribute: GetRequestAttribute::<Identity, OFFSET>,
            GetCertificateProperty: GetCertificateProperty::<Identity, OFFSET>,
            SetCertificateProperty: SetCertificateProperty::<Identity, OFFSET>,
            GetCertificateExtension: GetCertificateExtension::<Identity, OFFSET>,
            GetCertificateExtensionFlags: GetCertificateExtensionFlags::<Identity, OFFSET>,
            SetCertificateExtension: SetCertificateExtension::<Identity, OFFSET>,
            EnumerateExtensionsSetup: EnumerateExtensionsSetup::<Identity, OFFSET>,
            EnumerateExtensions: EnumerateExtensions::<Identity, OFFSET>,
            EnumerateExtensionsClose: EnumerateExtensionsClose::<Identity, OFFSET>,
            EnumerateAttributesSetup: EnumerateAttributesSetup::<Identity, OFFSET>,
            EnumerateAttributes: EnumerateAttributes::<Identity, OFFSET>,
            EnumerateAttributesClose: EnumerateAttributesClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertServerPolicy as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertView_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn OpenConnection(&self, strconfig: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EnumCertViewColumn(&self, fresultcolumn: CVRC_COLUMN) -> windows_core::Result<IEnumCERTVIEWCOLUMN>;
    fn GetColumnCount(&self, fresultcolumn: CVRC_COLUMN, pccolumn: *mut i32) -> windows_core::Result<()>;
    fn GetColumnIndex(&self, fresultcolumn: CVRC_COLUMN, strcolumnname: &windows_core::BSTR, pcolumnindex: *mut i32) -> windows_core::Result<()>;
    fn SetResultColumnCount(&self, cresultcolumn: i32) -> windows_core::Result<()>;
    fn SetResultColumn(&self, columnindex: i32) -> windows_core::Result<()>;
    fn SetRestriction(&self, columnindex: CERT_VIEW_COLUMN_INDEX, seekoperator: CERT_VIEW_SEEK_OPERATOR_FLAGS, sortorder: i32, pvarvalue: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn OpenView(&self) -> windows_core::Result<IEnumCERTVIEWROW>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertView {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertView_Vtbl {
    pub const fn new<Identity: ICertView_Impl, const OFFSET: isize>() -> ICertView_Vtbl {
        unsafe extern "system" fn OpenConnection<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strconfig: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertView_Impl::OpenConnection(this, core::mem::transmute(&strconfig)).into()
        }
        unsafe extern "system" fn EnumCertViewColumn<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fresultcolumn: CVRC_COLUMN, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertView_Impl::EnumCertViewColumn(this, core::mem::transmute_copy(&fresultcolumn)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnCount<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fresultcolumn: CVRC_COLUMN, pccolumn: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertView_Impl::GetColumnCount(this, core::mem::transmute_copy(&fresultcolumn), core::mem::transmute_copy(&pccolumn)).into()
        }
        unsafe extern "system" fn GetColumnIndex<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fresultcolumn: CVRC_COLUMN, strcolumnname: core::mem::MaybeUninit<windows_core::BSTR>, pcolumnindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertView_Impl::GetColumnIndex(this, core::mem::transmute_copy(&fresultcolumn), core::mem::transmute(&strcolumnname), core::mem::transmute_copy(&pcolumnindex)).into()
        }
        unsafe extern "system" fn SetResultColumnCount<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cresultcolumn: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertView_Impl::SetResultColumnCount(this, core::mem::transmute_copy(&cresultcolumn)).into()
        }
        unsafe extern "system" fn SetResultColumn<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, columnindex: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertView_Impl::SetResultColumn(this, core::mem::transmute_copy(&columnindex)).into()
        }
        unsafe extern "system" fn SetRestriction<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, columnindex: CERT_VIEW_COLUMN_INDEX, seekoperator: CERT_VIEW_SEEK_OPERATOR_FLAGS, sortorder: i32, pvarvalue: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertView_Impl::SetRestriction(this, core::mem::transmute_copy(&columnindex), core::mem::transmute_copy(&seekoperator), core::mem::transmute_copy(&sortorder), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn OpenView<Identity: ICertView_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertView_Impl::OpenView(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OpenConnection: OpenConnection::<Identity, OFFSET>,
            EnumCertViewColumn: EnumCertViewColumn::<Identity, OFFSET>,
            GetColumnCount: GetColumnCount::<Identity, OFFSET>,
            GetColumnIndex: GetColumnIndex::<Identity, OFFSET>,
            SetResultColumnCount: SetResultColumnCount::<Identity, OFFSET>,
            SetResultColumn: SetResultColumn::<Identity, OFFSET>,
            SetRestriction: SetRestriction::<Identity, OFFSET>,
            OpenView: OpenView::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertView as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertView2_Impl: Sized + ICertView_Impl {
    fn SetTable(&self, table: CVRC_TABLE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertView2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertView2_Vtbl {
    pub const fn new<Identity: ICertView2_Impl, const OFFSET: isize>() -> ICertView2_Vtbl {
        unsafe extern "system" fn SetTable<Identity: ICertView2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, table: CVRC_TABLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertView2_Impl::SetTable(this, core::mem::transmute_copy(&table)).into()
        }
        Self { base__: ICertView_Vtbl::new::<Identity, OFFSET>(), SetTable: SetTable::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertView2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertView as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertificateAttestationChallenge_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, encoding: EncodingType, strpendingfullcmcresponsewithchallenge: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DecryptChallenge(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn RequestID(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertificateAttestationChallenge {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertificateAttestationChallenge_Vtbl {
    pub const fn new<Identity: ICertificateAttestationChallenge_Impl, const OFFSET: isize>() -> ICertificateAttestationChallenge_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertificateAttestationChallenge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strpendingfullcmcresponsewithchallenge: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateAttestationChallenge_Impl::Initialize(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strpendingfullcmcresponsewithchallenge)).into()
        }
        unsafe extern "system" fn DecryptChallenge<Identity: ICertificateAttestationChallenge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pstrenvelopedpkcs7reencryptedtoca: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificateAttestationChallenge_Impl::DecryptChallenge(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrenvelopedpkcs7reencryptedtoca.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestID<Identity: ICertificateAttestationChallenge_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrrequestid: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificateAttestationChallenge_Impl::RequestID(this) {
                Ok(ok__) => {
                    pstrrequestid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            DecryptChallenge: DecryptChallenge::<Identity, OFFSET>,
            RequestID: RequestID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificateAttestationChallenge as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertificateAttestationChallenge2_Impl: Sized + ICertificateAttestationChallenge_Impl {
    fn SetKeyContainerName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn put_KeyBlob(&self, encoding: EncodingType, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertificateAttestationChallenge2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertificateAttestationChallenge2_Vtbl {
    pub const fn new<Identity: ICertificateAttestationChallenge2_Impl, const OFFSET: isize>() -> ICertificateAttestationChallenge2_Vtbl {
        unsafe extern "system" fn SetKeyContainerName<Identity: ICertificateAttestationChallenge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateAttestationChallenge2_Impl::SetKeyContainerName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn put_KeyBlob<Identity: ICertificateAttestationChallenge2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificateAttestationChallenge2_Impl::put_KeyBlob(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&value)).into()
        }
        Self {
            base__: ICertificateAttestationChallenge_Vtbl::new::<Identity, OFFSET>(),
            SetKeyContainerName: SetKeyContainerName::<Identity, OFFSET>,
            put_KeyBlob: put_KeyBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificateAttestationChallenge2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<ICertificateAttestationChallenge as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertificatePolicies_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICertificatePolicy>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&ICertificatePolicy>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertificatePolicies {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertificatePolicies_Vtbl {
    pub const fn new<Identity: ICertificatePolicies_Impl, const OFFSET: isize>() -> ICertificatePolicies_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: ICertificatePolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificatePolicies_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ICertificatePolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificatePolicies_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ICertificatePolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificatePolicies_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ICertificatePolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificatePolicies_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: ICertificatePolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificatePolicies_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ICertificatePolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificatePolicies_Impl::Clear(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificatePolicies as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertificatePolicy_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, pvalue: Option<&IObjectId>) -> windows_core::Result<()>;
    fn ObjectId(&self) -> windows_core::Result<IObjectId>;
    fn PolicyQualifiers(&self) -> windows_core::Result<IPolicyQualifiers>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertificatePolicy {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertificatePolicy_Vtbl {
    pub const fn new<Identity: ICertificatePolicy_Impl, const OFFSET: isize>() -> ICertificatePolicy_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICertificatePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificatePolicy_Impl::Initialize(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn ObjectId<Identity: ICertificatePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificatePolicy_Impl::ObjectId(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyQualifiers<Identity: ICertificatePolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificatePolicy_Impl::PolicyQualifiers(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            ObjectId: ObjectId::<Identity, OFFSET>,
            PolicyQualifiers: PolicyQualifiers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificatePolicy as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertificationAuthorities_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICertificationAuthority>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&ICertificationAuthority>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn ComputeSiteCosts(&self) -> windows_core::Result<()>;
    fn get_ItemByName(&self, strname: &windows_core::BSTR) -> windows_core::Result<ICertificationAuthority>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertificationAuthorities {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertificationAuthorities_Vtbl {
    pub const fn new<Identity: ICertificationAuthorities_Impl, const OFFSET: isize>() -> ICertificationAuthorities_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: ICertificationAuthorities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificationAuthorities_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ICertificationAuthorities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificationAuthorities_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ICertificationAuthorities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificationAuthorities_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ICertificationAuthorities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificationAuthorities_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: ICertificationAuthorities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificationAuthorities_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ICertificationAuthorities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificationAuthorities_Impl::Clear(this).into()
        }
        unsafe extern "system" fn ComputeSiteCosts<Identity: ICertificationAuthorities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICertificationAuthorities_Impl::ComputeSiteCosts(this).into()
        }
        unsafe extern "system" fn get_ItemByName<Identity: ICertificationAuthorities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificationAuthorities_Impl::get_ItemByName(this, core::mem::transmute(&strname)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            ComputeSiteCosts: ComputeSiteCosts::<Identity, OFFSET>,
            get_ItemByName: get_ItemByName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificationAuthorities as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICertificationAuthority_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_Property(&self, property: EnrollmentCAProperty) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICertificationAuthority {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICertificationAuthority_Vtbl {
    pub const fn new<Identity: ICertificationAuthority_Impl, const OFFSET: isize>() -> ICertificationAuthority_Vtbl {
        unsafe extern "system" fn get_Property<Identity: ICertificationAuthority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: EnrollmentCAProperty, pvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICertificationAuthority_Impl::get_Property(this, core::mem::transmute_copy(&property)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_Property: get_Property::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificationAuthority as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICryptAttribute_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn InitializeFromObjectId(&self, pobjectid: Option<&IObjectId>) -> windows_core::Result<()>;
    fn InitializeFromValues(&self, pattributes: Option<&IX509Attributes>) -> windows_core::Result<()>;
    fn ObjectId(&self) -> windows_core::Result<IObjectId>;
    fn Values(&self) -> windows_core::Result<IX509Attributes>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICryptAttribute {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICryptAttribute_Vtbl {
    pub const fn new<Identity: ICryptAttribute_Impl, const OFFSET: isize>() -> ICryptAttribute_Vtbl {
        unsafe extern "system" fn InitializeFromObjectId<Identity: ICryptAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICryptAttribute_Impl::InitializeFromObjectId(this, windows_core::from_raw_borrowed(&pobjectid)).into()
        }
        unsafe extern "system" fn InitializeFromValues<Identity: ICryptAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICryptAttribute_Impl::InitializeFromValues(this, windows_core::from_raw_borrowed(&pattributes)).into()
        }
        unsafe extern "system" fn ObjectId<Identity: ICryptAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICryptAttribute_Impl::ObjectId(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Values<Identity: ICryptAttribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICryptAttribute_Impl::Values(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromObjectId: InitializeFromObjectId::<Identity, OFFSET>,
            InitializeFromValues: InitializeFromValues::<Identity, OFFSET>,
            ObjectId: ObjectId::<Identity, OFFSET>,
            Values: Values::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICryptAttribute as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICryptAttributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICryptAttribute>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&ICryptAttribute>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn get_IndexByObjectId(&self, pobjectid: Option<&IObjectId>) -> windows_core::Result<i32>;
    fn AddRange(&self, pvalue: Option<&ICryptAttributes>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICryptAttributes {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICryptAttributes_Vtbl {
    pub const fn new<Identity: ICryptAttributes_Impl, const OFFSET: isize>() -> ICryptAttributes_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: ICryptAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICryptAttributes_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ICryptAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICryptAttributes_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ICryptAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICryptAttributes_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ICryptAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICryptAttributes_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: ICryptAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICryptAttributes_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ICryptAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICryptAttributes_Impl::Clear(this).into()
        }
        unsafe extern "system" fn get_IndexByObjectId<Identity: ICryptAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectid: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICryptAttributes_Impl::get_IndexByObjectId(this, windows_core::from_raw_borrowed(&pobjectid)) {
                Ok(ok__) => {
                    pindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRange<Identity: ICryptAttributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICryptAttributes_Impl::AddRange(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            get_IndexByObjectId: get_IndexByObjectId::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICryptAttributes as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICspAlgorithm_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn GetAlgorithmOid(&self, length: i32, algflags: AlgorithmFlags) -> windows_core::Result<IObjectId>;
    fn DefaultLength(&self) -> windows_core::Result<i32>;
    fn IncrementLength(&self) -> windows_core::Result<i32>;
    fn LongName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Valid(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn MaxLength(&self) -> windows_core::Result<i32>;
    fn MinLength(&self) -> windows_core::Result<i32>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<AlgorithmType>;
    fn Operations(&self) -> windows_core::Result<AlgorithmOperationFlags>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICspAlgorithm {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICspAlgorithm_Vtbl {
    pub const fn new<Identity: ICspAlgorithm_Impl, const OFFSET: isize>() -> ICspAlgorithm_Vtbl {
        unsafe extern "system" fn GetAlgorithmOid<Identity: ICspAlgorithm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: i32, algflags: AlgorithmFlags, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithm_Impl::GetAlgorithmOid(this, core::mem::transmute_copy(&length), core::mem::transmute_copy(&algflags)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultLength<Identity: ICspAlgorithm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithm_Impl::DefaultLength(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IncrementLength<Identity: ICspAlgorithm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithm_Impl::IncrementLength(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LongName<Identity: ICspAlgorithm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithm_Impl::LongName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Valid<Identity: ICspAlgorithm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithm_Impl::Valid(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxLength<Identity: ICspAlgorithm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithm_Impl::MaxLength(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinLength<Identity: ICspAlgorithm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithm_Impl::MinLength(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ICspAlgorithm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithm_Impl::Name(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ICspAlgorithm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut AlgorithmType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithm_Impl::Type(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Operations<Identity: ICspAlgorithm_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut AlgorithmOperationFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithm_Impl::Operations(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetAlgorithmOid: GetAlgorithmOid::<Identity, OFFSET>,
            DefaultLength: DefaultLength::<Identity, OFFSET>,
            IncrementLength: IncrementLength::<Identity, OFFSET>,
            LongName: LongName::<Identity, OFFSET>,
            Valid: Valid::<Identity, OFFSET>,
            MaxLength: MaxLength::<Identity, OFFSET>,
            MinLength: MinLength::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Operations: Operations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICspAlgorithm as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICspAlgorithms_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICspAlgorithm>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&ICspAlgorithm>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn get_ItemByName(&self, strname: &windows_core::BSTR) -> windows_core::Result<ICspAlgorithm>;
    fn get_IndexByObjectId(&self, pobjectid: Option<&IObjectId>) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICspAlgorithms {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICspAlgorithms_Vtbl {
    pub const fn new<Identity: ICspAlgorithms_Impl, const OFFSET: isize>() -> ICspAlgorithms_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: ICspAlgorithms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithms_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ICspAlgorithms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithms_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ICspAlgorithms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithms_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ICspAlgorithms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspAlgorithms_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: ICspAlgorithms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspAlgorithms_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ICspAlgorithms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspAlgorithms_Impl::Clear(this).into()
        }
        unsafe extern "system" fn get_ItemByName<Identity: ICspAlgorithms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithms_Impl::get_ItemByName(this, core::mem::transmute(&strname)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_IndexByObjectId<Identity: ICspAlgorithms_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectid: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspAlgorithms_Impl::get_IndexByObjectId(this, windows_core::from_raw_borrowed(&pobjectid)) {
                Ok(ok__) => {
                    pindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            get_ItemByName: get_ItemByName::<Identity, OFFSET>,
            get_IndexByObjectId: get_IndexByObjectId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICspAlgorithms as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICspInformation_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn InitializeFromName(&self, strname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromType(&self, r#type: X509ProviderType, palgorithm: Option<&IObjectId>, machinecontext: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn CspAlgorithms(&self) -> windows_core::Result<ICspAlgorithms>;
    fn HasHardwareRandomNumberGenerator(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn IsHardwareDevice(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn IsRemovable(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn IsSoftwareDevice(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Valid(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn MaxKeyContainerNameLength(&self) -> windows_core::Result<i32>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<X509ProviderType>;
    fn Version(&self) -> windows_core::Result<i32>;
    fn KeySpec(&self) -> windows_core::Result<X509KeySpec>;
    fn IsSmartCard(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn GetDefaultSecurityDescriptor(&self, machinecontext: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<windows_core::BSTR>;
    fn LegacyCsp(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn GetCspStatusFromOperations(&self, palgorithm: Option<&IObjectId>, operations: AlgorithmOperationFlags) -> windows_core::Result<ICspStatus>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICspInformation {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICspInformation_Vtbl {
    pub const fn new<Identity: ICspInformation_Impl, const OFFSET: isize>() -> ICspInformation_Vtbl {
        unsafe extern "system" fn InitializeFromName<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspInformation_Impl::InitializeFromName(this, core::mem::transmute(&strname)).into()
        }
        unsafe extern "system" fn InitializeFromType<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: X509ProviderType, palgorithm: *mut core::ffi::c_void, machinecontext: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspInformation_Impl::InitializeFromType(this, core::mem::transmute_copy(&r#type), windows_core::from_raw_borrowed(&palgorithm), core::mem::transmute_copy(&machinecontext)).into()
        }
        unsafe extern "system" fn CspAlgorithms<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::CspAlgorithms(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasHardwareRandomNumberGenerator<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::HasHardwareRandomNumberGenerator(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHardwareDevice<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::IsHardwareDevice(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRemovable<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::IsRemovable(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSoftwareDevice<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::IsSoftwareDevice(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Valid<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::Valid(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxKeyContainerNameLength<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::MaxKeyContainerNameLength(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::Name(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509ProviderType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::Type(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::Version(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeySpec<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509KeySpec) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::KeySpec(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSmartCard<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::IsSmartCard(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultSecurityDescriptor<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, machinecontext: super::super::super::Foundation::VARIANT_BOOL, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::GetDefaultSecurityDescriptor(this, core::mem::transmute_copy(&machinecontext)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LegacyCsp<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::LegacyCsp(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCspStatusFromOperations<Identity: ICspInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palgorithm: *mut core::ffi::c_void, operations: AlgorithmOperationFlags, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformation_Impl::GetCspStatusFromOperations(this, windows_core::from_raw_borrowed(&palgorithm), core::mem::transmute_copy(&operations)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromName: InitializeFromName::<Identity, OFFSET>,
            InitializeFromType: InitializeFromType::<Identity, OFFSET>,
            CspAlgorithms: CspAlgorithms::<Identity, OFFSET>,
            HasHardwareRandomNumberGenerator: HasHardwareRandomNumberGenerator::<Identity, OFFSET>,
            IsHardwareDevice: IsHardwareDevice::<Identity, OFFSET>,
            IsRemovable: IsRemovable::<Identity, OFFSET>,
            IsSoftwareDevice: IsSoftwareDevice::<Identity, OFFSET>,
            Valid: Valid::<Identity, OFFSET>,
            MaxKeyContainerNameLength: MaxKeyContainerNameLength::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            Version: Version::<Identity, OFFSET>,
            KeySpec: KeySpec::<Identity, OFFSET>,
            IsSmartCard: IsSmartCard::<Identity, OFFSET>,
            GetDefaultSecurityDescriptor: GetDefaultSecurityDescriptor::<Identity, OFFSET>,
            LegacyCsp: LegacyCsp::<Identity, OFFSET>,
            GetCspStatusFromOperations: GetCspStatusFromOperations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICspInformation as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICspInformations_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICspInformation>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&ICspInformation>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddAvailableCsps(&self) -> windows_core::Result<()>;
    fn get_ItemByName(&self, strname: &windows_core::BSTR) -> windows_core::Result<ICspInformation>;
    fn GetCspStatusFromProviderName(&self, strprovidername: &windows_core::BSTR, legacykeyspec: X509KeySpec) -> windows_core::Result<ICspStatus>;
    fn GetCspStatusesFromOperations(&self, operations: AlgorithmOperationFlags, pcspinformation: Option<&ICspInformation>) -> windows_core::Result<ICspStatuses>;
    fn GetEncryptionCspAlgorithms(&self, pcspinformation: Option<&ICspInformation>) -> windows_core::Result<ICspAlgorithms>;
    fn GetHashAlgorithms(&self, pcspinformation: Option<&ICspInformation>) -> windows_core::Result<IObjectIds>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICspInformations {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICspInformations_Vtbl {
    pub const fn new<Identity: ICspInformations_Impl, const OFFSET: isize>() -> ICspInformations_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformations_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformations_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformations_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspInformations_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspInformations_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspInformations_Impl::Clear(this).into()
        }
        unsafe extern "system" fn AddAvailableCsps<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspInformations_Impl::AddAvailableCsps(this).into()
        }
        unsafe extern "system" fn get_ItemByName<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, ppcspinformation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformations_Impl::get_ItemByName(this, core::mem::transmute(&strname)) {
                Ok(ok__) => {
                    ppcspinformation.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCspStatusFromProviderName<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprovidername: core::mem::MaybeUninit<windows_core::BSTR>, legacykeyspec: X509KeySpec, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformations_Impl::GetCspStatusFromProviderName(this, core::mem::transmute(&strprovidername), core::mem::transmute_copy(&legacykeyspec)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCspStatusesFromOperations<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, operations: AlgorithmOperationFlags, pcspinformation: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformations_Impl::GetCspStatusesFromOperations(this, core::mem::transmute_copy(&operations), windows_core::from_raw_borrowed(&pcspinformation)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEncryptionCspAlgorithms<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcspinformation: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformations_Impl::GetEncryptionCspAlgorithms(this, windows_core::from_raw_borrowed(&pcspinformation)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashAlgorithms<Identity: ICspInformations_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcspinformation: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspInformations_Impl::GetHashAlgorithms(this, windows_core::from_raw_borrowed(&pcspinformation)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddAvailableCsps: AddAvailableCsps::<Identity, OFFSET>,
            get_ItemByName: get_ItemByName::<Identity, OFFSET>,
            GetCspStatusFromProviderName: GetCspStatusFromProviderName::<Identity, OFFSET>,
            GetCspStatusesFromOperations: GetCspStatusesFromOperations::<Identity, OFFSET>,
            GetEncryptionCspAlgorithms: GetEncryptionCspAlgorithms::<Identity, OFFSET>,
            GetHashAlgorithms: GetHashAlgorithms::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICspInformations as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICspStatus_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, pcsp: Option<&ICspInformation>, palgorithm: Option<&ICspAlgorithm>) -> windows_core::Result<()>;
    fn Ordinal(&self) -> windows_core::Result<i32>;
    fn SetOrdinal(&self, value: i32) -> windows_core::Result<()>;
    fn CspAlgorithm(&self) -> windows_core::Result<ICspAlgorithm>;
    fn CspInformation(&self) -> windows_core::Result<ICspInformation>;
    fn EnrollmentStatus(&self) -> windows_core::Result<IX509EnrollmentStatus>;
    fn DisplayName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICspStatus {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICspStatus_Vtbl {
    pub const fn new<Identity: ICspStatus_Impl, const OFFSET: isize>() -> ICspStatus_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ICspStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcsp: *mut core::ffi::c_void, palgorithm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspStatus_Impl::Initialize(this, windows_core::from_raw_borrowed(&pcsp), windows_core::from_raw_borrowed(&palgorithm)).into()
        }
        unsafe extern "system" fn Ordinal<Identity: ICspStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatus_Impl::Ordinal(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrdinal<Identity: ICspStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspStatus_Impl::SetOrdinal(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn CspAlgorithm<Identity: ICspStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatus_Impl::CspAlgorithm(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CspInformation<Identity: ICspStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatus_Impl::CspInformation(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnrollmentStatus<Identity: ICspStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatus_Impl::EnrollmentStatus(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ICspStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatus_Impl::DisplayName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Ordinal: Ordinal::<Identity, OFFSET>,
            SetOrdinal: SetOrdinal::<Identity, OFFSET>,
            CspAlgorithm: CspAlgorithm::<Identity, OFFSET>,
            CspInformation: CspInformation::<Identity, OFFSET>,
            EnrollmentStatus: EnrollmentStatus::<Identity, OFFSET>,
            DisplayName: DisplayName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICspStatus as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICspStatuses_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ICspStatus>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&ICspStatus>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn get_ItemByName(&self, strcspname: &windows_core::BSTR, stralgorithmname: &windows_core::BSTR) -> windows_core::Result<ICspStatus>;
    fn get_ItemByOrdinal(&self, ordinal: i32) -> windows_core::Result<ICspStatus>;
    fn get_ItemByOperations(&self, strcspname: &windows_core::BSTR, stralgorithmname: &windows_core::BSTR, operations: AlgorithmOperationFlags) -> windows_core::Result<ICspStatus>;
    fn get_ItemByProvider(&self, pcspstatus: Option<&ICspStatus>) -> windows_core::Result<ICspStatus>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ICspStatuses {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICspStatuses_Vtbl {
    pub const fn new<Identity: ICspStatuses_Impl, const OFFSET: isize>() -> ICspStatuses_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: ICspStatuses_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatuses_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ICspStatuses_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatuses_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ICspStatuses_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatuses_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ICspStatuses_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspStatuses_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: ICspStatuses_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspStatuses_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ICspStatuses_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ICspStatuses_Impl::Clear(this).into()
        }
        unsafe extern "system" fn get_ItemByName<Identity: ICspStatuses_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strcspname: core::mem::MaybeUninit<windows_core::BSTR>, stralgorithmname: core::mem::MaybeUninit<windows_core::BSTR>, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatuses_Impl::get_ItemByName(this, core::mem::transmute(&strcspname), core::mem::transmute(&stralgorithmname)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ItemByOrdinal<Identity: ICspStatuses_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ordinal: i32, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatuses_Impl::get_ItemByOrdinal(this, core::mem::transmute_copy(&ordinal)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ItemByOperations<Identity: ICspStatuses_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strcspname: core::mem::MaybeUninit<windows_core::BSTR>, stralgorithmname: core::mem::MaybeUninit<windows_core::BSTR>, operations: AlgorithmOperationFlags, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatuses_Impl::get_ItemByOperations(this, core::mem::transmute(&strcspname), core::mem::transmute(&stralgorithmname), core::mem::transmute_copy(&operations)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ItemByProvider<Identity: ICspStatuses_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcspstatus: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ICspStatuses_Impl::get_ItemByProvider(this, windows_core::from_raw_borrowed(&pcspstatus)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            get_ItemByName: get_ItemByName::<Identity, OFFSET>,
            get_ItemByOrdinal: get_ItemByOrdinal::<Identity, OFFSET>,
            get_ItemByOperations: get_ItemByOperations::<Identity, OFFSET>,
            get_ItemByProvider: get_ItemByProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICspStatuses as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait IEnroll_Impl: Sized + windows_core::IUnknownImpl {
    fn createFilePKCS10WStr(&self, dnname: &windows_core::PCWSTR, usage: &windows_core::PCWSTR, wszpkcs10filename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn acceptFilePKCS7WStr(&self, wszpkcs7filename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn createPKCS10WStr(&self, dnname: &windows_core::PCWSTR, usage: &windows_core::PCWSTR, ppkcs10blob: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn acceptPKCS7Blob(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn getCertContextFromPKCS7(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> *mut super::CERT_CONTEXT;
    fn getMyStore(&self) -> super::HCERTSTORE;
    fn getCAStore(&self) -> super::HCERTSTORE;
    fn getROOTHStore(&self) -> super::HCERTSTORE;
    fn enumProvidersWStr(&self, dwindex: i32, dwflags: i32, pbstrprovname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn enumContainersWStr(&self, dwindex: i32, pbstr: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn freeRequestInfoBlob(&self, pkcs7orpkcs10: &super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn MyStoreNameWStr(&self, szwname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetMyStoreNameWStr(&self, szwname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn MyStoreTypeWStr(&self, szwtype: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetMyStoreTypeWStr(&self, szwtype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn MyStoreFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()>;
    fn SetMyStoreFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn CAStoreNameWStr(&self, szwname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetCAStoreNameWStr(&self, szwname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CAStoreTypeWStr(&self, szwtype: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetCAStoreTypeWStr(&self, szwtype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CAStoreFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()>;
    fn SetCAStoreFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn RootStoreNameWStr(&self, szwname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetRootStoreNameWStr(&self, szwname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RootStoreTypeWStr(&self, szwtype: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetRootStoreTypeWStr(&self, szwtype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RootStoreFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()>;
    fn SetRootStoreFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn RequestStoreNameWStr(&self, szwname: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetRequestStoreNameWStr(&self, szwname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RequestStoreTypeWStr(&self, szwtype: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetRequestStoreTypeWStr(&self, szwtype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RequestStoreFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()>;
    fn SetRequestStoreFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn ContainerNameWStr(&self, szwcontainer: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetContainerNameWStr(&self, szwcontainer: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ProviderNameWStr(&self, szwprovider: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetProviderNameWStr(&self, szwprovider: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ProviderType(&self, pdwtype: *mut i32) -> windows_core::Result<()>;
    fn SetProviderType(&self, dwtype: i32) -> windows_core::Result<()>;
    fn KeySpec(&self, pdw: *mut i32) -> windows_core::Result<()>;
    fn SetKeySpec(&self, dw: i32) -> windows_core::Result<()>;
    fn ProviderFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()>;
    fn SetProviderFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn UseExistingKeySet(&self, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetUseExistingKeySet(&self, fuseexistingkeys: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GenKeyFlags(&self, pdwflags: *mut i32) -> windows_core::Result<()>;
    fn SetGenKeyFlags(&self, dwflags: i32) -> windows_core::Result<()>;
    fn DeleteRequestCert(&self, fdelete: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetDeleteRequestCert(&self, fdelete: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn WriteCertToUserDS(&self, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetWriteCertToUserDS(&self, fbool: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EnableT61DNEncoding(&self, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetEnableT61DNEncoding(&self, fbool: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn WriteCertToCSP(&self, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetWriteCertToCSP(&self, fbool: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SPCFileNameWStr(&self, szw: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetSPCFileNameWStr(&self, szw: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn PVKFileNameWStr(&self, szw: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetPVKFileNameWStr(&self, szw: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn HashAlgorithmWStr(&self, szw: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetHashAlgorithmWStr(&self, szw: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RenewalCertificate(&self, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> windows_core::Result<()>;
    fn SetRenewalCertificate(&self, pcertcontext: *const super::CERT_CONTEXT) -> windows_core::Result<()>;
    fn AddCertTypeToRequestWStr(&self, szw: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddNameValuePairToSignatureWStr(&self, name: &windows_core::PCWSTR, value: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddExtensionsToRequest(&self, pcertextensions: *mut super::CERT_EXTENSIONS) -> windows_core::Result<()>;
    fn AddAuthenticatedAttributesToPKCS7Request(&self, pattributes: *mut super::CRYPT_ATTRIBUTES) -> windows_core::Result<()>;
    fn CreatePKCS7RequestFromRequest(&self, prequest: *mut super::CRYPT_INTEGER_BLOB, psigningcertcontext: *const super::CERT_CONTEXT, ppkcs7blob: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnroll {}
impl IEnroll_Vtbl {
    pub const fn new<Identity: IEnroll_Impl, const OFFSET: isize>() -> IEnroll_Vtbl {
        unsafe extern "system" fn createFilePKCS10WStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dnname: windows_core::PCWSTR, usage: windows_core::PCWSTR, wszpkcs10filename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::createFilePKCS10WStr(this, core::mem::transmute(&dnname), core::mem::transmute(&usage), core::mem::transmute(&wszpkcs10filename)).into()
        }
        unsafe extern "system" fn acceptFilePKCS7WStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpkcs7filename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::acceptFilePKCS7WStr(this, core::mem::transmute(&wszpkcs7filename)).into()
        }
        unsafe extern "system" fn createPKCS10WStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dnname: windows_core::PCWSTR, usage: windows_core::PCWSTR, ppkcs10blob: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::createPKCS10WStr(this, core::mem::transmute(&dnname), core::mem::transmute(&usage), core::mem::transmute_copy(&ppkcs10blob)).into()
        }
        unsafe extern "system" fn acceptPKCS7Blob<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::acceptPKCS7Blob(this, core::mem::transmute_copy(&pblobpkcs7)).into()
        }
        unsafe extern "system" fn getCertContextFromPKCS7<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> *mut super::CERT_CONTEXT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::getCertContextFromPKCS7(this, core::mem::transmute_copy(&pblobpkcs7))
        }
        unsafe extern "system" fn getMyStore<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::HCERTSTORE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::getMyStore(this)
        }
        unsafe extern "system" fn getCAStore<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::HCERTSTORE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::getCAStore(this)
        }
        unsafe extern "system" fn getROOTHStore<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::HCERTSTORE {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::getROOTHStore(this)
        }
        unsafe extern "system" fn enumProvidersWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: i32, dwflags: i32, pbstrprovname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::enumProvidersWStr(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pbstrprovname)).into()
        }
        unsafe extern "system" fn enumContainersWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: i32, pbstr: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::enumContainersWStr(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pbstr)).into()
        }
        unsafe extern "system" fn freeRequestInfoBlob<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkcs7orpkcs10: super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::freeRequestInfoBlob(this, core::mem::transmute(&pkcs7orpkcs10)).into()
        }
        unsafe extern "system" fn MyStoreNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::MyStoreNameWStr(this, core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn SetMyStoreNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetMyStoreNameWStr(this, core::mem::transmute(&szwname)).into()
        }
        unsafe extern "system" fn MyStoreTypeWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwtype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::MyStoreTypeWStr(this, core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn SetMyStoreTypeWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwtype: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetMyStoreTypeWStr(this, core::mem::transmute(&szwtype)).into()
        }
        unsafe extern "system" fn MyStoreFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::MyStoreFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetMyStoreFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetMyStoreFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CAStoreNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::CAStoreNameWStr(this, core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn SetCAStoreNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetCAStoreNameWStr(this, core::mem::transmute(&szwname)).into()
        }
        unsafe extern "system" fn CAStoreTypeWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwtype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::CAStoreTypeWStr(this, core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn SetCAStoreTypeWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwtype: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetCAStoreTypeWStr(this, core::mem::transmute(&szwtype)).into()
        }
        unsafe extern "system" fn CAStoreFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::CAStoreFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetCAStoreFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetCAStoreFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RootStoreNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::RootStoreNameWStr(this, core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn SetRootStoreNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetRootStoreNameWStr(this, core::mem::transmute(&szwname)).into()
        }
        unsafe extern "system" fn RootStoreTypeWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwtype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::RootStoreTypeWStr(this, core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn SetRootStoreTypeWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwtype: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetRootStoreTypeWStr(this, core::mem::transmute(&szwtype)).into()
        }
        unsafe extern "system" fn RootStoreFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::RootStoreFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetRootStoreFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetRootStoreFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn RequestStoreNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::RequestStoreNameWStr(this, core::mem::transmute_copy(&szwname)).into()
        }
        unsafe extern "system" fn SetRequestStoreNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetRequestStoreNameWStr(this, core::mem::transmute(&szwname)).into()
        }
        unsafe extern "system" fn RequestStoreTypeWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwtype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::RequestStoreTypeWStr(this, core::mem::transmute_copy(&szwtype)).into()
        }
        unsafe extern "system" fn SetRequestStoreTypeWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwtype: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetRequestStoreTypeWStr(this, core::mem::transmute(&szwtype)).into()
        }
        unsafe extern "system" fn RequestStoreFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::RequestStoreFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetRequestStoreFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetRequestStoreFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ContainerNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwcontainer: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::ContainerNameWStr(this, core::mem::transmute_copy(&szwcontainer)).into()
        }
        unsafe extern "system" fn SetContainerNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwcontainer: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetContainerNameWStr(this, core::mem::transmute(&szwcontainer)).into()
        }
        unsafe extern "system" fn ProviderNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwprovider: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::ProviderNameWStr(this, core::mem::transmute_copy(&szwprovider)).into()
        }
        unsafe extern "system" fn SetProviderNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szwprovider: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetProviderNameWStr(this, core::mem::transmute(&szwprovider)).into()
        }
        unsafe extern "system" fn ProviderType<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtype: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::ProviderType(this, core::mem::transmute_copy(&pdwtype)).into()
        }
        unsafe extern "system" fn SetProviderType<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtype: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetProviderType(this, core::mem::transmute_copy(&dwtype)).into()
        }
        unsafe extern "system" fn KeySpec<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdw: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::KeySpec(this, core::mem::transmute_copy(&pdw)).into()
        }
        unsafe extern "system" fn SetKeySpec<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dw: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetKeySpec(this, core::mem::transmute_copy(&dw)).into()
        }
        unsafe extern "system" fn ProviderFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::ProviderFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetProviderFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetProviderFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn UseExistingKeySet<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuseexistingkeys: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::UseExistingKeySet(this, core::mem::transmute_copy(&fuseexistingkeys)).into()
        }
        unsafe extern "system" fn SetUseExistingKeySet<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fuseexistingkeys: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetUseExistingKeySet(this, core::mem::transmute_copy(&fuseexistingkeys)).into()
        }
        unsafe extern "system" fn GenKeyFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::GenKeyFlags(this, core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn SetGenKeyFlags<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetGenKeyFlags(this, core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DeleteRequestCert<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdelete: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::DeleteRequestCert(this, core::mem::transmute_copy(&fdelete)).into()
        }
        unsafe extern "system" fn SetDeleteRequestCert<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fdelete: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetDeleteRequestCert(this, core::mem::transmute_copy(&fdelete)).into()
        }
        unsafe extern "system" fn WriteCertToUserDS<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::WriteCertToUserDS(this, core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn SetWriteCertToUserDS<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetWriteCertToUserDS(this, core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn EnableT61DNEncoding<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::EnableT61DNEncoding(this, core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn SetEnableT61DNEncoding<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetEnableT61DNEncoding(this, core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn WriteCertToCSP<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::WriteCertToCSP(this, core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn SetWriteCertToCSP<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fbool: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetWriteCertToCSP(this, core::mem::transmute_copy(&fbool)).into()
        }
        unsafe extern "system" fn SPCFileNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szw: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SPCFileNameWStr(this, core::mem::transmute_copy(&szw)).into()
        }
        unsafe extern "system" fn SetSPCFileNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szw: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetSPCFileNameWStr(this, core::mem::transmute(&szw)).into()
        }
        unsafe extern "system" fn PVKFileNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szw: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::PVKFileNameWStr(this, core::mem::transmute_copy(&szw)).into()
        }
        unsafe extern "system" fn SetPVKFileNameWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szw: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetPVKFileNameWStr(this, core::mem::transmute(&szw)).into()
        }
        unsafe extern "system" fn HashAlgorithmWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szw: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::HashAlgorithmWStr(this, core::mem::transmute_copy(&szw)).into()
        }
        unsafe extern "system" fn SetHashAlgorithmWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szw: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetHashAlgorithmWStr(this, core::mem::transmute(&szw)).into()
        }
        unsafe extern "system" fn RenewalCertificate<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::RenewalCertificate(this, core::mem::transmute_copy(&ppcertcontext)).into()
        }
        unsafe extern "system" fn SetRenewalCertificate<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcertcontext: *const super::CERT_CONTEXT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::SetRenewalCertificate(this, core::mem::transmute_copy(&pcertcontext)).into()
        }
        unsafe extern "system" fn AddCertTypeToRequestWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szw: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::AddCertTypeToRequestWStr(this, core::mem::transmute(&szw)).into()
        }
        unsafe extern "system" fn AddNameValuePairToSignatureWStr<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, value: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::AddNameValuePairToSignatureWStr(this, core::mem::transmute(&name), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn AddExtensionsToRequest<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcertextensions: *mut super::CERT_EXTENSIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::AddExtensionsToRequest(this, core::mem::transmute_copy(&pcertextensions)).into()
        }
        unsafe extern "system" fn AddAuthenticatedAttributesToPKCS7Request<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattributes: *mut super::CRYPT_ATTRIBUTES) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::AddAuthenticatedAttributesToPKCS7Request(this, core::mem::transmute_copy(&pattributes)).into()
        }
        unsafe extern "system" fn CreatePKCS7RequestFromRequest<Identity: IEnroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *mut super::CRYPT_INTEGER_BLOB, psigningcertcontext: *const super::CERT_CONTEXT, ppkcs7blob: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll_Impl::CreatePKCS7RequestFromRequest(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&psigningcertcontext), core::mem::transmute_copy(&ppkcs7blob)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            createFilePKCS10WStr: createFilePKCS10WStr::<Identity, OFFSET>,
            acceptFilePKCS7WStr: acceptFilePKCS7WStr::<Identity, OFFSET>,
            createPKCS10WStr: createPKCS10WStr::<Identity, OFFSET>,
            acceptPKCS7Blob: acceptPKCS7Blob::<Identity, OFFSET>,
            getCertContextFromPKCS7: getCertContextFromPKCS7::<Identity, OFFSET>,
            getMyStore: getMyStore::<Identity, OFFSET>,
            getCAStore: getCAStore::<Identity, OFFSET>,
            getROOTHStore: getROOTHStore::<Identity, OFFSET>,
            enumProvidersWStr: enumProvidersWStr::<Identity, OFFSET>,
            enumContainersWStr: enumContainersWStr::<Identity, OFFSET>,
            freeRequestInfoBlob: freeRequestInfoBlob::<Identity, OFFSET>,
            MyStoreNameWStr: MyStoreNameWStr::<Identity, OFFSET>,
            SetMyStoreNameWStr: SetMyStoreNameWStr::<Identity, OFFSET>,
            MyStoreTypeWStr: MyStoreTypeWStr::<Identity, OFFSET>,
            SetMyStoreTypeWStr: SetMyStoreTypeWStr::<Identity, OFFSET>,
            MyStoreFlags: MyStoreFlags::<Identity, OFFSET>,
            SetMyStoreFlags: SetMyStoreFlags::<Identity, OFFSET>,
            CAStoreNameWStr: CAStoreNameWStr::<Identity, OFFSET>,
            SetCAStoreNameWStr: SetCAStoreNameWStr::<Identity, OFFSET>,
            CAStoreTypeWStr: CAStoreTypeWStr::<Identity, OFFSET>,
            SetCAStoreTypeWStr: SetCAStoreTypeWStr::<Identity, OFFSET>,
            CAStoreFlags: CAStoreFlags::<Identity, OFFSET>,
            SetCAStoreFlags: SetCAStoreFlags::<Identity, OFFSET>,
            RootStoreNameWStr: RootStoreNameWStr::<Identity, OFFSET>,
            SetRootStoreNameWStr: SetRootStoreNameWStr::<Identity, OFFSET>,
            RootStoreTypeWStr: RootStoreTypeWStr::<Identity, OFFSET>,
            SetRootStoreTypeWStr: SetRootStoreTypeWStr::<Identity, OFFSET>,
            RootStoreFlags: RootStoreFlags::<Identity, OFFSET>,
            SetRootStoreFlags: SetRootStoreFlags::<Identity, OFFSET>,
            RequestStoreNameWStr: RequestStoreNameWStr::<Identity, OFFSET>,
            SetRequestStoreNameWStr: SetRequestStoreNameWStr::<Identity, OFFSET>,
            RequestStoreTypeWStr: RequestStoreTypeWStr::<Identity, OFFSET>,
            SetRequestStoreTypeWStr: SetRequestStoreTypeWStr::<Identity, OFFSET>,
            RequestStoreFlags: RequestStoreFlags::<Identity, OFFSET>,
            SetRequestStoreFlags: SetRequestStoreFlags::<Identity, OFFSET>,
            ContainerNameWStr: ContainerNameWStr::<Identity, OFFSET>,
            SetContainerNameWStr: SetContainerNameWStr::<Identity, OFFSET>,
            ProviderNameWStr: ProviderNameWStr::<Identity, OFFSET>,
            SetProviderNameWStr: SetProviderNameWStr::<Identity, OFFSET>,
            ProviderType: ProviderType::<Identity, OFFSET>,
            SetProviderType: SetProviderType::<Identity, OFFSET>,
            KeySpec: KeySpec::<Identity, OFFSET>,
            SetKeySpec: SetKeySpec::<Identity, OFFSET>,
            ProviderFlags: ProviderFlags::<Identity, OFFSET>,
            SetProviderFlags: SetProviderFlags::<Identity, OFFSET>,
            UseExistingKeySet: UseExistingKeySet::<Identity, OFFSET>,
            SetUseExistingKeySet: SetUseExistingKeySet::<Identity, OFFSET>,
            GenKeyFlags: GenKeyFlags::<Identity, OFFSET>,
            SetGenKeyFlags: SetGenKeyFlags::<Identity, OFFSET>,
            DeleteRequestCert: DeleteRequestCert::<Identity, OFFSET>,
            SetDeleteRequestCert: SetDeleteRequestCert::<Identity, OFFSET>,
            WriteCertToUserDS: WriteCertToUserDS::<Identity, OFFSET>,
            SetWriteCertToUserDS: SetWriteCertToUserDS::<Identity, OFFSET>,
            EnableT61DNEncoding: EnableT61DNEncoding::<Identity, OFFSET>,
            SetEnableT61DNEncoding: SetEnableT61DNEncoding::<Identity, OFFSET>,
            WriteCertToCSP: WriteCertToCSP::<Identity, OFFSET>,
            SetWriteCertToCSP: SetWriteCertToCSP::<Identity, OFFSET>,
            SPCFileNameWStr: SPCFileNameWStr::<Identity, OFFSET>,
            SetSPCFileNameWStr: SetSPCFileNameWStr::<Identity, OFFSET>,
            PVKFileNameWStr: PVKFileNameWStr::<Identity, OFFSET>,
            SetPVKFileNameWStr: SetPVKFileNameWStr::<Identity, OFFSET>,
            HashAlgorithmWStr: HashAlgorithmWStr::<Identity, OFFSET>,
            SetHashAlgorithmWStr: SetHashAlgorithmWStr::<Identity, OFFSET>,
            RenewalCertificate: RenewalCertificate::<Identity, OFFSET>,
            SetRenewalCertificate: SetRenewalCertificate::<Identity, OFFSET>,
            AddCertTypeToRequestWStr: AddCertTypeToRequestWStr::<Identity, OFFSET>,
            AddNameValuePairToSignatureWStr: AddNameValuePairToSignatureWStr::<Identity, OFFSET>,
            AddExtensionsToRequest: AddExtensionsToRequest::<Identity, OFFSET>,
            AddAuthenticatedAttributesToPKCS7Request: AddAuthenticatedAttributesToPKCS7Request::<Identity, OFFSET>,
            CreatePKCS7RequestFromRequest: CreatePKCS7RequestFromRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnroll as windows_core::Interface>::IID
    }
}
pub trait IEnroll2_Impl: Sized + IEnroll_Impl {
    fn InstallPKCS7Blob(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn GetSupportedKeySpec(&self, pdwkeyspec: *mut i32) -> windows_core::Result<()>;
    fn GetKeyLen(&self, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL, pdwkeysize: *mut i32) -> windows_core::Result<()>;
    fn EnumAlgs(&self, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> windows_core::Result<()>;
    fn GetAlgNameWStr(&self, algid: i32, ppwsz: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn SetReuseHardwareKeyIfUnableToGenNew(&self, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn ReuseHardwareKeyIfUnableToGenNew(&self, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetHashAlgID(&self, hashalgid: i32) -> windows_core::Result<()>;
    fn HashAlgID(&self, hashalgid: *mut i32) -> windows_core::Result<()>;
    fn SetHStoreMy(&self, hstore: super::HCERTSTORE) -> windows_core::Result<()>;
    fn SetHStoreCA(&self, hstore: super::HCERTSTORE) -> windows_core::Result<()>;
    fn SetHStoreROOT(&self, hstore: super::HCERTSTORE) -> windows_core::Result<()>;
    fn SetHStoreRequest(&self, hstore: super::HCERTSTORE) -> windows_core::Result<()>;
    fn SetLimitExchangeKeyToEncipherment(&self, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn LimitExchangeKeyToEncipherment(&self, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetEnableSMIMECapabilities(&self, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn EnableSMIMECapabilities(&self, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnroll2 {}
impl IEnroll2_Vtbl {
    pub const fn new<Identity: IEnroll2_Impl, const OFFSET: isize>() -> IEnroll2_Vtbl {
        unsafe extern "system" fn InstallPKCS7Blob<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::InstallPKCS7Blob(this, core::mem::transmute_copy(&pblobpkcs7)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::Reset(this).into()
        }
        unsafe extern "system" fn GetSupportedKeySpec<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwkeyspec: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::GetSupportedKeySpec(this, core::mem::transmute_copy(&pdwkeyspec)).into()
        }
        unsafe extern "system" fn GetKeyLen<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmin: super::super::super::Foundation::BOOL, fexchange: super::super::super::Foundation::BOOL, pdwkeysize: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::GetKeyLen(this, core::mem::transmute_copy(&fmin), core::mem::transmute_copy(&fexchange), core::mem::transmute_copy(&pdwkeysize)).into()
        }
        unsafe extern "system" fn EnumAlgs<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: i32, algclass: i32, pdwalgid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::EnumAlgs(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&algclass), core::mem::transmute_copy(&pdwalgid)).into()
        }
        unsafe extern "system" fn GetAlgNameWStr<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, algid: i32, ppwsz: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::GetAlgNameWStr(this, core::mem::transmute_copy(&algid), core::mem::transmute_copy(&ppwsz)).into()
        }
        unsafe extern "system" fn SetReuseHardwareKeyIfUnableToGenNew<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, freusehardwarekeyifunabletogennew: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::SetReuseHardwareKeyIfUnableToGenNew(this, core::mem::transmute_copy(&freusehardwarekeyifunabletogennew)).into()
        }
        unsafe extern "system" fn ReuseHardwareKeyIfUnableToGenNew<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, freusehardwarekeyifunabletogennew: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::ReuseHardwareKeyIfUnableToGenNew(this, core::mem::transmute_copy(&freusehardwarekeyifunabletogennew)).into()
        }
        unsafe extern "system" fn SetHashAlgID<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashalgid: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::SetHashAlgID(this, core::mem::transmute_copy(&hashalgid)).into()
        }
        unsafe extern "system" fn HashAlgID<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hashalgid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::HashAlgID(this, core::mem::transmute_copy(&hashalgid)).into()
        }
        unsafe extern "system" fn SetHStoreMy<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hstore: super::HCERTSTORE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::SetHStoreMy(this, core::mem::transmute_copy(&hstore)).into()
        }
        unsafe extern "system" fn SetHStoreCA<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hstore: super::HCERTSTORE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::SetHStoreCA(this, core::mem::transmute_copy(&hstore)).into()
        }
        unsafe extern "system" fn SetHStoreROOT<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hstore: super::HCERTSTORE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::SetHStoreROOT(this, core::mem::transmute_copy(&hstore)).into()
        }
        unsafe extern "system" fn SetHStoreRequest<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hstore: super::HCERTSTORE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::SetHStoreRequest(this, core::mem::transmute_copy(&hstore)).into()
        }
        unsafe extern "system" fn SetLimitExchangeKeyToEncipherment<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flimitexchangekeytoencipherment: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::SetLimitExchangeKeyToEncipherment(this, core::mem::transmute_copy(&flimitexchangekeytoencipherment)).into()
        }
        unsafe extern "system" fn LimitExchangeKeyToEncipherment<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flimitexchangekeytoencipherment: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::LimitExchangeKeyToEncipherment(this, core::mem::transmute_copy(&flimitexchangekeytoencipherment)).into()
        }
        unsafe extern "system" fn SetEnableSMIMECapabilities<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablesmimecapabilities: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::SetEnableSMIMECapabilities(this, core::mem::transmute_copy(&fenablesmimecapabilities)).into()
        }
        unsafe extern "system" fn EnableSMIMECapabilities<Identity: IEnroll2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenablesmimecapabilities: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll2_Impl::EnableSMIMECapabilities(this, core::mem::transmute_copy(&fenablesmimecapabilities)).into()
        }
        Self {
            base__: IEnroll_Vtbl::new::<Identity, OFFSET>(),
            InstallPKCS7Blob: InstallPKCS7Blob::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            GetSupportedKeySpec: GetSupportedKeySpec::<Identity, OFFSET>,
            GetKeyLen: GetKeyLen::<Identity, OFFSET>,
            EnumAlgs: EnumAlgs::<Identity, OFFSET>,
            GetAlgNameWStr: GetAlgNameWStr::<Identity, OFFSET>,
            SetReuseHardwareKeyIfUnableToGenNew: SetReuseHardwareKeyIfUnableToGenNew::<Identity, OFFSET>,
            ReuseHardwareKeyIfUnableToGenNew: ReuseHardwareKeyIfUnableToGenNew::<Identity, OFFSET>,
            SetHashAlgID: SetHashAlgID::<Identity, OFFSET>,
            HashAlgID: HashAlgID::<Identity, OFFSET>,
            SetHStoreMy: SetHStoreMy::<Identity, OFFSET>,
            SetHStoreCA: SetHStoreCA::<Identity, OFFSET>,
            SetHStoreROOT: SetHStoreROOT::<Identity, OFFSET>,
            SetHStoreRequest: SetHStoreRequest::<Identity, OFFSET>,
            SetLimitExchangeKeyToEncipherment: SetLimitExchangeKeyToEncipherment::<Identity, OFFSET>,
            LimitExchangeKeyToEncipherment: LimitExchangeKeyToEncipherment::<Identity, OFFSET>,
            SetEnableSMIMECapabilities: SetEnableSMIMECapabilities::<Identity, OFFSET>,
            EnableSMIMECapabilities: EnableSMIMECapabilities::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnroll2 as windows_core::Interface>::IID || iid == &<IEnroll as windows_core::Interface>::IID
    }
}
pub trait IEnroll4_Impl: Sized + IEnroll2_Impl {
    fn SetThumbPrintWStr(&self, thumbprintblob: &super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn ThumbPrintWStr(&self, thumbprintblob: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn SetPrivateKeyArchiveCertificate(&self, pprivatekeyarchivecert: *const super::CERT_CONTEXT) -> windows_core::Result<()>;
    fn GetPrivateKeyArchiveCertificate(&self) -> *mut super::CERT_CONTEXT;
    fn binaryBlobToString(&self, flags: i32, pblobbinary: *mut super::CRYPT_INTEGER_BLOB, ppwszstring: *mut windows_core::PWSTR) -> windows_core::Result<()>;
    fn stringToBinaryBlob(&self, flags: i32, pwszstring: &windows_core::PCWSTR, pblobbinary: *mut super::CRYPT_INTEGER_BLOB, pdwskip: *mut i32, pdwflags: *mut i32) -> windows_core::Result<()>;
    fn addExtensionToRequestWStr(&self, flags: i32, pwszname: &windows_core::PCWSTR, pblobvalue: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn addAttributeToRequestWStr(&self, flags: i32, pwszname: &windows_core::PCWSTR, pblobvalue: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn addNameValuePairToRequestWStr(&self, flags: i32, pwszname: &windows_core::PCWSTR, pwszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn resetExtensions(&self) -> windows_core::Result<()>;
    fn resetAttributes(&self) -> windows_core::Result<()>;
    fn createRequestWStr(&self, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: &windows_core::PCWSTR, pwszusage: &windows_core::PCWSTR, pblobrequest: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn createFileRequestWStr(&self, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: &windows_core::PCWSTR, pwszusage: &windows_core::PCWSTR, pwszrequestfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn acceptResponseBlob(&self, pblobresponse: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn acceptFileResponseWStr(&self, pwszresponsefilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn getCertContextFromResponseBlob(&self, pblobresponse: *mut super::CRYPT_INTEGER_BLOB, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> windows_core::Result<()>;
    fn getCertContextFromFileResponseWStr(&self, pwszresponsefilename: &windows_core::PCWSTR, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> windows_core::Result<()>;
    fn createPFXWStr(&self, pwszpassword: &windows_core::PCWSTR, pblobpfx: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn createFilePFXWStr(&self, pwszpassword: &windows_core::PCWSTR, pwszpfxfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn setPendingRequestInfoWStr(&self, lrequestid: i32, pwszcadns: &windows_core::PCWSTR, pwszcaname: &windows_core::PCWSTR, pwszfriendlyname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn enumPendingRequestWStr(&self, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY, ppproperty: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn removePendingRequestWStr(&self, thumbprintblob: &super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn GetKeyLenEx(&self, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC, pdwkeysize: *mut i32) -> windows_core::Result<()>;
    fn InstallPKCS7BlobEx(&self, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB, plcertinstalled: *mut i32) -> windows_core::Result<()>;
    fn AddCertTypeToRequestWStrEx(&self, ltype: ADDED_CERT_TYPE, pwszoidorname: &windows_core::PCWSTR, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> windows_core::Result<()>;
    fn getProviderTypeWStr(&self, pwszprovname: &windows_core::PCWSTR, plprovtype: *mut i32) -> windows_core::Result<()>;
    fn addBlobPropertyToCertificateWStr(&self, lpropertyid: i32, lreserved: i32, pblobproperty: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::Result<()>;
    fn SetSignerCertificate(&self, psignercert: *const super::CERT_CONTEXT) -> windows_core::Result<()>;
    fn SetClientId(&self, lclientid: i32) -> windows_core::Result<()>;
    fn ClientId(&self, plclientid: *mut i32) -> windows_core::Result<()>;
    fn SetIncludeSubjectKeyID(&self, finclude: super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn IncludeSubjectKeyID(&self, pfinclude: *mut super::super::super::Foundation::BOOL) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IEnroll4 {}
impl IEnroll4_Vtbl {
    pub const fn new<Identity: IEnroll4_Impl, const OFFSET: isize>() -> IEnroll4_Vtbl {
        unsafe extern "system" fn SetThumbPrintWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, thumbprintblob: super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::SetThumbPrintWStr(this, core::mem::transmute(&thumbprintblob)).into()
        }
        unsafe extern "system" fn ThumbPrintWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, thumbprintblob: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::ThumbPrintWStr(this, core::mem::transmute_copy(&thumbprintblob)).into()
        }
        unsafe extern "system" fn SetPrivateKeyArchiveCertificate<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprivatekeyarchivecert: *const super::CERT_CONTEXT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::SetPrivateKeyArchiveCertificate(this, core::mem::transmute_copy(&pprivatekeyarchivecert)).into()
        }
        unsafe extern "system" fn GetPrivateKeyArchiveCertificate<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut super::CERT_CONTEXT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::GetPrivateKeyArchiveCertificate(this)
        }
        unsafe extern "system" fn binaryBlobToString<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pblobbinary: *mut super::CRYPT_INTEGER_BLOB, ppwszstring: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::binaryBlobToString(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pblobbinary), core::mem::transmute_copy(&ppwszstring)).into()
        }
        unsafe extern "system" fn stringToBinaryBlob<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pwszstring: windows_core::PCWSTR, pblobbinary: *mut super::CRYPT_INTEGER_BLOB, pdwskip: *mut i32, pdwflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::stringToBinaryBlob(this, core::mem::transmute_copy(&flags), core::mem::transmute(&pwszstring), core::mem::transmute_copy(&pblobbinary), core::mem::transmute_copy(&pdwskip), core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn addExtensionToRequestWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pwszname: windows_core::PCWSTR, pblobvalue: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::addExtensionToRequestWStr(this, core::mem::transmute_copy(&flags), core::mem::transmute(&pwszname), core::mem::transmute_copy(&pblobvalue)).into()
        }
        unsafe extern "system" fn addAttributeToRequestWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pwszname: windows_core::PCWSTR, pblobvalue: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::addAttributeToRequestWStr(this, core::mem::transmute_copy(&flags), core::mem::transmute(&pwszname), core::mem::transmute_copy(&pblobvalue)).into()
        }
        unsafe extern "system" fn addNameValuePairToRequestWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, pwszname: windows_core::PCWSTR, pwszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::addNameValuePairToRequestWStr(this, core::mem::transmute_copy(&flags), core::mem::transmute(&pwszname), core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn resetExtensions<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::resetExtensions(this).into()
        }
        unsafe extern "system" fn resetAttributes<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::resetAttributes(this).into()
        }
        unsafe extern "system" fn createRequestWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: windows_core::PCWSTR, pwszusage: windows_core::PCWSTR, pblobrequest: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::createRequestWStr(this, core::mem::transmute_copy(&flags), core::mem::transmute(&pwszdnname), core::mem::transmute(&pwszusage), core::mem::transmute_copy(&pblobrequest)).into()
        }
        unsafe extern "system" fn createFileRequestWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: CERT_CREATE_REQUEST_FLAGS, pwszdnname: windows_core::PCWSTR, pwszusage: windows_core::PCWSTR, pwszrequestfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::createFileRequestWStr(this, core::mem::transmute_copy(&flags), core::mem::transmute(&pwszdnname), core::mem::transmute(&pwszusage), core::mem::transmute(&pwszrequestfilename)).into()
        }
        unsafe extern "system" fn acceptResponseBlob<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblobresponse: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::acceptResponseBlob(this, core::mem::transmute_copy(&pblobresponse)).into()
        }
        unsafe extern "system" fn acceptFileResponseWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszresponsefilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::acceptFileResponseWStr(this, core::mem::transmute(&pwszresponsefilename)).into()
        }
        unsafe extern "system" fn getCertContextFromResponseBlob<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblobresponse: *mut super::CRYPT_INTEGER_BLOB, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::getCertContextFromResponseBlob(this, core::mem::transmute_copy(&pblobresponse), core::mem::transmute_copy(&ppcertcontext)).into()
        }
        unsafe extern "system" fn getCertContextFromFileResponseWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszresponsefilename: windows_core::PCWSTR, ppcertcontext: *mut *mut super::CERT_CONTEXT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::getCertContextFromFileResponseWStr(this, core::mem::transmute(&pwszresponsefilename), core::mem::transmute_copy(&ppcertcontext)).into()
        }
        unsafe extern "system" fn createPFXWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpassword: windows_core::PCWSTR, pblobpfx: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::createPFXWStr(this, core::mem::transmute(&pwszpassword), core::mem::transmute_copy(&pblobpfx)).into()
        }
        unsafe extern "system" fn createFilePFXWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszpassword: windows_core::PCWSTR, pwszpfxfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::createFilePFXWStr(this, core::mem::transmute(&pwszpassword), core::mem::transmute(&pwszpfxfilename)).into()
        }
        unsafe extern "system" fn setPendingRequestInfoWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lrequestid: i32, pwszcadns: windows_core::PCWSTR, pwszcaname: windows_core::PCWSTR, pwszfriendlyname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::setPendingRequestInfoWStr(this, core::mem::transmute_copy(&lrequestid), core::mem::transmute(&pwszcadns), core::mem::transmute(&pwszcaname), core::mem::transmute(&pwszfriendlyname)).into()
        }
        unsafe extern "system" fn enumPendingRequestWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lindex: i32, ldesiredproperty: PENDING_REQUEST_DESIRED_PROPERTY, ppproperty: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::enumPendingRequestWStr(this, core::mem::transmute_copy(&lindex), core::mem::transmute_copy(&ldesiredproperty), core::mem::transmute_copy(&ppproperty)).into()
        }
        unsafe extern "system" fn removePendingRequestWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, thumbprintblob: super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::removePendingRequestWStr(this, core::mem::transmute(&thumbprintblob)).into()
        }
        unsafe extern "system" fn GetKeyLenEx<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lsizespec: XEKL_KEYSIZE, lkeyspec: XEKL_KEYSPEC, pdwkeysize: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::GetKeyLenEx(this, core::mem::transmute_copy(&lsizespec), core::mem::transmute_copy(&lkeyspec), core::mem::transmute_copy(&pdwkeysize)).into()
        }
        unsafe extern "system" fn InstallPKCS7BlobEx<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pblobpkcs7: *mut super::CRYPT_INTEGER_BLOB, plcertinstalled: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::InstallPKCS7BlobEx(this, core::mem::transmute_copy(&pblobpkcs7), core::mem::transmute_copy(&plcertinstalled)).into()
        }
        unsafe extern "system" fn AddCertTypeToRequestWStrEx<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ltype: ADDED_CERT_TYPE, pwszoidorname: windows_core::PCWSTR, lmajorversion: i32, fminorversion: super::super::super::Foundation::BOOL, lminorversion: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::AddCertTypeToRequestWStrEx(this, core::mem::transmute_copy(&ltype), core::mem::transmute(&pwszoidorname), core::mem::transmute_copy(&lmajorversion), core::mem::transmute_copy(&fminorversion), core::mem::transmute_copy(&lminorversion)).into()
        }
        unsafe extern "system" fn getProviderTypeWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszprovname: windows_core::PCWSTR, plprovtype: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::getProviderTypeWStr(this, core::mem::transmute(&pwszprovname), core::mem::transmute_copy(&plprovtype)).into()
        }
        unsafe extern "system" fn addBlobPropertyToCertificateWStr<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpropertyid: i32, lreserved: i32, pblobproperty: *mut super::CRYPT_INTEGER_BLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::addBlobPropertyToCertificateWStr(this, core::mem::transmute_copy(&lpropertyid), core::mem::transmute_copy(&lreserved), core::mem::transmute_copy(&pblobproperty)).into()
        }
        unsafe extern "system" fn SetSignerCertificate<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psignercert: *const super::CERT_CONTEXT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::SetSignerCertificate(this, core::mem::transmute_copy(&psignercert)).into()
        }
        unsafe extern "system" fn SetClientId<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lclientid: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::SetClientId(this, core::mem::transmute_copy(&lclientid)).into()
        }
        unsafe extern "system" fn ClientId<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plclientid: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::ClientId(this, core::mem::transmute_copy(&plclientid)).into()
        }
        unsafe extern "system" fn SetIncludeSubjectKeyID<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finclude: super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::SetIncludeSubjectKeyID(this, core::mem::transmute_copy(&finclude)).into()
        }
        unsafe extern "system" fn IncludeSubjectKeyID<Identity: IEnroll4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfinclude: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnroll4_Impl::IncludeSubjectKeyID(this, core::mem::transmute_copy(&pfinclude)).into()
        }
        Self {
            base__: IEnroll2_Vtbl::new::<Identity, OFFSET>(),
            SetThumbPrintWStr: SetThumbPrintWStr::<Identity, OFFSET>,
            ThumbPrintWStr: ThumbPrintWStr::<Identity, OFFSET>,
            SetPrivateKeyArchiveCertificate: SetPrivateKeyArchiveCertificate::<Identity, OFFSET>,
            GetPrivateKeyArchiveCertificate: GetPrivateKeyArchiveCertificate::<Identity, OFFSET>,
            binaryBlobToString: binaryBlobToString::<Identity, OFFSET>,
            stringToBinaryBlob: stringToBinaryBlob::<Identity, OFFSET>,
            addExtensionToRequestWStr: addExtensionToRequestWStr::<Identity, OFFSET>,
            addAttributeToRequestWStr: addAttributeToRequestWStr::<Identity, OFFSET>,
            addNameValuePairToRequestWStr: addNameValuePairToRequestWStr::<Identity, OFFSET>,
            resetExtensions: resetExtensions::<Identity, OFFSET>,
            resetAttributes: resetAttributes::<Identity, OFFSET>,
            createRequestWStr: createRequestWStr::<Identity, OFFSET>,
            createFileRequestWStr: createFileRequestWStr::<Identity, OFFSET>,
            acceptResponseBlob: acceptResponseBlob::<Identity, OFFSET>,
            acceptFileResponseWStr: acceptFileResponseWStr::<Identity, OFFSET>,
            getCertContextFromResponseBlob: getCertContextFromResponseBlob::<Identity, OFFSET>,
            getCertContextFromFileResponseWStr: getCertContextFromFileResponseWStr::<Identity, OFFSET>,
            createPFXWStr: createPFXWStr::<Identity, OFFSET>,
            createFilePFXWStr: createFilePFXWStr::<Identity, OFFSET>,
            setPendingRequestInfoWStr: setPendingRequestInfoWStr::<Identity, OFFSET>,
            enumPendingRequestWStr: enumPendingRequestWStr::<Identity, OFFSET>,
            removePendingRequestWStr: removePendingRequestWStr::<Identity, OFFSET>,
            GetKeyLenEx: GetKeyLenEx::<Identity, OFFSET>,
            InstallPKCS7BlobEx: InstallPKCS7BlobEx::<Identity, OFFSET>,
            AddCertTypeToRequestWStrEx: AddCertTypeToRequestWStrEx::<Identity, OFFSET>,
            getProviderTypeWStr: getProviderTypeWStr::<Identity, OFFSET>,
            addBlobPropertyToCertificateWStr: addBlobPropertyToCertificateWStr::<Identity, OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Identity, OFFSET>,
            SetClientId: SetClientId::<Identity, OFFSET>,
            ClientId: ClientId::<Identity, OFFSET>,
            SetIncludeSubjectKeyID: SetIncludeSubjectKeyID::<Identity, OFFSET>,
            IncludeSubjectKeyID: IncludeSubjectKeyID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnroll4 as windows_core::Interface>::IID || iid == &<IEnroll as windows_core::Interface>::IID || iid == &<IEnroll2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumCERTVIEWATTRIBUTE_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Next(&self, pindex: *mut i32) -> windows_core::Result<()>;
    fn GetName(&self, pstrout: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetValue(&self, pstrout: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn Skip(&self, celt: i32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWATTRIBUTE>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEnumCERTVIEWATTRIBUTE {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumCERTVIEWATTRIBUTE_Vtbl {
    pub const fn new<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>() -> IEnumCERTVIEWATTRIBUTE_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWATTRIBUTE_Impl::Next(this, core::mem::transmute_copy(&pindex)).into()
        }
        unsafe extern "system" fn GetName<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrout: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWATTRIBUTE_Impl::GetName(this, core::mem::transmute_copy(&pstrout)).into()
        }
        unsafe extern "system" fn GetValue<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrout: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWATTRIBUTE_Impl::GetValue(this, core::mem::transmute_copy(&pstrout)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWATTRIBUTE_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWATTRIBUTE_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumCERTVIEWATTRIBUTE_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumCERTVIEWATTRIBUTE_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCERTVIEWATTRIBUTE as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumCERTVIEWCOLUMN_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Next(&self, pindex: *mut i32) -> windows_core::Result<()>;
    fn GetName(&self, pstrout: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDisplayName(&self, pstrout: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetType(&self, ptype: *mut i32) -> windows_core::Result<()>;
    fn IsIndexed(&self, pindexed: *mut i32) -> windows_core::Result<()>;
    fn GetMaxLength(&self, pmaxlength: *mut i32) -> windows_core::Result<()>;
    fn GetValue(&self, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Skip(&self, celt: i32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWCOLUMN>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEnumCERTVIEWCOLUMN {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumCERTVIEWCOLUMN_Vtbl {
    pub const fn new<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>() -> IEnumCERTVIEWCOLUMN_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWCOLUMN_Impl::Next(this, core::mem::transmute_copy(&pindex)).into()
        }
        unsafe extern "system" fn GetName<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrout: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWCOLUMN_Impl::GetName(this, core::mem::transmute_copy(&pstrout)).into()
        }
        unsafe extern "system" fn GetDisplayName<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrout: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWCOLUMN_Impl::GetDisplayName(this, core::mem::transmute_copy(&pstrout)).into()
        }
        unsafe extern "system" fn GetType<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWCOLUMN_Impl::GetType(this, core::mem::transmute_copy(&ptype)).into()
        }
        unsafe extern "system" fn IsIndexed<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindexed: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWCOLUMN_Impl::IsIndexed(this, core::mem::transmute_copy(&pindexed)).into()
        }
        unsafe extern "system" fn GetMaxLength<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmaxlength: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWCOLUMN_Impl::GetMaxLength(this, core::mem::transmute_copy(&pmaxlength)).into()
        }
        unsafe extern "system" fn GetValue<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWCOLUMN_Impl::GetValue(this, core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWCOLUMN_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWCOLUMN_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumCERTVIEWCOLUMN_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumCERTVIEWCOLUMN_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            IsIndexed: IsIndexed::<Identity, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCERTVIEWCOLUMN as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumCERTVIEWEXTENSION_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Next(&self, pindex: *mut i32) -> windows_core::Result<()>;
    fn GetName(&self, pstrout: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetFlags(&self, pflags: *mut i32) -> windows_core::Result<()>;
    fn GetValue(&self, r#type: CERT_PROPERTY_TYPE, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Skip(&self, celt: i32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWEXTENSION>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEnumCERTVIEWEXTENSION {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumCERTVIEWEXTENSION_Vtbl {
    pub const fn new<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>() -> IEnumCERTVIEWEXTENSION_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWEXTENSION_Impl::Next(this, core::mem::transmute_copy(&pindex)).into()
        }
        unsafe extern "system" fn GetName<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrout: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWEXTENSION_Impl::GetName(this, core::mem::transmute_copy(&pstrout)).into()
        }
        unsafe extern "system" fn GetFlags<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWEXTENSION_Impl::GetFlags(this, core::mem::transmute_copy(&pflags)).into()
        }
        unsafe extern "system" fn GetValue<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: CERT_PROPERTY_TYPE, flags: ENUM_CERT_COLUMN_VALUE_FLAGS, pvarvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWEXTENSION_Impl::GetValue(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&pvarvalue)).into()
        }
        unsafe extern "system" fn Skip<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWEXTENSION_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWEXTENSION_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumCERTVIEWEXTENSION_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumCERTVIEWEXTENSION_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            GetName: GetName::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCERTVIEWEXTENSION as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumCERTVIEWROW_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Next(&self, pindex: *mut i32) -> windows_core::Result<()>;
    fn EnumCertViewColumn(&self) -> windows_core::Result<IEnumCERTVIEWCOLUMN>;
    fn EnumCertViewAttribute(&self, flags: i32) -> windows_core::Result<IEnumCERTVIEWATTRIBUTE>;
    fn EnumCertViewExtension(&self, flags: i32) -> windows_core::Result<IEnumCERTVIEWEXTENSION>;
    fn Skip(&self, celt: i32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumCERTVIEWROW>;
    fn GetMaxIndex(&self, pindex: *mut i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IEnumCERTVIEWROW {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumCERTVIEWROW_Vtbl {
    pub const fn new<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>() -> IEnumCERTVIEWROW_Vtbl {
        unsafe extern "system" fn Next<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWROW_Impl::Next(this, core::mem::transmute_copy(&pindex)).into()
        }
        unsafe extern "system" fn EnumCertViewColumn<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumCERTVIEWROW_Impl::EnumCertViewColumn(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCertViewAttribute<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumCERTVIEWROW_Impl::EnumCertViewAttribute(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumCertViewExtension<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: i32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumCERTVIEWROW_Impl::EnumCertViewExtension(this, core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWROW_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWROW_Impl::Reset(this).into()
        }
        unsafe extern "system" fn Clone<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IEnumCERTVIEWROW_Impl::Clone(this) {
                Ok(ok__) => {
                    ppenum.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxIndex<Identity: IEnumCERTVIEWROW_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IEnumCERTVIEWROW_Impl::GetMaxIndex(this, core::mem::transmute_copy(&pindex)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            EnumCertViewColumn: EnumCertViewColumn::<Identity, OFFSET>,
            EnumCertViewAttribute: EnumCertViewAttribute::<Identity, OFFSET>,
            EnumCertViewExtension: EnumCertViewExtension::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetMaxIndex: GetMaxIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumCERTVIEWROW as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
pub trait INDESPolicy_Impl: Sized + windows_core::IUnknownImpl {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn Uninitialize(&self) -> windows_core::Result<()>;
    fn GenerateChallenge(&self, pwsztemplate: &windows_core::PCWSTR, pwszparams: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PWSTR>;
    fn VerifyRequest(&self, pctbrequest: *mut CERTTRANSBLOB, pctbsigningcertencoded: *mut CERTTRANSBLOB, pwsztemplate: &windows_core::PCWSTR, pwsztransactionid: &windows_core::PCWSTR) -> windows_core::Result<super::super::super::Foundation::BOOL>;
    fn Notify(&self, pwszchallenge: &windows_core::PCWSTR, pwsztransactionid: &windows_core::PCWSTR, disposition: X509SCEPDisposition, lasthresult: i32, pctbissuedcertencoded: *mut CERTTRANSBLOB) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for INDESPolicy {}
impl INDESPolicy_Vtbl {
    pub const fn new<Identity: INDESPolicy_Impl, const OFFSET: isize>() -> INDESPolicy_Vtbl {
        unsafe extern "system" fn Initialize<Identity: INDESPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INDESPolicy_Impl::Initialize(this).into()
        }
        unsafe extern "system" fn Uninitialize<Identity: INDESPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INDESPolicy_Impl::Uninitialize(this).into()
        }
        unsafe extern "system" fn GenerateChallenge<Identity: INDESPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwsztemplate: windows_core::PCWSTR, pwszparams: windows_core::PCWSTR, ppwszresponse: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INDESPolicy_Impl::GenerateChallenge(this, core::mem::transmute(&pwsztemplate), core::mem::transmute(&pwszparams)) {
                Ok(ok__) => {
                    ppwszresponse.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VerifyRequest<Identity: INDESPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pctbrequest: *mut CERTTRANSBLOB, pctbsigningcertencoded: *mut CERTTRANSBLOB, pwsztemplate: windows_core::PCWSTR, pwsztransactionid: windows_core::PCWSTR, pfverified: *mut super::super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match INDESPolicy_Impl::VerifyRequest(this, core::mem::transmute_copy(&pctbrequest), core::mem::transmute_copy(&pctbsigningcertencoded), core::mem::transmute(&pwsztemplate), core::mem::transmute(&pwsztransactionid)) {
                Ok(ok__) => {
                    pfverified.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Notify<Identity: INDESPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszchallenge: windows_core::PCWSTR, pwsztransactionid: windows_core::PCWSTR, disposition: X509SCEPDisposition, lasthresult: i32, pctbissuedcertencoded: *mut CERTTRANSBLOB) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            INDESPolicy_Impl::Notify(this, core::mem::transmute(&pwszchallenge), core::mem::transmute(&pwsztransactionid), core::mem::transmute_copy(&disposition), core::mem::transmute_copy(&lasthresult), core::mem::transmute_copy(&pctbissuedcertencoded)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
            GenerateChallenge: GenerateChallenge::<Identity, OFFSET>,
            VerifyRequest: VerifyRequest::<Identity, OFFSET>,
            Notify: Notify::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INDESPolicy as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IOCSPAdmin_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn OCSPServiceProperties(&self) -> windows_core::Result<IOCSPPropertyCollection>;
    fn OCSPCAConfigurationCollection(&self) -> windows_core::Result<IOCSPCAConfigurationCollection>;
    fn GetConfiguration(&self, bstrservername: &windows_core::BSTR, bforce: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetConfiguration(&self, bstrservername: &windows_core::BSTR, bforce: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetMyRoles(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn Ping(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetSecurity(&self, bstrservername: &windows_core::BSTR, bstrval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetSecurity(&self, bstrservername: &windows_core::BSTR) -> windows_core::Result<windows_core::BSTR>;
    fn GetSigningCertificates(&self, bstrservername: &windows_core::BSTR, pcacertvar: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn GetHashAlgorithms(&self, bstrservername: &windows_core::BSTR, bstrcaid: &windows_core::BSTR) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IOCSPAdmin {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IOCSPAdmin_Vtbl {
    pub const fn new<Identity: IOCSPAdmin_Impl, const OFFSET: isize>() -> IOCSPAdmin_Vtbl {
        unsafe extern "system" fn OCSPServiceProperties<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPAdmin_Impl::OCSPServiceProperties(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OCSPCAConfigurationCollection<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPAdmin_Impl::OCSPCAConfigurationCollection(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConfiguration<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>, bforce: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPAdmin_Impl::GetConfiguration(this, core::mem::transmute(&bstrservername), core::mem::transmute_copy(&bforce)).into()
        }
        unsafe extern "system" fn SetConfiguration<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>, bforce: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPAdmin_Impl::SetConfiguration(this, core::mem::transmute(&bstrservername), core::mem::transmute_copy(&bforce)).into()
        }
        unsafe extern "system" fn GetMyRoles<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>, proles: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPAdmin_Impl::GetMyRoles(this, core::mem::transmute(&bstrservername)) {
                Ok(ok__) => {
                    proles.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Ping<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPAdmin_Impl::Ping(this, core::mem::transmute(&bstrservername)).into()
        }
        unsafe extern "system" fn SetSecurity<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>, bstrval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPAdmin_Impl::SetSecurity(this, core::mem::transmute(&bstrservername), core::mem::transmute(&bstrval)).into()
        }
        unsafe extern "system" fn GetSecurity<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPAdmin_Impl::GetSecurity(this, core::mem::transmute(&bstrservername)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSigningCertificates<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>, pcacertvar: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPAdmin_Impl::GetSigningCertificates(this, core::mem::transmute(&bstrservername), core::mem::transmute_copy(&pcacertvar)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHashAlgorithms<Identity: IOCSPAdmin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrservername: core::mem::MaybeUninit<windows_core::BSTR>, bstrcaid: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPAdmin_Impl::GetHashAlgorithms(this, core::mem::transmute(&bstrservername), core::mem::transmute(&bstrcaid)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            OCSPServiceProperties: OCSPServiceProperties::<Identity, OFFSET>,
            OCSPCAConfigurationCollection: OCSPCAConfigurationCollection::<Identity, OFFSET>,
            GetConfiguration: GetConfiguration::<Identity, OFFSET>,
            SetConfiguration: SetConfiguration::<Identity, OFFSET>,
            GetMyRoles: GetMyRoles::<Identity, OFFSET>,
            Ping: Ping::<Identity, OFFSET>,
            SetSecurity: SetSecurity::<Identity, OFFSET>,
            GetSecurity: GetSecurity::<Identity, OFFSET>,
            GetSigningCertificates: GetSigningCertificates::<Identity, OFFSET>,
            GetHashAlgorithms: GetHashAlgorithms::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPAdmin as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IOCSPCAConfiguration_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Identifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CACertificate(&self) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn HashAlgorithm(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetHashAlgorithm(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SigningFlags(&self) -> windows_core::Result<u32>;
    fn SetSigningFlags(&self, newval: u32) -> windows_core::Result<()>;
    fn SigningCertificate(&self) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetSigningCertificate(&self, newval: &super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn ReminderDuration(&self) -> windows_core::Result<u32>;
    fn SetReminderDuration(&self, newval: u32) -> windows_core::Result<()>;
    fn ErrorCode(&self) -> windows_core::Result<u32>;
    fn CSPName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn KeySpec(&self) -> windows_core::Result<u32>;
    fn ProviderCLSID(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProviderCLSID(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProviderProperties(&self) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetProviderProperties(&self, newval: &super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Modified(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn LocalRevocationInformation(&self) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetLocalRevocationInformation(&self, newval: &super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn SigningCertificateTemplate(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSigningCertificateTemplate(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CAConfig(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCAConfig(&self, newval: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IOCSPCAConfiguration {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IOCSPCAConfiguration_Vtbl {
    pub const fn new<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>() -> IOCSPCAConfiguration_Vtbl {
        unsafe extern "system" fn Identifier<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::Identifier(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CACertificate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::CACertificate(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HashAlgorithm<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::HashAlgorithm(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPCAConfiguration_Impl::SetHashAlgorithm(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn SigningFlags<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::SigningFlags(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningFlags<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPCAConfiguration_Impl::SetSigningFlags(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SigningCertificate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::SigningCertificate(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningCertificate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPCAConfiguration_Impl::SetSigningCertificate(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn ReminderDuration<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::ReminderDuration(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReminderDuration<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPCAConfiguration_Impl::SetReminderDuration(this, core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ErrorCode<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::ErrorCode(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CSPName<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::CSPName(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeySpec<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::KeySpec(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderCLSID<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::ProviderCLSID(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderCLSID<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPCAConfiguration_Impl::SetProviderCLSID(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn ProviderProperties<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::ProviderProperties(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderProperties<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPCAConfiguration_Impl::SetProviderProperties(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Modified<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::Modified(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalRevocationInformation<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::LocalRevocationInformation(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalRevocationInformation<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPCAConfiguration_Impl::SetLocalRevocationInformation(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn SigningCertificateTemplate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::SigningCertificateTemplate(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSigningCertificateTemplate<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPCAConfiguration_Impl::SetSigningCertificateTemplate(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn CAConfig<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfiguration_Impl::CAConfig(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAConfig<Identity: IOCSPCAConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPCAConfiguration_Impl::SetCAConfig(this, core::mem::transmute(&newval)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Identifier: Identifier::<Identity, OFFSET>,
            CACertificate: CACertificate::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            SigningFlags: SigningFlags::<Identity, OFFSET>,
            SetSigningFlags: SetSigningFlags::<Identity, OFFSET>,
            SigningCertificate: SigningCertificate::<Identity, OFFSET>,
            SetSigningCertificate: SetSigningCertificate::<Identity, OFFSET>,
            ReminderDuration: ReminderDuration::<Identity, OFFSET>,
            SetReminderDuration: SetReminderDuration::<Identity, OFFSET>,
            ErrorCode: ErrorCode::<Identity, OFFSET>,
            CSPName: CSPName::<Identity, OFFSET>,
            KeySpec: KeySpec::<Identity, OFFSET>,
            ProviderCLSID: ProviderCLSID::<Identity, OFFSET>,
            SetProviderCLSID: SetProviderCLSID::<Identity, OFFSET>,
            ProviderProperties: ProviderProperties::<Identity, OFFSET>,
            SetProviderProperties: SetProviderProperties::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
            LocalRevocationInformation: LocalRevocationInformation::<Identity, OFFSET>,
            SetLocalRevocationInformation: SetLocalRevocationInformation::<Identity, OFFSET>,
            SigningCertificateTemplate: SigningCertificateTemplate::<Identity, OFFSET>,
            SetSigningCertificateTemplate: SetSigningCertificateTemplate::<Identity, OFFSET>,
            CAConfig: CAConfig::<Identity, OFFSET>,
            SetCAConfig: SetCAConfig::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPCAConfiguration as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IOCSPCAConfigurationCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_ItemByName(&self, bstridentifier: &windows_core::BSTR) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn CreateCAConfiguration(&self, bstridentifier: &windows_core::BSTR, varcacert: &super::super::super::System::Variant::VARIANT) -> windows_core::Result<IOCSPCAConfiguration>;
    fn DeleteCAConfiguration(&self, bstridentifier: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IOCSPCAConfigurationCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IOCSPCAConfigurationCollection_Vtbl {
    pub const fn new<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>() -> IOCSPCAConfigurationCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfigurationCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfigurationCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfigurationCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ItemByName<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstridentifier: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfigurationCollection_Impl::get_ItemByName(this, core::mem::transmute(&bstridentifier)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateCAConfiguration<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstridentifier: core::mem::MaybeUninit<windows_core::BSTR>, varcacert: core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPCAConfigurationCollection_Impl::CreateCAConfiguration(this, core::mem::transmute(&bstridentifier), core::mem::transmute(&varcacert)) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteCAConfiguration<Identity: IOCSPCAConfigurationCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstridentifier: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPCAConfigurationCollection_Impl::DeleteCAConfiguration(this, core::mem::transmute(&bstridentifier)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            get_ItemByName: get_ItemByName::<Identity, OFFSET>,
            CreateCAConfiguration: CreateCAConfiguration::<Identity, OFFSET>,
            DeleteCAConfiguration: DeleteCAConfiguration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPCAConfigurationCollection as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IOCSPProperty_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Value(&self) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetValue(&self, newval: &super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Modified(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IOCSPProperty {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IOCSPProperty_Vtbl {
    pub const fn new<Identity: IOCSPProperty_Impl, const OFFSET: isize>() -> IOCSPProperty_Vtbl {
        unsafe extern "system" fn Name<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPProperty_Impl::Name(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPProperty_Impl::Value(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newval: core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPProperty_Impl::SetValue(this, core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn Modified<Identity: IOCSPProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPProperty_Impl::Modified(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Name: Name::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            Modified: Modified::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPProperty as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IOCSPPropertyCollection_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_Item(&self, index: i32) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn get_ItemByName(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn CreateProperty(&self, bstrpropname: &windows_core::BSTR, pvarpropvalue: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<IOCSPProperty>;
    fn DeleteProperty(&self, bstrpropname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromProperties(&self, pvarproperties: *const super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn GetAllProperties(&self) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IOCSPPropertyCollection {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IOCSPPropertyCollection_Vtbl {
    pub const fn new<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>() -> IOCSPPropertyCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPPropertyCollection_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPPropertyCollection_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPPropertyCollection_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ItemByName<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPPropertyCollection_Impl::get_ItemByName(this, core::mem::transmute(&bstrpropname)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProperty<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>, pvarpropvalue: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>, ppval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPPropertyCollection_Impl::CreateProperty(this, core::mem::transmute(&bstrpropname), core::mem::transmute_copy(&pvarpropvalue)) {
                Ok(ok__) => {
                    ppval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteProperty<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpropname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPPropertyCollection_Impl::DeleteProperty(this, core::mem::transmute(&bstrpropname)).into()
        }
        unsafe extern "system" fn InitializeFromProperties<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarproperties: *const core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IOCSPPropertyCollection_Impl::InitializeFromProperties(this, core::mem::transmute_copy(&pvarproperties)).into()
        }
        unsafe extern "system" fn GetAllProperties<Identity: IOCSPPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarproperties: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOCSPPropertyCollection_Impl::GetAllProperties(this) {
                Ok(ok__) => {
                    pvarproperties.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            get_Item: get_Item::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            get_ItemByName: get_ItemByName::<Identity, OFFSET>,
            CreateProperty: CreateProperty::<Identity, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, OFFSET>,
            InitializeFromProperties: InitializeFromProperties::<Identity, OFFSET>,
            GetAllProperties: GetAllProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOCSPPropertyCollection as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IObjectId_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn InitializeFromName(&self, name: CERTENROLL_OBJECTID) -> windows_core::Result<()>;
    fn InitializeFromValue(&self, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromAlgorithmName(&self, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags, algflags: AlgorithmFlags, stralgorithmname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<CERTENROLL_OBJECTID>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFriendlyName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAlgorithmName(&self, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IObjectId {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IObjectId_Vtbl {
    pub const fn new<Identity: IObjectId_Impl, const OFFSET: isize>() -> IObjectId_Vtbl {
        unsafe extern "system" fn InitializeFromName<Identity: IObjectId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: CERTENROLL_OBJECTID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectId_Impl::InitializeFromName(this, core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn InitializeFromValue<Identity: IObjectId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectId_Impl::InitializeFromValue(this, core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn InitializeFromAlgorithmName<Identity: IObjectId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags, algflags: AlgorithmFlags, stralgorithmname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectId_Impl::InitializeFromAlgorithmName(this, core::mem::transmute_copy(&groupid), core::mem::transmute_copy(&keyflags), core::mem::transmute_copy(&algflags), core::mem::transmute(&stralgorithmname)).into()
        }
        unsafe extern "system" fn Name<Identity: IObjectId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut CERTENROLL_OBJECTID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectId_Impl::Name(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FriendlyName<Identity: IObjectId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectId_Impl::FriendlyName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Identity: IObjectId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectId_Impl::SetFriendlyName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Value<Identity: IObjectId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectId_Impl::Value(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlgorithmName<Identity: IObjectId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, groupid: ObjectIdGroupId, keyflags: ObjectIdPublicKeyFlags, pstralgorithmname: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectId_Impl::GetAlgorithmName(this, core::mem::transmute_copy(&groupid), core::mem::transmute_copy(&keyflags)) {
                Ok(ok__) => {
                    pstralgorithmname.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromName: InitializeFromName::<Identity, OFFSET>,
            InitializeFromValue: InitializeFromValue::<Identity, OFFSET>,
            InitializeFromAlgorithmName: InitializeFromAlgorithmName::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            FriendlyName: FriendlyName::<Identity, OFFSET>,
            SetFriendlyName: SetFriendlyName::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            GetAlgorithmName: GetAlgorithmName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectId as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IObjectIds_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IObjectId>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&IObjectId>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddRange(&self, pvalue: Option<&IObjectIds>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IObjectIds {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IObjectIds_Vtbl {
    pub const fn new<Identity: IObjectIds_Impl, const OFFSET: isize>() -> IObjectIds_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: IObjectIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectIds_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IObjectIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectIds_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IObjectIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IObjectIds_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IObjectIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectIds_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: IObjectIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectIds_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: IObjectIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectIds_Impl::Clear(this).into()
        }
        unsafe extern "system" fn AddRange<Identity: IObjectIds_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IObjectIds_Impl::AddRange(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectIds as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPolicyQualifier_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn InitializeEncode(&self, strqualifier: &windows_core::BSTR, r#type: PolicyQualifierType) -> windows_core::Result<()>;
    fn ObjectId(&self) -> windows_core::Result<IObjectId>;
    fn Qualifier(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Type(&self) -> windows_core::Result<PolicyQualifierType>;
    fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPolicyQualifier {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPolicyQualifier_Vtbl {
    pub const fn new<Identity: IPolicyQualifier_Impl, const OFFSET: isize>() -> IPolicyQualifier_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IPolicyQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strqualifier: core::mem::MaybeUninit<windows_core::BSTR>, r#type: PolicyQualifierType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPolicyQualifier_Impl::InitializeEncode(this, core::mem::transmute(&strqualifier), core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn ObjectId<Identity: IPolicyQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPolicyQualifier_Impl::ObjectId(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Qualifier<Identity: IPolicyQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPolicyQualifier_Impl::Qualifier(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: IPolicyQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut PolicyQualifierType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPolicyQualifier_Impl::Type(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_RawData<Identity: IPolicyQualifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPolicyQualifier_Impl::get_RawData(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            ObjectId: ObjectId::<Identity, OFFSET>,
            Qualifier: Qualifier::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            get_RawData: get_RawData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPolicyQualifier as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPolicyQualifiers_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IPolicyQualifier>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&IPolicyQualifier>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IPolicyQualifiers {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPolicyQualifiers_Vtbl {
    pub const fn new<Identity: IPolicyQualifiers_Impl, const OFFSET: isize>() -> IPolicyQualifiers_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: IPolicyQualifiers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPolicyQualifiers_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IPolicyQualifiers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPolicyQualifiers_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IPolicyQualifiers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPolicyQualifiers_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IPolicyQualifiers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPolicyQualifiers_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: IPolicyQualifiers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPolicyQualifiers_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: IPolicyQualifiers_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPolicyQualifiers_Impl::Clear(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPolicyQualifiers as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISignerCertificate_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, machinecontext: super::super::super::Foundation::VARIANT_BOOL, verifytype: X509PrivateKeyVerify, encoding: EncodingType, strcertificate: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Certificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn PrivateKey(&self) -> windows_core::Result<IX509PrivateKey>;
    fn Silent(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetSilent(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ParentWindow(&self) -> windows_core::Result<i32>;
    fn SetParentWindow(&self, value: i32) -> windows_core::Result<()>;
    fn UIContextMessage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetUIContextMessage(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetPin(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SignatureInformation(&self) -> windows_core::Result<IX509SignatureInformation>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISignerCertificate {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISignerCertificate_Vtbl {
    pub const fn new<Identity: ISignerCertificate_Impl, const OFFSET: isize>() -> ISignerCertificate_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ISignerCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, machinecontext: super::super::super::Foundation::VARIANT_BOOL, verifytype: X509PrivateKeyVerify, encoding: EncodingType, strcertificate: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISignerCertificate_Impl::Initialize(this, core::mem::transmute_copy(&machinecontext), core::mem::transmute_copy(&verifytype), core::mem::transmute_copy(&encoding), core::mem::transmute(&strcertificate)).into()
        }
        unsafe extern "system" fn get_Certificate<Identity: ISignerCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISignerCertificate_Impl::get_Certificate(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateKey<Identity: ISignerCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISignerCertificate_Impl::PrivateKey(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Silent<Identity: ISignerCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISignerCertificate_Impl::Silent(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSilent<Identity: ISignerCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISignerCertificate_Impl::SetSilent(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Identity: ISignerCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISignerCertificate_Impl::ParentWindow(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentWindow<Identity: ISignerCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISignerCertificate_Impl::SetParentWindow(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UIContextMessage<Identity: ISignerCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISignerCertificate_Impl::UIContextMessage(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUIContextMessage<Identity: ISignerCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISignerCertificate_Impl::SetUIContextMessage(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SetPin<Identity: ISignerCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISignerCertificate_Impl::SetPin(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SignatureInformation<Identity: ISignerCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISignerCertificate_Impl::SignatureInformation(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            get_Certificate: get_Certificate::<Identity, OFFSET>,
            PrivateKey: PrivateKey::<Identity, OFFSET>,
            Silent: Silent::<Identity, OFFSET>,
            SetSilent: SetSilent::<Identity, OFFSET>,
            ParentWindow: ParentWindow::<Identity, OFFSET>,
            SetParentWindow: SetParentWindow::<Identity, OFFSET>,
            UIContextMessage: UIContextMessage::<Identity, OFFSET>,
            SetUIContextMessage: SetUIContextMessage::<Identity, OFFSET>,
            SetPin: SetPin::<Identity, OFFSET>,
            SignatureInformation: SignatureInformation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISignerCertificate as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISignerCertificates_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ISignerCertificate>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&ISignerCertificate>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn Find(&self, psignercert: Option<&ISignerCertificate>) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISignerCertificates {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISignerCertificates_Vtbl {
    pub const fn new<Identity: ISignerCertificates_Impl, const OFFSET: isize>() -> ISignerCertificates_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: ISignerCertificates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISignerCertificates_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ISignerCertificates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISignerCertificates_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISignerCertificates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISignerCertificates_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ISignerCertificates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISignerCertificates_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: ISignerCertificates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISignerCertificates_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ISignerCertificates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISignerCertificates_Impl::Clear(this).into()
        }
        unsafe extern "system" fn Find<Identity: ISignerCertificates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psignercert: *mut core::ffi::c_void, pisignercert: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISignerCertificates_Impl::Find(this, windows_core::from_raw_borrowed(&psignercert)) {
                Ok(ok__) => {
                    pisignercert.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            Find: Find::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISignerCertificates as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISmimeCapabilities_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<ISmimeCapability>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&ISmimeCapability>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn AddFromCsp(&self, pvalue: Option<&ICspInformation>) -> windows_core::Result<()>;
    fn AddAvailableSmimeCapabilities(&self, machinecontext: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISmimeCapabilities {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISmimeCapabilities_Vtbl {
    pub const fn new<Identity: ISmimeCapabilities_Impl, const OFFSET: isize>() -> ISmimeCapabilities_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: ISmimeCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmimeCapabilities_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ISmimeCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmimeCapabilities_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ISmimeCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmimeCapabilities_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ISmimeCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmimeCapabilities_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: ISmimeCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmimeCapabilities_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: ISmimeCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmimeCapabilities_Impl::Clear(this).into()
        }
        unsafe extern "system" fn AddFromCsp<Identity: ISmimeCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmimeCapabilities_Impl::AddFromCsp(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn AddAvailableSmimeCapabilities<Identity: ISmimeCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, machinecontext: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmimeCapabilities_Impl::AddAvailableSmimeCapabilities(this, core::mem::transmute_copy(&machinecontext)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            AddFromCsp: AddFromCsp::<Identity, OFFSET>,
            AddAvailableSmimeCapabilities: AddAvailableSmimeCapabilities::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmimeCapabilities as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISmimeCapability_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, pobjectid: Option<&IObjectId>, bitcount: i32) -> windows_core::Result<()>;
    fn ObjectId(&self) -> windows_core::Result<IObjectId>;
    fn BitCount(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for ISmimeCapability {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISmimeCapability_Vtbl {
    pub const fn new<Identity: ISmimeCapability_Impl, const OFFSET: isize>() -> ISmimeCapability_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ISmimeCapability_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectid: *mut core::ffi::c_void, bitcount: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISmimeCapability_Impl::Initialize(this, windows_core::from_raw_borrowed(&pobjectid), core::mem::transmute_copy(&bitcount)).into()
        }
        unsafe extern "system" fn ObjectId<Identity: ISmimeCapability_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmimeCapability_Impl::ObjectId(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BitCount<Identity: ISmimeCapability_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISmimeCapability_Impl::BitCount(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            ObjectId: ObjectId::<Identity, OFFSET>,
            BitCount: BitCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISmimeCapability as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX500DistinguishedName_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Decode(&self, strencodedname: &windows_core::BSTR, encoding: EncodingType, nameflags: X500NameFlags) -> windows_core::Result<()>;
    fn Encode(&self, strname: &windows_core::BSTR, nameflags: X500NameFlags) -> windows_core::Result<()>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_EncodedName(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX500DistinguishedName {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX500DistinguishedName_Vtbl {
    pub const fn new<Identity: IX500DistinguishedName_Impl, const OFFSET: isize>() -> IX500DistinguishedName_Vtbl {
        unsafe extern "system" fn Decode<Identity: IX500DistinguishedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodedname: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType, nameflags: X500NameFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX500DistinguishedName_Impl::Decode(this, core::mem::transmute(&strencodedname), core::mem::transmute_copy(&encoding), core::mem::transmute_copy(&nameflags)).into()
        }
        unsafe extern "system" fn Encode<Identity: IX500DistinguishedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, nameflags: X500NameFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX500DistinguishedName_Impl::Encode(this, core::mem::transmute(&strname), core::mem::transmute_copy(&nameflags)).into()
        }
        unsafe extern "system" fn Name<Identity: IX500DistinguishedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX500DistinguishedName_Impl::Name(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_EncodedName<Identity: IX500DistinguishedName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX500DistinguishedName_Impl::get_EncodedName(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Decode: Decode::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            get_EncodedName: get_EncodedName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX500DistinguishedName as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509Attribute_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, pobjectid: Option<&IObjectId>, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ObjectId(&self) -> windows_core::Result<IObjectId>;
    fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509Attribute {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509Attribute_Vtbl {
    pub const fn new<Identity: IX509Attribute_Impl, const OFFSET: isize>() -> IX509Attribute_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509Attribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectid: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Attribute_Impl::Initialize(this, windows_core::from_raw_borrowed(&pobjectid), core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn ObjectId<Identity: IX509Attribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Attribute_Impl::ObjectId(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_RawData<Identity: IX509Attribute_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Attribute_Impl::get_RawData(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            ObjectId: ObjectId::<Identity, OFFSET>,
            get_RawData: get_RawData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509Attribute as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509AttributeArchiveKey_Impl: Sized + IX509Attribute_Impl {
    fn InitializeEncode(&self, pkey: Option<&IX509PrivateKey>, encoding: EncodingType, strcaxcert: &windows_core::BSTR, palgorithm: Option<&IObjectId>, encryptionstrength: i32) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_EncryptedKeyBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn EncryptionAlgorithm(&self) -> windows_core::Result<IObjectId>;
    fn EncryptionStrength(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509AttributeArchiveKey {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509AttributeArchiveKey_Vtbl {
    pub const fn new<Identity: IX509AttributeArchiveKey_Impl, const OFFSET: isize>() -> IX509AttributeArchiveKey_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509AttributeArchiveKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *mut core::ffi::c_void, encoding: EncodingType, strcaxcert: core::mem::MaybeUninit<windows_core::BSTR>, palgorithm: *mut core::ffi::c_void, encryptionstrength: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeArchiveKey_Impl::InitializeEncode(this, windows_core::from_raw_borrowed(&pkey), core::mem::transmute_copy(&encoding), core::mem::transmute(&strcaxcert), windows_core::from_raw_borrowed(&palgorithm), core::mem::transmute_copy(&encryptionstrength)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509AttributeArchiveKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeArchiveKey_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn get_EncryptedKeyBlob<Identity: IX509AttributeArchiveKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeArchiveKey_Impl::get_EncryptedKeyBlob(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptionAlgorithm<Identity: IX509AttributeArchiveKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeArchiveKey_Impl::EncryptionAlgorithm(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EncryptionStrength<Identity: IX509AttributeArchiveKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeArchiveKey_Impl::EncryptionStrength(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Attribute_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            get_EncryptedKeyBlob: get_EncryptedKeyBlob::<Identity, OFFSET>,
            EncryptionAlgorithm: EncryptionAlgorithm::<Identity, OFFSET>,
            EncryptionStrength: EncryptionStrength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509AttributeArchiveKey as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Attribute as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509AttributeArchiveKeyHash_Impl: Sized + IX509Attribute_Impl {
    fn InitializeEncodeFromEncryptedKeyBlob(&self, encoding: EncodingType, strencryptedkeyblob: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_EncryptedKeyHashBlob(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509AttributeArchiveKeyHash {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509AttributeArchiveKeyHash_Vtbl {
    pub const fn new<Identity: IX509AttributeArchiveKeyHash_Impl, const OFFSET: isize>() -> IX509AttributeArchiveKeyHash_Vtbl {
        unsafe extern "system" fn InitializeEncodeFromEncryptedKeyBlob<Identity: IX509AttributeArchiveKeyHash_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencryptedkeyblob: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeArchiveKeyHash_Impl::InitializeEncodeFromEncryptedKeyBlob(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencryptedkeyblob)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509AttributeArchiveKeyHash_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeArchiveKeyHash_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn get_EncryptedKeyHashBlob<Identity: IX509AttributeArchiveKeyHash_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeArchiveKeyHash_Impl::get_EncryptedKeyHashBlob(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Attribute_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncodeFromEncryptedKeyBlob: InitializeEncodeFromEncryptedKeyBlob::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            get_EncryptedKeyHashBlob: get_EncryptedKeyHashBlob::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509AttributeArchiveKeyHash as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Attribute as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509AttributeClientId_Impl: Sized + IX509Attribute_Impl {
    fn InitializeEncode(&self, clientid: RequestClientInfoClientId, strmachinednsname: &windows_core::BSTR, strusersamname: &windows_core::BSTR, strprocessname: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientId(&self) -> windows_core::Result<RequestClientInfoClientId>;
    fn MachineDnsName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn UserSamName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProcessName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509AttributeClientId {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509AttributeClientId_Vtbl {
    pub const fn new<Identity: IX509AttributeClientId_Impl, const OFFSET: isize>() -> IX509AttributeClientId_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509AttributeClientId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientid: RequestClientInfoClientId, strmachinednsname: core::mem::MaybeUninit<windows_core::BSTR>, strusersamname: core::mem::MaybeUninit<windows_core::BSTR>, strprocessname: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeClientId_Impl::InitializeEncode(this, core::mem::transmute_copy(&clientid), core::mem::transmute(&strmachinednsname), core::mem::transmute(&strusersamname), core::mem::transmute(&strprocessname)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509AttributeClientId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeClientId_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn ClientId<Identity: IX509AttributeClientId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut RequestClientInfoClientId) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeClientId_Impl::ClientId(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MachineDnsName<Identity: IX509AttributeClientId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeClientId_Impl::MachineDnsName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSamName<Identity: IX509AttributeClientId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeClientId_Impl::UserSamName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessName<Identity: IX509AttributeClientId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeClientId_Impl::ProcessName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Attribute_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            ClientId: ClientId::<Identity, OFFSET>,
            MachineDnsName: MachineDnsName::<Identity, OFFSET>,
            UserSamName: UserSamName::<Identity, OFFSET>,
            ProcessName: ProcessName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509AttributeClientId as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Attribute as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509AttributeCspProvider_Impl: Sized + IX509Attribute_Impl {
    fn InitializeEncode(&self, keyspec: X509KeySpec, strprovidername: &windows_core::BSTR, encoding: EncodingType, strsignature: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn KeySpec(&self) -> windows_core::Result<X509KeySpec>;
    fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Signature(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509AttributeCspProvider {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509AttributeCspProvider_Vtbl {
    pub const fn new<Identity: IX509AttributeCspProvider_Impl, const OFFSET: isize>() -> IX509AttributeCspProvider_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509AttributeCspProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keyspec: X509KeySpec, strprovidername: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType, strsignature: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeCspProvider_Impl::InitializeEncode(this, core::mem::transmute_copy(&keyspec), core::mem::transmute(&strprovidername), core::mem::transmute_copy(&encoding), core::mem::transmute(&strsignature)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509AttributeCspProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeCspProvider_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn KeySpec<Identity: IX509AttributeCspProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509KeySpec) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeCspProvider_Impl::KeySpec(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProviderName<Identity: IX509AttributeCspProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeCspProvider_Impl::ProviderName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Signature<Identity: IX509AttributeCspProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeCspProvider_Impl::get_Signature(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Attribute_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            KeySpec: KeySpec::<Identity, OFFSET>,
            ProviderName: ProviderName::<Identity, OFFSET>,
            get_Signature: get_Signature::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509AttributeCspProvider as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Attribute as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509AttributeExtensions_Impl: Sized + IX509Attribute_Impl {
    fn InitializeEncode(&self, pextensions: Option<&IX509Extensions>) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn X509Extensions(&self) -> windows_core::Result<IX509Extensions>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509AttributeExtensions {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509AttributeExtensions_Vtbl {
    pub const fn new<Identity: IX509AttributeExtensions_Impl, const OFFSET: isize>() -> IX509AttributeExtensions_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509AttributeExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pextensions: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeExtensions_Impl::InitializeEncode(this, windows_core::from_raw_borrowed(&pextensions)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509AttributeExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeExtensions_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn X509Extensions<Identity: IX509AttributeExtensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeExtensions_Impl::X509Extensions(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Attribute_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            X509Extensions: X509Extensions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509AttributeExtensions as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Attribute as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509AttributeOSVersion_Impl: Sized + IX509Attribute_Impl {
    fn InitializeEncode(&self, strosversion: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn OSVersion(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509AttributeOSVersion {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509AttributeOSVersion_Vtbl {
    pub const fn new<Identity: IX509AttributeOSVersion_Impl, const OFFSET: isize>() -> IX509AttributeOSVersion_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509AttributeOSVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strosversion: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeOSVersion_Impl::InitializeEncode(this, core::mem::transmute(&strosversion)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509AttributeOSVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeOSVersion_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn OSVersion<Identity: IX509AttributeOSVersion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeOSVersion_Impl::OSVersion(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Attribute_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            OSVersion: OSVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509AttributeOSVersion as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Attribute as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509AttributeRenewalCertificate_Impl: Sized + IX509Attribute_Impl {
    fn InitializeEncode(&self, encoding: EncodingType, strcert: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_RenewalCertificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509AttributeRenewalCertificate {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509AttributeRenewalCertificate_Vtbl {
    pub const fn new<Identity: IX509AttributeRenewalCertificate_Impl, const OFFSET: isize>() -> IX509AttributeRenewalCertificate_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509AttributeRenewalCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strcert: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeRenewalCertificate_Impl::InitializeEncode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strcert)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509AttributeRenewalCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509AttributeRenewalCertificate_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn get_RenewalCertificate<Identity: IX509AttributeRenewalCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509AttributeRenewalCertificate_Impl::get_RenewalCertificate(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Attribute_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            get_RenewalCertificate: get_RenewalCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509AttributeRenewalCertificate as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Attribute as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509Attributes_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509Attribute>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&IX509Attribute>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509Attributes {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509Attributes_Vtbl {
    pub const fn new<Identity: IX509Attributes_Impl, const OFFSET: isize>() -> IX509Attributes_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: IX509Attributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Attributes_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IX509Attributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Attributes_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IX509Attributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Attributes_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IX509Attributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Attributes_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: IX509Attributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Attributes_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: IX509Attributes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Attributes_Impl::Clear(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509Attributes as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRequest_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<()>;
    fn ResetForEncode(&self) -> windows_core::Result<()>;
    fn GetInnerRequest(&self, level: InnerRequestLevel) -> windows_core::Result<IX509CertificateRequest>;
    fn Type(&self) -> windows_core::Result<X509RequestType>;
    fn EnrollmentContext(&self) -> windows_core::Result<X509CertificateEnrollmentContext>;
    fn Silent(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetSilent(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ParentWindow(&self) -> windows_core::Result<i32>;
    fn SetParentWindow(&self, value: i32) -> windows_core::Result<()>;
    fn UIContextMessage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetUIContextMessage(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SuppressDefaults(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetSuppressDefaults(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_RenewalCertificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn put_RenewalCertificate(&self, encoding: EncodingType, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientId(&self) -> windows_core::Result<RequestClientInfoClientId>;
    fn SetClientId(&self, value: RequestClientInfoClientId) -> windows_core::Result<()>;
    fn CspInformations(&self) -> windows_core::Result<ICspInformations>;
    fn SetCspInformations(&self, pvalue: Option<&ICspInformations>) -> windows_core::Result<()>;
    fn HashAlgorithm(&self) -> windows_core::Result<IObjectId>;
    fn SetHashAlgorithm(&self, pvalue: Option<&IObjectId>) -> windows_core::Result<()>;
    fn AlternateSignatureAlgorithm(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetAlternateSignatureAlgorithm(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRequest {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRequest_Vtbl {
    pub const fn new<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>() -> IX509CertificateRequest_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::Initialize(this, core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn Encode<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::Encode(this).into()
        }
        unsafe extern "system" fn ResetForEncode<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::ResetForEncode(this).into()
        }
        unsafe extern "system" fn GetInnerRequest<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, level: InnerRequestLevel, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::GetInnerRequest(this, core::mem::transmute_copy(&level)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509RequestType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::Type(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnrollmentContext<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509CertificateEnrollmentContext) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::EnrollmentContext(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Silent<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::Silent(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSilent<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::SetSilent(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::ParentWindow(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentWindow<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::SetParentWindow(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UIContextMessage<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::UIContextMessage(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUIContextMessage<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::SetUIContextMessage(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SuppressDefaults<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::SuppressDefaults(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSuppressDefaults<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::SetSuppressDefaults(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn get_RenewalCertificate<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::get_RenewalCertificate(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_RenewalCertificate<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::put_RenewalCertificate(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ClientId<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut RequestClientInfoClientId) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::ClientId(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientId<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: RequestClientInfoClientId) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::SetClientId(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn CspInformations<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::CspInformations(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCspInformations<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::SetCspInformations(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn HashAlgorithm<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::HashAlgorithm(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::SetHashAlgorithm(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithm<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::AlternateSignatureAlgorithm(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlternateSignatureAlgorithm<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequest_Impl::SetAlternateSignatureAlgorithm(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn get_RawData<Identity: IX509CertificateRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequest_Impl::get_RawData(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
            ResetForEncode: ResetForEncode::<Identity, OFFSET>,
            GetInnerRequest: GetInnerRequest::<Identity, OFFSET>,
            Type: Type::<Identity, OFFSET>,
            EnrollmentContext: EnrollmentContext::<Identity, OFFSET>,
            Silent: Silent::<Identity, OFFSET>,
            SetSilent: SetSilent::<Identity, OFFSET>,
            ParentWindow: ParentWindow::<Identity, OFFSET>,
            SetParentWindow: SetParentWindow::<Identity, OFFSET>,
            UIContextMessage: UIContextMessage::<Identity, OFFSET>,
            SetUIContextMessage: SetUIContextMessage::<Identity, OFFSET>,
            SuppressDefaults: SuppressDefaults::<Identity, OFFSET>,
            SetSuppressDefaults: SetSuppressDefaults::<Identity, OFFSET>,
            get_RenewalCertificate: get_RenewalCertificate::<Identity, OFFSET>,
            put_RenewalCertificate: put_RenewalCertificate::<Identity, OFFSET>,
            ClientId: ClientId::<Identity, OFFSET>,
            SetClientId: SetClientId::<Identity, OFFSET>,
            CspInformations: CspInformations::<Identity, OFFSET>,
            SetCspInformations: SetCspInformations::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            AlternateSignatureAlgorithm: AlternateSignatureAlgorithm::<Identity, OFFSET>,
            SetAlternateSignatureAlgorithm: SetAlternateSignatureAlgorithm::<Identity, OFFSET>,
            get_RawData: get_RawData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRequest as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRequestCertificate_Impl: Sized + IX509CertificateRequestPkcs10_Impl {
    fn CheckPublicKeySignature(&self, ppublickey: Option<&IX509PublicKey>) -> windows_core::Result<()>;
    fn Issuer(&self) -> windows_core::Result<IX500DistinguishedName>;
    fn SetIssuer(&self, pvalue: Option<&IX500DistinguishedName>) -> windows_core::Result<()>;
    fn NotBefore(&self) -> windows_core::Result<f64>;
    fn SetNotBefore(&self, value: f64) -> windows_core::Result<()>;
    fn NotAfter(&self) -> windows_core::Result<f64>;
    fn SetNotAfter(&self, value: f64) -> windows_core::Result<()>;
    fn get_SerialNumber(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn put_SerialNumber(&self, encoding: EncodingType, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SignerCertificate(&self) -> windows_core::Result<ISignerCertificate>;
    fn SetSignerCertificate(&self, pvalue: Option<&ISignerCertificate>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRequestCertificate {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRequestCertificate_Vtbl {
    pub const fn new<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>() -> IX509CertificateRequestCertificate_Vtbl {
        unsafe extern "system" fn CheckPublicKeySignature<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppublickey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCertificate_Impl::CheckPublicKeySignature(this, windows_core::from_raw_borrowed(&ppublickey)).into()
        }
        unsafe extern "system" fn Issuer<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCertificate_Impl::Issuer(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIssuer<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCertificate_Impl::SetIssuer(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn NotBefore<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCertificate_Impl::NotBefore(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotBefore<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCertificate_Impl::SetNotBefore(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn NotAfter<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCertificate_Impl::NotAfter(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotAfter<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCertificate_Impl::SetNotAfter(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn get_SerialNumber<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCertificate_Impl::get_SerialNumber(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_SerialNumber<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCertificate_Impl::put_SerialNumber(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SignerCertificate<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCertificate_Impl::SignerCertificate(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignerCertificate<Identity: IX509CertificateRequestCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCertificate_Impl::SetSignerCertificate(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        Self {
            base__: IX509CertificateRequestPkcs10_Vtbl::new::<Identity, OFFSET>(),
            CheckPublicKeySignature: CheckPublicKeySignature::<Identity, OFFSET>,
            Issuer: Issuer::<Identity, OFFSET>,
            SetIssuer: SetIssuer::<Identity, OFFSET>,
            NotBefore: NotBefore::<Identity, OFFSET>,
            SetNotBefore: SetNotBefore::<Identity, OFFSET>,
            NotAfter: NotAfter::<Identity, OFFSET>,
            SetNotAfter: SetNotAfter::<Identity, OFFSET>,
            get_SerialNumber: get_SerialNumber::<Identity, OFFSET>,
            put_SerialNumber: put_SerialNumber::<Identity, OFFSET>,
            SignerCertificate: SignerCertificate::<Identity, OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRequestCertificate as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509CertificateRequest as windows_core::Interface>::IID || iid == &<IX509CertificateRequestPkcs10 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRequestCertificate2_Impl: Sized + IX509CertificateRequestCertificate_Impl {
    fn InitializeFromTemplate(&self, context: X509CertificateEnrollmentContext, ppolicyserver: Option<&IX509EnrollmentPolicyServer>, ptemplate: Option<&IX509CertificateTemplate>) -> windows_core::Result<()>;
    fn InitializeFromPrivateKeyTemplate(&self, context: X509CertificateEnrollmentContext, pprivatekey: Option<&IX509PrivateKey>, ppolicyserver: Option<&IX509EnrollmentPolicyServer>, ptemplate: Option<&IX509CertificateTemplate>) -> windows_core::Result<()>;
    fn PolicyServer(&self) -> windows_core::Result<IX509EnrollmentPolicyServer>;
    fn Template(&self) -> windows_core::Result<IX509CertificateTemplate>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRequestCertificate2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRequestCertificate2_Vtbl {
    pub const fn new<Identity: IX509CertificateRequestCertificate2_Impl, const OFFSET: isize>() -> IX509CertificateRequestCertificate2_Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Identity: IX509CertificateRequestCertificate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCertificate2_Impl::InitializeFromTemplate(this, core::mem::transmute_copy(&context), windows_core::from_raw_borrowed(&ppolicyserver), windows_core::from_raw_borrowed(&ptemplate)).into()
        }
        unsafe extern "system" fn InitializeFromPrivateKeyTemplate<Identity: IX509CertificateRequestCertificate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, pprivatekey: *mut core::ffi::c_void, ppolicyserver: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCertificate2_Impl::InitializeFromPrivateKeyTemplate(this, core::mem::transmute_copy(&context), windows_core::from_raw_borrowed(&pprivatekey), windows_core::from_raw_borrowed(&ppolicyserver), windows_core::from_raw_borrowed(&ptemplate)).into()
        }
        unsafe extern "system" fn PolicyServer<Identity: IX509CertificateRequestCertificate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppolicyserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCertificate2_Impl::PolicyServer(this) {
                Ok(ok__) => {
                    pppolicyserver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Template<Identity: IX509CertificateRequestCertificate2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCertificate2_Impl::Template(this) {
                Ok(ok__) => {
                    pptemplate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509CertificateRequestCertificate_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromTemplate: InitializeFromTemplate::<Identity, OFFSET>,
            InitializeFromPrivateKeyTemplate: InitializeFromPrivateKeyTemplate::<Identity, OFFSET>,
            PolicyServer: PolicyServer::<Identity, OFFSET>,
            Template: Template::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRequestCertificate2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509CertificateRequest as windows_core::Interface>::IID || iid == &<IX509CertificateRequestPkcs10 as windows_core::Interface>::IID || iid == &<IX509CertificateRequestCertificate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRequestCmc_Impl: Sized + IX509CertificateRequestPkcs7_Impl {
    fn InitializeFromInnerRequestTemplateName(&self, pinnerrequest: Option<&IX509CertificateRequest>, strtemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TemplateObjectId(&self) -> windows_core::Result<IObjectId>;
    fn NullSigned(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn CryptAttributes(&self) -> windows_core::Result<ICryptAttributes>;
    fn NameValuePairs(&self) -> windows_core::Result<IX509NameValuePairs>;
    fn X509Extensions(&self) -> windows_core::Result<IX509Extensions>;
    fn CriticalExtensions(&self) -> windows_core::Result<IObjectIds>;
    fn SuppressOids(&self) -> windows_core::Result<IObjectIds>;
    fn TransactionId(&self) -> windows_core::Result<i32>;
    fn SetTransactionId(&self, value: i32) -> windows_core::Result<()>;
    fn get_SenderNonce(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn put_SenderNonce(&self, encoding: EncodingType, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SignatureInformation(&self) -> windows_core::Result<IX509SignatureInformation>;
    fn ArchivePrivateKey(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetArchivePrivateKey(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_KeyArchivalCertificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn put_KeyArchivalCertificate(&self, encoding: EncodingType, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EncryptionAlgorithm(&self) -> windows_core::Result<IObjectId>;
    fn SetEncryptionAlgorithm(&self, pvalue: Option<&IObjectId>) -> windows_core::Result<()>;
    fn EncryptionStrength(&self) -> windows_core::Result<i32>;
    fn SetEncryptionStrength(&self, value: i32) -> windows_core::Result<()>;
    fn get_EncryptedKeyHash(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn SignerCertificates(&self) -> windows_core::Result<ISignerCertificates>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRequestCmc {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRequestCmc_Vtbl {
    pub const fn new<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>() -> IX509CertificateRequestCmc_Vtbl {
        unsafe extern "system" fn InitializeFromInnerRequestTemplateName<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinnerrequest: *mut core::ffi::c_void, strtemplatename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCmc_Impl::InitializeFromInnerRequestTemplateName(this, windows_core::from_raw_borrowed(&pinnerrequest), core::mem::transmute(&strtemplatename)).into()
        }
        unsafe extern "system" fn TemplateObjectId<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::TemplateObjectId(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NullSigned<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::NullSigned(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CryptAttributes<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::CryptAttributes(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NameValuePairs<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::NameValuePairs(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X509Extensions<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::X509Extensions(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CriticalExtensions<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::CriticalExtensions(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuppressOids<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::SuppressOids(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionId<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::TransactionId(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransactionId<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCmc_Impl::SetTransactionId(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn get_SenderNonce<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::get_SenderNonce(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_SenderNonce<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCmc_Impl::put_SenderNonce(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SignatureInformation<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::SignatureInformation(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArchivePrivateKey<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::ArchivePrivateKey(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetArchivePrivateKey<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCmc_Impl::SetArchivePrivateKey(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn get_KeyArchivalCertificate<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::get_KeyArchivalCertificate(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_KeyArchivalCertificate<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCmc_Impl::put_KeyArchivalCertificate(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn EncryptionAlgorithm<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::EncryptionAlgorithm(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptionAlgorithm<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCmc_Impl::SetEncryptionAlgorithm(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn EncryptionStrength<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::EncryptionStrength(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptionStrength<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCmc_Impl::SetEncryptionStrength(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn get_EncryptedKeyHash<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::get_EncryptedKeyHash(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignerCertificates<Identity: IX509CertificateRequestCmc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc_Impl::SignerCertificates(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509CertificateRequestPkcs7_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromInnerRequestTemplateName: InitializeFromInnerRequestTemplateName::<Identity, OFFSET>,
            TemplateObjectId: TemplateObjectId::<Identity, OFFSET>,
            NullSigned: NullSigned::<Identity, OFFSET>,
            CryptAttributes: CryptAttributes::<Identity, OFFSET>,
            NameValuePairs: NameValuePairs::<Identity, OFFSET>,
            X509Extensions: X509Extensions::<Identity, OFFSET>,
            CriticalExtensions: CriticalExtensions::<Identity, OFFSET>,
            SuppressOids: SuppressOids::<Identity, OFFSET>,
            TransactionId: TransactionId::<Identity, OFFSET>,
            SetTransactionId: SetTransactionId::<Identity, OFFSET>,
            get_SenderNonce: get_SenderNonce::<Identity, OFFSET>,
            put_SenderNonce: put_SenderNonce::<Identity, OFFSET>,
            SignatureInformation: SignatureInformation::<Identity, OFFSET>,
            ArchivePrivateKey: ArchivePrivateKey::<Identity, OFFSET>,
            SetArchivePrivateKey: SetArchivePrivateKey::<Identity, OFFSET>,
            get_KeyArchivalCertificate: get_KeyArchivalCertificate::<Identity, OFFSET>,
            put_KeyArchivalCertificate: put_KeyArchivalCertificate::<Identity, OFFSET>,
            EncryptionAlgorithm: EncryptionAlgorithm::<Identity, OFFSET>,
            SetEncryptionAlgorithm: SetEncryptionAlgorithm::<Identity, OFFSET>,
            EncryptionStrength: EncryptionStrength::<Identity, OFFSET>,
            SetEncryptionStrength: SetEncryptionStrength::<Identity, OFFSET>,
            get_EncryptedKeyHash: get_EncryptedKeyHash::<Identity, OFFSET>,
            SignerCertificates: SignerCertificates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRequestCmc as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509CertificateRequest as windows_core::Interface>::IID || iid == &<IX509CertificateRequestPkcs7 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRequestCmc2_Impl: Sized + IX509CertificateRequestCmc_Impl {
    fn InitializeFromTemplate(&self, context: X509CertificateEnrollmentContext, ppolicyserver: Option<&IX509EnrollmentPolicyServer>, ptemplate: Option<&IX509CertificateTemplate>) -> windows_core::Result<()>;
    fn InitializeFromInnerRequestTemplate(&self, pinnerrequest: Option<&IX509CertificateRequest>, ppolicyserver: Option<&IX509EnrollmentPolicyServer>, ptemplate: Option<&IX509CertificateTemplate>) -> windows_core::Result<()>;
    fn PolicyServer(&self) -> windows_core::Result<IX509EnrollmentPolicyServer>;
    fn Template(&self) -> windows_core::Result<IX509CertificateTemplate>;
    fn CheckSignature(&self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> windows_core::Result<()>;
    fn CheckCertificateSignature(&self, psignercertificate: Option<&ISignerCertificate>, validatecertificatechain: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRequestCmc2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRequestCmc2_Vtbl {
    pub const fn new<Identity: IX509CertificateRequestCmc2_Impl, const OFFSET: isize>() -> IX509CertificateRequestCmc2_Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Identity: IX509CertificateRequestCmc2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCmc2_Impl::InitializeFromTemplate(this, core::mem::transmute_copy(&context), windows_core::from_raw_borrowed(&ppolicyserver), windows_core::from_raw_borrowed(&ptemplate)).into()
        }
        unsafe extern "system" fn InitializeFromInnerRequestTemplate<Identity: IX509CertificateRequestCmc2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinnerrequest: *mut core::ffi::c_void, ppolicyserver: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCmc2_Impl::InitializeFromInnerRequestTemplate(this, windows_core::from_raw_borrowed(&pinnerrequest), windows_core::from_raw_borrowed(&ppolicyserver), windows_core::from_raw_borrowed(&ptemplate)).into()
        }
        unsafe extern "system" fn PolicyServer<Identity: IX509CertificateRequestCmc2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppolicyserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc2_Impl::PolicyServer(this) {
                Ok(ok__) => {
                    pppolicyserver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Template<Identity: IX509CertificateRequestCmc2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestCmc2_Impl::Template(this) {
                Ok(ok__) => {
                    pptemplate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckSignature<Identity: IX509CertificateRequestCmc2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCmc2_Impl::CheckSignature(this, core::mem::transmute_copy(&allowedsignaturetypes)).into()
        }
        unsafe extern "system" fn CheckCertificateSignature<Identity: IX509CertificateRequestCmc2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psignercertificate: *mut core::ffi::c_void, validatecertificatechain: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestCmc2_Impl::CheckCertificateSignature(this, windows_core::from_raw_borrowed(&psignercertificate), core::mem::transmute_copy(&validatecertificatechain)).into()
        }
        Self {
            base__: IX509CertificateRequestCmc_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromTemplate: InitializeFromTemplate::<Identity, OFFSET>,
            InitializeFromInnerRequestTemplate: InitializeFromInnerRequestTemplate::<Identity, OFFSET>,
            PolicyServer: PolicyServer::<Identity, OFFSET>,
            Template: Template::<Identity, OFFSET>,
            CheckSignature: CheckSignature::<Identity, OFFSET>,
            CheckCertificateSignature: CheckCertificateSignature::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRequestCmc2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509CertificateRequest as windows_core::Interface>::IID || iid == &<IX509CertificateRequestPkcs7 as windows_core::Interface>::IID || iid == &<IX509CertificateRequestCmc as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRequestPkcs10_Impl: Sized + IX509CertificateRequest_Impl {
    fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromPrivateKey(&self, context: X509CertificateEnrollmentContext, pprivatekey: Option<&IX509PrivateKey>, strtemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromPublicKey(&self, context: X509CertificateEnrollmentContext, ppublickey: Option<&IX509PublicKey>, strtemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromCertificate(&self, context: X509CertificateEnrollmentContext, strcertificate: &windows_core::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> windows_core::Result<()>;
    fn InitializeDecode(&self, strencodeddata: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn CheckSignature(&self, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> windows_core::Result<()>;
    fn IsSmartCard(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn TemplateObjectId(&self) -> windows_core::Result<IObjectId>;
    fn PublicKey(&self) -> windows_core::Result<IX509PublicKey>;
    fn PrivateKey(&self) -> windows_core::Result<IX509PrivateKey>;
    fn NullSigned(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn ReuseKey(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn get_OldCertificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn Subject(&self) -> windows_core::Result<IX500DistinguishedName>;
    fn SetSubject(&self, pvalue: Option<&IX500DistinguishedName>) -> windows_core::Result<()>;
    fn CspStatuses(&self) -> windows_core::Result<ICspStatuses>;
    fn SmimeCapabilities(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetSmimeCapabilities(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SignatureInformation(&self) -> windows_core::Result<IX509SignatureInformation>;
    fn KeyContainerNamePrefix(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetKeyContainerNamePrefix(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CryptAttributes(&self) -> windows_core::Result<ICryptAttributes>;
    fn X509Extensions(&self) -> windows_core::Result<IX509Extensions>;
    fn CriticalExtensions(&self) -> windows_core::Result<IObjectIds>;
    fn SuppressOids(&self) -> windows_core::Result<IObjectIds>;
    fn get_RawDataToBeSigned(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn get_Signature(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn GetCspStatuses(&self, keyspec: X509KeySpec) -> windows_core::Result<ICspStatuses>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRequestPkcs10 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRequestPkcs10_Vtbl {
    pub const fn new<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>() -> IX509CertificateRequestPkcs10_Vtbl {
        unsafe extern "system" fn InitializeFromTemplateName<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, strtemplatename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10_Impl::InitializeFromTemplateName(this, core::mem::transmute_copy(&context), core::mem::transmute(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeFromPrivateKey<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, pprivatekey: *mut core::ffi::c_void, strtemplatename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10_Impl::InitializeFromPrivateKey(this, core::mem::transmute_copy(&context), windows_core::from_raw_borrowed(&pprivatekey), core::mem::transmute(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeFromPublicKey<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, ppublickey: *mut core::ffi::c_void, strtemplatename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10_Impl::InitializeFromPublicKey(this, core::mem::transmute_copy(&context), windows_core::from_raw_borrowed(&ppublickey), core::mem::transmute(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeFromCertificate<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, strcertificate: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10_Impl::InitializeFromCertificate(this, core::mem::transmute_copy(&context), core::mem::transmute(&strcertificate), core::mem::transmute_copy(&encoding), core::mem::transmute_copy(&inheritoptions)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10_Impl::InitializeDecode(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn CheckSignature<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowedsignaturetypes: Pkcs10AllowedSignatureTypes) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10_Impl::CheckSignature(this, core::mem::transmute_copy(&allowedsignaturetypes)).into()
        }
        unsafe extern "system" fn IsSmartCard<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::IsSmartCard(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TemplateObjectId<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::TemplateObjectId(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublicKey<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::PublicKey(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrivateKey<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::PrivateKey(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NullSigned<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::NullSigned(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReuseKey<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::ReuseKey(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_OldCertificate<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::get_OldCertificate(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Subject<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::Subject(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubject<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10_Impl::SetSubject(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn CspStatuses<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::CspStatuses(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SmimeCapabilities<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::SmimeCapabilities(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSmimeCapabilities<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10_Impl::SetSmimeCapabilities(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SignatureInformation<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::SignatureInformation(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyContainerNamePrefix<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::KeyContainerNamePrefix(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyContainerNamePrefix<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10_Impl::SetKeyContainerNamePrefix(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CryptAttributes<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::CryptAttributes(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X509Extensions<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::X509Extensions(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CriticalExtensions<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::CriticalExtensions(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuppressOids<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::SuppressOids(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_RawDataToBeSigned<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::get_RawDataToBeSigned(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Signature<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::get_Signature(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCspStatuses<Identity: IX509CertificateRequestPkcs10_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keyspec: X509KeySpec, ppcspstatuses: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10_Impl::GetCspStatuses(this, core::mem::transmute_copy(&keyspec)) {
                Ok(ok__) => {
                    ppcspstatuses.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509CertificateRequest_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromTemplateName: InitializeFromTemplateName::<Identity, OFFSET>,
            InitializeFromPrivateKey: InitializeFromPrivateKey::<Identity, OFFSET>,
            InitializeFromPublicKey: InitializeFromPublicKey::<Identity, OFFSET>,
            InitializeFromCertificate: InitializeFromCertificate::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            CheckSignature: CheckSignature::<Identity, OFFSET>,
            IsSmartCard: IsSmartCard::<Identity, OFFSET>,
            TemplateObjectId: TemplateObjectId::<Identity, OFFSET>,
            PublicKey: PublicKey::<Identity, OFFSET>,
            PrivateKey: PrivateKey::<Identity, OFFSET>,
            NullSigned: NullSigned::<Identity, OFFSET>,
            ReuseKey: ReuseKey::<Identity, OFFSET>,
            get_OldCertificate: get_OldCertificate::<Identity, OFFSET>,
            Subject: Subject::<Identity, OFFSET>,
            SetSubject: SetSubject::<Identity, OFFSET>,
            CspStatuses: CspStatuses::<Identity, OFFSET>,
            SmimeCapabilities: SmimeCapabilities::<Identity, OFFSET>,
            SetSmimeCapabilities: SetSmimeCapabilities::<Identity, OFFSET>,
            SignatureInformation: SignatureInformation::<Identity, OFFSET>,
            KeyContainerNamePrefix: KeyContainerNamePrefix::<Identity, OFFSET>,
            SetKeyContainerNamePrefix: SetKeyContainerNamePrefix::<Identity, OFFSET>,
            CryptAttributes: CryptAttributes::<Identity, OFFSET>,
            X509Extensions: X509Extensions::<Identity, OFFSET>,
            CriticalExtensions: CriticalExtensions::<Identity, OFFSET>,
            SuppressOids: SuppressOids::<Identity, OFFSET>,
            get_RawDataToBeSigned: get_RawDataToBeSigned::<Identity, OFFSET>,
            get_Signature: get_Signature::<Identity, OFFSET>,
            GetCspStatuses: GetCspStatuses::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509CertificateRequest as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRequestPkcs10V2_Impl: Sized + IX509CertificateRequestPkcs10_Impl {
    fn InitializeFromTemplate(&self, context: X509CertificateEnrollmentContext, ppolicyserver: Option<&IX509EnrollmentPolicyServer>, ptemplate: Option<&IX509CertificateTemplate>) -> windows_core::Result<()>;
    fn InitializeFromPrivateKeyTemplate(&self, context: X509CertificateEnrollmentContext, pprivatekey: Option<&IX509PrivateKey>, ppolicyserver: Option<&IX509EnrollmentPolicyServer>, ptemplate: Option<&IX509CertificateTemplate>) -> windows_core::Result<()>;
    fn InitializeFromPublicKeyTemplate(&self, context: X509CertificateEnrollmentContext, ppublickey: Option<&IX509PublicKey>, ppolicyserver: Option<&IX509EnrollmentPolicyServer>, ptemplate: Option<&IX509CertificateTemplate>) -> windows_core::Result<()>;
    fn PolicyServer(&self) -> windows_core::Result<IX509EnrollmentPolicyServer>;
    fn Template(&self) -> windows_core::Result<IX509CertificateTemplate>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRequestPkcs10V2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRequestPkcs10V2_Vtbl {
    pub const fn new<Identity: IX509CertificateRequestPkcs10V2_Impl, const OFFSET: isize>() -> IX509CertificateRequestPkcs10V2_Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Identity: IX509CertificateRequestPkcs10V2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10V2_Impl::InitializeFromTemplate(this, core::mem::transmute_copy(&context), windows_core::from_raw_borrowed(&ppolicyserver), windows_core::from_raw_borrowed(&ptemplate)).into()
        }
        unsafe extern "system" fn InitializeFromPrivateKeyTemplate<Identity: IX509CertificateRequestPkcs10V2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, pprivatekey: *mut core::ffi::c_void, ppolicyserver: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10V2_Impl::InitializeFromPrivateKeyTemplate(this, core::mem::transmute_copy(&context), windows_core::from_raw_borrowed(&pprivatekey), windows_core::from_raw_borrowed(&ppolicyserver), windows_core::from_raw_borrowed(&ptemplate)).into()
        }
        unsafe extern "system" fn InitializeFromPublicKeyTemplate<Identity: IX509CertificateRequestPkcs10V2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, ppublickey: *mut core::ffi::c_void, ppolicyserver: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10V2_Impl::InitializeFromPublicKeyTemplate(this, core::mem::transmute_copy(&context), windows_core::from_raw_borrowed(&ppublickey), windows_core::from_raw_borrowed(&ppolicyserver), windows_core::from_raw_borrowed(&ptemplate)).into()
        }
        unsafe extern "system" fn PolicyServer<Identity: IX509CertificateRequestPkcs10V2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppolicyserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10V2_Impl::PolicyServer(this) {
                Ok(ok__) => {
                    pppolicyserver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Template<Identity: IX509CertificateRequestPkcs10V2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10V2_Impl::Template(this) {
                Ok(ok__) => {
                    pptemplate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509CertificateRequestPkcs10_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromTemplate: InitializeFromTemplate::<Identity, OFFSET>,
            InitializeFromPrivateKeyTemplate: InitializeFromPrivateKeyTemplate::<Identity, OFFSET>,
            InitializeFromPublicKeyTemplate: InitializeFromPublicKeyTemplate::<Identity, OFFSET>,
            PolicyServer: PolicyServer::<Identity, OFFSET>,
            Template: Template::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10V2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509CertificateRequest as windows_core::Interface>::IID || iid == &<IX509CertificateRequestPkcs10 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRequestPkcs10V3_Impl: Sized + IX509CertificateRequestPkcs10V2_Impl {
    fn AttestPrivateKey(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetAttestPrivateKey(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_AttestationEncryptionCertificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn put_AttestationEncryptionCertificate(&self, encoding: EncodingType, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EncryptionAlgorithm(&self) -> windows_core::Result<IObjectId>;
    fn SetEncryptionAlgorithm(&self, pvalue: Option<&IObjectId>) -> windows_core::Result<()>;
    fn EncryptionStrength(&self) -> windows_core::Result<i32>;
    fn SetEncryptionStrength(&self, value: i32) -> windows_core::Result<()>;
    fn ChallengePassword(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetChallengePassword(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NameValuePairs(&self) -> windows_core::Result<IX509NameValuePairs>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRequestPkcs10V3 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRequestPkcs10V3_Vtbl {
    pub const fn new<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>() -> IX509CertificateRequestPkcs10V3_Vtbl {
        unsafe extern "system" fn AttestPrivateKey<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10V3_Impl::AttestPrivateKey(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttestPrivateKey<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10V3_Impl::SetAttestPrivateKey(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn get_AttestationEncryptionCertificate<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10V3_Impl::get_AttestationEncryptionCertificate(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_AttestationEncryptionCertificate<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10V3_Impl::put_AttestationEncryptionCertificate(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn EncryptionAlgorithm<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10V3_Impl::EncryptionAlgorithm(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptionAlgorithm<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10V3_Impl::SetEncryptionAlgorithm(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn EncryptionStrength<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10V3_Impl::EncryptionStrength(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEncryptionStrength<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10V3_Impl::SetEncryptionStrength(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ChallengePassword<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10V3_Impl::ChallengePassword(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChallengePassword<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10V3_Impl::SetChallengePassword(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn NameValuePairs<Identity: IX509CertificateRequestPkcs10V3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10V3_Impl::NameValuePairs(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509CertificateRequestPkcs10V2_Vtbl::new::<Identity, OFFSET>(),
            AttestPrivateKey: AttestPrivateKey::<Identity, OFFSET>,
            SetAttestPrivateKey: SetAttestPrivateKey::<Identity, OFFSET>,
            get_AttestationEncryptionCertificate: get_AttestationEncryptionCertificate::<Identity, OFFSET>,
            put_AttestationEncryptionCertificate: put_AttestationEncryptionCertificate::<Identity, OFFSET>,
            EncryptionAlgorithm: EncryptionAlgorithm::<Identity, OFFSET>,
            SetEncryptionAlgorithm: SetEncryptionAlgorithm::<Identity, OFFSET>,
            EncryptionStrength: EncryptionStrength::<Identity, OFFSET>,
            SetEncryptionStrength: SetEncryptionStrength::<Identity, OFFSET>,
            ChallengePassword: ChallengePassword::<Identity, OFFSET>,
            SetChallengePassword: SetChallengePassword::<Identity, OFFSET>,
            NameValuePairs: NameValuePairs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10V3 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509CertificateRequest as windows_core::Interface>::IID || iid == &<IX509CertificateRequestPkcs10 as windows_core::Interface>::IID || iid == &<IX509CertificateRequestPkcs10V2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRequestPkcs10V4_Impl: Sized + IX509CertificateRequestPkcs10V3_Impl {
    fn ClaimType(&self) -> windows_core::Result<KeyAttestationClaimType>;
    fn SetClaimType(&self, value: KeyAttestationClaimType) -> windows_core::Result<()>;
    fn AttestPrivateKeyPreferred(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetAttestPrivateKeyPreferred(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRequestPkcs10V4 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRequestPkcs10V4_Vtbl {
    pub const fn new<Identity: IX509CertificateRequestPkcs10V4_Impl, const OFFSET: isize>() -> IX509CertificateRequestPkcs10V4_Vtbl {
        unsafe extern "system" fn ClaimType<Identity: IX509CertificateRequestPkcs10V4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut KeyAttestationClaimType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10V4_Impl::ClaimType(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClaimType<Identity: IX509CertificateRequestPkcs10V4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: KeyAttestationClaimType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10V4_Impl::SetClaimType(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AttestPrivateKeyPreferred<Identity: IX509CertificateRequestPkcs10V4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs10V4_Impl::AttestPrivateKeyPreferred(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttestPrivateKeyPreferred<Identity: IX509CertificateRequestPkcs10V4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs10V4_Impl::SetAttestPrivateKeyPreferred(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IX509CertificateRequestPkcs10V3_Vtbl::new::<Identity, OFFSET>(),
            ClaimType: ClaimType::<Identity, OFFSET>,
            SetClaimType: SetClaimType::<Identity, OFFSET>,
            AttestPrivateKeyPreferred: AttestPrivateKeyPreferred::<Identity, OFFSET>,
            SetAttestPrivateKeyPreferred: SetAttestPrivateKeyPreferred::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs10V4 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509CertificateRequest as windows_core::Interface>::IID || iid == &<IX509CertificateRequestPkcs10 as windows_core::Interface>::IID || iid == &<IX509CertificateRequestPkcs10V2 as windows_core::Interface>::IID || iid == &<IX509CertificateRequestPkcs10V3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRequestPkcs7_Impl: Sized + IX509CertificateRequest_Impl {
    fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromCertificate(&self, context: X509CertificateEnrollmentContext, renewalrequest: super::super::super::Foundation::VARIANT_BOOL, strcertificate: &windows_core::BSTR, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> windows_core::Result<()>;
    fn InitializeFromInnerRequest(&self, pinnerrequest: Option<&IX509CertificateRequest>) -> windows_core::Result<()>;
    fn InitializeDecode(&self, strencodeddata: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn RequesterName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRequesterName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SignerCertificate(&self) -> windows_core::Result<ISignerCertificate>;
    fn SetSignerCertificate(&self, pvalue: Option<&ISignerCertificate>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRequestPkcs7 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRequestPkcs7_Vtbl {
    pub const fn new<Identity: IX509CertificateRequestPkcs7_Impl, const OFFSET: isize>() -> IX509CertificateRequestPkcs7_Vtbl {
        unsafe extern "system" fn InitializeFromTemplateName<Identity: IX509CertificateRequestPkcs7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, strtemplatename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs7_Impl::InitializeFromTemplateName(this, core::mem::transmute_copy(&context), core::mem::transmute(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeFromCertificate<Identity: IX509CertificateRequestPkcs7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, renewalrequest: super::super::super::Foundation::VARIANT_BOOL, strcertificate: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType, inheritoptions: X509RequestInheritOptions) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs7_Impl::InitializeFromCertificate(this, core::mem::transmute_copy(&context), core::mem::transmute_copy(&renewalrequest), core::mem::transmute(&strcertificate), core::mem::transmute_copy(&encoding), core::mem::transmute_copy(&inheritoptions)).into()
        }
        unsafe extern "system" fn InitializeFromInnerRequest<Identity: IX509CertificateRequestPkcs7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinnerrequest: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs7_Impl::InitializeFromInnerRequest(this, windows_core::from_raw_borrowed(&pinnerrequest)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509CertificateRequestPkcs7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs7_Impl::InitializeDecode(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn RequesterName<Identity: IX509CertificateRequestPkcs7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs7_Impl::RequesterName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequesterName<Identity: IX509CertificateRequestPkcs7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs7_Impl::SetRequesterName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SignerCertificate<Identity: IX509CertificateRequestPkcs7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs7_Impl::SignerCertificate(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignerCertificate<Identity: IX509CertificateRequestPkcs7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs7_Impl::SetSignerCertificate(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        Self {
            base__: IX509CertificateRequest_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromTemplateName: InitializeFromTemplateName::<Identity, OFFSET>,
            InitializeFromCertificate: InitializeFromCertificate::<Identity, OFFSET>,
            InitializeFromInnerRequest: InitializeFromInnerRequest::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            RequesterName: RequesterName::<Identity, OFFSET>,
            SetRequesterName: SetRequesterName::<Identity, OFFSET>,
            SignerCertificate: SignerCertificate::<Identity, OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs7 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509CertificateRequest as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRequestPkcs7V2_Impl: Sized + IX509CertificateRequestPkcs7_Impl {
    fn InitializeFromTemplate(&self, context: X509CertificateEnrollmentContext, ppolicyserver: Option<&IX509EnrollmentPolicyServer>, ptemplate: Option<&IX509CertificateTemplate>) -> windows_core::Result<()>;
    fn PolicyServer(&self) -> windows_core::Result<IX509EnrollmentPolicyServer>;
    fn Template(&self) -> windows_core::Result<IX509CertificateTemplate>;
    fn CheckCertificateSignature(&self, validatecertificatechain: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRequestPkcs7V2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRequestPkcs7V2_Vtbl {
    pub const fn new<Identity: IX509CertificateRequestPkcs7V2_Impl, const OFFSET: isize>() -> IX509CertificateRequestPkcs7V2_Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Identity: IX509CertificateRequestPkcs7V2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs7V2_Impl::InitializeFromTemplate(this, core::mem::transmute_copy(&context), windows_core::from_raw_borrowed(&ppolicyserver), windows_core::from_raw_borrowed(&ptemplate)).into()
        }
        unsafe extern "system" fn PolicyServer<Identity: IX509CertificateRequestPkcs7V2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppolicyserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs7V2_Impl::PolicyServer(this) {
                Ok(ok__) => {
                    pppolicyserver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Template<Identity: IX509CertificateRequestPkcs7V2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRequestPkcs7V2_Impl::Template(this) {
                Ok(ok__) => {
                    pptemplate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckCertificateSignature<Identity: IX509CertificateRequestPkcs7V2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, validatecertificatechain: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRequestPkcs7V2_Impl::CheckCertificateSignature(this, core::mem::transmute_copy(&validatecertificatechain)).into()
        }
        Self {
            base__: IX509CertificateRequestPkcs7_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromTemplate: InitializeFromTemplate::<Identity, OFFSET>,
            PolicyServer: PolicyServer::<Identity, OFFSET>,
            Template: Template::<Identity, OFFSET>,
            CheckCertificateSignature: CheckCertificateSignature::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRequestPkcs7V2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509CertificateRequest as windows_core::Interface>::IID || iid == &<IX509CertificateRequestPkcs7 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRevocationList_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self) -> windows_core::Result<()>;
    fn InitializeDecode(&self, strencodeddata: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn Encode(&self) -> windows_core::Result<()>;
    fn ResetForEncode(&self) -> windows_core::Result<()>;
    fn CheckPublicKeySignature(&self, ppublickey: Option<&IX509PublicKey>) -> windows_core::Result<()>;
    fn CheckSignature(&self) -> windows_core::Result<()>;
    fn Issuer(&self) -> windows_core::Result<IX500DistinguishedName>;
    fn SetIssuer(&self, pvalue: Option<&IX500DistinguishedName>) -> windows_core::Result<()>;
    fn ThisUpdate(&self) -> windows_core::Result<f64>;
    fn SetThisUpdate(&self, value: f64) -> windows_core::Result<()>;
    fn NextUpdate(&self) -> windows_core::Result<f64>;
    fn SetNextUpdate(&self, value: f64) -> windows_core::Result<()>;
    fn X509CRLEntries(&self) -> windows_core::Result<IX509CertificateRevocationListEntries>;
    fn X509Extensions(&self) -> windows_core::Result<IX509Extensions>;
    fn CriticalExtensions(&self) -> windows_core::Result<IObjectIds>;
    fn SignerCertificate(&self) -> windows_core::Result<ISignerCertificate>;
    fn SetSignerCertificate(&self, pvalue: Option<&ISignerCertificate>) -> windows_core::Result<()>;
    fn get_CRLNumber(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn put_CRLNumber(&self, encoding: EncodingType, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CAVersion(&self) -> windows_core::Result<i32>;
    fn SetCAVersion(&self, pvalue: i32) -> windows_core::Result<()>;
    fn BaseCRL(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn NullSigned(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn HashAlgorithm(&self) -> windows_core::Result<IObjectId>;
    fn SetHashAlgorithm(&self, pvalue: Option<&IObjectId>) -> windows_core::Result<()>;
    fn AlternateSignatureAlgorithm(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetAlternateSignatureAlgorithm(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SignatureInformation(&self) -> windows_core::Result<IX509SignatureInformation>;
    fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn get_RawDataToBeSigned(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn get_Signature(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRevocationList {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRevocationList_Vtbl {
    pub const fn new<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>() -> IX509CertificateRevocationList_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::Initialize(this).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::InitializeDecode(this, core::mem::transmute(&strencodeddata), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn Encode<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::Encode(this).into()
        }
        unsafe extern "system" fn ResetForEncode<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::ResetForEncode(this).into()
        }
        unsafe extern "system" fn CheckPublicKeySignature<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppublickey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::CheckPublicKeySignature(this, windows_core::from_raw_borrowed(&ppublickey)).into()
        }
        unsafe extern "system" fn CheckSignature<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::CheckSignature(this).into()
        }
        unsafe extern "system" fn Issuer<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::Issuer(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIssuer<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::SetIssuer(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn ThisUpdate<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::ThisUpdate(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetThisUpdate<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::SetThisUpdate(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn NextUpdate<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::NextUpdate(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNextUpdate<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::SetNextUpdate(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn X509CRLEntries<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::X509CRLEntries(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X509Extensions<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::X509Extensions(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CriticalExtensions<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::CriticalExtensions(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignerCertificate<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::SignerCertificate(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignerCertificate<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::SetSignerCertificate(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn get_CRLNumber<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::get_CRLNumber(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_CRLNumber<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::put_CRLNumber(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CAVersion<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::CAVersion(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCAVersion<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::SetCAVersion(this, core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn BaseCRL<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::BaseCRL(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NullSigned<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::NullSigned(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HashAlgorithm<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::HashAlgorithm(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::SetHashAlgorithm(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithm<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::AlternateSignatureAlgorithm(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlternateSignatureAlgorithm<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationList_Impl::SetAlternateSignatureAlgorithm(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SignatureInformation<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::SignatureInformation(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_RawData<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::get_RawData(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_RawDataToBeSigned<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::get_RawDataToBeSigned(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Signature<Identity: IX509CertificateRevocationList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationList_Impl::get_Signature(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            Encode: Encode::<Identity, OFFSET>,
            ResetForEncode: ResetForEncode::<Identity, OFFSET>,
            CheckPublicKeySignature: CheckPublicKeySignature::<Identity, OFFSET>,
            CheckSignature: CheckSignature::<Identity, OFFSET>,
            Issuer: Issuer::<Identity, OFFSET>,
            SetIssuer: SetIssuer::<Identity, OFFSET>,
            ThisUpdate: ThisUpdate::<Identity, OFFSET>,
            SetThisUpdate: SetThisUpdate::<Identity, OFFSET>,
            NextUpdate: NextUpdate::<Identity, OFFSET>,
            SetNextUpdate: SetNextUpdate::<Identity, OFFSET>,
            X509CRLEntries: X509CRLEntries::<Identity, OFFSET>,
            X509Extensions: X509Extensions::<Identity, OFFSET>,
            CriticalExtensions: CriticalExtensions::<Identity, OFFSET>,
            SignerCertificate: SignerCertificate::<Identity, OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Identity, OFFSET>,
            get_CRLNumber: get_CRLNumber::<Identity, OFFSET>,
            put_CRLNumber: put_CRLNumber::<Identity, OFFSET>,
            CAVersion: CAVersion::<Identity, OFFSET>,
            SetCAVersion: SetCAVersion::<Identity, OFFSET>,
            BaseCRL: BaseCRL::<Identity, OFFSET>,
            NullSigned: NullSigned::<Identity, OFFSET>,
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            AlternateSignatureAlgorithm: AlternateSignatureAlgorithm::<Identity, OFFSET>,
            SetAlternateSignatureAlgorithm: SetAlternateSignatureAlgorithm::<Identity, OFFSET>,
            SignatureInformation: SignatureInformation::<Identity, OFFSET>,
            get_RawData: get_RawData::<Identity, OFFSET>,
            get_RawDataToBeSigned: get_RawDataToBeSigned::<Identity, OFFSET>,
            get_Signature: get_Signature::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRevocationList as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRevocationListEntries_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509CertificateRevocationListEntry>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&IX509CertificateRevocationListEntry>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn get_IndexBySerialNumber(&self, encoding: EncodingType, serialnumber: &windows_core::BSTR) -> windows_core::Result<i32>;
    fn AddRange(&self, pvalue: Option<&IX509CertificateRevocationListEntries>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRevocationListEntries {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRevocationListEntries_Vtbl {
    pub const fn new<Identity: IX509CertificateRevocationListEntries_Impl, const OFFSET: isize>() -> IX509CertificateRevocationListEntries_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: IX509CertificateRevocationListEntries_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationListEntries_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IX509CertificateRevocationListEntries_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationListEntries_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IX509CertificateRevocationListEntries_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationListEntries_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IX509CertificateRevocationListEntries_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationListEntries_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: IX509CertificateRevocationListEntries_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationListEntries_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: IX509CertificateRevocationListEntries_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationListEntries_Impl::Clear(this).into()
        }
        unsafe extern "system" fn get_IndexBySerialNumber<Identity: IX509CertificateRevocationListEntries_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, serialnumber: core::mem::MaybeUninit<windows_core::BSTR>, pindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationListEntries_Impl::get_IndexBySerialNumber(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&serialnumber)) {
                Ok(ok__) => {
                    pindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRange<Identity: IX509CertificateRevocationListEntries_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationListEntries_Impl::AddRange(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            get_IndexBySerialNumber: get_IndexBySerialNumber::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRevocationListEntries as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateRevocationListEntry_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, encoding: EncodingType, serialnumber: &windows_core::BSTR, revocationdate: f64) -> windows_core::Result<()>;
    fn get_SerialNumber(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn RevocationDate(&self) -> windows_core::Result<f64>;
    fn RevocationReason(&self) -> windows_core::Result<CRLRevocationReason>;
    fn SetRevocationReason(&self, value: CRLRevocationReason) -> windows_core::Result<()>;
    fn X509Extensions(&self) -> windows_core::Result<IX509Extensions>;
    fn CriticalExtensions(&self) -> windows_core::Result<IObjectIds>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateRevocationListEntry {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateRevocationListEntry_Vtbl {
    pub const fn new<Identity: IX509CertificateRevocationListEntry_Impl, const OFFSET: isize>() -> IX509CertificateRevocationListEntry_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509CertificateRevocationListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, serialnumber: core::mem::MaybeUninit<windows_core::BSTR>, revocationdate: f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationListEntry_Impl::Initialize(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&serialnumber), core::mem::transmute_copy(&revocationdate)).into()
        }
        unsafe extern "system" fn get_SerialNumber<Identity: IX509CertificateRevocationListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationListEntry_Impl::get_SerialNumber(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevocationDate<Identity: IX509CertificateRevocationListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationListEntry_Impl::RevocationDate(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevocationReason<Identity: IX509CertificateRevocationListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut CRLRevocationReason) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationListEntry_Impl::RevocationReason(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRevocationReason<Identity: IX509CertificateRevocationListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: CRLRevocationReason) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateRevocationListEntry_Impl::SetRevocationReason(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn X509Extensions<Identity: IX509CertificateRevocationListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationListEntry_Impl::X509Extensions(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CriticalExtensions<Identity: IX509CertificateRevocationListEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateRevocationListEntry_Impl::CriticalExtensions(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            get_SerialNumber: get_SerialNumber::<Identity, OFFSET>,
            RevocationDate: RevocationDate::<Identity, OFFSET>,
            RevocationReason: RevocationReason::<Identity, OFFSET>,
            SetRevocationReason: SetRevocationReason::<Identity, OFFSET>,
            X509Extensions: X509Extensions::<Identity, OFFSET>,
            CriticalExtensions: CriticalExtensions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateRevocationListEntry as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateTemplate_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_Property(&self, property: EnrollmentTemplateProperty) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateTemplate {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateTemplate_Vtbl {
    pub const fn new<Identity: IX509CertificateTemplate_Impl, const OFFSET: isize>() -> IX509CertificateTemplate_Vtbl {
        unsafe extern "system" fn get_Property<Identity: IX509CertificateTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: EnrollmentTemplateProperty, pvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateTemplate_Impl::get_Property(this, core::mem::transmute_copy(&property)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_Property: get_Property::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateTemplate as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateTemplateWritable_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, pvalue: Option<&IX509CertificateTemplate>) -> windows_core::Result<()>;
    fn Commit(&self, commitflags: CommitTemplateFlags, strservercontext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Property(&self, property: EnrollmentTemplateProperty) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn put_Property(&self, property: EnrollmentTemplateProperty, value: &super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Template(&self) -> windows_core::Result<IX509CertificateTemplate>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateTemplateWritable {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateTemplateWritable_Vtbl {
    pub const fn new<Identity: IX509CertificateTemplateWritable_Impl, const OFFSET: isize>() -> IX509CertificateTemplateWritable_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509CertificateTemplateWritable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateTemplateWritable_Impl::Initialize(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn Commit<Identity: IX509CertificateTemplateWritable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, commitflags: CommitTemplateFlags, strservercontext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateTemplateWritable_Impl::Commit(this, core::mem::transmute_copy(&commitflags), core::mem::transmute(&strservercontext)).into()
        }
        unsafe extern "system" fn get_Property<Identity: IX509CertificateTemplateWritable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: EnrollmentTemplateProperty, pvalue: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateTemplateWritable_Impl::get_Property(this, core::mem::transmute_copy(&property)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Property<Identity: IX509CertificateTemplateWritable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: EnrollmentTemplateProperty, value: core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateTemplateWritable_Impl::put_Property(this, core::mem::transmute_copy(&property), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Template<Identity: IX509CertificateTemplateWritable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateTemplateWritable_Impl::Template(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            get_Property: get_Property::<Identity, OFFSET>,
            put_Property: put_Property::<Identity, OFFSET>,
            Template: Template::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateTemplateWritable as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509CertificateTemplates_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509CertificateTemplate>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&IX509CertificateTemplate>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn get_ItemByName(&self, bstrname: &windows_core::BSTR) -> windows_core::Result<IX509CertificateTemplate>;
    fn get_ItemByOid(&self, poid: Option<&IObjectId>) -> windows_core::Result<IX509CertificateTemplate>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509CertificateTemplates {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509CertificateTemplates_Vtbl {
    pub const fn new<Identity: IX509CertificateTemplates_Impl, const OFFSET: isize>() -> IX509CertificateTemplates_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: IX509CertificateTemplates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateTemplates_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IX509CertificateTemplates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateTemplates_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IX509CertificateTemplates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateTemplates_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IX509CertificateTemplates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateTemplates_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: IX509CertificateTemplates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateTemplates_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: IX509CertificateTemplates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509CertificateTemplates_Impl::Clear(this).into()
        }
        unsafe extern "system" fn get_ItemByName<Identity: IX509CertificateTemplates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: core::mem::MaybeUninit<windows_core::BSTR>, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateTemplates_Impl::get_ItemByName(this, core::mem::transmute(&bstrname)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_ItemByOid<Identity: IX509CertificateTemplates_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poid: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509CertificateTemplates_Impl::get_ItemByOid(this, windows_core::from_raw_borrowed(&poid)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            get_ItemByName: get_ItemByName::<Identity, OFFSET>,
            get_ItemByOid: get_ItemByOid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509CertificateTemplates as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509EndorsementKey_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProviderName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn Opened(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn AddCertificate(&self, encoding: EncodingType, strcertificate: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveCertificate(&self, encoding: EncodingType, strcertificate: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCertificateByIndex(&self, manufactureronly: super::super::super::Foundation::VARIANT_BOOL, dwindex: i32, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn GetCertificateCount(&self, manufactureronly: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<i32>;
    fn ExportPublicKey(&self) -> windows_core::Result<IX509PublicKey>;
    fn Open(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509EndorsementKey {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509EndorsementKey_Vtbl {
    pub const fn new<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>() -> IX509EndorsementKey_Vtbl {
        unsafe extern "system" fn ProviderName<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EndorsementKey_Impl::ProviderName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderName<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EndorsementKey_Impl::SetProviderName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Length<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EndorsementKey_Impl::Length(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Opened<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EndorsementKey_Impl::Opened(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddCertificate<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strcertificate: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EndorsementKey_Impl::AddCertificate(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strcertificate)).into()
        }
        unsafe extern "system" fn RemoveCertificate<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strcertificate: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EndorsementKey_Impl::RemoveCertificate(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strcertificate)).into()
        }
        unsafe extern "system" fn GetCertificateByIndex<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manufactureronly: super::super::super::Foundation::VARIANT_BOOL, dwindex: i32, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EndorsementKey_Impl::GetCertificateByIndex(this, core::mem::transmute_copy(&manufactureronly), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCertificateCount<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, manufactureronly: super::super::super::Foundation::VARIANT_BOOL, pcount: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EndorsementKey_Impl::GetCertificateCount(this, core::mem::transmute_copy(&manufactureronly)) {
                Ok(ok__) => {
                    pcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPublicKey<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppublickey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EndorsementKey_Impl::ExportPublicKey(this) {
                Ok(ok__) => {
                    pppublickey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EndorsementKey_Impl::Open(this).into()
        }
        unsafe extern "system" fn Close<Identity: IX509EndorsementKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EndorsementKey_Impl::Close(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            ProviderName: ProviderName::<Identity, OFFSET>,
            SetProviderName: SetProviderName::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            Opened: Opened::<Identity, OFFSET>,
            AddCertificate: AddCertificate::<Identity, OFFSET>,
            RemoveCertificate: RemoveCertificate::<Identity, OFFSET>,
            GetCertificateByIndex: GetCertificateByIndex::<Identity, OFFSET>,
            GetCertificateCount: GetCertificateCount::<Identity, OFFSET>,
            ExportPublicKey: ExportPublicKey::<Identity, OFFSET>,
            Open: Open::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509EndorsementKey as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509Enrollment_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()>;
    fn InitializeFromTemplateName(&self, context: X509CertificateEnrollmentContext, strtemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeFromRequest(&self, prequest: Option<&IX509CertificateRequest>) -> windows_core::Result<()>;
    fn CreateRequest(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn Enroll(&self) -> windows_core::Result<()>;
    fn InstallResponse(&self, restrictions: InstallResponseRestrictionFlags, strresponse: &windows_core::BSTR, encoding: EncodingType, strpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CreatePFX(&self, strpassword: &windows_core::BSTR, exportoptions: PFXExportOptions, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn Request(&self) -> windows_core::Result<IX509CertificateRequest>;
    fn Silent(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetSilent(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ParentWindow(&self) -> windows_core::Result<i32>;
    fn SetParentWindow(&self, value: i32) -> windows_core::Result<()>;
    fn NameValuePairs(&self) -> windows_core::Result<IX509NameValuePairs>;
    fn EnrollmentContext(&self) -> windows_core::Result<X509CertificateEnrollmentContext>;
    fn Status(&self) -> windows_core::Result<IX509EnrollmentStatus>;
    fn get_Certificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn get_Response(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn CertificateFriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCertificateFriendlyName(&self, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CertificateDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCertificateDescription(&self, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RequestId(&self) -> windows_core::Result<i32>;
    fn CAConfigString(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509Enrollment {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509Enrollment_Vtbl {
    pub const fn new<Identity: IX509Enrollment_Impl, const OFFSET: isize>() -> IX509Enrollment_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Enrollment_Impl::Initialize(this, core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn InitializeFromTemplateName<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, strtemplatename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Enrollment_Impl::InitializeFromTemplateName(this, core::mem::transmute_copy(&context), core::mem::transmute(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeFromRequest<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Enrollment_Impl::InitializeFromRequest(this, windows_core::from_raw_borrowed(&prequest)).into()
        }
        unsafe extern "system" fn CreateRequest<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::CreateRequest(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enroll<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Enrollment_Impl::Enroll(this).into()
        }
        unsafe extern "system" fn InstallResponse<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictions: InstallResponseRestrictionFlags, strresponse: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType, strpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Enrollment_Impl::InstallResponse(this, core::mem::transmute_copy(&restrictions), core::mem::transmute(&strresponse), core::mem::transmute_copy(&encoding), core::mem::transmute(&strpassword)).into()
        }
        unsafe extern "system" fn CreatePFX<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, exportoptions: PFXExportOptions, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::CreatePFX(this, core::mem::transmute(&strpassword), core::mem::transmute_copy(&exportoptions), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Request<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::Request(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Silent<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::Silent(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSilent<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Enrollment_Impl::SetSilent(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::ParentWindow(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentWindow<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Enrollment_Impl::SetParentWindow(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn NameValuePairs<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::NameValuePairs(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnrollmentContext<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509CertificateEnrollmentContext) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::EnrollmentContext(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Status<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::Status(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Certificate<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::get_Certificate(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Response<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::get_Response(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CertificateFriendlyName<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::CertificateFriendlyName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateFriendlyName<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Enrollment_Impl::SetCertificateFriendlyName(this, core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn CertificateDescription<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::CertificateDescription(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateDescription<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Enrollment_Impl::SetCertificateDescription(this, core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn RequestId<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::RequestId(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CAConfigString<Identity: IX509Enrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment_Impl::CAConfigString(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            InitializeFromTemplateName: InitializeFromTemplateName::<Identity, OFFSET>,
            InitializeFromRequest: InitializeFromRequest::<Identity, OFFSET>,
            CreateRequest: CreateRequest::<Identity, OFFSET>,
            Enroll: Enroll::<Identity, OFFSET>,
            InstallResponse: InstallResponse::<Identity, OFFSET>,
            CreatePFX: CreatePFX::<Identity, OFFSET>,
            Request: Request::<Identity, OFFSET>,
            Silent: Silent::<Identity, OFFSET>,
            SetSilent: SetSilent::<Identity, OFFSET>,
            ParentWindow: ParentWindow::<Identity, OFFSET>,
            SetParentWindow: SetParentWindow::<Identity, OFFSET>,
            NameValuePairs: NameValuePairs::<Identity, OFFSET>,
            EnrollmentContext: EnrollmentContext::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            get_Certificate: get_Certificate::<Identity, OFFSET>,
            get_Response: get_Response::<Identity, OFFSET>,
            CertificateFriendlyName: CertificateFriendlyName::<Identity, OFFSET>,
            SetCertificateFriendlyName: SetCertificateFriendlyName::<Identity, OFFSET>,
            CertificateDescription: CertificateDescription::<Identity, OFFSET>,
            SetCertificateDescription: SetCertificateDescription::<Identity, OFFSET>,
            RequestId: RequestId::<Identity, OFFSET>,
            CAConfigString: CAConfigString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509Enrollment as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509Enrollment2_Impl: Sized + IX509Enrollment_Impl {
    fn InitializeFromTemplate(&self, context: X509CertificateEnrollmentContext, ppolicyserver: Option<&IX509EnrollmentPolicyServer>, ptemplate: Option<&IX509CertificateTemplate>) -> windows_core::Result<()>;
    fn InstallResponse2(&self, restrictions: InstallResponseRestrictionFlags, strresponse: &windows_core::BSTR, encoding: EncodingType, strpassword: &windows_core::BSTR, strenrollmentpolicyserverurl: &windows_core::BSTR, strenrollmentpolicyserverid: &windows_core::BSTR, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags) -> windows_core::Result<()>;
    fn PolicyServer(&self) -> windows_core::Result<IX509EnrollmentPolicyServer>;
    fn Template(&self) -> windows_core::Result<IX509CertificateTemplate>;
    fn RequestIdString(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509Enrollment2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509Enrollment2_Vtbl {
    pub const fn new<Identity: IX509Enrollment2_Impl, const OFFSET: isize>() -> IX509Enrollment2_Vtbl {
        unsafe extern "system" fn InitializeFromTemplate<Identity: IX509Enrollment2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, ppolicyserver: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Enrollment2_Impl::InitializeFromTemplate(this, core::mem::transmute_copy(&context), windows_core::from_raw_borrowed(&ppolicyserver), windows_core::from_raw_borrowed(&ptemplate)).into()
        }
        unsafe extern "system" fn InstallResponse2<Identity: IX509Enrollment2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictions: InstallResponseRestrictionFlags, strresponse: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType, strpassword: core::mem::MaybeUninit<windows_core::BSTR>, strenrollmentpolicyserverurl: core::mem::MaybeUninit<windows_core::BSTR>, strenrollmentpolicyserverid: core::mem::MaybeUninit<windows_core::BSTR>, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Enrollment2_Impl::InstallResponse2(this, core::mem::transmute_copy(&restrictions), core::mem::transmute(&strresponse), core::mem::transmute_copy(&encoding), core::mem::transmute(&strpassword), core::mem::transmute(&strenrollmentpolicyserverurl), core::mem::transmute(&strenrollmentpolicyserverid), core::mem::transmute_copy(&enrollmentpolicyserverflags), core::mem::transmute_copy(&authflags)).into()
        }
        unsafe extern "system" fn PolicyServer<Identity: IX509Enrollment2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppolicyserver: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment2_Impl::PolicyServer(this) {
                Ok(ok__) => {
                    pppolicyserver.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Template<Identity: IX509Enrollment2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptemplate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment2_Impl::Template(this) {
                Ok(ok__) => {
                    pptemplate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestIdString<Identity: IX509Enrollment2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Enrollment2_Impl::RequestIdString(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Enrollment_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromTemplate: InitializeFromTemplate::<Identity, OFFSET>,
            InstallResponse2: InstallResponse2::<Identity, OFFSET>,
            PolicyServer: PolicyServer::<Identity, OFFSET>,
            Template: Template::<Identity, OFFSET>,
            RequestIdString: RequestIdString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509Enrollment2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Enrollment as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509EnrollmentHelper_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn AddPolicyServer(&self, strenrollmentpolicyserveruri: &windows_core::BSTR, strenrollmentpolicyid: &windows_core::BSTR, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags, strcredential: &windows_core::BSTR, strpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AddEnrollmentServer(&self, strenrollmentserveruri: &windows_core::BSTR, authflags: X509EnrollmentAuthFlags, strcredential: &windows_core::BSTR, strpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enroll(&self, strenrollmentpolicyserveruri: &windows_core::BSTR, strtemplatename: &windows_core::BSTR, encoding: EncodingType, enrollflags: WebEnrollmentFlags) -> windows_core::Result<windows_core::BSTR>;
    fn Initialize(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509EnrollmentHelper {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509EnrollmentHelper_Vtbl {
    pub const fn new<Identity: IX509EnrollmentHelper_Impl, const OFFSET: isize>() -> IX509EnrollmentHelper_Vtbl {
        unsafe extern "system" fn AddPolicyServer<Identity: IX509EnrollmentHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strenrollmentpolicyserveruri: core::mem::MaybeUninit<windows_core::BSTR>, strenrollmentpolicyid: core::mem::MaybeUninit<windows_core::BSTR>, enrollmentpolicyserverflags: PolicyServerUrlFlags, authflags: X509EnrollmentAuthFlags, strcredential: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentHelper_Impl::AddPolicyServer(this, core::mem::transmute(&strenrollmentpolicyserveruri), core::mem::transmute(&strenrollmentpolicyid), core::mem::transmute_copy(&enrollmentpolicyserverflags), core::mem::transmute_copy(&authflags), core::mem::transmute(&strcredential), core::mem::transmute(&strpassword)).into()
        }
        unsafe extern "system" fn AddEnrollmentServer<Identity: IX509EnrollmentHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strenrollmentserveruri: core::mem::MaybeUninit<windows_core::BSTR>, authflags: X509EnrollmentAuthFlags, strcredential: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentHelper_Impl::AddEnrollmentServer(this, core::mem::transmute(&strenrollmentserveruri), core::mem::transmute_copy(&authflags), core::mem::transmute(&strcredential), core::mem::transmute(&strpassword)).into()
        }
        unsafe extern "system" fn Enroll<Identity: IX509EnrollmentHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strenrollmentpolicyserveruri: core::mem::MaybeUninit<windows_core::BSTR>, strtemplatename: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType, enrollflags: WebEnrollmentFlags, pstrcertificate: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentHelper_Impl::Enroll(this, core::mem::transmute(&strenrollmentpolicyserveruri), core::mem::transmute(&strtemplatename), core::mem::transmute_copy(&encoding), core::mem::transmute_copy(&enrollflags)) {
                Ok(ok__) => {
                    pstrcertificate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: IX509EnrollmentHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentHelper_Impl::Initialize(this, core::mem::transmute_copy(&context)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AddPolicyServer: AddPolicyServer::<Identity, OFFSET>,
            AddEnrollmentServer: AddEnrollmentServer::<Identity, OFFSET>,
            Enroll: Enroll::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509EnrollmentHelper as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509EnrollmentPolicyServer_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, bstrpolicyserverurl: &windows_core::BSTR, bstrpolicyserverid: &windows_core::BSTR, authflags: X509EnrollmentAuthFlags, fisuntrusted: super::super::super::Foundation::VARIANT_BOOL, context: X509CertificateEnrollmentContext) -> windows_core::Result<()>;
    fn LoadPolicy(&self, option: X509EnrollmentPolicyLoadOption) -> windows_core::Result<()>;
    fn GetTemplates(&self) -> windows_core::Result<IX509CertificateTemplates>;
    fn GetCAsForTemplate(&self, ptemplate: Option<&IX509CertificateTemplate>) -> windows_core::Result<ICertificationAuthorities>;
    fn GetCAs(&self) -> windows_core::Result<ICertificationAuthorities>;
    fn Validate(&self) -> windows_core::Result<()>;
    fn GetCustomOids(&self) -> windows_core::Result<IObjectIds>;
    fn GetNextUpdateTime(&self) -> windows_core::Result<f64>;
    fn GetLastUpdateTime(&self) -> windows_core::Result<f64>;
    fn GetPolicyServerUrl(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPolicyServerId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetIsDefaultCEP(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn GetUseClientId(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn GetAllowUnTrustedCA(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn GetCachePath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCacheDir(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAuthFlags(&self) -> windows_core::Result<X509EnrollmentAuthFlags>;
    fn SetCredential(&self, hwndparent: i32, flag: X509EnrollmentAuthFlags, strcredential: &windows_core::BSTR, strpassword: &windows_core::BSTR) -> windows_core::Result<()>;
    fn QueryChanges(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn InitializeImport(&self, val: &super::super::super::System::Variant::VARIANT) -> windows_core::Result<()>;
    fn Export(&self, exportflags: X509EnrollmentPolicyExportFlags) -> windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn Cost(&self) -> windows_core::Result<u32>;
    fn SetCost(&self, value: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509EnrollmentPolicyServer {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509EnrollmentPolicyServer_Vtbl {
    pub const fn new<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>() -> IX509EnrollmentPolicyServer_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrpolicyserverurl: core::mem::MaybeUninit<windows_core::BSTR>, bstrpolicyserverid: core::mem::MaybeUninit<windows_core::BSTR>, authflags: X509EnrollmentAuthFlags, fisuntrusted: super::super::super::Foundation::VARIANT_BOOL, context: X509CertificateEnrollmentContext) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentPolicyServer_Impl::Initialize(this, core::mem::transmute(&bstrpolicyserverurl), core::mem::transmute(&bstrpolicyserverid), core::mem::transmute_copy(&authflags), core::mem::transmute_copy(&fisuntrusted), core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn LoadPolicy<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, option: X509EnrollmentPolicyLoadOption) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentPolicyServer_Impl::LoadPolicy(this, core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn GetTemplates<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptemplates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetTemplates(this) {
                Ok(ok__) => {
                    ptemplates.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAsForTemplate<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptemplate: *mut core::ffi::c_void, ppcas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetCAsForTemplate(this, windows_core::from_raw_borrowed(&ptemplate)) {
                Ok(ok__) => {
                    ppcas.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCAs<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetCAs(this) {
                Ok(ok__) => {
                    ppcas.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentPolicyServer_Impl::Validate(this).into()
        }
        unsafe extern "system" fn GetCustomOids<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobjectids: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetCustomOids(this) {
                Ok(ok__) => {
                    ppobjectids.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextUpdateTime<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetNextUpdateTime(this) {
                Ok(ok__) => {
                    pdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastUpdateTime<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdate: *mut f64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetLastUpdateTime(this) {
                Ok(ok__) => {
                    pdate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicyServerUrl<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetPolicyServerUrl(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPolicyServerId<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetPolicyServerId(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFriendlyName<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetFriendlyName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsDefaultCEP<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetIsDefaultCEP(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUseClientId<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetUseClientId(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllowUnTrustedCA<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetAllowUnTrustedCA(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCachePath<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetCachePath(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCacheDir<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetCacheDir(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthFlags<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::GetAuthFlags(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredential<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndparent: i32, flag: X509EnrollmentAuthFlags, strcredential: core::mem::MaybeUninit<windows_core::BSTR>, strpassword: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentPolicyServer_Impl::SetCredential(this, core::mem::transmute_copy(&hwndparent), core::mem::transmute_copy(&flag), core::mem::transmute(&strcredential), core::mem::transmute(&strpassword)).into()
        }
        unsafe extern "system" fn QueryChanges<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::QueryChanges(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeImport<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentPolicyServer_Impl::InitializeImport(this, core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn Export<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, exportflags: X509EnrollmentPolicyExportFlags, pval: *mut core::mem::MaybeUninit<super::super::super::System::Variant::VARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::Export(this, core::mem::transmute_copy(&exportflags)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cost<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentPolicyServer_Impl::Cost(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCost<Identity: IX509EnrollmentPolicyServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentPolicyServer_Impl::SetCost(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            LoadPolicy: LoadPolicy::<Identity, OFFSET>,
            GetTemplates: GetTemplates::<Identity, OFFSET>,
            GetCAsForTemplate: GetCAsForTemplate::<Identity, OFFSET>,
            GetCAs: GetCAs::<Identity, OFFSET>,
            Validate: Validate::<Identity, OFFSET>,
            GetCustomOids: GetCustomOids::<Identity, OFFSET>,
            GetNextUpdateTime: GetNextUpdateTime::<Identity, OFFSET>,
            GetLastUpdateTime: GetLastUpdateTime::<Identity, OFFSET>,
            GetPolicyServerUrl: GetPolicyServerUrl::<Identity, OFFSET>,
            GetPolicyServerId: GetPolicyServerId::<Identity, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, OFFSET>,
            GetIsDefaultCEP: GetIsDefaultCEP::<Identity, OFFSET>,
            GetUseClientId: GetUseClientId::<Identity, OFFSET>,
            GetAllowUnTrustedCA: GetAllowUnTrustedCA::<Identity, OFFSET>,
            GetCachePath: GetCachePath::<Identity, OFFSET>,
            GetCacheDir: GetCacheDir::<Identity, OFFSET>,
            GetAuthFlags: GetAuthFlags::<Identity, OFFSET>,
            SetCredential: SetCredential::<Identity, OFFSET>,
            QueryChanges: QueryChanges::<Identity, OFFSET>,
            InitializeImport: InitializeImport::<Identity, OFFSET>,
            Export: Export::<Identity, OFFSET>,
            Cost: Cost::<Identity, OFFSET>,
            SetCost: SetCost::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509EnrollmentPolicyServer as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509EnrollmentStatus_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn AppendText(&self, strtext: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Text(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetText(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Selected(&self) -> windows_core::Result<EnrollmentSelectionStatus>;
    fn SetSelected(&self, value: EnrollmentSelectionStatus) -> windows_core::Result<()>;
    fn Display(&self) -> windows_core::Result<EnrollmentDisplayStatus>;
    fn SetDisplay(&self, value: EnrollmentDisplayStatus) -> windows_core::Result<()>;
    fn Status(&self) -> windows_core::Result<EnrollmentEnrollStatus>;
    fn SetStatus(&self, value: EnrollmentEnrollStatus) -> windows_core::Result<()>;
    fn Error(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn SetError(&self, value: windows_core::HRESULT) -> windows_core::Result<()>;
    fn ErrorText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509EnrollmentStatus {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509EnrollmentStatus_Vtbl {
    pub const fn new<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>() -> IX509EnrollmentStatus_Vtbl {
        unsafe extern "system" fn AppendText<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strtext: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentStatus_Impl::AppendText(this, core::mem::transmute(&strtext)).into()
        }
        unsafe extern "system" fn Text<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentStatus_Impl::Text(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetText<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentStatus_Impl::SetText(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Selected<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut EnrollmentSelectionStatus) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentStatus_Impl::Selected(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelected<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: EnrollmentSelectionStatus) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentStatus_Impl::SetSelected(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Display<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut EnrollmentDisplayStatus) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentStatus_Impl::Display(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplay<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: EnrollmentDisplayStatus) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentStatus_Impl::SetDisplay(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Status<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut EnrollmentEnrollStatus) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentStatus_Impl::Status(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStatus<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: EnrollmentEnrollStatus) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentStatus_Impl::SetStatus(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Error<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentStatus_Impl::Error(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetError<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::HRESULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509EnrollmentStatus_Impl::SetError(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ErrorText<Identity: IX509EnrollmentStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentStatus_Impl::ErrorText(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            AppendText: AppendText::<Identity, OFFSET>,
            Text: Text::<Identity, OFFSET>,
            SetText: SetText::<Identity, OFFSET>,
            Selected: Selected::<Identity, OFFSET>,
            SetSelected: SetSelected::<Identity, OFFSET>,
            Display: Display::<Identity, OFFSET>,
            SetDisplay: SetDisplay::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            SetStatus: SetStatus::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
            SetError: SetError::<Identity, OFFSET>,
            ErrorText: ErrorText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509EnrollmentStatus as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509EnrollmentWebClassFactory_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn CreateObject(&self, strprogid: &windows_core::BSTR) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509EnrollmentWebClassFactory {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509EnrollmentWebClassFactory_Vtbl {
    pub const fn new<Identity: IX509EnrollmentWebClassFactory_Impl, const OFFSET: isize>() -> IX509EnrollmentWebClassFactory_Vtbl {
        unsafe extern "system" fn CreateObject<Identity: IX509EnrollmentWebClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprogid: core::mem::MaybeUninit<windows_core::BSTR>, ppiunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509EnrollmentWebClassFactory_Impl::CreateObject(this, core::mem::transmute(&strprogid)) {
                Ok(ok__) => {
                    ppiunknown.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), CreateObject: CreateObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509EnrollmentWebClassFactory as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509Extension_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, pobjectid: Option<&IObjectId>, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ObjectId(&self) -> windows_core::Result<IObjectId>;
    fn get_RawData(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn Critical(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetCritical(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509Extension {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509Extension_Vtbl {
    pub const fn new<Identity: IX509Extension_Impl, const OFFSET: isize>() -> IX509Extension_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectid: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Extension_Impl::Initialize(this, windows_core::from_raw_borrowed(&pobjectid), core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn ObjectId<Identity: IX509Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Extension_Impl::ObjectId(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_RawData<Identity: IX509Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Extension_Impl::get_RawData(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Critical<Identity: IX509Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Extension_Impl::Critical(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCritical<Identity: IX509Extension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Extension_Impl::SetCritical(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            ObjectId: ObjectId::<Identity, OFFSET>,
            get_RawData: get_RawData::<Identity, OFFSET>,
            Critical: Critical::<Identity, OFFSET>,
            SetCritical: SetCritical::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509Extension as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509ExtensionAlternativeNames_Impl: Sized + IX509Extension_Impl {
    fn InitializeEncode(&self, pvalue: Option<&IAlternativeNames>) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AlternativeNames(&self) -> windows_core::Result<IAlternativeNames>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509ExtensionAlternativeNames {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509ExtensionAlternativeNames_Vtbl {
    pub const fn new<Identity: IX509ExtensionAlternativeNames_Impl, const OFFSET: isize>() -> IX509ExtensionAlternativeNames_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509ExtensionAlternativeNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionAlternativeNames_Impl::InitializeEncode(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509ExtensionAlternativeNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionAlternativeNames_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn AlternativeNames<Identity: IX509ExtensionAlternativeNames_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionAlternativeNames_Impl::AlternativeNames(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Extension_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            AlternativeNames: AlternativeNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509ExtensionAlternativeNames as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Extension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509ExtensionAuthorityKeyIdentifier_Impl: Sized + IX509Extension_Impl {
    fn InitializeEncode(&self, encoding: EncodingType, strkeyidentifier: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_AuthorityKeyIdentifier(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509ExtensionAuthorityKeyIdentifier {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509ExtensionAuthorityKeyIdentifier_Vtbl {
    pub const fn new<Identity: IX509ExtensionAuthorityKeyIdentifier_Impl, const OFFSET: isize>() -> IX509ExtensionAuthorityKeyIdentifier_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509ExtensionAuthorityKeyIdentifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strkeyidentifier: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionAuthorityKeyIdentifier_Impl::InitializeEncode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strkeyidentifier)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509ExtensionAuthorityKeyIdentifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionAuthorityKeyIdentifier_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn get_AuthorityKeyIdentifier<Identity: IX509ExtensionAuthorityKeyIdentifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionAuthorityKeyIdentifier_Impl::get_AuthorityKeyIdentifier(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Extension_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            get_AuthorityKeyIdentifier: get_AuthorityKeyIdentifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509ExtensionAuthorityKeyIdentifier as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Extension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509ExtensionBasicConstraints_Impl: Sized + IX509Extension_Impl {
    fn InitializeEncode(&self, isca: super::super::super::Foundation::VARIANT_BOOL, pathlenconstraint: i32) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn IsCA(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn PathLenConstraint(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509ExtensionBasicConstraints {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509ExtensionBasicConstraints_Vtbl {
    pub const fn new<Identity: IX509ExtensionBasicConstraints_Impl, const OFFSET: isize>() -> IX509ExtensionBasicConstraints_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509ExtensionBasicConstraints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isca: super::super::super::Foundation::VARIANT_BOOL, pathlenconstraint: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionBasicConstraints_Impl::InitializeEncode(this, core::mem::transmute_copy(&isca), core::mem::transmute_copy(&pathlenconstraint)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509ExtensionBasicConstraints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionBasicConstraints_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn IsCA<Identity: IX509ExtensionBasicConstraints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionBasicConstraints_Impl::IsCA(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PathLenConstraint<Identity: IX509ExtensionBasicConstraints_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionBasicConstraints_Impl::PathLenConstraint(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Extension_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            IsCA: IsCA::<Identity, OFFSET>,
            PathLenConstraint: PathLenConstraint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509ExtensionBasicConstraints as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Extension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509ExtensionCertificatePolicies_Impl: Sized + IX509Extension_Impl {
    fn InitializeEncode(&self, pvalue: Option<&ICertificatePolicies>) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Policies(&self) -> windows_core::Result<ICertificatePolicies>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509ExtensionCertificatePolicies {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509ExtensionCertificatePolicies_Vtbl {
    pub const fn new<Identity: IX509ExtensionCertificatePolicies_Impl, const OFFSET: isize>() -> IX509ExtensionCertificatePolicies_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509ExtensionCertificatePolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionCertificatePolicies_Impl::InitializeEncode(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509ExtensionCertificatePolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionCertificatePolicies_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn Policies<Identity: IX509ExtensionCertificatePolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionCertificatePolicies_Impl::Policies(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Extension_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            Policies: Policies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509ExtensionCertificatePolicies as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Extension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509ExtensionEnhancedKeyUsage_Impl: Sized + IX509Extension_Impl {
    fn InitializeEncode(&self, pvalue: Option<&IObjectIds>) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn EnhancedKeyUsage(&self) -> windows_core::Result<IObjectIds>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509ExtensionEnhancedKeyUsage {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509ExtensionEnhancedKeyUsage_Vtbl {
    pub const fn new<Identity: IX509ExtensionEnhancedKeyUsage_Impl, const OFFSET: isize>() -> IX509ExtensionEnhancedKeyUsage_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509ExtensionEnhancedKeyUsage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionEnhancedKeyUsage_Impl::InitializeEncode(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509ExtensionEnhancedKeyUsage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionEnhancedKeyUsage_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn EnhancedKeyUsage<Identity: IX509ExtensionEnhancedKeyUsage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionEnhancedKeyUsage_Impl::EnhancedKeyUsage(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Extension_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            EnhancedKeyUsage: EnhancedKeyUsage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509ExtensionEnhancedKeyUsage as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Extension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509ExtensionKeyUsage_Impl: Sized + IX509Extension_Impl {
    fn InitializeEncode(&self, usageflags: X509KeyUsageFlags) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn KeyUsage(&self) -> windows_core::Result<X509KeyUsageFlags>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509ExtensionKeyUsage {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509ExtensionKeyUsage_Vtbl {
    pub const fn new<Identity: IX509ExtensionKeyUsage_Impl, const OFFSET: isize>() -> IX509ExtensionKeyUsage_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509ExtensionKeyUsage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usageflags: X509KeyUsageFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionKeyUsage_Impl::InitializeEncode(this, core::mem::transmute_copy(&usageflags)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509ExtensionKeyUsage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionKeyUsage_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn KeyUsage<Identity: IX509ExtensionKeyUsage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509KeyUsageFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionKeyUsage_Impl::KeyUsage(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Extension_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            KeyUsage: KeyUsage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509ExtensionKeyUsage as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Extension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509ExtensionMSApplicationPolicies_Impl: Sized + IX509Extension_Impl {
    fn InitializeEncode(&self, pvalue: Option<&ICertificatePolicies>) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Policies(&self) -> windows_core::Result<ICertificatePolicies>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509ExtensionMSApplicationPolicies {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509ExtensionMSApplicationPolicies_Vtbl {
    pub const fn new<Identity: IX509ExtensionMSApplicationPolicies_Impl, const OFFSET: isize>() -> IX509ExtensionMSApplicationPolicies_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509ExtensionMSApplicationPolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionMSApplicationPolicies_Impl::InitializeEncode(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509ExtensionMSApplicationPolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionMSApplicationPolicies_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn Policies<Identity: IX509ExtensionMSApplicationPolicies_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionMSApplicationPolicies_Impl::Policies(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Extension_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            Policies: Policies::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509ExtensionMSApplicationPolicies as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Extension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509ExtensionSmimeCapabilities_Impl: Sized + IX509Extension_Impl {
    fn InitializeEncode(&self, pvalue: Option<&ISmimeCapabilities>) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SmimeCapabilities(&self) -> windows_core::Result<ISmimeCapabilities>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509ExtensionSmimeCapabilities {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509ExtensionSmimeCapabilities_Vtbl {
    pub const fn new<Identity: IX509ExtensionSmimeCapabilities_Impl, const OFFSET: isize>() -> IX509ExtensionSmimeCapabilities_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509ExtensionSmimeCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionSmimeCapabilities_Impl::InitializeEncode(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509ExtensionSmimeCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionSmimeCapabilities_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn SmimeCapabilities<Identity: IX509ExtensionSmimeCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionSmimeCapabilities_Impl::SmimeCapabilities(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Extension_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            SmimeCapabilities: SmimeCapabilities::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509ExtensionSmimeCapabilities as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Extension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509ExtensionSubjectKeyIdentifier_Impl: Sized + IX509Extension_Impl {
    fn InitializeEncode(&self, encoding: EncodingType, strkeyidentifier: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_SubjectKeyIdentifier(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509ExtensionSubjectKeyIdentifier {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509ExtensionSubjectKeyIdentifier_Vtbl {
    pub const fn new<Identity: IX509ExtensionSubjectKeyIdentifier_Impl, const OFFSET: isize>() -> IX509ExtensionSubjectKeyIdentifier_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509ExtensionSubjectKeyIdentifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strkeyidentifier: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionSubjectKeyIdentifier_Impl::InitializeEncode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strkeyidentifier)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509ExtensionSubjectKeyIdentifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionSubjectKeyIdentifier_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn get_SubjectKeyIdentifier<Identity: IX509ExtensionSubjectKeyIdentifier_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionSubjectKeyIdentifier_Impl::get_SubjectKeyIdentifier(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Extension_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            get_SubjectKeyIdentifier: get_SubjectKeyIdentifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509ExtensionSubjectKeyIdentifier as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Extension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509ExtensionTemplate_Impl: Sized + IX509Extension_Impl {
    fn InitializeEncode(&self, ptemplateoid: Option<&IObjectId>, majorversion: i32, minorversion: i32) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TemplateOid(&self) -> windows_core::Result<IObjectId>;
    fn MajorVersion(&self) -> windows_core::Result<i32>;
    fn MinorVersion(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509ExtensionTemplate {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509ExtensionTemplate_Vtbl {
    pub const fn new<Identity: IX509ExtensionTemplate_Impl, const OFFSET: isize>() -> IX509ExtensionTemplate_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509ExtensionTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptemplateoid: *mut core::ffi::c_void, majorversion: i32, minorversion: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionTemplate_Impl::InitializeEncode(this, windows_core::from_raw_borrowed(&ptemplateoid), core::mem::transmute_copy(&majorversion), core::mem::transmute_copy(&minorversion)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509ExtensionTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionTemplate_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn TemplateOid<Identity: IX509ExtensionTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionTemplate_Impl::TemplateOid(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: IX509ExtensionTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionTemplate_Impl::MajorVersion(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: IX509ExtensionTemplate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionTemplate_Impl::MinorVersion(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Extension_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            TemplateOid: TemplateOid::<Identity, OFFSET>,
            MajorVersion: MajorVersion::<Identity, OFFSET>,
            MinorVersion: MinorVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509ExtensionTemplate as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Extension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509ExtensionTemplateName_Impl: Sized + IX509Extension_Impl {
    fn InitializeEncode(&self, strtemplatename: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeDecode(&self, encoding: EncodingType, strencodeddata: &windows_core::BSTR) -> windows_core::Result<()>;
    fn TemplateName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509ExtensionTemplateName {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509ExtensionTemplateName_Vtbl {
    pub const fn new<Identity: IX509ExtensionTemplateName_Impl, const OFFSET: isize>() -> IX509ExtensionTemplateName_Vtbl {
        unsafe extern "system" fn InitializeEncode<Identity: IX509ExtensionTemplateName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strtemplatename: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionTemplateName_Impl::InitializeEncode(this, core::mem::transmute(&strtemplatename)).into()
        }
        unsafe extern "system" fn InitializeDecode<Identity: IX509ExtensionTemplateName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, strencodeddata: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509ExtensionTemplateName_Impl::InitializeDecode(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&strencodeddata)).into()
        }
        unsafe extern "system" fn TemplateName<Identity: IX509ExtensionTemplateName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509ExtensionTemplateName_Impl::TemplateName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IX509Extension_Vtbl::new::<Identity, OFFSET>(),
            InitializeEncode: InitializeEncode::<Identity, OFFSET>,
            InitializeDecode: InitializeDecode::<Identity, OFFSET>,
            TemplateName: TemplateName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509ExtensionTemplateName as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509Extension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509Extensions_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509Extension>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&IX509Extension>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn get_IndexByObjectId(&self, pobjectid: Option<&IObjectId>) -> windows_core::Result<i32>;
    fn AddRange(&self, pvalue: Option<&IX509Extensions>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509Extensions {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509Extensions_Vtbl {
    pub const fn new<Identity: IX509Extensions_Impl, const OFFSET: isize>() -> IX509Extensions_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: IX509Extensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Extensions_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IX509Extensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Extensions_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IX509Extensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Extensions_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IX509Extensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Extensions_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: IX509Extensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Extensions_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: IX509Extensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Extensions_Impl::Clear(this).into()
        }
        unsafe extern "system" fn get_IndexByObjectId<Identity: IX509Extensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectid: *mut core::ffi::c_void, pindex: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509Extensions_Impl::get_IndexByObjectId(this, windows_core::from_raw_borrowed(&pobjectid)) {
                Ok(ok__) => {
                    pindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRange<Identity: IX509Extensions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509Extensions_Impl::AddRange(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            get_IndexByObjectId: get_IndexByObjectId::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509Extensions as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509MachineEnrollmentFactory_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn CreateObject(&self, strprogid: &windows_core::BSTR) -> windows_core::Result<IX509EnrollmentHelper>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509MachineEnrollmentFactory {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509MachineEnrollmentFactory_Vtbl {
    pub const fn new<Identity: IX509MachineEnrollmentFactory_Impl, const OFFSET: isize>() -> IX509MachineEnrollmentFactory_Vtbl {
        unsafe extern "system" fn CreateObject<Identity: IX509MachineEnrollmentFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strprogid: core::mem::MaybeUninit<windows_core::BSTR>, ppihelper: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509MachineEnrollmentFactory_Impl::CreateObject(this, core::mem::transmute(&strprogid)) {
                Ok(ok__) => {
                    ppihelper.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(), CreateObject: CreateObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509MachineEnrollmentFactory as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509NameValuePair_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, strname: &windows_core::BSTR, strvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509NameValuePair {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509NameValuePair_Vtbl {
    pub const fn new<Identity: IX509NameValuePair_Impl, const OFFSET: isize>() -> IX509NameValuePair_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509NameValuePair_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strname: core::mem::MaybeUninit<windows_core::BSTR>, strvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509NameValuePair_Impl::Initialize(this, core::mem::transmute(&strname), core::mem::transmute(&strvalue)).into()
        }
        unsafe extern "system" fn Value<Identity: IX509NameValuePair_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509NameValuePair_Impl::Value(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: IX509NameValuePair_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509NameValuePair_Impl::Name(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509NameValuePair as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509NameValuePairs_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509NameValuePair>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&IX509NameValuePair>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509NameValuePairs {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509NameValuePairs_Vtbl {
    pub const fn new<Identity: IX509NameValuePairs_Impl, const OFFSET: isize>() -> IX509NameValuePairs_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: IX509NameValuePairs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509NameValuePairs_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IX509NameValuePairs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509NameValuePairs_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IX509NameValuePairs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509NameValuePairs_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IX509NameValuePairs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509NameValuePairs_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: IX509NameValuePairs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509NameValuePairs_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: IX509NameValuePairs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509NameValuePairs_Impl::Clear(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509NameValuePairs as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509PolicyServerListManager_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn get_ItemByIndex(&self, index: i32) -> windows_core::Result<IX509PolicyServerUrl>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn _NewEnum(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Add(&self, pval: Option<&IX509PolicyServerUrl>) -> windows_core::Result<()>;
    fn Remove(&self, index: i32) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn Initialize(&self, context: X509CertificateEnrollmentContext, flags: PolicyServerUrlFlags) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509PolicyServerListManager {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509PolicyServerListManager_Vtbl {
    pub const fn new<Identity: IX509PolicyServerListManager_Impl, const OFFSET: isize>() -> IX509PolicyServerListManager_Vtbl {
        unsafe extern "system" fn get_ItemByIndex<Identity: IX509PolicyServerListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PolicyServerListManager_Impl::get_ItemByIndex(this, core::mem::transmute_copy(&index)) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: IX509PolicyServerListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PolicyServerListManager_Impl::Count(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: IX509PolicyServerListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PolicyServerListManager_Impl::_NewEnum(this) {
                Ok(ok__) => {
                    pval.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: IX509PolicyServerListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pval: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerListManager_Impl::Add(this, windows_core::from_raw_borrowed(&pval)).into()
        }
        unsafe extern "system" fn Remove<Identity: IX509PolicyServerListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerListManager_Impl::Remove(this, core::mem::transmute_copy(&index)).into()
        }
        unsafe extern "system" fn Clear<Identity: IX509PolicyServerListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerListManager_Impl::Clear(this).into()
        }
        unsafe extern "system" fn Initialize<Identity: IX509PolicyServerListManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, flags: PolicyServerUrlFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerListManager_Impl::Initialize(this, core::mem::transmute_copy(&context), core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_ItemByIndex: get_ItemByIndex::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            _NewEnum: _NewEnum::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509PolicyServerListManager as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509PolicyServerUrl_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()>;
    fn Url(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetUrl(&self, pvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Default(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetDefault(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Flags(&self) -> windows_core::Result<PolicyServerUrlFlags>;
    fn SetFlags(&self, flags: PolicyServerUrlFlags) -> windows_core::Result<()>;
    fn AuthFlags(&self) -> windows_core::Result<X509EnrollmentAuthFlags>;
    fn SetAuthFlags(&self, flags: X509EnrollmentAuthFlags) -> windows_core::Result<()>;
    fn Cost(&self) -> windows_core::Result<u32>;
    fn SetCost(&self, value: u32) -> windows_core::Result<()>;
    fn GetStringProperty(&self, propertyid: PolicyServerUrlPropertyID) -> windows_core::Result<windows_core::BSTR>;
    fn SetStringProperty(&self, propertyid: PolicyServerUrlPropertyID, pvalue: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UpdateRegistry(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()>;
    fn RemoveFromRegistry(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509PolicyServerUrl {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509PolicyServerUrl_Vtbl {
    pub const fn new<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>() -> IX509PolicyServerUrl_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerUrl_Impl::Initialize(this, core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn Url<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PolicyServerUrl_Impl::Url(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUrl<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerUrl_Impl::SetUrl(this, core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn Default<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PolicyServerUrl_Impl::Default(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefault<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerUrl_Impl::SetDefault(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Flags<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut PolicyServerUrlFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PolicyServerUrl_Impl::Flags(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFlags<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: PolicyServerUrlFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerUrl_Impl::SetFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn AuthFlags<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509EnrollmentAuthFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PolicyServerUrl_Impl::AuthFlags(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthFlags<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: X509EnrollmentAuthFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerUrl_Impl::SetAuthFlags(this, core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Cost<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PolicyServerUrl_Impl::Cost(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCost<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerUrl_Impl::SetCost(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetStringProperty<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: PolicyServerUrlPropertyID, ppvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PolicyServerUrl_Impl::GetStringProperty(this, core::mem::transmute_copy(&propertyid)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStringProperty<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: PolicyServerUrlPropertyID, pvalue: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerUrl_Impl::SetStringProperty(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&pvalue)).into()
        }
        unsafe extern "system" fn UpdateRegistry<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerUrl_Impl::UpdateRegistry(this, core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn RemoveFromRegistry<Identity: IX509PolicyServerUrl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PolicyServerUrl_Impl::RemoveFromRegistry(this, core::mem::transmute_copy(&context)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Url: Url::<Identity, OFFSET>,
            SetUrl: SetUrl::<Identity, OFFSET>,
            Default: Default::<Identity, OFFSET>,
            SetDefault: SetDefault::<Identity, OFFSET>,
            Flags: Flags::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            AuthFlags: AuthFlags::<Identity, OFFSET>,
            SetAuthFlags: SetAuthFlags::<Identity, OFFSET>,
            Cost: Cost::<Identity, OFFSET>,
            SetCost: SetCost::<Identity, OFFSET>,
            GetStringProperty: GetStringProperty::<Identity, OFFSET>,
            SetStringProperty: SetStringProperty::<Identity, OFFSET>,
            UpdateRegistry: UpdateRegistry::<Identity, OFFSET>,
            RemoveFromRegistry: RemoveFromRegistry::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509PolicyServerUrl as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509PrivateKey_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Open(&self) -> windows_core::Result<()>;
    fn Create(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Delete(&self) -> windows_core::Result<()>;
    fn Verify(&self, verifytype: X509PrivateKeyVerify) -> windows_core::Result<()>;
    fn Import(&self, strexporttype: &windows_core::BSTR, strencodedkey: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn Export(&self, strexporttype: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn ExportPublicKey(&self) -> windows_core::Result<IX509PublicKey>;
    fn ContainerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetContainerName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ContainerNamePrefix(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetContainerNamePrefix(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReaderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetReaderName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CspInformations(&self) -> windows_core::Result<ICspInformations>;
    fn SetCspInformations(&self, pvalue: Option<&ICspInformations>) -> windows_core::Result<()>;
    fn CspStatus(&self) -> windows_core::Result<ICspStatus>;
    fn SetCspStatus(&self, pvalue: Option<&ICspStatus>) -> windows_core::Result<()>;
    fn ProviderName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetProviderName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ProviderType(&self) -> windows_core::Result<X509ProviderType>;
    fn SetProviderType(&self, value: X509ProviderType) -> windows_core::Result<()>;
    fn LegacyCsp(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetLegacyCsp(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Algorithm(&self) -> windows_core::Result<IObjectId>;
    fn SetAlgorithm(&self, pvalue: Option<&IObjectId>) -> windows_core::Result<()>;
    fn KeySpec(&self) -> windows_core::Result<X509KeySpec>;
    fn SetKeySpec(&self, value: X509KeySpec) -> windows_core::Result<()>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn SetLength(&self, value: i32) -> windows_core::Result<()>;
    fn ExportPolicy(&self) -> windows_core::Result<X509PrivateKeyExportFlags>;
    fn SetExportPolicy(&self, value: X509PrivateKeyExportFlags) -> windows_core::Result<()>;
    fn KeyUsage(&self) -> windows_core::Result<X509PrivateKeyUsageFlags>;
    fn SetKeyUsage(&self, value: X509PrivateKeyUsageFlags) -> windows_core::Result<()>;
    fn KeyProtection(&self) -> windows_core::Result<X509PrivateKeyProtection>;
    fn SetKeyProtection(&self, value: X509PrivateKeyProtection) -> windows_core::Result<()>;
    fn MachineContext(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetMachineContext(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SecurityDescriptor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetSecurityDescriptor(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_Certificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn put_Certificate(&self, encoding: EncodingType, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn UniqueContainerName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Opened(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn DefaultContainer(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Existing(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetExisting(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn Silent(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetSilent(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ParentWindow(&self) -> windows_core::Result<i32>;
    fn SetParentWindow(&self, value: i32) -> windows_core::Result<()>;
    fn UIContextMessage(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetUIContextMessage(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn SetPin(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetFriendlyName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDescription(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509PrivateKey {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509PrivateKey_Vtbl {
    pub const fn new<Identity: IX509PrivateKey_Impl, const OFFSET: isize>() -> IX509PrivateKey_Vtbl {
        unsafe extern "system" fn Open<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::Open(this).into()
        }
        unsafe extern "system" fn Create<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::Create(this).into()
        }
        unsafe extern "system" fn Close<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::Close(this).into()
        }
        unsafe extern "system" fn Delete<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::Delete(this).into()
        }
        unsafe extern "system" fn Verify<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, verifytype: X509PrivateKeyVerify) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::Verify(this, core::mem::transmute_copy(&verifytype)).into()
        }
        unsafe extern "system" fn Import<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strexporttype: core::mem::MaybeUninit<windows_core::BSTR>, strencodedkey: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::Import(this, core::mem::transmute(&strexporttype), core::mem::transmute(&strencodedkey), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn Export<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strexporttype: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType, pstrencodedkey: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::Export(this, core::mem::transmute(&strexporttype), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pstrencodedkey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPublicKey<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppublickey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::ExportPublicKey(this) {
                Ok(ok__) => {
                    pppublickey.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContainerName<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::ContainerName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerName<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetContainerName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ContainerNamePrefix<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::ContainerNamePrefix(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContainerNamePrefix<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetContainerNamePrefix(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ReaderName<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::ReaderName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReaderName<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetReaderName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CspInformations<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::CspInformations(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCspInformations<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetCspInformations(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn CspStatus<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::CspStatus(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCspStatus<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetCspStatus(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn ProviderName<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::ProviderName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderName<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetProviderName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ProviderType<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509ProviderType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::ProviderType(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProviderType<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: X509ProviderType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetProviderType(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn LegacyCsp<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::LegacyCsp(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLegacyCsp<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetLegacyCsp(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Algorithm<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::Algorithm(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlgorithm<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetAlgorithm(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn KeySpec<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509KeySpec) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::KeySpec(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeySpec<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: X509KeySpec) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetKeySpec(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Length<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::Length(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetLength(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ExportPolicy<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509PrivateKeyExportFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::ExportPolicy(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExportPolicy<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: X509PrivateKeyExportFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetExportPolicy(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn KeyUsage<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509PrivateKeyUsageFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::KeyUsage(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyUsage<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: X509PrivateKeyUsageFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetKeyUsage(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn KeyProtection<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509PrivateKeyProtection) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::KeyProtection(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetKeyProtection<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: X509PrivateKeyProtection) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetKeyProtection(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn MachineContext<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::MachineContext(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachineContext<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetMachineContext(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::SecurityDescriptor(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetSecurityDescriptor(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn get_Certificate<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::get_Certificate(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Certificate<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::put_Certificate(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn UniqueContainerName<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::UniqueContainerName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Opened<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::Opened(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DefaultContainer<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::DefaultContainer(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Existing<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::Existing(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExisting<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetExisting(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Silent<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::Silent(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSilent<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetSilent(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ParentWindow<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::ParentWindow(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParentWindow<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetParentWindow(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UIContextMessage<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::UIContextMessage(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUIContextMessage<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetUIContextMessage(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn SetPin<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetPin(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn FriendlyName<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::FriendlyName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFriendlyName<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetFriendlyName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Description<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey_Impl::Description(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: IX509PrivateKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey_Impl::SetDescription(this, core::mem::transmute(&value)).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            Create: Create::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Delete: Delete::<Identity, OFFSET>,
            Verify: Verify::<Identity, OFFSET>,
            Import: Import::<Identity, OFFSET>,
            Export: Export::<Identity, OFFSET>,
            ExportPublicKey: ExportPublicKey::<Identity, OFFSET>,
            ContainerName: ContainerName::<Identity, OFFSET>,
            SetContainerName: SetContainerName::<Identity, OFFSET>,
            ContainerNamePrefix: ContainerNamePrefix::<Identity, OFFSET>,
            SetContainerNamePrefix: SetContainerNamePrefix::<Identity, OFFSET>,
            ReaderName: ReaderName::<Identity, OFFSET>,
            SetReaderName: SetReaderName::<Identity, OFFSET>,
            CspInformations: CspInformations::<Identity, OFFSET>,
            SetCspInformations: SetCspInformations::<Identity, OFFSET>,
            CspStatus: CspStatus::<Identity, OFFSET>,
            SetCspStatus: SetCspStatus::<Identity, OFFSET>,
            ProviderName: ProviderName::<Identity, OFFSET>,
            SetProviderName: SetProviderName::<Identity, OFFSET>,
            ProviderType: ProviderType::<Identity, OFFSET>,
            SetProviderType: SetProviderType::<Identity, OFFSET>,
            LegacyCsp: LegacyCsp::<Identity, OFFSET>,
            SetLegacyCsp: SetLegacyCsp::<Identity, OFFSET>,
            Algorithm: Algorithm::<Identity, OFFSET>,
            SetAlgorithm: SetAlgorithm::<Identity, OFFSET>,
            KeySpec: KeySpec::<Identity, OFFSET>,
            SetKeySpec: SetKeySpec::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            SetLength: SetLength::<Identity, OFFSET>,
            ExportPolicy: ExportPolicy::<Identity, OFFSET>,
            SetExportPolicy: SetExportPolicy::<Identity, OFFSET>,
            KeyUsage: KeyUsage::<Identity, OFFSET>,
            SetKeyUsage: SetKeyUsage::<Identity, OFFSET>,
            KeyProtection: KeyProtection::<Identity, OFFSET>,
            SetKeyProtection: SetKeyProtection::<Identity, OFFSET>,
            MachineContext: MachineContext::<Identity, OFFSET>,
            SetMachineContext: SetMachineContext::<Identity, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, OFFSET>,
            get_Certificate: get_Certificate::<Identity, OFFSET>,
            put_Certificate: put_Certificate::<Identity, OFFSET>,
            UniqueContainerName: UniqueContainerName::<Identity, OFFSET>,
            Opened: Opened::<Identity, OFFSET>,
            DefaultContainer: DefaultContainer::<Identity, OFFSET>,
            Existing: Existing::<Identity, OFFSET>,
            SetExisting: SetExisting::<Identity, OFFSET>,
            Silent: Silent::<Identity, OFFSET>,
            SetSilent: SetSilent::<Identity, OFFSET>,
            ParentWindow: ParentWindow::<Identity, OFFSET>,
            SetParentWindow: SetParentWindow::<Identity, OFFSET>,
            UIContextMessage: UIContextMessage::<Identity, OFFSET>,
            SetUIContextMessage: SetUIContextMessage::<Identity, OFFSET>,
            SetPin: SetPin::<Identity, OFFSET>,
            FriendlyName: FriendlyName::<Identity, OFFSET>,
            SetFriendlyName: SetFriendlyName::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            SetDescription: SetDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509PrivateKey as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509PrivateKey2_Impl: Sized + IX509PrivateKey_Impl {
    fn HardwareKeyUsage(&self) -> windows_core::Result<X509HardwareKeyUsageFlags>;
    fn SetHardwareKeyUsage(&self, value: X509HardwareKeyUsageFlags) -> windows_core::Result<()>;
    fn AlternateStorageLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAlternateStorageLocation(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AlgorithmName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetAlgorithmName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_AlgorithmParameters(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn put_AlgorithmParameters(&self, encoding: EncodingType, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ParametersExportType(&self) -> windows_core::Result<X509KeyParametersExportType>;
    fn SetParametersExportType(&self, value: X509KeyParametersExportType) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509PrivateKey2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509PrivateKey2_Vtbl {
    pub const fn new<Identity: IX509PrivateKey2_Impl, const OFFSET: isize>() -> IX509PrivateKey2_Vtbl {
        unsafe extern "system" fn HardwareKeyUsage<Identity: IX509PrivateKey2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509HardwareKeyUsageFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey2_Impl::HardwareKeyUsage(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHardwareKeyUsage<Identity: IX509PrivateKey2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: X509HardwareKeyUsageFlags) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey2_Impl::SetHardwareKeyUsage(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AlternateStorageLocation<Identity: IX509PrivateKey2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey2_Impl::AlternateStorageLocation(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlternateStorageLocation<Identity: IX509PrivateKey2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey2_Impl::SetAlternateStorageLocation(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn AlgorithmName<Identity: IX509PrivateKey2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey2_Impl::AlgorithmName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlgorithmName<Identity: IX509PrivateKey2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey2_Impl::SetAlgorithmName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn get_AlgorithmParameters<Identity: IX509PrivateKey2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey2_Impl::get_AlgorithmParameters(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_AlgorithmParameters<Identity: IX509PrivateKey2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey2_Impl::put_AlgorithmParameters(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ParametersExportType<Identity: IX509PrivateKey2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509KeyParametersExportType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PrivateKey2_Impl::ParametersExportType(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetParametersExportType<Identity: IX509PrivateKey2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: X509KeyParametersExportType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PrivateKey2_Impl::SetParametersExportType(this, core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: IX509PrivateKey_Vtbl::new::<Identity, OFFSET>(),
            HardwareKeyUsage: HardwareKeyUsage::<Identity, OFFSET>,
            SetHardwareKeyUsage: SetHardwareKeyUsage::<Identity, OFFSET>,
            AlternateStorageLocation: AlternateStorageLocation::<Identity, OFFSET>,
            SetAlternateStorageLocation: SetAlternateStorageLocation::<Identity, OFFSET>,
            AlgorithmName: AlgorithmName::<Identity, OFFSET>,
            SetAlgorithmName: SetAlgorithmName::<Identity, OFFSET>,
            get_AlgorithmParameters: get_AlgorithmParameters::<Identity, OFFSET>,
            put_AlgorithmParameters: put_AlgorithmParameters::<Identity, OFFSET>,
            ParametersExportType: ParametersExportType::<Identity, OFFSET>,
            SetParametersExportType: SetParametersExportType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509PrivateKey2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509PrivateKey as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509PublicKey_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, pobjectid: Option<&IObjectId>, strencodedkey: &windows_core::BSTR, strencodedparameters: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn InitializeFromEncodedPublicKeyInfo(&self, strencodedpublickeyinfo: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn Algorithm(&self) -> windows_core::Result<IObjectId>;
    fn Length(&self) -> windows_core::Result<i32>;
    fn get_EncodedKey(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn get_EncodedParameters(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn ComputeKeyIdentifier(&self, algorithm: KeyIdentifierHashAlgorithm, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509PublicKey {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509PublicKey_Vtbl {
    pub const fn new<Identity: IX509PublicKey_Impl, const OFFSET: isize>() -> IX509PublicKey_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509PublicKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectid: *mut core::ffi::c_void, strencodedkey: core::mem::MaybeUninit<windows_core::BSTR>, strencodedparameters: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PublicKey_Impl::Initialize(this, windows_core::from_raw_borrowed(&pobjectid), core::mem::transmute(&strencodedkey), core::mem::transmute(&strencodedparameters), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn InitializeFromEncodedPublicKeyInfo<Identity: IX509PublicKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strencodedpublickeyinfo: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509PublicKey_Impl::InitializeFromEncodedPublicKeyInfo(this, core::mem::transmute(&strencodedpublickeyinfo), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn Algorithm<Identity: IX509PublicKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PublicKey_Impl::Algorithm(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: IX509PublicKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut i32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PublicKey_Impl::Length(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_EncodedKey<Identity: IX509PublicKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PublicKey_Impl::get_EncodedKey(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_EncodedParameters<Identity: IX509PublicKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PublicKey_Impl::get_EncodedParameters(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ComputeKeyIdentifier<Identity: IX509PublicKey_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, algorithm: KeyIdentifierHashAlgorithm, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509PublicKey_Impl::ComputeKeyIdentifier(this, core::mem::transmute_copy(&algorithm), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            InitializeFromEncodedPublicKeyInfo: InitializeFromEncodedPublicKeyInfo::<Identity, OFFSET>,
            Algorithm: Algorithm::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            get_EncodedKey: get_EncodedKey::<Identity, OFFSET>,
            get_EncodedParameters: get_EncodedParameters::<Identity, OFFSET>,
            ComputeKeyIdentifier: ComputeKeyIdentifier::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509PublicKey as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509SCEPEnrollment_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, prequest: Option<&IX509CertificateRequestPkcs10>, strthumbprint: &windows_core::BSTR, thumprintencoding: EncodingType, strservercertificates: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<()>;
    fn InitializeForPending(&self, context: X509CertificateEnrollmentContext) -> windows_core::Result<()>;
    fn CreateRequestMessage(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn CreateRetrievePendingMessage(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn CreateRetrieveCertificateMessage(&self, context: X509CertificateEnrollmentContext, strissuer: &windows_core::BSTR, issuerencoding: EncodingType, strserialnumber: &windows_core::BSTR, serialnumberencoding: EncodingType, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn ProcessResponseMessage(&self, strresponse: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<X509SCEPDisposition>;
    fn SetServerCapabilities(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn FailInfo(&self) -> windows_core::Result<X509SCEPFailInfo>;
    fn SignerCertificate(&self) -> windows_core::Result<ISignerCertificate>;
    fn SetSignerCertificate(&self, pvalue: Option<&ISignerCertificate>) -> windows_core::Result<()>;
    fn OldCertificate(&self) -> windows_core::Result<ISignerCertificate>;
    fn SetOldCertificate(&self, pvalue: Option<&ISignerCertificate>) -> windows_core::Result<()>;
    fn get_TransactionId(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn put_TransactionId(&self, encoding: EncodingType, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Request(&self) -> windows_core::Result<IX509CertificateRequestPkcs10>;
    fn CertificateFriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetCertificateFriendlyName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Status(&self) -> windows_core::Result<IX509EnrollmentStatus>;
    fn get_Certificate(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn Silent(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetSilent(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DeleteRequest(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509SCEPEnrollment {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509SCEPEnrollment_Vtbl {
    pub const fn new<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>() -> IX509SCEPEnrollment_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *mut core::ffi::c_void, strthumbprint: core::mem::MaybeUninit<windows_core::BSTR>, thumprintencoding: EncodingType, strservercertificates: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollment_Impl::Initialize(this, windows_core::from_raw_borrowed(&prequest), core::mem::transmute(&strthumbprint), core::mem::transmute_copy(&thumprintencoding), core::mem::transmute(&strservercertificates), core::mem::transmute_copy(&encoding)).into()
        }
        unsafe extern "system" fn InitializeForPending<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollment_Impl::InitializeForPending(this, core::mem::transmute_copy(&context)).into()
        }
        unsafe extern "system" fn CreateRequestMessage<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::CreateRequestMessage(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRetrievePendingMessage<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::CreateRetrievePendingMessage(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRetrieveCertificateMessage<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, context: X509CertificateEnrollmentContext, strissuer: core::mem::MaybeUninit<windows_core::BSTR>, issuerencoding: EncodingType, strserialnumber: core::mem::MaybeUninit<windows_core::BSTR>, serialnumberencoding: EncodingType, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::CreateRetrieveCertificateMessage(this, core::mem::transmute_copy(&context), core::mem::transmute(&strissuer), core::mem::transmute_copy(&issuerencoding), core::mem::transmute(&strserialnumber), core::mem::transmute_copy(&serialnumberencoding), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessResponseMessage<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strresponse: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType, pdisposition: *mut X509SCEPDisposition) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::ProcessResponseMessage(this, core::mem::transmute(&strresponse), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pdisposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCapabilities<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollment_Impl::SetServerCapabilities(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn FailInfo<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut X509SCEPFailInfo) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::FailInfo(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignerCertificate<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::SignerCertificate(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSignerCertificate<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollment_Impl::SetSignerCertificate(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn OldCertificate<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::OldCertificate(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOldCertificate<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollment_Impl::SetOldCertificate(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn get_TransactionId<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::get_TransactionId(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_TransactionId<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollment_Impl::put_TransactionId(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Request<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::Request(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CertificateFriendlyName<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::CertificateFriendlyName(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateFriendlyName<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollment_Impl::SetCertificateFriendlyName(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Status<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::Status(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Certificate<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::get_Certificate(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Silent<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment_Impl::Silent(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSilent<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollment_Impl::SetSilent(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DeleteRequest<Identity: IX509SCEPEnrollment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollment_Impl::DeleteRequest(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            InitializeForPending: InitializeForPending::<Identity, OFFSET>,
            CreateRequestMessage: CreateRequestMessage::<Identity, OFFSET>,
            CreateRetrievePendingMessage: CreateRetrievePendingMessage::<Identity, OFFSET>,
            CreateRetrieveCertificateMessage: CreateRetrieveCertificateMessage::<Identity, OFFSET>,
            ProcessResponseMessage: ProcessResponseMessage::<Identity, OFFSET>,
            SetServerCapabilities: SetServerCapabilities::<Identity, OFFSET>,
            FailInfo: FailInfo::<Identity, OFFSET>,
            SignerCertificate: SignerCertificate::<Identity, OFFSET>,
            SetSignerCertificate: SetSignerCertificate::<Identity, OFFSET>,
            OldCertificate: OldCertificate::<Identity, OFFSET>,
            SetOldCertificate: SetOldCertificate::<Identity, OFFSET>,
            get_TransactionId: get_TransactionId::<Identity, OFFSET>,
            put_TransactionId: put_TransactionId::<Identity, OFFSET>,
            Request: Request::<Identity, OFFSET>,
            CertificateFriendlyName: CertificateFriendlyName::<Identity, OFFSET>,
            SetCertificateFriendlyName: SetCertificateFriendlyName::<Identity, OFFSET>,
            Status: Status::<Identity, OFFSET>,
            get_Certificate: get_Certificate::<Identity, OFFSET>,
            Silent: Silent::<Identity, OFFSET>,
            SetSilent: SetSilent::<Identity, OFFSET>,
            DeleteRequest: DeleteRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509SCEPEnrollment as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509SCEPEnrollment2_Impl: Sized + IX509SCEPEnrollment_Impl {
    fn CreateChallengeAnswerMessage(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn ProcessResponseMessage2(&self, flags: X509SCEPProcessMessageFlags, strresponse: &windows_core::BSTR, encoding: EncodingType) -> windows_core::Result<X509SCEPDisposition>;
    fn ResultMessageText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DelayRetry(&self) -> windows_core::Result<DelayRetryAction>;
    fn ActivityId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetActivityId(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509SCEPEnrollment2 {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509SCEPEnrollment2_Vtbl {
    pub const fn new<Identity: IX509SCEPEnrollment2_Impl, const OFFSET: isize>() -> IX509SCEPEnrollment2_Vtbl {
        unsafe extern "system" fn CreateChallengeAnswerMessage<Identity: IX509SCEPEnrollment2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment2_Impl::CreateChallengeAnswerMessage(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessResponseMessage2<Identity: IX509SCEPEnrollment2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: X509SCEPProcessMessageFlags, strresponse: core::mem::MaybeUninit<windows_core::BSTR>, encoding: EncodingType, pdisposition: *mut X509SCEPDisposition) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment2_Impl::ProcessResponseMessage2(this, core::mem::transmute_copy(&flags), core::mem::transmute(&strresponse), core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pdisposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultMessageText<Identity: IX509SCEPEnrollment2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment2_Impl::ResultMessageText(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelayRetry<Identity: IX509SCEPEnrollment2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut DelayRetryAction) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment2_Impl::DelayRetry(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Identity: IX509SCEPEnrollment2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollment2_Impl::ActivityId(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityId<Identity: IX509SCEPEnrollment2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollment2_Impl::SetActivityId(this, core::mem::transmute(&value)).into()
        }
        Self {
            base__: IX509SCEPEnrollment_Vtbl::new::<Identity, OFFSET>(),
            CreateChallengeAnswerMessage: CreateChallengeAnswerMessage::<Identity, OFFSET>,
            ProcessResponseMessage2: ProcessResponseMessage2::<Identity, OFFSET>,
            ResultMessageText: ResultMessageText::<Identity, OFFSET>,
            DelayRetry: DelayRetry::<Identity, OFFSET>,
            ActivityId: ActivityId::<Identity, OFFSET>,
            SetActivityId: SetActivityId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509SCEPEnrollment2 as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID || iid == &<IX509SCEPEnrollment as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509SCEPEnrollmentHelper_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn Initialize(&self, strserverurl: &windows_core::BSTR, strrequestheaders: &windows_core::BSTR, prequest: Option<&IX509CertificateRequestPkcs10>, strcacertificatethumbprint: &windows_core::BSTR) -> windows_core::Result<()>;
    fn InitializeForPending(&self, strserverurl: &windows_core::BSTR, strrequestheaders: &windows_core::BSTR, context: X509CertificateEnrollmentContext, strtransactionid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn Enroll(&self, processflags: X509SCEPProcessMessageFlags) -> windows_core::Result<X509SCEPDisposition>;
    fn FetchPending(&self, processflags: X509SCEPProcessMessageFlags) -> windows_core::Result<X509SCEPDisposition>;
    fn X509SCEPEnrollment(&self) -> windows_core::Result<IX509SCEPEnrollment>;
    fn ResultMessageText(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509SCEPEnrollmentHelper {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509SCEPEnrollmentHelper_Vtbl {
    pub const fn new<Identity: IX509SCEPEnrollmentHelper_Impl, const OFFSET: isize>() -> IX509SCEPEnrollmentHelper_Vtbl {
        unsafe extern "system" fn Initialize<Identity: IX509SCEPEnrollmentHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserverurl: core::mem::MaybeUninit<windows_core::BSTR>, strrequestheaders: core::mem::MaybeUninit<windows_core::BSTR>, prequest: *mut core::ffi::c_void, strcacertificatethumbprint: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollmentHelper_Impl::Initialize(this, core::mem::transmute(&strserverurl), core::mem::transmute(&strrequestheaders), windows_core::from_raw_borrowed(&prequest), core::mem::transmute(&strcacertificatethumbprint)).into()
        }
        unsafe extern "system" fn InitializeForPending<Identity: IX509SCEPEnrollmentHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strserverurl: core::mem::MaybeUninit<windows_core::BSTR>, strrequestheaders: core::mem::MaybeUninit<windows_core::BSTR>, context: X509CertificateEnrollmentContext, strtransactionid: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SCEPEnrollmentHelper_Impl::InitializeForPending(this, core::mem::transmute(&strserverurl), core::mem::transmute(&strrequestheaders), core::mem::transmute_copy(&context), core::mem::transmute(&strtransactionid)).into()
        }
        unsafe extern "system" fn Enroll<Identity: IX509SCEPEnrollmentHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processflags: X509SCEPProcessMessageFlags, pdisposition: *mut X509SCEPDisposition) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollmentHelper_Impl::Enroll(this, core::mem::transmute_copy(&processflags)) {
                Ok(ok__) => {
                    pdisposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FetchPending<Identity: IX509SCEPEnrollmentHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processflags: X509SCEPProcessMessageFlags, pdisposition: *mut X509SCEPDisposition) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollmentHelper_Impl::FetchPending(this, core::mem::transmute_copy(&processflags)) {
                Ok(ok__) => {
                    pdisposition.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn X509SCEPEnrollment<Identity: IX509SCEPEnrollmentHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollmentHelper_Impl::X509SCEPEnrollment(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultMessageText<Identity: IX509SCEPEnrollmentHelper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SCEPEnrollmentHelper_Impl::ResultMessageText(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            InitializeForPending: InitializeForPending::<Identity, OFFSET>,
            Enroll: Enroll::<Identity, OFFSET>,
            FetchPending: FetchPending::<Identity, OFFSET>,
            X509SCEPEnrollment: X509SCEPEnrollment::<Identity, OFFSET>,
            ResultMessageText: ResultMessageText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509SCEPEnrollmentHelper as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IX509SignatureInformation_Impl: Sized + super::super::super::System::Com::IDispatch_Impl {
    fn HashAlgorithm(&self) -> windows_core::Result<IObjectId>;
    fn SetHashAlgorithm(&self, pvalue: Option<&IObjectId>) -> windows_core::Result<()>;
    fn PublicKeyAlgorithm(&self) -> windows_core::Result<IObjectId>;
    fn SetPublicKeyAlgorithm(&self, pvalue: Option<&IObjectId>) -> windows_core::Result<()>;
    fn get_Parameters(&self, encoding: EncodingType) -> windows_core::Result<windows_core::BSTR>;
    fn put_Parameters(&self, encoding: EncodingType, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn AlternateSignatureAlgorithm(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetAlternateSignatureAlgorithm(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn AlternateSignatureAlgorithmSet(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn NullSigned(&self) -> windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetNullSigned(&self, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<()>;
    fn GetSignatureAlgorithm(&self, pkcs7signature: super::super::super::Foundation::VARIANT_BOOL, signaturekey: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::Result<IObjectId>;
    fn SetDefaultValues(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IX509SignatureInformation {}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IX509SignatureInformation_Vtbl {
    pub const fn new<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>() -> IX509SignatureInformation_Vtbl {
        unsafe extern "system" fn HashAlgorithm<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SignatureInformation_Impl::HashAlgorithm(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHashAlgorithm<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SignatureInformation_Impl::SetHashAlgorithm(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn PublicKeyAlgorithm<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SignatureInformation_Impl::PublicKeyAlgorithm(this) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublicKeyAlgorithm<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SignatureInformation_Impl::SetPublicKeyAlgorithm(this, windows_core::from_raw_borrowed(&pvalue)).into()
        }
        unsafe extern "system" fn get_Parameters<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, pvalue: *mut core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SignatureInformation_Impl::get_Parameters(this, core::mem::transmute_copy(&encoding)) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Parameters<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, encoding: EncodingType, value: core::mem::MaybeUninit<windows_core::BSTR>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SignatureInformation_Impl::put_Parameters(this, core::mem::transmute_copy(&encoding), core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithm<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SignatureInformation_Impl::AlternateSignatureAlgorithm(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlternateSignatureAlgorithm<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SignatureInformation_Impl::SetAlternateSignatureAlgorithm(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AlternateSignatureAlgorithmSet<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SignatureInformation_Impl::AlternateSignatureAlgorithmSet(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NullSigned<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SignatureInformation_Impl::NullSigned(this) {
                Ok(ok__) => {
                    pvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNullSigned<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::super::Foundation::VARIANT_BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SignatureInformation_Impl::SetNullSigned(this, core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn GetSignatureAlgorithm<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkcs7signature: super::super::super::Foundation::VARIANT_BOOL, signaturekey: super::super::super::Foundation::VARIANT_BOOL, ppvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IX509SignatureInformation_Impl::GetSignatureAlgorithm(this, core::mem::transmute_copy(&pkcs7signature), core::mem::transmute_copy(&signaturekey)) {
                Ok(ok__) => {
                    ppvalue.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultValues<Identity: IX509SignatureInformation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IX509SignatureInformation_Impl::SetDefaultValues(this).into()
        }
        Self {
            base__: super::super::super::System::Com::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            HashAlgorithm: HashAlgorithm::<Identity, OFFSET>,
            SetHashAlgorithm: SetHashAlgorithm::<Identity, OFFSET>,
            PublicKeyAlgorithm: PublicKeyAlgorithm::<Identity, OFFSET>,
            SetPublicKeyAlgorithm: SetPublicKeyAlgorithm::<Identity, OFFSET>,
            get_Parameters: get_Parameters::<Identity, OFFSET>,
            put_Parameters: put_Parameters::<Identity, OFFSET>,
            AlternateSignatureAlgorithm: AlternateSignatureAlgorithm::<Identity, OFFSET>,
            SetAlternateSignatureAlgorithm: SetAlternateSignatureAlgorithm::<Identity, OFFSET>,
            AlternateSignatureAlgorithmSet: AlternateSignatureAlgorithmSet::<Identity, OFFSET>,
            NullSigned: NullSigned::<Identity, OFFSET>,
            SetNullSigned: SetNullSigned::<Identity, OFFSET>,
            GetSignatureAlgorithm: GetSignatureAlgorithm::<Identity, OFFSET>,
            SetDefaultValues: SetDefaultValues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IX509SignatureInformation as windows_core::Interface>::IID || iid == &<super::super::super::System::Com::IDispatch as windows_core::Interface>::IID
    }
}
