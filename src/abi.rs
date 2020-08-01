use crate::windows::application_model::activation::*;
use crate::windows::ui::xaml::controls::*;
use crate::windows::ui::xaml::navigation::*;
use crate::windows::ui::xaml::*;
use winrt::*;

#[repr(C)]
pub struct abi_IInspectable {
    pub iunknown: abi_IUnknown,

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
    pub on_navigated_from: unsafe extern "system" fn(
        NonNullRawComPtr<IPageOverrides>,
        <NavigationEventArgs as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub on_navigated_to: unsafe extern "system" fn(
        NonNullRawComPtr<IPageOverrides>,
        <NavigationEventArgs as AbiTransferable>::Abi,
    ) -> ErrorCode,
    pub on_navigating_from: unsafe extern "system" fn(
        NonNullRawComPtr<IPageOverrides>,
        <NavigatingCancelEventArgs as AbiTransferable>::Abi,
    ) -> ErrorCode,
}
