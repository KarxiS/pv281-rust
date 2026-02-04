# Rust Systems Engineering & Application Development

Welcome to this repository. It serves as a comprehensive portfolio demonstrating advanced systems programming capabilities in Rust, featuring a production-ready Linux driver utility and a series of technical implementations covering the language's core architectural patterns.

## 🚀 Featured Project: KMF Driver (Remote Desktop Linux)

**Location:** [`/remote_desktop_linux`](./remote_desktop_linux)

A specialized Linux driver and application designed to break the physical boundaries between computers. It allows users to seamlessly control multiple machines with a single mouse and keyboard and transfer files by simply dragging them across screens.

### Key Capabilities
*   **Cross-Device Input Sharing:** utilizing the Linux Input Subsystem (`evdev`) to capture and redirect input events with low latency.
*   **Seamless Workflow:** Intuitive drag-and-drop file transfer between distinct physical machines.
*   **Custom Network Protocol:** Engineered for reliability and speed to ensure a lag-free user experience.
*   **Modern Management UI:** Built with **Tauri** for a lightweight, native-feeling configuration interface.

### Technical Highlights
*   **Systems Programming:** Direct interaction with Linux kernel devices (`/dev/input`).
*   **Networking:** TCP/UDP/QUIC protocol implementation for real-time state synchronization.
*   **Architecture:** Master/Slave architecture with a robust event loop design.

---

## 🛠️ Rust Technical Portfolio (PV281)

**Location:** [`/rust_mini_projects`](./rust_mini_projects)

A structured collection of implementations verifying deep technical understanding of the Rust ecosystem, ranging from low-level memory management to full-stack web development.

| Project | Description | Core Concepts |
| :--- | :--- | :--- |
| **01 - Interpreter Engine** | Implementation of a custom language interpreter. | Control Flow, State Management, Parsing |
| **02 - Data Pipeline** | High-performance CSV processing engine. | Zero-cost Abstractions, Iterators, Pattern Matching |
| **03 - Concurrent Simulation** | Multi-threaded race simulation engine. | `Arc`, `Mutex`, `Barriers`, `mpsc` Channels, Thread Safety |
| **04 - Database Service** | Backend architecture for a social platform. | SQL Integration, Schema Design, Type-safe Persistence |
| **05 - Web Application** | Full-stack Todo application. | HTTP Routing, API Design, Frontend Integration |

## 💻 Tech Stack Overview

*   **Core:** Rust (Edition 2021+)
*   **Systems:** Linux (NixOS, Ubuntu), Shell Scripting
*   **Frameworks & Libs:** Tauri, Serde, Tokio (Async Runtime), SQLx
*   **Tools:** Docker, Git, Cargo, Nix Flakes

## 🏃 Getting Started

Each project folder contains its own detailed `README.md` with specific build and run instructions.

1.  **For the Driver:** Navigate to `remote_desktop_linux` and check the setup guide.
2.  **For Code Samples:** Explore `rust_mini_projects` to see specific implementation patterns.