use alpm::Alpm;

fn get_pacman_packages() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pacman = Alpm::new("/", "/var/lib/pacman")?;
    let db = pacman.localdb();
    let packages = db.pkgs().iter().map(|p| p.name().to_string()).collect();
    Ok(packages)
}

pub fn has_package_name(text: &str) -> bool {
    match get_pacman_packages() {
        Ok(package_names) => package_names
            .iter()
            .any(|package_name| text.contains(package_name)),
        Err(_) => false,
    }
}
