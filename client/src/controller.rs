use qmetaobject::prelude::*;

#[derive(Default, QObject)]
pub struct Controller {
    base: qt_base_class!(trait QObject),

    load_disks: qt_method!(fn(&mut self)),
    disks_json: qt_property!(QString; NOTIFY disks_changed),
    disks_changed: qt_signal!(),

    load_roots: qt_method!(fn(&mut self, disk_id: QString)),
    roots_json: qt_property!(QString; NOTIFY roots_changed),
    roots_changed: qt_signal!(),

    load_events: qt_method!(fn(&mut self, root_id: QString)),
    events_json: qt_property!(QString; NOTIFY events_changed),
    events_changed: qt_signal!(),

    load_cameras: qt_method!(fn(&mut self, event_id: QString)),
    cameras_json: qt_property!(QString; NOTIFY cameras_changed),
    cameras_changed: qt_signal!(),
}

impl Controller {
    pub fn new() -> Self {
        let mut c = Self::default();
        c.disks_json = "[]".into();
        c.roots_json = "[]".into();
        c
    }

    fn load_disks(&mut self) {
        match crate::api::fetch_disks() {
            Ok(disks) => {
                let json = serde_json::to_string(&disks).unwrap_or_default();
                self.disks_json = json.into();
                self.disks_changed();
            }
            Err(e) => eprintln!("Ошибка загрузки дисков: {}", e),
        }
    }

    fn load_roots(&mut self, disk_id: QString) {
        let disk_id = disk_id.to_string();
        match crate::api::fetch_roots(&disk_id) {
            Ok(roots) => {
                let json = serde_json::to_string(&roots).unwrap_or_default();
                println!("Roots loaded: {}", json);
                self.roots_json = json.into();
                self.roots_changed();
            }
            Err(e) => eprintln!("Ошибка загрузки roots: {}", e),
        }
    }

    fn load_events(&mut self, root_id: QString) {
        let root_id = root_id.to_string();
        println!("load_events called with: {}", root_id);
        match crate::api::fetch_events(&root_id) {
            Ok(events) => {
                let json = serde_json::to_string(&events).unwrap_or_default();
                println!("Events loaded: {}", json);
                self.events_json = json.into();
                self.events_changed();
            }
            Err(e) => eprintln!("Ошибка загрузки events: {}", e),
        }
    }

    fn load_cameras(&mut self, event_id: QString) {
        let event_id = event_id.to_string();
        match crate::api::fetch_cameras(&event_id) {
            Ok(cameras) => {
                let json = serde_json::to_string(&cameras).unwrap_or_default();
                self.cameras_json = json.into();
                self.cameras_changed();
            }
            Err(e) => eprintln!("Ошибка загрузки камер: {}", e),
        }
    }
}