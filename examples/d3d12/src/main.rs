use bindings::Windows::Win32::{
    Direct3D11::*, Direct3D12::*, DisplayDevices::*, Dxgi::*, SystemServices::*,
    WindowsAndMessaging::*,
};
use std::mem::transmute;
use windows::*;

trait DXSample {
    fn init(command_line: &SampleCommandLine) -> Result<Self>
    where
        Self: Sized;

    fn update(&mut self) {}
    fn render(&mut self) {}
    fn on_key_up(&mut self, _key: u8) {}
    fn on_key_down(&mut self, _key: u8) {}

    fn title(&self) -> String {
        "DXSample".into()
    }

    fn window_size(&self) -> (i32, i32) {
        (640, 480)
    }
}

#[derive(Clone)]
struct SampleCommandLine {
    use_warp_device: bool,
}

fn build_command_line() -> SampleCommandLine {
    let mut use_warp_device = false;

    for arg in std::env::args() {
        if arg.eq_ignore_ascii_case("-warp") || arg.eq_ignore_ascii_case("/warp") {
            use_warp_device = true;
        }
    }

    SampleCommandLine { use_warp_device }
}

fn run_sample<S>() -> Result<()>
where
    S: DXSample,
{
    unsafe {
        let instance = HINSTANCE(GetModuleHandleA(None));
        debug_assert!(instance.0 != 0);

        let wc = WNDCLASSEXA {
            cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
            style: WNDCLASS_STYLES::CS_HREDRAW | WNDCLASS_STYLES::CS_VREDRAW,
            lpfnWndProc: Some(wndproc::<S>),
            hInstance: instance,
            hCursor: LoadCursorW(None, IDC_ARROW),
            lpszClassName: PSTR(b"RustWindowClass\0".as_ptr() as _),
            ..Default::default()
        };

        let command_line = build_command_line();
        let mut sample = S::init(&command_line)?;

        let size = sample.window_size();

        let atom = RegisterClassExA(&wc);
        debug_assert!(atom != 0);

        let mut window_rect = RECT {
            left: 0,
            top: 0,
            right: size.0,
            bottom: size.1,
        };
        AdjustWindowRect(&mut window_rect, WINDOW_STYLE::WS_OVERLAPPEDWINDOW.0, false);

        let mut title = sample.title();

        if command_line.use_warp_device {
            title.push_str(" (WARP)");
        }

        let hwnd = CreateWindowExA(
            Default::default(),
            "RustWindowClass",
            title,
            WINDOW_STYLE::WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            window_rect.right - window_rect.left,
            window_rect.bottom - window_rect.top,
            None, // no parent window
            None, // no menus
            instance,
            &mut sample as *mut _ as _,
        );
        debug_assert!(hwnd.0 != 0);

        ShowWindow(hwnd, SHOW_WINDOW_CMD::SW_SHOW);

        loop {
            let mut message = MSG::default();

            if PeekMessageA(
                &mut message,
                None,
                0,
                0,
                PEEK_MESSAGE_REMOVE_TYPE::PM_REMOVE,
            )
            .into()
            {
                TranslateMessage(&mut message);
                DispatchMessageA(&mut message);

                if message.message == WM_QUIT {
                    break;
                }
            }
        }

        Ok(())
    }
}

fn sample_wndproc<S: DXSample>(sample: &mut S, message: u32, wparam: WPARAM) -> bool {
    match message {
        WM_KEYDOWN => {
            sample.on_key_down(wparam.0 as u8);
            true
        }
        WM_KEYUP => {
            sample.on_key_up(wparam.0 as u8);
            true
        }
        WM_PAINT => {
            sample.update();
            sample.render();
            true
        }
        _ => false,
    }
}

extern "system" fn wndproc<S: DXSample>(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        let user_data = GetWindowLongPtrA(window, WINDOW_LONG_PTR_INDEX::GWLP_USERDATA);
        let sample: Option<&mut S> = match user_data {
            0 => None,
            _ => Some(transmute(user_data)),
        };

        match message {
            WM_CREATE => {
                let create_struct: &CREATESTRUCTA = transmute(lparam);
                SetWindowLongPtrA(
                    window,
                    WINDOW_LONG_PTR_INDEX::GWLP_USERDATA,
                    transmute(create_struct.lpCreateParams),
                );
                LRESULT::default()
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT::default()
            }
            _ => {
                let handled = match sample {
                    Some(s) => sample_wndproc(s, message, wparam),
                    None => false,
                };

                if handled {
                    LRESULT::default()
                } else {
                    DefWindowProcA(window, message, wparam, lparam)
                }
            }
        }
    }
}

fn main() -> Result<()> {
    run_sample::<D3D12HelloWindow>()?;

    Ok(())
}

fn get_hardware_adapter(factory: &IDXGIFactory4) -> Result<IDXGIAdapter1> {
    let mut adapter_: Option<IDXGIAdapter1> = None;

    let mut i = 0;

    unsafe {
        while factory.EnumAdapters1(i, &mut adapter_) != DXGI_ERROR_NOT_FOUND {
            i = i + 1;

            if let Some(adapter) = adapter_ {
                adapter_ = None;
                let mut desc: DXGI_ADAPTER_DESC1 = DXGI_ADAPTER_DESC1::default();
                adapter.GetDesc1(&mut desc).ok()?;

                if (DXGI_ADAPTER_FLAG::from(desc.Flags)
                    & DXGI_ADAPTER_FLAG::DXGI_ADAPTER_FLAG_SOFTWARE)
                    != DXGI_ADAPTER_FLAG::DXGI_ADAPTER_FLAG_NONE
                {
                    // Don't select the Basic Render Driver adapter.
                    // If you want a software adapter, pass in "/warp" on the command line.
                    continue;
                }

    // We need the variant where we pass in NULL for the outparam.
    #[link(name = "d3d12")]
    extern "system" {
        pub fn D3D12CreateDevice(
            padapter: ::windows::RawPtr,
            minimumfeaturelevel: D3D_FEATURE_LEVEL,
            riid: *const ::windows::Guid,
            ppdevice: *mut *mut ::std::ffi::c_void,
        ) -> ::windows::HRESULT;
    }

    // Check to see whether the adapter supports Direct3D 12, but
    // don't create the actual device yet.
    if D3D12CreateDevice(
        adapter.abi(),
        D3D_FEATURE_LEVEL::D3D_FEATURE_LEVEL_11_0,
        &ID3D12Device::IID,
        std::ptr::null_mut(),
    )
    .is_ok()
    {
        return Ok(adapter);
    }
            }
        }
    }

    Err(DXGI_ERROR_NOT_FOUND.into())
}

struct D3D12HelloWindow {}

impl DXSample for D3D12HelloWindow {
    fn init(command_line: &SampleCommandLine) -> Result<Self> {
        if cfg!(debug_assertions) {
            unsafe {
                if let Ok(debug) = D3D12GetDebugInterface::<ID3D12Debug>() {
                    debug.EnableDebugLayer();
                }
            }
        }

        let dxgi_factory_flags = if cfg!(debug_assertions) {
            DXGI_CREATE_FACTORY_DEBUG
        } else {
            0
        };

        let factory = unsafe { CreateDXGIFactory2::<IDXGIFactory4>(dxgi_factory_flags)? };

        let _adapter = if command_line.use_warp_device {
            unsafe { factory.EnumWarpAdapter() }
        } else {
            get_hardware_adapter(&factory)
        };

        //let mut device: Option<ID3D12Device> = None;

        Ok(D3D12HelloWindow {})
    }

    fn title(&self) -> String {
        "D3D12 Hello Window".into()
    }

    fn window_size(&self) -> (i32, i32) {
        (1280, 720)
    }
}
