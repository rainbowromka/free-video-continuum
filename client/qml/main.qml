import QtQuick 2.15
import QtQuick.Controls 2.15
import Continuum 1.0
import "components"

ApplicationWindow {
    visible: true
    width: 1280
    height: 720
    title: "Free Video Continuum"

    Controller {
        id: controller
        Component.onCompleted: controller.load_disks()
    }

    Row {
        anchors.fill: parent

        Rectangle {
            width: 250
            height: parent.height
            color: "#2b2b2b"

            Column {
                anchors.fill: parent
                anchors.margins: 8

                Text {
                    text: "Навигация"
                    color: "white"
                    font.bold: true
                    font.pixelSize: 16
                }

                Rectangle { height: 1; width: parent.width; color: "#3a3a3a" }

                ListView {
                    width: parent.width
                    height: parent.height * 0.65
                    clip: true
                    model: JSON.parse(controller.disks_json)
                    delegate: DiskItem { }
                }

                Rectangle { height: 1; width: parent.width; color: "#3a3a3a" }

                Text {
                    text: "Субклипы"
                    color: "white"
                    font.bold: true
                    font.pixelSize: 16
                }
            }
        }

        Rectangle {
            width: parent.width - 550
            height: parent.height
            color: "#1e1e1e"
            Text { text: "Видеоплеер"; color: "white"; anchors.centerIn: parent }
        }

        Rectangle {
            width: 300
            height: parent.height
            color: "#2b2b2b"
            Text { text: "Метаданные"; color: "white"; anchors.centerIn: parent }
        }
    }
}