import QtQuick 2.15
import Continuum 1.0

Column {
    width: parent.width
    property bool expanded: false
    property var diskData: modelData

    Rectangle {
        width: parent.width
        height: 28
        color: mouseArea.containsMouse ? "#3a3a3a" : "transparent"

        Text {
            text: "[DSK] " + diskData.label + " (" + diskData.mount_path + ")"
            color: "white"
            anchors.verticalCenter: parent.verticalCenter
            anchors.left: parent.left
            anchors.leftMargin: 0
        }

        MouseArea {
            id: mouseArea
            anchors.fill: parent
            hoverEnabled: true
            onClicked: {
                parent.parent.expanded = !parent.parent.expanded
                controller.load_roots(diskData.disk_id)
            }
        }
    }

    // Roots
    ListView {
        visible: parent.expanded
        width: parent.width
        height: childrenRect.height
        clip: true
        model: controller.roots_json.length > 0 ? JSON.parse(controller.roots_json) : []
        delegate: RootItem { }
    }
}