winrt::import!(
    dependencies
        os
    types
        windows::foundation::PropertyValue
        windows::storage::StorageFolder
        windows::ui::xaml::{Application, IApplicationFactory, Window}
        windows::ui::xaml::controls::{
            Button, IButtonFactory, TextBlock, Page, IPageFactory, Frame
        }
        windows::ui::xaml::markup::{IXamlMetadataProvider, XamlBinaryWriter}
);
