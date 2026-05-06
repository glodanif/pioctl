pub trait DesktopManager {
    fn get_monitors(&self) -> Result<(), ()>;
}
