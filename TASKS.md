# Fluffy Parakeet Task Tracker

Status legend: `[x]` done, `[ ]` todo, `[~]` in progress / needs verification.

## 0. Current foundation

- [x] Create Rust workspace with `engine-rs` and `renderer`
- [x] Add basic scene graph model
- [x] Add timeline and animation primitives
- [x] Add Flutter desktop editor shell
- [x] Add AI DSL schema and example scene
- [x] Add wgpu renderer skeleton
- [x] Add quad mesh rendering
- [x] Add transform uniforms
- [x] Add DSL-backed scene loading
- [x] Add multi-node rendering
- [x] Add color, opacity, rotation, layer, width, and height semantics
- [x] Add alpha blending
- [x] Add Rust check script and CI workflow
- [~] Verify CI actually runs on GitHub Actions
- [~] Fix any real compiler errors from `cargo check --workspace`

## 1. Build and repository hygiene

- [ ] Confirm GitHub Actions is enabled for the repository
- [ ] Run `scripts/check.sh` locally or through CI
- [ ] Fix `wgpu` / `winit` API mismatches if build fails
- [ ] Generate and commit `Cargo.lock`
- [ ] Add root `README.md` with project vision, architecture, and local setup
- [ ] Add `docs/architecture.md`
- [ ] Add `docs/dsl.md`
- [ ] Add issue/PR templates

## 2. Scene graph and timeline MVP

- [x] Add `start_time` and `end_time` node lifecycle fields
- [x] Skip inactive nodes during render
- [x] Add timeline visibility tests
- [x] Add easing support to keyframes
- [x] Add easing tests
- [x] Add transform origin / anchor point support
- [x] Add parent-child node transforms
- [x] Add scene validation errors instead of panics
- [x] Add stable node IDs and selection metadata

## 3. DSL v3

- [ ] Version the DSL with a top-level `version` field
- [ ] Add strict JSON schema for current DSL
- [ ] Add schema validation to scripts/CI
- [ ] Add reusable animation presets
- [ ] Add named styles
- [ ] Add asset references
- [ ] Add project-level metadata: fps, width, height, background
- [ ] Add deterministic scene loading from external files, not only `include_str!`

## 4. Text nodes

- [ ] Choose text renderer crate, likely `glyphon`
- [ ] Add text node to DSL
- [ ] Add font size, font family, weight, alignment, line height
- [ ] Add text color and opacity
- [ ] Render text in wgpu
- [ ] Add text wrapping
- [ ] Add caption/subtitle-friendly text presets

## 5. Image nodes

- [ ] Add texture loading pipeline
- [ ] Add PNG/JPEG asset loading
- [ ] Add UV coordinates to mesh
- [ ] Add image node to DSL
- [ ] Add `contain`, `cover`, and `stretch` fit modes
- [ ] Add asset cache
- [ ] Add missing-asset fallback rendering

## 6. Video and audio nodes

- [ ] Choose decode strategy for source video/audio
- [ ] Add video node DSL shape
- [ ] Add frame decode/upload path
- [ ] Add audio track model
- [ ] Add basic waveform metadata
- [ ] Add timeline sync for video/audio nodes

## 7. Flutter editor integration

- [ ] Create complete Flutter project scaffold if missing
- [ ] Add Rust renderer process launch from Flutter
- [ ] Choose bridge: local WebSocket, gRPC, or FFI
- [ ] Implement renderer command protocol
- [ ] Add play/pause/scrub controls
- [ ] Add timeline panel with node tracks
- [ ] Add inspector panel for selected node
- [ ] Add DSL JSON load/reload
- [ ] Add live preview updates

## 8. AI-first workflow

- [ ] Add prompt-to-DSL generator interface
- [ ] Add DSL validation and repair loop
- [ ] Add edit-current-scene command format
- [ ] Add operations: change colors, add title, adjust timing, resize format
- [ ] Add scene diff/patch representation
- [ ] Add undo/redo-friendly AI edits
- [ ] Add prompt examples and eval scenes

## 9. Export pipeline

- [ ] Add offscreen renderer mode
- [ ] Add deterministic frame rendering by timestamp/frame index
- [ ] Add framebuffer readback
- [ ] Add PNG sequence export
- [ ] Add FFmpeg MP4 export
- [ ] Add export presets: 1080p, 4K, vertical short
- [ ] Add progress reporting and cancellation

## 10. Product packaging

- [ ] Define `.fluffy` project folder format
- [ ] Add project save/load
- [ ] Bundle renderer binary with Flutter desktop app
- [ ] Add macOS packaging
- [ ] Add Windows packaging
- [ ] Add auto-update plan

## Recommended next sprint

1. Verify build and CI.
2. Version the DSL with a top-level `version` field.
3. Add README and DSL docs.
4. Start text node rendering.
5. Add renderer/editor bridge.
