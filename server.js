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
      } else if (file.endsWith(".json_result") && !file.startsWith("package")) {
        fs.unlinkSync(file);
      }
    }
  });
}

parseCrashJson = (crashJson) => {
  console.log(crashJson);
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
    const { stdout, stderr } = await exec(`cd ItinaryEngine && cargo run -- --createItinary ../${fileName} && cd ../`);
    const content = fs.readFileSync("./ItinaryEngine/" +
      stdout.replaceAll('"', "").replace("\n", ""),
      { encoding: "utf8", flag: "r" }
    );
    res.send(JSON.parse(content));
    deleteJsonFiles();
  } catch (error) {
    deleteJsonFiles();
    console.log(error);
    res.send({code: 1,  error: "Error while launching Engine check your json input", reason: parseCrashJson(error.stderr)});
  }
});

app.post("/update_itinary/", async (req, res) => {
  let fileName = crypto.randomBytes(20).toString("hex") + ".json";
  fs.writeFileSync(fileName, JSON.stringify(req.body));
  try {
    const { stdout, stderr } = await exec(`cd ItinaryEngine && cargo run -- --updateItinary ../${fileName} && cd ../`);
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

app.post("/missing_deliverer/", async (req, res) => {
  // console.log("request", req);
  // console.log("Request Body:", req.body);
  // console.log("Request Headers:", req.headers);
  // console.log("Request Method:", req.method);

  let fileName = crypto.randomBytes(20).toString("hex") + ".json";
  fs.writeFileSync(fileName, JSON.stringify(req.body));
  // let { stdout, stderr } = {};
  try {
    ({ stdout, stderr } = await exec(`python3 ./MissingEngine/src/main.py ./${fileName}`));
    // console.log("stdout", stdout);
    // console.log("stderr", stderr);
    const content = fs.readFileSync(
      stdout.replaceAll('"', "").replace("\n", ""),
      { encoding: "utf8", flag: "r" }
    );
    // console.log("Content", content);
    res.send(JSON.parse(content));
    deleteJsonFiles();
  } catch (error) {
    // console.log("error", error);
    deleteJsonFiles();
    res.send({code: 1,  error: "Error while launching Engine check your json input", reason: 'error'});
  }
});

app.post("/delivery_forecast/", async (req, res) => {
  try {
    fs.unlink('schedule.json', (err) => {
        if (err) {
            console.error('Error deleting file:', err);
        } else {
            console.log('Successfully deleted schedule.json');
        }
    });
  } catch (err) {
    console.error(err)
  }
  try {
    const { stdout, stderr } = await exec(`python3 ./DeliveryForeCastEngine/src/main.py`);
    console.log(stdout, stderr)

  fs.readFile('schedule.json', 'utf8', (err, data) => {
      if (err) {
          console.error('Error reading file:', err);
          res.status(500).send({ message: 'An error occurred.', error: err.toString() });
      } else {
          const jsonData = JSON.parse(data);
          console.log(jsonData);
          res.send(jsonData);
      }
  });
  } catch (error) {
    console.log(error);
    res.send({code: 1,  error: "Error while launching Engine check your json input", reason: parseCrashJson(error.stderr)});
  }
});

app.listen(port, () => {
  console.log("Ready to launch Evaluation on port: " + port);
});
