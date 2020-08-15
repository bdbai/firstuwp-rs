use bindings::windows::application_model::activation::*;
use bindings::windows::ui::xaml::controls::*;
use bindings::windows::ui::xaml::data::*;
use bindings::windows::ui::xaml::interop::*;
use bindings::windows::ui::xaml::markup::*;
use bindings::windows::ui::xaml::navigation::*;
use bindings::windows::ui::xaml::*;
use std::ffi::c_void;
use winrt::*;

#[repr(C)]
pub struct abi_IInspectable {
    pub iunknown: abi_IUnknown,

    pub inspectable_iids:
        extern "system" fn(NonNullRawComPtr<Object>, *mut u32, *mut *mut Guid) -> ErrorCode,
    pub inspectable_type_name: extern "system" fn(
        NonNullRawComPtr<Object>,
        *mut <HString as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub inspectable_trust_level:
        extern "system" fn(NonNullRawComPtr<Object>, *mut i32) -> ErrorCode,
}

#[repr(C)]
pub struct abi_IApplicationOverride {
    pub iinspectable: abi_IInspectable,
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

#[repr(C)]
pub struct abi_IPageOverrides {
    pub iinspectable: abi_IInspectable,
    pub on_navigated_from: extern "system" fn(
        NonNullRawComPtr<IPageOverrides>,
        <NavigationEventArgs as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub on_navigated_to: extern "system" fn(
        NonNullRawComPtr<IPageOverrides>,
        <NavigationEventArgs as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub on_navigating_from: extern "system" fn(
        NonNullRawComPtr<IPageOverrides>,
        <NavigatingCancelEventArgs as AbiTransferable>::Abi,
    ) -> ErrorCode,
}

#[repr(C)]
pub struct abi_IXamlMetadataProvider {
    pub iinspectable: abi_IInspectable,
    pub get_xaml_type: extern "system" fn(
        NonNullRawComPtr<IXamlMetadataProvider>,
        <TypeName as AbiTransferable>::Abi,
        *mut <IXamlType as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub get_xaml_type_by_full_name: extern "system" fn(
        NonNullRawComPtr<IXamlMetadataProvider>,
        <HString as AbiTransferable>::Abi,
        *mut <IXamlType as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub get_xmlns_definitions: extern "system" fn(
        NonNullRawComPtr<IXamlMetadataProvider>,
        *mut u32,
        *mut *mut <XmlnsDefinition as AbiTransferable>::Abi,
    ) -> ErrorCode,
}

#[repr(C)]
pub struct abi_IXamlType {
    pub iinspectable: abi_IInspectable,
    pub base_type: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        *mut <IXamlType as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub content_property: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        *mut <IXamlMember as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub full_name: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        *mut <HString as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub is_array: extern "system" fn(NonNullRawComPtr<IXamlType>, *mut bool) -> ErrorCode,
    pub is_collection: extern "system" fn(NonNullRawComPtr<IXamlType>, *mut bool) -> ErrorCode,
    pub is_constructible: extern "system" fn(NonNullRawComPtr<IXamlType>, *mut bool) -> ErrorCode,
    pub is_dictionary: extern "system" fn(NonNullRawComPtr<IXamlType>, *mut bool) -> ErrorCode,
    pub is_markup_extension:
        extern "system" fn(NonNullRawComPtr<IXamlType>, *mut bool) -> ErrorCode,
    pub is_bindable: extern "system" fn(NonNullRawComPtr<IXamlType>, *mut bool) -> ErrorCode,
    pub item_type: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        *mut <IXamlType as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub key_type: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        *mut <IXamlType as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub underlying_type: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        *mut <TypeName as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub activate_instance:
        extern "system" fn(NonNullRawComPtr<IXamlType>, *mut RawComPtr<Object>) -> ErrorCode,
    pub create_from_string: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        value: <HString as AbiTransferable>::Abi,
        *mut RawComPtr<Object>,
    ) -> ErrorCode,
    pub get_member: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        name: <HString as AbiTransferable>::Abi,
        *mut <IXamlMember as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub add_to_vector: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        instance: RawComPtr<Object>,
        value: RawComPtr<Object>,
    ) -> ErrorCode,
    pub add_to_map: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        instance: RawComPtr<Object>,
        key: RawComPtr<Object>,
        value: RawComPtr<Object>,
    ) -> ErrorCode,
    pub run_initializer: extern "system" fn(NonNullRawComPtr<IXamlType>) -> ErrorCode,
}

#[repr(C)]
pub struct abi_IComponentConnector {
    pub iinspectable: abi_IInspectable,
    pub connect: extern "system" fn(
        NonNullRawComPtr<IComponentConnector>,
        i32,
        object: RawComPtr<Object>,
    ) -> ErrorCode,
}

#[repr(C)]
pub struct abi_IWeakReference {
    pub iunknown: abi_IUnknown,
    pub resolve: extern "system" fn(
        this: NonNullRawComPtr<IUnknown>,
        *const Guid,
        *mut *mut IUnknown,
    ) -> i32,
}

#[repr(C)]
pub struct abi_IWeakReferenceSource {
    pub iunknown: abi_IUnknown,
    pub get_weak_reference:
        extern "system" fn(this: NonNullRawComPtr<IUnknown>, result: *mut *mut c_void) -> i32,
}

#[repr(C)]
pub struct abi_ICustomPropertyProvider {
    pub iinspectable: abi_IInspectable,
    pub get_custom_property: extern "system" fn(
        NonNullRawComPtr<ICustomPropertyProvider>,
        name: <HString as AbiTransferable>::Abi,
        result: *mut <ICustomProperty as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub get_indexed_custom_property: extern "system" fn(
        NonNullRawComPtr<ICustomPropertyProvider>,
        name: <HString as AbiTransferable>::Abi,
        typename: <TypeName as AbiTransferable>::Abi,
        result: *mut <ICustomProperty as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub get_string_representation: extern "system" fn(
        NonNullRawComPtr<ICustomPropertyProvider>,
        result: *mut <HString as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub get_type: extern "system" fn(
        NonNullRawComPtr<ICustomPropertyProvider>,
        result: *mut <TypeName as AbiTransferable>::Abi,
    ) -> ErrorCode,
}

#[repr(C)]
pub struct abi_ICustomProperty {
    pub iinspectable: abi_IInspectable,
    pub get_type: extern "system" fn(
        NonNullRawComPtr<ICustomProperty>,
        result: *mut <TypeName as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub get_name: extern "system" fn(
        NonNullRawComPtr<ICustomProperty>,
        result: *mut <HString as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub get_value: extern "system" fn(
        NonNullRawComPtr<ICustomProperty>,
        target: RawComPtr<Object>,
        result: *mut RawComPtr<Object>,
    ) -> ErrorCode,
    pub set_value: extern "system" fn(
        NonNullRawComPtr<ICustomProperty>,
        target: RawComPtr<Object>,
        value: RawComPtr<Object>,
    ) -> ErrorCode,
    pub get_indexed_value: extern "system" fn(
        NonNullRawComPtr<ICustomProperty>,
        target: RawComPtr<Object>,
        index: RawComPtr<Object>,
        result: *mut RawComPtr<Object>,
    ) -> ErrorCode,
    pub set_indexed_value: extern "system" fn(
        NonNullRawComPtr<ICustomProperty>,
        target: RawComPtr<Object>,
        value: RawComPtr<Object>,
        index: RawComPtr<Object>,
    ) -> ErrorCode,
    pub get_can_write:
        extern "system" fn(NonNullRawComPtr<ICustomProperty>, result: *mut bool) -> ErrorCode,
    pub get_can_read:
        extern "system" fn(NonNullRawComPtr<ICustomProperty>, result: *mut bool) -> ErrorCode,
}
