mod DStorage;
use DStorage::Direct3D::DirectStorage::DStorageGetFactory;
use DStorage::Direct3D::DirectStorage::IDStorageFactory;

fn main() {
    let factory = unsafe { DStorageGetFactory::<IDStorageFactory>() };
}
