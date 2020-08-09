winrt::import!(
    dependencies
        os
    types
        windows::foundation::PropertyValue
        windows::storage::StorageFolder
        windows::system::Launcher
        windows::ui::xaml::{Application, IApplicationFactory, Window}
        windows::ui::xaml::controls::{
            Button, Frame, GridViewItem, IButtonFactory, Image, IPageFactory,
            Page, TextBlock
        }
        windows::ui::xaml::markup::{
            IComponentConnector2, IXamlMetadataProvider, XamlBinaryWriter
        }
        windows::ui::xaml::media::animation::Storyboard
);
