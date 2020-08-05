use crate::windows::application_model::activation::*;
use crate::windows::ui::xaml::controls::*;
use crate::windows::ui::xaml::interop::*;
use crate::windows::ui::xaml::markup::*;
use crate::windows::ui::xaml::navigation::*;
use crate::windows::ui::xaml::*;
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
    pub activate_instance: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        *mut <Object as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub create_from_string: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        value: <HString as AbiTransferable>::Abi,
        *mut <Object as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub get_member: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        name: <HString as AbiTransferable>::Abi,
        *mut <IXamlMember as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub add_to_vector: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        instance: <Object as AbiTransferable>::Abi,
        value: <Object as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub add_to_map: extern "system" fn(
        NonNullRawComPtr<IXamlType>,
        instance: <Object as AbiTransferable>::Abi,
        key: <Object as AbiTransferable>::Abi,
        value: <Object as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub run_initializer: extern "system" fn(NonNullRawComPtr<IXamlType>) -> ErrorCode,
}
