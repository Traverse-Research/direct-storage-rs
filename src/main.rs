mod DStorage;
use std::mem::ManuallyDrop;
use windows::Win32::Foundation::{FALSE, TRUE};
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_12_1;
use windows::Win32::Graphics::Direct3D12::D3D12CreateDevice;
use windows::Win32::Graphics::Direct3D12::ID3D12Device;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows::Win32::Storage::FileSystem::BY_HANDLE_FILE_INFORMATION;
use windows::Win32::System::Threading::{CreateEventA, WaitForSingleObject, INFINITE};
use windows_core::w;
use windows_core::{IUnknown, Interface};
use DStorage::Direct3D::DirectStorage::*;
use DStorage::Direct3D::DirectStorage::{IDStorageFactory, IDStorageFile};

fn main() {
    let mut device: Option<ID3D12Device> = None;
    unsafe { D3D12CreateDevice(None, D3D_FEATURE_LEVEL_12_1, &mut device) };
    let device = ManuallyDrop::new(device);

    let factory = unsafe { DStorageGetFactory::<IDStorageFactory>() }.unwrap();

    let file: IDStorageFile = unsafe { factory.OpenFile(w!("./bindings.txt")) }.unwrap();
    let mut info = BY_HANDLE_FILE_INFORMATION::default();
    unsafe { file.GetFileInformation(&mut info) }.unwrap();
    let fileSize = info.nFileSizeLow;

    let queueDesc = DSTORAGE_QUEUE_DESC {
        Capacity: DSTORAGE_MAX_QUEUE_CAPACITY as u16,
        Priority: DSTORAGE_PRIORITY_NORMAL,
        SourceType: DSTORAGE_REQUEST_SOURCE_FILE,
        Device: device.clone(),
        ..Default::default()
    };

    let queue: IDStorageQueue = unsafe { factory.CreateQueue(&queueDesc).unwrap() };

    let bufferHeapProps = D3D12_HEAP_PROPERTIES {
        Type: D3D12_HEAP_TYPE_DEFAULT,
        ..Default::default()
    };

    let bufferDesc = D3D12_RESOURCE_DESC {
        Dimension: D3D12_RESOURCE_DIMENSION_BUFFER,
        Width: fileSize as u64,
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

    let mut bufferResource: Option<ID3D12Resource> = None;
    unsafe {
        device
            .as_ref()
            .unwrap()
            .CreateCommittedResource(
                &bufferHeapProps,
                D3D12_HEAP_FLAG_NONE,
                &bufferDesc,
                D3D12_RESOURCE_STATE_COMMON,
                None,
                &mut bufferResource,
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
                Size: fileSize,
            }),
        },
        UncompressedSize: fileSize,
        Destination: DSTORAGE_DESTINATION {
            Buffer: ManuallyDrop::new(DSTORAGE_DESTINATION_BUFFER {
                Resource: ManuallyDrop::new(bufferResource.clone()),
                Offset: 0,
                Size: fileSize,
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

    let fenceEvent = unsafe { CreateEventA(None, FALSE, FALSE, None) }.unwrap();
    let fenceValue = 1;
    unsafe { fence.SetEventOnCompletion(fenceValue, fenceEvent) };
    unsafe { queue.EnqueueSignal(&fence, fenceValue) };

    unsafe { queue.Submit() };

    println!("Waiting for the DirectStorage request to complete...");
    unsafe { WaitForSingleObject(fenceEvent, INFINITE) };

    let mut errorRecord = DSTORAGE_ERROR_RECORD::default();
    unsafe { queue.RetrieveErrorRecord(&mut errorRecord) };

    dbg!(errorRecord.FailureCount);
    dbg!(errorRecord.FirstFailure.HResult);
}
