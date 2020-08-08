use crate::abi::*;
use crate::xaml_user_type::XamlUserType;
use bindings::windows::ui::xaml::interop::TypeName;
use bindings::windows::ui::xaml::markup::{IXamlMetadataProvider, IXamlType, XmlnsDefinition};
use std::ptr::NonNull;
use winrt::*;

#[repr(transparent)]
pub struct XamlMetadataProvider {
    ptr: ComPtr<XamlMetadataProvider>,
}

impl XamlMetadataProvider {
    pub fn new() -> Result<XamlMetadataProvider> {
        impl_XamlMetadataProvider::new()
    }
}

unsafe impl ComInterface for XamlMetadataProvider {
    type VTable = abi_IXamlMetadataProvider;
    fn iid() -> Guid {
        Guid::from_values(
            0x1d6a7d3a,
            0x4746,
            0x42ab,
            [0x9c, 0x4d, 0x25, 0xf1, 0xba, 0xdd, 0xb7, 0xd8],
        )
    }
}

unsafe impl AbiTransferable for XamlMetadataProvider {
    type Abi = RawComPtr<Self>;
    fn get_abi(&self) -> Self::Abi {
        <ComPtr<Self> as AbiTransferable>::get_abi(&self.ptr)
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        <ComPtr<Self> as AbiTransferable>::set_abi(&mut self.ptr)
    }
}

#[repr(C)]
struct impl_XamlMetadataProvider {
    vtable: *const abi_IXamlMetadataProvider,
    count: RefCount,
}

impl impl_XamlMetadataProvider {
    const VTABLE: abi_IXamlMetadataProvider = abi_IXamlMetadataProvider {
        iinspectable: abi_IInspectable {
            iunknown: abi_IUnknown {
                unknown_query_interface: impl_XamlMetadataProvider::unknown_query_interface,
                unknown_add_ref: impl_XamlMetadataProvider::unknown_add_ref,
                unknown_release: impl_XamlMetadataProvider::unknown_release,
            },
            inspectable_iids: impl_XamlMetadataProvider::inspectable_iids,
            inspectable_type_name: impl_XamlMetadataProvider::inspectable_type_name,
            inspectable_trust_level: impl_XamlMetadataProvider::inspectable_trust_level,
        },
        get_xaml_type: impl_XamlMetadataProvider::get_xaml_type,
        get_xaml_type_by_full_name: impl_XamlMetadataProvider::get_xaml_type_by_full_name,
        get_xmlns_definitions: impl_XamlMetadataProvider::get_xmlns_definitions,
    };

    fn new() -> Result<XamlMetadataProvider> {
        let value = Self {
            vtable: &Self::VTABLE,
            count: RefCount::new(),
        };
        unsafe {
            let mut result = std::mem::zeroed();
            let ptr: NonNull<Self> = NonNull::new_unchecked(Box::into_raw(Box::new(value)));
            *<XamlMetadataProvider as AbiTransferable>::set_abi(&mut result) =
                Some(NonNullRawComPtr::new(ptr.cast()));

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
        if iid == &<XamlMetadataProvider as ComInterface>::iid()
            || iid == &<IUnknown as ComInterface>::iid()
            || iid == &<Object as ComInterface>::iid()
            || iid == &<IXamlMetadataProvider as ComInterface>::iid()
        {
            *interface = this as *mut Self as *mut _;
            this.count.add_ref();
            return ErrorCode(0);
        }

        *interface = std::ptr::null_mut();
        ErrorCode(0x80004002)
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
        let typename: HString = From::from("FirstUwp.XamlMetadataProvider");
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

    fn get_xaml_type_impl(&mut self, r#type: TypeName) -> Result<IXamlType> {
        Ok(match r#type.name.to_string().as_str() {
            "FirstUwp.MainPage" => XamlUserType::new("FirstUwp.MainPage")?.query(),
            _ => IXamlType::default(),
        })
    }

    extern "system" fn get_xaml_type(
        this: NonNullRawComPtr<IXamlMetadataProvider>,
        r#type: <TypeName as AbiTransferable>::Abi,
        result: *mut <IXamlType as AbiTransferable>::Abi,
    ) -> ErrorCode {
        let this: &mut Self = unsafe { std::mem::transmute(this) };
        match this.get_xaml_type_impl(r#type) {
            Ok(ret) => {
                unsafe { *result = ret.into_abi() };
                ErrorCode(0)
            }
            Err(e) => e.code(),
        }
    }
    extern "system" fn get_xaml_type_by_full_name(
        _this: NonNullRawComPtr<IXamlMetadataProvider>,
        _type: <HString as AbiTransferable>::Abi,
        result: *mut <IXamlType as AbiTransferable>::Abi,
    ) -> ErrorCode {
        unsafe { *result = IXamlType::default().into_abi() };
        ErrorCode(0)
    }
    extern "system" fn get_xmlns_definitions(
        _this: NonNullRawComPtr<IXamlMetadataProvider>,
        _result_size: *mut u32,
        _result: *mut *mut <XmlnsDefinition as AbiTransferable>::Abi,
    ) -> ErrorCode {
        todo!()
    }
}
