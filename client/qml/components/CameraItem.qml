import QtQuick 2.15
import Continuum 1.0

Column {
    width: parent.width
    property bool expanded: false
    property var cameraData: modelData

    Rectangle {
        width: parent.width
        height: 26
        color: "transparent"

        Text {
            text: "  [CAM] " + cameraData.camera_name
            color: "#888888"
            anchors.verticalCenter: parent.verticalCenter
            anchors.left: parent.left
            anchors.leftMargin: 24
        }

        MouseArea {
            anchors.fill: parent
            onClicked: {
                parent.parent.expanded = !parent.parent.expanded
                // TODO: controller.load_assets(cameraData.id)
            }
        }
    }

    // Видео — будут добавлены позже
}