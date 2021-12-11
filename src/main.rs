// SPDX-License-Identifier: MIT
//
// Copyright 2021 Sony Group Corporation
//

use libseccomp::*;

fn print_num_name(arch: ScmpArch) {
    for x in 0..700 {
        let name = get_syscall_name_from_arch(arch, x);
        if !name.is_err() {
            println!("syscalls[{}] = \"{}\";", x, name.unwrap());
        }
    }
}

fn main() {
    print_num_name(ScmpArch::Native)
}
