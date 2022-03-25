function generate_sheet() {
  var sheet = 
     SpreadsheetApp.getActiveSpreadsheet().getSheetByName('Sheet1');
  var sheet2 = 
     SpreadsheetApp.getActiveSpreadsheet().getSheetByName('Sheet2');
  var data = sheet.getDataRange().getValues();  
  var myRA = new Array(2); 
  for (var i = 0; i < data.length; i++)  {
    //GET IAVA No
    var title = data[i][0];
    var dx = data[i][1];
    var diff = data[i][2];
    var jacket = data[i][3];

    if (dx == "DX") {
      if (diff == "BAS") {
        var level = "DXBAS"
      }
      else if (diff == "ADV") {
        var level = "DXADV"
      }
      else if (diff == "EXP") {
        var level = "DXEXP"
      }
      else if (diff == "MAS") {
        var level = "DXMAS"
      }
      else if (diff == "REM") {
        var level = "DXREM"
      }
    }
    else if (dx == "STD") {
      if (diff == "BAS") {
        var level = "BASIC"
      }
      else if (diff == "ADV") {
        var level = "ADVANCED"
      }
      else if (diff == "EXP") {
        var level = "EXPERT"
      }
      else if (diff == "MAS") {
        var level = "MASTER"
      }
      else if (diff == "REM") {
        var level = "Re:MAS"
      }
    }

    let image = SpreadsheetApp
                .newCellImage()
                .setSourceUrl(jacket)
                .build();

    let row_base = 5 + Math.floor(i / 10) * 7;
    let col_base = 3 + i % 10 * 3;
    
    sheet2.getRange(row_base + 1, col_base + 1).setValue(image);  
    sheet2.getRange(row_base + 3, col_base + 1).setValue(title);
    sheet2.getRange(row_base + 4, col_base + 1).setValue(level);

  }
}