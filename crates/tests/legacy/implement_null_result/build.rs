fn main() {
    windows::runtime::build! {
        Windows::Win32::Foundation::S_OK, Windows::UI::Xaml::Markup::IXamlType2,
    };
}