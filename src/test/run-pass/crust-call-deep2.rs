native mod rustrt {
    fn rust_dbg_call(cb: *u8,
                     data: ctypes::uintptr_t) -> ctypes::uintptr_t;
}

crust fn cb(data: ctypes::uintptr_t) -> ctypes::uintptr_t {
    if data == 1u {
        data
    } else {
        count(data - 1u) + 1u
    }
}

fn count(n: uint) -> uint {
    #debug("n = %?", n);
    rustrt::rust_dbg_call(cb, n)
}

fn main() {
    // Make sure we're on a task with small Rust stacks (main currently
    // has a large stack)
    task::spawn {||
        let result = count(1000u);
        #debug("result = %?", result);
        assert result == 1000u;
    };
}