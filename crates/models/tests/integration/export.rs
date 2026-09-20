use models::export::ExportType;

#[test]
fn test_export() {
    for (export, expected) in [
        (ExportType::Json, "json"),
        (ExportType::Text, "txt"),
        (ExportType::Markdown, "md"),
        (ExportType::Pdf, "pdf"),
    ] {
        let got = export.extension();

        pretty_assertions::assert_eq!(got, expected);
    }
}
