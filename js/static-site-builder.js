const fs = require('fs');
const dir = '../posts';

fs.readdir(dir, (err, files) => {
  console.log(files.length);
});
