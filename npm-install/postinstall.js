import { CONFIG } from "./config.js";
import https from "https";
import { createWriteStream } from "fs";
import * as fs from "fs/promises";
import { exec } from "child_process";
import os from "os";
import { pipeline } from "stream/promises";
import tar from "tar";

async function install() {
  const { url, name: binaryName } = CONFIG;
  const dest = `./node_modules/.bin/${binaryName}`;

  console.log(`Downloading 1 package from: ${url}.\n`);
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error("Failed fetching the binary: " + response.statusText);
  }
  console.log("Successfully Downloaded 1 package.\n");

  const tarFile = "downloaded.tar.gz";

  await fs.mkdir(dest, { recursive: true }, (err) => {
    console.log("There was an error", err.message);
    throw new Error("Operation Failed!");
  });

  await pipeline(response.body, createWriteStream(tarFile));
  await tar.x({ file: tarFile, cwd: dest });
  await fs.rm(tarFile);

  if (os.platform() !== "win32") {
    console.log("Was this hit ?", os.platform());
    makeExecutable(dest);
  }
}

function makeExecutable(path) {
  const command = `chmod +x ${path}`;
  exec(command, (error) => {
    if (error) {
      console.error(`Error making file executable: ${error}`);
      return;
    }
    console.log("File is now executable");
  });
}

install()
  .then(() => process.exit(0))
  .catch((err) => {
    console.log("Application Error: ", err.message);
    process.exit(1);
  });
