#include "qt-tray.hpp"
#include <QApplication>
#include <QIcon>
#include <QMenu>
#include <QSystemTrayIcon>
#include <QBuffer>
#include <string>
#include <vector>
#include <memory>
#include <QDebug> // デバッグ用に追加

// --- トレイの内部C++実装 ---

class QtTrayWrapper {
public:
    // コンストラクタ: QApplicationインスタンスへのポインタを受け取る
    QtTrayWrapper(QApplication* app_instance) : app(app_instance), menu(nullptr), tray(nullptr) {
        if (!app) {
            qWarning() << "QtTrayWrapper created with null QApplication pointer!";
        }
    }

    // デストラクタ: Qtオブジェクトは通常Qtの所有権モデルによってクリーンアップされますが、
    // ここで明示的に削除することで、QtTrayWrapperが削除されたときに適切にクリーンアップされます。
    ~QtTrayWrapper() {
        if (tray) {
            tray->hide(); // 削除前に非表示にする
            delete tray;
            tray = nullptr;
        }
        if (menu) {
            delete menu;
            menu = nullptr;
        }
        // 'app'はQtAppWrapper/メインアプリケーションが所有しているため、ここでは削除しません。

        // キューに残っているmenu_id_strを解放
        for (const auto& event : event_queue) {
            if (event.type == QtTrayEventType_MenuItemClicked && event.menu_id_str) {
                free((void*)event.menu_id_str);
            }
        }
        event_queue.clear();
    }

    // トレイアイコンを設定
    void setTrayIcon(const unsigned char* data, size_t size, const char* format) {
        // QByteArrayのコンストラクタはconst char*とintを期待するため、キャストが必要
        iconData = QByteArray(reinterpret_cast<const char*>(data), static_cast<int>(size));
        iconFormat = format;
        if (tray) { // トレイが既に存在する場合、アイコンを更新
            QPixmap pixmap;
            if (pixmap.loadFromData(iconData, iconFormat.c_str())) {
                tray->setIcon(QIcon(pixmap));
                qDebug() << "Tray icon updated.";
            } else {
                qWarning() << "Failed to load tray icon from data for update. Format:" << format;
            }
        }
    }

    // トレイを初期化し、表示
    void initTray() {
        qDebug() << "QtTrayWrapper::initTray started.";
        if (!app) {
            qWarning() << "Cannot initialize tray: QApplication is null.";
            return;
        }
        if (!QSystemTrayIcon::isSystemTrayAvailable()) {
            qWarning() << "System tray is not available on this system.";
            return;
        }
        qDebug() << "System tray is available.";

        menu = new QMenu(); // 親なし (tray->setContextMenuによって管理される)
        QIcon trayIcon;
        if (!iconData.isEmpty()) {
            QPixmap pixmap;
            if (pixmap.loadFromData(iconData, iconFormat.c_str())) {
                trayIcon = QIcon(pixmap);
                qDebug() << "Tray icon loaded from data.";
            } else {
                qWarning() << "Failed to load tray icon from data. Format:" << iconFormat.c_str();
            }
        } else {
            qDebug() << "No tray icon data provided for tray.";
        }

        tray = new QSystemTrayIcon(trayIcon, app); // QApplicationを親とする
        tray->setContextMenu(menu);
        qDebug() << "Tray icon and menu created and linked.";

        // トレイアイコンのアクティベートシグナルを接続
        QObject::connect(tray, &QSystemTrayIcon::activated, [this](QSystemTrayIcon::ActivationReason reason) {
            if (reason == QSystemTrayIcon::Context) {
                qDebug() << "Tray icon context menu activated (right-click).";
            } else if (reason == QSystemTrayIcon::Trigger) {
                qDebug() << "Tray icon triggered (left-click).";
                event_queue.push_back({QtTrayEventType_TrayClicked, nullptr});
            } else if (reason == QSystemTrayIcon::DoubleClick) {
                qDebug() << "Tray icon double-clicked.";
                event_queue.push_back({QtTrayEventType_TrayDoubleClicked, nullptr});
            }
        });
        tray->show(); // トレイアイコンを表示
        qDebug() << "Tray icon shown.";

        // initTray()より前に追加された保留中のメニューアイテムを追加
        for (const auto& item : pending_menu_items) {
            addTrayMenuItem(item.first, item.second);
        }
        pending_menu_items.clear(); // 保留リストをクリア
    }

    // トレイイベントをポーリング
    QtTrayEvent pollTrayEvent() {
        if (event_queue.empty()) {
            return {QtTrayEventType_None, nullptr};
        }
        QtTrayEvent event = event_queue.front();
        event_queue.erase(event_queue.begin()); // イベントをキューから削除
        return event;
    }

    // トレイメニューアイテムを追加
    void addTrayMenuItem(const std::string& text, const std::string& id_str) {
        qDebug() << "Attempting to add tray menu item:" << QString::fromStdString(text) << "with ID:" << QString::fromStdString(id_str);
        if (!menu) {
            // メニューがまだ初期化されていない場合、保留リストに追加
            pending_menu_items.push_back({text, id_str});
            qDebug() << "Menu not yet initialized, pending menu item.";
            return;
        }

        QAction* action = menu->addAction(QString::fromStdString(text));
        // アクションのtriggeredシグナルを接続
        QObject::connect(action, &QAction::triggered, [this, id_str]() {
            qDebug() << "Menu item triggered:" << QString::fromStdString(id_str) << ", adding to event queue.";
            // 文字列を複製し、Rustが後で解放できるようにする
            char* id_cstr = strdup(id_str.c_str());
            event_queue.push_back({QtTrayEventType_MenuItemClicked, id_cstr});
        });
        qDebug() << "Successfully added tray menu item.";
    }

private:
    QApplication* app; // このクラスが所有するのではなく、参照として保持
    QByteArray iconData;
    std::string iconFormat;
    std::vector<QtTrayEvent> event_queue;
    std::vector<std::pair<std::string, std::string>> pending_menu_items; // initTray()より前に追加されたメニューアイテムを格納

    QMenu* menu;
    QSystemTrayIcon* tray;
};

// 不透明なハンドル (C++実装へのポインタ)
struct QtTrayHandle {
    QtTrayWrapper* impl;
};


// --- CスタイルAPI実装 (トレイ用) ---

extern "C" {

QtTrayHandle* create_qt_tray(void* app_ptr) {
    QApplication* app = static_cast<QApplication*>(app_ptr);
    return new QtTrayHandle{new QtTrayWrapper(app)};
}

void set_tray_icon_from_data(QtTrayHandle* handle, const unsigned char* data, size_t size, const char* format) {
    if (handle && handle->impl) {
        handle->impl->setTrayIcon(data, size, format);
    }
}

void init_tray(QtTrayHandle* handle) {
    if (handle && handle->impl) {
        handle->impl->initTray();
    }
}

void add_tray_menu_item_to_tray(QtTrayHandle* handle, const char* text, const char* id) {
    if (handle && handle->impl) {
        handle->impl->addTrayMenuItem(text, id);
    }
}

QtTrayEvent poll_tray_event(QtTrayHandle* handle) {
    if (handle && handle->impl) {
        return handle->impl->pollTrayEvent();
    }
    return {QtTrayEventType_None, nullptr}; // 無効なハンドル
}

void cleanup_qt_tray(QtTrayHandle* handle) {
    if (handle) {
        delete handle->impl;
        delete handle;
    }
}

} // extern "C"