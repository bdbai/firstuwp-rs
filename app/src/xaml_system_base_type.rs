use crate::abi::*;
use bindings::windows::ui::xaml::interop::{TypeKind, TypeName};
use bindings::windows::ui::xaml::markup::{IXamlMember, IXamlType};
use std::ptr::NonNull;
use winrt::*;

#[repr(transparent)]
pub struct XamlSystemBaseType {
    ptr: ComPtr<XamlSystemBaseType>,
}

impl XamlSystemBaseType {
    pub fn new(type_name: &'static str) -> Result<XamlSystemBaseType> {
        impl_XamlSystemBaseType::new(type_name)
    }
}

unsafe impl ComInterface for XamlSystemBaseType {
    type VTable = abi_IXamlType;
    fn iid() -> Guid {
        Guid::from_values(
            0xc206b097,
            0xda07,
            0x4e2a,
            [0xa7, 0x01, 0x37, 0x38, 0xb0, 0x81, 0xff, 0x35],
        )
    }
}

unsafe impl AbiTransferable for XamlSystemBaseType {
    type Abi = RawComPtr<Self>;
    fn get_abi(&self) -> Self::Abi {
        <ComPtr<Self> as AbiTransferable>::get_abi(&self.ptr)
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        <ComPtr<Self> as AbiTransferable>::set_abi(&mut self.ptr)
    }
}

#[repr(C)]
struct impl_XamlSystemBaseType {
    vtable: *const abi_IXamlType,
    count: RefCount,
    full_name: &'static str,
}

impl impl_XamlSystemBaseType {
    const VTABLE: abi_IXamlType = abi_IXamlType {
        iinspectable: abi_IInspectable {
            iunknown: abi_IUnknown {
                unknown_query_interface: impl_XamlSystemBaseType::unknown_query_interface,
                unknown_add_ref: impl_XamlSystemBaseType::unknown_add_ref,
                unknown_release: impl_XamlSystemBaseType::unknown_release,
            },
            inspectable_iids: impl_XamlSystemBaseType::inspectable_iids,
            inspectable_type_name: impl_XamlSystemBaseType::inspectable_type_name,
            inspectable_trust_level: impl_XamlSystemBaseType::inspectable_trust_level,
        },
        base_type: impl_XamlSystemBaseType::base_type,
        content_property: impl_XamlSystemBaseType::content_property,
        full_name: impl_XamlSystemBaseType::full_name,
        is_array: impl_XamlSystemBaseType::is_array,
        is_collection: impl_XamlSystemBaseType::is_collection,
        is_constructible: impl_XamlSystemBaseType::is_constructible,
        is_dictionary: impl_XamlSystemBaseType::is_dictionary,
        is_markup_extension: impl_XamlSystemBaseType::is_markup_extension,
        is_bindable: impl_XamlSystemBaseType::is_bindable,
        item_type: impl_XamlSystemBaseType::item_type,
        key_type: impl_XamlSystemBaseType::key_type,
        underlying_type: impl_XamlSystemBaseType::underlying_type,
        activate_instance: impl_XamlSystemBaseType::activate_instance,
        create_from_string: impl_XamlSystemBaseType::create_from_string,
        get_member: impl_XamlSystemBaseType::get_member,
        add_to_vector: impl_XamlSystemBaseType::add_to_vector,
        add_to_map: impl_XamlSystemBaseType::add_to_map,
        run_initializer: impl_XamlSystemBaseType::run_initializer,
    };
    fn new(full_name: &'static str) -> Result<XamlSystemBaseType> {
        let value = Self {
            vtable: &Self::VTABLE,
            count: RefCount::new(),
            full_name,
        };
        unsafe {
            let mut result = std::mem::zeroed();
            let ptr: NonNull<Self> = NonNull::new_unchecked(Box::into_raw(Box::new(value)));
            *<XamlSystemBaseType as AbiTransferable>::set_abi(&mut result) =
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
        if iid == &<XamlSystemBaseType as ComInterface>::iid()
            || iid == &<IUnknown as ComInterface>::iid()
            || iid == &<Object as ComInterface>::iid()
            || iid == &<IXamlType as ComInterface>::iid()
        {
            *interface = this as *mut Self as *mut _;
            this.count.add_ref();
            return ErrorCode(0);
        }

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

    extern "system" fn base_type(
        _this: NonNullRawComPtr<IXamlType>,
        _result: *mut <IXamlType as AbiTransferable>::Abi,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }
    extern "system" fn content_property(
        _this: NonNullRawComPtr<IXamlType>,
        _result: *mut <IXamlMember as AbiTransferable>::Abi,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
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
        _result: *mut bool,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }

    extern "system" fn is_collection(
        _this: NonNullRawComPtr<IXamlType>,
        _result: *mut bool,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }

    extern "system" fn is_constructible(
        _this: NonNullRawComPtr<IXamlType>,
        _result: *mut bool,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }

    extern "system" fn is_dictionary(
        _this: NonNullRawComPtr<IXamlType>,
        _result: *mut bool,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }

    extern "system" fn is_markup_extension(
        _this: NonNullRawComPtr<IXamlType>,
        _result: *mut bool,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
    }
    extern "system" fn is_bindable(
        _this: NonNullRawComPtr<IXamlType>,
        _result: *mut bool,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
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
            kind: TypeKind::Primitive,
        };
        unsafe { *result = type_name.into_abi() };
        ErrorCode(0)
    }
    extern "system" fn activate_instance(
        _this: NonNullRawComPtr<IXamlType>,
        _result: *mut <Object as AbiTransferable>::Abi,
    ) -> ErrorCode {
        ErrorCode(0x80004002)
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
