Here's the polished version:

# Debugging Contract

## Native Simulator Debugging

By running `make generate`, a Rust contract is created with a `-dbg` suffix in the contract directory. This project allows you to compile the contract as a native platform dynamic library. To build it, simply run `make build-simulator`. Afterward, you can insert breakpoints and debug the contract code by enabling the `simulator` feature of `ckb-testtool` in unit tests.

## Debugging with ckb-debugger and VSCode

An alternative debugging method is available using `ckb-debugger` (in this case, with VSCode). `ckb-debugger` operates as a `gdb server`, supporting both gdb and lldb-18. In VSCode, the [Native Debug](https://marketplace.visualstudio.com/items?itemName=webfreak.debug) extension supports remote gdb debugging, which you'll need for this setup.
* Note: Although CodeLLDB supports remote debugging, it does not work well here due to the use of lldb version 17.

### Compilation

By default, the compiled output lacks debug symbols due to size constraints, so you need to compile using the `profile.ckb-debug` profile. You'll also need to create a copy of the binary for gdb/llvm debugging and strip the symbols from the original file for `ckb-debugger` to run.

```shell
make build MODE=ckb-debug CARGO_ARGS='--profile=ckb-debug'
cp build/ckb-debug/<Contract-Name> build/ckb-debug/<Contract-Name>.debug
llvm-objcopy --strip-debug --strip-all build/ckb-debug/<Contract-Name>
```

To configure `tasks.json` in VSCode (using `c1` from CI as an example):

```json
{
    "label": "Build Debug",
    "type": "shell",
    "command": "make build MODE=ckb-debug CARGO_ARGS='--profile=ckb-debug' && cp build/ckb-debug/c1 build/ckb-debug/c1.debug && llvm-objcopy --strip-debug --strip-all build/ckb-debug/c1"
},
```

#### Running ckb-debugger

(You can skip this section if the contract you are debugging doesn’t require transaction information.)

Typically, contracts need transaction data, which can be retrieved from unit tests. Add the following code before `context.verify_tx(&tx, MAX_CYCLES)` in your unit tests:

```rust
let tx_data = context.dump_tx(&tx).expect("dump tx info");
std::fs::write(
    "tx.json",
    serde_json::to_string_pretty(&tx_data).expect("json"),
)
.expect("write tx");
```

Then, start `ckb-debugger` with:

```shell
ckb-debugger \
    --bin=build/ckb-debug/c1 \
    --mode=gdb_gdbstub \
    --gdb-listen=0.0.0.0:8000 \
    --tx-file=tests/tx.json \
    -s=lock \
    -i=0
```
* The example uses port 8000; feel free to change it if necessary.
* Adjust `-s` and `-i` according to your contract's needs.

You can configure `tasks.json` in VSCode to automatically start `ckb-debugger`:

```json
{
    "label": "Debug c1",
    "isBackground": true,
    "type": "process",
    "command": "ckb-debugger",
    "args": [
        "--bin=build/ckb-debug/c1",
        "--mode=gdb_gdbstub",
        "--gdb-listen=0.0.0.0:8000",
        "--tx-file=tests/tx.json",
        "-s=lock",
        "-i=0"
    ],
    "options": {
        "cwd": "${workspaceRoot}"
    },
},
```
* The `isBackground` setting ensures the task runs in the background and doesn't terminate during debugging.

Since `ckb-debugger` doesn’t exit automatically after debugging, you'll need to configure a task to stop it:

```json
{
    "label": "stop-ckb-debugger",
    "type": "shell",
    "command": "killall ckb-debugger || true"
},
```

#### GDB Debugging

```shell
gdb build/ckb-debug/c1.debug
```
Then connect to `ckb-debugger`'s gdb server:

```shell
target remote 127.0.0.1:8000
```
Once connected, you can debug using standard GDB commands.

#### LLDB Debugging

* Ensure you are using lldb version 18 or later.

```shell
lldb build/ckb-debugger/c1.debug
```
Then connect to `ckb-debugger`'s gdb server (you can omit the local address and just use the port):

```shell
gdb-remote 8000
```
After connecting, you can use standard LLDB commands to debug.

#### VSCode Debugging

* GDB must be installed.
* The Native Debug extension is required for debugging in VSCode.

First, configure `tasks.json` as described above. Then, set up your `launch.json` for debugging:

```json
{
    "name": "GDB",
    "type": "gdb",
    "request": "attach",
    "executable": "build/ckb-debug/c1.debug",
    "debugger_args": [],
    "cwd": "${workspaceRoot}",
    "remote": true,
    "target": "127.0.0.1:8000",
    "preLaunchTask": "Debug c1",
    "postDebugTask": "stop-ckb-debugger"
}
```
After launching the debugger, you can set breakpoints and inspect variables as usual.

### Additional Notes

* The Native Simulator method is more convenient and supports advanced debugging features, which may not be available with `ckb-debugger`.
* `ckb-debugger` provides a debugging environment closer to the contract's runtime environment, while the Native Simulator only emulates the online environment.
* `ckb-debugger` may perform poorly compared to the Native Simulator, especially on low-end computers.