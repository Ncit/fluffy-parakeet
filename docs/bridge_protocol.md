# Renderer Bridge Protocol

The editor talks to the renderer with small JSON commands.

## Commands

### load_scene

Load a scene from disk.

```json
{ "type": "load_scene", "path": "project/scene.json" }
```

### reload_scene

Reload the current scene.

```json
{ "type": "reload_scene" }
```

### set_time

Seek preview to a timestamp in seconds.

```json
{ "type": "set_time", "time": 1.25 }
```

### play

Start playback.

```json
{ "type": "play" }
```

### pause

Pause playback.

```json
{ "type": "pause" }
```

## Events

### scene_loaded

```json
{ "type": "scene_loaded", "duration": 3.0, "fps": 60 }
```

### render_error

```json
{ "type": "render_error", "message": "failed to load scene" }
```
