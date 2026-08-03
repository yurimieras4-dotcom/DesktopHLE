# DesktopHLE

**DesktopHLE** is a high-level emulator (HLE) / translation layer designed to run legacy Mac OS X (10.4 Tiger – 10.6 Snow Leopard) binaries natively on modern systems without full OS virtualization.

Inspired by the philosophy of [touchHLE](https://github.com/touchHLE/touchHLE), DesktopHLE aims to incrementally implement framework stubs and system call bridges for a frozen era of desktop software.

---

## 🚀 Roadmap & Milestones

- [ ] **`UITest (Beta)`**
  - [ ] Framebuffer & window creation (`winit` / `pixels`)
  - [ ] On-screen virtual key overlay (Command `⌘`, Option `⌥`, Touch buttons)
- [ ] **`v0.0.0 (Preview)`**
  - [ ] 32-bit x86 / FAT Mach-O binary parsing
  - [ ] Memory segment layout (`__TEXT`, `__DATA`, `__LINKEDIT`)
  - [ ] Import symbol table logging
- [ ] **`v0.0.1 (Preview)`**
  - [ ] Execution entry point jump
  - [ ] Core C library (`libSystem` / `libc`) stubs
- [ ] **`v0.0.2 (Official Release)`**
  - [ ] Objective-C runtime bridge (`objc_msgSend`)
  - [ ] Windowing & graphics context binding to host canvas
  - [ ] First targeted 2000s Mac OS X app execution

---

## 🛠️ Building & Running Locally

### Prerequisites
* Rust compiler & Cargo (`rustup`): [https://rustup.rs](https://rustup.rs)

### Commands
```bash
# Clone the repository
git clone [https://github.com/yurimieras4-dotcom/DesktopHLE.git](https://github.com/yurimieras4-dotcom/DesktopHLE.git)
cd DesktopHLE

# Build and run
cargo run
