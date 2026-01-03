use crate::dyld::{export_c_func, FunctionExports};
use crate::libc::errno::{EINVAL, E2BIG};
use crate::mem::{ConstPtr, MutPtr};
use crate::Environment;
use std::ops::Add;

type iconv_t = MutPtr<()>;

fn iconv_open(
    _env: &mut Environment,
    _tocode: ConstPtr<u8>,
    _fromcode: ConstPtr<u8>,
) -> iconv_t {
    // Dummy non-null handle means “identity converter”
    MutPtr::from_bits(1)
}

fn iconv(
    env: &mut Environment,
    cd: iconv_t,
    inbuf: MutPtr<MutPtr<u8>>,
    inbytesleft: MutPtr<u32>,
    outbuf: MutPtr<MutPtr<u8>>,
    outbytesleft: MutPtr<u32>,
) -> u32 {
    if cd.is_null() {
        env.libc_state.errno = EINVAL;
        return u32::MAX;
    }

    let mut in_ptr = env.mem.read(inbuf);
    let mut out_ptr = env.mem.read(outbuf);
    let mut in_left = env.mem.read(inbytesleft);
    let mut out_left = env.mem.read(outbytesleft);

    let mut converted = 0usize;

    while in_left > 0 && out_left > 0 {
        let byte = env.mem.read(in_ptr);
        env.mem.write(out_ptr, byte);

        in_ptr = in_ptr.add(1);
        out_ptr = out_ptr.add(1);
        in_left -= 1;
        out_left -= 1;
        converted += 1;
    }

    if in_left > 0 {
        env.errno = E2BIG;
        return u32::MAX;
    }

    env.mem.write(inbuf, in_ptr);
    env.mem.write(outbuf, out_ptr);
    env.mem.write(inbytesleft, in_left);
    env.mem.write(outbytesleft, out_left);

    0
}

fn iconv_close(_env: &mut Environment, _cd: iconv_t) -> i32 {
    0
}

pub const FUNCTIONS: FunctionExports = &[
    export_c_func!(iconv_open(_, _)),
    export_c_func!(iconv(_, _, _, _, _)),
    export_c_func!(iconv_close(_)),
];
