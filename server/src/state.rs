pub struct ActiveState {
    pub disk_id: Option<String>,
    pub root_id: Option<String>,
}

impl ActiveState {
    pub fn new() -> Self {
        Self {
            disk_id: None,
            root_id: None,
        }
    }

    pub fn set_disk(&mut self, disk_id: String) {
        self.disk_id = Some(disk_id);
        self.root_id = None; // сбрасываем root при смене диска
    }

    pub fn set_root(&mut self, disk_id: String, root_id: String) {
        self.disk_id = Some(disk_id);
        self.root_id = Some(root_id);
    }
}