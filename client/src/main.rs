mod controller;
mod api;

use qmetaobject::prelude::*;
use qmetaobject::QmlEngine;

fn main() {
    qml_register_type::<controller::Controller>(
        &std::ffi::CString::new("Continuum").unwrap(),
        1, 0,
        &std::ffi::CString::new("Controller").unwrap()
    );

    let mut engine = QmlEngine::new();
    engine.load_file("qml/main.qml".into());
    engine.exec();
}