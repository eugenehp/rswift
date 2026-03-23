//! System information via Foundation.
//!
//! cargo run -p rswift-foundation --example sysinfo

fn main() {
    println!("=== System Info ===\n");

    let (major, minor, patch) = foundation::ProcessInfo::os_version();
    println!("Hostname:    {}", foundation::ProcessInfo::hostname());
    println!("OS Version:  {major}.{minor}.{patch}");
    println!("Process:     {}", foundation::ProcessInfo::process_name());
    println!("CPUs:        {} ({} active)",
        foundation::ProcessInfo::processor_count(),
        foundation::ProcessInfo::active_processor_count());
    println!("RAM:         {:.1} GB",
        foundation::ProcessInfo::physical_memory() as f64 / 1_073_741_824.0);
    println!("Uptime:      {:.0}s ({:.1} hours)",
        foundation::ProcessInfo::system_uptime(),
        foundation::ProcessInfo::system_uptime() / 3600.0);
    println!("Thermal:     {:?}", foundation::ProcessInfo::thermal_state());
    println!("Low Power:   {}", foundation::ProcessInfo::is_low_power_mode());

    println!("\n=== Directories ===\n");
    println!("Home:        {}", foundation::FileManager::home_directory());
    println!("Temp:        {}", foundation::FileManager::temp_directory());
    println!("Documents:   {:?}", foundation::FileManager::documents_directory());
    println!("Downloads:   {:?}", foundation::FileManager::downloads_directory());
    println!("Desktop:     {:?}", foundation::FileManager::desktop_directory());
    println!("AppSupport:  {:?}", foundation::FileManager::application_support_directory());
    println!("Caches:      {:?}", foundation::FileManager::caches_directory());

    println!("\n=== Bundle ===\n");
    println!("Main path:   {}", foundation::Bundle::main_path());
    println!("Identifier:  {:?}", foundation::Bundle::identifier());
}
