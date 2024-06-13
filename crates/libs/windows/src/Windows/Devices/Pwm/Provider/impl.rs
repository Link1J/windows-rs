pub trait IPwmControllerProvider_Impl: Sized {
    fn PinCount(&self) -> windows_core::Result<i32>;
    fn ActualFrequency(&self) -> windows_core::Result<f64>;
    fn SetDesiredFrequency(&self, frequency: f64) -> windows_core::Result<f64>;
    fn MaxFrequency(&self) -> windows_core::Result<f64>;
    fn MinFrequency(&self) -> windows_core::Result<f64>;
    fn AcquirePin(&self, pin: i32) -> windows_core::Result<()>;
    fn ReleasePin(&self, pin: i32) -> windows_core::Result<()>;
    fn EnablePin(&self, pin: i32) -> windows_core::Result<()>;
    fn DisablePin(&self, pin: i32) -> windows_core::Result<()>;
    fn SetPulseParameters(&self, pin: i32, dutycycle: f64, invertpolarity: bool) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IPwmControllerProvider {
    const NAME: &'static str = "Windows.Devices.Pwm.Provider.IPwmControllerProvider";
}
impl IPwmControllerProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPwmControllerProvider_Vtbl
    where
        Identity: IPwmControllerProvider_Impl,
    {
        unsafe extern "system" fn PinCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT
        where
            Identity: IPwmControllerProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPwmControllerProvider_Impl::PinCount(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActualFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT
        where
            Identity: IPwmControllerProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPwmControllerProvider_Impl::ActualFrequency(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDesiredFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, frequency: f64, result__: *mut f64) -> windows_core::HRESULT
        where
            Identity: IPwmControllerProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPwmControllerProvider_Impl::SetDesiredFrequency(this, frequency) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT
        where
            Identity: IPwmControllerProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPwmControllerProvider_Impl::MaxFrequency(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinFrequency<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT
        where
            Identity: IPwmControllerProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPwmControllerProvider_Impl::MinFrequency(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquirePin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32) -> windows_core::HRESULT
        where
            Identity: IPwmControllerProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPwmControllerProvider_Impl::AcquirePin(this, pin).into()
        }
        unsafe extern "system" fn ReleasePin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32) -> windows_core::HRESULT
        where
            Identity: IPwmControllerProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPwmControllerProvider_Impl::ReleasePin(this, pin).into()
        }
        unsafe extern "system" fn EnablePin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32) -> windows_core::HRESULT
        where
            Identity: IPwmControllerProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPwmControllerProvider_Impl::EnablePin(this, pin).into()
        }
        unsafe extern "system" fn DisablePin<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32) -> windows_core::HRESULT
        where
            Identity: IPwmControllerProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPwmControllerProvider_Impl::DisablePin(this, pin).into()
        }
        unsafe extern "system" fn SetPulseParameters<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: i32, dutycycle: f64, invertpolarity: bool) -> windows_core::HRESULT
        where
            Identity: IPwmControllerProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPwmControllerProvider_Impl::SetPulseParameters(this, pin, dutycycle, invertpolarity).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPwmControllerProvider, OFFSET>(),
            PinCount: PinCount::<Identity, OFFSET>,
            ActualFrequency: ActualFrequency::<Identity, OFFSET>,
            SetDesiredFrequency: SetDesiredFrequency::<Identity, OFFSET>,
            MaxFrequency: MaxFrequency::<Identity, OFFSET>,
            MinFrequency: MinFrequency::<Identity, OFFSET>,
            AcquirePin: AcquirePin::<Identity, OFFSET>,
            ReleasePin: ReleasePin::<Identity, OFFSET>,
            EnablePin: EnablePin::<Identity, OFFSET>,
            DisablePin: DisablePin::<Identity, OFFSET>,
            SetPulseParameters: SetPulseParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPwmControllerProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPwmProvider_Impl: Sized {
    fn GetControllers(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IVectorView<IPwmControllerProvider>>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IPwmProvider {
    const NAME: &'static str = "Windows.Devices.Pwm.Provider.IPwmProvider";
}
#[cfg(feature = "Foundation_Collections")]
impl IPwmProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPwmProvider_Vtbl
    where
        Identity: IPwmProvider_Impl,
    {
        unsafe extern "system" fn GetControllers<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPwmProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPwmProvider_Impl::GetControllers(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPwmProvider, OFFSET>(), GetControllers: GetControllers::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPwmProvider as windows_core::Interface>::IID
    }
}
