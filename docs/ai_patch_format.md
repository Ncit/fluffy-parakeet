# AI Patch Format

AI edits should be represented as structured operations instead of raw text rewrites.

## Patch envelope

```json
{
  "version": 1,
  "operations": []
}
```

## Operations

### add_node

```json
{ "op": "add_node", "node": { "id": "title", "type": "text", "text": "Hello" } }
```

### update_node

```json
{ "op": "update_node", "id": "title", "set": { "text": "Updated" } }
```

### remove_node

```json
{ "op": "remove_node", "id": "title" }
```

### add_asset

```json
{ "op": "add_asset", "id": "logo", "asset": { "type": "image", "src": "assets/logo.png" } }
```

### add_style

```json
{ "op": "add_style", "id": "hero", "style": { "color": [1, 1, 1, 1] } }
```

## Design goals

- deterministic edits
- easy undo/redo
- small scene diffs
- validation after every patch
- repair loop for invalid patches
