// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Formula, Table, TableColumn, TableFunction, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_low_memory();

    worksheet.set_column_width(2, 10.288)?;
    worksheet.set_column_width(3, 10.288)?;
    worksheet.set_column_width(4, 10.288)?;
    worksheet.set_column_width(5, 10.288)?;

    worksheet.write(0, 0, "Column1")?;
    worksheet.write(0, 1, "Column2")?;
    worksheet.write(0, 2, "Column3")?;
    worksheet.write(0, 3, "Column4")?;
    worksheet.write(0, 4, "Total")?;

    let columns = vec![
        TableColumn::new().set_total_label("Total"),
        TableColumn::new().set_total_function(TableFunction::Custom(Formula::new("BASE(0,2)"))),
        TableColumn::new().set_total_function(TableFunction::Custom("SUM([Column3])".into())),
        TableColumn::new().set_total_function(TableFunction::Count),
    ];

    let table = Table::new().set_columns(&columns).set_total_row(true);

    worksheet.add_table(2, 2, 13, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize_table36() {
    let test_runner = common::TestRunner::new()
        .set_name("table36")
        .set_function(create_new_xlsx_file)
        .ignore_calc_chain()
        .unique("optimize")
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
