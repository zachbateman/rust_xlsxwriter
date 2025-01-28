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
    let bold = Format::new().set_bold();

    // Constant memory worksheet.
    let worksheet = workbook.add_worksheet_with_constant_memory();

    worksheet.set_row_format(0, &bold)?;
    worksheet.set_row_format(1, &bold)?;
    worksheet.set_row_format(2, &bold)?;
    worksheet.set_row_format(4, &bold)?;

    worksheet.write_number(1, 0, 123)?;
    worksheet.write_number(3, 0, 123)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize29() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize29")
        .set_function(create_new_xlsx_file)
        .ignore_worksheet_spans()
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
