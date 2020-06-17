use crate::windows::application_model::activation::*;
use crate::windows::ui::xaml::controls::TextBlock;
use crate::windows::ui::xaml::*;
use std::ptr::NonNull;
use winrt::*;

#[derive(Default, Clone, PartialEq)]
#[repr(transparent)]
pub struct App {
    ptr: ComPtr<App>,
}

impl App {
    pub fn new() -> Result<Self> {
        impl_App::new()
    }
}

unsafe impl ComInterface for App {
    type VTable = abi_IApplicationOverride;
    fn iid() -> Guid {
        Guid::from_values(
            2426272436,
            15997,
            28479,
            [184, 119, 248, 40, 145, 234, 89, 75],
        )
    }
}

#[repr(C)]
pub struct abi_IUnknown {
    pub unknown_query_interface:
        extern "system" fn(NonNullRawComPtr<IUnknown>, &Guid, *mut RawPtr) -> ErrorCode,
    pub unknown_add_ref: extern "system" fn(NonNullRawComPtr<IUnknown>) -> u32,
    pub unknown_release: extern "system" fn(NonNullRawComPtr<IUnknown>) -> u32,
}
#[repr(C)]
pub struct abi_IInspectable {
    iunknown: abi_IUnknown,

    pub inspectable_iids:
        unsafe extern "system" fn(NonNullRawComPtr<Object>, *mut u32, *mut *mut Guid) -> ErrorCode,
    pub inspectable_type_name: unsafe extern "system" fn(
        NonNullRawComPtr<Object>,
        *mut <HString as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub inspectable_trust_level:
        unsafe extern "system" fn(NonNullRawComPtr<Object>, *mut i32) -> ErrorCode,
}
#[repr(C)]
pub struct abi_IApplicationOverride {
    iinspectable: abi_IInspectable,
    pub on_activated:
        extern "system" fn(NonNullRawComPtr<IUnknown>, RawComPtr<IActivatedEventArgs>) -> ErrorCode,
    pub on_launched: extern "system" fn(
        NonNullRawComPtr<IUnknown>,
        RawComPtr<LaunchActivatedEventArgs>,
    ) -> ErrorCode,
    pub on_file_activated: extern "system" fn(
        NonNullRawComPtr<IUnknown>,
        RawComPtr<FileActivatedEventArgs>,
    ) -> ErrorCode,
    pub on_search_activated: extern "system" fn(
        NonNullRawComPtr<IUnknown>,
        RawComPtr<SearchActivatedEventArgs>,
    ) -> ErrorCode,
    pub on_share_target_activated: extern "system" fn(
        NonNullRawComPtr<IUnknown>,
        RawComPtr<ShareTargetActivatedEventArgs>,
    ) -> ErrorCode,
    pub on_file_open_picker_activated: extern "system" fn(
        NonNullRawComPtr<IUnknown>,
        RawComPtr<FileOpenPickerActivatedEventArgs>,
    ) -> ErrorCode,
    pub on_file_save_picker_activated: extern "system" fn(
        NonNullRawComPtr<IUnknown>,
        RawComPtr<FileSavePickerActivatedEventArgs>,
    ) -> ErrorCode,
    pub on_cached_file_updater_activated: extern "system" fn(
        NonNullRawComPtr<IUnknown>,
        RawComPtr<CachedFileUpdaterActivatedEventArgs>,
    ) -> ErrorCode,
    pub on_window_created: extern "system" fn(
        NonNullRawComPtr<IUnknown>,
        RawComPtr<IWindowCreatedEventArgs>,
    ) -> ErrorCode,
}

unsafe impl AbiTransferable for App {
    type Abi = RawComPtr<Self>;
    fn get_abi(&self) -> Self::Abi {
        <ComPtr<Self> as AbiTransferable>::get_abi(&self.ptr)
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        <ComPtr<Self> as AbiTransferable>::set_abi(&mut self.ptr)
    }
}

#[repr(C)]
struct impl_App {
    vtable: *const abi_IApplicationOverride,
    count: RefCount,
    base: IApplicationOverrides,
}

impl impl_App {
    const VTABLE: abi_IApplicationOverride = abi_IApplicationOverride {
        iinspectable: abi_IInspectable {
            iunknown: abi_IUnknown {
                unknown_query_interface: impl_App::unknown_query_interface,
                unknown_add_ref: impl_App::unknown_add_ref,
                unknown_release: impl_App::unknown_release,
            },
            inspectable_iids: impl_App::inspectable_iids,
            inspectable_type_name: impl_App::inspectable_type_name,
            inspectable_trust_level: impl_App::inspectable_trust_level,
        },
        on_activated: impl_App::on_activated,
        on_launched: impl_App::on_launched,
        on_file_activated: impl_App::on_file_activated,
        on_search_activated: impl_App::on_search_activated,
        on_share_target_activated: impl_App::on_share_target_activated,
        on_file_open_picker_activated: impl_App::on_file_open_picker_activated,
        on_file_save_picker_activated: impl_App::on_file_save_picker_activated,
        on_cached_file_updater_activated: impl_App::on_cached_file_updater_activated,
        on_window_created: impl_App::on_window_created,
    };

    fn new() -> Result<App> {
        let value = Self {
            vtable: &Self::VTABLE,
            count: RefCount::new(),
            base: Default::default(),
        };
        unsafe {
            // Initialize impl_App and App
            let mut result: App = std::mem::zeroed();
            let mut ptr: NonNull<Self> = NonNull::new_unchecked(Box::into_raw(Box::new(value)));
            *<App as AbiTransferable>::set_abi(&mut result) =
                Some(NonNullRawComPtr::new(ptr.cast()));

            // Set base
            let mut inner = Default::default();
            winrt::factory::<Application, IApplicationFactory>()?
                .create_instance(result.query::<Object>(), &mut inner)?;
            // TODO: safe to hold an exclusive reference to ptr?
            ptr.as_mut().base = inner.query::<IApplicationOverrides>();

            // WARNING: This is only a trick to prevent the instances from
            // releasing too early.
            std::mem::forget(inner);

            Ok(result)
        }
    }

    extern "system" fn unknown_query_interface(
        this: NonNullRawComPtr<IUnknown>,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> ErrorCode {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            if iid == &<App as ComInterface>::iid()
                || iid == &<IUnknown as ComInterface>::iid()
                || iid == &<Object as ComInterface>::iid()
                || iid == &<IAgileObject as ComInterface>::iid()
                || iid == &<IApplicationOverrides as ComInterface>::iid()
            {
                *interface = this as RawPtr;
                (*this).count.add_ref();
                return ErrorCode(0);
            }
            *interface = std::ptr::null_mut();
            ErrorCode(0x80004002)
        }
    }
    extern "system" fn unknown_add_ref(this: NonNullRawComPtr<IUnknown>) -> u32 {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            (*this).count.add_ref()
        }
    }
    extern "system" fn unknown_release(this: NonNullRawComPtr<IUnknown>) -> u32 {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            let remaining = (*this).count.release();
            if remaining == 0 {
                Box::from_raw(this);
            }
            remaining
        }
    }
    extern "system" fn inspectable_iids(
        _this: NonNullRawComPtr<Object>,
        _iidcount: *mut u32,
        _iids: *mut *mut Guid,
    ) -> ErrorCode {
        ErrorCode(0x80004001)
    }
    extern "system" fn inspectable_type_name(
        _this: NonNullRawComPtr<Object>,
        _class_name: *mut <HString as AbiTransferable>::Abi,
    ) -> ErrorCode {
        ErrorCode(0x80004001)
    }
    extern "system" fn inspectable_trust_level(
        _this: NonNullRawComPtr<Object>,
        _trust_level: *mut i32,
    ) -> ErrorCode {
        ErrorCode(0x80004001)
    }
    extern "system" fn on_activated(
        _this: NonNullRawComPtr<IUnknown>,
        _: RawComPtr<IActivatedEventArgs>,
    ) -> ErrorCode {
        ErrorCode(0x80004001)
    }
    fn on_launch_callback() -> Result<()> {
        let frame = TextBlock::new()?;
        frame.set_text(format!("Hello Rust from UWP!"))?;
        let win = Window::current()?;
        win.set_content(frame.query::<UIElement>())?;
        win.activate()?;
        Ok(())
    }
    extern "system" fn on_launched(
        _this: NonNullRawComPtr<IUnknown>,
        _: RawComPtr<LaunchActivatedEventArgs>,
    ) -> ErrorCode {
        match Self::on_launch_callback() {
            Ok(()) => ErrorCode(0),
            Err(c) => c.code(),
        }
    }
    extern "system" fn on_file_activated(
        _this: NonNullRawComPtr<IUnknown>,
        _: RawComPtr<FileActivatedEventArgs>,
    ) -> ErrorCode {
        ErrorCode(0x80004001)
    }
    extern "system" fn on_search_activated(
        _this: NonNullRawComPtr<IUnknown>,
        _: RawComPtr<SearchActivatedEventArgs>,
    ) -> ErrorCode {
        ErrorCode(0x80004001)
    }
    extern "system" fn on_share_target_activated(
        _this: NonNullRawComPtr<IUnknown>,
        _: RawComPtr<ShareTargetActivatedEventArgs>,
    ) -> ErrorCode {
        ErrorCode(0x80004001)
    }
    extern "system" fn on_file_open_picker_activated(
        _this: NonNullRawComPtr<IUnknown>,
        _: RawComPtr<FileOpenPickerActivatedEventArgs>,
    ) -> ErrorCode {
        ErrorCode(0x80004001)
    }
    extern "system" fn on_file_save_picker_activated(
        _this: NonNullRawComPtr<IUnknown>,
        _: RawComPtr<FileSavePickerActivatedEventArgs>,
    ) -> ErrorCode {
        ErrorCode(0x80004001)
    }
    extern "system" fn on_cached_file_updater_activated(
        _this: NonNullRawComPtr<IUnknown>,
        _: RawComPtr<CachedFileUpdaterActivatedEventArgs>,
    ) -> ErrorCode {
        ErrorCode(0x80004001)
    }
    extern "system" fn on_window_created(
        this: NonNullRawComPtr<IUnknown>,
        args: RawComPtr<IWindowCreatedEventArgs>,
    ) -> ErrorCode {
        let this = this.as_raw() as *mut Self;
        unsafe {
            let this = &*this;
            this.base.get_abi().map_or(ErrorCode(0x80004002), |abi| {
                (abi.vtable().on_window_created)(abi, args)
                    .ok()
                    .map_or_else(|e| e.code(), |()| ErrorCode(0))
            })
        }
    }
}
