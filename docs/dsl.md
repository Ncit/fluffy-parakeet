# DSL v3

The scene DSL is a JSON format for deterministic video scenes.

## Top-level fields

- `version`: must be `3`
- `duration`: scene duration in seconds
- `fps`: frames per second, defaults to 60
- `width`: output width, defaults to 1920
- `height`: output height, defaults to 1080
- `background`: RGBA background color
- `assets`: named media or font references
- `styles`: reusable visual style blocks
- `presets`: reusable animation blocks
- `nodes`: ordered scene objects

## Node fields

Every node requires:

- `id`
- `type`

Supported node types today:

- `rect`
- `text`
- `image`

Common optional fields:

- `parent_id`
- `style`
- `preset`
- `asset`
- `layer`
- `start_time`
- `end_time`
- `selection`
- `color`
- `width`
- `height`
- `anchor_x`
- `anchor_y`
- `x`
- `y`
- `scale_x`
- `scale_y`
- `rotation`
- `opacity`

## Keyframes

Animated fields use arrays of keyframes:

```json
[{ "time": 0.0, "value": 0.0 }, { "time": 1.0, "value": 1.0, "easing": "ease_out" }]
```

Supported easing values:

- `linear`
- `ease_in`
- `ease_out`
- `ease_in_out`
