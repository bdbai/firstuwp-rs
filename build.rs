winrt::build!(
    dependencies
        os
    types
        windows::foundation::PropertyValue
        windows::ui::xaml::{Application, IApplicationFactory, Window}
        windows::ui::xaml::controls::{
            Button, IButtonFactory, TextBlock, Page, IPageFactory, Frame
        }
        windows::ui::xaml::markup::IXamlMetadataProvider
);

fn main() {
    build();
}
