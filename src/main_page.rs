use crate::abi::*;
use crate::windows::foundation::PropertyValue;
use crate::windows::ui::xaml::controls::{Button, IButtonFactory};
use crate::windows::ui::xaml::controls::{IPageFactory, IPageOverrides, IUserControl, Page};
use crate::windows::ui::xaml::navigation::{NavigatingCancelEventArgs, NavigationEventArgs};
use crate::windows::ui::xaml::{RoutedEventHandler, UIElement};
use std::ptr::NonNull;
use winrt::*;

#[repr(transparent)]
pub struct MainPage {
    pub ptr: ComPtr<MainPage>,
}

impl MainPage {
    pub fn new() -> Result<MainPage> {
        impl_MainPage::new()
    }
}

unsafe impl ComInterface for MainPage {
    type VTable = abi_IPageOverrides;
    fn iid() -> Guid {
        Guid::from_values(
            0x00141d4c,
            0x7375,
            0x4638,
            [0xe8, 0x98, 0x44, 0x94, 0x4a, 0xc6, 0xa1, 0xf2],
        )
    }
}

unsafe impl AbiTransferable for MainPage {
    type Abi = RawComPtr<Self>;
    fn get_abi(&self) -> Self::Abi {
        <ComPtr<Self> as AbiTransferable>::get_abi(&self.ptr)
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        <ComPtr<Self> as AbiTransferable>::set_abi(&mut self.ptr)
    }
}

#[repr(C)]
struct impl_MainPage {
    vtable: *const abi_IPageOverrides,
    count: RefCount,
    base: Object,
}

impl impl_MainPage {
    const VTABLE: abi_IPageOverrides = abi_IPageOverrides {
        iinspectable: abi_IInspectable {
            iunknown: abi_IUnknown {
                unknown_query_interface: impl_MainPage::unknown_query_interface,
                unknown_add_ref: impl_MainPage::unknown_add_ref,
                unknown_release: impl_MainPage::unknown_release,
            },
            inspectable_iids: impl_MainPage::inspectable_iids,
            inspectable_type_name: impl_MainPage::inspectable_type_name,
            inspectable_trust_level: impl_MainPage::inspectable_trust_level,
        },
        on_navigated_from: impl_MainPage::on_navigated_from,
        on_navigated_to: impl_MainPage::on_navigated_to,
        on_navigating_from: impl_MainPage::on_navigating_from,
    };

    fn new() -> Result<MainPage> {
        let value = Self {
            vtable: &Self::VTABLE,
            count: RefCount::new(),
            base: Default::default(),
        };
        unsafe {
            // Initialize impl_MainPage and MainPage
            let mut result: MainPage = std::mem::zeroed();
            let mut ptr: NonNull<Self> = NonNull::new_unchecked(Box::into_raw(Box::new(value)));
            *<MainPage as AbiTransferable>::set_abi(&mut result) =
                Some(NonNullRawComPtr::new(ptr.cast()));

            // Construct inner content
            let mut content = Default::default();
            winrt::factory::<Button, IButtonFactory>()?
                .create_instance(Object::default(), &mut content)?;
            let button = content.query::<Button>();
            button.set_content(PropertyValue::create_string("Hello!")?)?;
            button.click(RoutedEventHandler::new({
                let mut clicked = 0;
                let button = button.clone();
                move |_sender, _e| {
                    clicked += 1;
                    button.set_content(PropertyValue::create_string(format!(
                        "You clicked the button {} times",
                        clicked
                    ))?)?;
                    Ok(())
                }
            }))?;
            let content = content.query::<UIElement>();

            // Set base
            let mut inner = Default::default();
            winrt::factory::<Page, IPageFactory>()?
                .create_instance(result.query::<Object>(), &mut inner)?;
            inner.query::<IUserControl>().set_content(content)?;
            (*ptr.as_mut()).base = inner;

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
        if iid == &<MainPage as ComInterface>::iid()
            || iid == &<IUnknown as ComInterface>::iid()
            || iid == &<Object as ComInterface>::iid()
            || iid == &<IPageOverrides as ComInterface>::iid()
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
        loop {}
    }
    extern "system" fn inspectable_type_name(
        _this: NonNullRawComPtr<Object>,
        class_name: *mut <HString as AbiTransferable>::Abi,
    ) -> ErrorCode {
        let typename: HString = From::from("FirstUwp.MainPage");
        unsafe {
            *class_name = typename.into_abi();
        }
        ErrorCode(0)
    }
    extern "system" fn inspectable_trust_level(
        _this: NonNullRawComPtr<Object>,
        _trust_level: *mut i32,
    ) -> ErrorCode {
        loop {}
    }
    extern "system" fn on_navigated_from(
        this: NonNullRawComPtr<IPageOverrides>,
        e: <NavigationEventArgs as AbiTransferable>::Abi,
    ) -> ErrorCode {
        unsafe {
            let this = this.as_raw() as *mut Self;
            (*this)
                .base
                .query::<IPageOverrides>()
                .get_abi()
                .map_or(ErrorCode(0x80004001), |abi| {
                    (abi.vtable().on_navigated_from)(abi, e)
                        .ok()
                        .map_or_else(|e| e.code(), |()| ErrorCode(0))
                })
        }
    }
    extern "system" fn on_navigated_to(
        this: NonNullRawComPtr<IPageOverrides>,
        e: <NavigationEventArgs as AbiTransferable>::Abi,
    ) -> ErrorCode {
        unsafe {
            let this = this.as_raw() as *mut Self;
            (*this)
                .base
                .query::<IPageOverrides>()
                .get_abi()
                .map_or(ErrorCode(0x80004001), |abi| {
                    (abi.vtable().on_navigated_to)(abi, e)
                        .ok()
                        .map_or_else(|e| e.code(), |()| ErrorCode(0))
                })
        }
    }
    extern "system" fn on_navigating_from(
        this: NonNullRawComPtr<IPageOverrides>,
        e: <NavigatingCancelEventArgs as AbiTransferable>::Abi,
    ) -> ErrorCode {
        unsafe {
            let this = this.as_raw() as *mut Self;
            (*this)
                .base
                .query::<IPageOverrides>()
                .get_abi()
                .map_or(ErrorCode(0x80004001), |abi| {
                    (abi.vtable().on_navigating_from)(abi, e)
                        .ok()
                        .map_or_else(|e| e.code(), |()| ErrorCode(0))
                })
        }
    }
}
