// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, FormatAlign, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_constant_memory();

    let format = Format::new().set_align(FormatAlign::Center);

    worksheet.merge_range(1, 1, 5, 3, "", &format)?;

    worksheet.write(1, 5, 123)?;
    worksheet.write(2, 5, 123)?;
    worksheet.write(3, 5, 123)?;
    worksheet.write(4, 5, 123)?;
    worksheet.write(5, 5, 123)?;
    worksheet.write(6, 5, 123)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize26() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize26")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
