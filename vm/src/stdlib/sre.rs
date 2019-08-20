#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(
    const_raw_ptr_to_usize_cast,
    const_transmute,
    label_break_value,
    ptr_wrapping_offset_from
)]

use crate::pyobject::PyObjectRef;
use crate::VirtualMachine;

mod constants;
use self::constants::*;

pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    py_module!(vm, "_sre", {
        "MAXREPEAT" => vm.new_int(std::usize::MAX),
        "MAXGROUPS" => vm.new_int(std::usize::MAX),
    })
}

extern crate c2rust_bitfields;
extern crate libc;
use c2rust_bitfields::BitfieldStruct;
extern "C" {
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    static _Py_ctype_tolower: [libc::c_uchar; 256];
    #[no_mangle]
    static _Py_ctype_table: [libc::c_uint; 256];
    #[no_mangle]
    fn PyMapping_Keys(o: *mut PyObject) -> *mut PyObject;
    #[no_mangle]
    fn PySequence_GetItem(o: *mut PyObject, i: Py_ssize_t) -> *mut PyObject;
    #[no_mangle]
    fn PyNumber_AsSsize_t(o: *mut PyObject, exc: *mut PyObject) -> Py_ssize_t;
    #[no_mangle]
    fn PyBuffer_Release(view: *mut Py_buffer);
    #[no_mangle]
    fn PyObject_GetBuffer(
        obj: *mut PyObject,
        view: *mut Py_buffer,
        flags: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn PyObject_GetItem(o: *mut PyObject, key: *mut PyObject) -> *mut PyObject;
    #[no_mangle]
    fn PyObject_CallObject(callable_object: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
    #[no_mangle]
    fn PyImport_Import(name: *mut PyObject) -> *mut PyObject;
    #[no_mangle]
    fn PyModule_Create2(_: *mut PyModuleDef, apiver: libc::c_int) -> *mut PyObject;
    #[no_mangle]
    fn _PyArg_ParseStack_SizeT(
        args: *mut *mut PyObject,
        nargs: Py_ssize_t,
        kwnames: *mut PyObject,
        _: *mut _PyArg_Parser,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn PyArg_UnpackTuple(
        _: *mut PyObject,
        _: *const libc::c_char,
        _: Py_ssize_t,
        _: Py_ssize_t,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn _PyArg_ParseTuple_SizeT(_: *mut PyObject, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn PyErr_CheckSignals() -> libc::c_int;
    #[no_mangle]
    fn PyErr_NoMemory() -> *mut PyObject;
    #[no_mangle]
    static mut PyExc_ValueError: *mut PyObject;
    #[no_mangle]
    static mut PyExc_TypeError: *mut PyObject;
    #[no_mangle]
    static mut PyExc_RecursionError: *mut PyObject;
    #[no_mangle]
    static mut PyExc_RuntimeError: *mut PyObject;
    #[no_mangle]
    static mut PyExc_OverflowError: *mut PyObject;
    #[no_mangle]
    static mut PyExc_IndexError: *mut PyObject;
    #[no_mangle]
    fn PyErr_Clear();
    #[no_mangle]
    fn PyErr_Occurred() -> *mut PyObject;
    #[no_mangle]
    fn PyErr_SetString(exception: *mut PyObject, string: *const libc::c_char);
    #[no_mangle]
    fn PyType_Ready(_: *mut PyTypeObject) -> libc::c_int;
    #[no_mangle]
    fn PyObject_GetAttrString(_: *mut PyObject, _: *const libc::c_char) -> *mut PyObject;
    #[no_mangle]
    static mut _Py_NoneStruct: PyObject;
    #[no_mangle]
    fn PyMem_Free(ptr: *mut libc::c_void);
    #[no_mangle]
    fn PyObject_Free(ptr: *mut libc::c_void);
    #[no_mangle]
    static mut PyBytes_Type: PyTypeObject;
    #[no_mangle]
    fn PyBytes_FromStringAndSize(_: *const libc::c_char, _: Py_ssize_t) -> *mut PyObject;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn tolower(_: libc::c_int) -> libc::c_int;
    /* Initializes the canonical string representation from the deprecated
    wstr/Py_UNICODE representation. This function is used to convert Unicode
    objects which were created using the old API to the new flexible format
    introduced with PEP 393.

    Don't call this function directly, use the public PyUnicode_READY() macro
    instead. */
    #[no_mangle]
    fn _PyUnicode_Ready(unicode: *mut PyObject) -> libc::c_int;
    /* Similar to PyUnicode_FromUnicode(), but u points to null-terminated
    UTF-8 encoded bytes.  The size is determined with strlen(). */
    #[no_mangle]
    fn PyUnicode_FromString(u: *const libc::c_char) -> *mut PyObject;
    #[no_mangle]
    fn PyUnicode_Substring(str: *mut PyObject, start: Py_ssize_t, end: Py_ssize_t)
        -> *mut PyObject;
    #[no_mangle]
    fn PyUnicode_FromFormat(format: *const libc::c_char, _: ...) -> *mut PyObject;
    /* === Characters Type APIs =============================================== */
    /* Helper array used by Py_UNICODE_ISSPACE(). */
    #[no_mangle]
    static _Py_ascii_whitespace: [libc::c_uchar; 0];
    #[no_mangle]
    fn _PyUnicode_IsWhitespace(ch: Py_UCS4) -> libc::c_int;
    #[no_mangle]
    fn _PyUnicode_IsLinebreak(ch: Py_UCS4) -> libc::c_int;
    #[no_mangle]
    fn _PyUnicode_ToLowercase(ch: Py_UCS4) -> Py_UCS4;
    #[no_mangle]
    fn _PyUnicode_IsDecimalDigit(ch: Py_UCS4) -> libc::c_int;
    #[no_mangle]
    fn _PyUnicode_IsDigit(ch: Py_UCS4) -> libc::c_int;
    #[no_mangle]
    fn _PyUnicode_IsNumeric(ch: Py_UCS4) -> libc::c_int;
    #[no_mangle]
    fn _PyUnicode_IsAlpha(ch: Py_UCS4) -> libc::c_int;
    #[no_mangle]
    fn PyLong_FromLong(_: libc::c_long) -> *mut PyObject;
    #[no_mangle]
    fn PyLong_FromUnsignedLong(_: libc::c_ulong) -> *mut PyObject;
    #[no_mangle]
    fn PyLong_FromSsize_t(_: Py_ssize_t) -> *mut PyObject;
    #[no_mangle]
    fn PyLong_AsSsize_t(_: *mut PyObject) -> Py_ssize_t;
    #[no_mangle]
    fn PyLong_AsUnsignedLong(_: *mut PyObject) -> libc::c_ulong;
    #[no_mangle]
    static mut _Py_FalseStruct: _longobject;
    #[no_mangle]
    fn PyTuple_New(size: Py_ssize_t) -> *mut PyObject;
    #[no_mangle]
    fn PyTuple_Pack(_: Py_ssize_t, _: ...) -> *mut PyObject;
    #[no_mangle]
    static mut PyList_Type: PyTypeObject;
    /* This excludes Values, since they are not sets. */
    #[no_mangle]
    fn PyDict_New() -> *mut PyObject;
    #[no_mangle]
    fn PyDict_SetItem(mp: *mut PyObject, key: *mut PyObject, item: *mut PyObject) -> libc::c_int;
    /* !Py_LIMITED_API */
    #[no_mangle]
    fn PyDict_SetItemString(
        dp: *mut PyObject,
        key: *const libc::c_char,
        item: *mut PyObject,
    ) -> libc::c_int;
    #[no_mangle]
    fn PyModule_GetDict(_: *mut PyObject) -> *mut PyObject;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: libc::c_int,
    pub _mode: libc::c_int,
    pub _unused2: libc::c_char,
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = libc::c_ulong;
/* Py_ssize_t is a signed integral type such that sizeof(Py_ssize_t) ==
 * sizeof(size_t).  C99 doesn't define such a thing directly (size_t is an
 * unsigned integral type).  See PEP 353 for details.
 */
pub type Py_ssize_t = ssize_t;
/* Py_hash_t is the same size as a pointer. */
pub type Py_hash_t = Py_ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _typeobject {
    pub ob_base: PyVarObject,
    pub tp_name: *const libc::c_char,
    pub tp_basicsize: Py_ssize_t,
    pub tp_itemsize: Py_ssize_t,
    pub tp_dealloc: destructor,
    pub tp_print: printfunc,
    pub tp_getattr: getattrfunc,
    pub tp_setattr: setattrfunc,
    pub tp_as_async: *mut PyAsyncMethods,
    pub tp_repr: reprfunc,
    pub tp_as_number: *mut PyNumberMethods,
    pub tp_as_sequence: *mut PySequenceMethods,
    pub tp_as_mapping: *mut PyMappingMethods,
    pub tp_hash: hashfunc,
    pub tp_call: ternaryfunc,
    pub tp_str: reprfunc,
    pub tp_getattro: getattrofunc,
    pub tp_setattro: setattrofunc,
    pub tp_as_buffer: *mut PyBufferProcs,
    pub tp_flags: libc::c_ulong,
    pub tp_doc: *const libc::c_char,
    pub tp_traverse: traverseproc,
    pub tp_clear: inquiry,
    pub tp_richcompare: richcmpfunc,
    pub tp_weaklistoffset: Py_ssize_t,
    pub tp_iter: getiterfunc,
    pub tp_iternext: iternextfunc,
    pub tp_methods: *mut PyMethodDef,
    pub tp_members: *mut PyMemberDef,
    pub tp_getset: *mut PyGetSetDef,
    pub tp_base: *mut _typeobject,
    pub tp_dict: *mut PyObject,
    pub tp_descr_get: descrgetfunc,
    pub tp_descr_set: descrsetfunc,
    pub tp_dictoffset: Py_ssize_t,
    pub tp_init: initproc,
    pub tp_alloc: allocfunc,
    pub tp_new: newfunc,
    pub tp_free: freefunc,
    pub tp_is_gc: inquiry,
    pub tp_bases: *mut PyObject,
    pub tp_mro: *mut PyObject,
    pub tp_cache: *mut PyObject,
    pub tp_subclasses: *mut PyObject,
    pub tp_weaklist: *mut PyObject,
    pub tp_del: destructor,
    pub tp_version_tag: libc::c_uint,
    pub tp_finalize: destructor,
}
pub type destructor = Option<unsafe extern "C" fn(_: *mut PyObject) -> ()>;
pub type PyObject = _object;
pub type inquiry = Option<unsafe extern "C" fn(_: *mut PyObject) -> libc::c_int>;
pub type freefunc = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type newfunc = Option<
    unsafe extern "C" fn(_: *mut _typeobject, _: *mut PyObject, _: *mut PyObject) -> *mut PyObject,
>;
pub type allocfunc =
    Option<unsafe extern "C" fn(_: *mut _typeobject, _: Py_ssize_t) -> *mut PyObject>;
pub type initproc = Option<
    unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int,
>;
pub type descrsetfunc = Option<
    unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int,
>;
pub type descrgetfunc = Option<
    unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> *mut PyObject,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyGetSetDef {
    pub name: *mut libc::c_char,
    pub get: getter,
    pub set: setter,
    pub doc: *mut libc::c_char,
    pub closure: *mut libc::c_void,
}
pub type setter = Option<
    unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject, _: *mut libc::c_void) -> libc::c_int,
>;
pub type getter =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_void) -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMemberDef {
    pub name: *mut libc::c_char,
    pub type_0: libc::c_int,
    pub offset: Py_ssize_t,
    pub flags: libc::c_int,
    pub doc: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMethodDef {
    pub ml_name: *const libc::c_char,
    pub ml_meth: PyCFunction,
    pub ml_flags: libc::c_int,
    pub ml_doc: *const libc::c_char,
}
pub type PyCFunction =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
pub type iternextfunc = Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type getiterfunc = Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type richcmpfunc = Option<
    unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject, _: libc::c_int) -> *mut PyObject,
>;
pub type traverseproc = Option<
    unsafe extern "C" fn(_: *mut PyObject, _: visitproc, _: *mut libc::c_void) -> libc::c_int,
>;
pub type visitproc =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyBufferProcs {
    pub bf_getbuffer: getbufferproc,
    pub bf_releasebuffer: releasebufferproc,
}
pub type releasebufferproc =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut Py_buffer) -> ()>;
pub type Py_buffer = bufferinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufferinfo {
    pub buf: *mut libc::c_void,
    pub obj: *mut PyObject,
    pub len: Py_ssize_t,
    pub itemsize: Py_ssize_t,
    pub readonly: libc::c_int,
    pub ndim: libc::c_int,
    pub format: *mut libc::c_char,
    pub shape: *mut Py_ssize_t,
    pub strides: *mut Py_ssize_t,
    pub suboffsets: *mut Py_ssize_t,
    pub internal: *mut libc::c_void,
}
pub type getbufferproc = Option<
    unsafe extern "C" fn(_: *mut PyObject, _: *mut Py_buffer, _: libc::c_int) -> libc::c_int,
>;
pub type setattrofunc = Option<
    unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int,
>;
pub type getattrofunc =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
pub type reprfunc = Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type ternaryfunc = Option<
    unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> *mut PyObject,
>;
pub type hashfunc = Option<unsafe extern "C" fn(_: *mut PyObject) -> Py_hash_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMappingMethods {
    pub mp_length: lenfunc,
    pub mp_subscript: binaryfunc,
    pub mp_ass_subscript: objobjargproc,
}
pub type objobjargproc = Option<
    unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int,
>;
pub type binaryfunc =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
pub type lenfunc = Option<unsafe extern "C" fn(_: *mut PyObject) -> Py_ssize_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PySequenceMethods {
    pub sq_length: lenfunc,
    pub sq_concat: binaryfunc,
    pub sq_repeat: ssizeargfunc,
    pub sq_item: ssizeargfunc,
    pub was_sq_slice: *mut libc::c_void,
    pub sq_ass_item: ssizeobjargproc,
    pub was_sq_ass_slice: *mut libc::c_void,
    pub sq_contains: objobjproc,
    pub sq_inplace_concat: binaryfunc,
    pub sq_inplace_repeat: ssizeargfunc,
}
pub type ssizeargfunc =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t) -> *mut PyObject>;
pub type objobjproc =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type ssizeobjargproc =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t, _: *mut PyObject) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyNumberMethods {
    pub nb_add: binaryfunc,
    pub nb_subtract: binaryfunc,
    pub nb_multiply: binaryfunc,
    pub nb_remainder: binaryfunc,
    pub nb_divmod: binaryfunc,
    pub nb_power: ternaryfunc,
    pub nb_negative: unaryfunc,
    pub nb_positive: unaryfunc,
    pub nb_absolute: unaryfunc,
    pub nb_bool: inquiry,
    pub nb_invert: unaryfunc,
    pub nb_lshift: binaryfunc,
    pub nb_rshift: binaryfunc,
    pub nb_and: binaryfunc,
    pub nb_xor: binaryfunc,
    pub nb_or: binaryfunc,
    pub nb_int: unaryfunc,
    pub nb_reserved: *mut libc::c_void,
    pub nb_float: unaryfunc,
    pub nb_inplace_add: binaryfunc,
    pub nb_inplace_subtract: binaryfunc,
    pub nb_inplace_multiply: binaryfunc,
    pub nb_inplace_remainder: binaryfunc,
    pub nb_inplace_power: ternaryfunc,
    pub nb_inplace_lshift: binaryfunc,
    pub nb_inplace_rshift: binaryfunc,
    pub nb_inplace_and: binaryfunc,
    pub nb_inplace_xor: binaryfunc,
    pub nb_inplace_or: binaryfunc,
    pub nb_floor_divide: binaryfunc,
    pub nb_true_divide: binaryfunc,
    pub nb_inplace_floor_divide: binaryfunc,
    pub nb_inplace_true_divide: binaryfunc,
    pub nb_index: unaryfunc,
    pub nb_matrix_multiply: binaryfunc,
    pub nb_inplace_matrix_multiply: binaryfunc,
}
pub type unaryfunc = Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyAsyncMethods {
    pub am_await: unaryfunc,
    pub am_aiter: unaryfunc,
    pub am_anext: unaryfunc,
}
pub type setattrfunc = Option<
    unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_char, _: *mut PyObject) -> libc::c_int,
>;
pub type getattrfunc =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_char) -> *mut PyObject>;
pub type printfunc =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut FILE, _: libc::c_int) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyVarObject {
    pub ob_base: PyObject,
    pub ob_size: Py_ssize_t,
}
pub type PyTypeObject = _typeobject;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
/* If the compiler provides a wchar_t type we try to support it
through the interface functions PyUnicode_FromWideChar(),
PyUnicode_AsWideChar() and PyUnicode_AsWideCharString(). */
/* Work around a cosmetic bug in BSDI 4.x wchar.h; thanks to Thomas Wouters */
/* Py_UCS4 and Py_UCS2 are typedefs for the respective
unicode representations. */
pub type Py_UCS4 = uint32_t;
pub type Py_UCS2 = uint16_t;
pub type Py_UCS1 = uint8_t;
/* --- Internal Unicode Operations ---------------------------------------- */
/* Since splitting on whitespace is an important use case, and
  whitespace in most situations is solely ASCII whitespace, we
  optimize for the common case by using a quick look-up table
  _Py_ascii_whitespace (see below) with an inlined check.

*/
/* macros to work with surrogates */
/* Join two surrogate characters and return a single Py_UCS4 value. */
/* high surrogate = top 10 bits added to D800 */
/* low surrogate = bottom 10 bits added to DC00 */
/* Check if substring matches at given offset.  The offset must be
valid, and the substring must not be empty. */
/* Py_LIMITED_API */
/* --- Unicode Type ------------------------------------------------------- */
/* ASCII-only strings created through PyUnicode_New use the PyASCIIObject
structure. state.ascii and state.compact are set, and the data
immediately follow the structure. utf8_length and wstr_length can be found
in the length field; the utf8 pointer is equal to the data pointer. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyASCIIObject {
    pub ob_base: PyObject,
    pub length: Py_ssize_t,
    pub hash: Py_hash_t,
    pub state: C2RustUnnamed_0,
    pub wstr: *mut libc::c_int,
    /* wchar_t representation (null-terminated) */
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    #[bitfield(name = "interned", ty = "libc::c_uint", bits = "0..=1")]
    #[bitfield(name = "kind", ty = "libc::c_uint", bits = "2..=4")]
    #[bitfield(name = "compact", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "ascii", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "ready", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "c2rust_unnamed", ty = "libc::c_uint", bits = "8..=31")]
    pub interned_kind_compact_ascii_ready_c2rust_unnamed: [u8; 4],
}
/* Non-ASCII strings allocated through PyUnicode_New use the
PyCompactUnicodeObject structure. state.compact is set, and the data
immediately follow the structure. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyCompactUnicodeObject {
    pub _base: PyASCIIObject,
    pub utf8_length: Py_ssize_t,
    pub utf8: *mut libc::c_char,
    pub wstr_length: Py_ssize_t,
    /* Number of code points in wstr, possible
     * surrogates count as two code points. */
}
/* Strings allocated through PyUnicode_FromUnicode(NULL, len) use the
PyUnicodeObject structure. The actual string data is initially in the wstr
block, and copied into the data block using _PyUnicode_Ready. */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyUnicodeObject {
    pub _base: PyCompactUnicodeObject,
    pub data: C2RustUnnamed_1,
    /* Canonical, smallest-form Unicode buffer */
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub any: *mut libc::c_void,
    pub latin1: *mut Py_UCS1,
    pub ucs2: *mut Py_UCS2,
    pub ucs4: *mut Py_UCS4,
}
/* Long (arbitrary precision) integer object interface */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _longobject {
    pub ob_base: PyVarObject,
    pub ob_digit: [digit; 1],
    /* Revealed in longintrepr.h */
     /* It may be useful in the future. I've added it in the PyInt -> PyLong
    cleanup to keep the extra information. [CH] */
     /* Issue #1983: pid_t can be longer than a C long on some systems */
     /* SIZEOF_PID_T */
     /* SIZEOF_VOID_P */
     /* Used by Python/mystrtoul.c, _PyBytes_FromHex(),
    _PyBytes_DecodeEscapeRecode(), etc. */
     /* _PyLong_Frexp returns a double x and an exponent e such that the
    true value is approximately equal to x * 2**e.  e is >= 0.  x is
    0.0 if and only if the input is 0 (in which case, e and x are both
    zeroes); otherwise, 0.5 <= abs(x) < 1.0.  On overflow, which is
    possible if the number of bits doesn't fit into a Py_ssize_t, sets
    OverflowError and returns -1.0 for x, 0 for e. */
}
pub type digit = uint32_t;
/* Tuple object interface */
/*
Another generally useful object type is a tuple of object pointers.
For Python, this is an immutable type.  C code can change the tuple items
(but not their number), and even use tuples are general-purpose arrays of
object references, but in general only brand new tuples should be mutated,
not ones that might already have been exposed to Python code.

*** WARNING *** PyTuple_SetItem does not increment the new item's reference
count, but does decrement the reference count of the item it replaces,
if not nil.  It does *decrement* the reference count if it is *not*
inserted in the tuple.  Similarly, PyTuple_GetItem does not increment the
returned item's reference count.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyTupleObject {
    pub ob_base: PyVarObject,
    pub ob_item: [*mut PyObject; 1],
    /* ob_item contains space for 'ob_size' elements.
     * Items must normally not be NULL, except during construction when
     * the tuple is not yet visible outside the function that builds it.
     */
}
/* List object interface */
/*
Another generally useful object type is a list of object pointers.
This is a mutable type: the list items can be changed, and items can be
added or removed.  Out-of-range indices or non-list objects are ignored.

*** WARNING *** PyList_SetItem does not increment the new item's reference
count, but does decrement the reference count of the item it replaces,
if not nil.  It does *decrement* the reference count if it is *not*
inserted in the list.  Similarly, PyList_GetItem does not increment the
returned item's reference count.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyListObject {
    pub ob_base: PyVarObject,
    pub ob_item: *mut *mut PyObject,
    pub allocated: Py_ssize_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyModuleDef {
    pub m_base: PyModuleDef_Base,
    pub m_name: *const libc::c_char,
    pub m_doc: *const libc::c_char,
    pub m_size: Py_ssize_t,
    pub m_methods: *mut PyMethodDef,
    pub m_slots: *mut PyModuleDef_Slot,
    pub m_traverse: traverseproc,
    pub m_clear: inquiry,
    pub m_free: freefunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyModuleDef_Slot {
    pub slot: libc::c_int,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyModuleDef_Base {
    pub ob_base: PyObject,
    pub m_init: Option<unsafe extern "C" fn() -> *mut PyObject>,
    pub m_index: Py_ssize_t,
    pub m_copy: *mut PyObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _PyArg_Parser {
    pub format: *const libc::c_char,
    pub keywords: *const *const libc::c_char,
    pub fname: *const libc::c_char,
    pub custom_msg: *const libc::c_char,
    pub pos: libc::c_int,
    pub min: libc::c_int,
    pub max: libc::c_int,
    pub kwtuple: *mut PyObject,
    pub next: *mut _PyArg_Parser,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PatternObject {
    pub ob_base: PyVarObject,
    pub groups: Py_ssize_t,
    pub groupindex: *mut PyObject,
    pub indexgroup: *mut PyObject,
    pub pattern: *mut PyObject,
    pub flags: libc::c_int,
    pub weakreflist: *mut PyObject,
    pub isbytes: libc::c_int,
    pub codesize: Py_ssize_t,
    pub code: [Py_UCS4; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchObject {
    pub ob_base: PyVarObject,
    pub string: *mut PyObject,
    pub regs: *mut PyObject,
    pub pattern: *mut PatternObject,
    pub pos: Py_ssize_t,
    pub endpos: Py_ssize_t,
    pub lastindex: Py_ssize_t,
    pub groups: Py_ssize_t,
    pub mark: [Py_ssize_t; 1],
}
pub type SRE_TOLOWER_HOOK = Option<unsafe extern "C" fn(_: libc::c_uint) -> libc::c_uint>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRE_REPEAT_T {
    pub count: Py_ssize_t,
    pub pattern: *mut Py_UCS4,
    pub last_ptr: *mut libc::c_void,
    pub prev: *mut SRE_REPEAT_T,
}
pub type SRE_REPEAT = SRE_REPEAT_T;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SRE_STATE {
    pub ptr: *mut libc::c_void,
    pub beginning: *mut libc::c_void,
    pub start: *mut libc::c_void,
    pub end: *mut libc::c_void,
    pub string: *mut PyObject,
    pub pos: Py_ssize_t,
    pub endpos: Py_ssize_t,
    pub isbytes: libc::c_int,
    pub charsize: libc::c_int,
    pub lastindex: Py_ssize_t,
    pub lastmark: Py_ssize_t,
    pub mark: *mut *mut libc::c_void,
    pub data_stack: *mut libc::c_char,
    pub data_stack_size: libc::c_int,
    pub data_stack_base: libc::c_int,
    pub buffer: Py_buffer,
    pub repeat: *mut SRE_REPEAT,
    pub lower: SRE_TOLOWER_HOOK,
    pub upper: SRE_TOLOWER_HOOK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScannerObject {
    pub ob_base: PyObject,
    pub pattern: *mut PyObject,
    pub state: SRE_STATE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sre_ucs1_match_context {
    pub last_ctx_pos: Py_ssize_t,
    pub jump: Py_ssize_t,
    pub ptr: *mut Py_UCS1,
    pub pattern: *mut Py_UCS4,
    pub count: Py_ssize_t,
    pub lastmark: Py_ssize_t,
    pub lastindex: Py_ssize_t,
    pub u: C2RustUnnamed_2,
    pub match_all: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub chr: Py_UCS4,
    pub rep: *mut SRE_REPEAT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sre_ucs2_match_context {
    pub last_ctx_pos: Py_ssize_t,
    pub jump: Py_ssize_t,
    pub ptr: *mut Py_UCS2,
    pub pattern: *mut Py_UCS4,
    pub count: Py_ssize_t,
    pub lastmark: Py_ssize_t,
    pub lastindex: Py_ssize_t,
    pub u: C2RustUnnamed_3,
    pub match_all: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub chr: Py_UCS4,
    pub rep: *mut SRE_REPEAT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sre_ucs4_match_context {
    pub last_ctx_pos: Py_ssize_t,
    pub jump: Py_ssize_t,
    pub ptr: *mut Py_UCS4,
    pub pattern: *mut Py_UCS4,
    pub count: Py_ssize_t,
    pub lastmark: Py_ssize_t,
    pub lastindex: Py_ssize_t,
    pub u: C2RustUnnamed_4,
    pub match_all: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub chr: Py_UCS4,
    pub rep: *mut SRE_REPEAT,
}
/* points to REPEAT operator arguments */
/* helper to check for infinite loops */
/* points to previous repeat context */
/*
 * Secret Labs' Regular Expression Engine
 *
 * regular expression matching engine
 *
 * partial history:
 * 1999-10-24 fl   created (based on existing template matcher code)
 * 2000-03-06 fl   first alpha, sort of
 * 2000-08-01 fl   fixes for 1.6b1
 * 2000-08-07 fl   use PyOS_CheckStack() if available
 * 2000-09-20 fl   added expand method
 * 2001-03-20 fl   lots of fixes for 2.1b2
 * 2001-04-15 fl   export copyright as Python attribute, not global
 * 2001-04-28 fl   added __copy__ methods (work in progress)
 * 2001-05-14 fl   fixes for 1.5.2 compatibility
 * 2001-07-01 fl   added BIGCHARSET support (from Martin von Loewis)
 * 2001-10-18 fl   fixed group reset issue (from Matthew Mueller)
 * 2001-10-20 fl   added split primitive; reenable unicode for 1.6/2.0/2.1
 * 2001-10-21 fl   added sub/subn primitive
 * 2001-10-24 fl   added finditer primitive (for 2.2 only)
 * 2001-12-07 fl   fixed memory leak in sub/subn (Guido van Rossum)
 * 2002-11-09 fl   fixed empty sub/subn return type
 * 2003-04-18 mvl  fully support 4-byte codes
 * 2003-10-17 gn   implemented non recursive scheme
 * 2013-02-04 mrab added fullmatch primitive
 *
 * Copyright (c) 1997-2001 by Secret Labs AB.  All rights reserved.
 *
 * This version of the SRE library can be redistributed under CNRI's
 * Python 1.6 license.  For any other use, please contact Secret Labs
 * AB (info@pythonware.com).
 *
 * Portions of this engine have been developed in cooperation with
 * CNRI.  Hewlett-Packard provided funding for 1.6 integration and
 * other compatibility work.
 */
static mut copyright: [libc::c_char; 54] = [
    32, 83, 82, 69, 32, 50, 46, 50, 46, 50, 32, 67, 111, 112, 121, 114, 105, 103, 104, 116, 32, 40,
    99, 41, 32, 49, 57, 57, 55, 45, 50, 48, 48, 50, 32, 98, 121, 32, 83, 101, 99, 114, 101, 116,
    32, 76, 97, 98, 115, 32, 65, 66, 32, 0,
];
/* signal handler raised exception */
/* -------------------------------------------------------------------- */
/* search engine state */

/* locale-specific character predicates */
/* !(c & ~N) == (c < N+1) for any unsigned c, this avoids
 * warnings when c's type supports only numbers < N+1 */
unsafe extern "C" fn sre_lower_locale(mut ch: libc::c_uint) -> libc::c_uint {
    return if ch < 256i32 as libc::c_uint {
        tolower(ch as libc::c_int) as libc::c_uint
    } else {
        ch
    };
}
/* unicode-specific character predicates */
unsafe extern "C" fn sre_lower_unicode(mut ch: libc::c_uint) -> libc::c_uint {
    return _PyUnicode_ToLowercase(ch);
}
#[inline]
unsafe extern "C" fn sre_category(mut category: Py_UCS4, mut ch: libc::c_uint) -> libc::c_int {
    match category {
        SRE_CATEGORY_DIGIT => {
            return (ch < 128i32 as libc::c_uint
                && _Py_ctype_table[(ch & 0xffi32 as libc::c_uint) as libc::c_uchar as usize]
                    & 0x4i32 as libc::c_uint
                    != 0) as libc::c_int
        }
        SRE_CATEGORY_NOT_DIGIT => {
            return !(ch < 128i32 as libc::c_uint
                && _Py_ctype_table[(ch & 0xffi32 as libc::c_uint) as libc::c_uchar as usize]
                    & 0x4i32 as libc::c_uint
                    != 0) as libc::c_int
        }
        SRE_CATEGORY_SPACE => {
            return (ch < 128i32 as libc::c_uint
                && _Py_ctype_table[(ch & 0xffi32 as libc::c_uint) as libc::c_uchar as usize]
                    & 0x8i32 as libc::c_uint
                    != 0) as libc::c_int
        }
        SRE_CATEGORY_NOT_SPACE => {
            return !(ch < 128i32 as libc::c_uint
                && _Py_ctype_table[(ch & 0xffi32 as libc::c_uint) as libc::c_uchar as usize]
                    & 0x8i32 as libc::c_uint
                    != 0) as libc::c_int
        }
        SRE_CATEGORY_WORD => {
            return (ch < 128i32 as libc::c_uint
                && (_Py_ctype_table[(ch & 0xffi32 as libc::c_uint) as libc::c_uchar as usize]
                    & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                    != 0
                    || ch == '_' as i32 as libc::c_uint)) as libc::c_int
        }
        SRE_CATEGORY_NOT_WORD => {
            return !(ch < 128i32 as libc::c_uint
                && (_Py_ctype_table[(ch & 0xffi32 as libc::c_uint) as libc::c_uchar as usize]
                    & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                    != 0
                    || ch == '_' as i32 as libc::c_uint)) as libc::c_int
        }
        SRE_CATEGORY_LINEBREAK => return (ch == '\n' as i32 as libc::c_uint) as libc::c_int,
        SRE_CATEGORY_NOT_LINEBREAK => return !(ch == '\n' as i32 as libc::c_uint) as libc::c_int,
        SRE_CATEGORY_LOC_WORD => {
            return ((if ch & !255i32 as libc::c_uint == 0 {
                *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            } else {
                0i32
            }) != 0
                || ch == '_' as i32 as libc::c_uint) as libc::c_int
        }
        SRE_CATEGORY_LOC_NOT_WORD => {
            return !((if ch & !255i32 as libc::c_uint == 0 {
                *(*__ctype_b_loc()).offset(ch as libc::c_int as isize) as libc::c_int
                    & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
            } else {
                0i32
            }) != 0
                || ch == '_' as i32 as libc::c_uint) as libc::c_int
        }
        SRE_CATEGORY_UNI_DIGIT => return _PyUnicode_IsDecimalDigit(ch),
        SRE_CATEGORY_UNI_NOT_DIGIT => return (_PyUnicode_IsDecimalDigit(ch) == 0) as libc::c_int,
        SRE_CATEGORY_UNI_SPACE => {
            return if ch < 128u32 {
                *_Py_ascii_whitespace.as_ptr().offset(ch as isize) as libc::c_int
            } else {
                _PyUnicode_IsWhitespace(ch)
            }
        }
        SRE_CATEGORY_UNI_NOT_SPACE => {
            return (if ch < 128u32 {
                *_Py_ascii_whitespace.as_ptr().offset(ch as isize) as libc::c_int
            } else {
                _PyUnicode_IsWhitespace(ch)
            } == 0) as libc::c_int
        }
        SRE_CATEGORY_UNI_WORD => {
            return (_PyUnicode_IsAlpha(ch) != 0
                || _PyUnicode_IsDecimalDigit(ch) != 0
                || _PyUnicode_IsDigit(ch) != 0
                || _PyUnicode_IsNumeric(ch) != 0
                || ch == '_' as i32 as libc::c_uint) as libc::c_int
        }
        SRE_CATEGORY_UNI_NOT_WORD => {
            return !(_PyUnicode_IsAlpha(ch) != 0
                || _PyUnicode_IsDecimalDigit(ch) != 0
                || _PyUnicode_IsDigit(ch) != 0
                || _PyUnicode_IsNumeric(ch) != 0
                || ch == '_' as i32 as libc::c_uint) as libc::c_int
        }
        SRE_CATEGORY_UNI_LINEBREAK => return _PyUnicode_IsLinebreak(ch),
        SRE_CATEGORY_UNI_NOT_LINEBREAK => return (_PyUnicode_IsLinebreak(ch) == 0) as libc::c_int,
        _ => {}
    }
    return 0i32;
}
/* helpers */
unsafe extern "C" fn data_stack_dealloc(mut state: *mut SRE_STATE) {
    if !(*state).data_stack.is_null() {
        PyMem_Free((*state).data_stack as *mut libc::c_void);
        (*state).data_stack = 0 as *mut libc::c_char
    };
}
unsafe extern "C" fn data_stack_grow(
    mut state: *mut SRE_STATE,
    mut size: Py_ssize_t,
) -> libc::c_int {
    let mut minsize: Py_ssize_t = 0;
    let mut cursize: Py_ssize_t = 0;
    if cursize < minsize {
        let mut stack: *mut libc::c_void = 0 as *mut libc::c_void;
        cursize = minsize + minsize / 4i32 as libc::c_long + 1024i32 as libc::c_long;
        if stack.is_null() {
            data_stack_dealloc(state);
            return -9i32;
        }
        (*state).data_stack = stack as *mut libc::c_char
    }
    return 0i32;
}
/*
 * Secret Labs' Regular Expression Engine
 *
 * regular expression matching engine
 *
 * Copyright (c) 1997-2001 by Secret Labs AB.  All rights reserved.
 *
 * See the _sre.c file for information on usage and redistribution.
 */
/*
 * Secret Labs' Regular Expression Engine
 *
 * regular expression matching engine
 *
 * Copyright (c) 1997-2001 by Secret Labs AB.  All rights reserved.
 *
 * See the _sre.c file for information on usage and redistribution.
 */
/*
 * Secret Labs' Regular Expression Engine
 *
 * regular expression matching engine
 *
 * Copyright (c) 1997-2001 by Secret Labs AB.  All rights reserved.
 *
 * See the _sre.c file for information on usage and redistribution.
 */
/* String matching engine */
/* String matching engine */
/* String matching engine */
/* This file is included three times, with different character settings */
/* This file is included three times, with different character settings */
/* This file is included three times, with different character settings */
#[inline]
unsafe extern "C" fn sre_ucs2_at(
    mut state: *mut SRE_STATE,
    mut ptr: *mut Py_UCS2,
    mut at: Py_UCS4,
) -> libc::c_int {
    /* check if pointer is at given position */
    /* check if pointer is at given position */
    /* check if pointer is at given position */
    let mut thisp: Py_ssize_t = 0;
    let mut thatp: Py_ssize_t = 0;
    match at {
        0 | 2 => return (ptr as *mut libc::c_void == (*state).beginning) as libc::c_int,
        1 => {
            return (ptr as *mut libc::c_void == (*state).beginning
                || *ptr.offset(-1i32 as isize) as libc::c_int == '\n' as i32)
                as libc::c_int
        }
        5 => {
            return (((*state).end as *mut Py_UCS2).wrapping_offset_from(ptr) as libc::c_long
                == 1i32 as libc::c_long
                && *ptr.offset(0) as libc::c_int == '\n' as i32
                || ptr as *mut libc::c_void == (*state).end) as libc::c_int
        }
        6 => {
            return (ptr as *mut libc::c_void == (*state).end
                || *ptr.offset(0) as libc::c_int == '\n' as i32) as libc::c_int
        }
        7 => return (ptr as *mut libc::c_void == (*state).end) as libc::c_int,
        3 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((*ptr.offset(-1i32 as isize) as libc::c_int) < 128i32
                    && (_Py_ctype_table[(*ptr.offset(-1i32 as isize) as libc::c_int & 0xffi32)
                        as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((*ptr.offset(0) as libc::c_int) < 128i32
                    && (_Py_ctype_table
                        [(*ptr.offset(0) as libc::c_int & 0xffi32) as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(0) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp != thatp) as libc::c_int;
        }
        4 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((*ptr.offset(-1i32 as isize) as libc::c_int) < 128i32
                    && (_Py_ctype_table[(*ptr.offset(-1i32 as isize) as libc::c_int & 0xffi32)
                        as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((*ptr.offset(0) as libc::c_int) < 128i32
                    && (_Py_ctype_table
                        [(*ptr.offset(0) as libc::c_int & 0xffi32) as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(0) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp == thatp) as libc::c_int;
        }
        8 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((if *ptr.offset(-1i32 as isize) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(-1i32 as isize) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((if *ptr.offset(0) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(0) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp != thatp) as libc::c_int;
        }
        9 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((if *ptr.offset(-1i32 as isize) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(-1i32 as isize) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((if *ptr.offset(0) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(0) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp == thatp) as libc::c_int;
        }
        10 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                (_PyUnicode_IsAlpha(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(
                        *ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4
                    ) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                (_PyUnicode_IsAlpha(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp != thatp) as libc::c_int;
        }
        11 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                (_PyUnicode_IsAlpha(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(
                        *ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4
                    ) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                (_PyUnicode_IsAlpha(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp == thatp) as libc::c_int;
        }
        _ => {}
    }
    return 0i32;
}
#[inline]
unsafe extern "C" fn sre_ucs4_at(
    mut state: *mut SRE_STATE,
    mut ptr: *mut Py_UCS4,
    mut at: Py_UCS4,
) -> libc::c_int {
    let mut thisp: Py_ssize_t = 0;
    let mut thatp: Py_ssize_t = 0;
    match at {
        0 | 2 => return (ptr as *mut libc::c_void == (*state).beginning) as libc::c_int,
        1 => {
            return (ptr as *mut libc::c_void == (*state).beginning
                || *ptr.offset(-1i32 as isize) as libc::c_int == '\n' as i32)
                as libc::c_int
        }
        5 => {
            return (((*state).end as *mut Py_UCS4).wrapping_offset_from(ptr) as libc::c_long
                == 1i32 as libc::c_long
                && *ptr.offset(0) as libc::c_int == '\n' as i32
                || ptr as *mut libc::c_void == (*state).end) as libc::c_int
        }
        6 => {
            return (ptr as *mut libc::c_void == (*state).end
                || *ptr.offset(0) as libc::c_int == '\n' as i32) as libc::c_int
        }
        7 => return (ptr as *mut libc::c_void == (*state).end) as libc::c_int,
        3 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((*ptr.offset(-1i32 as isize) as libc::c_int) < 128i32
                    && (_Py_ctype_table[(*ptr.offset(-1i32 as isize) as libc::c_int & 0xffi32)
                        as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((*ptr.offset(0) as libc::c_int) < 128i32
                    && (_Py_ctype_table
                        [(*ptr.offset(0) as libc::c_int & 0xffi32) as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(0) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp != thatp) as libc::c_int;
        }
        4 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((*ptr.offset(-1i32 as isize) as libc::c_int) < 128i32
                    && (_Py_ctype_table[(*ptr.offset(-1i32 as isize) as libc::c_int & 0xffi32)
                        as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((*ptr.offset(0) as libc::c_int) < 128i32
                    && (_Py_ctype_table
                        [(*ptr.offset(0) as libc::c_int & 0xffi32) as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(0) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp == thatp) as libc::c_int;
        }
        8 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((if *ptr.offset(-1i32 as isize) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(-1i32 as isize) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((if *ptr.offset(0) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(0) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp != thatp) as libc::c_int;
        }
        9 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((if *ptr.offset(-1i32 as isize) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(-1i32 as isize) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((if *ptr.offset(0) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(0) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp == thatp) as libc::c_int;
        }
        10 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                (_PyUnicode_IsAlpha(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(
                        *ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4
                    ) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                (_PyUnicode_IsAlpha(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp != thatp) as libc::c_int;
        }
        11 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                (_PyUnicode_IsAlpha(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(
                        *ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4
                    ) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                (_PyUnicode_IsAlpha(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp == thatp) as libc::c_int;
        }
        _ => {}
    }
    return 0i32;
}
#[inline]
unsafe extern "C" fn sre_ucs1_at(
    mut state: *mut SRE_STATE,
    mut ptr: *mut Py_UCS1,
    mut at: Py_UCS4,
) -> libc::c_int {
    let mut thisp: Py_ssize_t = 0;
    let mut thatp: Py_ssize_t = 0;
    match at {
        0 | 2 => return (ptr as *mut libc::c_void == (*state).beginning) as libc::c_int,
        1 => {
            return (ptr as *mut libc::c_void == (*state).beginning
                || *ptr.offset(-1i32 as isize) as libc::c_int == '\n' as i32)
                as libc::c_int
        }
        5 => {
            return (((*state).end as *mut Py_UCS1).wrapping_offset_from(ptr) as libc::c_long
                == 1i32 as libc::c_long
                && *ptr.offset(0) as libc::c_int == '\n' as i32
                || ptr as *mut libc::c_void == (*state).end) as libc::c_int
        }
        6 => {
            return (ptr as *mut libc::c_void == (*state).end
                || *ptr.offset(0) as libc::c_int == '\n' as i32) as libc::c_int
        }
        7 => return (ptr as *mut libc::c_void == (*state).end) as libc::c_int,
        3 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((*ptr.offset(-1i32 as isize) as libc::c_int) < 128i32
                    && (_Py_ctype_table[(*ptr.offset(-1i32 as isize) as libc::c_int & 0xffi32)
                        as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((*ptr.offset(0) as libc::c_int) < 128i32
                    && (_Py_ctype_table
                        [(*ptr.offset(0) as libc::c_int & 0xffi32) as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(0) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp != thatp) as libc::c_int;
        }
        4 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((*ptr.offset(-1i32 as isize) as libc::c_int) < 128i32
                    && (_Py_ctype_table[(*ptr.offset(-1i32 as isize) as libc::c_int & 0xffi32)
                        as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((*ptr.offset(0) as libc::c_int) < 128i32
                    && (_Py_ctype_table
                        [(*ptr.offset(0) as libc::c_int & 0xffi32) as libc::c_uchar as usize]
                        & (0x1i32 | 0x2i32 | 0x4i32) as libc::c_uint
                        != 0
                        || *ptr.offset(0) as libc::c_int == '_' as i32))
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp == thatp) as libc::c_int;
        }
        8 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((if *ptr.offset(-1i32 as isize) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(-1i32 as isize) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((if *ptr.offset(0) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(0) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp != thatp) as libc::c_int;
        }
        9 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                ((if *ptr.offset(-1i32 as isize) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(-1i32 as isize) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                ((if *ptr.offset(0) as libc::c_int & !255i32 == 0 {
                    *(*__ctype_b_loc()).offset(*ptr.offset(0) as libc::c_int as isize)
                        as libc::c_int
                        & _ISalnum as libc::c_int as libc::c_ushort as libc::c_int
                } else {
                    0i32
                }) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp == thatp) as libc::c_int;
        }
        10 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                (_PyUnicode_IsAlpha(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(
                        *ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4
                    ) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                (_PyUnicode_IsAlpha(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp != thatp) as libc::c_int;
        }
        11 => {
            if (*state).beginning == (*state).end {
                return 0i32;
            }
            thatp = (if ptr as *mut libc::c_void > (*state).beginning {
                (_PyUnicode_IsAlpha(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(
                        *ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4
                    ) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(-1i32 as isize) as libc::c_int as Py_UCS4)
                        != 0
                    || *ptr.offset(-1i32 as isize) as libc::c_int == '_' as i32)
                    as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            thisp = (if (ptr as *mut libc::c_void) < (*state).end {
                (_PyUnicode_IsAlpha(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDecimalDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsDigit(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || _PyUnicode_IsNumeric(*ptr.offset(0) as libc::c_int as Py_UCS4) != 0
                    || *ptr.offset(0) as libc::c_int == '_' as i32) as libc::c_int
            } else {
                0i32
            }) as Py_ssize_t;
            return (thisp == thatp) as libc::c_int;
        }
        _ => {}
    }
    return 0i32;
}
#[inline]
unsafe extern "C" fn sre_ucs2_charset(
    mut state: *mut SRE_STATE,
    mut set: *mut Py_UCS4,
    mut ch: Py_UCS4,
) -> libc::c_int {
    /* check if character is a member of the given set */
    /* check if character is a member of the given set */
    /* check if character is a member of the given set */
    let mut ok: libc::c_int = 1i32;
    loop {
        let fresh0 = set;
        set = set.offset(1);
        match *fresh0 {
            0 => return (ok == 0) as libc::c_int,
            19 => {
                /* <LITERAL> <code> */
                /* <LITERAL> <code> */
                /* <LITERAL> <code> */
                if ch == *set.offset(0) {
                    return ok;
                }
                set = set.offset(1)
            }
            9 => {
                /* <CATEGORY> <code> */
                /* <CATEGORY> <code> */
                /* <CATEGORY> <code> */
                if sre_category(*set.offset(0), ch as libc::c_int as libc::c_uint) != 0 {
                    return ok;
                }
                set = set.offset(1)
            }
            10 => {
                /* <CHARSET> <bitmap> */
                /* <CHARSET> <bitmap> */
                /* <CHARSET> <bitmap> */
                if ch < 256i32 as libc::c_uint
                    && *set.offset(
                        (ch as libc::c_ulong).wrapping_div(
                            (8i32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong),
                        ) as isize,
                    ) & 1u32
                        << (ch as libc::c_ulong
                            & (8i32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong)
                                .wrapping_sub(1i32 as libc::c_ulong))
                        != 0
                {
                    return ok;
                }
                set = set.offset(
                    (256i32 as libc::c_ulong).wrapping_div(
                        (8i32 as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong),
                    ) as isize,
                )
            }
            27 => {
                /* <RANGE> <lower> <upper> */
                /* <RANGE> <lower> <upper> */
                /* <RANGE> <lower> <upper> */
                if *set.offset(0) <= ch && ch <= *set.offset(1) {
                    return ok;
                }
                set = set.offset(2)
            }
            32 => {
                /* <RANGE_IGNORE> <lower> <upper> */
                /* <RANGE_IGNORE> <lower> <upper> */
                /* <RANGE_IGNORE> <lower> <upper> */
                let mut uch: Py_UCS4 = 0;
                /* ch is already lower cased */
                /* ch is already lower cased */
                /* ch is already lower cased */
                if *set.offset(0) <= ch && ch <= *set.offset(1) {
                    return ok;
                }
                uch = (*state).upper.expect("non-null function pointer")(ch);
                if *set.offset(0) <= uch && uch <= *set.offset(1) {
                    return ok;
                }
                set = set.offset(2)
            }
            26 => ok = (ok == 0) as libc::c_int,
            11 => {
                /* <BIGCHARSET> <blockcount> <256 blockindices> <blocks> */
                /* <BIGCHARSET> <blockcount> <256 blockindices> <blocks> */
                /* <BIGCHARSET> <blockcount> <256 blockindices> <blocks> */
                let mut count: Py_ssize_t = 0;
                let mut block: Py_ssize_t = 0;
                let fresh1 = set;
                set = set.offset(1);
                count = *fresh1 as Py_ssize_t;
                if ch < 0x10000u32 {
                    block = *(set as *mut libc::c_uchar).offset((ch >> 8i32) as isize) as Py_ssize_t
                } else {
                    block = -1i32 as Py_ssize_t
                }
                set = set.offset(
                    (256i32 as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong)
                        as isize,
                );
                if block >= 0i32 as libc::c_long &&
                       *set.offset(((block * 256i32 as libc::c_long +
                                         (ch & 255i32 as libc::c_uint) as
                                             libc::c_long) as
                                        libc::c_ulong).wrapping_div((8i32 as
                                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<Py_UCS4>()
                                                                                                         as
                                                                                                         libc::c_ulong))
                                       as isize) &
                           1u32 <<
                               (ch as libc::c_ulong &
                                    (8i32 as
                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<Py_UCS4>()
                                                                         as
                                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                                         as
                                                                                                         libc::c_ulong))
                           != 0 {
                    return ok
                }
                set = set.offset(
                    (count as libc::c_ulong).wrapping_mul(
                        (256i32 as libc::c_ulong).wrapping_div(
                            (8i32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong),
                        ),
                    ) as isize,
                )
            }
            _ => {
                /* internal error -- there's not much we can do about it
                here, so let's just pretend it didn't match... */
                /* internal error -- there's not much we can do about it
                here, so let's just pretend it didn't match... */
                /* internal error -- there's not much we can do about it
                here, so let's just pretend it didn't match... */
                return 0i32;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn sre_ucs1_charset(
    mut state: *mut SRE_STATE,
    mut set: *mut Py_UCS4,
    mut ch: Py_UCS4,
) -> libc::c_int {
    let mut ok: libc::c_int = 1i32;
    loop {
        let fresh2 = set;
        set = set.offset(1);
        match *fresh2 {
            0 => return (ok == 0) as libc::c_int,
            19 => {
                if ch == *set.offset(0) {
                    return ok;
                }
                set = set.offset(1)
            }
            9 => {
                if sre_category(*set.offset(0), ch as libc::c_int as libc::c_uint) != 0 {
                    return ok;
                }
                set = set.offset(1)
            }
            10 => {
                if ch < 256i32 as libc::c_uint
                    && *set.offset(
                        (ch as libc::c_ulong).wrapping_div(
                            (8i32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong),
                        ) as isize,
                    ) & 1u32
                        << (ch as libc::c_ulong
                            & (8i32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong)
                                .wrapping_sub(1i32 as libc::c_ulong))
                        != 0
                {
                    return ok;
                }
                set = set.offset(
                    (256i32 as libc::c_ulong).wrapping_div(
                        (8i32 as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong),
                    ) as isize,
                )
            }
            27 => {
                if *set.offset(0) <= ch && ch <= *set.offset(1) {
                    return ok;
                }
                set = set.offset(2)
            }
            32 => {
                let mut uch: Py_UCS4 = 0;
                if *set.offset(0) <= ch && ch <= *set.offset(1) {
                    return ok;
                }
                uch = (*state).upper.expect("non-null function pointer")(ch);
                if *set.offset(0) <= uch && uch <= *set.offset(1) {
                    return ok;
                }
                set = set.offset(2)
            }
            26 => ok = (ok == 0) as libc::c_int,
            11 => {
                let mut count: Py_ssize_t = 0;
                let mut block: Py_ssize_t = 0;
                let fresh3 = set;
                set = set.offset(1);
                count = *fresh3 as Py_ssize_t;
                if ch < 0x10000u32 {
                    block = *(set as *mut libc::c_uchar).offset((ch >> 8i32) as isize) as Py_ssize_t
                } else {
                    block = -1i32 as Py_ssize_t
                }
                set = set.offset(
                    (256i32 as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong)
                        as isize,
                );
                if block >= 0i32 as libc::c_long &&
                       *set.offset(((block * 256i32 as libc::c_long +
                                         (ch & 255i32 as libc::c_uint) as
                                             libc::c_long) as
                                        libc::c_ulong).wrapping_div((8i32 as
                                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<Py_UCS4>()
                                                                                                         as
                                                                                                         libc::c_ulong))
                                       as isize) &
                           1u32 <<
                               (ch as libc::c_ulong &
                                    (8i32 as
                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<Py_UCS4>()
                                                                         as
                                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                                         as
                                                                                                         libc::c_ulong))
                           != 0 {
                    return ok
                }
                set = set.offset(
                    (count as libc::c_ulong).wrapping_mul(
                        (256i32 as libc::c_ulong).wrapping_div(
                            (8i32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong),
                        ),
                    ) as isize,
                )
            }
            _ => return 0i32,
        }
    }
}
#[inline]
unsafe extern "C" fn sre_ucs4_charset(
    mut state: *mut SRE_STATE,
    mut set: *mut Py_UCS4,
    mut ch: Py_UCS4,
) -> libc::c_int {
    let mut ok: libc::c_int = 1i32;
    loop {
        let fresh4 = set;
        set = set.offset(1);
        match *fresh4 {
            0 => return (ok == 0) as libc::c_int,
            19 => {
                if ch == *set.offset(0) {
                    return ok;
                }
                set = set.offset(1)
            }
            9 => {
                if sre_category(*set.offset(0), ch as libc::c_int as libc::c_uint) != 0 {
                    return ok;
                }
                set = set.offset(1)
            }
            10 => {
                if ch < 256i32 as libc::c_uint
                    && *set.offset(
                        (ch as libc::c_ulong).wrapping_div(
                            (8i32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong),
                        ) as isize,
                    ) & 1u32
                        << (ch as libc::c_ulong
                            & (8i32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong)
                                .wrapping_sub(1i32 as libc::c_ulong))
                        != 0
                {
                    return ok;
                }
                set = set.offset(
                    (256i32 as libc::c_ulong).wrapping_div(
                        (8i32 as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong),
                    ) as isize,
                )
            }
            27 => {
                if *set.offset(0) <= ch && ch <= *set.offset(1) {
                    return ok;
                }
                set = set.offset(2)
            }
            32 => {
                let mut uch: Py_UCS4 = 0;
                if *set.offset(0) <= ch && ch <= *set.offset(1) {
                    return ok;
                }
                uch = (*state).upper.expect("non-null function pointer")(ch);
                if *set.offset(0) <= uch && uch <= *set.offset(1) {
                    return ok;
                }
                set = set.offset(2)
            }
            26 => ok = (ok == 0) as libc::c_int,
            11 => {
                let mut count: Py_ssize_t = 0;
                let mut block: Py_ssize_t = 0;
                let fresh5 = set;
                set = set.offset(1);
                count = *fresh5 as Py_ssize_t;
                if ch < 0x10000u32 {
                    block = *(set as *mut libc::c_uchar).offset((ch >> 8i32) as isize) as Py_ssize_t
                } else {
                    block = -1i32 as Py_ssize_t
                }
                set = set.offset(
                    (256i32 as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong)
                        as isize,
                );
                if block >= 0i32 as libc::c_long &&
                       *set.offset(((block * 256i32 as libc::c_long +
                                         (ch & 255i32 as libc::c_uint) as
                                             libc::c_long) as
                                        libc::c_ulong).wrapping_div((8i32 as
                                                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<Py_UCS4>()
                                                                                                         as
                                                                                                         libc::c_ulong))
                                       as isize) &
                           1u32 <<
                               (ch as libc::c_ulong &
                                    (8i32 as
                                         libc::c_ulong).wrapping_mul(::std::mem::size_of::<Py_UCS4>()
                                                                         as
                                                                         libc::c_ulong).wrapping_sub(1i32
                                                                                                         as
                                                                                                         libc::c_ulong))
                           != 0 {
                    return ok
                }
                set = set.offset(
                    (count as libc::c_ulong).wrapping_mul(
                        (256i32 as libc::c_ulong).wrapping_div(
                            (8i32 as libc::c_ulong)
                                .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong),
                        ),
                    ) as isize,
                )
            }
            _ => return 0i32,
        }
    }
}
#[inline]
unsafe extern "C" fn sre_ucs4_count(
    mut state: *mut SRE_STATE,
    mut pattern: *mut Py_UCS4,
    mut maxcount: Py_ssize_t,
) -> Py_ssize_t {
    let mut chr: Py_UCS4 = 0;
    let mut c: Py_UCS4 = 0;
    let mut ptr: *mut Py_UCS4 = (*state).ptr as *mut Py_UCS4;
    let mut end: *mut Py_UCS4 = (*state).end as *mut Py_UCS4;
    let mut i: Py_ssize_t = 0;
    /* adjust end */
    /* adjust end */
    /* adjust end */
    if maxcount < end.wrapping_offset_from(ptr) as libc::c_long
        && maxcount != !(0i32 as Py_UCS4) as libc::c_long
    {
        end = ptr.offset(maxcount as isize)
    }
    match *pattern.offset(0) {
        15 => {
            /* repeated set */
            /* repeated set */
            /* repeated set */
            while ptr < end && sre_ucs4_charset(state, pattern.offset(2), *ptr) != 0 {
                ptr = ptr.offset(1)
            }
        }
        2 => {
            /* repeated dot wildcard. */
            /* repeated dot wildcard. */
            /* repeated dot wildcard. */
            while ptr < end && !(*ptr == '\n' as i32 as libc::c_uint) {
                ptr = ptr.offset(1)
            }
        }
        3 => {
            /* repeated dot wildcard.  skip to the end of the target
            string, and backtrack from there */
            /* repeated dot wildcard.  skip to the end of the target
            string, and backtrack from there */
            /* repeated dot wildcard.  skip to the end of the target
            string, and backtrack from there */
            ptr = end
        }
        19 => {
            /* repeated literal */
            /* repeated literal */
            /* repeated literal */
            chr = *pattern.offset(1);
            c = chr;
            /* literal can't match: doesn't fit in char width */
            /* literal can't match: doesn't fit in char width */
            while ptr < end && *ptr == c {
                ptr = ptr.offset(1)
            }
        }
        20 => {
            /* repeated literal */
            /* repeated literal */
            /* repeated literal */
            chr = *pattern.offset(1);
            while ptr < end && (*state).lower.expect("non-null function pointer")(*ptr) == chr {
                ptr = ptr.offset(1)
            }
        }
        24 => {
            /* repeated non-literal */
            /* repeated non-literal */
            /* repeated non-literal */
            chr = *pattern.offset(1);
            c = chr;
            /* literal can't match: doesn't fit in char width */
            /* literal can't match: doesn't fit in char width */
            while ptr < end && *ptr != c {
                ptr = ptr.offset(1)
            }
        }
        25 => {
            /* repeated non-literal */
            /* repeated non-literal */
            /* repeated non-literal */
            chr = *pattern.offset(1);
            while ptr < end && (*state).lower.expect("non-null function pointer")(*ptr) != chr {
                ptr = ptr.offset(1)
            }
        }
        _ => {
            /* repeated single character pattern */
            /* repeated single character pattern */
            /* repeated single character pattern */
            while ((*state).ptr as *mut Py_UCS4) < end {
                i = sre_ucs4_match(state, pattern, 0i32);
                if i < 0i32 as libc::c_long {
                    return i;
                }
                if i == 0 {
                    break;
                }
            }
            return ((*state).ptr as *mut Py_UCS4).wrapping_offset_from(ptr) as libc::c_long;
        }
    }
    return ptr.wrapping_offset_from((*state).ptr as *mut Py_UCS4) as libc::c_long;
}
#[inline]
unsafe extern "C" fn sre_ucs2_count(
    mut state: *mut SRE_STATE,
    mut pattern: *mut Py_UCS4,
    mut maxcount: Py_ssize_t,
) -> Py_ssize_t {
    let mut chr: Py_UCS4 = 0;
    let mut c: Py_UCS2 = 0;
    let mut ptr: *mut Py_UCS2 = (*state).ptr as *mut Py_UCS2;
    let mut end: *mut Py_UCS2 = (*state).end as *mut Py_UCS2;
    let mut i: Py_ssize_t = 0;
    if maxcount < end.wrapping_offset_from(ptr) as libc::c_long
        && maxcount != !(0i32 as Py_UCS4) as libc::c_long
    {
        end = ptr.offset(maxcount as isize)
    }
    match *pattern.offset(0) {
        15 => {
            while ptr < end && sre_ucs2_charset(state, pattern.offset(2), *ptr as Py_UCS4) != 0 {
                ptr = ptr.offset(1)
            }
        }
        2 => {
            while ptr < end && !(*ptr as libc::c_int == '\n' as i32) {
                ptr = ptr.offset(1)
            }
        }
        3 => ptr = end,
        19 => {
            chr = *pattern.offset(1);
            c = chr as Py_UCS2;
            if !(c as Py_UCS4 != chr) {
                while ptr < end && *ptr as libc::c_int == c as libc::c_int {
                    ptr = ptr.offset(1)
                }
            }
        }
        20 => {
            chr = *pattern.offset(1);
            while ptr < end
                && (*state).lower.expect("non-null function pointer")(*ptr as libc::c_uint) == chr
            {
                ptr = ptr.offset(1)
            }
        }
        24 => {
            chr = *pattern.offset(1);
            c = chr as Py_UCS2;
            if c as Py_UCS4 != chr {
                ptr = end
            } else {
                while ptr < end && *ptr as libc::c_int != c as libc::c_int {
                    ptr = ptr.offset(1)
                }
            }
        }
        25 => {
            chr = *pattern.offset(1);
            while ptr < end
                && (*state).lower.expect("non-null function pointer")(*ptr as libc::c_uint) != chr
            {
                ptr = ptr.offset(1)
            }
        }
        _ => {
            while ((*state).ptr as *mut Py_UCS2) < end {
                i = sre_ucs2_match(state, pattern, 0i32);
                if i < 0i32 as libc::c_long {
                    return i;
                }
                if i == 0 {
                    break;
                }
            }
            return ((*state).ptr as *mut Py_UCS2).wrapping_offset_from(ptr) as libc::c_long;
        }
    }
    return ptr.wrapping_offset_from((*state).ptr as *mut Py_UCS2) as libc::c_long;
}
#[inline]
unsafe extern "C" fn sre_ucs1_count(
    mut state: *mut SRE_STATE,
    mut pattern: *mut Py_UCS4,
    mut maxcount: Py_ssize_t,
) -> Py_ssize_t {
    let mut chr: Py_UCS4 = 0;
    let mut c: Py_UCS1 = 0;
    let mut ptr: *mut Py_UCS1 = (*state).ptr as *mut Py_UCS1;
    let mut end: *mut Py_UCS1 = (*state).end as *mut Py_UCS1;
    let mut i: Py_ssize_t = 0;
    if maxcount < end.wrapping_offset_from(ptr) as libc::c_long
        && maxcount != !(0i32 as Py_UCS4) as libc::c_long
    {
        end = ptr.offset(maxcount as isize)
    }
    match *pattern.offset(0) {
        15 => {
            while ptr < end && sre_ucs1_charset(state, pattern.offset(2), *ptr as Py_UCS4) != 0 {
                ptr = ptr.offset(1)
            }
        }
        2 => {
            while ptr < end && !(*ptr as libc::c_int == '\n' as i32) {
                ptr = ptr.offset(1)
            }
        }
        3 => ptr = end,
        19 => {
            chr = *pattern.offset(1);
            c = chr as Py_UCS1;
            if !(c as Py_UCS4 != chr) {
                while ptr < end && *ptr as libc::c_int == c as libc::c_int {
                    ptr = ptr.offset(1)
                }
            }
        }
        20 => {
            chr = *pattern.offset(1);
            while ptr < end
                && (*state).lower.expect("non-null function pointer")(*ptr as libc::c_uint) == chr
            {
                ptr = ptr.offset(1)
            }
        }
        24 => {
            chr = *pattern.offset(1);
            c = chr as Py_UCS1;
            if c as Py_UCS4 != chr {
                ptr = end
            } else {
                while ptr < end && *ptr as libc::c_int != c as libc::c_int {
                    ptr = ptr.offset(1)
                }
            }
        }
        25 => {
            chr = *pattern.offset(1);
            while ptr < end
                && (*state).lower.expect("non-null function pointer")(*ptr as libc::c_uint) != chr
            {
                ptr = ptr.offset(1)
            }
        }
        _ => {
            while ((*state).ptr as *mut Py_UCS1) < end {
                i = sre_ucs1_match(state, pattern, 0i32);
                if i < 0i32 as libc::c_long {
                    return i;
                }
                if i == 0 {
                    break;
                }
            }
            return ((*state).ptr as *mut Py_UCS1).wrapping_offset_from(ptr) as libc::c_long;
        }
    }
    return ptr.wrapping_offset_from((*state).ptr as *mut Py_UCS1) as libc::c_long;
}
#[inline]
unsafe extern "C" fn sre_ucs4_search(
    mut state: *mut SRE_STATE,
    mut pattern: *mut Py_UCS4,
) -> Py_ssize_t {
    let mut ptr: *mut Py_UCS4 = (*state).start as *mut Py_UCS4;
    let mut end: *mut Py_UCS4 = (*state).end as *mut Py_UCS4;
    let mut status: Py_ssize_t = 0i32 as Py_ssize_t;
    let mut prefix_len: Py_ssize_t = 0i32 as Py_ssize_t;
    let mut prefix_skip: Py_ssize_t = 0i32 as Py_ssize_t;
    let mut prefix: *mut Py_UCS4 = 0 as *mut Py_UCS4;
    let mut charset: *mut Py_UCS4 = 0 as *mut Py_UCS4;
    let mut overlap: *mut Py_UCS4 = 0 as *mut Py_UCS4;
    let mut flags: libc::c_int = 0i32;
    if ptr > end {
        return 0i32 as Py_ssize_t;
    }
    if *pattern.offset(0) == 17i32 as libc::c_uint {
        /* optimization info block */
        /* <INFO> <1=skip> <2=flags> <3=min> <4=max> <5=prefix info>  */
        /* optimization info block */
        /* <INFO> <1=skip> <2=flags> <3=min> <4=max> <5=prefix info>  */
        /* optimization info block */
        /* <INFO> <1=skip> <2=flags> <3=min> <4=max> <5=prefix info>  */
        flags = *pattern.offset(2) as libc::c_int;
        if *pattern.offset(3) != 0
            && (end.wrapping_offset_from(ptr) as libc::c_long) < *pattern.offset(3) as Py_ssize_t
        {
            return 0i32 as Py_ssize_t;
        }
        if *pattern.offset(3) > 1i32 as libc::c_uint {
            /* adjust end point (but make sure we leave at least one
            character in there, so literal search will work) */
            /* adjust end point (but make sure we leave at least one
            character in there, so literal search will work) */
            /* adjust end point (but make sure we leave at least one
            character in there, so literal search will work) */
            end = end.offset(-((*pattern.offset(3)).wrapping_sub(1i32 as libc::c_uint) as isize));
            if end <= ptr {
                end = ptr
            }
        }
        if flags & 1i32 != 0 {
            /* pattern starts with a known prefix */
            /* <length> <skip> <prefix data> <overlap data> */
            /* pattern starts with a known prefix */
            /* <length> <skip> <prefix data> <overlap data> */
            /* pattern starts with a known prefix */
            /* <length> <skip> <prefix data> <overlap data> */
            prefix_len = *pattern.offset(5) as Py_ssize_t;
            prefix_skip = *pattern.offset(6) as Py_ssize_t;
            prefix = pattern.offset(7);
            overlap = prefix.offset(prefix_len as isize).offset(-1)
        } else if flags & 4i32 != 0 {
            /* pattern starts with a character from a known set */
            /* <charset> */
            /* pattern starts with a character from a known set */
            /* <charset> */
            /* pattern starts with a character from a known set */
            /* <charset> */
            charset = pattern.offset(5)
        }
        pattern = pattern.offset((1i32 as libc::c_uint).wrapping_add(*pattern.offset(1)) as isize)
    }
    if prefix_len == 1i32 as libc::c_long {
        /* pattern starts with a literal character */
        /* pattern starts with a literal character */
        /* pattern starts with a literal character */
        let mut c: Py_UCS4 = *prefix.offset(0);
        /* literal can't match: doesn't fit in char width */
        /* literal can't match: doesn't fit in char width */
        end = (*state).end as *mut Py_UCS4; /* we got all of it */
        while ptr < end {
            while *ptr != c {
                ptr = ptr.offset(1); /* we got all of it */
                if ptr >= end {
                    return 0i32 as Py_ssize_t;
                }
            } /* we got all of it */
            (*state).start = ptr as *mut libc::c_void;
            (*state).ptr = ptr.offset(prefix_skip as isize) as *mut libc::c_void;
            if flags & 2i32 != 0 {
                return 1i32 as Py_ssize_t;
            }
            status = sre_ucs4_match(
                state,
                pattern.offset((2i32 as libc::c_long * prefix_skip) as isize),
                0i32,
            );
            if status != 0i32 as libc::c_long {
                return status;
            }
            ptr = ptr.offset(1)
        }
        return 0i32 as Py_ssize_t;
    }
    if prefix_len > 1i32 as libc::c_long {
        /* pattern starts with a known prefix.  use the overlap
        table to skip forward as fast as we possibly can */
        /* pattern starts with a known prefix.  use the overlap
        table to skip forward as fast as we possibly can */
        /* pattern starts with a known prefix.  use the overlap
        table to skip forward as fast as we possibly can */
        let mut i: Py_ssize_t = 0i32 as Py_ssize_t;
        end = (*state).end as *mut Py_UCS4;
        if prefix_len > end.wrapping_offset_from(ptr) as libc::c_long {
            return 0i32 as Py_ssize_t;
        }
        /* literal can't match: doesn't fit in char width */
        /* literal can't match: doesn't fit in char width */
        while ptr < end {
            let mut c_0: Py_UCS4 = *prefix.offset(0);
            loop {
                let fresh6 = ptr;
                ptr = ptr.offset(1);
                if !(*fresh6 != c_0) {
                    break;
                }
                if ptr >= end {
                    return 0i32 as Py_ssize_t;
                }
            }
            if ptr >= end {
                return 0i32 as Py_ssize_t;
            }
            i = 1i32 as Py_ssize_t;
            let mut current_block_66: u64;
            loop {
                if *ptr == *prefix.offset(i as isize) {
                    i += 1;
                    if i != prefix_len {
                        ptr = ptr.offset(1);
                        if ptr >= end {
                            return 0i32 as Py_ssize_t;
                        }
                        current_block_66 = 7343950298149844727;
                    } else {
                        /* found a potential match */
                        /* found a potential match */
                        /* found a potential match */
                        (*state).start = ptr.offset(-((prefix_len - 1i32 as libc::c_long) as isize))
                            as *mut libc::c_void; /* we got all of it */
                        (*state).ptr = ptr
                            .offset(-((prefix_len - prefix_skip - 1i32 as libc::c_long) as isize))
                            as *mut libc::c_void; /* we got all of it */
                        if flags & 2i32 != 0 {
                            return 1i32 as Py_ssize_t;
                        } /* we got all of it */
                        status = sre_ucs4_match(
                            state,
                            pattern.offset((2i32 as libc::c_long * prefix_skip) as isize),
                            0i32,
                        );
                        if status != 0i32 as libc::c_long {
                            return status;
                        }
                        /* close but no cigar -- try again */
                        /* close but no cigar -- try again */
                        /* close but no cigar -- try again */
                        ptr = ptr.offset(1);
                        if ptr >= end {
                            return 0i32 as Py_ssize_t;
                        }
                        current_block_66 = 7189308829251266000;
                    }
                } else {
                    current_block_66 = 7189308829251266000;
                }
                match current_block_66 {
                    7189308829251266000 => i = *overlap.offset(i as isize) as Py_ssize_t,
                    _ => {}
                }
                if !(i != 0i32 as libc::c_long) {
                    break;
                }
            }
        }
        return 0i32 as Py_ssize_t;
    }
    if !charset.is_null() {
        /* pattern starts with a character from a known set */
        /* pattern starts with a character from a known set */
        /* pattern starts with a character from a known set */
        end = (*state).end as *mut Py_UCS4;
        loop {
            while ptr < end && sre_ucs4_charset(state, charset, *ptr) == 0 {
                ptr = ptr.offset(1)
            }
            if ptr >= end {
                return 0i32 as Py_ssize_t;
            }
            (*state).start = ptr as *mut libc::c_void;
            (*state).ptr = ptr as *mut libc::c_void;
            status = sre_ucs4_match(state, pattern, 0i32);
            if status != 0i32 as libc::c_long {
                break;
            }
            ptr = ptr.offset(1)
        }
    } else {
        /* general case */
        /* general case */
        /* general case */
        if ptr <= end {
        } else {
            __assert_fail(
                b"ptr <= end\x00" as *const u8 as *const libc::c_char,
                b"Modules/sre_lib.h\x00" as *const u8 as *const libc::c_char,
                1350i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                    b"Py_ssize_t sre_ucs4_search(SRE_STATE *, Py_UCS4 *)\x00",
                ))
                .as_ptr(),
            );
        }
        loop {
            (*state).ptr = ptr as *mut libc::c_void;
            (*state).start = (*state).ptr;
            status = sre_ucs4_match(state, pattern, 0i32);
            if status != 0i32 as libc::c_long || ptr >= end {
                break;
            }
            ptr = ptr.offset(1)
        }
    }
    return status;
}
#[inline]
unsafe extern "C" fn sre_ucs1_search(
    mut state: *mut SRE_STATE,
    mut pattern: *mut Py_UCS4,
) -> Py_ssize_t {
    let mut ptr: *mut Py_UCS1 = (*state).start as *mut Py_UCS1;
    let mut end: *mut Py_UCS1 = (*state).end as *mut Py_UCS1;
    let mut status: Py_ssize_t = 0i32 as Py_ssize_t;
    let mut prefix_len: Py_ssize_t = 0i32 as Py_ssize_t;
    let mut prefix_skip: Py_ssize_t = 0i32 as Py_ssize_t;
    let mut prefix: *mut Py_UCS4 = 0 as *mut Py_UCS4;
    let mut charset: *mut Py_UCS4 = 0 as *mut Py_UCS4;
    let mut overlap: *mut Py_UCS4 = 0 as *mut Py_UCS4;
    let mut flags: libc::c_int = 0i32;
    if ptr > end {
        return 0i32 as Py_ssize_t;
    }
    if *pattern.offset(0) == 17i32 as libc::c_uint {
        flags = *pattern.offset(2) as libc::c_int;
        if *pattern.offset(3) != 0
            && (end.wrapping_offset_from(ptr) as libc::c_long) < *pattern.offset(3) as Py_ssize_t
        {
            return 0i32 as Py_ssize_t;
        }
        if *pattern.offset(3) > 1i32 as libc::c_uint {
            end = end.offset(-((*pattern.offset(3)).wrapping_sub(1i32 as libc::c_uint) as isize));
            if end <= ptr {
                end = ptr
            }
        }
        if flags & 1i32 != 0 {
            prefix_len = *pattern.offset(5) as Py_ssize_t;
            prefix_skip = *pattern.offset(6) as Py_ssize_t;
            prefix = pattern.offset(7);
            overlap = prefix.offset(prefix_len as isize).offset(-1)
        } else if flags & 4i32 != 0 {
            charset = pattern.offset(5)
        }
        pattern = pattern.offset((1i32 as libc::c_uint).wrapping_add(*pattern.offset(1)) as isize)
    }
    if prefix_len == 1i32 as libc::c_long {
        let mut c: Py_UCS1 = *prefix.offset(0) as Py_UCS1;
        if c as Py_UCS4 != *prefix.offset(0) {
            return 0i32 as Py_ssize_t;
        }
        end = (*state).end as *mut Py_UCS1;
        while ptr < end {
            while *ptr as libc::c_int != c as libc::c_int {
                ptr = ptr.offset(1);
                if ptr >= end {
                    return 0i32 as Py_ssize_t;
                }
            }
            (*state).start = ptr as *mut libc::c_void;
            (*state).ptr = ptr.offset(prefix_skip as isize) as *mut libc::c_void;
            if flags & 2i32 != 0 {
                return 1i32 as Py_ssize_t;
            }
            status = sre_ucs1_match(
                state,
                pattern.offset((2i32 as libc::c_long * prefix_skip) as isize),
                0i32,
            );
            if status != 0i32 as libc::c_long {
                return status;
            }
            ptr = ptr.offset(1)
        }
        return 0i32 as Py_ssize_t;
    }
    if prefix_len > 1i32 as libc::c_long {
        let mut i: Py_ssize_t = 0i32 as Py_ssize_t;
        end = (*state).end as *mut Py_UCS1;
        if prefix_len > end.wrapping_offset_from(ptr) as libc::c_long {
            return 0i32 as Py_ssize_t;
        }
        i = 0i32 as Py_ssize_t;
        while i < prefix_len {
            if *prefix.offset(i as isize) as Py_UCS1 as Py_UCS4 != *prefix.offset(i as isize) {
                return 0i32 as Py_ssize_t;
            }
            i += 1
        }
        while ptr < end {
            let mut c_0: Py_UCS1 = *prefix.offset(0) as Py_UCS1;
            loop {
                let fresh7 = ptr;
                ptr = ptr.offset(1);
                if !(*fresh7 as libc::c_int != c_0 as libc::c_int) {
                    break;
                }
                if ptr >= end {
                    return 0i32 as Py_ssize_t;
                }
            }
            if ptr >= end {
                return 0i32 as Py_ssize_t;
            }
            i = 1i32 as Py_ssize_t;
            let mut current_block_72: u64;
            loop {
                if *ptr as libc::c_int == *prefix.offset(i as isize) as Py_UCS1 as libc::c_int {
                    i += 1;
                    if i != prefix_len {
                        ptr = ptr.offset(1);
                        if ptr >= end {
                            return 0i32 as Py_ssize_t;
                        }
                        current_block_72 = 16203797167131938757;
                    } else {
                        (*state).start = ptr.offset(-((prefix_len - 1i32 as libc::c_long) as isize))
                            as *mut libc::c_void;
                        (*state).ptr = ptr
                            .offset(-((prefix_len - prefix_skip - 1i32 as libc::c_long) as isize))
                            as *mut libc::c_void;
                        if flags & 2i32 != 0 {
                            return 1i32 as Py_ssize_t;
                        }
                        status = sre_ucs1_match(
                            state,
                            pattern.offset((2i32 as libc::c_long * prefix_skip) as isize),
                            0i32,
                        );
                        if status != 0i32 as libc::c_long {
                            return status;
                        }
                        ptr = ptr.offset(1);
                        if ptr >= end {
                            return 0i32 as Py_ssize_t;
                        }
                        current_block_72 = 8834769789432328951;
                    }
                } else {
                    current_block_72 = 8834769789432328951;
                }
                match current_block_72 {
                    8834769789432328951 => i = *overlap.offset(i as isize) as Py_ssize_t,
                    _ => {}
                }
                if !(i != 0i32 as libc::c_long) {
                    break;
                }
            }
        }
        return 0i32 as Py_ssize_t;
    }
    if !charset.is_null() {
        end = (*state).end as *mut Py_UCS1;
        loop {
            while ptr < end && sre_ucs1_charset(state, charset, *ptr as Py_UCS4) == 0 {
                ptr = ptr.offset(1)
            }
            if ptr >= end {
                return 0i32 as Py_ssize_t;
            }
            (*state).start = ptr as *mut libc::c_void;
            (*state).ptr = ptr as *mut libc::c_void;
            status = sre_ucs1_match(state, pattern, 0i32);
            if status != 0i32 as libc::c_long {
                break;
            }
            ptr = ptr.offset(1)
        }
    } else {
        if ptr <= end {
        } else {
            __assert_fail(
                b"ptr <= end\x00" as *const u8 as *const libc::c_char,
                b"Modules/sre_lib.h\x00" as *const u8 as *const libc::c_char,
                1350i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                    b"Py_ssize_t sre_ucs1_search(SRE_STATE *, Py_UCS4 *)\x00",
                ))
                .as_ptr(),
            );
        }
        loop {
            (*state).ptr = ptr as *mut libc::c_void;
            (*state).start = (*state).ptr;
            status = sre_ucs1_match(state, pattern, 0i32);
            if status != 0i32 as libc::c_long || ptr >= end {
                break;
            }
            ptr = ptr.offset(1)
        }
    }
    return status;
}
#[inline]
unsafe extern "C" fn sre_ucs2_search(
    mut state: *mut SRE_STATE,
    mut pattern: *mut Py_UCS4,
) -> Py_ssize_t {
    let mut ptr: *mut Py_UCS2 = (*state).start as *mut Py_UCS2;
    let mut end: *mut Py_UCS2 = (*state).end as *mut Py_UCS2;
    let mut status: Py_ssize_t = 0i32 as Py_ssize_t;
    let mut prefix_len: Py_ssize_t = 0i32 as Py_ssize_t;
    let mut prefix_skip: Py_ssize_t = 0i32 as Py_ssize_t;
    let mut prefix: *mut Py_UCS4 = 0 as *mut Py_UCS4;
    let mut charset: *mut Py_UCS4 = 0 as *mut Py_UCS4;
    let mut overlap: *mut Py_UCS4 = 0 as *mut Py_UCS4;
    let mut flags: libc::c_int = 0i32;
    if ptr > end {
        return 0i32 as Py_ssize_t;
    }
    if *pattern.offset(0) == 17i32 as libc::c_uint {
        flags = *pattern.offset(2) as libc::c_int;
        if *pattern.offset(3) != 0
            && (end.wrapping_offset_from(ptr) as libc::c_long) < *pattern.offset(3) as Py_ssize_t
        {
            return 0i32 as Py_ssize_t;
        }
        if *pattern.offset(3) > 1i32 as libc::c_uint {
            end = end.offset(-((*pattern.offset(3)).wrapping_sub(1i32 as libc::c_uint) as isize));
            if end <= ptr {
                end = ptr
            }
        }
        if flags & 1i32 != 0 {
            prefix_len = *pattern.offset(5) as Py_ssize_t;
            prefix_skip = *pattern.offset(6) as Py_ssize_t;
            prefix = pattern.offset(7);
            overlap = prefix.offset(prefix_len as isize).offset(-1)
        } else if flags & 4i32 != 0 {
            charset = pattern.offset(5)
        }
        pattern = pattern.offset((1i32 as libc::c_uint).wrapping_add(*pattern.offset(1)) as isize)
    }
    if prefix_len == 1i32 as libc::c_long {
        let mut c: Py_UCS2 = *prefix.offset(0) as Py_UCS2;
        if c as Py_UCS4 != *prefix.offset(0) {
            return 0i32 as Py_ssize_t;
        }
        end = (*state).end as *mut Py_UCS2;
        while ptr < end {
            while *ptr as libc::c_int != c as libc::c_int {
                ptr = ptr.offset(1);
                if ptr >= end {
                    return 0i32 as Py_ssize_t;
                }
            }
            (*state).start = ptr as *mut libc::c_void;
            (*state).ptr = ptr.offset(prefix_skip as isize) as *mut libc::c_void;
            if flags & 2i32 != 0 {
                return 1i32 as Py_ssize_t;
            }
            status = sre_ucs2_match(
                state,
                pattern.offset((2i32 as libc::c_long * prefix_skip) as isize),
                0i32,
            );
            if status != 0i32 as libc::c_long {
                return status;
            }
            ptr = ptr.offset(1)
        }
        return 0i32 as Py_ssize_t;
    }
    if prefix_len > 1i32 as libc::c_long {
        let mut i: Py_ssize_t = 0i32 as Py_ssize_t;
        end = (*state).end as *mut Py_UCS2;
        if prefix_len > end.wrapping_offset_from(ptr) as libc::c_long {
            return 0i32 as Py_ssize_t;
        }
        i = 0i32 as Py_ssize_t;
        while i < prefix_len {
            if *prefix.offset(i as isize) as Py_UCS2 as Py_UCS4 != *prefix.offset(i as isize) {
                return 0i32 as Py_ssize_t;
            }
            i += 1
        }
        while ptr < end {
            let mut c_0: Py_UCS2 = *prefix.offset(0) as Py_UCS2;
            loop {
                let fresh8 = ptr;
                ptr = ptr.offset(1);
                if !(*fresh8 as libc::c_int != c_0 as libc::c_int) {
                    break;
                }
                if ptr >= end {
                    return 0i32 as Py_ssize_t;
                }
            }
            if ptr >= end {
                return 0i32 as Py_ssize_t;
            }
            i = 1i32 as Py_ssize_t;
            let mut current_block_72: u64;
            loop {
                if *ptr as libc::c_int == *prefix.offset(i as isize) as Py_UCS2 as libc::c_int {
                    i += 1;
                    if i != prefix_len {
                        ptr = ptr.offset(1);
                        if ptr >= end {
                            return 0i32 as Py_ssize_t;
                        }
                        current_block_72 = 16203797167131938757;
                    } else {
                        (*state).start = ptr.offset(-((prefix_len - 1i32 as libc::c_long) as isize))
                            as *mut libc::c_void;
                        (*state).ptr = ptr
                            .offset(-((prefix_len - prefix_skip - 1i32 as libc::c_long) as isize))
                            as *mut libc::c_void;
                        if flags & 2i32 != 0 {
                            return 1i32 as Py_ssize_t;
                        }
                        status = sre_ucs2_match(
                            state,
                            pattern.offset((2i32 as libc::c_long * prefix_skip) as isize),
                            0i32,
                        );
                        if status != 0i32 as libc::c_long {
                            return status;
                        }
                        ptr = ptr.offset(1);
                        if ptr >= end {
                            return 0i32 as Py_ssize_t;
                        }
                        current_block_72 = 8834769789432328951;
                    }
                } else {
                    current_block_72 = 8834769789432328951;
                }
                match current_block_72 {
                    8834769789432328951 => i = *overlap.offset(i as isize) as Py_ssize_t,
                    _ => {}
                }
                if !(i != 0i32 as libc::c_long) {
                    break;
                }
            }
        }
        return 0i32 as Py_ssize_t;
    }
    if !charset.is_null() {
        end = (*state).end as *mut Py_UCS2;
        loop {
            while ptr < end && sre_ucs2_charset(state, charset, *ptr as Py_UCS4) == 0 {
                ptr = ptr.offset(1)
            }
            if ptr >= end {
                return 0i32 as Py_ssize_t;
            }
            (*state).start = ptr as *mut libc::c_void;
            (*state).ptr = ptr as *mut libc::c_void;
            status = sre_ucs2_match(state, pattern, 0i32);
            if status != 0i32 as libc::c_long {
                break;
            }
            ptr = ptr.offset(1)
        }
    } else {
        if ptr <= end {
        } else {
            __assert_fail(
                b"ptr <= end\x00" as *const u8 as *const libc::c_char,
                b"Modules/sre_lib.h\x00" as *const u8 as *const libc::c_char,
                1350i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 51], &[libc::c_char; 51]>(
                    b"Py_ssize_t sre_ucs2_search(SRE_STATE *, Py_UCS4 *)\x00",
                ))
                .as_ptr(),
            );
        }
        loop {
            (*state).ptr = ptr as *mut libc::c_void;
            (*state).start = (*state).ptr;
            status = sre_ucs2_match(state, pattern, 0i32);
            if status != 0i32 as libc::c_long || ptr >= end {
                break;
            }
            ptr = ptr.offset(1)
        }
    }
    return status;
}
/*[clinic input]
module _sre
class _sre.SRE_Pattern "PatternObject *" "&Pattern_Type"
class _sre.SRE_Match "MatchObject *" "&Match_Type"
class _sre.SRE_Scanner "ScannerObject *" "&Scanner_Type"
[clinic start generated code]*/
/*[clinic end generated code: output=da39a3ee5e6b4b0d input=b0230ec19a0deac8]*/
/*[clinic input]
_sre.getcodesize -> int
[clinic start generated code]*/
unsafe extern "C" fn _sre_getcodesize_impl(mut module: *mut PyObject) -> libc::c_int
/*[clinic end generated code: output=e0db7ce34a6dd7b1 input=bd6f6ecf4916bb2b]*/ {
    return ::std::mem::size_of::<Py_UCS4>() as libc::c_ulong as libc::c_int;
}
/*[clinic input]
_sre.getlower -> int

    character: int
    flags: int
    /

[clinic start generated code]*/
unsafe extern "C" fn _sre_getlower_impl(
    mut module: *mut PyObject,
    mut character: libc::c_int,
    mut flags: libc::c_int,
) -> libc::c_int
/*[clinic end generated code: output=47eebc4c1214feb5 input=087d2f1c44bbca6f]*/ {
    if flags & 4i32 != 0 {
        return sre_lower_locale(character as libc::c_uint) as libc::c_int;
    }
    if flags & 32i32 != 0 {
        return sre_lower_unicode(character as libc::c_uint) as libc::c_int;
    }
    return sre_lower(character as libc::c_uint) as libc::c_int;
}
#[inline]
unsafe extern "C" fn state_reset(mut state: *mut SRE_STATE) {
    /* FIXME: dynamic! */
    /*memset(state->mark, 0, sizeof(*state->mark) * SRE_MARK_SIZE);*/
    (*state).lastmark = -1i32 as Py_ssize_t;
    (*state).lastindex = -1i32 as Py_ssize_t;
    (*state).repeat = 0 as *mut SRE_REPEAT;
    data_stack_dealloc(state);
}
unsafe extern "C" fn getstring(
    mut string: *mut PyObject,
    mut p_length: *mut Py_ssize_t,
    mut p_isbytes: *mut libc::c_int,
    mut p_charsize: *mut libc::c_int,
    mut view: *mut Py_buffer,
) -> *mut libc::c_void {
    /* given a python object, return a data pointer, a length (in
    characters), and a character size.  return NULL if the object
    is not a string (or not compatible) */
    /* Unicode objects do not support the buffer API. So, get the data
    directly instead. */
    if (*(*string).ob_type).tp_flags & 1u64 << 28i32 != 0i32 as libc::c_ulong {
        if (*(*string).ob_type).tp_flags & 1u64 << 28i32 != 0i32 as libc::c_ulong {
        } else {
            __assert_fail(
                b"PyUnicode_Check(string)\x00" as *const u8 as *const libc::c_char,
                b"Modules/_sre.c\x00" as *const u8 as *const libc::c_char,
                330i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                    b"void *getstring(PyObject *, Py_ssize_t *, int *, int *, Py_buffer *)\x00",
                ))
                .as_ptr(),
            );
        }
        if (if (*(string as *mut PyASCIIObject)).state.ready() as libc::c_int != 0 {
            0i32
        } else {
            _PyUnicode_Ready(string)
        }) == -1i32
        {
            return 0 as *mut libc::c_void;
        }
        if (*(*string).ob_type).tp_flags & 1u64 << 28i32 != 0i32 as libc::c_ulong {
        } else {
            __assert_fail(
                b"PyUnicode_Check(string)\x00" as *const u8 as *const libc::c_char,
                b"Modules/_sre.c\x00" as *const u8 as *const libc::c_char,
                332i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                    b"void *getstring(PyObject *, Py_ssize_t *, int *, int *, Py_buffer *)\x00",
                ))
                .as_ptr(),
            );
        }
        if (*(string as *mut PyASCIIObject)).state.ready() != 0 {
        } else {
            __assert_fail(
                b"PyUnicode_IS_READY(string)\x00" as *const u8 as *const libc::c_char,
                b"Modules/_sre.c\x00" as *const u8 as *const libc::c_char,
                332i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                    b"void *getstring(PyObject *, Py_ssize_t *, int *, int *, Py_buffer *)\x00",
                ))
                .as_ptr(),
            );
        }
        *p_length = (*(string as *mut PyASCIIObject)).length;
        if (*(*string).ob_type).tp_flags & 1u64 << 28i32 != 0i32 as libc::c_ulong {
        } else {
            __assert_fail(
                b"PyUnicode_Check(string)\x00" as *const u8 as *const libc::c_char,
                b"Modules/_sre.c\x00" as *const u8 as *const libc::c_char,
                333i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                    b"void *getstring(PyObject *, Py_ssize_t *, int *, int *, Py_buffer *)\x00",
                ))
                .as_ptr(),
            );
        }
        if (*(string as *mut PyASCIIObject)).state.ready() != 0 {
        } else {
            __assert_fail(
                b"PyUnicode_IS_READY(string)\x00" as *const u8 as *const libc::c_char,
                b"Modules/_sre.c\x00" as *const u8 as *const libc::c_char,
                333i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                    b"void *getstring(PyObject *, Py_ssize_t *, int *, int *, Py_buffer *)\x00",
                ))
                .as_ptr(),
            );
        }
        *p_charsize = (*(string as *mut PyASCIIObject)).state.kind() as libc::c_int;
        *p_isbytes = 0i32;
        if (*(*string).ob_type).tp_flags & 1u64 << 28i32 != 0i32 as libc::c_ulong {
        } else {
            __assert_fail(
                b"PyUnicode_Check(string)\x00" as *const u8 as *const libc::c_char,
                b"Modules/_sre.c\x00" as *const u8 as *const libc::c_char,
                335i32 as libc::c_uint,
                (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                    b"void *getstring(PyObject *, Py_ssize_t *, int *, int *, Py_buffer *)\x00",
                ))
                .as_ptr(),
            );
        }
        return (if (*(string as *mut PyASCIIObject)).state.compact() as libc::c_int != 0 {
            if (*(*string).ob_type).tp_flags & 1u64 << 28i32 != 0i32 as libc::c_ulong {
            } else {
                __assert_fail(
                    b"PyUnicode_Check(string)\x00" as *const u8 as *const libc::c_char,
                    b"Modules/_sre.c\x00" as *const u8 as *const libc::c_char,
                    335i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                        b"void *getstring(PyObject *, Py_ssize_t *, int *, int *, Py_buffer *)\x00",
                    ))
                    .as_ptr(),
                );
            }
            if (*(string as *mut PyASCIIObject)).state.ready() != 0 {
            } else {
                __assert_fail(
                    b"PyUnicode_IS_READY(string)\x00" as *const u8 as *const libc::c_char,
                    b"Modules/_sre.c\x00" as *const u8 as *const libc::c_char,
                    335i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                        b"void *getstring(PyObject *, Py_ssize_t *, int *, int *, Py_buffer *)\x00",
                    ))
                    .as_ptr(),
                );
            }
            (if (*(string as *mut PyASCIIObject)).state.ascii() as libc::c_int != 0 {
                (string as *mut PyASCIIObject).offset(1) as *mut libc::c_void
            } else {
                (string as *mut PyCompactUnicodeObject).offset(1) as *mut libc::c_void
            })
        } else {
            if !(*(string as *mut PyUnicodeObject)).data.any.is_null() {
            } else {
                __assert_fail(
                    b"((PyUnicodeObject*)(string))->data.any\x00" as *const u8
                        as *const libc::c_char,
                    b"Modules/_sre.c\x00" as *const u8 as *const libc::c_char,
                    335i32 as libc::c_uint,
                    (*::std::mem::transmute::<&[u8; 69], &[libc::c_char; 69]>(
                        b"void *getstring(PyObject *, Py_ssize_t *, int *, int *, Py_buffer *)\x00",
                    ))
                    .as_ptr(),
                );
            }
            (*(string as *mut PyUnicodeObject)).data.any
        });
    }
    /* get pointer to byte string buffer */
    if PyObject_GetBuffer(string, view, 0i32) != 0i32 {
        PyErr_SetString(
            PyExc_TypeError,
            b"expected string or bytes-like object\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut libc::c_void;
    }
    *p_length = (*view).len;
    *p_charsize = 1i32;
    *p_isbytes = 1i32;
    if (*view).buf.is_null() {
        PyErr_SetString(
            PyExc_ValueError,
            b"Buffer is NULL\x00" as *const u8 as *const libc::c_char,
        );
        PyBuffer_Release(view);
        (*view).buf = 0 as *mut libc::c_void;
        return 0 as *mut libc::c_void;
    }
    return (*view).buf;
}
/* calculate offset from start of string */
#[inline]
unsafe extern "C" fn getslice(
    mut isbytes: libc::c_int,
    mut ptr: *const libc::c_void,
    mut string: *mut PyObject,
    mut start: Py_ssize_t,
    mut end: Py_ssize_t,
) -> *mut PyObject {
    if isbytes != 0 {
        if (*string).ob_type == &mut PyBytes_Type as *mut PyTypeObject
            && start == 0i32 as libc::c_long
            && {
                if (*(*string).ob_type).tp_flags & 1u64 << 27i32 != 0i32 as libc::c_ulong {
                } else {
                    __assert_fail(b"PyBytes_Check(string)\x00" as *const u8
                                         as *const libc::c_char,
                                     b"Modules/_sre.c\x00" as *const u8 as
                                         *const libc::c_char,
                                     460i32 as libc::c_uint,
                                     (*::std::mem::transmute::<&[u8; 74],
                                                               &[libc::c_char; 74]>(b"PyObject *getslice(int, const void *, PyObject *, Py_ssize_t, Py_ssize_t)\x00")).as_ptr());
                }
                end == (*(string as *mut PyVarObject)).ob_size
            }
        {
            (*string).ob_refcnt += 1;
            return string;
        }
        return PyBytes_FromStringAndSize(
            (ptr as *const libc::c_char).offset(start as isize),
            end - start,
        );
    } else {
        return PyUnicode_Substring(string, start, end);
    };
}
unsafe extern "C" fn pattern_error(mut status: Py_ssize_t) {
    match status {
        -3 => {
            /* This error code seems to be unused. */
            PyErr_SetString(
                PyExc_RecursionError,
                b"maximum recursion limit exceeded\x00" as *const u8 as *const libc::c_char,
            );
        }
        -9 => {
            PyErr_NoMemory();
        }
        -10 => {}
        _ => {
            /* other error codes indicate compiler/engine bugs */
            PyErr_SetString(
                PyExc_RuntimeError,
                b"internal error in regular expression engine\x00" as *const u8
                    as *const libc::c_char,
            );
        }
    };
}
#[inline]
unsafe extern "C" fn sre_match(
    mut state: *mut SRE_STATE,
    mut pattern: *mut Py_UCS4,
    mut match_all: libc::c_int,
) -> Py_ssize_t {
    if (*state).charsize == 1i32 {
        return sre_ucs1_match(state, pattern, match_all);
    }
    if (*state).charsize == 2i32 {
        return sre_ucs2_match(state, pattern, match_all);
    }
    if (*state).charsize == 4i32 {
    } else {
        __assert_fail(
            b"state->charsize == 4\x00" as *const u8 as *const libc::c_char,
            b"Modules/_sre.c\x00" as *const u8 as *const libc::c_char,
            539i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 50], &[libc::c_char; 50]>(
                b"Py_ssize_t sre_match(SRE_STATE *, Py_UCS4 *, int)\x00",
            ))
            .as_ptr(),
        );
    }
    return sre_ucs4_match(state, pattern, match_all);
}
#[inline]
unsafe extern "C" fn sre_search(
    mut state: *mut SRE_STATE,
    mut pattern: *mut Py_UCS4,
) -> Py_ssize_t {
    if (*state).charsize == 1i32 {
        return sre_ucs1_search(state, pattern);
    }
    if (*state).charsize == 2i32 {
        return sre_ucs2_search(state, pattern);
    }
    if (*state).charsize == 4i32 {
    } else {
        __assert_fail(
            b"state->charsize == 4\x00" as *const u8 as *const libc::c_char,
            b"Modules/_sre.c\x00" as *const u8 as *const libc::c_char,
            550i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 46], &[libc::c_char; 46]>(
                b"Py_ssize_t sre_search(SRE_STATE *, Py_UCS4 *)\x00",
            ))
            .as_ptr(),
        );
    }
    return sre_ucs4_search(state, pattern);
}
unsafe extern "C" fn call(
    mut module: *const libc::c_char,
    mut function: *const libc::c_char,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut name: *mut PyObject = 0 as *mut PyObject;
    let mut mod_0: *mut PyObject = 0 as *mut PyObject;
    let mut func: *mut PyObject = 0 as *mut PyObject;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    if args.is_null() {
        return 0 as *mut PyObject;
    }
    name = PyUnicode_FromString(module);
    if name.is_null() {
        return 0 as *mut PyObject;
    }
    mod_0 = PyImport_Import(name);
    let mut _py_decref_tmp: *mut PyObject = name;
    (*_py_decref_tmp).ob_refcnt -= 1;
    if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
        (*(*_py_decref_tmp).ob_type)
            .tp_dealloc
            .expect("non-null function pointer")(_py_decref_tmp);
    }
    if mod_0.is_null() {
        return 0 as *mut PyObject;
    }
    func = PyObject_GetAttrString(mod_0, function);
    let mut _py_decref_tmp_0: *mut PyObject = mod_0;
    (*_py_decref_tmp_0).ob_refcnt -= 1;
    if !((*_py_decref_tmp_0).ob_refcnt != 0i32 as libc::c_long) {
        (*(*_py_decref_tmp_0).ob_type)
            .tp_dealloc
            .expect("non-null function pointer")(_py_decref_tmp_0);
    }
    if func.is_null() {
        return 0 as *mut PyObject;
    }
    result = PyObject_CallObject(func, args);
    let mut _py_decref_tmp_1: *mut PyObject = func;
    (*_py_decref_tmp_1).ob_refcnt -= 1;
    if !((*_py_decref_tmp_1).ob_refcnt != 0i32 as libc::c_long) {
        (*(*_py_decref_tmp_1).ob_type)
            .tp_dealloc
            .expect("non-null function pointer")(_py_decref_tmp_1);
    }
    let mut _py_decref_tmp_2: *mut PyObject = args;
    (*_py_decref_tmp_2).ob_refcnt -= 1;
    if !((*_py_decref_tmp_2).ob_refcnt != 0i32 as libc::c_long) {
        (*(*_py_decref_tmp_2).ob_type)
            .tp_dealloc
            .expect("non-null function pointer")(_py_decref_tmp_2);
    }
    return result;
}
/* Forward */
/*[clinic input]
_sre.compile

    pattern: object
    flags: int
    code: object(subclass_of='&PyList_Type')
    groups: Py_ssize_t
    groupindex: object
    indexgroup: object

[clinic start generated code]*/
unsafe extern "C" fn _sre_compile_impl(
    mut module: *mut PyObject,
    mut pattern: *mut PyObject,
    mut flags: libc::c_int,
    mut code: *mut PyObject,
    mut groups: Py_ssize_t,
    mut groupindex: *mut PyObject,
    mut indexgroup: *mut PyObject,
) -> *mut PyObject
/*[clinic end generated code: output=ef9c2b3693776404 input=7d059ec8ae1edb85]*/ {
    /* "compile" pattern descriptor to pattern object */
    let mut self_0: *mut PatternObject = 0 as *mut PatternObject;
    let mut i: Py_ssize_t = 0;
    let mut n: Py_ssize_t = 0;
    n = (*(code as *mut PyVarObject)).ob_size;
    /* coverity[ampersand_in_size] */
    if self_0.is_null() {
        return 0 as *mut PyObject;
    }
    (*self_0).weakreflist = 0 as *mut PyObject;
    (*self_0).pattern = 0 as *mut PyObject;
    (*self_0).groupindex = 0 as *mut PyObject;
    (*self_0).indexgroup = 0 as *mut PyObject;
    (*self_0).codesize = n;
    i = 0i32 as Py_ssize_t;
    while i < n {
        let mut o: *mut PyObject = *(*(code as *mut PyListObject)).ob_item.offset(i as isize);
        let mut value: libc::c_ulong = PyLong_AsUnsignedLong(o);
        *(*self_0).code.as_mut_ptr().offset(i as isize) = value as Py_UCS4;
        if *(*self_0).code.as_mut_ptr().offset(i as isize) as libc::c_ulong != value {
            PyErr_SetString(
                PyExc_OverflowError,
                b"regular expression code size limit exceeded\x00" as *const u8
                    as *const libc::c_char,
            );
            break;
        } else {
            i += 1
        }
    }
    if !PyErr_Occurred().is_null() {
        let mut _py_decref_tmp: *mut PyObject = self_0 as *mut PyObject;
        (*_py_decref_tmp).ob_refcnt -= 1;
        if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
            (*(*_py_decref_tmp).ob_type)
                .tp_dealloc
                .expect("non-null function pointer")(_py_decref_tmp);
        }
        return 0 as *mut PyObject;
    }
    if pattern == &mut _Py_NoneStruct as *mut PyObject {
        (*self_0).isbytes = -1i32
    } else {
        let mut p_length: Py_ssize_t = 0;
        let mut charsize: libc::c_int = 0;
        let mut view: Py_buffer = Py_buffer {
            buf: 0 as *mut libc::c_void,
            obj: 0 as *mut PyObject,
            len: 0,
            itemsize: 0,
            readonly: 0,
            ndim: 0,
            format: 0 as *mut libc::c_char,
            shape: 0 as *mut Py_ssize_t,
            strides: 0 as *mut Py_ssize_t,
            suboffsets: 0 as *mut Py_ssize_t,
            internal: 0 as *mut libc::c_void,
        };
        view.buf = 0 as *mut libc::c_void;
        if getstring(
            pattern,
            &mut p_length,
            &mut (*self_0).isbytes,
            &mut charsize,
            &mut view,
        )
        .is_null()
        {
            let mut _py_decref_tmp_0: *mut PyObject = self_0 as *mut PyObject;
            (*_py_decref_tmp_0).ob_refcnt -= 1;
            if !((*_py_decref_tmp_0).ob_refcnt != 0i32 as libc::c_long) {
                (*(*_py_decref_tmp_0).ob_type)
                    .tp_dealloc
                    .expect("non-null function pointer")(_py_decref_tmp_0);
            }
            return 0 as *mut PyObject;
        }
        if !view.buf.is_null() {
            PyBuffer_Release(&mut view);
        }
    }
    (*pattern).ob_refcnt += 1;
    (*self_0).pattern = pattern;
    (*self_0).flags = flags;
    (*self_0).groups = groups;
    (*groupindex).ob_refcnt += 1;
    (*self_0).groupindex = groupindex;
    (*indexgroup).ob_refcnt += 1;
    (*self_0).indexgroup = indexgroup;
    if _validate(self_0) == 0 {
        let mut _py_decref_tmp_1: *mut PyObject = self_0 as *mut PyObject;
        (*_py_decref_tmp_1).ob_refcnt -= 1;
        if !((*_py_decref_tmp_1).ob_refcnt != 0i32 as libc::c_long) {
            (*(*_py_decref_tmp_1).ob_type)
                .tp_dealloc
                .expect("non-null function pointer")(_py_decref_tmp_1);
        }
        return 0 as *mut PyObject;
    }
    return self_0 as *mut PyObject;
}
/* -------------------------------------------------------------------- */
/* Code validation */
/* To learn more about this code, have a look at the _compile() function in
   Lib/sre_compile.py.  The validation functions below checks the code array
   for conformance with the code patterns generated there.

   The nice thing about the generated code is that it is position-independent:
   all jumps are relative jumps forward.  Also, jumps don't cross each other:
   the target of a later jump is always earlier than the target of an earlier
   jump.  IOW, this is okay:

   J---------J-------T--------T
    \         \_____/        /
     \______________________/

   but this is not:

   J---------J-------T--------T
    \_________\_____/        /
               \____________/

   It also helps that SRE_CODE is always an unsigned type.
*/
/* Defining this one enables tracing of the validator */
/* Trace macro for the validator */
/* do nothing */
/* Report failure */
/* Extract opcode, argument, or skip count from code array */
unsafe extern "C" fn _validate_charset(
    mut code: *mut Py_UCS4,
    mut end: *mut Py_UCS4,
) -> libc::c_int {
    /* Some variables are manipulated by the macros above */
    let mut op: Py_UCS4 = 0; /* 256-bit bitmap */
    let mut arg: Py_UCS4 = 0;
    let mut offset: Py_UCS4 = 0;
    let mut i: libc::c_int = 0;
    while code < end {
        if code >= end {
            return 0i32;
        }
        let fresh9 = code;
        code = code.offset(1);
        op = *fresh9;
        match op {
            26 => {}
            19 => {
                if code >= end {
                    return 0i32;
                }
                let fresh10 = code;
                code = code.offset(1);
                arg = *fresh10
            }
            27 | 32 => {
                if code >= end {
                    return 0i32;
                }
                let fresh11 = code;
                code = code.offset(1);
                arg = *fresh11;
                if code >= end {
                    return 0i32;
                }
                let fresh12 = code;
                code = code.offset(1);
                arg = *fresh12
            }
            10 => {
                offset = (256i32 as libc::c_ulong).wrapping_div(
                    (8i32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong),
                ) as Py_UCS4;
                if offset as libc::c_ulong
                    > end.wrapping_offset_from(code) as libc::c_long as uintptr_t
                {
                    return 0i32;
                }
                code = code.offset(offset as isize)
            }
            11 => {
                /* Number of blocks */
                if code >= end {
                    return 0i32;
                } /* 256-byte table */
                let fresh13 = code;
                code = code.offset(1);
                arg = *fresh13;
                offset = (256i32 as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong)
                    as Py_UCS4;
                if offset as libc::c_ulong
                    > end.wrapping_offset_from(code) as libc::c_long as uintptr_t
                {
                    return 0i32;
                }
                /* Make sure that each byte points to a valid block */
                i = 0i32; /* 256-bit bitmap times arg */
                while i < 256i32 {
                    if *(code as *mut libc::c_uchar).offset(i as isize) as libc::c_uint >= arg {
                        return 0i32;
                    }
                    i += 1
                }
                code = code.offset(offset as isize);
                offset = (arg as libc::c_ulong).wrapping_mul(
                    (256i32 as libc::c_ulong).wrapping_div(
                        (8i32 as libc::c_ulong)
                            .wrapping_mul(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong),
                    ),
                ) as Py_UCS4;
                if offset as libc::c_ulong
                    > end.wrapping_offset_from(code) as libc::c_long as uintptr_t
                {
                    return 0i32;
                }
                code = code.offset(offset as isize)
            }
            9 => {
                if code >= end {
                    return 0i32;
                }
                let fresh14 = code;
                code = code.offset(1);
                arg = *fresh14;
                match arg {
                    0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16
                    | 17 => {}
                    _ => return 0i32,
                }
            }
            _ => return 0i32,
        }
    }
    return 1i32;
}
unsafe extern "C" fn _validate(mut self_0: *mut PatternObject) -> libc::c_int {
    if _validate_outer(
        (*self_0).code.as_mut_ptr(),
        (*self_0)
            .code
            .as_mut_ptr()
            .offset((*self_0).codesize as isize),
        (*self_0).groups,
    ) == 0
    {
        PyErr_SetString(
            PyExc_RuntimeError,
            b"invalid SRE code\x00" as *const u8 as *const libc::c_char,
        );
        return 0i32;
    }
    return 1i32;
}
/* -------------------------------------------------------------------- */
/* match methods */
unsafe extern "C" fn match_dealloc(mut self_0: *mut MatchObject) {
    let mut _py_xdecref_tmp: *mut PyObject = (*self_0).regs;
    if !_py_xdecref_tmp.is_null() {
        let mut _py_decref_tmp: *mut PyObject = _py_xdecref_tmp;
        (*_py_decref_tmp).ob_refcnt -= 1;
        if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
            (*(*_py_decref_tmp).ob_type)
                .tp_dealloc
                .expect("non-null function pointer")(_py_decref_tmp);
        }
    }
    let mut _py_xdecref_tmp_0: *mut PyObject = (*self_0).string;
    if !_py_xdecref_tmp_0.is_null() {
        let mut _py_decref_tmp_0: *mut PyObject = _py_xdecref_tmp_0;
        (*_py_decref_tmp_0).ob_refcnt -= 1;
        if !((*_py_decref_tmp_0).ob_refcnt != 0i32 as libc::c_long) {
            (*(*_py_decref_tmp_0).ob_type)
                .tp_dealloc
                .expect("non-null function pointer")(_py_decref_tmp_0);
        }
    }
    let mut _py_decref_tmp_1: *mut PyObject = (*self_0).pattern as *mut PyObject;
    (*_py_decref_tmp_1).ob_refcnt -= 1;
    if !((*_py_decref_tmp_1).ob_refcnt != 0i32 as libc::c_long) {
        (*(*_py_decref_tmp_1).ob_type)
            .tp_dealloc
            .expect("non-null function pointer")(_py_decref_tmp_1);
    }
    PyObject_Free(self_0 as *mut libc::c_void);
}
unsafe extern "C" fn match_getslice_by_index(
    mut self_0: *mut MatchObject,
    mut index: Py_ssize_t,
    mut def: *mut PyObject,
) -> *mut PyObject {
    let mut length: Py_ssize_t = 0;
    let mut isbytes: libc::c_int = 0;
    let mut charsize: libc::c_int = 0;
    let mut view: Py_buffer = Py_buffer {
        buf: 0 as *mut libc::c_void,
        obj: 0 as *mut PyObject,
        len: 0,
        itemsize: 0,
        readonly: 0,
        ndim: 0,
        format: 0 as *mut libc::c_char,
        shape: 0 as *mut Py_ssize_t,
        strides: 0 as *mut Py_ssize_t,
        suboffsets: 0 as *mut Py_ssize_t,
        internal: 0 as *mut libc::c_void,
    };
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if index < 0i32 as libc::c_long || index >= (*self_0).groups {
        /* raise IndexError if we were given a bad group number */
        PyErr_SetString(
            PyExc_IndexError,
            b"no such group\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    }
    index *= 2i32 as libc::c_long;
    if (*self_0).string == &mut _Py_NoneStruct as *mut PyObject
        || *(*self_0).mark.as_mut_ptr().offset(index as isize) < 0i32 as libc::c_long
    {
        /* return default value if the string or group is undefined */
        (*def).ob_refcnt += 1;
        return def;
    }
    ptr = getstring(
        (*self_0).string,
        &mut length,
        &mut isbytes,
        &mut charsize,
        &mut view,
    );
    if ptr.is_null() {
        return 0 as *mut PyObject;
    }
    result = getslice(
        isbytes,
        ptr,
        (*self_0).string,
        *(*self_0).mark.as_mut_ptr().offset(index as isize),
        *(*self_0)
            .mark
            .as_mut_ptr()
            .offset((index + 1i32 as libc::c_long) as isize),
    );
    if isbytes != 0 && !view.buf.is_null() {
        PyBuffer_Release(&mut view);
    }
    return result;
}
unsafe extern "C" fn match_getindex(
    mut self_0: *mut MatchObject,
    mut index: *mut PyObject,
) -> Py_ssize_t {
    let mut i: Py_ssize_t = 0;
    if index.is_null() {
        /* Default value */
        return 0i32 as Py_ssize_t;
    }
    if !(*(*index).ob_type).tp_as_number.is_null()
        && (*(*(*index).ob_type).tp_as_number).nb_index.is_some()
    {
        return PyNumber_AsSsize_t(index, 0 as *mut PyObject);
    }
    i = -1i32 as Py_ssize_t;
    if !(*(*self_0).pattern).groupindex.is_null() {
        index = PyObject_GetItem((*(*self_0).pattern).groupindex, index);
        if !index.is_null() {
            if (*(*index).ob_type).tp_flags & 1u64 << 24i32 != 0i32 as libc::c_ulong {
                i = PyLong_AsSsize_t(index)
            }
            let mut _py_decref_tmp: *mut PyObject = index;
            (*_py_decref_tmp).ob_refcnt -= 1;
            if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
                (*(*_py_decref_tmp).ob_type)
                    .tp_dealloc
                    .expect("non-null function pointer")(_py_decref_tmp);
            }
        } else {
            PyErr_Clear();
        }
    }
    return i;
}
unsafe extern "C" fn match_getslice(
    mut self_0: *mut MatchObject,
    mut index: *mut PyObject,
    mut def: *mut PyObject,
) -> *mut PyObject {
    return match_getslice_by_index(self_0, match_getindex(self_0, index), def);
}
/*[clinic input]
_sre.SRE_Match.expand

    template: object

Return the string obtained by doing backslash substitution on the string template, as done by the sub() method.
[clinic start generated code]*/
unsafe extern "C" fn _sre_SRE_Match_expand_impl(
    mut self_0: *mut MatchObject,
    mut template: *mut PyObject,
) -> *mut PyObject
/*[clinic end generated code: output=931b58ccc323c3a1 input=4bfdb22c2f8b146a]*/ {
    /* delegate to Python code */
    return call(
        b"re\x00" as *const u8 as *const libc::c_char,
        b"_expand\x00" as *const u8 as *const libc::c_char,
        PyTuple_Pack(3i32 as Py_ssize_t, (*self_0).pattern, self_0, template),
    );
}
unsafe extern "C" fn match_group(
    mut self_0: *mut MatchObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut i: Py_ssize_t = 0;
    let mut size: Py_ssize_t = 0;
    size = (*(args as *mut PyVarObject)).ob_size;
    match size {
        0 => {
            result = match_getslice(
                self_0,
                &mut _Py_FalseStruct as *mut _longobject as *mut PyObject,
                &mut _Py_NoneStruct,
            )
        }
        1 => {
            result = match_getslice(
                self_0,
                *(*(args as *mut PyTupleObject))
                    .ob_item
                    .as_mut_ptr()
                    .offset(0),
                &mut _Py_NoneStruct,
            )
        }
        _ => {
            /* fetch multiple items */
            result = PyTuple_New(size);
            if result.is_null() {
                return 0 as *mut PyObject;
            }
            i = 0i32 as Py_ssize_t;
            while i < size {
                let mut item: *mut PyObject = match_getslice(
                    self_0,
                    *(*(args as *mut PyTupleObject))
                        .ob_item
                        .as_mut_ptr()
                        .offset(i as isize),
                    &mut _Py_NoneStruct,
                );
                if item.is_null() {
                    let mut _py_decref_tmp: *mut PyObject = result;
                    (*_py_decref_tmp).ob_refcnt -= 1;
                    if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
                        (*(*_py_decref_tmp).ob_type)
                            .tp_dealloc
                            .expect("non-null function pointer")(
                            _py_decref_tmp
                        );
                    }
                    return 0 as *mut PyObject;
                }
                let ref mut fresh17 = *(*(result as *mut PyTupleObject))
                    .ob_item
                    .as_mut_ptr()
                    .offset(i as isize);
                *fresh17 = item;
                i += 1
            }
        }
    }
    return result;
}
unsafe extern "C" fn match_getitem(
    mut self_0: *mut MatchObject,
    mut name: *mut PyObject,
) -> *mut PyObject {
    return match_getslice(self_0, name, &mut _Py_NoneStruct);
}
/*[clinic input]
_sre.SRE_Match.groups

    default: object = None
        Is used for groups that did not participate in the match.

Return a tuple containing all the subgroups of the match, from 1.
[clinic start generated code]*/
unsafe extern "C" fn _sre_SRE_Match_groups_impl(
    mut self_0: *mut MatchObject,
    mut default_value: *mut PyObject,
) -> *mut PyObject
/*[clinic end generated code: output=daf8e2641537238a input=bb069ef55dabca91]*/ {
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut index: Py_ssize_t = 0;
    result = PyTuple_New((*self_0).groups - 1i32 as libc::c_long);
    if result.is_null() {
        return 0 as *mut PyObject;
    }
    index = 1i32 as Py_ssize_t;
    while index < (*self_0).groups {
        let mut item: *mut PyObject = 0 as *mut PyObject;
        item = match_getslice_by_index(self_0, index, default_value);
        if item.is_null() {
            let mut _py_decref_tmp: *mut PyObject = result;
            (*_py_decref_tmp).ob_refcnt -= 1;
            if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
                (*(*_py_decref_tmp).ob_type)
                    .tp_dealloc
                    .expect("non-null function pointer")(_py_decref_tmp);
            }
            return 0 as *mut PyObject;
        }
        let ref mut fresh18 = *(*(result as *mut PyTupleObject))
            .ob_item
            .as_mut_ptr()
            .offset((index - 1i32 as libc::c_long) as isize);
        *fresh18 = item;
        index += 1
    }
    return result;
}
/*[clinic input]
_sre.SRE_Match.groupdict

    default: object = None
        Is used for groups that did not participate in the match.

Return a dictionary containing all the named subgroups of the match, keyed by the subgroup name.
[clinic start generated code]*/
unsafe extern "C" fn _sre_SRE_Match_groupdict_impl(
    mut self_0: *mut MatchObject,
    mut default_value: *mut PyObject,
) -> *mut PyObject
/*[clinic end generated code: output=29917c9073e41757 input=0ded7960b23780aa]*/ {
    let mut current_block: u64;
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut keys: *mut PyObject = 0 as *mut PyObject;
    let mut index: Py_ssize_t = 0;
    result = PyDict_New();
    if result.is_null() || (*(*self_0).pattern).groupindex.is_null() {
        return result;
    }
    keys = PyMapping_Keys((*(*self_0).pattern).groupindex);
    if !keys.is_null() {
        index = 0i32 as Py_ssize_t;
        loop {
            if !(index < (*(keys as *mut PyVarObject)).ob_size) {
                current_block = 2668756484064249700;
                break;
            }
            let mut status: libc::c_int = 0;
            let mut key: *mut PyObject = 0 as *mut PyObject;
            let mut value: *mut PyObject = 0 as *mut PyObject;
            key = *(*(keys as *mut PyListObject))
                .ob_item
                .offset(index as isize);
            if key.is_null() {
                current_block = 12107152634974002367;
                break;
            }
            value = match_getslice(self_0, key, default_value);
            if value.is_null() {
                current_block = 12107152634974002367;
                break;
            }
            status = PyDict_SetItem(result, key, value);
            let mut _py_decref_tmp: *mut PyObject = value;
            (*_py_decref_tmp).ob_refcnt -= 1;
            if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
                (*(*_py_decref_tmp).ob_type)
                    .tp_dealloc
                    .expect("non-null function pointer")(_py_decref_tmp);
            }
            if status < 0i32 {
                current_block = 12107152634974002367;
                break;
            }
            index += 1
        }
        match current_block {
            12107152634974002367 => {}
            _ => {
                let mut _py_decref_tmp_0: *mut PyObject = keys;
                (*_py_decref_tmp_0).ob_refcnt -= 1;
                if !((*_py_decref_tmp_0).ob_refcnt != 0i32 as libc::c_long) {
                    (*(*_py_decref_tmp_0).ob_type)
                        .tp_dealloc
                        .expect("non-null function pointer")(_py_decref_tmp_0);
                }
                return result;
            }
        }
    }
    let mut _py_xdecref_tmp: *mut PyObject = keys;
    if !_py_xdecref_tmp.is_null() {
        let mut _py_decref_tmp_1: *mut PyObject = _py_xdecref_tmp;
        (*_py_decref_tmp_1).ob_refcnt -= 1;
        if !((*_py_decref_tmp_1).ob_refcnt != 0i32 as libc::c_long) {
            (*(*_py_decref_tmp_1).ob_type)
                .tp_dealloc
                .expect("non-null function pointer")(_py_decref_tmp_1);
        }
    }
    let mut _py_decref_tmp_2: *mut PyObject = result;
    (*_py_decref_tmp_2).ob_refcnt -= 1;
    if !((*_py_decref_tmp_2).ob_refcnt != 0i32 as libc::c_long) {
        (*(*_py_decref_tmp_2).ob_type)
            .tp_dealloc
            .expect("non-null function pointer")(_py_decref_tmp_2);
    }
    return 0 as *mut PyObject;
}
/*[clinic input]
_sre.SRE_Match.start -> Py_ssize_t

    group: object(c_default="NULL") = 0
    /

Return index of the start of the substring matched by group.
[clinic start generated code]*/
unsafe extern "C" fn _sre_SRE_Match_start_impl(
    mut self_0: *mut MatchObject,
    mut group: *mut PyObject,
) -> Py_ssize_t
/*[clinic end generated code: output=3f6e7f9df2fb5201 input=ced8e4ed4b33ee6c]*/ {
    let mut index: Py_ssize_t = match_getindex(self_0, group);
    if index < 0i32 as libc::c_long || index >= (*self_0).groups {
        PyErr_SetString(
            PyExc_IndexError,
            b"no such group\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32 as Py_ssize_t;
    }
    /* mark is -1 if group is undefined */
    return *(*self_0)
        .mark
        .as_mut_ptr()
        .offset((index * 2i32 as libc::c_long) as isize);
}
/*[clinic input]
_sre.SRE_Match.end -> Py_ssize_t

    group: object(c_default="NULL") = 0
    /

Return index of the end of the substring matched by group.
[clinic start generated code]*/
unsafe extern "C" fn _sre_SRE_Match_end_impl(
    mut self_0: *mut MatchObject,
    mut group: *mut PyObject,
) -> Py_ssize_t
/*[clinic end generated code: output=f4240b09911f7692 input=1b799560c7f3d7e6]*/ {
    let mut index: Py_ssize_t = match_getindex(self_0, group);
    if index < 0i32 as libc::c_long || index >= (*self_0).groups {
        PyErr_SetString(
            PyExc_IndexError,
            b"no such group\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32 as Py_ssize_t;
    }
    /* mark is -1 if group is undefined */
    return *(*self_0)
        .mark
        .as_mut_ptr()
        .offset((index * 2i32 as libc::c_long + 1i32 as libc::c_long) as isize);
}
#[inline]
unsafe extern "C" fn _pair(mut i1: Py_ssize_t, mut i2: Py_ssize_t) -> *mut PyObject {
    let mut pair: *mut PyObject = 0 as *mut PyObject;
    let mut item: *mut PyObject = 0 as *mut PyObject;
    pair = PyTuple_New(2i32 as Py_ssize_t);
    if pair.is_null() {
        return 0 as *mut PyObject;
    }
    item = PyLong_FromSsize_t(i1);
    if !item.is_null() {
        let ref mut fresh19 = *(*(pair as *mut PyTupleObject))
            .ob_item
            .as_mut_ptr()
            .offset(0);
        *fresh19 = item;
        item = PyLong_FromSsize_t(i2);
        if !item.is_null() {
            let ref mut fresh20 = *(*(pair as *mut PyTupleObject))
                .ob_item
                .as_mut_ptr()
                .offset(1);
            *fresh20 = item;
            return pair;
        }
    }
    let mut _py_decref_tmp: *mut PyObject = pair;
    (*_py_decref_tmp).ob_refcnt -= 1;
    if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
        (*(*_py_decref_tmp).ob_type)
            .tp_dealloc
            .expect("non-null function pointer")(_py_decref_tmp);
    }
    return 0 as *mut PyObject;
}
/*[clinic input]
_sre.SRE_Match.span

    group: object(c_default="NULL") = 0
    /

For MatchObject m, return the 2-tuple (m.start(group), m.end(group)).
[clinic start generated code]*/
unsafe extern "C" fn _sre_SRE_Match_span_impl(
    mut self_0: *mut MatchObject,
    mut group: *mut PyObject,
) -> *mut PyObject
/*[clinic end generated code: output=f02ae40594d14fe6 input=49092b6008d176d3]*/ {
    let mut index: Py_ssize_t = match_getindex(self_0, group);
    if index < 0i32 as libc::c_long || index >= (*self_0).groups {
        PyErr_SetString(
            PyExc_IndexError,
            b"no such group\x00" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut PyObject;
    }
    /* marks are -1 if group is undefined */
    return _pair(
        *(*self_0)
            .mark
            .as_mut_ptr()
            .offset((index * 2i32 as libc::c_long) as isize),
        *(*self_0)
            .mark
            .as_mut_ptr()
            .offset((index * 2i32 as libc::c_long + 1i32 as libc::c_long) as isize),
    );
}
unsafe extern "C" fn match_regs(mut self_0: *mut MatchObject) -> *mut PyObject {
    let mut regs: *mut PyObject = 0 as *mut PyObject;
    let mut item: *mut PyObject = 0 as *mut PyObject;
    let mut index: Py_ssize_t = 0;
    regs = PyTuple_New((*self_0).groups);
    if regs.is_null() {
        return 0 as *mut PyObject;
    }
    index = 0i32 as Py_ssize_t;
    while index < (*self_0).groups {
        item = _pair(
            *(*self_0)
                .mark
                .as_mut_ptr()
                .offset((index * 2i32 as libc::c_long) as isize),
            *(*self_0)
                .mark
                .as_mut_ptr()
                .offset((index * 2i32 as libc::c_long + 1i32 as libc::c_long) as isize),
        );
        if item.is_null() {
            let mut _py_decref_tmp: *mut PyObject = regs;
            (*_py_decref_tmp).ob_refcnt -= 1;
            if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
                (*(*_py_decref_tmp).ob_type)
                    .tp_dealloc
                    .expect("non-null function pointer")(_py_decref_tmp);
            }
            return 0 as *mut PyObject;
        }
        let ref mut fresh21 = *(*(regs as *mut PyTupleObject))
            .ob_item
            .as_mut_ptr()
            .offset(index as isize);
        *fresh21 = item;
        index += 1
    }
    (*regs).ob_refcnt += 1;
    (*self_0).regs = regs;
    return regs;
}
/*[clinic input]
_sre.SRE_Match.__copy__

[clinic start generated code]*/
unsafe extern "C" fn _sre_SRE_Match___copy___impl(mut self_0: *mut MatchObject) -> *mut PyObject
/*[clinic end generated code: output=a779c5fc8b5b4eb4 input=3bb4d30b6baddb5b]*/ {
    PyErr_SetString(
        PyExc_TypeError,
        b"cannot copy this match object\x00" as *const u8 as *const libc::c_char,
    );
    return 0 as *mut PyObject;
}
/*[clinic input]
_sre.SRE_Match.__deepcopy__

    memo: object

[clinic start generated code]*/
unsafe extern "C" fn _sre_SRE_Match___deepcopy___impl(
    mut self_0: *mut MatchObject,
    mut memo: *mut PyObject,
) -> *mut PyObject
/*[clinic end generated code: output=2b657578eb03f4a3 input=b65b72489eac64cc]*/ {
    PyErr_SetString(
        PyExc_TypeError,
        b"cannot deepcopy this match object\x00" as *const u8 as *const libc::c_char,
    );
    return 0 as *mut PyObject;
}
static mut match_doc: [libc::c_char; 93] = [
    84, 104, 101, 32, 114, 101, 115, 117, 108, 116, 32, 111, 102, 32, 114, 101, 46, 109, 97, 116,
    99, 104, 40, 41, 32, 97, 110, 100, 32, 114, 101, 46, 115, 101, 97, 114, 99, 104, 40, 41, 46,
    10, 77, 97, 116, 99, 104, 32, 111, 98, 106, 101, 99, 116, 115, 32, 97, 108, 119, 97, 121, 115,
    32, 104, 97, 118, 101, 32, 97, 32, 98, 111, 111, 108, 101, 97, 110, 32, 118, 97, 108, 117, 101,
    32, 111, 102, 32, 84, 114, 117, 101, 46, 0,
];
static mut match_group_doc: [libc::c_char; 131] = [
    103, 114, 111, 117, 112, 40, 91, 103, 114, 111, 117, 112, 49, 44, 32, 46, 46, 46, 93, 41, 32,
    45, 62, 32, 115, 116, 114, 32, 111, 114, 32, 116, 117, 112, 108, 101, 46, 10, 32, 32, 32, 32,
    82, 101, 116, 117, 114, 110, 32, 115, 117, 98, 103, 114, 111, 117, 112, 40, 115, 41, 32, 111,
    102, 32, 116, 104, 101, 32, 109, 97, 116, 99, 104, 32, 98, 121, 32, 105, 110, 100, 105, 99,
    101, 115, 32, 111, 114, 32, 110, 97, 109, 101, 115, 46, 10, 32, 32, 32, 32, 70, 111, 114, 32,
    48, 32, 114, 101, 116, 117, 114, 110, 115, 32, 116, 104, 101, 32, 101, 110, 116, 105, 114, 101,
    32, 109, 97, 116, 99, 104, 46, 0,
];
unsafe extern "C" fn match_lastindex_get(mut self_0: *mut MatchObject) -> *mut PyObject {
    if (*self_0).lastindex >= 0i32 as libc::c_long {
        return PyLong_FromSsize_t((*self_0).lastindex);
    }
    let ref mut fresh22 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh22 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn match_lastgroup_get(mut self_0: *mut MatchObject) -> *mut PyObject {
    if !(*(*self_0).pattern).indexgroup.is_null() && (*self_0).lastindex >= 0i32 as libc::c_long {
        let mut result: *mut PyObject =
            PySequence_GetItem((*(*self_0).pattern).indexgroup, (*self_0).lastindex);
        if !result.is_null() {
            return result;
        }
        PyErr_Clear();
    }
    let ref mut fresh23 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh23 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn match_regs_get(mut self_0: *mut MatchObject) -> *mut PyObject {
    if !(*self_0).regs.is_null() {
        (*(*self_0).regs).ob_refcnt += 1;
        return (*self_0).regs;
    } else {
        return match_regs(self_0);
    };
}
unsafe extern "C" fn match_repr(mut self_0: *mut MatchObject) -> *mut PyObject {
    let mut result: *mut PyObject = 0 as *mut PyObject;
    let mut group0: *mut PyObject =
        match_getslice_by_index(self_0, 0i32 as Py_ssize_t, &mut _Py_NoneStruct);
    if group0.is_null() {
        return 0 as *mut PyObject;
    }
    result = PyUnicode_FromFormat(
        b"<%s object; span=(%d, %d), match=%.50R>\x00" as *const u8 as *const libc::c_char,
        (*(*(self_0 as *mut PyObject)).ob_type).tp_name,
        *(*self_0).mark.as_mut_ptr().offset(0),
        *(*self_0).mark.as_mut_ptr().offset(1),
        group0,
    );
    let mut _py_decref_tmp: *mut PyObject = group0;
    (*_py_decref_tmp).ob_refcnt -= 1;
    if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
        (*(*_py_decref_tmp).ob_type)
            .tp_dealloc
            .expect("non-null function pointer")(_py_decref_tmp);
    }
    return result;
}
/* generate 8-bit version */
/* generate 16-bit unicode version */
/* generate 32-bit unicode version */
/* -------------------------------------------------------------------- */
/* factories and destructors */
/* see sre.h for object declarations */
unsafe extern "C" fn pattern_new_match(
    mut pattern: *mut PatternObject,
    mut state: *mut SRE_STATE,
    mut status: Py_ssize_t,
) -> *mut PyObject {
    /* create match object (from state object) */
    let mut match_0: *mut MatchObject = 0 as *mut MatchObject;
    let mut i: Py_ssize_t = 0;
    let mut j: Py_ssize_t = 0;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: libc::c_int = 0;
    if status > 0i32 as libc::c_long {
        /* create match object (with room for extra group marks) */
        /* coverity[ampersand_in_size] */
        if match_0.is_null() {
            return 0 as *mut PyObject;
        }
        let ref mut fresh24 = (*(pattern as *mut PyObject)).ob_refcnt;
        *fresh24 += 1;
        (*match_0).pattern = pattern;
        (*(*state).string).ob_refcnt += 1;
        (*match_0).string = (*state).string;
        (*match_0).regs = 0 as *mut PyObject;
        (*match_0).groups = (*pattern).groups + 1i32 as libc::c_long;
        /* fill in group slices */
        base = (*state).beginning as *mut libc::c_char; /* undefined */
        n = (*state).charsize;
        *(*match_0).mark.as_mut_ptr().offset(0) =
            ((*state).start as *mut libc::c_char).wrapping_offset_from(base) as libc::c_long
                / n as libc::c_long;
        *(*match_0).mark.as_mut_ptr().offset(1) =
            ((*state).ptr as *mut libc::c_char).wrapping_offset_from(base) as libc::c_long
                / n as libc::c_long;
        j = 0i32 as Py_ssize_t;
        i = j;
        while i < (*pattern).groups {
            if j + 1i32 as libc::c_long <= (*state).lastmark
                && !(*(*state).mark.offset(j as isize)).is_null()
                && !(*(*state).mark.offset((j + 1i32 as libc::c_long) as isize)).is_null()
            {
                *(*match_0)
                    .mark
                    .as_mut_ptr()
                    .offset((j + 2i32 as libc::c_long) as isize) =
                    (*(*state).mark.offset(j as isize) as *mut libc::c_char)
                        .wrapping_offset_from(base) as libc::c_long
                        / n as libc::c_long;
                *(*match_0)
                    .mark
                    .as_mut_ptr()
                    .offset((j + 3i32 as libc::c_long) as isize) =
                    (*(*state).mark.offset((j + 1i32 as libc::c_long) as isize)
                        as *mut libc::c_char)
                        .wrapping_offset_from(base) as libc::c_long
                        / n as libc::c_long
            } else {
                let ref mut fresh25 = *(*match_0)
                    .mark
                    .as_mut_ptr()
                    .offset((j + 3i32 as libc::c_long) as isize);
                *fresh25 = -1i32 as Py_ssize_t;
                *(*match_0)
                    .mark
                    .as_mut_ptr()
                    .offset((j + 2i32 as libc::c_long) as isize) = *fresh25
            }
            i += 1;
            j += 2i32 as libc::c_long
        }
        (*match_0).pos = (*state).pos;
        (*match_0).endpos = (*state).endpos;
        (*match_0).lastindex = (*state).lastindex;
        return match_0 as *mut PyObject;
    } else {
        if status == 0i32 as libc::c_long {
            /* no match */
            let ref mut fresh26 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
            *fresh26 += 1;
            return &mut _Py_NoneStruct;
        }
    }
    /* internal error */
    pattern_error(status);
    return 0 as *mut PyObject;
}
/* -------------------------------------------------------------------- */
/* scanner methods (experimental) */
unsafe extern "C" fn scanner_dealloc(mut self_0: *mut ScannerObject) {
    let mut _py_xdecref_tmp: *mut PyObject = (*self_0).pattern;
    if !_py_xdecref_tmp.is_null() {
        let mut _py_decref_tmp: *mut PyObject = _py_xdecref_tmp;
        (*_py_decref_tmp).ob_refcnt -= 1;
        if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
            (*(*_py_decref_tmp).ob_type)
                .tp_dealloc
                .expect("non-null function pointer")(_py_decref_tmp);
        }
    }
    PyObject_Free(self_0 as *mut libc::c_void);
}
/*[clinic input]
_sre.SRE_Scanner.match

[clinic start generated code]*/
unsafe extern "C" fn _sre_SRE_Scanner_match_impl(mut self_0: *mut ScannerObject) -> *mut PyObject
/*[clinic end generated code: output=936b30c63d4b81eb input=881a0154f8c13d9a]*/ {
    let mut state: *mut SRE_STATE = 0 as *mut SRE_STATE;
    let mut match_0: *mut PyObject = 0 as *mut PyObject;
    let mut status: Py_ssize_t = 0;
    if (*state).start.is_null() {
        let ref mut fresh27 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh27 += 1;
        return &mut _Py_NoneStruct;
    }
    state_reset(state);
    (*state).ptr = (*state).start;
    status = sre_match(
        state,
        (*((*self_0).pattern as *mut PatternObject))
            .code
            .as_mut_ptr(),
        0i32,
    );
    if !PyErr_Occurred().is_null() {
        return 0 as *mut PyObject;
    }
    match_0 = pattern_new_match((*self_0).pattern as *mut PatternObject, state, status);
    if status == 0i32 as libc::c_long {
        (*state).start = 0 as *mut libc::c_void
    } else if (*state).ptr != (*state).start {
        (*state).start = (*state).ptr
    } else if (*state).ptr != (*state).end {
        (*state).start = ((*state).ptr as *mut libc::c_char).offset((*state).charsize as isize)
            as *mut libc::c_void
    } else {
        (*state).start = 0 as *mut libc::c_void
    }
    return match_0;
}
/*[clinic input]
_sre.SRE_Scanner.search

[clinic start generated code]*/
unsafe extern "C" fn _sre_SRE_Scanner_search_impl(mut self_0: *mut ScannerObject) -> *mut PyObject
/*[clinic end generated code: output=7dc211986088f025 input=161223ee92ef9270]*/ {
    let mut state: *mut SRE_STATE = 0 as *mut SRE_STATE;
    let mut match_0: *mut PyObject = 0 as *mut PyObject;
    let mut status: Py_ssize_t = 0;
    if (*state).start.is_null() {
        let ref mut fresh28 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
        *fresh28 += 1;
        return &mut _Py_NoneStruct;
    }
    state_reset(state);
    (*state).ptr = (*state).start;
    status = sre_search(
        state,
        (*((*self_0).pattern as *mut PatternObject))
            .code
            .as_mut_ptr(),
    );
    if !PyErr_Occurred().is_null() {
        return 0 as *mut PyObject;
    }
    match_0 = pattern_new_match((*self_0).pattern as *mut PatternObject, state, status);
    if status == 0i32 as libc::c_long {
        (*state).start = 0 as *mut libc::c_void
    } else if (*state).ptr != (*state).start {
        (*state).start = (*state).ptr
    } else if (*state).ptr != (*state).end {
        (*state).start = ((*state).ptr as *mut libc::c_char).offset((*state).charsize as isize)
            as *mut libc::c_void
    } else {
        (*state).start = 0 as *mut libc::c_void
    }
    return match_0;
}
/*[clinic input]
preserve
[clinic start generated code]*/
static mut _sre_getcodesize__doc__: [libc::c_char; 29] = [
    103, 101, 116, 99, 111, 100, 101, 115, 105, 122, 101, 40, 36, 109, 111, 100, 117, 108, 101, 44,
    32, 47, 41, 10, 45, 45, 10, 10, 0,
];
unsafe extern "C" fn _sre_getcodesize(
    mut module: *mut PyObject,
    mut _unused_ignored: *mut PyObject,
) -> *mut PyObject {
    let mut return_value: *mut PyObject = 0 as *mut PyObject;
    let mut _return_value: libc::c_int = 0;
    _return_value = _sre_getcodesize_impl(module);
    if !(_return_value == -1i32 && !PyErr_Occurred().is_null()) {
        return_value = PyLong_FromLong(_return_value as libc::c_long)
    }
    return return_value;
}
static mut _sre_getlower__doc__: [libc::c_char; 44] = [
    103, 101, 116, 108, 111, 119, 101, 114, 40, 36, 109, 111, 100, 117, 108, 101, 44, 32, 99, 104,
    97, 114, 97, 99, 116, 101, 114, 44, 32, 102, 108, 97, 103, 115, 44, 32, 47, 41, 10, 45, 45, 10,
    10, 0,
];
unsafe extern "C" fn _sre_getlower(
    mut module: *mut PyObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut return_value: *mut PyObject = 0 as *mut PyObject;
    let mut character: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut _return_value: libc::c_int = 0;
    if !(_PyArg_ParseTuple_SizeT(
        args,
        b"ii:getlower\x00" as *const u8 as *const libc::c_char,
        &mut character as *mut libc::c_int,
        &mut flags as *mut libc::c_int,
    ) == 0)
    {
        _return_value = _sre_getlower_impl(module, character, flags);
        if !(_return_value == -1i32 && !PyErr_Occurred().is_null()) {
            return_value = PyLong_FromLong(_return_value as libc::c_long)
        }
    }
    return return_value;
}
static mut _sre_compile__doc__: [libc::c_char; 87] = [
    99, 111, 109, 112, 105, 108, 101, 40, 36, 109, 111, 100, 117, 108, 101, 44, 32, 47, 44, 32,
    112, 97, 116, 116, 101, 114, 110, 44, 32, 102, 108, 97, 103, 115, 44, 32, 99, 111, 100, 101,
    44, 32, 103, 114, 111, 117, 112, 115, 44, 32, 103, 114, 111, 117, 112, 105, 110, 100, 101, 120,
    44, 10, 32, 32, 32, 32, 32, 32, 32, 32, 105, 110, 100, 101, 120, 103, 114, 111, 117, 112, 41,
    10, 45, 45, 10, 10, 0,
];
unsafe extern "C" fn _sre_compile(
    mut module: *mut PyObject,
    mut args: *mut *mut PyObject,
    mut nargs: Py_ssize_t,
    mut kwnames: *mut PyObject,
) -> *mut PyObject {
    let mut return_value: *mut PyObject = 0 as *mut PyObject;
    static mut _keywords: [*const libc::c_char; 7] = [
        b"pattern\x00" as *const u8 as *const libc::c_char,
        b"flags\x00" as *const u8 as *const libc::c_char,
        b"code\x00" as *const u8 as *const libc::c_char,
        b"groups\x00" as *const u8 as *const libc::c_char,
        b"groupindex\x00" as *const u8 as *const libc::c_char,
        b"indexgroup\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut _parser: _PyArg_Parser = unsafe {
        {
            let mut init = _PyArg_Parser {
                format: b"OiO!nOO:compile\x00" as *const u8 as *const libc::c_char,
                keywords: _keywords.as_ptr(),
                fname: 0 as *const libc::c_char,
                custom_msg: 0 as *const libc::c_char,
                pos: 0,
                min: 0,
                max: 0,
                kwtuple: 0 as *const PyObject as *mut PyObject,
                next: 0 as *const _PyArg_Parser as *mut _PyArg_Parser,
            };
            init
        }
    };
    let mut pattern: *mut PyObject = 0 as *mut PyObject;
    let mut flags: libc::c_int = 0;
    let mut code: *mut PyObject = 0 as *mut PyObject;
    let mut groups: Py_ssize_t = 0;
    let mut groupindex: *mut PyObject = 0 as *mut PyObject;
    let mut indexgroup: *mut PyObject = 0 as *mut PyObject;
    if !(_PyArg_ParseStack_SizeT(
        args,
        nargs,
        kwnames,
        &mut _parser as *mut _PyArg_Parser,
        &mut pattern as *mut *mut PyObject,
        &mut flags as *mut libc::c_int,
        &mut PyList_Type as *mut PyTypeObject,
        &mut code as *mut *mut PyObject,
        &mut groups as *mut Py_ssize_t,
        &mut groupindex as *mut *mut PyObject,
        &mut indexgroup as *mut *mut PyObject,
    ) == 0)
    {
        return_value =
            _sre_compile_impl(module, pattern, flags, code, groups, groupindex, indexgroup)
    }
    return return_value;
}
static mut _sre_SRE_Match_expand__doc__: [libc::c_char; 143] = [
    101, 120, 112, 97, 110, 100, 40, 36, 115, 101, 108, 102, 44, 32, 47, 44, 32, 116, 101, 109,
    112, 108, 97, 116, 101, 41, 10, 45, 45, 10, 10, 82, 101, 116, 117, 114, 110, 32, 116, 104, 101,
    32, 115, 116, 114, 105, 110, 103, 32, 111, 98, 116, 97, 105, 110, 101, 100, 32, 98, 121, 32,
    100, 111, 105, 110, 103, 32, 98, 97, 99, 107, 115, 108, 97, 115, 104, 32, 115, 117, 98, 115,
    116, 105, 116, 117, 116, 105, 111, 110, 32, 111, 110, 32, 116, 104, 101, 32, 115, 116, 114,
    105, 110, 103, 32, 116, 101, 109, 112, 108, 97, 116, 101, 44, 32, 97, 115, 32, 100, 111, 110,
    101, 32, 98, 121, 32, 116, 104, 101, 32, 115, 117, 98, 40, 41, 32, 109, 101, 116, 104, 111,
    100, 46, 0,
];
unsafe extern "C" fn _sre_SRE_Match_expand(
    mut self_0: *mut MatchObject,
    mut args: *mut *mut PyObject,
    mut nargs: Py_ssize_t,
    mut kwnames: *mut PyObject,
) -> *mut PyObject {
    let mut return_value: *mut PyObject = 0 as *mut PyObject;
    static mut _keywords: [*const libc::c_char; 2] = [
        b"template\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut _parser: _PyArg_Parser = unsafe {
        {
            let mut init = _PyArg_Parser {
                format: b"O:expand\x00" as *const u8 as *const libc::c_char,
                keywords: _keywords.as_ptr(),
                fname: 0 as *const libc::c_char,
                custom_msg: 0 as *const libc::c_char,
                pos: 0,
                min: 0,
                max: 0,
                kwtuple: 0 as *const PyObject as *mut PyObject,
                next: 0 as *const _PyArg_Parser as *mut _PyArg_Parser,
            };
            init
        }
    };
    let mut template: *mut PyObject = 0 as *mut PyObject;
    if !(_PyArg_ParseStack_SizeT(
        args,
        nargs,
        kwnames,
        &mut _parser as *mut _PyArg_Parser,
        &mut template as *mut *mut PyObject,
    ) == 0)
    {
        return_value = _sre_SRE_Match_expand_impl(self_0, template)
    }
    return return_value;
}
static mut _sre_SRE_Match_groups__doc__: [libc::c_char; 174] = [
    103, 114, 111, 117, 112, 115, 40, 36, 115, 101, 108, 102, 44, 32, 47, 44, 32, 100, 101, 102,
    97, 117, 108, 116, 61, 78, 111, 110, 101, 41, 10, 45, 45, 10, 10, 82, 101, 116, 117, 114, 110,
    32, 97, 32, 116, 117, 112, 108, 101, 32, 99, 111, 110, 116, 97, 105, 110, 105, 110, 103, 32,
    97, 108, 108, 32, 116, 104, 101, 32, 115, 117, 98, 103, 114, 111, 117, 112, 115, 32, 111, 102,
    32, 116, 104, 101, 32, 109, 97, 116, 99, 104, 44, 32, 102, 114, 111, 109, 32, 49, 46, 10, 10,
    32, 32, 100, 101, 102, 97, 117, 108, 116, 10, 32, 32, 32, 32, 73, 115, 32, 117, 115, 101, 100,
    32, 102, 111, 114, 32, 103, 114, 111, 117, 112, 115, 32, 116, 104, 97, 116, 32, 100, 105, 100,
    32, 110, 111, 116, 32, 112, 97, 114, 116, 105, 99, 105, 112, 97, 116, 101, 32, 105, 110, 32,
    116, 104, 101, 32, 109, 97, 116, 99, 104, 46, 0,
];
unsafe extern "C" fn _sre_SRE_Match_groups(
    mut self_0: *mut MatchObject,
    mut args: *mut *mut PyObject,
    mut nargs: Py_ssize_t,
    mut kwnames: *mut PyObject,
) -> *mut PyObject {
    let mut return_value: *mut PyObject = 0 as *mut PyObject;
    static mut _keywords: [*const libc::c_char; 2] = [
        b"default\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut _parser: _PyArg_Parser = unsafe {
        {
            let mut init = _PyArg_Parser {
                format: b"|O:groups\x00" as *const u8 as *const libc::c_char,
                keywords: _keywords.as_ptr(),
                fname: 0 as *const libc::c_char,
                custom_msg: 0 as *const libc::c_char,
                pos: 0,
                min: 0,
                max: 0,
                kwtuple: 0 as *const PyObject as *mut PyObject,
                next: 0 as *const _PyArg_Parser as *mut _PyArg_Parser,
            };
            init
        }
    };
    let mut default_value: *mut PyObject = &mut _Py_NoneStruct;
    if !(_PyArg_ParseStack_SizeT(
        args,
        nargs,
        kwnames,
        &mut _parser as *mut _PyArg_Parser,
        &mut default_value as *mut *mut PyObject,
    ) == 0)
    {
        return_value = _sre_SRE_Match_groups_impl(self_0, default_value)
    }
    return return_value;
}
static mut _sre_SRE_Match_groupdict__doc__: [libc::c_char; 208] = [
    103, 114, 111, 117, 112, 100, 105, 99, 116, 40, 36, 115, 101, 108, 102, 44, 32, 47, 44, 32,
    100, 101, 102, 97, 117, 108, 116, 61, 78, 111, 110, 101, 41, 10, 45, 45, 10, 10, 82, 101, 116,
    117, 114, 110, 32, 97, 32, 100, 105, 99, 116, 105, 111, 110, 97, 114, 121, 32, 99, 111, 110,
    116, 97, 105, 110, 105, 110, 103, 32, 97, 108, 108, 32, 116, 104, 101, 32, 110, 97, 109, 101,
    100, 32, 115, 117, 98, 103, 114, 111, 117, 112, 115, 32, 111, 102, 32, 116, 104, 101, 32, 109,
    97, 116, 99, 104, 44, 32, 107, 101, 121, 101, 100, 32, 98, 121, 32, 116, 104, 101, 32, 115,
    117, 98, 103, 114, 111, 117, 112, 32, 110, 97, 109, 101, 46, 10, 10, 32, 32, 100, 101, 102, 97,
    117, 108, 116, 10, 32, 32, 32, 32, 73, 115, 32, 117, 115, 101, 100, 32, 102, 111, 114, 32, 103,
    114, 111, 117, 112, 115, 32, 116, 104, 97, 116, 32, 100, 105, 100, 32, 110, 111, 116, 32, 112,
    97, 114, 116, 105, 99, 105, 112, 97, 116, 101, 32, 105, 110, 32, 116, 104, 101, 32, 109, 97,
    116, 99, 104, 46, 0,
];
unsafe extern "C" fn _sre_SRE_Match_groupdict(
    mut self_0: *mut MatchObject,
    mut args: *mut *mut PyObject,
    mut nargs: Py_ssize_t,
    mut kwnames: *mut PyObject,
) -> *mut PyObject {
    let mut return_value: *mut PyObject = 0 as *mut PyObject;
    static mut _keywords: [*const libc::c_char; 2] = [
        b"default\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut _parser: _PyArg_Parser = unsafe {
        {
            let mut init = _PyArg_Parser {
                format: b"|O:groupdict\x00" as *const u8 as *const libc::c_char,
                keywords: _keywords.as_ptr(),
                fname: 0 as *const libc::c_char,
                custom_msg: 0 as *const libc::c_char,
                pos: 0,
                min: 0,
                max: 0,
                kwtuple: 0 as *const PyObject as *mut PyObject,
                next: 0 as *const _PyArg_Parser as *mut _PyArg_Parser,
            };
            init
        }
    };
    let mut default_value: *mut PyObject = &mut _Py_NoneStruct;
    if !(_PyArg_ParseStack_SizeT(
        args,
        nargs,
        kwnames,
        &mut _parser as *mut _PyArg_Parser,
        &mut default_value as *mut *mut PyObject,
    ) == 0)
    {
        return_value = _sre_SRE_Match_groupdict_impl(self_0, default_value)
    }
    return return_value;
}
static mut _sre_SRE_Match_start__doc__: [libc::c_char; 90] = [
    115, 116, 97, 114, 116, 40, 36, 115, 101, 108, 102, 44, 32, 103, 114, 111, 117, 112, 61, 48,
    44, 32, 47, 41, 10, 45, 45, 10, 10, 82, 101, 116, 117, 114, 110, 32, 105, 110, 100, 101, 120,
    32, 111, 102, 32, 116, 104, 101, 32, 115, 116, 97, 114, 116, 32, 111, 102, 32, 116, 104, 101,
    32, 115, 117, 98, 115, 116, 114, 105, 110, 103, 32, 109, 97, 116, 99, 104, 101, 100, 32, 98,
    121, 32, 103, 114, 111, 117, 112, 46, 0,
];
unsafe extern "C" fn _sre_SRE_Match_start(
    mut self_0: *mut MatchObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut return_value: *mut PyObject = 0 as *mut PyObject;
    let mut group: *mut PyObject = 0 as *mut PyObject;
    let mut _return_value: Py_ssize_t = 0;
    if !(PyArg_UnpackTuple(
        args,
        b"start\x00" as *const u8 as *const libc::c_char,
        0i32 as Py_ssize_t,
        1i32 as Py_ssize_t,
        &mut group as *mut *mut PyObject,
    ) == 0)
    {
        _return_value = _sre_SRE_Match_start_impl(self_0, group);
        if !(_return_value == -1i32 as libc::c_long && !PyErr_Occurred().is_null()) {
            return_value = PyLong_FromSsize_t(_return_value)
        }
    }
    return return_value;
}
static mut _sre_SRE_Match_end__doc__: [libc::c_char; 86] = [
    101, 110, 100, 40, 36, 115, 101, 108, 102, 44, 32, 103, 114, 111, 117, 112, 61, 48, 44, 32, 47,
    41, 10, 45, 45, 10, 10, 82, 101, 116, 117, 114, 110, 32, 105, 110, 100, 101, 120, 32, 111, 102,
    32, 116, 104, 101, 32, 101, 110, 100, 32, 111, 102, 32, 116, 104, 101, 32, 115, 117, 98, 115,
    116, 114, 105, 110, 103, 32, 109, 97, 116, 99, 104, 101, 100, 32, 98, 121, 32, 103, 114, 111,
    117, 112, 46, 0,
];
unsafe extern "C" fn _sre_SRE_Match_end(
    mut self_0: *mut MatchObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut return_value: *mut PyObject = 0 as *mut PyObject;
    let mut group: *mut PyObject = 0 as *mut PyObject;
    let mut _return_value: Py_ssize_t = 0;
    if !(PyArg_UnpackTuple(
        args,
        b"end\x00" as *const u8 as *const libc::c_char,
        0i32 as Py_ssize_t,
        1i32 as Py_ssize_t,
        &mut group as *mut *mut PyObject,
    ) == 0)
    {
        _return_value = _sre_SRE_Match_end_impl(self_0, group);
        if !(_return_value == -1i32 as libc::c_long && !PyErr_Occurred().is_null()) {
            return_value = PyLong_FromSsize_t(_return_value)
        }
    }
    return return_value;
}
static mut _sre_SRE_Match_span__doc__: [libc::c_char; 98] = [
    115, 112, 97, 110, 40, 36, 115, 101, 108, 102, 44, 32, 103, 114, 111, 117, 112, 61, 48, 44, 32,
    47, 41, 10, 45, 45, 10, 10, 70, 111, 114, 32, 77, 97, 116, 99, 104, 79, 98, 106, 101, 99, 116,
    32, 109, 44, 32, 114, 101, 116, 117, 114, 110, 32, 116, 104, 101, 32, 50, 45, 116, 117, 112,
    108, 101, 32, 40, 109, 46, 115, 116, 97, 114, 116, 40, 103, 114, 111, 117, 112, 41, 44, 32,
    109, 46, 101, 110, 100, 40, 103, 114, 111, 117, 112, 41, 41, 46, 0,
];
unsafe extern "C" fn _sre_SRE_Match_span(
    mut self_0: *mut MatchObject,
    mut args: *mut PyObject,
) -> *mut PyObject {
    let mut return_value: *mut PyObject = 0 as *mut PyObject;
    let mut group: *mut PyObject = 0 as *mut PyObject;
    if !(PyArg_UnpackTuple(
        args,
        b"span\x00" as *const u8 as *const libc::c_char,
        0i32 as Py_ssize_t,
        1i32 as Py_ssize_t,
        &mut group as *mut *mut PyObject,
    ) == 0)
    {
        return_value = _sre_SRE_Match_span_impl(self_0, group)
    }
    return return_value;
}
static mut _sre_SRE_Match___copy____doc__: [libc::c_char; 24] = [
    95, 95, 99, 111, 112, 121, 95, 95, 40, 36, 115, 101, 108, 102, 44, 32, 47, 41, 10, 45, 45, 10,
    10, 0,
];
unsafe extern "C" fn _sre_SRE_Match___copy__(
    mut self_0: *mut MatchObject,
    mut _unused_ignored: *mut PyObject,
) -> *mut PyObject {
    return _sre_SRE_Match___copy___impl(self_0);
}
static mut _sre_SRE_Match___deepcopy____doc__: [libc::c_char; 34] = [
    95, 95, 100, 101, 101, 112, 99, 111, 112, 121, 95, 95, 40, 36, 115, 101, 108, 102, 44, 32, 47,
    44, 32, 109, 101, 109, 111, 41, 10, 45, 45, 10, 10, 0,
];
unsafe extern "C" fn _sre_SRE_Match___deepcopy__(
    mut self_0: *mut MatchObject,
    mut args: *mut *mut PyObject,
    mut nargs: Py_ssize_t,
    mut kwnames: *mut PyObject,
) -> *mut PyObject {
    let mut return_value: *mut PyObject = 0 as *mut PyObject;
    static mut _keywords: [*const libc::c_char; 2] = [
        b"memo\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    static mut _parser: _PyArg_Parser = unsafe {
        {
            let mut init = _PyArg_Parser {
                format: b"O:__deepcopy__\x00" as *const u8 as *const libc::c_char,
                keywords: _keywords.as_ptr(),
                fname: 0 as *const libc::c_char,
                custom_msg: 0 as *const libc::c_char,
                pos: 0,
                min: 0,
                max: 0,
                kwtuple: 0 as *const PyObject as *mut PyObject,
                next: 0 as *const _PyArg_Parser as *mut _PyArg_Parser,
            };
            init
        }
    };
    let mut memo: *mut PyObject = 0 as *mut PyObject;
    if !(_PyArg_ParseStack_SizeT(
        args,
        nargs,
        kwnames,
        &mut _parser as *mut _PyArg_Parser,
        &mut memo as *mut *mut PyObject,
    ) == 0)
    {
        return_value = _sre_SRE_Match___deepcopy___impl(self_0, memo)
    }
    return return_value;
}
static mut _sre_SRE_Scanner_match__doc__: [libc::c_char; 21] = [
    109, 97, 116, 99, 104, 40, 36, 115, 101, 108, 102, 44, 32, 47, 41, 10, 45, 45, 10, 10, 0,
];
unsafe extern "C" fn _sre_SRE_Scanner_match(
    mut self_0: *mut ScannerObject,
    mut _unused_ignored: *mut PyObject,
) -> *mut PyObject {
    return _sre_SRE_Scanner_match_impl(self_0);
}
static mut _sre_SRE_Scanner_search__doc__: [libc::c_char; 22] = [
    115, 101, 97, 114, 99, 104, 40, 36, 115, 101, 108, 102, 44, 32, 47, 41, 10, 45, 45, 10, 10, 0,
];
unsafe extern "C" fn _sre_SRE_Scanner_search(
    mut self_0: *mut ScannerObject,
    mut _unused_ignored: *mut PyObject,
) -> *mut PyObject {
    return _sre_SRE_Scanner_search_impl(self_0);
}
/* Sentinel */
static mut Pattern_Type: PyTypeObject = PyTypeObject {
    ob_base: PyVarObject {
        ob_base: PyObject {
            ob_refcnt: 0,
            ob_type: 0 as *const _typeobject as *mut _typeobject,
        },
        ob_size: 0,
    },
    tp_name: 0 as *const libc::c_char,
    tp_basicsize: 0,
    tp_itemsize: 0,
    tp_dealloc: None,
    tp_print: None,
    tp_getattr: None,
    tp_setattr: None,
    tp_as_async: 0 as *const PyAsyncMethods as *mut PyAsyncMethods,
    tp_repr: None,
    tp_as_number: 0 as *const PyNumberMethods as *mut PyNumberMethods,
    tp_as_sequence: 0 as *const PySequenceMethods as *mut PySequenceMethods,
    tp_as_mapping: 0 as *const PyMappingMethods as *mut PyMappingMethods,
    tp_hash: None,
    tp_call: None,
    tp_str: None,
    tp_getattro: None,
    tp_setattro: None,
    tp_as_buffer: 0 as *const PyBufferProcs as *mut PyBufferProcs,
    tp_flags: 0,
    tp_doc: 0 as *const libc::c_char,
    tp_traverse: None,
    tp_clear: None,
    tp_richcompare: None,
    tp_weaklistoffset: 0,
    tp_iter: None,
    tp_iternext: None,
    tp_methods: 0 as *const PyMethodDef as *mut PyMethodDef,
    tp_members: 0 as *const PyMemberDef as *mut PyMemberDef,
    tp_getset: 0 as *const PyGetSetDef as *mut PyGetSetDef,
    tp_base: 0 as *const _typeobject as *mut _typeobject,
    tp_dict: 0 as *const PyObject as *mut PyObject,
    tp_descr_get: None,
    tp_descr_set: None,
    tp_dictoffset: 0,
    tp_init: None,
    tp_alloc: None,
    tp_new: None,
    tp_free: None,
    tp_is_gc: None,
    tp_bases: 0 as *const PyObject as *mut PyObject,
    tp_mro: 0 as *const PyObject as *mut PyObject,
    tp_cache: 0 as *const PyObject as *mut PyObject,
    tp_subclasses: 0 as *const PyObject as *mut PyObject,
    tp_weaklist: 0 as *const PyObject as *mut PyObject,
    tp_del: None,
    tp_version_tag: 0,
    tp_finalize: None,
};
/* tp_dealloc */
/* tp_print */
/* tp_getattr */
/* tp_setattr */
/* tp_reserved */
/* tp_repr */
/* tp_as_number */
/* tp_as_sequence */
/* tp_as_mapping */
/* tp_hash */
/* tp_call */
/* tp_str */
/* tp_getattro */
/* tp_setattro */
/* tp_as_buffer */
/* tp_flags */
/* tp_doc */
/* tp_traverse */
/* tp_clear */
/* tp_richcompare */
/* tp_weaklistoffset */
/* tp_iter */
/* tp_iternext */
/* tp_methods */
/* tp_members */
/* tp_getset */
/* Match objects do not support length or assignment, but do support
__getitem__. */
static mut match_as_mapping: PyMappingMethods = unsafe {
    {
        let mut init = PyMappingMethods {
            mp_length: None,
            mp_subscript: ::std::mem::transmute::<
                Option<
                    unsafe extern "C" fn(_: *mut MatchObject, _: *mut PyObject) -> *mut PyObject,
                >,
                binaryfunc,
            >(Some(
                match_getitem
                    as unsafe extern "C" fn(_: *mut MatchObject, _: *mut PyObject) -> *mut PyObject,
            )),
            mp_ass_subscript: None,
        };
        init
    }
};
static mut match_methods: [PyMethodDef; 10] = unsafe {
    [
        {
            let mut init = PyMethodDef {
                ml_name: b"group\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    match_group
                        as unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x1i32,
                ml_doc: match_group_doc.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"start\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_SRE_Match_start
                        as unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x1i32,
                ml_doc: _sre_SRE_Match_start__doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"end\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_SRE_Match_end
                        as unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x1i32,
                ml_doc: _sre_SRE_Match_end__doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"span\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_SRE_Match_span
                        as unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x1i32,
                ml_doc: _sre_SRE_Match_span__doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"groups\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut *mut PyObject,
                            _: Py_ssize_t,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_SRE_Match_groups
                        as unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut *mut PyObject,
                            _: Py_ssize_t,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x80i32,
                ml_doc: _sre_SRE_Match_groups__doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"groupdict\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut *mut PyObject,
                            _: Py_ssize_t,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_SRE_Match_groupdict
                        as unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut *mut PyObject,
                            _: Py_ssize_t,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x80i32,
                ml_doc: _sre_SRE_Match_groupdict__doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"expand\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut *mut PyObject,
                            _: Py_ssize_t,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_SRE_Match_expand
                        as unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut *mut PyObject,
                            _: Py_ssize_t,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x80i32,
                ml_doc: _sre_SRE_Match_expand__doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"__copy__\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_SRE_Match___copy__
                        as unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x4i32,
                ml_doc: _sre_SRE_Match___copy____doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"__deepcopy__\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut *mut PyObject,
                            _: Py_ssize_t,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_SRE_Match___deepcopy__
                        as unsafe extern "C" fn(
                            _: *mut MatchObject,
                            _: *mut *mut PyObject,
                            _: Py_ssize_t,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x80i32,
                ml_doc: _sre_SRE_Match___deepcopy____doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: 0 as *const libc::c_char,
                ml_meth: None,
                ml_flags: 0,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
    ]
};
static mut match_getset: [PyGetSetDef; 4] = unsafe {
    [
        {
            let mut init = PyGetSetDef {
                name: b"lastindex\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(_: *mut MatchObject) -> *mut PyObject>,
                    getter,
                >(Some(
                    match_lastindex_get
                        as unsafe extern "C" fn(_: *mut MatchObject) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<*mut libc::c_void, setter>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: b"lastgroup\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(_: *mut MatchObject) -> *mut PyObject>,
                    getter,
                >(Some(
                    match_lastgroup_get
                        as unsafe extern "C" fn(_: *mut MatchObject) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<*mut libc::c_void, setter>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: b"regs\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(_: *mut MatchObject) -> *mut PyObject>,
                    getter,
                >(Some(
                    match_regs_get as unsafe extern "C" fn(_: *mut MatchObject) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<*mut libc::c_void, setter>(
                    0 as *const libc::c_void as *mut libc::c_void,
                ),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                get: None,
                set: None,
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
static mut match_members: [PyMemberDef; 0] = [];
/* FIXME: implement setattr("string", None) as a special case (to
detach the associated string, if any */
static mut Match_Type: PyTypeObject = unsafe {
    {
        let mut init = _typeobject {
            ob_base: {
                let mut init = PyVarObject {
                    ob_base: {
                        let mut init = _object {
                            ob_refcnt: 1i32 as Py_ssize_t,
                            ob_type: 0 as *const _typeobject as *mut _typeobject,
                        };
                        init
                    },
                    ob_size: 0i32 as Py_ssize_t,
                };
                init
            },
            tp_name: b"_sre.SRE_Match\x00" as *const u8 as *const libc::c_char,
            tp_basicsize: ::std::mem::size_of::<MatchObject>() as libc::c_ulong as Py_ssize_t,
            tp_itemsize: ::std::mem::size_of::<Py_ssize_t>() as libc::c_ulong as Py_ssize_t,
            tp_dealloc: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut MatchObject) -> ()>,
                destructor,
            >(Some(
                match_dealloc as unsafe extern "C" fn(_: *mut MatchObject) -> (),
            )),
            tp_print: None,
            tp_getattr: None,
            tp_setattr: None,
            tp_as_async: 0 as *const PyAsyncMethods as *mut PyAsyncMethods,
            tp_repr: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut MatchObject) -> *mut PyObject>,
                reprfunc,
            >(Some(
                match_repr as unsafe extern "C" fn(_: *mut MatchObject) -> *mut PyObject,
            )),
            tp_as_number: 0 as *const PyNumberMethods as *mut PyNumberMethods,
            tp_as_sequence: 0 as *const PySequenceMethods as *mut PySequenceMethods,
            tp_as_mapping: &match_as_mapping as *const PyMappingMethods as *mut PyMappingMethods,
            tp_hash: None,
            tp_call: None,
            tp_str: None,
            tp_getattro: None,
            tp_setattro: None,
            tp_as_buffer: 0 as *const PyBufferProcs as *mut PyBufferProcs,
            tp_flags: 0i32 as libc::c_ulong | 1u64 << 18i32 | 0i32 as libc::c_ulong,
            tp_doc: match_doc.as_ptr() as *mut _,
            tp_traverse: None,
            tp_clear: None,
            tp_richcompare: None,
            tp_weaklistoffset: 0i32 as Py_ssize_t,
            tp_iter: None,
            tp_iternext: None,
            tp_methods: match_methods.as_ptr() as *mut _,
            tp_members: match_members.as_ptr() as *mut _,
            tp_getset: match_getset.as_ptr() as *mut _,
            tp_base: 0 as *const _typeobject as *mut _typeobject,
            tp_dict: 0 as *const PyObject as *mut PyObject,
            tp_descr_get: None,
            tp_descr_set: None,
            tp_dictoffset: 0,
            tp_init: None,
            tp_alloc: None,
            tp_new: None,
            tp_free: None,
            tp_is_gc: None,
            tp_bases: 0 as *const PyObject as *mut PyObject,
            tp_mro: 0 as *const PyObject as *mut PyObject,
            tp_cache: 0 as *const PyObject as *mut PyObject,
            tp_subclasses: 0 as *const PyObject as *mut PyObject,
            tp_weaklist: 0 as *const PyObject as *mut PyObject,
            tp_del: None,
            tp_version_tag: 0,
            tp_finalize: None,
        };
        init
    }
};
static mut scanner_methods: [PyMethodDef; 3] = unsafe {
    [
        {
            let mut init = PyMethodDef {
                ml_name: b"match\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut ScannerObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_SRE_Scanner_match
                        as unsafe extern "C" fn(
                            _: *mut ScannerObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x4i32,
                ml_doc: _sre_SRE_Scanner_match__doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"search\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut ScannerObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_SRE_Scanner_search
                        as unsafe extern "C" fn(
                            _: *mut ScannerObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x4i32,
                ml_doc: _sre_SRE_Scanner_search__doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: 0 as *const libc::c_char,
                ml_meth: None,
                ml_flags: 0,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
    ]
};
static mut scanner_members: [PyMemberDef; 0] = [];
/* Sentinel */
static mut Scanner_Type: PyTypeObject = unsafe {
    {
        let mut init = _typeobject {
            ob_base: {
                let mut init = PyVarObject {
                    ob_base: {
                        let mut init = _object {
                            ob_refcnt: 1i32 as Py_ssize_t,
                            ob_type: 0 as *const _typeobject as *mut _typeobject,
                        };
                        init
                    },
                    ob_size: 0i32 as Py_ssize_t,
                };
                init
            },
            tp_name: b"_sre.SRE_Scanner\x00" as *const u8 as *const libc::c_char,
            tp_basicsize: ::std::mem::size_of::<ScannerObject>() as libc::c_ulong as Py_ssize_t,
            tp_itemsize: 0i32 as Py_ssize_t,
            tp_dealloc: ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut ScannerObject) -> ()>,
                destructor,
            >(Some(
                scanner_dealloc as unsafe extern "C" fn(_: *mut ScannerObject) -> (),
            )),
            tp_print: None,
            tp_getattr: None,
            tp_setattr: None,
            tp_as_async: 0 as *const PyAsyncMethods as *mut PyAsyncMethods,
            tp_repr: None,
            tp_as_number: 0 as *const PyNumberMethods as *mut PyNumberMethods,
            tp_as_sequence: 0 as *const PySequenceMethods as *mut PySequenceMethods,
            tp_as_mapping: 0 as *const PyMappingMethods as *mut PyMappingMethods,
            tp_hash: None,
            tp_call: None,
            tp_str: None,
            tp_getattro: None,
            tp_setattro: None,
            tp_as_buffer: 0 as *const PyBufferProcs as *mut PyBufferProcs,
            tp_flags: 0i32 as libc::c_ulong | 1u64 << 18i32 | 0i32 as libc::c_ulong,
            tp_doc: 0 as *const libc::c_char,
            tp_traverse: None,
            tp_clear: None,
            tp_richcompare: None,
            tp_weaklistoffset: 0i32 as Py_ssize_t,
            tp_iter: None,
            tp_iternext: None,
            tp_methods: scanner_methods.as_ptr() as *mut _,
            tp_members: scanner_members.as_ptr() as *mut _,
            tp_getset: 0 as *const PyGetSetDef as *mut PyGetSetDef,
            tp_base: 0 as *const _typeobject as *mut _typeobject,
            tp_dict: 0 as *const PyObject as *mut PyObject,
            tp_descr_get: None,
            tp_descr_set: None,
            tp_dictoffset: 0,
            tp_init: None,
            tp_alloc: None,
            tp_new: None,
            tp_free: None,
            tp_is_gc: None,
            tp_bases: 0 as *const PyObject as *mut PyObject,
            tp_mro: 0 as *const PyObject as *mut PyObject,
            tp_cache: 0 as *const PyObject as *mut PyObject,
            tp_subclasses: 0 as *const PyObject as *mut PyObject,
            tp_weaklist: 0 as *const PyObject as *mut PyObject,
            tp_del: None,
            tp_version_tag: 0,
            tp_finalize: None,
        };
        init
    }
};
static mut _functions: [PyMethodDef; 4] = unsafe {
    [
        {
            let mut init = PyMethodDef {
                ml_name: b"compile\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            _: *mut PyObject,
                            _: *mut *mut PyObject,
                            _: Py_ssize_t,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_compile
                        as unsafe extern "C" fn(
                            _: *mut PyObject,
                            _: *mut *mut PyObject,
                            _: Py_ssize_t,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x80i32,
                ml_doc: _sre_compile__doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"getcodesize\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_getcodesize
                        as unsafe extern "C" fn(
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x4i32,
                ml_doc: _sre_getcodesize__doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: b"getlower\x00" as *const u8 as *const libc::c_char,
                ml_meth: ::std::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject) -> *mut PyObject,
                    >,
                    PyCFunction,
                >(Some(
                    _sre_getlower
                        as unsafe extern "C" fn(
                            _: *mut PyObject,
                            _: *mut PyObject,
                        ) -> *mut PyObject,
                )),
                ml_flags: 0x1i32,
                ml_doc: _sre_getlower__doc__.as_ptr() as *mut _,
            };
            init
        },
        {
            let mut init = PyMethodDef {
                ml_name: 0 as *const libc::c_char,
                ml_meth: None,
                ml_flags: 0,
                ml_doc: 0 as *const libc::c_char,
            };
            init
        },
    ]
};
static mut sremodule: PyModuleDef = unsafe {
    {
        let mut init = PyModuleDef {
            m_base: {
                let mut init = PyModuleDef_Base {
                    ob_base: {
                        let mut init = _object {
                            ob_refcnt: 1i32 as Py_ssize_t,
                            ob_type: 0 as *const _typeobject as *mut _typeobject,
                        };
                        init
                    },
                    m_init: None,
                    m_index: 0i32 as Py_ssize_t,
                    m_copy: 0 as *const PyObject as *mut PyObject,
                };
                init
            },
            m_name: b"_sre\x00" as *const u8 as *const libc::c_char,
            m_doc: 0 as *const libc::c_char,
            m_size: -1i32 as Py_ssize_t,
            m_methods: _functions.as_ptr() as *mut _,
            m_slots: 0 as *const PyModuleDef_Slot as *mut PyModuleDef_Slot,
            m_traverse: None,
            m_clear: None,
            m_free: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn PyInit__sre() -> *mut PyObject {
    let mut m: *mut PyObject = 0 as *mut PyObject;
    let mut d: *mut PyObject = 0 as *mut PyObject;
    let mut x: *mut PyObject = 0 as *mut PyObject;
    /* Patch object types */
    if PyType_Ready(&mut Pattern_Type) != 0
        || PyType_Ready(&mut Match_Type) != 0
        || PyType_Ready(&mut Scanner_Type) != 0
    {
        return 0 as *mut PyObject;
    }
    m = PyModule_Create2(&mut sremodule, 1013i32);
    if m.is_null() {
        return 0 as *mut PyObject;
    }
    d = PyModule_GetDict(m);
    x = PyLong_FromLong(20140917i32 as libc::c_long);
    if !x.is_null() {
        PyDict_SetItemString(d, b"MAGIC\x00" as *const u8 as *const libc::c_char, x);
        let mut _py_decref_tmp: *mut PyObject = x;
        (*_py_decref_tmp).ob_refcnt -= 1;
        if !((*_py_decref_tmp).ob_refcnt != 0i32 as libc::c_long) {
            (*(*_py_decref_tmp).ob_type)
                .tp_dealloc
                .expect("non-null function pointer")(_py_decref_tmp);
        }
    }
    x = PyLong_FromLong(::std::mem::size_of::<Py_UCS4>() as libc::c_ulong as libc::c_long);
    if !x.is_null() {
        PyDict_SetItemString(d, b"CODESIZE\x00" as *const u8 as *const libc::c_char, x);
        let mut _py_decref_tmp_0: *mut PyObject = x;
        (*_py_decref_tmp_0).ob_refcnt -= 1;
        if !((*_py_decref_tmp_0).ob_refcnt != 0i32 as libc::c_long) {
            (*(*_py_decref_tmp_0).ob_type)
                .tp_dealloc
                .expect("non-null function pointer")(_py_decref_tmp_0);
        }
    }
    x = PyLong_FromUnsignedLong(!(0i32 as Py_UCS4) as libc::c_ulong);
    if !x.is_null() {
        PyDict_SetItemString(d, b"MAXREPEAT\x00" as *const u8 as *const libc::c_char, x);
        let mut _py_decref_tmp_1: *mut PyObject = x;
        (*_py_decref_tmp_1).ob_refcnt -= 1;
        if !((*_py_decref_tmp_1).ob_refcnt != 0i32 as libc::c_long) {
            (*(*_py_decref_tmp_1).ob_type)
                .tp_dealloc
                .expect("non-null function pointer")(_py_decref_tmp_1);
        }
    }
    x = PyLong_FromUnsignedLong(
        (!(0i32 as Py_UCS4)).wrapping_div(2i32 as libc::c_uint) as libc::c_ulong
    );
    if !x.is_null() {
        PyDict_SetItemString(d, b"MAXGROUPS\x00" as *const u8 as *const libc::c_char, x);
        let mut _py_decref_tmp_2: *mut PyObject = x;
        (*_py_decref_tmp_2).ob_refcnt -= 1;
        if !((*_py_decref_tmp_2).ob_refcnt != 0i32 as libc::c_long) {
            (*(*_py_decref_tmp_2).ob_type)
                .tp_dealloc
                .expect("non-null function pointer")(_py_decref_tmp_2);
        }
    }
    x = PyUnicode_FromString(copyright.as_ptr());
    if !x.is_null() {
        PyDict_SetItemString(d, b"copyright\x00" as *const u8 as *const libc::c_char, x);
        let mut _py_decref_tmp_3: *mut PyObject = x;
        (*_py_decref_tmp_3).ob_refcnt -= 1;
        if !((*_py_decref_tmp_3).ob_refcnt != 0i32 as libc::c_long) {
            (*(*_py_decref_tmp_3).ob_type)
                .tp_dealloc
                .expect("non-null function pointer")(_py_decref_tmp_3);
        }
    }
    return m;
}
/* vim:ts=4:sw=4:et
*/
