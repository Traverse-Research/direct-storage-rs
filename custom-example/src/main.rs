use std::mem::ManuallyDrop;
use windows::Win32::Foundation::FALSE;
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_12_1;
use windows::Win32::Graphics::Direct3D12::D3D12CreateDevice;
use windows::Win32::Graphics::Direct3D12::ID3D12Device;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION;
use windows::Win32::System::Threading::{CreateEventA, WaitForSingleObject, INFINITE};
use windows::core::w;
use direct_storage_rs::Direct3D::DirectStorage::*;
use direct_storage_rs::Direct3D::DirectStorage::{IDStorageFactory, IDStorageFile};

fn main() {
    let mut device: Option<ID3D12Device> = None;
    unsafe { D3D12CreateDevice(None, D3D_FEATURE_LEVEL_12_1, &mut device) }.unwrap();
    let device = ManuallyDrop::new(device);

    let factory = unsafe { DStorageGetFactory::<IDStorageFactory>() }.unwrap();

    let file: IDStorageFile = unsafe { factory.OpenFile(w!("./bindings.txt")) }.unwrap();
    let mut info = BY_HANDLE_FILE_INFORMATION::default();
    unsafe { file.GetFileInformation(&mut info) }.unwrap();
    let file_size = info.nFileSizeLow;

    let queue_desc = DSTORAGE_QUEUE_DESC {
        Capacity: DSTORAGE_MAX_QUEUE_CAPACITY as u16,
        Priority: DSTORAGE_PRIORITY_NORMAL,
        SourceType: DSTORAGE_REQUEST_SOURCE_FILE,
        Device: device.clone(),
        ..Default::default()
    };

    let queue: IDStorageQueue = unsafe { factory.CreateQueue(&queue_desc).unwrap() };

    let buffer_heap_props = D3D12_HEAP_PROPERTIES {
        Type: D3D12_HEAP_TYPE_DEFAULT,
        ..Default::default()
    };

    let buffer_desc = D3D12_RESOURCE_DESC {
        Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
        Width: file_size as u64,
        Height: 1,
        DepthOrArraySize: 1,
        MipLevels: 1,
        Format: DXGI_FORMAT_UNKNOWN,
        Layout: D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut buffer_resource: Option<ID3D12Resource> = None;
    unsafe {
        device
            .as_ref()
            .unwrap()
            .CreateCommittedResource(
                &buffer_heap_props,
                D3D12_HEAP_FLAG_NONE,
                &buffer_desc,
                D3D12_RESOURCE_STATE_COMMON,
                None,
                &mut buffer_resource,
            )
            .unwrap()
    };

    let request = DSTORAGE_REQUEST {
        Options: DSTORAGE_REQUEST_OPTIONS {
            _bitfield1: 0,
            Reserved1: Default::default(),
            _bitfield2: DSTORAGE_REQUEST_SOURCE_FILE.0
                | (DSTORAGE_REQUEST_DESTINATION_BUFFER.0 << 1),
        },
        Source: DSTORAGE_SOURCE {
            File: ManuallyDrop::new(DSTORAGE_SOURCE_FILE {
                Source: ManuallyDrop::new(Some(file.clone())),
                Offset: 0,
                Size: file_size,
            }),
        },
        UncompressedSize: file_size,
        Destination: DSTORAGE_DESTINATION {
            Buffer: ManuallyDrop::new(DSTORAGE_DESTINATION_BUFFER {
                Resource: ManuallyDrop::new(buffer_resource.clone()),
                Offset: 0,
                Size: file_size,
            }),
        },
        ..Default::default()
    };

    unsafe { queue.EnqueueRequest(&request) };

    let fence: ID3D12Fence = unsafe {
        device
            .as_ref()
            .unwrap()
            .CreateFence(0, D3D12_FENCE_FLAG_NONE)
    }
    .unwrap();

    let fence_event = unsafe { CreateEventA(None, FALSE, FALSE, None) }.unwrap();
    let fence_value = 1;
    unsafe { fence.SetEventOnCompletion(fence_value, fence_event) }.unwrap();
    unsafe { queue.EnqueueSignal(&fence, fence_value) };

    unsafe { queue.Submit() };

    println!("Waiting for the DirectStorage request to complete...");
    unsafe { WaitForSingleObject(fence_event, INFINITE) };

    let mut error_record = DSTORAGE_ERROR_RECORD::default();
    unsafe { queue.RetrieveErrorRecord(&mut error_record) };

    println!("{:?}", error_record.FailureCount);
    println!("{:?}", error_record.FirstFailure.HResult);
}
