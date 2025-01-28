// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet_with_constant_memory();

    let bold = Format::new().set_bold();
    let italic = Format::new().set_italic();
    let default = Format::default();

    worksheet.write_string_with_format(0, 0, "Foo", &bold)?;
    worksheet.write_string_with_format(1, 0, "Bar", &italic)?;

    let segments = [(&default, "a"), (&bold, "bc"), (&default, "defg")];
    worksheet.write_rich_string(2, 0, &segments)?;

    workbook.save(filename)?;

    Ok(())
}

// Test with standalone worksheet.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let mut worksheet = workbook.new_worksheet_with_constant_memory();

    let bold = Format::new().set_bold();
    let italic = Format::new().set_italic();
    let default = Format::default();

    worksheet.write_string_with_format(0, 0, "Foo", &bold)?;
    worksheet.write_string_with_format(1, 0, "Bar", &italic)?;

    let segments = [(&default, "a"), (&bold, "bc"), (&default, "defg")];
    worksheet.write_rich_string(2, 0, &segments)?;

    workbook.push_worksheet(worksheet);

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_optimize04_1() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize04")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_optimize04_2() {
    let test_runner = common::TestRunner::new()
        .set_name("optimize04")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
