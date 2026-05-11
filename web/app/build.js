import { execSync } from "node:child_process";

const files = [
  "core",
  "toast",
  "wakelock",
  "json-highlighter",
  "media",
  "timer",
  "gestures",
];

for (const file of files) {
  try {
    console.log(`Minifying ${file}...`);
    execSync(
      `uglifyjs ${file}.js --compress --mangle -o ../public/js/${file}.min.js`,
      { stdio: "inherit" },
    );
  } catch (err) {
    console.error(`Failed to minify ${file}`);
    process.exit(1);
  }
}

console.log("Build complete!");
