
# ArrowIO: Evasion Integrity Verifier

ArrowIO is a cross-platform tool designed to audit EDR telemetry. By utilizing raw assembly syscalls and bypassing CRT/glibc, this script assists in confirming evasion via naked Rust binaries through user-mode API hooking and IAT-based static analysis.

⚠️ **Please Note:** This project is strictly for **Educational and Authorized Penetration Testing**. I am not responsible for any of the shenanigans you guys pull.


## 🛡️Technical Overview

Modern security solutions monitor high-level function calls (e.g., ``CreateFile`` or ``write``). ArrowIO bypasses these by communicating directly with the OS Kernel.

- **Zero Dependencies:** No linking to ``libc``, ``ntdll.dll``, or ``kernel32.dll``.
- **Minimalist Footprint:** Linux binaries at **~13KB**, Windows binaries at **2.5KB**.
- **Import Stealth:** Empty IAT, making the binary invisible to static API monitors.

## 🚀 Implementation Details


### 🐧 Linux Implementation

Building binary on a Linux environment (or WSL2).

#### 1. Environment Setup
Ensure you have Rust toolchain and `musl` targets installed. This allows the file to link without a dynamic C library.

```bash
# install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# add the static 64-bit target
rustup target add x86_64-unknown-linux-musl

# install musl tools
sudo apt update && sudo apt install musl-tools -y
```

#### 2. Project Compilation
Change to the Linux directory. Copy this build command to override the standard entry point and strip all runtime metadata.

```bash
cd Linux

RUSTFLAGS="-C panic=abort -C link-arg=-nostartfiles -C link-self-contained=no" cargo build --release --target x86_64-unknown-linux-musl
```

#### 3. Verification of Evasion
Once the build is complete, verify that the binary has no dependencies and is talking directly to the kernel.

**A. Dependency Check:**
Check if the binary is statically linked.
```bash
ldd ./target/x86_64-unknown-linux-musl/release/arrowio-linux
```
*   **Success Result:** `not a dynamic executable`, `statically linked`

**B. Syscall Audit:**
Use `strace` to see the direct kernel transitions.
```bash
strace ./target/x86_64-unknown-linux-musl/release/arrowio-linux
```
*   **Observation:** Look for `write` and `exit` calls occurring without the usual `glibc` initialization noise (like `mmap`, `open`, or `brk` calls that standard binaries make before reaching your code).

**C. Size Audit:**
Verify the minimalist footprint.
```bash
ls -lh ./target/x86_64-unknown-linux-musl/release/arrowio-linux
```
*   **Result:** Should be ~13KB.

### 🪟 Windows Implementation

This script bypasses the standard Win32 subsystem (`kernel32.dll` and `ntdll.dll`) by using direct syscalls and a custom entry point.

#### 1. Environment Setup
To build this, you need the Rust toolchain and the C++ Build Tools (Linker and SDK).

- **Install Rust:** Download and run `rustup-init.exe` from [rustup.rs](https://rustup.rs).
- **Install Build Tools:** Complete the installation via the Visual Studio installer.
- **Terminal:** To access Linker tools, open **Developer PowerShell for VS 2022** instead of regular PowerShell.

#### 2. Project Compilation
Navigate to the `Windows` directory within the repo. To strip CRT from the binary, pass these flags:

```powershell
cd Windows

# set environment flags
$env:RUSTFLAGS="-C target-feature=+crt-static -C link-arg=/NODEFAULTLIB -C link-arg=/SUBSYSTEM:CONSOLE"

# compile
cargo build --release
```

#### 3. Forensic Verification
Since the binary has no console output, verify its integrity by auditing its structure.

**A. IAT Audit:**
Verify that the binary does not import any functions from Windows DLLs.
```powershell
dumpbin /IMPORTS .\target\release\arrowio-win.exe
```
*   **Expected Result:** The output should only show a summary (e.g., `.text`, `.rdata`). There should be **no list of DLLs** like `KERNEL32.dll`. This confirms the binary slipped unrecognized to API hookers.

**B. Exit Code Verification:**
Run the binary and check the process exit code. This confirms the direct `syscall` reached the kernel successfully.
```powershell
.\target\release\arrowio-win.exe
echo $LASTEXITCODE
```
*   **Expected Result:** `0` (This confirms `NtTerminateProcess` was executed via raw assembly).

**C. Footprint Audit:**
Verify the binary size.
```powershell
(Get-Item .\target\release\arrowio-win.exe).length
```
*   **Expected Result:** `~2.5 KB`
## 🧩 Evasion Matrix



| Detection Layer | Standard Binary Behavior | ArrowIO Stealth Behavior |
| :--- | :--- | :--- |
| **Static IAT Analysis** | Lists all used functions (e.g., `CreateFile`) in the Import Address Table. | **Bypassed.** IAT is empty. Scanners see no suspicious API dependencies. |
| **User-Mode Hooking** | Calls functions in `ntdll.dll` or `libc` where EDRs have placed hooks. | **Bypassed.** Assembly `syscall` instructions jump directly to the kernel skipping the hooked libraries. |
| **Standard Telemetry** | Generates noisy logs via CRT initialization. | **Minimized.** No runtime overhead, no thread initialization noise, sub-3KB footprint. |
| **API Monitoring** | Security tools monitor `kernel32!CreateFile` or `libc!write`. | **Bypassed.** Execution flow never enters these monitored function addresses. |

### MITRE
This script demonstrates **MITRE ATT&CK T1059 (Command and Scripting Interpreter)** and **T1106 (Native API)**.

---
