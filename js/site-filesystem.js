const fs = require("fs");
const folderName = "../style/style-1";
console.log(fs.readdirSync(folderName));

fs.readdirSync(folderName).map((folderName) => {
  return path.join(folderPath, fileName);
});


try {
  if (!fs.existsSync(folderName)) {
    //fs.mkdirSync(folderName);
  }
} catch (err) {
  console.error(err);
}
