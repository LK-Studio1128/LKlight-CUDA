# LKlight-CUDA v1.1.1

**Nucleic-acid scoring safety patch for the standalone CUDA build.**

## Fixed

- Nucleic-acid inputs reject unsupported scoring functions with a clear error
  and exit code 2 instead of returning a constant zero score.
- Worker-thread failures are reflected in the process exit status.
- The CUDA DNA regression uses a relative tolerance appropriate for the grid
  path.

## Validation

- `cargo test --release --lib`: **34 passed** on the CPU fallback build.
- Real protein–DNA regression: macOS CPU fallback **-5192.292247**; Linux CUDA
  **-5192.291734** with CUDA batching active.
- A 1×20×20 Linux CUDA docking produced ranked scores **39.606634–49.655229**
  (span 10.048595).
- Unsupported scorers fail with exit code 2 before they can produce `-0`.

## Binaries

- `LKlight-CUDA-v1.1.1-linux-x86_64` — Linux x86-64, CUDA
- `LKlight-CUDA-v1.1.1-mac-arm64` — macOS Apple Silicon, CPU fallback
- `LKlight-CUDA-v1.1.1-win64.exe` — Windows x64, CUDA
