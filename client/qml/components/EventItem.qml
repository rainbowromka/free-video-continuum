import QtQuick 2.15
import Continuum 1.0

Column {
    width: parent.width
    property bool expanded: false
    property var eventData: modelData

    Rectangle {
        width: parent.width
        height: 26
        color: "transparent"

        Text {
            text: "[EVT] " + eventData.folder_name
            color: "#aaaaaa"
            anchors.verticalCenter: parent.verticalCenter
            anchors.left: parent.left
            anchors.leftMargin: 16
        }

        MouseArea {
            anchors.fill: parent
            onClicked: {
                parent.parent.expanded = !parent.parent.expanded
                controller.load_cameras(eventData.id)
            }
        }
    }

    // Камеры
    ListView {
        visible: parent.expanded
        width: parent.width
        height: childrenRect.height
        clip: true
        model: controller.cameras_json.length > 0 ? JSON.parse(controller.cameras_json) : []
        delegate: CameraItem {}
    }
}