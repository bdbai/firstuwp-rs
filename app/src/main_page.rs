use crate::abi::*;
use crate::weak_ref::WeakRefObject;
use bindings::windows::foundation::{Point, Uri};
use bindings::windows::system::Launcher;
use bindings::windows::ui::xaml::controls::primitives::ComponentResourceLocation;
use bindings::windows::ui::xaml::controls::{
    GridViewItem, IPageFactory, IPageOverrides, Image, Page,
};
use bindings::windows::ui::xaml::input::{
    PointerEventHandler, PointerRoutedEventArgs, TappedEventHandler,
};
use bindings::windows::ui::xaml::markup::IComponentConnector;
use bindings::windows::ui::xaml::media::animation::Storyboard;
use bindings::windows::ui::xaml::navigation::{NavigatingCancelEventArgs, NavigationEventArgs};
use bindings::windows::ui::xaml::Application;
use std::mem::size_of;
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;
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
    type VTable = abi_IMainPage;
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
pub struct abi_IMainPage {
    page: abi_IPageOverrides,
    component_connector: abi_IComponentConnector,
}

#[repr(C)]
struct impl_MainPage {
    vtable: *const abi_IPageOverrides,
    vtable_component_connector: *const abi_IComponentConnector,
    count: AtomicUsize,
    base: Object,
    happy_ferris_expand_storyboard: Storyboard,
    happy_ferris_collapse_storyboard: Storyboard,
    happy_ferris_image: Image,
}

impl WeakRefObject for impl_MainPage {
    fn get_reference(&self) -> &AtomicUsize {
        &self.count
    }
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
    const VTABLE_COMPONENT_CONNECTOR: abi_IComponentConnector = abi_IComponentConnector {
        iinspectable: abi_IInspectable {
            iunknown: abi_IUnknown {
                unknown_query_interface: impl_MainPage::component_connector_unknown_query_interface,
                unknown_add_ref: impl_MainPage::component_connector_unknown_add_ref,
                unknown_release: impl_MainPage::component_connector_unknown_release,
            },
            inspectable_iids: impl_MainPage::component_connector_inspectable_iids,
            inspectable_type_name: impl_MainPage::component_connector_inspectable_type_name,
            inspectable_trust_level: impl_MainPage::component_connector_inspectable_trust_level,
        },
        connect: impl_MainPage::component_connector_connect,
    };

    fn new() -> Result<MainPage> {
        let value = Self {
            vtable: &Self::VTABLE,
            vtable_component_connector: &Self::VTABLE_COMPONENT_CONNECTOR,
            count: AtomicUsize::new(1),
            base: Default::default(),
            happy_ferris_expand_storyboard: Storyboard::default(),
            happy_ferris_collapse_storyboard: Storyboard::default(),
            happy_ferris_image: Image::default(),
        };
        unsafe {
            // Initialize impl_MainPage and MainPage
            let mut result: MainPage = std::mem::zeroed();
            let mut ptr: NonNull<Self> = NonNull::new_unchecked(Box::into_raw(Box::new(value)));
            *<MainPage as AbiTransferable>::set_abi(&mut result) =
                Some(NonNullRawComPtr::new(ptr.cast()));

            // Set base
            let mut inner = Default::default();
            winrt::factory::<Page, IPageFactory>()?
                .create_instance(result.query::<Object>(), &mut inner)?;
            (*ptr.as_mut()).base = inner;

            // Load XAML
            let resource_locator = Uri::create_uri("ms-appx:///MainPage.xaml")?;
            Application::load_component_with_resource_location(
                result.query::<Object>(),
                resource_locator,
                ComponentResourceLocation::Application,
            )?;

            Ok(result)
        }
    }

    unsafe extern "system" fn unknown_query_interface(
        this: NonNullRawComPtr<IUnknown>,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> ErrorCode {
        let this: *mut Self = this.as_raw() as _;
        let interface = interface.as_mut().unwrap();
        if iid == &<MainPage as ComInterface>::iid()
            || iid == &<IUnknown as ComInterface>::iid()
            || iid == &<Object as ComInterface>::iid()
            || iid == &<IPageOverrides as ComInterface>::iid()
        {
            *interface = this as *mut Self as *mut _;
            (*this).add_ref();
            return ErrorCode(0);
        }
        if iid == &<IComponentConnector as ComInterface>::iid() {
            *interface = &mut (*this).vtable_component_connector as *mut _ as *mut _;
            (*this).add_ref();
            return ErrorCode(0);
        }

        *interface = std::ptr::null_mut();
        if iid == &Guid::from("00000038-0000-0000-C000-000000000046") {
            *interface = (*this).make_weak_ref(NonNullRawComPtr::new(NonNull::new_unchecked(
                this as *mut _,
            ))) as *mut _;
            (*this).add_ref();
            return ErrorCode(0);
        }
        (*this)
            .base
            .raw_query::<Object>(iid, std::mem::transmute(interface as *mut _));
        if interface.is_null() {
            ErrorCode(0x80004002)
        } else {
            ErrorCode(0)
        }
    }
    extern "system" fn unknown_add_ref(this: NonNullRawComPtr<IUnknown>) -> u32 {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            (*this).add_ref()
        }
    }
    extern "system" fn unknown_release(this: NonNullRawComPtr<IUnknown>) -> u32 {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            let remaining = (*this).release();
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

    extern "system" fn component_connector_unknown_query_interface(
        this: NonNullRawComPtr<IUnknown>,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> ErrorCode {
        let this = NonNullRawComPtr::new(unsafe {
            NonNull::new_unchecked(
                (this.as_raw() as usize - size_of::<*const abi_IPageOverrides>()) as *mut _,
            )
        });
        unsafe { Self::unknown_query_interface(this, iid, interface) }
    }
    extern "system" fn component_connector_unknown_add_ref(
        this: NonNullRawComPtr<IUnknown>,
    ) -> u32 {
        let this = NonNullRawComPtr::new(unsafe {
            NonNull::new_unchecked(
                (this.as_raw() as usize - size_of::<*const abi_IPageOverrides>()) as *mut _,
            )
        });
        Self::unknown_add_ref(this)
    }
    extern "system" fn component_connector_unknown_release(
        this: NonNullRawComPtr<IUnknown>,
    ) -> u32 {
        let this = NonNullRawComPtr::new(unsafe {
            NonNull::new_unchecked(
                (this.as_raw() as usize - size_of::<*const abi_IPageOverrides>()) as *mut _,
            )
        });
        Self::unknown_release(this)
    }
    extern "system" fn component_connector_inspectable_iids(
        this: NonNullRawComPtr<Object>,
        iidcount: *mut u32,
        iids: *mut *mut Guid,
    ) -> ErrorCode {
        let this = NonNullRawComPtr::new(unsafe {
            NonNull::new_unchecked(
                (this.as_raw() as usize - size_of::<*const abi_IPageOverrides>()) as *mut _,
            )
        });
        Self::inspectable_iids(this, iidcount, iids)
    }
    extern "system" fn component_connector_inspectable_type_name(
        this: NonNullRawComPtr<Object>,
        class_name: *mut <HString as AbiTransferable>::Abi,
    ) -> ErrorCode {
        let this = NonNullRawComPtr::new(unsafe {
            NonNull::new_unchecked(
                (this.as_raw() as usize - size_of::<*const abi_IPageOverrides>()) as *mut _,
            )
        });
        Self::inspectable_type_name(this, class_name)
    }
    extern "system" fn component_connector_inspectable_trust_level(
        this: NonNullRawComPtr<Object>,
        trust_level: *mut i32,
    ) -> ErrorCode {
        let this = NonNullRawComPtr::new(unsafe {
            NonNull::new_unchecked(
                (this.as_raw() as usize - size_of::<*const abi_IPageOverrides>()) as *mut _,
            )
        });
        Self::inspectable_trust_level(this, trust_level)
    }
    extern "system" fn component_connector_connect(
        this: NonNullRawComPtr<IComponentConnector>,
        connect_id: i32,
        object: <Object as AbiTransferable>::Abi,
    ) -> ErrorCode {
        let object = object.unwrap();
        let this_raw: *mut Self = unsafe {
            &mut *((this.as_raw() as usize - size_of::<*const abi_IPageOverrides>()) as *mut _)
        };
        unsafe {
            let this_ptr = std::mem::transmute(this_raw);
            let this = &mut *this_raw;
            match connect_id {
                2 => this.happy_ferris_expand_storyboard = object.query(),
                3 => this.happy_ferris_collapse_storyboard = object.query(),
                6 => {
                    let object = object.query::<Image>();
                    this.happy_ferris_image = object.clone();
                    let weak = this.get_weak(this_ptr);
                    if let Err(e) =
                        object.pointer_entered(PointerEventHandler::new(move |sender, e| {
                            let this = Self::from_weak::<MainPage>(weak);
                            match this.get_abi() {
                                Some(ptr) => Self::happy_ferris_pointer_entered(
                                    &mut *(ptr.as_raw() as *mut _),
                                    sender,
                                    e,
                                ),
                                None => Ok(()),
                            }
                        }))
                    {
                        return e.code();
                    }
                    if let Err(e) =
                        object.pointer_exited(PointerEventHandler::new(move |sender, e| {
                            let this = Self::from_weak::<MainPage>(weak);
                            match this.get_abi() {
                                Some(ptr) => Self::happy_ferris_pointer_exited(
                                    &mut *(ptr.as_raw() as *mut _),
                                    sender,
                                    e,
                                ),
                                None => Ok(()),
                            }
                        }))
                    {
                        return e.code();
                    };
                }
                7 => {
                    if let Err(e) = Self::set_link_item_click_handler(
                        object,
                        "https://www.rust-lang.org/learn/get-started",
                    ) {
                        return e.code();
                    }
                }
                8 => {
                    if let Err(e) =
                        Self::set_link_item_click_handler(object, "https://www.rust-lang.org/learn")
                    {
                        return e.code();
                    }
                }
                9 => {
                    if let Err(e) =
                        Self::set_link_item_click_handler(object, "https://play.rust-lang.org/")
                    {
                        return e.code();
                    }
                }
                _ => {}
            }
        }
        ErrorCode(0)
    }

    fn happy_ferris_pointer_entered(
        &mut self,
        _sender: &Object,
        _e: &PointerRoutedEventArgs,
    ) -> Result<()> {
        self.happy_ferris_image
            .set_render_transform_origin(Point { x: 1f32, y: 1f32 })?;
        self.happy_ferris_expand_storyboard.begin()
    }

    fn happy_ferris_pointer_exited(
        &mut self,
        _sender: &Object,
        _e: &PointerRoutedEventArgs,
    ) -> Result<()> {
        self.happy_ferris_image
            .set_render_transform_origin(Point { x: 0f32, y: 1f32 })?;
        self.happy_ferris_collapse_storyboard.begin()
    }

    fn set_link_item_click_handler(
        object: NonNullRawComPtr<Object>,
        link: &'static str,
    ) -> Result<()> {
        let object = object.query::<GridViewItem>();
        object.tapped(TappedEventHandler::new(move |_sender, _e| {
            let uri = Uri::create_uri(link)?;
            let _ = Launcher::launch_uri_async(uri)?;
            Ok(())
        }))?;
        Ok(())
    }
}
