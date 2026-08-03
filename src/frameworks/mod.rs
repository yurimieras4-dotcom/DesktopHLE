pub mod foundation {
    pub fn ns_log(message: &str) {
        println!("[Foundation::NSLog] {message}");
    }
}

pub mod app_kit {
    pub fn ns_application_main() {
        println!("[AppKit::NSApplicationMain] Inicializando interfaz principal...");
    }
}
