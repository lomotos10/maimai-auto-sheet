function generate_sheet() {
  var columnArray = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z","AA","AB","AC","AD","AE","AF","AG","AH","AI","AJ","AK","AL","AM","AN","AO","AP","AQ","AR","AS","AT","AU","AV","AW","AX","AY","AZ"];
  var sheet = 
     SpreadsheetApp.getActiveSpreadsheet().getSheetByName('Sheet1');
  var sheet2 = 
     SpreadsheetApp.getActiveSpreadsheet().getSheetByName('Sheet2');
  var sheet0 = SpreadsheetApp.getActiveSpreadsheet().getSheetByName('Sheet0');
  var blank = " ";
  var data = sheet.getDataRange().getValues();
  var n = 0;
  // for (var i = n*50; i < data.length; i++)  {
  for (var i = n*50 + 0; i < n*50 + 50; i++)  {
    var title = data[i][0];
    var dx = data[i][1];
    var diff = data[i][2];
    var jacket = data[i][3];

    // calculate alignment
    sheet0.getRange(8, 4 + 3*n).setValue(title);
    sheet0.autoResizeColumn(4 + 3*n);
    
    let width = sheet0.getColumnWidth(4 + 3*n);
    if(width <= 116) {
      var center = true;
    } else {
      var center = false;
    }
    // EN ideal width: "SPICY SWINGY STYLE"
    // reference for later: JP ideal width is "スーパーシンメトリー"

    if (dx == "DX") {
      if (diff == "BAS") {
        var level = "DXBAS"
      }
      if (diff == "ADV") {
        var level = "DXADV"
      }
      if (diff == "EXP") {
        var level = "DXEXP"
      }
      if (diff == "MAS") {
        var level = "DXMAS"
      }
      if (diff == "REM") {
        var level = "DXREM"
      }
    }
    else if (dx == "STD") {
      if (diff == "BAS") {
        var level = "BASIC"
      }
      if (diff == "ADV") {
        var level = "ADVANCED"
      }
      if (diff == "EXP") {
        var level = "EXPERT"
      }
      if (diff == "MAS") {
        var level = "MASTER"
      }
      if (diff == "REM") {
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
    sheet2.getRange(row_base + 3, col_base + 1).setValue("'" + title);
    if(center) {
      sheet2.getRange(row_base + 3, col_base + 1).setHorizontalAlignment("center");
      sheet2.getRange(row_base + 3, col_base + 2).setValue("");
    } else {
      sheet2.getRange(row_base + 3, col_base + 1).setHorizontalAlignment("left");
      sheet2.getRange(row_base + 3, col_base + 2).setValue(blank);
    }
    sheet2.getRange(row_base + 4, col_base + 1).setValue(level);

    sheet2.getRange(row_base + 5, col_base + 1)
      .setFormula("=IFERROR(VLOOKUP(" + columnArray[col_base] + (row_base+3)
      +"&\"$$\"&"+ columnArray[col_base+0] + (row_base+4)+", data!$B$3:$E$5999, 4, FALSE), \"N/A\")");

  }
}