use crate::abi::*;
use crate::xaml_metadata_provider::XamlMetadataProvider;
use bindings::windows::application_model::activation::*;
use bindings::windows::ui::xaml::controls::{Frame, IFrame2, IFrameFactory};
use bindings::windows::ui::xaml::interop::{TypeKind, TypeName};
use bindings::windows::ui::xaml::media::animation::NavigationTransitionInfo;
use bindings::windows::ui::xaml::*;
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
    base: Object,
    xaml_metadata_provider: XamlMetadataProvider,
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
            xaml_metadata_provider: XamlMetadataProvider::new()?,
        };
        unsafe {
            // Initialize impl_App and App
            let mut result = std::mem::zeroed();
            let mut ptr: NonNull<Self> = NonNull::new_unchecked(Box::into_raw(Box::new(value)));
            *<App as AbiTransferable>::set_abi(&mut result) =
                Some(NonNullRawComPtr::new(ptr.cast()));

            // Set base
            let mut inner = Default::default();
            winrt::factory::<Application, IApplicationFactory>()?
                .create_instance(result.query::<Object>(), &mut inner)?;
            ptr.as_mut().base = inner;

            Ok(result)
        }
    }

    extern "system" fn unknown_query_interface(
        this: NonNullRawComPtr<IUnknown>,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> ErrorCode {
        let interface = unsafe { interface.as_mut().unwrap() };
        let this: &mut Self = unsafe { std::mem::transmute(this) };
        if iid == &<App as ComInterface>::iid()
            || iid == &<IUnknown as ComInterface>::iid()
            || iid == &<Object as ComInterface>::iid()
            || iid == &<IAgileObject as ComInterface>::iid()
            || iid == &<IApplicationOverrides as ComInterface>::iid()
        {
            *interface = this as *mut Self as *mut _;
            this.count.add_ref();
            return ErrorCode(0);
        }
        *interface = std::ptr::null_mut();
        unsafe {
            this.base
                .raw_query::<Object>(iid, std::mem::transmute(interface as *mut _));
        }
        if !interface.is_null() {
            return ErrorCode(0);
        }
        unsafe {
            this.xaml_metadata_provider
                .raw_query::<Object>(iid, std::mem::transmute(interface as *mut _));
        }
        if interface.is_null() {
            ErrorCode(0x80004002)
        } else {
            ErrorCode(0)
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
        let mut frame = Default::default();
        winrt::factory::<Frame, IFrameFactory>()?.create_instance(Object::default(), &mut frame)?;
        frame.query::<IFrame2>().navigate(
            TypeName {
                kind: TypeKind::Custom,
                name: "FirstUwp.MainPage".into(),
            },
            Object::default(),
            NavigationTransitionInfo::default(),
        )?;
        let content = frame.query::<UIElement>();
        let win = Window::current()?;
        win.set_content(content)?;
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
            this.base.query::<IApplicationOverrides>().get_abi().map_or(
                ErrorCode(0x80004002),
                |abi| {
                    (abi.vtable().on_window_created)(abi, args)
                        .ok()
                        .map_or_else(|e| e.code(), |()| ErrorCode(0))
                },
            )
        }
    }
}
