use crate::abi::*;
use std::ffi::c_void;
use std::ptr::NonNull;
use std::{
    mem::size_of,
    sync::atomic::{AtomicU32, AtomicUsize, Ordering},
};
use winrt::*;

#[repr(transparent)]
struct WeakRef {
    ptr: ComPtr<WeakRef>,
}

impl WeakRef {
    fn new(object: NonNullRawComPtr<IUnknown>, strong: u32) -> Self {
        impl_WeakRef::new(object, strong)
    }

    fn as_impl_ptr(&self) -> *const impl_WeakRef {
        self.as_iunknown().unwrap().as_raw() as *const _
    }
}

fn decode_weak_ref(reference: usize) -> *mut impl_WeakRef {
    reference.checked_shl(1).unwrap() as *mut _
}

fn encode_weak_ref(value: usize) -> usize {
    let pointer_flag = 1usize << (size_of::<usize>() * 8 - 1);
    assert_eq!(value & 1, 0);
    value >> 1 | pointer_flag
}

pub trait WeakRefObject {
    fn get_reference(&self) -> &AtomicUsize;

    fn make_weak_ref(
        &self,
        this: NonNullRawComPtr<IUnknown>,
    ) -> *const *const abi_IWeakReferenceSource {
        let references = self.get_reference();
        let count_or_pointer = references.load(Ordering::Relaxed);

        if (count_or_pointer as isize) < 0 {
            return unsafe { &*decode_weak_ref(count_or_pointer) }.get_source();
        }

        let weak_ref = WeakRef::new(this, count_or_pointer as u32);
        let encoding = encode_weak_ref(weak_ref.as_impl_ptr() as usize);

        loop {
            if references
                .compare_exchange_weak(
                    count_or_pointer,
                    encoding,
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                let result = unsafe { &*weak_ref.as_impl_ptr() }.get_source();
                std::mem::forget(weak_ref);
                return result;
            }
            if (count_or_pointer as isize) < 0 {
                return unsafe { &*decode_weak_ref(count_or_pointer) }.get_source();
            }
            unsafe { &*decode_weak_ref(count_or_pointer) }
                .strong
                .store(count_or_pointer as u32, Ordering::SeqCst);
        }
    }
    fn add_ref(&self) -> u32 {
        let references = self.get_reference();
        let count_or_pointer = references.load(Ordering::Relaxed);

        loop {
            if (count_or_pointer as isize) < 0 {
                return 1 + unsafe { &*decode_weak_ref(count_or_pointer) }
                    .strong
                    .fetch_add(1, Ordering::Relaxed);
            }

            let target = count_or_pointer + 1;
            if references
                .compare_exchange_weak(
                    count_or_pointer,
                    target,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return target as u32;
            }
        }
    }
    fn release(&self) -> u32 {
        let references = self.get_reference();
        let count_or_pointer = references.load(Ordering::Relaxed);

        loop {
            if (count_or_pointer as isize) < 0 {
                let weakref_ptr = decode_weak_ref(count_or_pointer);
                let target = unsafe { &*weakref_ptr }
                    .strong
                    .fetch_sub(1, Ordering::Release)
                    - 1;
                if target == 0 {
                    (impl_WeakRef::VTABLE.iunknown.unknown_release)(NonNullRawComPtr::new(
                        NonNull::new(weakref_ptr as *mut _).unwrap(),
                    ));
                }
                return target;
            }

            let target = count_or_pointer - 1;
            if references
                .compare_exchange_weak(
                    count_or_pointer,
                    target,
                    Ordering::Release,
                    Ordering::Relaxed,
                )
                .is_ok()
            {
                return target as u32;
            }
        }
    }
}

unsafe impl AbiTransferable for WeakRef {
    type Abi = RawComPtr<Self>;
    fn get_abi(&self) -> Self::Abi {
        <ComPtr<Self> as AbiTransferable>::get_abi(&self.ptr)
    }
    fn set_abi(&mut self) -> *mut Self::Abi {
        <ComPtr<Self> as AbiTransferable>::set_abi(&mut self.ptr)
    }
}

unsafe impl ComInterface for WeakRef {
    type VTable = abi_IXamlType;
    fn iid() -> Guid {
        Guid::from_values(
            0x00000037,
            0x0000,
            0x0000,
            [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
        )
    }
}

#[repr(C)]
pub struct impl_WeakRef {
    vtable: *const abi_IWeakReference,
    vtable_source: *const abi_IWeakReferenceSource,
    object: NonNullRawComPtr<IUnknown>,
    strong: AtomicU32,
    weak: AtomicU32,
}

impl impl_WeakRef {
    const VTABLE: abi_IWeakReference = abi_IWeakReference {
        iunknown: abi_IUnknown {
            unknown_query_interface: Self::unknown_query_interface,
            unknown_add_ref: Self::unknown_add_ref,
            unknown_release: Self::unknown_release,
        },
        resolve: Self::resolve,
    };
    const SOURCE_VTABLE: abi_IWeakReferenceSource = abi_IWeakReferenceSource {
        iunknown: abi_IUnknown {
            unknown_query_interface: Self::source_unknown_query_interface,
            unknown_add_ref: Self::source_unknown_add_ref,
            unknown_release: Self::source_unknown_release,
        },
        get_weak_reference: Self::get_weak_reference,
    };

    fn new(object: NonNullRawComPtr<IUnknown>, strong: u32) -> WeakRef {
        let value = Self {
            vtable: &Self::VTABLE,
            vtable_source: &Self::SOURCE_VTABLE,
            object,
            strong: strong.into(),
            weak: AtomicU32::new(1),
        };
        unsafe {
            let mut result = std::mem::zeroed();
            let ptr: NonNull<Self> = NonNull::new_unchecked(Box::into_raw(Box::new(value)));
            *<WeakRef as AbiTransferable>::set_abi(&mut result) =
                Some(NonNullRawComPtr::new(ptr.cast()));

            result
        }
    }

    pub fn get_source(&self) -> *const *const abi_IWeakReferenceSource {
        self.strong.fetch_add(1, Ordering::Relaxed);
        &self.vtable_source as *const _
    }

    extern "system" fn unknown_query_interface(
        this: NonNullRawComPtr<IUnknown>,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> ErrorCode {
        let interface = unsafe { interface.as_mut().unwrap() };
        let this: &mut Self = unsafe { std::mem::transmute(this) };
        if iid == &<WeakRef as ComInterface>::iid() || iid == &<IUnknown as ComInterface>::iid() {
            *interface = this as *mut Self as *mut _;
            this.weak.fetch_add(1, Ordering::Relaxed);
            return ErrorCode(0);
        }

        *interface = std::ptr::null_mut();
        ErrorCode(0x80004002)
    }
    extern "system" fn unknown_add_ref(this: NonNullRawComPtr<IUnknown>) -> u32 {
        let this: &mut Self = unsafe { std::mem::transmute(this) };
        this.weak.fetch_add(1, Ordering::Relaxed) + 1
    }
    extern "system" fn unknown_release(this: NonNullRawComPtr<IUnknown>) -> u32 {
        unsafe {
            let this: *mut Self = this.as_raw() as _;
            let remaining = (*this).weak.fetch_sub(1, Ordering::Relaxed) - 1;
            if remaining == 0 {
                Box::from_raw(this);
            }
            remaining
        }
    }
    extern "system" fn resolve(
        this: NonNullRawComPtr<IUnknown>,
        id: *const Guid,
        object_reference: *mut *mut IUnknown,
    ) -> i32 {
        let this: &mut Self = unsafe { std::mem::transmute(this) };
        let target = this.strong.load(Ordering::Relaxed);
        loop {
            if target == 0 {
                unsafe { *object_reference = std::ptr::null_mut() };
                return 0;
            }

            if this
                .strong
                .compare_exchange_weak(target, target + 1, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                let hr = unsafe {
                    (this.object.vtable().unknown_query_interface)(
                        this.object,
                        &*id,
                        object_reference as *mut _,
                    )
                };
                this.strong.fetch_sub(1, Ordering::Relaxed);
                return hr.0 as i32;
            }
        }
    }

    extern "system" fn source_unknown_query_interface(
        this: NonNullRawComPtr<IUnknown>,
        iid: &Guid,
        interface: *mut RawPtr,
    ) -> ErrorCode {
        let that: &mut Self = unsafe {
            std::mem::transmute(this.as_raw() as usize - size_of::<*const abi_IWeakReference>())
        };

        if iid
            == &Guid::from_values(
                0x00000038,
                0x0000,
                0x0000,
                [0xC0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46],
            )
        {
            unsafe { *interface = this.as_raw() as *mut _ };
            that.strong.fetch_add(1, Ordering::Relaxed);
            return ErrorCode(0);
        }
        unsafe {
            (that.object.vtable().unknown_query_interface)(that.object, &*iid, interface as *mut _)
        }
    }
    extern "system" fn source_unknown_add_ref(this: NonNullRawComPtr<IUnknown>) -> u32 {
        let that: &mut Self = unsafe {
            std::mem::transmute(this.as_raw() as usize - size_of::<*const abi_IWeakReference>())
        };
        that.strong.fetch_add(1, Ordering::Relaxed) + 1
    }
    extern "system" fn source_unknown_release(this: NonNullRawComPtr<IUnknown>) -> u32 {
        let that: &mut Self = unsafe {
            std::mem::transmute(this.as_raw() as usize - size_of::<*const abi_IWeakReference>())
        };
        (that.object.vtable().unknown_release)(that.object)
    }

    extern "system" fn get_weak_reference(
        this: NonNullRawComPtr<IUnknown>,
        result: *mut *mut c_void,
    ) -> i32 {
        let that = (this.as_raw() as usize - size_of::<*const abi_IWeakReference>()) as *mut _;
        unsafe { *result = that };
        let that: &mut Self = unsafe { &mut *(that as *mut _) };
        that.weak.fetch_add(1, Ordering::Relaxed);
        0
    }
}
