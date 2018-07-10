# next.rs :unicorn:
[SSR + SPA(WASM)], heavy wip

## How it works
[SSR] -> client lib -> static html output from params (path) given
[SPA] -> js/html output from stdweb, incl. client-side routing

The "original html" will just be passed off to the server on runtime.
Then it will transform itself into an SPA via `stdweb`.

## Worfklow
`yarn start` to see SSR in action
`yarn client` to dev SPA

## Requirements
rustup & `rustup default nightly`
