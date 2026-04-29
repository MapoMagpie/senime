# senime-browser-extension

`senibe` is a Firefox-first browser extension wrapper around `senime-wasm`.

## Layout

- `extension/manifest.json`: extension manifest
- `extension/background.html`: module-capable background page
- `extension/background.js`: global wasm bootstrap and completion service
- `extension/content.js`: active input watcher and suggestion overlay
- `extension/options.*`: dictionary upload UI
- `extension/vendor/senime-wasm/*`: vendored wasm build output

## Load In Firefox

1. Open `about:debugging#/runtime/this-firefox`
2. Click `Load Temporary Add-on`
3. Select `senime-browser-extension/extension/manifest.json`
4. Open the extension options page and upload a Senime dictionary binary

After a dictionary is loaded, typing in a text input will show the completion near the field. Press `Tab` to commit the suggestion.
