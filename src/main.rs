slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let main_window = CodeEditor::new()?;

    main_window.run()
}
