import QtQuick 2.2
import QtQuick.Controls 1.1
import QtQuick.Layouts 1.1
import QtQuick.Window 2.1

Item {
  TextField {
    id: searchField
    width: parent.width
    visible: true

    onTextChanged: {
      if(text.length > 0 ) {
        model.applyFilter(text);
      } else {
        model.reload();
      }
    }
  }

  ListView {
    id: contactListView
    clip: true
    width: parent.width
    height: parent.height
    visible: true

    model: ListModel {
      id: contactsModel
      Component.onCompleted: {
        reload();
      }

      function reload() {
        var contacts = DB.get_all_contacts();
        model.clear();
        for( var i=0; i < contacts.length ; ++i ) {
          model.append(contacts[i]);
        }
      }

      function applyFilter(bookName) {
        var contacts = DB.search_contacts(bookName);
        model.clear();
        for( var i=0; i < contacts.length ; ++i ) {
          model.append(contacts[i]);
        }
      }
    }

    delegate: Text {
      text: name + ": " + email
    }
  }
}
