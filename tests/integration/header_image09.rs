// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{HeaderImagePosition, Image, Note, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Worksheet 1.
    let worksheet1 = workbook.add_worksheet();
    worksheet1.set_portrait();
    worksheet1.set_default_note_author("John");

    worksheet1.write(0, 0, "Foo")?;

    let note = Note::new("Some text").add_author_prefix(false);
    worksheet1.insert_note(1, 1, &note)?;

    // Worksheet 2.
    let worksheet2 = workbook.add_worksheet();
    worksheet2.set_portrait();

    let image = Image::new("tests/input/images/red.jpg")?;

    worksheet2.set_header("&L&G");
    worksheet2.set_header_image(&image, HeaderImagePosition::Left)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_header_image09() {
    let test_runner = common::TestRunner::new()
        .set_name("header_image09")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
