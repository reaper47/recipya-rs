import { execSync } from "node:child_process";
import { readdirSync } from "node:fs";

const files = readdirSync(".").filter((file) => file.endsWith(".js") && file !== "build.js").join(" ");

try {
  execSync(
    `uglifyjs ${files} --compress --mangle -o ../public/js/bundle.min.js`,
    { stdio: "inherit" },
  );
} catch (err) {
  console.error(`Failed to minify files`);
  process.exit(1);
}

console.log("Build complete!");
