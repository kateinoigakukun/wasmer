#![allow(unused)]

mod types;

use self::types::*;
use crate::{
    ptr::{Array, WasmPtr},
    state::WasiState,
};
use wasmer_runtime_core::{memory::Memory, vm::Ctx};

#[allow(clippy::mut_from_ref)]
fn get_wasi_state(ctx: &Ctx) -> &mut WasiState {
    unsafe { &mut *(ctx.data as *mut WasiState) }
}

fn write_buffer_array(
    memory: &Memory,
    from: &[Vec<u8>],
    ptr_buffer_offset: u32,
    buffer_offset: u32,
) {
    let mut current_buffer_offset = buffer_offset;
    for (i, sub_buffer) in from.iter().enumerate() {
        memory.view::<u32>()[(ptr_buffer_offset as usize)..][i].set(current_buffer_offset);
        for (cell, &byte) in memory.view()[(current_buffer_offset as usize)..]
            .iter()
            .zip(sub_buffer.iter())
        {
            cell.set(byte);
        }
        current_buffer_offset += sub_buffer.len() as u32;
    }
}

/// ### `args_get()`
/// Read command-line argument data.
/// The sizes of the buffers should match that returned by [`args_sizes_get()`](#args_sizes_get).
/// Inputs:
/// - `char **argv`
///     A pointer to a buffer to write the argument pointers.
/// - `char *argv_buf`
///     A pointer to a buffer to write the argument string data.
///
pub fn args_get(ctx: &mut Ctx, ptr_buffer_offset: u32, buffer_offset: u32) -> __wasi_errno_t {
    let state = get_wasi_state(ctx);
    let memory = ctx.memory(0);

    write_buffer_array(memory, &*state.args, ptr_buffer_offset, buffer_offset);

    __WASI_ESUCCESS
}

/// ### `args_sizes_get()`
/// Return command-line argument data sizes.
/// Outputs:
/// - `size_t *argc`
///     The number of arguments.
/// - `size_t *argv_buf_size`
///     The size of the argument string data.
pub fn args_sizes_get(ctx: &mut Ctx, argc_out: u32, argv_buf_size_out: u32) -> __wasi_errno_t {
    let state = get_wasi_state(ctx);
    let memory = ctx.memory(0);

    let arg_count = state.args.len();
    let total_arg_size: usize = state.args.iter().map(|v| v.len()).sum();

    memory.view::<u32>()[(argc_out / 4) as usize].set(arg_count as u32);
    memory.view::<u32>()[(argv_buf_size_out / 4) as usize].set(total_arg_size as u32);

    __WASI_ESUCCESS
}

pub fn clock_res_get(
    ctx: &mut Ctx,
    clock_id: __wasi_clockid_t,
    resolution: WasmPtr<__wasi_timestamp_t>,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn clock_time_get(
    ctx: &mut Ctx,
    clock_id: __wasi_clockid_t,
    precision: __wasi_timestamp_t,
    time: WasmPtr<__wasi_timestamp_t>,
) -> __wasi_errno_t {
    unimplemented!()
}

/// ### `environ_get()`
/// Read environment variable data.
/// The sizes of the buffers should match that returned by [`environ_sizes_get()`](#environ_sizes_get).
/// Inputs:
/// - `char **environ`
///     A pointer to a buffer to write the environment variable pointers.
/// - `char *environ_buf`
///     A pointer to a buffer to write the environment variable string data.
pub fn environ_get(ctx: &mut Ctx, environ: u32, environ_buf: u32) -> __wasi_errno_t {
    let state = get_wasi_state(ctx);
    let memory = ctx.memory(0);

    write_buffer_array(memory, &*state.args, environ, environ_buf);

    __WASI_ESUCCESS
}

/// ### `environ_sizes_get()`
/// Return command-line argument data sizes.
/// Outputs:
/// - `size_t *environ_count`
///     The number of environment variables.
/// - `size_t *environ_buf_size`
///     The size of the environment variable string data.
pub fn environ_sizes_get(
    ctx: &mut Ctx,
    environ_count_out: u32,
    environ_buf_size_out: u32,
) -> __wasi_errno_t {
    let state = get_wasi_state(ctx);
    let memory = ctx.memory(0);

    let env_count = state.envs.len();
    let total_env_size: usize = state.envs.iter().map(|v| v.len()).sum();

    memory.view::<u32>()[(environ_count_out / 4) as usize].set(env_count as u32);
    memory.view::<u32>()[(environ_buf_size_out / 4) as usize].set(total_env_size as u32);

    __WASI_ESUCCESS
}

pub fn fd_advise(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    offset: __wasi_filesize_t,
    len: __wasi_filesize_t,
    advice: __wasi_advice_t,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_allocate(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    offset: __wasi_filesize_t,
    len: __wasi_filesize_t,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_close(ctx: &mut Ctx, fd: __wasi_fd_t) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_datasync(ctx: &mut Ctx, fd: __wasi_fd_t) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_fdstat_get(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    buf: WasmPtr<__wasi_fdstat_t>,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_fdstat_set_flags(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    flags: __wasi_fdflags_t,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_fdstat_set_rights(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    fs_rights_base: __wasi_rights_t,
    fs_rights_inheriting: __wasi_rights_t,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_filestat_get(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    buf: WasmPtr<__wasi_filestat_t>,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_filestat_set_size(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    st_size: __wasi_filesize_t,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_filestat_set_times(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    st_atim: __wasi_timestamp_t,
    st_mtim: __wasi_timestamp_t,
    fst_flags: __wasi_fstflags_t,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_pread(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    iovs: WasmPtr<__wasi_iovec_t, Array>,
    iovs_len: u32,
    offset: __wasi_filesize_t,
    nread: WasmPtr<u32>,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_prestat_get(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    buf: WasmPtr<__wasi_fdstat_t>,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_prestat_dir_name(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    path: WasmPtr<u8, Array>,
    path_len: u32,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_pwrite(
    ctx: &mut Ctx,
    fd: __wasi_fd_t,
    iovs: WasmPtr<__wasi_ciovec_t, Array>,
    iovs_len: u32,
    offset: __wasi_filesize_t,
    nwritten: WasmPtr<u32>,
) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_read(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_readdir(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_renumber(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_seek(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_sync(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_tell(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn fd_write(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn path_create_directory(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn path_filestat_get(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn path_filestat_set_times(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn path_link(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn path_open(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn path_readlink(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn path_remove_directory(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn path_rename(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn path_symlink(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn path_unlink_file(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn poll_oneoff(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn proc_exit(ctx: &mut Ctx) {
    unimplemented!()
}
pub fn proc_raise(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn random_get(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn sched_yield(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn sock_recv(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn sock_send(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
pub fn sock_shutdown(ctx: &mut Ctx) -> __wasi_errno_t {
    unimplemented!()
}
