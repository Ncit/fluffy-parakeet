# Architecture

Fluffy Parakeet is split into editor, scene, renderer, and export layers.

## Editor

The editor is planned as a Flutter desktop app. Flutter owns panels, timeline UX, inspectors, and AI controls.

## Scene runtime

Rust owns scene parsing, validation, sampling, and deterministic timeline evaluation.

The renderer currently consumes a JSON DSL scene and converts nodes into sampled uniforms for GPU rendering.

## Renderer

The renderer uses `wgpu` for realtime preview. The current implementation focuses on rect-like geometry and transform uniforms.

Planned renderer work:

- text nodes
- image nodes
- video frame upload
- offscreen frame rendering
- framebuffer readback

## AI workflow

AI should operate on the DSL, not pixels. The intended loop is:

1. prompt to scene
2. schema validation
3. repair if invalid
4. scene diff or patch
5. preview update
6. export

## Export

Export is planned as deterministic frame rendering plus FFmpeg encoding.
