# Project Format

A Fluffy project is a folder with a `.fluffy` extension.

## Layout

```text
example.fluffy/
  project.json
  scene.json
  assets/
```

## project.json

```json
{
  "version": 1,
  "scene": "scene.json",
  "assets_dir": "assets"
}
```

The project file points to the active scene and the asset directory. Scene files continue to use DSL v3.

## Goals

- portable project folders
- deterministic asset paths
- easy sync and version control
- simple handoff between editor, renderer, and exporter
