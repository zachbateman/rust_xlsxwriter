// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let bold = Format::new().set_bold();

    let worksheet = workbook.add_worksheet().set_name("Outline Columns")?;

    let headers = ["Month", "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Total"];
    let regions = ["North", "South", "East", "West"];
    let data = [
        [50, 20, 15, 25, 65, 80],
        [10, 20, 30, 50, 50, 50],
        [45, 75, 50, 15, 75, 100],
        [15, 15, 55, 35, 20, 50],
    ];

    worksheet.write_row(0, 0, headers)?;
    worksheet.write_column(1, 0, regions)?;
    worksheet.write_row_matrix(1, 1, data)?;

    worksheet
        .write_formula(1, 7, "=SUM(B2:G2)")?
        .set_formula_result(1, 7, "255");
    worksheet
        .write_formula(2, 7, "=SUM(B3:G3)")?
        .set_formula_result(2, 7, "210");
    worksheet
        .write_formula(3, 7, "=SUM(B4:G4)")?
        .set_formula_result(3, 7, "360");
    worksheet
        .write_formula(4, 7, "=SUM(B5:G5)")?
        .set_formula_result(4, 7, "190");
    worksheet
        .write_formula_with_format(5, 7, "=SUM(H2:H5)", &bold)?
        .set_formula_result(5, 7, "1015");

    worksheet.set_column_width(0, 10)?;
    worksheet.set_column_range_width(1, 6, 6)?;
    worksheet.set_column_width(7, 10)?;
    worksheet.set_column_format(0, &bold)?;
    worksheet.set_row_format(0, &bold)?;

    worksheet.group_columns(1, 6)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_outline03() {
    let test_runner = common::TestRunner::new()
        .set_name("outline03")
        .set_function(create_new_xlsx_file)
        .ignore_calc_chain()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
