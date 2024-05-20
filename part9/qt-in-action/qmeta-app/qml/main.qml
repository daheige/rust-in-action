import QtQuick 2.12;
import QtQuick.Controls 2.12;
import QtQuick.Window 2.12;
import QtQuick.Controls.Basic 2.12;

// 导入qmetaobject.myapp包
// 这个包名称和版本必须要和main.rs qml_register注册的名称一样
import qmetaobject.myapp 1.0;

Window {
    visible: true
    title: "Hello App"
    height: 480
    width: 640
    color: "#e4af79"

    // 自定义的Hello类型
    Hello {
        id: hello // 唯一标识
    }

    // 自定义的Rot类型
    Rot {
        id: rot // 唯一标识
        plain: "" // 存放md5加密之前的字符串
        secret: ""// 存放md5加密后的字符串
    }

    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
        /* space between widget */
        spacing: 10

        // 实现say hello功能
        Label {
            text: "please click this button"
            font.bold: true
        }

        Button {
            text: "Say Hello!"
            onClicked: {
                console.log("call say_hello fn from rust");
                // 调用Hello上面的say_hello方法
                hello.say_hello();
            }
        }

        // 实现md5加密功能
        Label {
            text: "please input text"
            font.bold: true
        }
        TextArea {
            placeholderText: qsTr("origin string")
            text: rot.plain
            onTextChanged: rot.plain = text
            background: Rectangle {
                implicitWidth: 400
                implicitHeight: 50
                radius: 3
                color: "#e2e8f0"
                border.color: "#21be2b"
            }
        }

        Button {
            text: "Md5 Encrypt"
            onClicked: {
                // qml支持js语法
                // console.log("plain: ", rot.plain);
                let secret = rot.md5(rot.plain);
                // console.log("secret: ", rot.secret);

                // 赋值后，就会自动填充 Label 中的text
                rot.secret = secret;
            }
        }

        TextArea {
            placeholderText: qsTr("md5 value")
            text: rot.secret
            background: Rectangle {
                implicitWidth: 400
                implicitHeight: 50
                radius: 3
                color: "#e2e8f0"
                border.color: "#21be2b"
            }
        }
    }
}