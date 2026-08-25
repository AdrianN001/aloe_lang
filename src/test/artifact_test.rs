use crate::{
    artifact::artifact::Artifact,
    ast::{Parser, program},
    lexer::Lexer,
};

#[test]
fn test_basic_artifact() {
    let testcases = vec![
        (r#"
            val x = 5;
            val y = 10;
            val z = x + y;
            "#,),
        (r#"
            val a = 1;
            val b = 2;
            val c = a * b;
            "#,),
    ];

    testcases.iter().for_each(|(input,)| {
        let lexer = Lexer::new(input.to_string());
        let parser = Parser::new(lexer);
        let program = parser.into_a_program().expect("Failed to parse program");

        let artifact = program.to_artifact();
        let bytes = artifact.to_bytes().expect("failed to serialize artifact");

        assert!(
            bytes[0..4].eq(&[0x41, 0x4C, 0x4F, 0x45]),
            "Artifact magic number mismatch"
        );

        let deserialized_artifact =
            Artifact::from_bytes(&bytes).expect("Failed to deserialize artifact");
        let deserialized_program = program::Program::from_artifact(deserialized_artifact)
            .expect("Failed to convert artifact back to program");

        assert_eq!(
            deserialized_program.statements, program.statements,
            "Deserialized program does not match original"
        );
    });
}

// Regression test for a header offset bug: AloeVersion::to_bytes() only writes 1 byte,
// but the reader previously assumed 4, corrupting the postcard payload for any
// program complex enough to expose the shift (functions, structs, enums, control flow).
#[test]
fn test_artifact_roundtrip_with_complex_program() {
    let input = r#"
        fun add(a, b){
            return a + b;
        }

        struct Point{
            fun new(this, x, y){}
        }

        enum Color{
            Red;
            Green;
            Blue;
        }

        let items = [1, 2, 3];
        let mapping = {"a": 1, "b": 2};

        if items.length > 0 {
            let total = 0;
            for item <- items {
                total = total + item;
            }
        } else {
            let total = 0;
        }

        let i = 0;
        while i < 3 {
            i = i + 1;
        }
    "#;

    let lexer = Lexer::new(input.to_string());
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().expect("Failed to parse program");

    let artifact = program.to_artifact();
    let bytes = artifact.to_bytes().expect("failed to serialize artifact");

    let deserialized_artifact =
        Artifact::from_bytes(&bytes).expect("Failed to deserialize artifact");
    let deserialized_program = program::Program::from_artifact(deserialized_artifact)
        .expect("Failed to convert artifact back to program");

    assert_eq!(
        deserialized_program.statements, program.statements,
        "Deserialized program does not match original"
    );
}

#[test]
fn test_artifact_from_bytes_rejects_too_short_input() {
    let result = Artifact::from_bytes(&[0x41, 0x4C, 0x4F]);
    assert!(
        result.is_err(),
        "expected error for too-short artifact bytes"
    );
}

#[test]
fn test_artifact_from_bytes_rejects_wrong_magic_number() {
    let mut bytes = vec![0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x01];
    bytes.extend_from_slice(&[0, 0]);

    let result = Artifact::from_bytes(&bytes);
    assert!(result.is_err(), "expected error for incorrect magic number");
}

#[test]
fn test_artifact_from_bytes_rejects_corrupted_program_payload() {
    let input = r#"val x = 5;"#;
    let lexer = Lexer::new(input.to_string());
    let parser = Parser::new(lexer);
    let program = parser.into_a_program().expect("Failed to parse program");

    let artifact = program.to_artifact();
    let mut bytes = artifact.to_bytes().expect("failed to serialize artifact");

    // corrupt the postcard payload while keeping the header intact
    let payload_start = 7;
    for byte in bytes.iter_mut().skip(payload_start) {
        *byte = 0xFF;
    }

    let result = Artifact::from_bytes(&bytes);
    assert!(
        result.is_err(),
        "expected error for corrupted program payload"
    );
}
