const results = await Promise.all([
  Bun.build({
    entrypoints: ["src/highlightjs/line-numbers.js"],
    minify: true,
    target: "browser",
  }),
  Bun.build({
    entrypoints: ["src/highlightjs/line-numbers.css"],
    minify: true,
  }),
  Bun.build({
    entrypoints: ["src/heading/numbering.css"],
    minify: true,
  }),
  Bun.build({
    entrypoints: ["src/heading/hide.css"],
    minify: true,
  }),
]);

const outfiles = [
  "src/highlightjs/line-numbers-min.js",
  "src/highlightjs/line-numbers-min.css",
  "src/heading/numbering-min.css",
  "src/heading/hide-min.css",
];

await Promise.all(
  results.map(async (result, index) => {
    if (!result.success) {
      throw new Error(`Build ${index} failed:\n${result.logs.join("\n")}`);
    }
    if (result.outputs.length > 1) {
      throw new Error(`Build ${index} produced multiple outputs.`);
    }
    const output = result.outputs[0];
    await Bun.write(outfiles[index], await output.arrayBuffer(), {
      mode: 0o644,
      createPath: false,
    });
  })
);

console.log("Build completed.");
