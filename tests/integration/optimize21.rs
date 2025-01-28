// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let format = Format::new().set_bold().set_italic();

    // Constant memory worksheet.
    let worksheet = workbook.add_worksheet_with_constant_memory();
    worksheet.write_number_with_format(0, 0, 123, &format)?;
    worksheet.set_portrait();

    // Standard in-memory worksheet.
    let worksheet = workbook.add_worksheet();
    worksheet.write_number_with_format(0, 0, 123, &format)?;
    worksheet.set_portrait();

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize21() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize21")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
