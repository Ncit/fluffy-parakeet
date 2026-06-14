# AI Validation and Repair Loop

AI-generated scenes and patches should be validated before they reach the renderer.

## Loop

1. Generate DSL or patch.
2. Validate against JSON schema.
3. Parse through the Rust scene loader.
4. Collect validation errors.
5. Ask the model to repair only the invalid fields.
6. Re-validate.
7. Apply to preview.

## Rules

- never silently drop invalid fields
- prefer small patches over full scene rewrites
- preserve stable node IDs
- keep operations undoable
- validate assets and style references
- validate duration and lifecycle ranges

## Repair prompt shape

```json
{
  "task": "repair_scene",
  "errors": [],
  "scene": {}
}
```
