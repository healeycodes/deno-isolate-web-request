# making a web request from an isolate

[Roll your own JavaScript runtime](https://deno.com/blog/roll-your-own-javascript-runtime) shows you how to write a JavaScript runtime, based on the V8 JavaScript engine, using parts of Deno. The example code can read and write to files and has a simplified console API.

This repository builds on that runtime and adds a global object called `request` that lets you make GET/POST web requests from within the runtime.

The bindings for `request` are set up in `runtime/src/main.rs` and `runtime/src/minijs.js`.

e.g.

```js
const getExample = await request.get("http://healeycodes.com", {
  "someHeaderKey": "someHeaderValue",
});
console.log({
  status: getExample.status,
  headers: getExample.headers,
  url: getExample.url,
  body: getExample.body,
});
```

I'm not super familiar with the Deno project, so I've only worked on this enough to get it working – I'm not using _Denoisms_ like zero-copy, etc.

## HTTP Server

There's also a HTTP server that accepts user code and evaluates it within the runtime.

So to play around, you can run the server with `cargo run` and then send some code like `curl -X POST -d 'console.log(await request.get("https://healeycodes.com", {}));' localhost:3000`. `console.log` prints to the server's stdout.

Errors are returned to the client e.g. `curl -X POST -d 'unknown;' localhost:3000` sends back `ReferenceError: unknown is not defined at at ...`

For real production example, look at how Deno's `fetch` function is setup: https://github.com/denoland/deno/tree/main/ext/fetch

## Run

Run the server: `cd runtime && cargo run`

Send some code: `curl -X POST -d 'unknown;' localhost:3000`
