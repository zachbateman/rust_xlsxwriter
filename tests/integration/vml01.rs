// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Button, Note, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    worksheet.set_default_note_author("John");

    worksheet.write(0, 0, "Foo")?;

    let note = Note::new("Some text").add_author_prefix(false);
    worksheet.insert_note(1, 1, &note)?;

    let button = Button::new();
    worksheet.insert_button(3, 2, &button)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_vml01() {
    let test_runner = common::TestRunner::new()
        .set_name("vml01")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
