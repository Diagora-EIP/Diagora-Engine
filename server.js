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

app.post("/launch_itinary/", async (req, res) => {
  let fileName = crypto.randomBytes(20).toString("hex") + ".json";
  fs.writeFileSync(fileName, JSON.stringify(req.body));
  const { stdout } = await exec(`cargo run -- --json ${fileName}`);
  console.log(stdout);
  const content = fs.readFileSync(
    stdout.replaceAll('"', "").replace("\n", ""),
    { encoding: "utf8", flag: "r" }
  );
  console.log(content);
  res.send(JSON.parse(content));
});

app.post("/update_itinary/", async (req, res) => {
  let fileName = crypto.randomBytes(20).toString("hex") + ".json";
  fs.writeFileSync(fileName, JSON.stringify(req.body));
  const { stdout } = await exec(`cargo run -- --json ${fileName}`);
  console.log(stdout);
  const content = fs.readFileSync(
    stdout.replaceAll('"', "").replace("\n", ""),
    { encoding: "utf8", flag: "r" }
  );
  console.log(content);
  res.send(JSON.parse(content));
});

app.listen(port, () => {
  console.log("Ready to launch Evaluation on port: " + port);
});
