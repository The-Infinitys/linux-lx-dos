#include "qt-app.hpp"
#include "qt-tray.cpp"
#include <QApplication>
#include <QIcon>
#include <QBuffer>
#include <string>
#include <vector>
#include <memory>
#include <QDebug> // デバッグ用に追加

// --- 内部C++実装 ---

class QtAppWrapper {
public:
    // コンストラクタ: trayWrapperをnullptrで初期化
    QtAppWrapper() : trayWrapper(nullptr) {}

    // デストラクタ: Qtオブジェクトは通常Qtのイベントループによって管理されますが、
    // カスタムラッパーオブジェクトは明示的にクリーンアップします。
    ~QtAppWrapper() {
        if (trayWrapper) {
            delete trayWrapper; // トレイラッパーインスタンスを削除
            trayWrapper = nullptr;
        }
        // QApplicationはapp->exec()が終了したときにQt自身によって削除されます
    }

    // アプリケーションIDを設定
    void setAppId(const std::string& id) {
        appId = id;
    }

    // アプリケーションアイコンを設定
    void setAppIcon(const unsigned char* data, size_t size, const char* format) {
        // QByteArrayのコンストラクタはconst char*とintを期待するため、キャストが必要
        iconData = QByteArray(reinterpret_cast<const char*>(data), static_cast<int>(size));
        iconFormat = format;
    }

    // init_tray_icon C関数によって呼び出され、トレイ初期化を要求するフラグを設定
    void requestInitTray() {
        shouldInitTray = true;
    }

    // Qtアプリケーションイベントループを実行
    int run(int argc, char* argv[]) {
        qDebug() << "QtAppWrapper::run started.";
        app = new QApplication(argc, argv); // QApplicationを作成
        qDebug() << "QApplication created.";

        if (!appId.empty()) {
            app->setApplicationName(QString::fromStdString(appId));
            qDebug() << "Application ID set to:" << QString::fromStdString(appId);
        }

        QIcon appIcon;
        if (!iconData.isEmpty()) {
            QPixmap pixmap;
            if (pixmap.loadFromData(iconData, iconFormat.c_str())) {
                appIcon = QIcon(pixmap);
                app->setWindowIcon(appIcon);
                qDebug() << "Application icon loaded and set.";
            } else {
                qWarning() << "Failed to load application icon from data. Format:" << iconFormat.c_str();
            }
        } else {
            qDebug() << "No icon data provided for application.";
        }

        if (shouldInitTray) {
            qDebug() << "Tray initialization requested. Creating QtTrayWrapper.";
            // QApplicationポインタをトレイラッパーに渡してインスタンスを作成
            trayWrapper = new QtTrayWrapper(app);
            if (!iconData.isEmpty()) {
                // アプリケーションアイコンデータをトレイアイコンとして設定
                // ここで const char* を const unsigned char* にキャスト
                trayWrapper->setTrayIcon(reinterpret_cast<const unsigned char*>(iconData.constData()), iconData.size(), iconFormat.c_str()); // ★修正箇所
            }
            trayWrapper->initTray(); // トレイを初期化
            qDebug() << "QtTrayWrapper initialized.";

            // QApplicationとQMenuが準備できてから、保留中のメニューアイテムを追加
            for (const auto& item : pending_menu_items) {
                trayWrapper->addTrayMenuItem(item.first, item.second);
            }
            pending_menu_items.clear(); // 保留リストをクリア
        } else {
            qDebug() << "Tray initialization not requested.";
        }

        qDebug() << "Starting QApplication event loop...";
        return app->exec(); // Qtイベントループを開始
    }

    // イベントをポーリング
    AppEvent pollEvent() {
        if (trayWrapper) {
            QtTrayEvent trayEvent = trayWrapper->pollTrayEvent();
            // QtTrayEventをAppEventにマッピング
            AppEvent appEvent;
            appEvent.menu_id_str = trayEvent.menu_id_str; // 文字列の所有権を渡す
            switch (trayEvent.type) {
                case QtTrayEventType_None:
                    appEvent.type = AppEventType_None;
                    break;
                case QtTrayEventType_TrayClicked:
                    appEvent.type = AppEventType_TrayClicked;
                    break;
                case QtTrayEventType_TrayDoubleClicked:
                    appEvent.type = AppEventType_TrayDoubleClicked;
                    break;
                case QtTrayEventType_MenuItemClicked:
                    appEvent.type = AppEventType_MenuItemClicked;
                    break;
            }
            return appEvent;
        }
        return {AppEventType_None, nullptr}; // トレイラッパーがない場合はNoneイベントを返す
    }

    // トレイメニューアイテムを追加
    void addTrayMenuItem(const std::string& text, const std::string& id_str) {
        qDebug() << "Attempting to add tray menu item:" << QString::fromStdString(text) << "with ID:" << QString::fromStdString(id_str);
        if (trayWrapper && app) { // QApplicationとtrayWrapperが既に作成されている場合
            trayWrapper->addTrayMenuItem(text, id_str); // 直接QtTrayWrapperに追加
            qDebug() << "Menu item added directly to QtTrayWrapper.";
        } else {
            // QApplicationまたはQtTrayWrapperがまだ作成されていない場合、保留リストに追加
            pending_menu_items.push_back({text, id_str});
            qDebug() << "QApplication or QtTrayWrapper not yet created, pending menu item.";
        }
    }

    // アプリケーションを終了
    void quitApp() {
        if (app) {
            app->quit();
        }
    }

private:
    // 設定
    std::string appId;
    QByteArray iconData;
    std::string iconFormat;
    bool shouldInitTray = false;
    std::vector<std::pair<std::string, std::string>> pending_menu_items; // run()より前に追加されたメニューアイテムを格納

    // Qtオブジェクト
    QApplication* app = nullptr; // このクラスが所有するのではなく、メインスレッドが所有

    // 管理されるQtTrayWrapperインスタンス
    QtTrayWrapper* trayWrapper;
};

// 不透明なハンドル (C++実装へのポインタ)
struct QtAppHandle {
    QtAppWrapper* impl;
};


// --- CスタイルAPI実装 ---

extern "C" {

QtAppHandle* create_qt_app() {
    return new QtAppHandle{new QtAppWrapper()};
}

void set_app_id(QtAppHandle* handle, const char* id) {
    if (handle && handle->impl) {
        handle->impl->setAppId(id);
    }
}

void set_app_icon_from_data(QtAppHandle* handle, const unsigned char* data, size_t size, const char* format) {
    if (handle && handle->impl) {
        handle->impl->setAppIcon(data, size, format);
    }
}

void init_tray_icon(QtAppHandle* handle) {
    if (handle && handle->impl) {
        handle->impl->requestInitTray();
    }
}

int run_qt_app(QtAppHandle* handle, int argc, char* argv[]) {
    if (handle && handle->impl) {
        return handle->impl->run(argc, argv);
    }
    return -1; // エラーコード
}

AppEvent poll_event(QtAppHandle* handle) {
    if (handle && handle->impl) {
        return handle->impl->pollEvent();
    }
    return {AppEventType_None, nullptr}; // 無効なハンドル
}

void quit_qt_app(QtAppHandle* handle) {
    if (handle && handle->impl) {
        handle->impl->quitApp();
    }
}

void cleanup_qt_app(QtAppHandle* handle) {
    if (handle) {
        delete handle->impl; // これによりtrayWrapperも削除される
        delete handle;
    }
}

void add_tray_menu_item(QtAppHandle* handle, const char* text, const char* id) {
    if (handle && handle->impl) {
        handle->impl->addTrayMenuItem(text, id);
    }
}


} // extern "C"