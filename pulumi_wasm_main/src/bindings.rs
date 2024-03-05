// Generated by `wit-bindgen` 0.20.0. DO NOT EDIT!
// Options used:
pub mod component {
    pub mod pulumi_wasm {

        #[allow(clippy::all)]
        pub mod output_interface {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;

            #[derive(Debug)]
            #[repr(transparent)]
            pub struct Output {
                handle: _rt::Resource<Output>,
            }

            impl Output {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }

                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }

                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }

            unsafe impl _rt::WasmResource for Output {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();

                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "component:pulumi-wasm/output-interface@0.1.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]output"]
                            fn drop(_: u32);
                        }

                        drop(_handle);
                    }
                }
            }

            #[allow(unused_unsafe, clippy::all)]
            pub fn create_struct(fields: &[(_rt::String, &Output)]) -> Output {
                unsafe {
                    let vec2 = fields;
                    let len2 = vec2.len();
                    let layout2 = _rt::alloc::Layout::from_size_align_unchecked(vec2.len() * 12, 4);
                    let result2 = if layout2.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout2).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout2);
                        }
                        ptr
                    } else {
                        {
                            ::core::ptr::null_mut()
                        }
                    };
                    for (i, e) in vec2.into_iter().enumerate() {
                        let base = result2.add(i * 12);
                        {
                            let (t0_0, t0_1) = e;
                            let vec1 = t0_0;
                            let ptr1 = vec1.as_ptr().cast::<u8>();
                            let len1 = vec1.len();
                            *base.add(4).cast::<usize>() = len1;
                            *base.add(0).cast::<*mut u8>() = ptr1.cast_mut();
                            *base.add(8).cast::<i32>() = (t0_1).handle() as i32;
                        }
                    }

                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "component:pulumi-wasm/output-interface@0.1.0")]
                    extern "C" {
                        #[link_name = "create-struct"]
                        fn wit_import(_: *mut u8, _: usize) -> i32;
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(result2, len2);
                    if layout2.size() != 0 {
                        _rt::alloc::dealloc(result2.cast(), layout2);
                    }
                    Output::from_handle(ret as u32)
                }
            }
            impl Output {
                #[allow(unused_unsafe, clippy::all)]
                pub fn new(value: &[u8]) -> Self {
                    unsafe {
                        let vec0 = value;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();

                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:pulumi-wasm/output-interface@0.1.0")]
                        extern "C" {
                            #[link_name = "[constructor]output"]
                            fn wit_import(_: *mut u8, _: usize) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import(ptr0.cast_mut(), len0);
                        Output::from_handle(ret as u32)
                    }
                }
            }
            impl Output {
                #[allow(unused_unsafe, clippy::all)]
                pub fn map(&self, function_name: &str) -> Output {
                    unsafe {
                        let vec0 = function_name;
                        let ptr0 = vec0.as_ptr().cast::<u8>();
                        let len0 = vec0.len();

                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:pulumi-wasm/output-interface@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]output.map"]
                            fn wit_import(_: i32, _: *mut u8, _: usize) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8, _: usize) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32, ptr0.cast_mut(), len0);
                        Output::from_handle(ret as u32)
                    }
                }
            }
            impl Output {
                #[allow(unused_unsafe, clippy::all)]
                pub fn get(&self) -> Option<_rt::Vec<u8>> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 12]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 12]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:pulumi-wasm/output-interface@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]output.get"]
                            fn wit_import(_: i32, _: *mut u8);
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                        match l1 {
                            0 => None,
                            1 => {
                                let e = {
                                    let l2 = *ptr0.add(4).cast::<*mut u8>();
                                    let l3 = *ptr0.add(8).cast::<usize>();
                                    let len4 = l3;

                                    _rt::Vec::from_raw_parts(l2.cast(), len4, len4)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        }
                    }
                }
            }
            impl Output {
                #[allow(unused_unsafe, clippy::all)]
                pub fn duplicate(&self) -> Output {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "component:pulumi-wasm/output-interface@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]output.duplicate"]
                            fn wit_import(_: i32) -> i32;
                        }

                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        Output::from_handle(ret as u32)
                    }
                }
            }
        }

        #[allow(clippy::all)]
        pub mod pulumi_provider_random_interface {
            #[used]
            #[doc(hidden)]
            #[cfg(target_arch = "wasm32")]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Output = super::super::super::component::pulumi_wasm::output_interface::Output;
            pub struct RandomStringArgs<'a> {
                pub name: _rt::String,
                pub length: &'a Output,
            }
            impl<'a> ::core::fmt::Debug for RandomStringArgs<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct("RandomStringArgs")
                        .field("name", &self.name)
                        .field("length", &self.length)
                        .finish()
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// create-random-string: func(args: random-string-args) -> random-string-result;
            pub fn create_random_string(args: &RandomStringArgs<'_>) {
                unsafe {
                    let RandomStringArgs {
                        name: name0,
                        length: length0,
                    } = args;
                    let vec1 = name0;
                    let ptr1 = vec1.as_ptr().cast::<u8>();
                    let len1 = vec1.len();

                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm/pulumi-provider-random-interface@0.1.0"
                    )]
                    extern "C" {
                        #[link_name = "create-random-string"]
                        fn wit_import(_: *mut u8, _: usize, _: i32);
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: i32) {
                        unreachable!()
                    }
                    wit_import(ptr1.cast_mut(), len1, (length0).handle() as i32);
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            pub fn handle_functions() {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(
                        wasm_import_module = "component:pulumi-wasm/pulumi-provider-random-interface@0.1.0"
                    )]
                    extern "C" {
                        #[link_name = "handle-functions"]
                        fn wit_import();
                    }

                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() {
                        unreachable!()
                    }
                    wit_import();
                }
            }
        }
    }
}
pub mod exports {
    pub mod component {
        pub mod pulumi_wasm {

            #[allow(clippy::all)]
            pub mod pulumi_main {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;

                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_main_cabi<T: Guest>() {
                    T::main();
                }
                pub trait Guest {
                    fn main();
                }
                #[doc(hidden)]

                macro_rules! __export_component_pulumi_wasm_pulumi_main_0_1_0_cabi{
        ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {


          #[export_name = "component:pulumi-wasm/pulumi-main@0.1.0#main"]
          unsafe extern "C" fn export_main() {
            $($path_to_types)*::_export_main_cabi::<$ty>()
          }
        };);
      }
                #[doc(hidden)]
                pub(crate) use __export_component_pulumi_wasm_pulumi_main_0_1_0_cabi;
            }
        }
    }
}
mod _rt {

    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        // NB: This would ideally be `u32` but it is not. The fact that this has
        // interior mutability is not exposed in the API of this type except for the
        // `take_handle` method which is supposed to in theory be private.
        //
        // This represents, almost all the time, a valid handle value. When it's
        // invalid it's stored as `u32::MAX`.
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }

    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }

    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }

        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }

        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }

    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource")
                .field("handle", &self.handle)
                .finish()
        }
    }

    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    // If this handle was "taken" then don't do anything in the
                    // destructor.
                    u32::MAX => {}

                    // ... but otherwise do actually destroy it with the imported
                    // component model intrinsic as defined through `T`.
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::alloc;
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_new_main_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::component::pulumi_wasm::pulumi_main::__export_component_pulumi_wasm_pulumi_main_0_1_0_cabi!($ty with_types_in $($path_to_types_root)*::exports::component::pulumi_wasm::pulumi_main);
  )
}
#[doc(inline)]
pub(crate) use __export_new_main_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.20.0:new-main:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 798] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x9f\x05\x01A\x02\x01\
A\x07\x01B\x11\x04\0\x06output\x03\x01\x01p}\x01i\0\x01@\x01\x05value\x01\0\x02\x04\
\0\x13[constructor]output\x01\x03\x01h\0\x01@\x02\x04self\x04\x0dfunction-names\0\
\x02\x04\0\x12[method]output.map\x01\x05\x01k\x01\x01@\x01\x04self\x04\0\x06\x04\
\0\x12[method]output.get\x01\x07\x01@\x01\x04self\x04\0\x02\x04\0\x18[method]out\
put.duplicate\x01\x08\x01o\x02s\x04\x01p\x09\x01@\x01\x06fields\x0a\0\x02\x04\0\x0d\
create-struct\x01\x0b\x03\x01,component:pulumi-wasm/output-interface@0.1.0\x05\0\
\x02\x03\0\0\x06output\x01B\x10\x02\x03\x02\x01\x01\x04\0\x06output\x03\0\0\x01k\
y\x01h\x01\x01k\x03\x01q\x02\x07literal\x01\x02\0\x03res\x01\x04\0\x04\0\x0aeith\
er-u32\x03\0\x05\x01r\x02\x04names\x06length\x03\x04\0\x12random-string-args\x03\
\0\x07\x01i\x01\x01r\x02\x03url\x09\x06result\x09\x04\0\x14random-string-result\x03\
\0\x0a\x01@\x01\x04args\x08\x01\0\x04\0\x14create-random-string\x01\x0c\x01@\0\x01\
\0\x04\0\x10handle-functions\x01\x0d\x03\x01<component:pulumi-wasm/pulumi-provid\
er-random-interface@0.1.0\x05\x02\x01B\x02\x01@\0\x01\0\x04\0\x04main\x01\0\x04\x01\
'component:pulumi-wasm/pulumi-main@0.1.0\x05\x03\x04\x01$component:pulumi-wasm/n\
ew-main@0.1.0\x04\0\x0b\x0e\x01\0\x08new-main\x03\0\0\0G\x09producers\x01\x0cpro\
cessed-by\x02\x0dwit-component\x070.201.0\x10wit-bindgen-rust\x060.20.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
