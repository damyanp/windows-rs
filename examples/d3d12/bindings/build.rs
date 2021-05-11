fn main() {
    windows::build!(
        Windows::Win32::Graphics::Direct3D12::*,
        Windows::Win32::Graphics::Dxgi::*,
        Windows::Win32::Graphics::Direct3D11::{
            D3D_FEATURE_LEVEL_11_0,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST
        },
        Windows::Win32::Graphics::Hlsl::*,
        Windows::Win32::UI::WindowsAndMessaging::{
            AdjustWindowRect,
            CREATESTRUCTA,
            CreateWindowExA,
            CS_HREDRAW,
            CS_VREDRAW,
            CW_USEDEFAULT,
            DefWindowProcA,
            DispatchMessageA,
            GetWindowLongPtrA,
            GWLP_USERDATA,
            IDC_ARROW,
            LoadCursorW,
            MSG,            
            PeekMessageA,
            PM_REMOVE,
            PostQuitMessage,
            RegisterClassExA,
            SetWindowLongPtrA,
            ShowWindow,
            SW_SHOW,
            TranslateMessage,
            WS_OVERLAPPEDWINDOW,
            WM_CREATE,
            WM_DESTROY,
            WM_KEYDOWN,
            WM_KEYUP,
            WM_PAINT,
            WM_QUIT,
            WNDCLASS_STYLES,
            WNDCLASSEXA,
        },
        Windows::Win32::System::SystemServices::{
            GetModuleHandleA,
            HINSTANCE,
            PSTR,
        },
        Windows::Win32::System::Threading::{
            CreateEventA,
            WaitForSingleObject
        },
        Windows::Win32::System::WindowsProgramming::{
            INFINITE
        },
        Windows::Win32::UI::DisplayDevices::{
            RECT
        },
    );
}
