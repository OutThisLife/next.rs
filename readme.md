# next.rs :unicorn:
[SSR + SPA(WASM)]

WORK IN PROGRESS

## Render flow
[SSR] -> client lib -> static html output from params given
[SPA] -> html output from stdweb

The "original html" will just be passed off to the server on runtime. Then it will transform itself into an SPA via `stdweb`
