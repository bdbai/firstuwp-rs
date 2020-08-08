use crate::abi::*;
use crate::xaml_system_base_type::XamlSystemBaseType;
use bindings::windows::ui::xaml::interop::{TypeKind, TypeName};
use bindings::windows::ui::xaml::markup::{IXamlMember, IXamlType};
use std::ptr::NonNull;
use winrt::*;

#[repr(transparent)]
pub struct XamlUserType {
    ptr: ComPtr<XamlUserType>,
}

impl XamlUserType {
    pub fn new(type_name: &'static str) -> Result<XamlUserType> {
        impl_XamlUserType::new(type_name)
    }
}

unsafe impl ComInterface for XamlUserType {
    type VTable = abi_IXamlType;
    fn iid() -> Guid {
        Guid::from_values(
            0x945350b1,
            0x1483,
            0x4780,
            [0xa0, 0xd3, 0x52, 0x96, 0x3a, 0x3a, 0x06, 0x8a],
        )
    }
}

unsafe impl AbiTransferable for XamlUserType {
    type Abi = RawComPtr<Self>;
    fn get_abi(&self) -> Self::Abi {
        <ComPtr<Self> as AbiTransferable>::get_abi(&self.ptr)
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        <ComPtr<Self> as AbiTransferable>::set_abi(&mut self.ptr)
    }
}

#[repr(C)]
struct impl_XamlUserType {
    vtable: *const abi_IXamlType,
    count: RefCount,
    full_name: &'static str,
}

impl impl_XamlUserType {
    const VTABLE: abi_IXamlType = abi_IXamlType {
        iinspectable: abi_IInspectable {
            iunknown: abi_IUnknown {
                unknown_query_interface: impl_XamlUserType::unknown_query_interface,
                unknown_add_ref: impl_XamlUserType::unknown_add_ref,
                unknown_release: impl_XamlUserType::unknown_release,
            },
            inspectable_iids: impl_XamlUserType::inspectable_iids,
            inspectable_type_name: impl_XamlUserType::inspectable_type_name,
            inspectable_trust_level: impl_XamlUserType::inspectable_trust_level,
        },
        base_type: impl_XamlUserType::base_type,
        content_property: impl_XamlUserType::content_property,
        full_name: impl_XamlUserType::full_name,
        is_array: impl_XamlUserType::is_array,
        is_collection: impl_XamlUserType::is_collection,
        is_constructible: impl_XamlUserType::is_constructible,
        is_dictionary: impl_XamlUserType::is_dictionary,
        is_markup_extension: impl_XamlUserType::is_markup_extension,
        is_bindable: impl_XamlUserType::is_bindable,
        item_type: impl_XamlUserType::item_type,
        key_type: impl_XamlUserType::key_type,
        underlying_type: impl_XamlUserType::underlying_type,
        activate_instance: impl_XamlUserType::activate_instance,
        create_from_string: impl_XamlUserType::create_from_string,
        get_member: impl_XamlUserType::get_member,
        add_to_vector: impl_XamlUserType::add_to_vector,
        add_to_map: impl_XamlUserType::add_to_map,
        run_initializer: impl_XamlUserType::run_initializer,
    };
    fn new(full_name: &'static str) -> Result<XamlUserType> {
        let value = Self {
            vtable: &Self::VTABLE,
            count: RefCount::new(),
            full_name,
        };
        unsafe {
            let mut result = std::mem::zeroed();
            let ptr: NonNull<Self> = NonNull::new_unchecked(Box::into_raw(Box::new(value)));
            *<XamlUserType as AbiTransferable>::set_abi(&mut result) =
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
        if iid == &<XamlUserType as ComInterface>::iid()
            || iid == &<IUnknown as ComInterface>::iid()
            || iid == &<Object as ComInterface>::iid()
            || iid == &<IXamlType as ComInterface>::iid()
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
        _class_name: *mut <HString as AbiTransferable>::Abi,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }
    extern "system" fn inspectable_trust_level(
        _this: NonNullRawComPtr<Object>,
        _trust_level: *mut i32,
    ) -> ErrorCode {
        loop {}
    }
    fn base_type_impl(&self) -> Result<IXamlType> {
        if self.full_name == "FirstUwp.MainPage" {
            Ok(XamlSystemBaseType::new("Windows.UI.Xaml.Controls.Page")?.query())
        } else {
            Ok(IXamlType::default())
        }
    }
    extern "system" fn base_type(
        this: NonNullRawComPtr<IXamlType>,
        result: *mut <IXamlType as AbiTransferable>::Abi,
    ) -> ErrorCode {
        let this: &mut Self = unsafe { std::mem::transmute(this) };
        match this.base_type_impl() {
            Ok(obj) => (unsafe { *result = obj.into_abi() }, ErrorCode(0)).1,
            Err(e) => e.code(),
        }
    }
    extern "system" fn content_property(
        _this: NonNullRawComPtr<IXamlType>,
        result: *mut <IXamlMember as AbiTransferable>::Abi,
    ) -> ErrorCode {
        unsafe { *result = IXamlMember::default().into_abi() };
        ErrorCode(0)
    }
    extern "system" fn full_name(
        this: NonNullRawComPtr<IXamlType>,
        result: *mut <HString as AbiTransferable>::Abi,
    ) -> ErrorCode {
        let this: &mut Self = unsafe { std::mem::transmute(this) };
        unsafe { *result = HString::from(this.full_name).into_abi() };
        ErrorCode(0)
    }
    extern "system" fn is_array(
        _this: NonNullRawComPtr<IXamlType>,
        result: *mut bool,
    ) -> ErrorCode {
        unsafe { *result = false };
        ErrorCode(0)
    }

    extern "system" fn is_collection(
        _this: NonNullRawComPtr<IXamlType>,
        result: *mut bool,
    ) -> ErrorCode {
        unsafe { *result = false };
        ErrorCode(0)
    }

    extern "system" fn is_constructible(
        _this: NonNullRawComPtr<IXamlType>,
        result: *mut bool,
    ) -> ErrorCode {
        unsafe { *result = true };
        ErrorCode(0)
    }

    extern "system" fn is_dictionary(
        _this: NonNullRawComPtr<IXamlType>,
        result: *mut bool,
    ) -> ErrorCode {
        unsafe { *result = false };
        ErrorCode(0)
    }

    extern "system" fn is_markup_extension(
        _this: NonNullRawComPtr<IXamlType>,
        result: *mut bool,
    ) -> ErrorCode {
        unsafe { *result = false };
        ErrorCode(0)
    }
    extern "system" fn is_bindable(
        _this: NonNullRawComPtr<IXamlType>,
        result: *mut bool,
    ) -> ErrorCode {
        unsafe { *result = false };
        ErrorCode(0)
    }
    extern "system" fn item_type(
        _this: NonNullRawComPtr<IXamlType>,
        _result: *mut <IXamlType as AbiTransferable>::Abi,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }
    extern "system" fn key_type(
        _this: NonNullRawComPtr<IXamlType>,
        _result: *mut <IXamlType as AbiTransferable>::Abi,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }
    extern "system" fn underlying_type(
        this: NonNullRawComPtr<IXamlType>,
        result: *mut <TypeName as AbiTransferable>::Abi,
    ) -> ErrorCode {
        let this: &mut Self = unsafe { std::mem::transmute(this) };
        let type_name = TypeName {
            name: HString::from(this.full_name),
            kind: TypeKind::Custom,
        };
        unsafe { *result = type_name.into_abi() };
        ErrorCode(0)
    }
    fn activate_instance_impl(&self) -> Result<Object> {
        if self.full_name == "FirstUwp.MainPage" {
            use crate::main_page::MainPage;
            Ok(MainPage::new()?.query())
        } else {
            Ok(Object::default())
        }
    }
    extern "system" fn activate_instance(
        this: NonNullRawComPtr<IXamlType>,
        result: *mut <Object as AbiTransferable>::Abi,
    ) -> ErrorCode {
        let this: &mut Self = unsafe { std::mem::transmute(this) };
        match this.activate_instance_impl() {
            Ok(obj) => (unsafe { *result = obj.into_abi() }, ErrorCode(0)).1,
            Err(e) => e.code(),
        }
    }
    extern "system" fn create_from_string(
        _this: NonNullRawComPtr<IXamlType>,
        _value: <HString as AbiTransferable>::Abi,
        _result: *mut <Object as AbiTransferable>::Abi,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }
    extern "system" fn get_member(
        _this: NonNullRawComPtr<IXamlType>,
        _name: <HString as AbiTransferable>::Abi,
        _result: *mut <IXamlMember as AbiTransferable>::Abi,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }
    extern "system" fn add_to_vector(
        _this: NonNullRawComPtr<IXamlType>,
        _instance: <Object as AbiTransferable>::Abi,
        _value: <Object as AbiTransferable>::Abi,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }
    extern "system" fn add_to_map(
        _this: NonNullRawComPtr<IXamlType>,
        _instance: <Object as AbiTransferable>::Abi,
        _key: <Object as AbiTransferable>::Abi,
        _value: <Object as AbiTransferable>::Abi,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }
    extern "system" fn run_initializer(_this: NonNullRawComPtr<IXamlType>) -> ErrorCode {
        ErrorCode(0x80004002)
    }
}
