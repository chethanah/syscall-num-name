# Generate Syscall Name to Num Mapping

- Generate syscall name->number mapping using libseccomp
- The generated output formate is used for confine createMap

## Usage
```
# git clone git@gitlab-bdk.sm.sony.co.jp:Chethan.Suresh/syscall-num-name.git
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