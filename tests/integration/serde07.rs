// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{CustomSerializeHeader, Workbook, XlsxError};
use serde::Serialize;

// Test case for Serde serialization. First test isn't serialized.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Not serialized.
    worksheet.write(0, 0, "col1")?;
    worksheet.write(1, 0, 1)?;
    worksheet.write(2, 0, 2)?;
    worksheet.write(3, 0, 3)?;
    worksheet.write(0, 1, "col2")?;
    worksheet.write(1, 1, "aaa")?;
    worksheet.write(2, 1, "bbb")?;
    worksheet.write(3, 1, "ccc")?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for Serde serialization.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: u8,
        col2: &'static str,
    }

    let data = [
        MyStruct {
            col1: 1,
            col2: "aaa",
        },
        MyStruct {
            col1: 2,
            col2: "bbb",
        },
        MyStruct {
            col1: 3,
            col2: "ccc",
        },
    ];

    worksheet.serialize_headers(0, 0, &data.get(0).unwrap())?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for skipping fields.
fn create_new_xlsx_file_3(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    #[allow(dead_code)]
    struct MyStruct {
        col1: u8,
        col2: &'static str,
        #[serde(skip_serializing)]
        col3: bool,
    }

    let data = [
        MyStruct {
            col1: 1,
            col2: "aaa",
            col3: true,
        },
        MyStruct {
            col1: 2,
            col2: "bbb",
            col3: true,
        },
        MyStruct {
            col1: 3,
            col2: "ccc",
            col3: true,
        },
    ];

    worksheet.serialize_headers(0, 0, &data.get(0).unwrap())?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for serialize_headers_with_options().
fn create_new_xlsx_file_4(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: u8,
        col2: &'static str,
    }

    let data = [
        MyStruct {
            col1: 1,
            col2: "aaa",
        },
        MyStruct {
            col1: 2,
            col2: "bbb",
        },
        MyStruct {
            col1: 3,
            col2: "ccc",
        },
    ];

    let custom_headers = [
        CustomSerializeHeader::new("col1"),
        CustomSerializeHeader::new("col2"),
    ];

    worksheet.serialize_headers_with_options(0, 0, "MyStruct", &custom_headers)?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for skipping fields via serialize_headers_with_options().
fn create_new_xlsx_file_5(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: u8,
        col2: &'static str,
        col3: bool,
    }

    let data = [
        MyStruct {
            col1: 1,
            col2: "aaa",
            col3: true,
        },
        MyStruct {
            col1: 2,
            col2: "bbb",
            col3: true,
        },
        MyStruct {
            col1: 3,
            col2: "ccc",
            col3: true,
        },
    ];

    let custom_headers = [
        CustomSerializeHeader::new("col1"),
        CustomSerializeHeader::new("col2"),
    ];

    worksheet.serialize_headers_with_options(0, 0, "MyStruct", &custom_headers)?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for skipping fields via set_skip().
fn create_new_xlsx_file_6(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col1: u8,
        col2: &'static str,
        col3: bool,
    }

    let data = [
        MyStruct {
            col1: 1,
            col2: "aaa",
            col3: true,
        },
        MyStruct {
            col1: 2,
            col2: "bbb",
            col3: true,
        },
        MyStruct {
            col1: 3,
            col2: "ccc",
            col3: true,
        },
    ];

    let custom_headers = [
        CustomSerializeHeader::new("col1"),
        CustomSerializeHeader::new("col2"),
        CustomSerializeHeader::new("col3").set_skip(true),
    ];

    worksheet.serialize_headers_with_options(0, 0, "MyStruct", &custom_headers)?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for serialize_headers_with_options(). Field order is changed.
fn create_new_xlsx_file_7(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        col2: &'static str,
        col1: u8,
    }

    let data = [
        MyStruct {
            col2: "aaa",
            col1: 1,
        },
        MyStruct {
            col2: "bbb",
            col1: 2,
        },
        MyStruct {
            col2: "ccc",
            col1: 3,
        },
    ];

    let custom_headers = [
        CustomSerializeHeader::new("col1"),
        CustomSerializeHeader::new("col2"),
    ];

    worksheet.serialize_headers_with_options(0, 0, "MyStruct", &custom_headers)?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for field rename via Serde.
fn create_new_xlsx_file_8(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        #[serde(rename = "col1")]
        field1: u8,

        #[serde(rename = "col2")]
        field2: &'static str,
    }

    let data = [
        MyStruct {
            field1: 1,
            field2: "aaa",
        },
        MyStruct {
            field1: 2,
            field2: "bbb",
        },
        MyStruct {
            field1: 3,
            field2: "ccc",
        },
    ];

    let custom_headers = [
        CustomSerializeHeader::new("col1"),
        CustomSerializeHeader::new("col2"),
    ];

    worksheet.serialize_headers_with_options(0, 0, "MyStruct", &custom_headers)?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case for field rename via rename().
fn create_new_xlsx_file_9(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Create a serializable test struct.
    #[derive(Serialize)]
    struct MyStruct {
        field1: u8,
        field2: &'static str,
    }

    let data = [
        MyStruct {
            field1: 1,
            field2: "aaa",
        },
        MyStruct {
            field1: 2,
            field2: "bbb",
        },
        MyStruct {
            field1: 3,
            field2: "ccc",
        },
    ];

    let custom_headers = [
        CustomSerializeHeader::new("field1").rename("col1"),
        CustomSerializeHeader::new("field2").rename("col2"),
    ];

    worksheet.serialize_headers_with_options(0, 0, "MyStruct", &custom_headers)?;
    worksheet.serialize(&data)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_serde07_1() {
    let test_runner = common::TestRunner::new()
        .set_name("serde07")
        .set_function(create_new_xlsx_file_1)
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde07_2() {
    let test_runner = common::TestRunner::new()
        .set_name("serde07")
        .set_function(create_new_xlsx_file_2)
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
#[test]
fn test_serde07_3() {
    let test_runner = common::TestRunner::new()
        .set_name("serde07")
        .set_function(create_new_xlsx_file_3)
        .unique("3")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde07_4() {
    let test_runner = common::TestRunner::new()
        .set_name("serde07")
        .set_function(create_new_xlsx_file_4)
        .unique("4")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde07_5() {
    let test_runner = common::TestRunner::new()
        .set_name("serde07")
        .set_function(create_new_xlsx_file_5)
        .unique("5")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde07_6() {
    let test_runner = common::TestRunner::new()
        .set_name("serde07")
        .set_function(create_new_xlsx_file_6)
        .unique("6")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde07_7() {
    let test_runner = common::TestRunner::new()
        .set_name("serde07")
        .set_function(create_new_xlsx_file_7)
        .unique("7")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde07_8() {
    let test_runner = common::TestRunner::new()
        .set_name("serde07")
        .set_function(create_new_xlsx_file_8)
        .unique("8")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_serde07_9() {
    let test_runner = common::TestRunner::new()
        .set_name("serde07")
        .set_function(create_new_xlsx_file_9)
        .unique("9")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
