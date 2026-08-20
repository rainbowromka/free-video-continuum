import QtQuick 2.15
import Continuum 1.0

Column {
    width: parent.width
    property bool expanded: false
    property var rootData: modelData    

    Rectangle {
        width: parent.width
        height: 26
        color: "transparent"

        Text {
            text: "[ROT] " + rootData.relative_path
            color: "#cccccc"
            anchors.verticalCenter: parent.verticalCenter
            anchors.left: parent.left
            anchors.leftMargin: 8
        }

        MouseArea {
            anchors.fill: parent
            onClicked: {
                parent.parent.expanded = !parent.parent.expanded                
                controller.load_events(rootData.id)
            }
        }
    }

    // События
    ListView {
        visible: parent.expanded
        width: parent.width
        height: childrenRect.height
        clip: true
        model: controller.events_json.length > 0 ? JSON.parse(controller.events_json) : []
        delegate: EventItem { }
    }
}