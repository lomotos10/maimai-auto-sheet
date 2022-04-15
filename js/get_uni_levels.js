const levelsModule = require('./levels');
// levels.js:
// module.exports = {
//     levels: function() {
//         var in_lv = ..
//     }
// }
const fs = require('fs');

const path = '../data/intl_lv_info.csv'

fs.writeFile(path, "", err => {
    if (err) {
      console.error(err)
      return
    }
    //file written successfully
  });

let levels = levelsModule.levels();

for (const song of levels) {
    var dx = song.dx;
    var lvs = song.lv;
    var name = song.n;

    for (const i of Array(6).keys()) {
        var cc = Math.abs(lvs[i]);
        if (cc === 0 || isNaN(cc)){
            continue;
        }
        var dx_st = "";
        if (dx === 1) {
            dx_st = "DX";
        } else {
            dx_st = "STD";
        }

        var lv_st = "";
        switch (i) {
            case 0:
                lv_st = "BAS";
                break;
            case 1:
                lv_st = "ADV";
                break;
            case 2:
                lv_st = "EXP";
                break;
            case 3:
                lv_st = "MAS";
                break;
            case 4:
            case 5:
                lv_st = "REM";
                break;
            default:
                process.exit(1);
                break;
        }

        fs.appendFile(path, name + '\t' + dx_st + '\t' + lv_st + '\t' + cc + '\n', err => {
            if (err) {
              console.error(err)
              return
            }
            //file written successfully
          });
    }
}