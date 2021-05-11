use bindings::Windows::Win32::{
    Graphics::{Direct3D11::*, Direct3D12::*, Dxgi::*, Hlsl::*},
    System::{SystemServices::*, Threading::*, WindowsProgramming::*},
    UI::{DisplayDevices::*, WindowsAndMessaging::*},
};
use std::mem::transmute;
use windows::*;

trait DXSample {
    fn new(command_line: &SampleCommandLine) -> Result<Self>
    where
        Self: Sized;

    fn bind_to_window(&mut self, hwnd: &HWND) -> Result<()>;

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
        let instance: HINSTANCE = GetModuleHandleA(None);
        debug_assert!(instance.0 != 0);

        let wc = WNDCLASSEXA {
            cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc::<S>),
            hInstance: instance,
            hCursor: LoadCursorW(None, IDC_ARROW),
            lpszClassName: PSTR(b"RustWindowClass\0".as_ptr() as _),
            ..Default::default()
        };

        let command_line = build_command_line();
        let mut sample = S::new(&command_line)?;

        let size = sample.window_size();

        let atom = RegisterClassExA(&wc);
        debug_assert!(atom != 0);

        let mut window_rect = RECT {
            left: 0,
            top: 0,
            right: size.0,
            bottom: size.1,
        };
        AdjustWindowRect(&mut window_rect, WS_OVERLAPPEDWINDOW.0, false);

        let mut title = sample.title();

        if command_line.use_warp_device {
            title.push_str(" (WARP)");
        }

        let hwnd = CreateWindowExA(
            Default::default(),
            "RustWindowClass",
            title,
            WS_OVERLAPPEDWINDOW,
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

        sample.bind_to_window(&hwnd)?;

        ShowWindow(hwnd, SW_SHOW);

        loop {
            let mut message = MSG::default();

            if PeekMessageA(&mut message, None, 0, 0, PM_REMOVE).into() {
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

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongA(window, index, value as _) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    SetWindowLongPtrA(window, index, value)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongA(window, index) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongPtrA(window, index)
}

extern "system" fn wndproc<S: DXSample>(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        let user_data = GetWindowLong(window, GWLP_USERDATA);
        let sample: Option<&mut S> = match user_data {
            0 => None,
            _ => Some(transmute(user_data)),
        };

        match message {
            WM_CREATE => {
                let create_struct: &CREATESTRUCTA = transmute(lparam);
                SetWindowLong(
                    window,
                    GWLP_USERDATA,
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

                if (DXGI_ADAPTER_FLAG::from(desc.Flags) & DXGI_ADAPTER_FLAG_SOFTWARE)
                    != DXGI_ADAPTER_FLAG_NONE
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
                    D3D_FEATURE_LEVEL_11_0,
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

const FRAME_COUNT: u32 = 2;

mod d3d12_hello_triangle {
    use super::*;

    pub struct Sample {
        dxgi_factory: IDXGIFactory4,
        device: ID3D12Device,
        resources: Option<Resources>,
    }

    struct Resources {
        command_queue: ID3D12CommandQueue,
        swap_chain: IDXGISwapChain3,
        frame_index: u32,
        render_targets: [ID3D12Resource; FRAME_COUNT as usize],
        rtv_heap: ID3D12DescriptorHeap,
        rtv_descriptor_size: usize,
        viewport: D3D12_VIEWPORT,
        scissor_rect: RECT,
        command_allocator: ID3D12CommandAllocator,
        root_signature: ID3D12RootSignature,
        pso: ID3D12PipelineState,
        command_list: ID3D12GraphicsCommandList,

        // we need to keep this around to keep the reference alive, even though
        // nothing reads from it
        #[allow(dead_code)]
        vertex_buffer: ID3D12Resource,

        vbv: D3D12_VERTEX_BUFFER_VIEW,
        fence: ID3D12Fence,
        fence_value: u64,
        fence_event: HANDLE,
    }

    impl DXSample for Sample {
        fn new(command_line: &SampleCommandLine) -> Result<Self> {
            let (dxgi_factory, device) = create_device(&command_line)?;

            Ok(Sample {
                dxgi_factory,
                device,
                resources: None,
            })
        }

        fn bind_to_window(&mut self, hwnd: &HWND) -> Result<()> {
            let command_queue: ID3D12CommandQueue = unsafe {
                self.device.CreateCommandQueue(&D3D12_COMMAND_QUEUE_DESC {
                    Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
                    ..Default::default()
                })?
            };

            let (width, height) = self.window_size();

            let swap_chain_desc = DXGI_SWAP_CHAIN_DESC1 {
                BufferCount: FRAME_COUNT,
                Width: width as u32,
                Height: height as u32,
                Format: DXGI_FORMAT_R8G8B8A8_UNORM,
                BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
                SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    ..Default::default()
                },
                ..Default::default()
            };

            let swap_chain: IDXGISwapChain3 = unsafe {
                let mut swap_chain_1 = None;
                self.dxgi_factory
                    .CreateSwapChainForHwnd(
                        &command_queue,
                        hwnd,
                        &swap_chain_desc,
                        std::ptr::null(),
                        None,
                        &mut swap_chain_1,
                    )
                    .ok()?;

                swap_chain_1.unwrap().cast()?
            };

            // This sample does not support fullscreen transitions
            unsafe {
                self.dxgi_factory
                    .MakeWindowAssociation(hwnd, DXGI_MWA_NO_ALT_ENTER)
                    .ok()?
            };

            let frame_index = unsafe { swap_chain.GetCurrentBackBufferIndex() };

            let rtv_heap: ID3D12DescriptorHeap = unsafe {
                self.device
                    .CreateDescriptorHeap(&D3D12_DESCRIPTOR_HEAP_DESC {
                        NumDescriptors: FRAME_COUNT,
                        Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
                        ..Default::default()
                    })?
            };

            let rtv_descriptor_size = unsafe {
                self.device
                    .GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV)
            } as usize;
            let rtv_handle = unsafe { rtv_heap.GetCPUDescriptorHandleForHeapStart() };

            let render_targets: [ID3D12Resource; FRAME_COUNT as usize] =
                array_init::try_array_init(|i: usize| -> Result<ID3D12Resource> {
                    let render_target: ID3D12Resource = unsafe { swap_chain.GetBuffer(i as u32)? };
                    unsafe {
                        self.device.CreateRenderTargetView(
                            &render_target,
                            std::ptr::null_mut(),
                            &D3D12_CPU_DESCRIPTOR_HANDLE {
                                ptr: rtv_handle.ptr + i * rtv_descriptor_size,
                            },
                        );
                    }
                    Ok(render_target)
                })?;

            let viewport = D3D12_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: width as f32,
                Height: height as f32,
                MinDepth: D3D12_MIN_DEPTH,
                MaxDepth: D3D12_MAX_DEPTH,
            };

            let scissor_rect = RECT {
                left: 0,
                top: 0,
                right: width,
                bottom: height,
            };

            let command_allocator = unsafe {
                self.device
                    .CreateCommandAllocator(D3D12_COMMAND_LIST_TYPE_DIRECT)?
            };

            let root_signature = create_root_signature(&self.device)?;
            let pso = create_pipeline_state(&self.device, &root_signature)?;

            let command_list: ID3D12GraphicsCommandList = unsafe {
                self.device.CreateCommandList(
                    0,
                    D3D12_COMMAND_LIST_TYPE_DIRECT,
                    &command_allocator,
                    &pso,
                )?
            };
            unsafe {
                command_list.Close().ok()?;
            }

            let aspect_ratio = width as f32 / height as f32;

            let (vertex_buffer, vbv) = create_vertex_buffer(&self.device, aspect_ratio)?;

            let fence = unsafe { self.device.CreateFence(0, D3D12_FENCE_FLAG_NONE)? };

            let fence_value = 1;

            let fence_event = unsafe { CreateEventA(std::ptr::null_mut(), false, false, None) };

            self.resources = Some(Resources {
                command_queue,
                swap_chain,
                frame_index,
                render_targets,
                rtv_heap,
                rtv_descriptor_size,
                viewport,
                scissor_rect,
                command_allocator,
                root_signature,
                pso,
                command_list,
                vertex_buffer,
                vbv,
                fence,
                fence_value,
                fence_event,
            });

            Ok(())
        }

        fn title(&self) -> String {
            "D3D12 Hello Window".into()
        }

        fn window_size(&self) -> (i32, i32) {
            (1280, 720)
        }

        fn render(&mut self) {
            if let Some(resources) = &mut self.resources {
                populate_command_list(&resources).unwrap();

                // Execute the command list.
                unsafe {
                    let command_list = ID3D12CommandList::from(&resources.command_list);

                    resources
                        .command_queue
                        .ExecuteCommandLists(1, &mut Some(command_list));

                    // Present the frame.
                    resources.swap_chain.Present(1, 0).ok().unwrap();
                }

                wait_for_previous_frame(resources);
            }
        }
    }

    fn populate_command_list(resources: &Resources) -> Result<()> {
        unsafe {
            // Command list allocators can only be reset when the associated
            // command lists have finished execution on the GPU; apps should use
            // fences to determine GPU execution progress.
            resources.command_allocator.Reset().ok()?;

            let command_list = &resources.command_list;

            // However, when ExecuteCommandList() is called on a particular
            // command list, that command list can then be reset at any time and
            // must be before re-recording.
            command_list
                .Reset(&resources.command_allocator, &resources.pso)
                .ok()?;

            // Set necessary state.
            command_list.SetGraphicsRootSignature(&resources.root_signature);
            command_list.RSSetViewports(1, &resources.viewport);
            command_list.RSSetScissorRects(1, &resources.scissor_rect);

            // Indicate that the back buffer will be used as a render target.
            let barrier = transition_barrier(
                &resources.render_targets[resources.frame_index as usize],
                D3D12_RESOURCE_STATE_PRESENT,
                D3D12_RESOURCE_STATE_RENDER_TARGET,
            );
            command_list.ResourceBarrier(1, &barrier);

            let rtv_handle = D3D12_CPU_DESCRIPTOR_HANDLE {
                ptr: resources.rtv_heap.GetCPUDescriptorHandleForHeapStart().ptr
                    + resources.frame_index as usize * resources.rtv_descriptor_size,
            };

            command_list.OMSetRenderTargets(1, &rtv_handle, false, std::ptr::null());

            // Record commands.
            let clear_color: [f32; 4] = [0.0, 0.2, 0.4, 1.0]; // https://github.com/microsoft/windows-rs/issues/790
            command_list.ClearRenderTargetView(rtv_handle, &clear_color[0], 0, std::ptr::null());
            command_list.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
            command_list.IASetVertexBuffers(0, 1, &resources.vbv);
            command_list.DrawInstanced(3, 1, 0, 0);

            // Indicate that the back buffer will now be used to present.
            command_list.ResourceBarrier(
                1,
                &transition_barrier(
                    &resources.render_targets[resources.frame_index as usize],
                    D3D12_RESOURCE_STATE_RENDER_TARGET,
                    D3D12_RESOURCE_STATE_PRESENT,
                ),
            );

            command_list.Close().ok()?;
        }

        Ok(())
    }

    fn transition_barrier(
        resource: &ID3D12Resource,
        state_before: D3D12_RESOURCE_STATES,
        state_after: D3D12_RESOURCE_STATES,
    ) -> D3D12_RESOURCE_BARRIER {
        D3D12_RESOURCE_BARRIER {
            Type: D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            Flags: D3D12_RESOURCE_BARRIER_FLAG_NONE,
            Anonymous: D3D12_RESOURCE_BARRIER_0 {
                Transition: D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: Some(resource.clone()),
                    StateBefore: state_before,
                    StateAfter: state_after,
                    Subresource: D3D12_RESOURCE_BARRIER_ALL_SUBRESOURCES,
                }
                .abi(),
            },
        }
    }

    fn create_device(command_line: &SampleCommandLine) -> Result<(IDXGIFactory4, ID3D12Device)> {
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

        let dxgi_factory: IDXGIFactory4 = unsafe { CreateDXGIFactory2(dxgi_factory_flags)? };

        let adapter = if command_line.use_warp_device {
            unsafe { dxgi_factory.EnumWarpAdapter() }
        } else {
            get_hardware_adapter(&dxgi_factory)
        }?;

        let device = unsafe { D3D12CreateDevice(adapter, D3D_FEATURE_LEVEL_11_0)? };
        Ok((dxgi_factory, device))
    }

    fn create_root_signature(device: &ID3D12Device) -> Result<ID3D12RootSignature> {
        let desc = D3D12_ROOT_SIGNATURE_DESC {
            Flags: D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT,
            ..Default::default()
        };

        let mut signature: Option<ID3DBlob> = None;
        let mut error: Option<ID3DBlob> = None;

        unsafe {
            D3D12SerializeRootSignature(
                &desc,
                D3D_ROOT_SIGNATURE_VERSION_1,
                &mut signature,
                &mut error,
            )
            .ok()?;

            assert!(signature.is_some());
            let signature = signature.unwrap();
            device.CreateRootSignature(0, signature.GetBufferPointer(), signature.GetBufferSize())
        }
    }

    fn create_pipeline_state(
        device: &ID3D12Device,
        root_signature: &ID3D12RootSignature,
    ) -> Result<ID3D12PipelineState> {
        let mut vertex_shader: Option<ID3DBlob> = None;
        let mut pixel_shader: Option<ID3DBlob> = None;

        let compile_flags = if cfg!(debug_assertions) {
            D3DCOMPILE_DEBUG | D3DCOMPILE_SKIP_OPTIMIZATION
        } else {
            0
        };

        let exe_path = std::env::current_exe().ok().unwrap();
        let asset_path = exe_path.parent().unwrap();
        let shaders_hlsl_path = asset_path.join("shaders.hlsl");
        let shaders_hlsl = shaders_hlsl_path.to_str().unwrap();

        unsafe {
            D3DCompileFromFile(
                shaders_hlsl,
                std::ptr::null_mut(),
                None,
                "VSMain",
                "vs_5_0",
                compile_flags,
                0,
                &mut vertex_shader,
                std::ptr::null_mut(),
            )
            .ok()?;

            D3DCompileFromFile(
                shaders_hlsl,
                std::ptr::null_mut(),
                None,
                "PSMain",
                "ps_5_0",
                compile_flags,
                0,
                &mut pixel_shader,
                std::ptr::null_mut(),
            )
            .ok()?;
        }

        let vertex_shader = vertex_shader.unwrap();
        let pixel_shader = pixel_shader.unwrap();

        let mut input_element_descs: [D3D12_INPUT_ELEMENT_DESC; 2] = [
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: PSTR(b"POSITION\0".as_ptr() as _),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 0,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: PSTR(b"COLOR\0".as_ptr() as _),
                SemanticIndex: 0,
                Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                InputSlot: 0,
                AlignedByteOffset: 12,
                InputSlotClass: D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
                InstanceDataStepRate: 0,
            },
        ];

        unsafe {
            let mut desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
                InputLayout: D3D12_INPUT_LAYOUT_DESC {
                    pInputElementDescs: input_element_descs.as_mut_ptr(),
                    NumElements: input_element_descs.len() as u32,
                },
                pRootSignature: Some(root_signature.clone()), // << https://github.com/microsoft/windows-rs/discussions/623
                VS: D3D12_SHADER_BYTECODE {
                    pShaderBytecode: vertex_shader.GetBufferPointer(),
                    BytecodeLength: vertex_shader.GetBufferSize(),
                },
                PS: D3D12_SHADER_BYTECODE {
                    pShaderBytecode: pixel_shader.GetBufferPointer(),
                    BytecodeLength: pixel_shader.GetBufferSize(),
                },
                RasterizerState: D3D12_RASTERIZER_DESC {
                    FillMode: D3D12_FILL_MODE_SOLID,
                    CullMode: D3D12_CULL_MODE_NONE,
                    ..Default::default()
                },
                BlendState: D3D12_BLEND_DESC {
                    AlphaToCoverageEnable: false.into(),
                    IndependentBlendEnable: false.into(),
                    RenderTarget: [
                        D3D12_RENDER_TARGET_BLEND_DESC {
                            BlendEnable: false.into(),
                            LogicOpEnable: false.into(),
                            SrcBlend: D3D12_BLEND_ONE,
                            DestBlend: D3D12_BLEND_ZERO,
                            BlendOp: D3D12_BLEND_OP_ADD,
                            SrcBlendAlpha: D3D12_BLEND_ONE,
                            DestBlendAlpha: D3D12_BLEND_ZERO,
                            BlendOpAlpha: D3D12_BLEND_OP_ADD,
                            LogicOp: D3D12_LOGIC_OP_NOOP,
                            RenderTargetWriteMask: D3D12_COLOR_WRITE_ENABLE_ALL.0 as u8,
                        },
                        D3D12_RENDER_TARGET_BLEND_DESC::default(),
                        D3D12_RENDER_TARGET_BLEND_DESC::default(),
                        D3D12_RENDER_TARGET_BLEND_DESC::default(),
                        D3D12_RENDER_TARGET_BLEND_DESC::default(),
                        D3D12_RENDER_TARGET_BLEND_DESC::default(),
                        D3D12_RENDER_TARGET_BLEND_DESC::default(),
                        D3D12_RENDER_TARGET_BLEND_DESC::default(),
                    ],
                },
                DepthStencilState: D3D12_DEPTH_STENCIL_DESC::default(),
                SampleMask: u32::max_value(),
                PrimitiveTopologyType: D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
                NumRenderTargets: 1,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    ..Default::default()
                },
                ..Default::default()
            };
            desc.RTVFormats[0] = DXGI_FORMAT_R8G8B8A8_UNORM;

            device.CreateGraphicsPipelineState(&desc)
        }
    }

    fn create_vertex_buffer(
        device: &ID3D12Device,
        aspect_ratio: f32,
    ) -> Result<(ID3D12Resource, D3D12_VERTEX_BUFFER_VIEW)> {
        let vertices: [Vertex; 3] = [
            Vertex {
                position: [0.0, 0.25 * aspect_ratio, 0.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.25, -0.25 * aspect_ratio, 0.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.25, -0.25 * aspect_ratio, 0.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ];

        // Note: using upload heaps to transfer static data like vert buffers is
        // not recommended. Every time the GPU needs it, the upload heap will be
        // marshalled over. Please read up on Default Heap usage. An upload heap
        // is used here for code simplicity and because there are very few verts
        // to actually transfer.
        let vertex_buffer: ID3D12Resource = unsafe {
            device.CreateCommittedResource(
                &D3D12_HEAP_PROPERTIES {
                    Type: D3D12_HEAP_TYPE_UPLOAD,
                    ..Default::default()
                },
                D3D12_HEAP_FLAG_NONE,
                &D3D12_RESOURCE_DESC {
                    Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
                    Width: std::mem::size_of_val(&vertices) as u64,
                    Height: 1,
                    DepthOrArraySize: 1,
                    MipLevels: 1,
                    SampleDesc: DXGI_SAMPLE_DESC {
                        Count: 1,
                        Quality: 0,
                    },
                    Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
                    ..Default::default()
                },
                D3D12_RESOURCE_STATE_GENERIC_READ,
                std::ptr::null(),
            )?
        };

        // Copy the triangle data to the vertex buffer.
        let mut data: *mut std::ffi::c_void = std::ptr::null_mut();

        let mut vbv = D3D12_VERTEX_BUFFER_VIEW::default();

        unsafe {
            vertex_buffer.Map(0, std::ptr::null(), &mut data).ok()?;
            std::ptr::copy_nonoverlapping(
                vertices.as_ptr(),
                std::mem::transmute(data),
                std::mem::size_of_val(&vertices),
            );
            vertex_buffer.Unmap(0, std::ptr::null());

            vbv.BufferLocation = vertex_buffer.GetGPUVirtualAddress();
            vbv.StrideInBytes = std::mem::size_of::<Vertex>() as u32;
            vbv.SizeInBytes = std::mem::size_of_val(&vertices) as u32;
        }

        Ok((vertex_buffer, vbv))
    }

    #[repr(C)]
    struct Vertex {
        position: [f32; 3],
        color: [f32; 4],
    }

    fn wait_for_previous_frame(resources: &mut Resources) {
        // WAITING FOR THE FRAME TO COMPLETE BEFORE CONTINUING IS NOT BEST
        // PRACTICE. This is code implemented as such for simplicity. The
        // D3D12HelloFrameBuffering sample illustrates how to use fences for
        // efficient resource usage and to maximize GPU utilization.

        // Signal and increment the fence value.
        let fence = resources.fence_value;

        unsafe {
            resources
                .command_queue
                .Signal(&resources.fence, fence)
                .ok()
                .unwrap();

            resources.fence_value = resources.fence_value + 1;

            // Wait until the previous frame is finished.
            if resources.fence.GetCompletedValue() < fence {
                resources
                    .fence
                    .SetEventOnCompletion(fence, resources.fence_event)
                    .ok()
                    .unwrap();
                WaitForSingleObject(resources.fence_event, INFINITE);
            }

            resources.frame_index = resources.swap_chain.GetCurrentBackBufferIndex();
        }
    }
}

fn main() -> Result<()> {
    run_sample::<d3d12_hello_triangle::Sample>()?;

    Ok(())
}
