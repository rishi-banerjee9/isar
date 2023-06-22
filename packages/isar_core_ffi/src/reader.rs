use crate::CIsarReader;
use isar_core::core::{reader::IsarReader, ser_de::IsarObjectSerialize};
use std::{mem, ptr, vec};

#[no_mangle]
pub unsafe extern "C" fn isar_read_id(reader: &'static CIsarReader) -> i64 {
    match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => reader.read_id(),
        #[cfg(feature = "native")]
        CIsarReader::NativeList(reader) => reader.read_id(),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => reader.read_id(),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteObject(reader) => reader.read_id(),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteList(reader) => reader.read_id(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_null(reader: &'static CIsarReader, index: u32) -> bool {
    match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => reader.is_null(index),
        #[cfg(feature = "native")]
        CIsarReader::NativeList(reader) => reader.is_null(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => reader.is_null(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteObject(reader) => reader.is_null(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteList(reader) => reader.is_null(index),
    }
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_bool(reader: &'static CIsarReader, index: u32) -> bool {
    let value = match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => reader.read_bool(index),
        #[cfg(feature = "native")]
        CIsarReader::NativeList(reader) => reader.read_bool(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => reader.read_bool(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteObject(reader) => reader.read_bool(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteList(reader) => reader.read_bool(index),
    };
    value.unwrap_or(false)
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_byte(reader: &'static CIsarReader, index: u32) -> u8 {
    match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => reader.read_byte(index),
        #[cfg(feature = "native")]
        CIsarReader::NativeList(reader) => reader.read_byte(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => reader.read_byte(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteObject(reader) => reader.read_byte(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteList(reader) => reader.read_byte(index),
    }
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_int(reader: &'static CIsarReader, index: u32) -> i32 {
    match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => reader.read_int(index),
        #[cfg(feature = "native")]
        CIsarReader::NativeList(reader) => reader.read_int(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => reader.read_int(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteObject(reader) => reader.read_int(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteList(reader) => reader.read_int(index),
    }
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_float(reader: &'static CIsarReader, index: u32) -> f32 {
    match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => reader.read_float(index),
        #[cfg(feature = "native")]
        CIsarReader::NativeList(reader) => reader.read_float(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => reader.read_float(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteObject(reader) => reader.read_float(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteList(reader) => reader.read_float(index),
    }
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_long(reader: &'static CIsarReader, index: u32) -> i64 {
    match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => reader.read_long(index),
        #[cfg(feature = "native")]
        CIsarReader::NativeList(reader) => reader.read_long(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => reader.read_long(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteObject(reader) => reader.read_long(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteList(reader) => reader.read_long(index),
    }
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_double(reader: &'static CIsarReader, index: u32) -> f64 {
    match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => reader.read_double(index),
        #[cfg(feature = "native")]
        CIsarReader::NativeList(reader) => reader.read_double(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => reader.read_double(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteObject(reader) => reader.read_double(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteList(reader) => reader.read_double(index),
    }
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_string(
    reader: &'static CIsarReader,
    index: u32,
    value: *mut *const u8,
    is_ascii: *mut bool,
) -> u32 {
    let str = match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => reader.read_string(index),
        #[cfg(feature = "native")]
        CIsarReader::NativeList(reader) => reader.read_string(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => reader.read_string(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteObject(reader) => reader.read_string(index),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteList(reader) => reader.read_string(index),
    };
    if let Some(str) = str {
        let len = str.len();
        let ptr = str.as_ptr();
        mem::forget(str);
        *value = ptr;
        *is_ascii = str.is_ascii();
        len as u32
    } else {
        *value = ptr::null();
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_object(
    reader: &'static CIsarReader,
    index: u32,
) -> *mut CIsarReader<'static> {
    let new_reader = match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => reader.read_object(index).map(|r| CIsarReader::Native(r)),
        #[cfg(feature = "native")]
        CIsarReader::NativeList(reader) => {
            reader.read_object(index).map(|r| CIsarReader::Native(r))
        }
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => reader
            .read_object(index)
            .map(|r| CIsarReader::SQLiteObject(r)),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteObject(reader) => reader
            .read_object(index)
            .map(|r| CIsarReader::SQLiteObject(r)),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteList(reader) => reader
            .read_object(index)
            .map(|r| CIsarReader::SQLiteObject(r)),
    };
    if let Some(new_reader) = new_reader {
        Box::into_raw(Box::new(new_reader))
    } else {
        ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_list(
    reader: &'static CIsarReader,
    index: u32,
    list_reader: *mut *mut CIsarReader<'static>,
) -> u32 {
    let reader_size = match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => reader
            .read_list(index)
            .map(|(r, s)| (CIsarReader::NativeList(r), s)),
        #[cfg(feature = "native")]
        CIsarReader::NativeList(reader) => reader
            .read_list(index)
            .map(|(r, s)| (CIsarReader::NativeList(r), s)),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => reader
            .read_list(index)
            .map(|(r, s)| (CIsarReader::SQLiteList(r), s)),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteObject(reader) => reader
            .read_list(index)
            .map(|(r, s)| (CIsarReader::SQLiteList(r), s)),
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLiteList(reader) => reader
            .read_list(index)
            .map(|(r, s)| (CIsarReader::SQLiteList(r), s)),
    };
    if let Some((new_reader, size)) = reader_size {
        *list_reader = Box::into_raw(Box::new(new_reader));
        size
    } else {
        *list_reader = ptr::null_mut();
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_to_json(
    reader: &'static CIsarReader,
    id_name: &String,
    buffer: *mut *mut u8,
    buffer_size: *mut u32,
) -> u32 {
    let mut new_buffer = if buffer.is_null() {
        vec![]
    } else {
        Vec::from_raw_parts(*buffer, 0, *buffer_size as usize)
    };

    let serialized = match reader {
        #[cfg(feature = "native")]
        CIsarReader::Native(reader) => {
            let ser = IsarObjectSerialize::new(Some(id_name), reader);
            serde_json::to_writer(&mut new_buffer, &ser).is_ok()
        }
        #[cfg(feature = "sqlite")]
        CIsarReader::SQLite(reader) => {
            let ser = IsarObjectSerialize::new(Some(id_name), reader);
            serde_json::to_writer(&mut new_buffer, &ser).is_ok()
        }
        _ => false,
    };

    if serialized {
        let (ptr, len, cap) = new_buffer.into_raw_parts();
        *buffer_size = cap as u32;
        *buffer = ptr;
        len as u32
    } else {
        *buffer_size = 0;
        *buffer = ptr::null_mut();
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn isar_read_free(reader: *const CIsarReader) {
    if !reader.is_null() {
        drop(Box::from_raw(reader as *mut CIsarReader));
    }
}
