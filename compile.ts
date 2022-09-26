import { bundle } from "https://deno.land/x/emit/mod.ts";
const result = await bundle(
  "./hello.ts"
);

const { code } = result;
console.log(code);

Deno.writeTextFileSync("hello.bundle.js", code)