use crate::client;

pub fn print_disks(disks: &[client::DiskInfo], active_disk_id: Option<&str>) {
    if disks.is_empty() {
        println!("Нет зарегистрированных дисков");
    } else {
        println!("{:<1} {:<36} {:<20} {:<30} {:<10} {:<10}", "", "ID", "LABEL", "PATH", "TYPE", "AVAILABLE");
        println!("{}", "-".repeat(108));
        for d in disks {
            let mark = if Some(d.disk_id.as_str()) == active_disk_id { "*" } else { " " };
            println!("{:<1} {:<36} {:<20} {:<30} {:<10} {:<10}",
                mark,
                d.disk_id,
                d.label,
                d.mount_path,
                d.disk_type,
                if d.is_available { "[OK]" } else { "[--]" }
            );
        }
    }
}

pub fn print_roots(roots: &[client::RootInfo], active_root_id: Option<&str>) {
    if roots.is_empty() {
        println!("Нет медиа-папок для диска");
    } else {
        println!("{:<1} {:<36} {:<30}", " ", "ID", "PATH");
        println!("{}", "-".repeat(68));
        for r in roots {
            let mark = if Some(r.id.as_str()) == active_root_id { "*" } else { " " };
            println!("{:<1} {:<36} {:<30}", mark, r.id, r.relative_path);
        }
    }
}