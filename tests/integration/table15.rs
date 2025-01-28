// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Table, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let items = ["Foo", "Bar", "Baz", "Bop"];
    let data = [
        [1234, 2000, 4321],
        [1256, 0, 4320],
        [2234, 3000, 4332],
        [1324, 1000, 4333],
    ];

    worksheet.write_column(2, 2, items)?;
    worksheet.write_row_matrix(2, 3, data)?;

    worksheet.set_column_range_width(2, 5, 10.288)?;

    let table = Table::new();
    worksheet.add_table(1, 2, 5, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table15() {
    let test_runner = common::TestRunner::new()
        .set_name("table15")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
