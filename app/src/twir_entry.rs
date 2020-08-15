use crate::abi::*;
use bindings::windows::foundation::{PropertyValue, Uri};
use bindings::windows::ui::xaml::data::{ICustomProperty, ICustomPropertyProvider};
use bindings::windows::ui::xaml::interop::{TypeKind, TypeName};
use paste::paste;
use std::ptr::NonNull;
use winrt::*;

#[repr(transparent)]
pub struct TwirEntry {
    ptr: ComPtr<TwirEntry>,
}

unsafe impl ComInterface for TwirEntry {
    type VTable = abi_ICustomPropertyProvider;
    fn iid() -> Guid {
        Guid::from_values(
            0xb13318ab,
            0x182d,
            0x4c5e,
            [0xac, 0x64, 0x79, 0x90, 0x4d, 0x01, 0xe2, 0x90],
        )
    }
}

unsafe impl AbiTransferable for TwirEntry {
    type Abi = RawComPtr<Self>;
    fn get_abi(&self) -> Self::Abi {
        <ComPtr<Self> as AbiTransferable>::get_abi(&self.ptr)
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        <ComPtr<Self> as AbiTransferable>::set_abi(&mut self.ptr)
    }
}

macro_rules! make_impl_twir_entry {
    ( $($field:ident: $t:ident),* ) => {
        impl TwirEntry {
            pub fn new(
                $(
                   paste!([<$field:lower>]): $t,
                )*
                link: Uri
            ) -> Result<TwirEntry> {
                impl_TwirEntry::new($(paste!([<$field:lower>]),)* link)
            }
        }

        paste!{
            #[allow(non_camel_case_types)]
            pub struct impl_TwirEntry {
                #[allow(unused)]
                vtable: *const abi_ICustomPropertyProvider,
                count: RefCount,
                $(
                    [<$field:lower>]: $t,
                )*
                pub link: Uri,
            }
        }

        impl impl_TwirEntry {
            const VTABLE: abi_ICustomPropertyProvider = abi_ICustomPropertyProvider {
                iinspectable: abi_IInspectable {
                    iunknown: abi_IUnknown {
                        unknown_query_interface: Self::unknown_query_interface,
                        unknown_add_ref: Self::unknown_add_ref,
                        unknown_release: Self::unknown_release,
                    },
                    inspectable_iids: Self::inspectable_iids,
                    inspectable_type_name: Self::inspectable_type_name,
                    inspectable_trust_level: Self::inspectable_trust_level,
                },
                get_type: Self::get_type,
                get_custom_property: Self::get_custom_property,
                get_indexed_custom_property: Self::get_indexed_custom_property,
                get_string_representation: Self::get_string_representation,
            };
            pub fn new(
                $(
                   paste!([<$field:lower>]): $t,
                )*
                link: Uri,
            ) -> Result<TwirEntry> {
                paste!{
                    let value = impl_TwirEntry {
                        vtable: &Self::VTABLE,
                        count: RefCount::new(),
                        $(
                           [<$field:lower>],
                        )*
                        link,
                    };
                }
                unsafe {
                    let mut result = std::mem::zeroed();
                    let ptr: NonNull<Self> = NonNull::new_unchecked(Box::into_raw(Box::new(value)));
                    *<TwirEntry as AbiTransferable>::set_abi(&mut result) =
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
                if iid == &<TwirEntry as ComInterface>::iid()
                    || iid == &<IUnknown as ComInterface>::iid()
                    || iid == &<Object as ComInterface>::iid()
                    || iid == &<ICustomPropertyProvider as ComInterface>::iid()
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
                unsafe { *class_name = HString::from("FirstUwp.TwirEntry").into_abi() };
                ErrorCode(0)
            }
            extern "system" fn inspectable_trust_level(
                _this: NonNullRawComPtr<Object>,
                _trust_level: *mut i32,
            ) -> ErrorCode {
                loop {}
            }
            extern "system" fn get_type(
                _this: NonNullRawComPtr<ICustomPropertyProvider>,
                result: *mut <TypeName as AbiTransferable>::Abi,
            ) -> ErrorCode {
                unsafe {
                    *result = TypeName {
                        kind: TypeKind::Custom,
                        name: HString::from("FirstUwp.TwirEntry"),
                    }
                    .into_abi()
                };
                ErrorCode(0)
            }
            extern "system" fn get_custom_property(
                _this: NonNullRawComPtr<ICustomPropertyProvider>,
                name: <HString as AbiTransferable>::Abi,
                result: *mut <ICustomProperty as AbiTransferable>::Abi,
            ) -> ErrorCode {
                let name = HString::from_abi(&name);
                paste! {
                    $(
                        const [<$field:upper _VTABLE>]: abi_ICustomProperty = abi_ICustomProperty {
                            iinspectable: abi_IInspectable {
                                iunknown: abi_IUnknown {
                                    unknown_query_interface:
                                        impl_TwirEntry::property_unknown_query_interface,
                                    unknown_add_ref: impl_TwirEntry::property_unknown_add_ref,
                                    unknown_release: impl_TwirEntry::property_unknown_release,
                                },
                                inspectable_iids: impl_TwirEntry::property_inspectable_iids,
                                inspectable_type_name: impl_TwirEntry::property_inspectable_type_name,
                                inspectable_trust_level: impl_TwirEntry::property_inspectable_trust_level,
                            },
                            get_can_read: impl_TwirEntry::get_can_read,
                            get_can_write: impl_TwirEntry::get_can_write,
                            get_name: impl_TwirEntry::[<get_name_ $field:lower>],
                            get_type: impl_TwirEntry::property_get_type,
                            get_value: impl_TwirEntry::[<get_value_ $field:lower>],
                            set_value: impl_TwirEntry::set_value,
                            get_indexed_value: impl_TwirEntry::get_indexed_value,
                            set_indexed_value: impl_TwirEntry::set_indexed_value,
                        };
                    )*
                }
                unsafe {
                    *result = std::mem::transmute(match name.to_string().as_str() {
                        $(
                            stringify!($field) => &&paste!([<$field:upper _VTABLE>]),
                        )*
                        _ => unreachable!(),
                    })
                };
                ErrorCode(0)
            }
            extern "system" fn get_indexed_custom_property(
                _this: NonNullRawComPtr<ICustomPropertyProvider>,
                _raw_name: <HString as AbiTransferable>::Abi,
                _typename: <TypeName as AbiTransferable>::Abi,
                _result: *mut <ICustomProperty as AbiTransferable>::Abi,
            ) -> ErrorCode {
                ErrorCode(0x80004001)
            }
            extern "system" fn get_string_representation(
                _this: NonNullRawComPtr<ICustomPropertyProvider>,
                result: *mut <HString as AbiTransferable>::Abi,
            ) -> ErrorCode {
                unsafe { *result = HString::from("TWiR Entry").into_abi() };
                ErrorCode(0)
            }

            extern "system" fn property_unknown_query_interface(
                this: NonNullRawComPtr<IUnknown>,
                iid: &Guid,
                interface: *mut RawPtr,
            ) -> ErrorCode {
                let interface = unsafe { interface.as_mut().unwrap() };
                let this: &mut Self = unsafe { std::mem::transmute(this) };
                if iid == &<TwirEntry as ComInterface>::iid()
                    || iid == &<IUnknown as ComInterface>::iid()
                    || iid == &<Object as ComInterface>::iid()
                    || iid == &<ICustomProperty as ComInterface>::iid()
                {
                    *interface = this as *mut Self as *mut _;
                    return ErrorCode(0);
                }

                *interface = std::ptr::null_mut();
                ErrorCode(0x80004002)
            }
            extern "system" fn property_unknown_add_ref(_this: NonNullRawComPtr<IUnknown>) -> u32 {
                1
            }
            extern "system" fn property_unknown_release(_this: NonNullRawComPtr<IUnknown>) -> u32 {
                1
            }
            extern "system" fn property_inspectable_iids(
                _this: NonNullRawComPtr<Object>,
                _iidcount: *mut u32,
                _iids: *mut *mut Guid,
            ) -> ErrorCode {
                loop {}
            }
            extern "system" fn property_inspectable_type_name(
                _this: NonNullRawComPtr<Object>,
                class_name: *mut <HString as AbiTransferable>::Abi,
            ) -> ErrorCode {
                unsafe { *class_name = HString::from("FirstUwp.TwirEntryProperty").into_abi() };
                ErrorCode(0)
            }
            extern "system" fn property_inspectable_trust_level(
                _this: NonNullRawComPtr<Object>,
                _trust_level: *mut i32,
            ) -> ErrorCode {
                loop {}
            }
            extern "system" fn get_can_read(
                _this: NonNullRawComPtr<ICustomProperty>,
                result: *mut bool,
            ) -> ErrorCode {
                unsafe { *result = true };
                ErrorCode(0)
            }
            extern "system" fn get_can_write(
                _this: NonNullRawComPtr<ICustomProperty>,
                result: *mut bool,
            ) -> ErrorCode {
                unsafe { *result = false };
                ErrorCode(0)
            }
            paste!{
                $(
                    extern "system" fn [<get_name_ $field:lower>](
                        _this: NonNullRawComPtr<ICustomProperty>,
                        result: *mut <HString as AbiTransferable>::Abi,
                    ) -> ErrorCode {
                        unsafe { *result = HString::from(stringify!($field)).into_abi() };
                        ErrorCode(0)
                    }
                )*
            }
            extern "system" fn property_get_type(
                _this: NonNullRawComPtr<ICustomProperty>,
                result: *mut <TypeName as AbiTransferable>::Abi,
            ) -> ErrorCode {
                unsafe {
                    *result = TypeName {
                        kind: TypeKind::Primitive,
                        name: HString::from("String"),
                    }
                };
                ErrorCode(0)
            }
            paste!{
                $(
                    extern "system" fn [<get_value_ $field:lower>](
                        _this: NonNullRawComPtr<ICustomProperty>,
                        target: RawComPtr<Object>,
                        result: *mut RawComPtr<Object>,
                    ) -> ErrorCode {
                        unsafe {
                            *result = PropertyValue::create_string(
                                (*(target.unwrap().as_raw() as *mut Self)).[<$field:lower>].clone(),
                            )
                            .unwrap()
                            .into_abi()
                        };
                        ErrorCode(0)
                    }
                )*
            }
            extern "system" fn set_value(
                _this: NonNullRawComPtr<ICustomProperty>,
                _target: RawComPtr<Object>,
                _value: RawComPtr<Object>,
            ) -> ErrorCode {
                ErrorCode(0x80004001)
            }
            extern "system" fn get_indexed_value(
                _this: NonNullRawComPtr<ICustomProperty>,
                _target: RawComPtr<Object>,
                _index: RawComPtr<Object>,
                _result: *mut RawComPtr<Object>,
            ) -> ErrorCode {
                ErrorCode(0x80004001)
            }
            extern "system" fn set_indexed_value(
                _this: NonNullRawComPtr<ICustomProperty>,
                _target: RawComPtr<Object>,
                _value: RawComPtr<Object>,
                _index: RawComPtr<Object>,
            ) -> ErrorCode {
                ErrorCode(0x80004001)
            }
        }
    };
}

make_impl_twir_entry!(Issue: HString, Date: HString, Summary: HString);
