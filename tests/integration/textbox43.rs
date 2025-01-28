// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Shape, ShapeFormat, ShapeLine, ShapeLineDashType, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let textbox = Shape::textbox().set_text("This is some text").set_format(
        &ShapeFormat::new().set_line(&ShapeLine::new().set_dash_type(ShapeLineDashType::RoundDot)),
    );

    worksheet.insert_shape(8, 4, &textbox)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_textbox43() {
    let test_runner = common::TestRunner::new()
        .set_name("textbox43")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
