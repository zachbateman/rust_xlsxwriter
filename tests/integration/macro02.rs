// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add the VBA macro file.
    workbook.add_vba_project("tests/input/macros/vbaProject03.bin")?;
    workbook.set_vba_name("MyWorkbook")?;

    let worksheet = workbook.add_worksheet();
    worksheet.set_vba_name("MySheet1")?;

    worksheet.write(0, 0, 123)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_macro02() {
    let test_runner = common::TestRunner::new()
        .set_name("macro02")
        .has_macros()
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
