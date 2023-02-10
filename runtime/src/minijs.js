// minijs.js
((globalThis) => {
  const core = Deno.core;

  function argsToMessage(...args) {
    return args.map((arg) => JSON.stringify(arg)).join(" ");
  }

  globalThis.console = {
    log: (...args) => {
      core.ops.op_log(argsToMessage(...args));
    },
    error: (...args) => {
      core.ops.op_log_err(argsToMessage(...args));
    },
  };

  const makeHeadersVec = (headers) => {
    const headersVec = [];
    for (const [key, value] of Object.entries(headers)) {
      headersVec.push([key, value]);
    }
    return headersVec;
  };

  core.initializeAsyncOps();
  globalThis.request = {
    get: async (url, headers) => {
      return core.ops.op_request(
        "GET",
        url.toString(),
        makeHeadersVec(headers),
        "",
      );
    },
    post: async (url, headers, body) => {
      return core.ops.op_request(
        "POST",
        url.toString(),
        makeHeadersVec(headers),
        body.toString(),
      );
    },
  };
  // };
})(globalThis);
