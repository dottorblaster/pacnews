use alpm::Alpm;

fn get_pacman_packages(db_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let pacman = Alpm::new("/", db_path)?;
    let db = pacman.localdb();
    let packages = db.pkgs().iter().map(|p| p.name().to_string()).collect();
    Ok(packages)
}

pub fn has_package_name(db_path: &str, text: &str) -> bool {
    match get_pacman_packages(db_path) {
        Ok(package_names) => package_names
            .iter()
            .any(|package_name| text.contains(package_name)),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_package_name() {
        let result = has_package_name("./tests/db", "archlinux-keyring");
        assert_eq!(result, true);
    }

    #[test]
    fn test_has_not_package_name() {
        let result = has_package_name("./tests/db", "pacnews");
        assert_eq!(result, false);
    }
}
