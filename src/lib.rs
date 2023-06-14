#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum MH_STATUS {
    MH_UNKNOWN = -1,

    MH_OK = 0,

    MH_ERROR_ALREADY_INITIALIZED,

    MH_ERROR_NOT_INITIALIZED,

    MH_ERROR_ALREADY_CREATED,

    MH_ERROR_NOT_CREATED,

    MH_ERROR_ENABLED,

    MH_ERROR_DISABLED,

    MH_ERROR_NOT_EXECUTABLE,

    MH_ERROR_UNSUPPORTED_FUNCTION,

    MH_ERROR_MEMORY_ALLOC,

    MH_ERROR_MEMORY_PROTECT,

    MH_ERROR_MODULE_NOT_FOUND,

    MH_ERROR_FUNCTION_NOT_FOUND,
}

#[allow(dead_code)]
extern "system" {

    pub fn MH_Initialize() -> MH_STATUS;

    pub fn MH_Uninitialize() -> MH_STATUS;

    pub fn MH_CreateHook(
        pTarget: *mut core::ffi::c_void,
        pDetour: *mut core::ffi::c_void,
        ppOriginal: *mut *mut core::ffi::c_void,
    ) -> MH_STATUS;

    pub fn MH_CreateHookApi(
        pszModule: *const u16,
        pszProcName: *const i8,
        pDetour: *mut core::ffi::c_void,
        ppOriginal: *mut *mut core::ffi::c_void,
    ) -> MH_STATUS;

    pub fn MH_CreateHookApiEx(
        pszModule: *const u16,
        pszProcName: *const i8,
        pDetour: *mut core::ffi::c_void,
        ppOriginal: *mut *mut core::ffi::c_void,
        ppTarget: *mut *mut core::ffi::c_void,
    ) -> MH_STATUS;

    pub fn MH_RemoveHook(pTarget: *mut core::ffi::c_void) -> MH_STATUS;

    pub fn MH_EnableHook(pTarget: *mut core::ffi::c_void) -> MH_STATUS;

    pub fn MH_DisableHook(pTarget: *mut core::ffi::c_void) -> MH_STATUS;

    pub fn MH_QueueEnableHook(pTarget: *mut core::ffi::c_void) -> MH_STATUS;

    pub fn MH_QueueDisableHook(pTarget: *mut core::ffi::c_void) -> MH_STATUS;

    pub fn MH_ApplyQueued() -> MH_STATUS;

    pub fn MH_StatusToString(status: MH_STATUS) -> *const i8;

}