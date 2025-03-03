#![allow(non_snake_case)]

use crate::api::context::*;
use crate::api::device::*;
use crate::api::event::*;
use crate::api::kernel::*;
use crate::api::memory::*;
use crate::api::platform::*;
use crate::api::program::*;
use crate::api::queue::*;
use crate::api::types::*;
use crate::api::util::*;

use mesa_rust_util::ptr::*;
use rusticl_opencl_gen::*;

use std::ffi::CStr;
use std::ptr;
use std::sync::Arc;

pub static DISPATCH: cl_icd_dispatch = cl_icd_dispatch {
    clGetPlatformIDs: Some(cl_get_platform_ids),
    clGetPlatformInfo: Some(cl_get_platform_info),
    clGetDeviceIDs: Some(cl_get_device_ids),
    clGetDeviceInfo: Some(cl_get_device_info),
    clCreateContext: Some(cl_create_context),
    clCreateContextFromType: Some(cl_create_context_from_type),
    clRetainContext: Some(cl_retain_context),
    clReleaseContext: Some(cl_release_context),
    clGetContextInfo: Some(cl_get_context_info),
    clCreateCommandQueue: Some(cl_create_command_queue),
    clRetainCommandQueue: Some(cl_retain_command_queue),
    clReleaseCommandQueue: Some(cl_release_command_queue),
    clGetCommandQueueInfo: Some(cl_get_command_queue_info),
    clSetCommandQueueProperty: None,
    clCreateBuffer: Some(cl_create_buffer),
    clCreateImage2D: Some(cl_create_image_2d),
    clCreateImage3D: Some(cl_create_image_3d),
    clRetainMemObject: Some(cl_retain_mem_object),
    clReleaseMemObject: Some(cl_release_mem_object),
    clGetSupportedImageFormats: Some(cl_get_supported_image_formats),
    clGetMemObjectInfo: Some(cl_get_mem_object_info),
    clGetImageInfo: Some(cl_get_image_info),
    clCreateSampler: Some(cl_create_sampler),
    clRetainSampler: Some(cl_retain_sampler),
    clReleaseSampler: Some(cl_release_sampler),
    clGetSamplerInfo: Some(cl_get_sampler_info),
    clCreateProgramWithSource: Some(cl_create_program_with_source),
    clCreateProgramWithBinary: Some(cl_create_program_with_binary),
    clRetainProgram: Some(cl_retain_program),
    clReleaseProgram: Some(cl_release_program),
    clBuildProgram: Some(cl_build_program),
    clUnloadCompiler: None,
    clGetProgramInfo: Some(cl_get_program_info),
    clGetProgramBuildInfo: Some(cl_get_program_build_info),
    clCreateKernel: Some(cl_create_kernel),
    clCreateKernelsInProgram: Some(cl_create_kernels_in_program),
    clRetainKernel: Some(cl_retain_kernel),
    clReleaseKernel: Some(cl_release_kernel),
    clSetKernelArg: Some(cl_set_kernel_arg),
    clGetKernelInfo: Some(cl_get_kernel_info),
    clGetKernelWorkGroupInfo: Some(cl_get_kernel_work_group_info),
    clWaitForEvents: Some(cl_wait_for_events),
    clGetEventInfo: Some(cl_get_event_info),
    clRetainEvent: Some(cl_retain_event),
    clReleaseEvent: Some(cl_release_event),
    clGetEventProfilingInfo: Some(cl_get_event_profiling_info),
    clFlush: Some(cl_flush),
    clFinish: Some(cl_finish),
    clEnqueueReadBuffer: Some(cl_enqueue_read_buffer),
    clEnqueueWriteBuffer: Some(cl_enqueue_write_buffer),
    clEnqueueCopyBuffer: Some(cl_enqueue_copy_buffer),
    clEnqueueReadImage: Some(cl_enqueue_read_image),
    clEnqueueWriteImage: Some(cl_enqueue_write_image),
    clEnqueueCopyImage: Some(cl_enqueue_copy_image),
    clEnqueueCopyImageToBuffer: Some(cl_enqueue_copy_image_to_buffer),
    clEnqueueCopyBufferToImage: Some(cl_enqueue_copy_buffer_to_image),
    clEnqueueMapBuffer: Some(cl_enqueue_map_buffer),
    clEnqueueMapImage: Some(cl_enqueue_map_image),
    clEnqueueUnmapMemObject: Some(cl_enqueue_unmap_mem_object),
    clEnqueueNDRangeKernel: Some(cl_enqueue_ndrange_kernel),
    clEnqueueTask: Some(cl_enqueue_task),
    clEnqueueNativeKernel: None,
    clEnqueueMarker: Some(cl_enqueue_marker),
    clEnqueueWaitForEvents: None,
    clEnqueueBarrier: Some(cl_enqueue_barrier),
    clGetExtensionFunctionAddress: Some(cl_get_extension_function_address),
    clCreateFromGLBuffer: Some(cl_create_from_gl_buffer),
    clCreateFromGLTexture2D: Some(cl_create_from_gl_texture_2d),
    clCreateFromGLTexture3D: Some(cl_create_from_gl_texture_3d),
    clCreateFromGLRenderbuffer: Some(cl_create_from_gl_renderbuffer),
    clGetGLObjectInfo: Some(cl_get_gl_object_info),
    clGetGLTextureInfo: Some(cl_get_gl_texture_info),
    clEnqueueAcquireGLObjects: Some(cl_enqueue_acquire_gl_objects),
    clEnqueueReleaseGLObjects: Some(cl_enqueue_release_gl_objects),
    clGetGLContextInfoKHR: Some(cl_get_gl_context_info_khr),
    clGetDeviceIDsFromD3D10KHR: ptr::null_mut(),
    clCreateFromD3D10BufferKHR: ptr::null_mut(),
    clCreateFromD3D10Texture2DKHR: ptr::null_mut(),
    clCreateFromD3D10Texture3DKHR: ptr::null_mut(),
    clEnqueueAcquireD3D10ObjectsKHR: ptr::null_mut(),
    clEnqueueReleaseD3D10ObjectsKHR: ptr::null_mut(),
    clSetEventCallback: Some(cl_set_event_callback),
    clCreateSubBuffer: Some(cl_create_sub_buffer),
    clSetMemObjectDestructorCallback: Some(cl_set_mem_object_destructor_callback),
    clCreateUserEvent: Some(cl_create_user_event),
    clSetUserEventStatus: Some(cl_set_user_event_status),
    clEnqueueReadBufferRect: Some(cl_enqueue_read_buffer_rect),
    clEnqueueWriteBufferRect: Some(cl_enqueue_write_buffer_rect),
    clEnqueueCopyBufferRect: Some(cl_enqueue_copy_buffer_rect),
    clCreateSubDevicesEXT: None,
    clRetainDeviceEXT: None,
    clReleaseDeviceEXT: None,
    clCreateEventFromGLsyncKHR: None,
    clCreateSubDevices: None,
    clRetainDevice: Some(cl_retain_device),
    clReleaseDevice: Some(cl_release_device),
    clCreateImage: Some(cl_create_image),
    clCreateProgramWithBuiltInKernels: None,
    clCompileProgram: Some(cl_compile_program),
    clLinkProgram: Some(cl_link_program),
    clUnloadPlatformCompiler: Some(cl_unload_platform_compiler),
    clGetKernelArgInfo: Some(cl_get_kernel_arg_info),
    clEnqueueFillBuffer: Some(cl_enqueue_fill_buffer),
    clEnqueueFillImage: Some(cl_enqueue_fill_image),
    clEnqueueMigrateMemObjects: Some(cl_enqueue_migrate_mem_objects),
    clEnqueueMarkerWithWaitList: Some(cl_enqueue_marker_with_wait_list),
    clEnqueueBarrierWithWaitList: Some(cl_enqueue_barrier_with_wait_list),
    clGetExtensionFunctionAddressForPlatform: Some(cl_get_extension_function_address_for_platform),
    clCreateFromGLTexture: Some(cl_create_from_gl_texture),
    clGetDeviceIDsFromD3D11KHR: ptr::null_mut(),
    clCreateFromD3D11BufferKHR: ptr::null_mut(),
    clCreateFromD3D11Texture2DKHR: ptr::null_mut(),
    clCreateFromD3D11Texture3DKHR: ptr::null_mut(),
    clCreateFromDX9MediaSurfaceKHR: ptr::null_mut(),
    clEnqueueAcquireD3D11ObjectsKHR: ptr::null_mut(),
    clEnqueueReleaseD3D11ObjectsKHR: ptr::null_mut(),
    clGetDeviceIDsFromDX9MediaAdapterKHR: ptr::null_mut(),
    clEnqueueAcquireDX9MediaSurfacesKHR: ptr::null_mut(),
    clEnqueueReleaseDX9MediaSurfacesKHR: ptr::null_mut(),
    clCreateFromEGLImageKHR: None,
    clEnqueueAcquireEGLObjectsKHR: None,
    clEnqueueReleaseEGLObjectsKHR: None,
    clCreateEventFromEGLSyncKHR: None,
    clCreateCommandQueueWithProperties: Some(cl_create_command_queue_with_properties),
    clCreatePipe: Some(cl_create_pipe),
    clGetPipeInfo: Some(cl_get_pipe_info),
    clSVMAlloc: Some(cl_svm_alloc),
    clSVMFree: Some(cl_svm_free),
    clEnqueueSVMFree: Some(cl_enqueue_svm_free),
    clEnqueueSVMMemcpy: Some(cl_enqueue_svm_memcpy),
    clEnqueueSVMMemFill: Some(cl_enqueue_svm_mem_fill),
    clEnqueueSVMMap: Some(cl_enqueue_svm_map),
    clEnqueueSVMUnmap: Some(cl_enqueue_svm_unmap),
    clCreateSamplerWithProperties: Some(cl_create_sampler_with_properties),
    clSetKernelArgSVMPointer: Some(cl_set_kernel_arg_svm_pointer),
    clSetKernelExecInfo: Some(cl_set_kernel_exec_info),
    clGetKernelSubGroupInfoKHR: Some(cl_get_kernel_sub_group_info),
    clCloneKernel: Some(cl_clone_kernel),
    clCreateProgramWithIL: Some(cl_create_program_with_il),
    clEnqueueSVMMigrateMem: Some(cl_enqueue_svm_migrate_mem),
    clGetDeviceAndHostTimer: Some(cl_get_device_and_host_timer),
    clGetHostTimer: Some(cl_get_host_timer),
    clGetKernelSubGroupInfo: Some(cl_get_kernel_sub_group_info),
    clSetDefaultDeviceCommandQueue: Some(cl_set_default_device_command_queue),
    clSetProgramReleaseCallback: Some(cl_set_program_release_callback),
    clSetProgramSpecializationConstant: Some(cl_set_program_specialization_constant),
    clCreateBufferWithProperties: Some(cl_create_buffer_with_properties),
    clCreateImageWithProperties: Some(cl_create_image_with_properties),
    clSetContextDestructorCallback: Some(cl_set_context_destructor_callback),
};

pub type CLError = cl_int;
pub type CLResult<T> = Result<T, CLError>;

#[derive(Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum RusticlTypes {
    // random number
    Buffer = 0xec4cf9a9,
    Context,
    Device,
    Event,
    Image,
    Kernel,
    Program,
    Queue,
    Sampler,
}

impl RusticlTypes {
    pub const fn u32(&self) -> u32 {
        *self as u32
    }

    pub const fn from_u32(val: u32) -> Option<Self> {
        let result = match val {
            0xec4cf9a9 => Self::Buffer,
            0xec4cf9aa => Self::Context,
            0xec4cf9ab => Self::Device,
            0xec4cf9ac => Self::Event,
            0xec4cf9ad => Self::Image,
            0xec4cf9ae => Self::Kernel,
            0xec4cf9af => Self::Program,
            0xec4cf9b0 => Self::Queue,
            0xec4cf9b1 => Self::Sampler,
            _ => return None,
        };
        debug_assert!(result.u32() == val);
        Some(result)
    }
}

#[repr(C)]
pub struct CLObjectBase<const ERR: i32> {
    dispatch: &'static cl_icd_dispatch,
    rusticl_type: u32,
}

impl<const ERR: i32> CLObjectBase<ERR> {
    pub fn new(t: RusticlTypes) -> Self {
        Self {
            dispatch: &DISPATCH,
            rusticl_type: t.u32(),
        }
    }

    pub fn check_ptr(ptr: *const Self) -> CLResult<RusticlTypes> {
        if ptr.is_null() {
            return Err(ERR);
        }

        unsafe {
            if !::std::ptr::eq((*ptr).dispatch, &DISPATCH) {
                return Err(ERR);
            }

            let Some(ty) = RusticlTypes::from_u32((*ptr).rusticl_type) else {
                return Err(ERR);
            };

            Ok(ty)
        }
    }

    pub fn get_type(&self) -> CLResult<RusticlTypes> {
        RusticlTypes::from_u32(self.rusticl_type).ok_or(ERR)
    }
}

pub trait ReferenceCountedAPIPointer<T, const ERR: i32> {
    fn get_ptr(&self) -> CLResult<*const T>;

    // TODO:  I can't find a trait that would let me say T: pointer so that
    // I can do the cast in the main trait implementation.  So we need to
    // implement that as part of the macro where we know the real type.
    fn from_ptr(ptr: *const T) -> Self;
}

pub trait BaseCLObject<'a, const ERR: i32, CL: ReferenceCountedAPIPointer<Self, ERR> + 'a>:
    Sized
{
    fn ref_from_raw(obj: CL) -> CLResult<&'a Self> {
        let obj = obj.get_ptr()?;
        // SAFETY: `get_ptr` already checks if it's one of our pointers and not null
        Ok(unsafe { &*obj })
    }

    fn refs_from_arr(objs: *const CL, count: u32) -> CLResult<Vec<&'a Self>>
    where
        CL: Copy,
    {
        // CL spec requires validation for obj arrays, both values have to make sense
        if objs.is_null() && count > 0 || !objs.is_null() && count == 0 {
            return Err(CL_INVALID_VALUE);
        }

        let mut res = Vec::new();
        if objs.is_null() || count == 0 {
            return Ok(res);
        }

        for i in 0..count as usize {
            res.push(Self::ref_from_raw(unsafe { *objs.add(i) })?);
        }
        Ok(res)
    }
}

pub trait CLObject<'a, const ERR: i32, CL: ReferenceCountedAPIPointer<Self, ERR> + 'a>:
    Sized + BaseCLObject<'a, ERR, CL>
{
    fn as_cl(&self) -> CL {
        CL::from_ptr(self)
    }
}

pub trait ArcedCLObject<'a, const ERR: i32, CL: ReferenceCountedAPIPointer<Self, ERR> + 'a>:
    Sized + BaseCLObject<'a, ERR, CL>
{
    /// Note: this operation increases the internal ref count as `ref_from_raw` is the better option
    /// when an Arc is not needed.
    fn arc_from_raw(ptr: CL) -> CLResult<Arc<Self>> {
        let ptr = ptr.get_ptr()?;
        // SAFETY: `get_ptr` already checks if it's one of our pointers.
        Ok(unsafe {
            Arc::increment_strong_count(ptr);
            Arc::from_raw(ptr)
        })
    }

    fn arcs_from_arr(objs: *const CL, count: u32) -> CLResult<Vec<Arc<Self>>>
    where
        CL: Copy,
    {
        // CL spec requires validation for obj arrays, both values have to make sense
        if objs.is_null() && count > 0 || !objs.is_null() && count == 0 {
            return Err(CL_INVALID_VALUE);
        }

        let mut res = Vec::new();
        if objs.is_null() || count == 0 {
            return Ok(res);
        }

        for i in 0..count as usize {
            unsafe {
                res.push(Self::arc_from_raw(*objs.add(i))?);
            }
        }
        Ok(res)
    }

    fn refcnt(ptr: CL) -> CLResult<u32> {
        let ptr = ptr.get_ptr()?;
        // SAFETY: `get_ptr` already checks if it's one of our pointers.
        let arc = unsafe { Arc::from_raw(ptr) };
        let res = Arc::strong_count(&arc);
        // leak the arc again, so we don't reduce the refcount by dropping `arc`
        let _ = Arc::into_raw(arc);
        Ok(res as u32)
    }

    fn into_cl(self: Arc<Self>) -> CL {
        CL::from_ptr(Arc::into_raw(self))
    }

    fn release(ptr: CL) -> CLResult<()> {
        let ptr = ptr.get_ptr()?;
        // SAFETY: `get_ptr` already checks if it's one of our pointers.
        unsafe { Arc::decrement_strong_count(ptr) };
        Ok(())
    }

    fn retain(ptr: CL) -> CLResult<()> {
        let ptr = ptr.get_ptr()?;
        // SAFETY: `get_ptr` already checks if it's one of our pointers.
        unsafe { Arc::increment_strong_count(ptr) };
        Ok(())
    }
}

#[macro_export]
macro_rules! impl_cl_type_trait_base {
    (@BASE $cl: ident, $t: ident, [$($types: ident),+], $err: ident, $($field:ident).+) => {
        impl $crate::api::icd::ReferenceCountedAPIPointer<$t, $err> for $cl {
            fn get_ptr(&self) -> CLResult<*const $t> {
                type Base = $crate::api::icd::CLObjectBase<$err>;
                let t = Base::check_ptr(self.cast())?;
                if ![$($crate::api::icd::RusticlTypes::$types),+].contains(&t) {
                    return Err($err);
                }

                let offset = ::mesa_rust_util::offset_of!($t, $($field).+);
                let mut obj_ptr: *const u8 = self.cast();
                // SAFETY: We offset the pointer back from the ICD specified base type to our
                //         internal type.
                unsafe { obj_ptr = obj_ptr.sub(offset) }

                let obj_ptr: *const $t = obj_ptr.cast();

                // Check at compile-time that we indeed got the right path
                unsafe { let _: &Base = &(*obj_ptr).$($field).+; }

                Ok(obj_ptr)
            }

            fn from_ptr(ptr: *const $t) -> Self {
                if ptr.is_null() {
                    return std::ptr::null_mut();
                }
                let offset = ::mesa_rust_util::offset_of!($t, $($field).+);
                // SAFETY: The resulting pointer is safe as we simply offset into the ICD specified
                //         base type.
                unsafe { (ptr as *const u8).add(offset) as Self }
            }
        }

        impl $crate::api::icd::BaseCLObject<'_, $err, $cl> for $t {}

        impl $t {
            fn _ensure_send_sync(&self) -> impl Send + Sync + '_ {
                self
            }
        }

        // there are two reason to implement those traits for all objects
        //   1. it speeds up operations
        //   2. we want to check for real equality more explicit to stay conformant with the API
        //      and to not break in subtle ways e.g. using CL objects as keys in HashMaps.
        impl std::cmp::Eq for $t {}
        impl std::cmp::PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                (self as *const Self) == (other as *const Self)
            }
        }

        impl std::hash::Hash for $t {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                (self as *const Self).hash(state);
            }
        }
    };

    ($cl: ident, $t: ident, [$($types: ident),+], $err: ident, $($field:ident).+) => {
        $crate::impl_cl_type_trait_base!(@BASE $cl, $t, [$($types),+], $err, $($field).+);
        impl $crate::api::icd::CLObject<'_, $err, $cl> for $t {}
    };

    ($cl: ident, $t: ident, [$($types: ident),+], $err: ident) => {
        $crate::impl_cl_type_trait_base!($cl, $t, [$($types),+], $err, base);
    };
}

#[macro_export]
macro_rules! impl_cl_type_trait {
    ($cl: ident, $t: ident, $err: ident, $($field:ident).+) => {
        $crate::impl_cl_type_trait_base!(@BASE $cl, $t, [$t], $err, $($field).+);
        impl $crate::api::icd::ArcedCLObject<'_, $err, $cl> for $t {}
    };

    ($cl: ident, $t: ident, $err: ident) => {
        $crate::impl_cl_type_trait!($cl, $t, $err, base);
    };
}

// We need those functions exported

#[no_mangle]
extern "C" fn clGetPlatformInfo(
    platform: cl_platform_id,
    param_name: cl_platform_info,
    param_value_size: usize,
    param_value: *mut ::std::ffi::c_void,
    param_value_size_ret: *mut usize,
) -> cl_int {
    cl_get_platform_info(
        platform,
        param_name,
        param_value_size,
        param_value,
        param_value_size_ret,
    )
}

#[no_mangle]
extern "C" fn clGetExtensionFunctionAddress(
    function_name: *const ::std::os::raw::c_char,
) -> *mut ::std::ffi::c_void {
    cl_get_extension_function_address(function_name)
}

#[no_mangle]
extern "C" fn clIcdGetPlatformIDsKHR(
    num_entries: cl_uint,
    platforms: *mut cl_platform_id,
    num_platforms: *mut cl_uint,
) -> cl_int {
    cl_get_platform_ids(num_entries, platforms, num_platforms)
}

extern "C" fn cl_get_extension_function_address(
    function_name: *const ::std::os::raw::c_char,
) -> *mut ::std::ffi::c_void {
    if function_name.is_null() {
        return ptr::null_mut();
    }
    match unsafe { CStr::from_ptr(function_name) }.to_str().unwrap() {
        // cl_khr_create_command_queue
        "clCreateCommandQueueWithPropertiesKHR" => {
            cl_create_command_queue_with_properties as *mut ::std::ffi::c_void
        }

        // cl_khr_icd
        "clGetPlatformInfo" => cl_get_platform_info as *mut ::std::ffi::c_void,
        "clIcdGetPlatformIDsKHR" => cl_get_platform_ids as *mut ::std::ffi::c_void,

        // cl_khr_il_program
        "clCreateProgramWithILKHR" => cl_create_program_with_il as *mut ::std::ffi::c_void,

        // cl_khr_gl_sharing
        "clCreateFromGLBuffer" => cl_create_from_gl_buffer as *mut ::std::ffi::c_void,
        "clCreateFromGLRenderbuffer" => cl_create_from_gl_renderbuffer as *mut ::std::ffi::c_void,
        "clCreateFromGLTexture" => cl_create_from_gl_texture as *mut ::std::ffi::c_void,
        "clCreateFromGLTexture2D" => cl_create_from_gl_texture_2d as *mut ::std::ffi::c_void,
        "clCreateFromGLTexture3D" => cl_create_from_gl_texture_3d as *mut ::std::ffi::c_void,
        "clEnqueueAcquireGLObjects" => cl_enqueue_acquire_gl_objects as *mut ::std::ffi::c_void,
        "clEnqueueReleaseGLObjects" => cl_enqueue_release_gl_objects as *mut ::std::ffi::c_void,
        "clGetGLContextInfoKHR" => cl_get_gl_context_info_khr as *mut ::std::ffi::c_void,
        "clGetGLObjectInfo" => cl_get_gl_object_info as *mut ::std::ffi::c_void,
        "clGetGLTextureInfo" => cl_get_gl_texture_info as *mut ::std::ffi::c_void,

        // cl_arm_shared_virtual_memory
        "clEnqueueSVMFreeARM" => cl_enqueue_svm_free_arm as *mut ::std::ffi::c_void,
        "clEnqueueSVMMapARM" => cl_enqueue_svm_map_arm as *mut ::std::ffi::c_void,
        "clEnqueueSVMMemcpyARM" => cl_enqueue_svm_memcpy_arm as *mut ::std::ffi::c_void,
        "clEnqueueSVMMemFillARM" => cl_enqueue_svm_mem_fill_arm as *mut ::std::ffi::c_void,
        "clEnqueueSVMUnmapARM" => cl_enqueue_svm_unmap_arm as *mut ::std::ffi::c_void,
        "clSetKernelArgSVMPointerARM" => cl_set_kernel_arg_svm_pointer as *mut ::std::ffi::c_void,
        "clSetKernelExecInfoARM" => cl_set_kernel_exec_info as *mut ::std::ffi::c_void,
        "clSVMAllocARM" => cl_svm_alloc as *mut ::std::ffi::c_void,
        "clSVMFreeARM" => cl_svm_free as *mut ::std::ffi::c_void,

        // DPCPP bug https://github.com/intel/llvm/issues/9964
        "clSetProgramSpecializationConstant" => {
            cl_set_program_specialization_constant as *mut ::std::ffi::c_void
        }

        _ => ptr::null_mut(),
    }
}

extern "C" fn cl_link_program(
    context: cl_context,
    num_devices: cl_uint,
    device_list: *const cl_device_id,
    options: *const ::std::os::raw::c_char,
    num_input_programs: cl_uint,
    input_programs: *const cl_program,
    pfn_notify: Option<FuncProgramCB>,
    user_data: *mut ::std::os::raw::c_void,
    errcode_ret: *mut cl_int,
) -> cl_program {
    let (ptr, err) = match link_program(
        context,
        num_devices,
        device_list,
        options,
        num_input_programs,
        input_programs,
        pfn_notify,
        user_data,
    ) {
        Ok((prog, code)) => (prog, code),
        Err(e) => (ptr::null_mut(), e),
    };

    errcode_ret.write_checked(err);
    ptr
}

extern "C" fn cl_get_extension_function_address_for_platform(
    _platform: cl_platform_id,
    function_name: *const ::std::os::raw::c_char,
) -> *mut ::std::os::raw::c_void {
    cl_get_extension_function_address(function_name)
}

extern "C" fn cl_svm_alloc(
    context: cl_context,
    flags: cl_svm_mem_flags,
    size: usize,
    alignment: ::std::os::raw::c_uint,
) -> *mut ::std::os::raw::c_void {
    svm_alloc(context, flags, size, alignment).unwrap_or(ptr::null_mut())
}

extern "C" fn cl_svm_free(context: cl_context, svm_pointer: *mut ::std::os::raw::c_void) {
    svm_free(context, svm_pointer as usize).ok();
}

extern "C" fn cl_get_kernel_sub_group_info(
    kernel: cl_kernel,
    device: cl_device_id,
    param_name: cl_kernel_sub_group_info,
    input_value_size: usize,
    input_value: *const ::std::os::raw::c_void,
    param_value_size: usize,
    param_value: *mut ::std::os::raw::c_void,
    param_value_size_ret: *mut usize,
) -> cl_int {
    match kernel.get_info_obj(
        (device, input_value_size, input_value, param_value_size),
        param_name,
        param_value_size,
        param_value,
        param_value_size_ret,
    ) {
        Ok(_) => CL_SUCCESS as cl_int,
        Err(e) => e,
    }
}
