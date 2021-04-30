fn main() {
    windows::build!(
        Windows::Win32::Direct3D12::*,
        Windows::Win32::Dxgi::*,
        Windows::Win32::Direct3D11::*,
        Windows::Win32::WindowsAndMessaging::*,
        Windows::Win32::SystemServices::{GetModuleHandleA, CallEnclave},
        Windows::Win32::DisplayDevices::{RECT}
    );
}
