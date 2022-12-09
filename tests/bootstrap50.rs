// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test to demonstrate simple hyperlinks.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write_url(
        0,
        0,
        r"file:///C:\Documents and Settings\john\My Documents\desktop.ini",
    )?;

    worksheet.write_url_with_text(1, 0, r"file:///hyperlinks.xlsx", r"J:\hyperlinks.xlsx")?;

    worksheet.write_url(2, 0, r"file:///\\Vboxsvr\share\autofilter01.xlsx")?;

    worksheet.write_url_with_text(
        3,
        0,
        r"file:///../../../demo.xlsx",
        "file:///Users/John/demo.xlsx",
    )?;

    worksheet.write_url(4, 0, r"file://Book1.xlsx")?;

    worksheet.write_url_with_text(
        5,
        0,
        r"file:///../rust/currency_format.xlsx",
        r"file:///Users/John/rust/currency_format.xlsx",
    )?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap50_hyperlinks() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap50")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}