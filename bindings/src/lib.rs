winrt::import!(
    dependencies
        os
    types
        windows::foundation::PropertyValue
        windows::globalization::date_time_formatting::DateTimeFormatter
        windows::system::Launcher
        windows::ui::xaml::{Application, IApplicationFactory, Window}
        windows::ui::xaml::controls::{
            Frame, GridViewItem, Image, IPageFactory, ListView, Page,
            ProgressRing, TextBlock
        }
        windows::ui::xaml::data::ICustomPropertyProvider
        windows::ui::xaml::markup::{
            IComponentConnector2, IXamlMetadataProvider, XamlBinaryWriter
        }
        windows::ui::xaml::media::animation::Storyboard
        windows::web::syndication::SyndicationClient
);
