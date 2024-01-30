import { CONFIG } from "./config.js";

async function install() {
  const { bin: binaryDirectory, url: binarySource, name: binaryName } = CONFIG;
  console.log(`
    ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏
    Downloading ${binaryName} from ${binarySource}.\n
    Installing the binary at ${binaryDirectory}
    `);
}

install()
  .then(() => process.exit(0))
  .catch((err) => {
    console.log("Application Error: ", err.message);
    process.exit(1);
  });
