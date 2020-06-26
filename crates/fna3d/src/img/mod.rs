//! FNA3D_Image.h

// use fna3d_sys as sys;

// type ImageSkipFunc = sys::FNA3D_Image_SkipFunc;
// type ImageReadFunc = sys::FNA3D_Image_ReadFunc;
// type ImageEofFunc = sys::FNA3D_Image_EOFFunc;

// extern "C" {
//     pub fn FNA3D_Image_Load(
//         readFunc: FNA3D_Image_ReadFunc,
//         skipFunc: FNA3D_Image_SkipFunc,
//         eofFunc: FNA3D_Image_EOFFunc,
//         context: *mut ::std::os::raw::c_void,
//         w: *mut i32,
//         h: *mut i32,
//         len: *mut i32,
//         forceW: i32,
//         forceH: i32,
//         zoom: u8,
//     ) -> *mut u8;
// }

// extern "C" {
//     pub fn FNA3D_Image_Free(mem: *mut u8);
// }

// pub type FNA3D_Image_WriteFunc = ::std::option::Option<
//     unsafe extern "C" fn(
//         context: *mut ::std::os::raw::c_void,
//         data: *mut ::std::os::raw::c_void,
//         size: i32,
//     ),
// >;

// extern "C" {
//     pub fn FNA3D_Image_SavePNG(
//         writeFunc: FNA3D_Image_WriteFunc,
//         context: *mut ::std::os::raw::c_void,
//         srcW: i32,
//         srcH: i32,
//         dstW: i32,
//         dstH: i32,
//         data: *mut u8,
//     );
// }

// extern "C" {
//     pub fn FNA3D_Image_SaveJPG(
//         writeFunc: FNA3D_Image_WriteFunc,
//         context: *mut ::std::os::raw::c_void,
//         srcW: i32,
//         srcH: i32,
//         dstW: i32,
//         dstH: i32,
//         data: *mut u8,
//         quality: i32,
//     );
// }
