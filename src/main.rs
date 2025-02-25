slint::include_modules!();
use slint::SharedString;

fn main() -> Result<(), slint::PlatformError> {
    let main_window = IDE::new()?; 

    main_window.run()
}
