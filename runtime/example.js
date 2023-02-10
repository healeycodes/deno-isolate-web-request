console.log("Hi!");

const getExample = await request.get("http://healeycodes.com", {
  "someHeaderKey": "someHeaderValue",
});
console.log({
  status: getExample.status,
  headers: getExample.headers,
  url: getExample.url,
  body: getExample.body,
});

const postExample = await request.post("http://healeycodes.com", {
  "someHeaderKey": "someHeaderValue",
}, "a body!");
console.log({
  status: postExample.status,
  headers: postExample.headers,
  url: postExample.url,
  body: postExample.body,
});
