// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Test to demonstrate row or column formatting. This test has an explicit
// format for the row/col intersection.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let bold = Format::new().set_bold();
    let mixed = Format::new().set_bold().set_italic();
    let italic = Format::new().set_italic();

    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, "Foo")?;

    worksheet.set_row_format(2, &bold)?;
    worksheet.set_column_format(2, &italic)?;

    worksheet.write_blank(2, 2, &mixed)?;

    workbook.save(filename)?;

    Ok(())
}

// Test to demonstrate row or column formatting. This test has an implicit
// format for the row/col intersection.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let bold = Format::new().set_bold();
    let italic = Format::new().set_italic();
    let default = Format::default();

    let worksheet = workbook.add_worksheet();

    worksheet.write(0, 0, "Foo")?;

    worksheet.set_row_format(2, &bold)?;
    worksheet.set_column_format(2, &italic)?;

    worksheet.write_blank(2, 2, &default)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_row_col_format15_1() {
    let test_runner = common::TestRunner::new()
        .set_name("row_col_format15")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_row_col_format15_2() {
    let test_runner = common::TestRunner::new()
        .set_name("row_col_format15")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
