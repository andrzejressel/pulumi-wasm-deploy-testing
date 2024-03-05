// Generated by `wit-bindgen` 0.18.0. DO NOT EDIT!
pub mod component {
  pub mod pulumi_wasm {
    
    #[allow(clippy::all)]
    pub mod output_interface {
      #[used]
      #[doc(hidden)]
      #[cfg(target_arch = "wasm32")]
      static __FORCE_SECTION_REF: fn() = super::super::super::__link_section;
      
      #[derive(Debug)]
      #[repr(transparent)]
      pub struct Output{
        handle: wit_bindgen::rt::Resource<Output>,
      }
      
      impl Output{
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
          Self {
            handle: wit_bindgen::rt::Resource::from_handle(handle),
          }
        }
        
        #[doc(hidden)]
        pub fn into_handle(self) -> u32 {
          wit_bindgen::rt::Resource::into_handle(self.handle)
        }
        
        #[doc(hidden)]
        pub fn handle(&self) -> u32 {
          wit_bindgen::rt::Resource::handle(&self.handle)
        }
      }
      
      
      unsafe impl wit_bindgen::rt::WasmResource for Output{
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
      pub fn create_struct(fields: &[(wit_bindgen::rt::string::String,&Output,)],) -> Output{
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        unsafe {
          let vec2 = fields;
          let len2 = vec2.len() as i32;
          let layout2 = alloc::Layout::from_size_align_unchecked(vec2.len() * 12, 4);
          let result2 = if layout2.size() != 0
          {
            let ptr = alloc::alloc(layout2);
            if ptr.is_null()
            {
              alloc::handle_alloc_error(layout2);
            }
            ptr
          }else {{
            ::core::ptr::null_mut()
          }};
          for (i, e) in vec2.into_iter().enumerate() {
            let base = result2 as i32 + (i as i32) * 12;
            {
              let (t0_0, t0_1, ) = e;
              let vec1 = t0_0;
              let ptr1 = vec1.as_ptr() as i32;
              let len1 = vec1.len() as i32;
              *((base + 4) as *mut i32) = len1;
              *((base + 0) as *mut i32) = ptr1;
              *((base + 8) as *mut i32) = (t0_1).handle() as i32;
            }
          }
          
          #[cfg(target_arch = "wasm32")]
          #[link(wasm_import_module = "component:pulumi-wasm/output-interface@0.1.0")]
          extern "C" {
            #[link_name = "create-struct"]
            fn wit_import(_: i32, _: i32, ) -> i32;
          }
          
          #[cfg(not(target_arch = "wasm32"))]
          fn wit_import(_: i32, _: i32, ) -> i32{ unreachable!() }
          let ret = wit_import(result2 as i32, len2);
          if layout2.size() != 0 {
            alloc::dealloc(result2, layout2);
          }
          Output::from_handle(ret as u32)
        }
      }
      impl Output {
        #[allow(unused_unsafe, clippy::all)]
        pub fn new(value: &[u8],) -> Self{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            let vec0 = value;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "component:pulumi-wasm/output-interface@0.1.0")]
            extern "C" {
              #[link_name = "[constructor]output"]
              fn wit_import(_: i32, _: i32, ) -> i32;
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ) -> i32{ unreachable!() }
            let ret = wit_import(ptr0, len0);
            Output::from_handle(ret as u32)
          }
        }
      }
      impl Output {
        #[allow(unused_unsafe, clippy::all)]
        pub fn map(&self,function_name: &str,) -> Output{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            let vec0 = function_name;
            let ptr0 = vec0.as_ptr() as i32;
            let len0 = vec0.len() as i32;
            
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "component:pulumi-wasm/output-interface@0.1.0")]
            extern "C" {
              #[link_name = "[method]output.map"]
              fn wit_import(_: i32, _: i32, _: i32, ) -> i32;
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, _: i32, ) -> i32{ unreachable!() }
            let ret = wit_import((self).handle() as i32, ptr0, len0);
            Output::from_handle(ret as u32)
          }
        }
      }
      impl Output {
        #[allow(unused_unsafe, clippy::all)]
        pub fn get(&self,) -> Option<wit_bindgen::rt::vec::Vec::<u8>>{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[repr(align(4))]
            struct RetArea([u8; 12]);
            let mut ret_area = ::core::mem::MaybeUninit::<RetArea>::uninit();
            let ptr0 = ret_area.as_mut_ptr() as i32;
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "component:pulumi-wasm/output-interface@0.1.0")]
            extern "C" {
              #[link_name = "[method]output.get"]
              fn wit_import(_: i32, _: i32, );
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, _: i32, ){ unreachable!() }
            wit_import((self).handle() as i32, ptr0);
            let l1 = i32::from(*((ptr0 + 0) as *const u8));
            match l1 {
              0 => None,
              1 => {
                let e = {
                  let l2 = *((ptr0 + 4) as *const i32);
                  let l3 = *((ptr0 + 8) as *const i32);
                  let len4 = l3 as usize;
                  
                  Vec::from_raw_parts(l2 as *mut _, len4, len4)
                };
                Some(e)
              }
              _ => wit_bindgen::rt::invalid_enum_discriminant(),
            }
          }
        }
      }
      impl Output {
        #[allow(unused_unsafe, clippy::all)]
        pub fn duplicate(&self,) -> Output{
          
          #[allow(unused_imports)]
          use wit_bindgen::rt::{alloc, vec::Vec, string::String};
          unsafe {
            
            #[cfg(target_arch = "wasm32")]
            #[link(wasm_import_module = "component:pulumi-wasm/output-interface@0.1.0")]
            extern "C" {
              #[link_name = "[method]output.duplicate"]
              fn wit_import(_: i32, ) -> i32;
            }
            
            #[cfg(not(target_arch = "wasm32"))]
            fn wit_import(_: i32, ) -> i32{ unreachable!() }
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
      static __FORCE_SECTION_REF: fn() = super::super::super::__link_section;
      pub type Output = super::super::super::component::pulumi_wasm::output_interface::Output;
      pub struct RandomStringArgs<'a,> {
        pub name: wit_bindgen::rt::string::String,
        pub length: &'a Output,
      }
      impl<'a,> ::core::fmt::Debug for RandomStringArgs<'a,> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
          f.debug_struct("RandomStringArgs").field("name", &self.name).field("length", &self.length).finish()
        }
      }
      #[allow(unused_unsafe, clippy::all)]
      /// create-random-string: func(args: random-string-args) -> random-string-result;
      pub fn create_random_string(args: RandomStringArgs<'_,>,){
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        unsafe {
          let RandomStringArgs{ name:name0, length:length0, } = args;
          let vec1 = name0;
          let ptr1 = vec1.as_ptr() as i32;
          let len1 = vec1.len() as i32;
          
          #[cfg(target_arch = "wasm32")]
          #[link(wasm_import_module = "component:pulumi-wasm/pulumi-provider-random-interface@0.1.0")]
          extern "C" {
            #[link_name = "create-random-string"]
            fn wit_import(_: i32, _: i32, _: i32, );
          }
          
          #[cfg(not(target_arch = "wasm32"))]
          fn wit_import(_: i32, _: i32, _: i32, ){ unreachable!() }
          wit_import(ptr1, len1, (length0).handle() as i32);
        }
      }
      #[allow(unused_unsafe, clippy::all)]
      pub fn handle_functions(){
        
        #[allow(unused_imports)]
        use wit_bindgen::rt::{alloc, vec::Vec, string::String};
        unsafe {
          
          #[cfg(target_arch = "wasm32")]
          #[link(wasm_import_module = "component:pulumi-wasm/pulumi-provider-random-interface@0.1.0")]
          extern "C" {
            #[link_name = "handle-functions"]
            fn wit_import();
          }
          
          #[cfg(not(target_arch = "wasm32"))]
          fn wit_import(){ unreachable!() }
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
        static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_section;
        const _: () = {
          
          #[doc(hidden)]
          #[export_name = "component:pulumi-wasm/pulumi-main@0.1.0#main"]
          #[allow(non_snake_case)]
          unsafe extern "C" fn __export_main() {
            #[allow(unused_imports)]
            use wit_bindgen::rt::{alloc, vec::Vec, string::String};
            
            // Before executing any other code, use this function to run all static
            // constructors, if they have not yet been run. This is a hack required
            // to work around wasi-libc ctors calling import functions to initialize
            // the environment.
            //
            // This functionality will be removed once rust 1.69.0 is stable, at which
            // point wasi-libc will no longer have this behavior.
            //
            // See
            // https://github.com/bytecodealliance/preview2-prototyping/issues/99
            // for more details.
            #[cfg(target_arch="wasm32")]
            wit_bindgen::rt::run_ctors_once();
            
            <_GuestImpl as Guest>::main();
          }
        };
        use super::super::super::super::super::Component as _GuestImpl;
        pub trait Guest {
          fn main();
        }
        
      }
      
    }
  }
}

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:new-main"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 797] = [0, 97, 115, 109, 13, 0, 1, 0, 0, 25, 22, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 45, 101, 110, 99, 111, 100, 105, 110, 103, 4, 0, 7, 159, 5, 1, 65, 2, 1, 65, 7, 1, 66, 17, 4, 0, 6, 111, 117, 116, 112, 117, 116, 3, 1, 1, 112, 125, 1, 105, 0, 1, 64, 1, 5, 118, 97, 108, 117, 101, 1, 0, 2, 4, 0, 19, 91, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 93, 111, 117, 116, 112, 117, 116, 1, 3, 1, 104, 0, 1, 64, 2, 4, 115, 101, 108, 102, 4, 13, 102, 117, 110, 99, 116, 105, 111, 110, 45, 110, 97, 109, 101, 115, 0, 2, 4, 0, 18, 91, 109, 101, 116, 104, 111, 100, 93, 111, 117, 116, 112, 117, 116, 46, 109, 97, 112, 1, 5, 1, 107, 1, 1, 64, 1, 4, 115, 101, 108, 102, 4, 0, 6, 4, 0, 18, 91, 109, 101, 116, 104, 111, 100, 93, 111, 117, 116, 112, 117, 116, 46, 103, 101, 116, 1, 7, 1, 64, 1, 4, 115, 101, 108, 102, 4, 0, 2, 4, 0, 24, 91, 109, 101, 116, 104, 111, 100, 93, 111, 117, 116, 112, 117, 116, 46, 100, 117, 112, 108, 105, 99, 97, 116, 101, 1, 8, 1, 111, 2, 115, 4, 1, 112, 9, 1, 64, 1, 6, 102, 105, 101, 108, 100, 115, 10, 0, 2, 4, 0, 13, 99, 114, 101, 97, 116, 101, 45, 115, 116, 114, 117, 99, 116, 1, 11, 3, 1, 44, 99, 111, 109, 112, 111, 110, 101, 110, 116, 58, 112, 117, 108, 117, 109, 105, 45, 119, 97, 115, 109, 47, 111, 117, 116, 112, 117, 116, 45, 105, 110, 116, 101, 114, 102, 97, 99, 101, 64, 48, 46, 49, 46, 48, 5, 0, 2, 3, 0, 0, 6, 111, 117, 116, 112, 117, 116, 1, 66, 16, 2, 3, 2, 1, 1, 4, 0, 6, 111, 117, 116, 112, 117, 116, 3, 0, 0, 1, 107, 121, 1, 104, 1, 1, 107, 3, 1, 113, 2, 7, 108, 105, 116, 101, 114, 97, 108, 1, 2, 0, 3, 114, 101, 115, 1, 4, 0, 4, 0, 10, 101, 105, 116, 104, 101, 114, 45, 117, 51, 50, 3, 0, 5, 1, 114, 2, 4, 110, 97, 109, 101, 115, 6, 108, 101, 110, 103, 116, 104, 3, 4, 0, 18, 114, 97, 110, 100, 111, 109, 45, 115, 116, 114, 105, 110, 103, 45, 97, 114, 103, 115, 3, 0, 7, 1, 105, 1, 1, 114, 2, 3, 117, 114, 108, 9, 6, 114, 101, 115, 117, 108, 116, 9, 4, 0, 20, 114, 97, 110, 100, 111, 109, 45, 115, 116, 114, 105, 110, 103, 45, 114, 101, 115, 117, 108, 116, 3, 0, 10, 1, 64, 1, 4, 97, 114, 103, 115, 8, 1, 0, 4, 0, 20, 99, 114, 101, 97, 116, 101, 45, 114, 97, 110, 100, 111, 109, 45, 115, 116, 114, 105, 110, 103, 1, 12, 1, 64, 0, 1, 0, 4, 0, 16, 104, 97, 110, 100, 108, 101, 45, 102, 117, 110, 99, 116, 105, 111, 110, 115, 1, 13, 3, 1, 60, 99, 111, 109, 112, 111, 110, 101, 110, 116, 58, 112, 117, 108, 117, 109, 105, 45, 119, 97, 115, 109, 47, 112, 117, 108, 117, 109, 105, 45, 112, 114, 111, 118, 105, 100, 101, 114, 45, 114, 97, 110, 100, 111, 109, 45, 105, 110, 116, 101, 114, 102, 97, 99, 101, 64, 48, 46, 49, 46, 48, 5, 2, 1, 66, 2, 1, 64, 0, 1, 0, 4, 0, 4, 109, 97, 105, 110, 1, 0, 4, 1, 39, 99, 111, 109, 112, 111, 110, 101, 110, 116, 58, 112, 117, 108, 117, 109, 105, 45, 119, 97, 115, 109, 47, 112, 117, 108, 117, 109, 105, 45, 109, 97, 105, 110, 64, 48, 46, 49, 46, 48, 5, 3, 4, 1, 36, 99, 111, 109, 112, 111, 110, 101, 110, 116, 58, 112, 117, 108, 117, 109, 105, 45, 119, 97, 115, 109, 47, 110, 101, 119, 45, 109, 97, 105, 110, 64, 48, 46, 49, 46, 48, 4, 0, 11, 14, 1, 0, 8, 110, 101, 119, 45, 109, 97, 105, 110, 3, 0, 0, 0, 70, 9, 112, 114, 111, 100, 117, 99, 101, 114, 115, 1, 12, 112, 114, 111, 99, 101, 115, 115, 101, 100, 45, 98, 121, 2, 13, 119, 105, 116, 45, 99, 111, 109, 112, 111, 110, 101, 110, 116, 6, 48, 46, 50, 49, 46, 48, 16, 119, 105, 116, 45, 98, 105, 110, 100, 103, 101, 110, 45, 114, 117, 115, 116, 6, 48, 46, 49, 56, 46, 48];

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_section() {}
