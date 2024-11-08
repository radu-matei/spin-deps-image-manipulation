#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod image_manipulation_lib {
            #[allow(dead_code, clippy::all)]
            pub mod image_manipulation {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                /// Error returned by image manipulation components.
                #[derive(Clone)]
                pub enum ImageError {
                    ImageError(_rt::String),
                    IoError(_rt::String),
                    Unknown(_rt::String),
                }
                impl ::core::fmt::Debug for ImageError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            ImageError::ImageError(e) => {
                                f.debug_tuple("ImageError::ImageError").field(e).finish()
                            }
                            ImageError::IoError(e) => {
                                f.debug_tuple("ImageError::IoError").field(e).finish()
                            }
                            ImageError::Unknown(e) => {
                                f.debug_tuple("ImageError::Unknown").field(e).finish()
                            }
                        }
                    }
                }
                impl ::core::fmt::Display for ImageError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        write!(f, "{:?}", self)
                    }
                }
                impl std::error::Error for ImageError {}
                /// Type representing an image.
                pub type Image = _rt::Vec<u8>;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_grayscale_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::grayscale(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                        arg2 as u8,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec3 = (e).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            match e {
                                ImageError::ImageError(e) => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                    let vec4 = (e.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *ptr2.add(12).cast::<usize>() = len4;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                                }
                                ImageError::IoError(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    let vec5 = (e.into_bytes()).into_boxed_slice();
                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                    let len5 = vec5.len();
                                    ::core::mem::forget(vec5);
                                    *ptr2.add(12).cast::<usize>() = len5;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                }
                                ImageError::Unknown(e) => {
                                    *ptr2.add(4).cast::<u8>() = (2i32) as u8;
                                    let vec6 = (e.into_bytes()).into_boxed_slice();
                                    let ptr6 = vec6.as_ptr().cast::<u8>();
                                    let len6 = vec6.len();
                                    ::core::mem::forget(vec6);
                                    *ptr2.add(12).cast::<usize>() = len6;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                                }
                            }
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_grayscale<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = i32::from(*arg0.add(4).cast::<u8>());
                            match l4 {
                                0 => {
                                    let l5 = *arg0.add(8).cast::<*mut u8>();
                                    let l6 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l5, l6, 1);
                                }
                                1 => {
                                    let l7 = *arg0.add(8).cast::<*mut u8>();
                                    let l8 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l7, l8, 1);
                                }
                                _ => {
                                    let l9 = *arg0.add(8).cast::<*mut u8>();
                                    let l10 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l9, l10, 1);
                                }
                            }
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_sepia_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let result1 = T::sepia(
                        _rt::Vec::from_raw_parts(arg0.cast(), len0, len0),
                        arg2 as u8,
                    );
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec3 = (e).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                        Err(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            match e {
                                ImageError::ImageError(e) => {
                                    *ptr2.add(4).cast::<u8>() = (0i32) as u8;
                                    let vec4 = (e.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *ptr2.add(12).cast::<usize>() = len4;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                                }
                                ImageError::IoError(e) => {
                                    *ptr2.add(4).cast::<u8>() = (1i32) as u8;
                                    let vec5 = (e.into_bytes()).into_boxed_slice();
                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                    let len5 = vec5.len();
                                    ::core::mem::forget(vec5);
                                    *ptr2.add(12).cast::<usize>() = len5;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                }
                                ImageError::Unknown(e) => {
                                    *ptr2.add(4).cast::<u8>() = (2i32) as u8;
                                    let vec6 = (e.into_bytes()).into_boxed_slice();
                                    let ptr6 = vec6.as_ptr().cast::<u8>();
                                    let len6 = vec6.len();
                                    ::core::mem::forget(vec6);
                                    *ptr2.add(12).cast::<usize>() = len6;
                                    *ptr2.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                                }
                            }
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_sepia<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base3 = l1;
                            let len3 = l2;
                            _rt::cabi_dealloc(base3, len3 * 1, 1);
                        }
                        _ => {
                            let l4 = i32::from(*arg0.add(4).cast::<u8>());
                            match l4 {
                                0 => {
                                    let l5 = *arg0.add(8).cast::<*mut u8>();
                                    let l6 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l5, l6, 1);
                                }
                                1 => {
                                    let l7 = *arg0.add(8).cast::<*mut u8>();
                                    let l8 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l7, l8, 1);
                                }
                                _ => {
                                    let l9 = *arg0.add(8).cast::<*mut u8>();
                                    let l10 = *arg0.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l9, l10, 1);
                                }
                            }
                        }
                    }
                }
                pub trait Guest {
                    /// Apply the grayscale transformation to the input image.
                    fn grayscale(img: Image, quality: u8) -> Result<Image, ImageError>;
                    /// Apply the sepia transformation to the input image.
                    fn sepia(img: Image, quality: u8) -> Result<Image, ImageError>;
                }
                #[doc(hidden)]
                macro_rules! __export_component_image_manipulation_lib_image_manipulation_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:image-manipulation-lib/image-manipulation#grayscale"]
                        unsafe extern "C" fn export_grayscale(arg0 : * mut u8, arg1 :
                        usize, arg2 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_grayscale_cabi::<$ty > (arg0, arg1, arg2) } #[export_name
                        =
                        "cabi_post_component:image-manipulation-lib/image-manipulation#grayscale"]
                        unsafe extern "C" fn _post_return_grayscale(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_grayscale::<$ty > (arg0) }
                        #[export_name =
                        "component:image-manipulation-lib/image-manipulation#sepia"]
                        unsafe extern "C" fn export_sepia(arg0 : * mut u8, arg1 : usize,
                        arg2 : i32,) -> * mut u8 { $($path_to_types)*::
                        _export_sepia_cabi::<$ty > (arg0, arg1, arg2) } #[export_name =
                        "cabi_post_component:image-manipulation-lib/image-manipulation#sepia"]
                        unsafe extern "C" fn _post_return_sepia(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_sepia::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_image_manipulation_lib_image_manipulation_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
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
macro_rules! __export_image_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::image_manipulation_lib::image_manipulation::__export_component_image_manipulation_lib_image_manipulation_cabi!($ty
        with_types_in $($path_to_types_root)*::
        exports::component::image_manipulation_lib::image_manipulation);
    };
}
#[doc(inline)]
pub(crate) use __export_image_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:image-manipulation-lib:image:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 353] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xe5\x01\x01A\x02\x01\
A\x02\x01B\x08\x01q\x03\x0bimage-error\x01s\0\x08io-error\x01s\0\x07unknown\x01s\
\0\x04\0\x0bimage-error\x03\0\0\x01p}\x04\0\x05image\x03\0\x02\x01j\x01\x03\x01\x01\
\x01@\x02\x03img\x03\x07quality}\0\x04\x04\0\x09grayscale\x01\x05\x04\0\x05sepia\
\x01\x05\x04\x013component:image-manipulation-lib/image-manipulation\x05\0\x04\x01\
&component:image-manipulation-lib/image\x04\0\x0b\x0b\x01\0\x05image\x03\0\0\0G\x09\
producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rus\
t\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
