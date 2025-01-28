// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Table, TableStyle, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_column_range_width(2, 5, 10.288)?;

    let table = Table::new().set_style(TableStyle::None);

    worksheet.add_table(2, 2, 12, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table25() {
    let test_runner = common::TestRunner::new()
        .set_name("table25")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
