
pub fn encode() {
    #[cfg(not(target_os = "linux"))]
    compile_error!("This program currently only works on linux");
}
