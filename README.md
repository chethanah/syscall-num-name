# Generate Syscall Name to Num Mapping

- Generate all syscall name to number mapping using libseccomp
- Libseccomp is platform independent and can be used with any Linux kernel version.

## Requirements
- Install `libseccomp` library using your package manager
- Example: In debian based systems

```
$ sudo apt install libseccomp-dev 
```

## Usage
- Build from source
```
# git clone https://github.com/chethanah/syscall-num-name
# cd syscall-num-name
# cargo build
# ./target/debug/syscall-num-name
syscalls[0] = "read";
syscalls[1] = "write";
syscalls[2] = "open";
syscalls[3] = "close";
syscalls[4] = "stat";
...
<SKIP>
```
- Or download the release binary
