// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2025, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Image, Url, Workbook, XlsxError};

// Create a rust_xlsxwriter file to compare against an Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let url = Url::new("https://github.com/jmcnamara").set_tip("GitHub");
    let image = Image::new("tests/input/images/red.png")?
        .set_alt_text("red.png")
        .set_url(url)?;

    worksheet.insert_image(8, 4, &image)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_hyperlink33() {
    let test_runner = common::TestRunner::new()
        .set_name("hyperlink33")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
