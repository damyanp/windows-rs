#![allow(
    non_upper_case_globals,
    non_snake_case,
    non_camel_case_types,
    dead_code,
    unused_variables
)]

use crate::*;
use std::sync::atomic::AtomicIsize;

/// A thread-safe reference count for use with COM weak reference implementations.
#[repr(transparent)]
#[derive(Default)]
pub struct WeakRefCount(AtomicIsize);

impl WeakRefCount {
    pub fn new() -> Self {
        Self(AtomicIsize::new(1))
    }

    pub fn add_ref(&self) -> u32 {
        panic!();
    }

    pub fn release(&self) -> u32 {
        panic!();
    }
}

#[repr(C)]
struct TearOff {
    strong_vtable: *const IWeakReferenceSource_vtable,
    weak_vtable: *const IWeakReference_vtable,
    object: RawPtr,
    strong_count: RefCount,
    weak_count: RefCount,
}

impl TearOff {
    fn new(object: RawPtr, strong_count: u32) -> IWeakReferenceSource {
        unsafe {
            std::mem::transmute(::std::boxed::Box::new(TearOff {
                strong_vtable: &Self::STRONG_VTABLE,
                weak_vtable: &Self::WEAK_VTABLE,
                object,
                strong_count: RefCount::new(strong_count),
                weak_count: RefCount::new(1),
            }))
        }
    }

    const STRONG_VTABLE: IWeakReferenceSource_vtable = IWeakReferenceSource_vtable(
        Self::StrongQueryInterface,
        Self::StrongAddRef,
        Self::StrongRelease,
        Self::StrongDowngrade,
    );

    const WEAK_VTABLE: IWeakReference_vtable = IWeakReference_vtable(
        Self::WeakQueryInterface,
        Self::WeakAddRef,
        Self::WeakRelease,
        Self::WeakUpgrade,
    );

    unsafe fn from_strong_ptr<'a>(this: RawPtr) -> &'a mut Self {
        &mut *(this as *mut RawPtr as *mut Self)
    }

    unsafe fn from_weak_ptr<'a>(this: RawPtr) -> &'a mut Self {
        &mut *((this as *mut RawPtr).sub(1) as *mut Self)
    }

    unsafe extern "system" fn StrongQueryInterface(
        ptr: RawPtr,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> HRESULT {
        let this = Self::from_strong_ptr(ptr);

        // Only directly respond to queries for the the tear-off's strong interface. This is
        // effectively a self-query.
        if iid == &IWeakReferenceSource::IID {
            *interface = ptr;
            this.strong_count.add_ref();
            return HRESULT(0);
        }

        // As the tear-off is sharing the identity of the object, simply delegate any remaining
        // queries to the object.
        ((*(*(this.object as *mut *mut _) as *mut IUnknown_vtable)).0)(this.object, iid, interface)
    }

    unsafe extern "system" fn WeakQueryInterface(
        ptr: RawPtr,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> HRESULT {
        let this = Self::from_weak_ptr(ptr);

        // While the weak vtable is packed into the same allocation as the strong vtable and
        // tear-off, it represents a distinct COM identity and thus does not share or delegate to
        // the object.

        *interface =
            if iid == &IWeakReference::IID || iid == &IUnknown::IID || iid == &IAgileObject::IID {
                ptr
            } else {
                std::ptr::null_mut()
            };

        // TODO: implement IMarshal

        if (*interface).is_null() {
            HRESULT(0x8000_4002) // E_NOINTERFACE
        } else {
            this.weak_count.add_ref();
            HRESULT(0)
        }
    }

    unsafe extern "system" fn StrongAddRef(ptr: ::windows::RawPtr) -> u32 {
        let this = Self::from_strong_ptr(ptr);

        // Implement `AddRef` directly as we own the strong reference.
        this.strong_count.add_ref()
    }

    unsafe extern "system" fn WeakAddRef(ptr: ::windows::RawPtr) -> u32 {
        let this = Self::from_weak_ptr(ptr);

        // Implement `AddRef` directly as we own the weak reference.
        this.weak_count.add_ref()
    }

    unsafe extern "system" fn StrongRelease(ptr: ::windows::RawPtr) -> u32 {
        let this = Self::from_strong_ptr(ptr);

        // Forward strong `Release` to the object so that it can destroy itself. It will then
        // decrement its weak reference and allow the tear-off to be released as needd.
        ((*(*(this.object as *mut *mut _) as *mut IUnknown_vtable)).2)((*this).object)
    }

    unsafe extern "system" fn WeakRelease(ptr: ::windows::RawPtr) -> u32 {
        let this = Self::from_weak_ptr(ptr);

        // Implement `Release` directly as we own the weak reference.
        let remaining = (*this).weak_count.release();

        // If there are no remaining references, it means that the object has already been
        // destroyed. Go ahead and destroy the tear-off.
        if remaining == 0 {
            Box::from_raw(this);
        }

        remaining
    }

    unsafe extern "system" fn StrongDowngrade(ptr: RawPtr, interface: *mut RawPtr) -> HRESULT {
        let this = Self::from_strong_ptr(ptr);

        // The strong vtable hands out a reference to the weak vtable. This is always safe and
        // straightforward since a strong refernece guarantees there is at lerast one weak
        // reference.
        *interface = &mut this.weak_vtable as *mut _ as _;
        this.weak_count.add_ref();
        HRESULT(0)
    }

    unsafe extern "system" fn WeakUpgrade(
        ptr: RawPtr,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> HRESULT {
        let this = Self::from_weak_ptr(ptr);

        panic!();
    }
}
