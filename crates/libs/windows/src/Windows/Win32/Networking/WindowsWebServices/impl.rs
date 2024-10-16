pub trait IContentPrefetcherTaskTrigger_Impl: Sized + windows_core::IUnknownImpl {
    fn TriggerContentPrefetcherTask(&self, packagefullname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn IsRegisteredForContentPrefetch(&self, packagefullname: &windows_core::PCWSTR) -> windows_core::Result<u8>;
}
impl windows_core::RuntimeName for IContentPrefetcherTaskTrigger {}
impl IContentPrefetcherTaskTrigger_Vtbl {
    pub const fn new<Identity: IContentPrefetcherTaskTrigger_Impl, const OFFSET: isize>() -> IContentPrefetcherTaskTrigger_Vtbl {
        unsafe extern "system" fn TriggerContentPrefetcherTask<Identity: IContentPrefetcherTaskTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefullname: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IContentPrefetcherTaskTrigger_Impl::TriggerContentPrefetcherTask(this, core::mem::transmute(&packagefullname)).into()
        }
        unsafe extern "system" fn IsRegisteredForContentPrefetch<Identity: IContentPrefetcherTaskTrigger_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packagefullname: windows_core::PCWSTR, isregistered: *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContentPrefetcherTaskTrigger_Impl::IsRegisteredForContentPrefetch(this, core::mem::transmute(&packagefullname)) {
                Ok(ok__) => {
                    isregistered.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContentPrefetcherTaskTrigger, OFFSET>(),
            TriggerContentPrefetcherTask: TriggerContentPrefetcherTask::<Identity, OFFSET>,
            IsRegisteredForContentPrefetch: IsRegisteredForContentPrefetch::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContentPrefetcherTaskTrigger as windows_core::Interface>::IID
    }
}
