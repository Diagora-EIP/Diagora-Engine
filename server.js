const express = require("express");
const cors = require("cors");
const fs = require("fs");
const crypto = require("crypto");
const util = require("node:util");
const exec = util.promisify(require("node:child_process").exec);
const app = express();
app.use(cors());
app.use(express.json());
const port = 9876;

deleteJsonFiles = () => {
  fs.readdir(".", (err, files) => {
    if (err) throw err;

    for (const file of files) {
      if (file.endsWith(".json") && !file.startsWith("package")) {
        fs.unlinkSync(file);
      }
    }
  });
}

parseCrashJson = (crashJson) => {
  let content = crashJson.split("\n");
  let reasonCrash = "";

  for (let i = 0; i < content.length; i++) {
    if (content[i].includes("Error:")) {
      reasonCrash = content[i];
      break;
    }
  }
  return reasonCrash;
}


app.post("/launch_itinary/", async (req, res) => {
  let fileName = crypto.randomBytes(20).toString("hex") + ".json";
  fs.writeFileSync(fileName, JSON.stringify(req.body));
  try {
    const { stdout, stderr } = await exec(`cargo run -- --createItinary ${fileName}`);
    const content = fs.readFileSync(
      stdout.replaceAll('"', "").replace("\n", ""),
      { encoding: "utf8", flag: "r" }
    );
    res.send(JSON.parse(content));
    deleteJsonFiles();
  } catch (error) {
    deleteJsonFiles();
    res.send({code: 1,  error: "Error while launching Engine check your json input", reason: parseCrashJson(error.stderr)});
  }
});

app.post("/update_itinary/", async (req, res) => {
  let fileName = crypto.randomBytes(20).toString("hex") + ".json";
  fs.writeFileSync(fileName, JSON.stringify(req.body));
  try {
    const { stdout, stderr } = await exec(`cargo run -- --updateItinary ${fileName}`);
    const content = fs.readFileSync(
      stdout.replaceAll('"', "").replace("\n", ""),
      { encoding: "utf8", flag: "r" }
    );
    res.send(JSON.parse(content));
    deleteJsonFiles();
  } catch (error) {
    deleteJsonFiles();
    res.send({code: 1,  error: "Error while launching Engine check your json input", reason: parseCrashJson(error.stderr)});
  }
});

app.listen(port, () => {
  console.log("Ready to launch Evaluation on port: " + port);
});
