#pragma once

#include <stddef.h> // For size_t

#ifdef __cplusplus
extern "C" {
#endif

// Qtアプリケーションの不透明なポインタ
typedef struct QtAppHandle QtAppHandle;

// Qtトレイハンドルの前方宣言 (QtAppHandleが内部で管理するため)
typedef struct QtTrayHandle QtTrayHandle;

/**
 * @brief 新しいQtアプリケーションハンドルを作成します。
 * これには内部的なQtTrayHandleの作成も含まれます。
 * @return 作成されたQtAppHandleへのポインタ。
 */
QtAppHandle* create_qt_app();

/**
 * @brief アプリケーションIDを設定します。
 * @param handle アプリケーションハンドル。
 * @param id 設定するアプリケーションID文字列。
 */
void set_app_id(QtAppHandle* handle, const char* id);

/**
 * @brief 生のバイナリデータからアプリケーションアイコンを設定します。
 *
 * @param handle アプリケーションハンドル。
 * @param data 生のアイコンデータへのポインタ。
 * @param size データのサイズ（バイト単位）。
 * @param format アイコンデータのフォーマット (例: "PNG", "JPG", "SVG")。
 */
void set_app_icon_from_data(QtAppHandle* handle, const unsigned char* data, size_t size, const char* format);

/**
 * @brief Qtアプリケーションのイベントループを実行します。
 * これはブロッキングコールであり、Qtイベントループを開始します。
 * Qt GUIスレッドとして意図されたスレッドから呼び出す必要があります。
 * init_tray_iconが呼び出されている場合、システムトレイも初期化されます。
 * @param handle アプリケーションハンドル。
 * @param argc コマンドライン引数の数。
 * @param argv コマンドライン引数の配列。
 * @return アプリケーションの終了コード。
 */
int run_qt_app(QtAppHandle* handle, int argc, char* argv[]);

/**
 * @brief Qtアプリケーションのイベントループを終了します。
 * Qtイベントループを終了させるために、どのスレッドからでも呼び出すことができます。
 * @param handle アプリケーションハンドル。
 */
void quit_qt_app(QtAppHandle* handle);

/**
 * @brief ハンドルに関連付けられたすべてのリソースをクリーンアップします。
 * @param handle アプリケーションハンドル。
 */
void cleanup_qt_app(QtAppHandle* handle);


// --- システムトレイ固有の関数 (内部のQtTrayHandleに委譲) ---

/**
 * @brief システムトレイアイコンをメニューとともに初期化します。
 * この関数はrun_qt_appの前に呼び出す必要があります。
 * @param handle アプリケーションハンドル。
 */
void init_tray_icon(QtAppHandle* handle);

/**
 * @brief システムトレイアイコンのコンテキストメニューにメニューアイテムを追加します。
 *
 * @param handle アプリケーションハンドル。
 * @param text メニューアイテムに表示するテキスト。
 * @param id クリックを識別するためのユニークな文字列ID。
 */
void add_tray_menu_item(QtAppHandle* handle, const char* text, const char* id);

// RustからポーリングできるイベントタイプのEnum
// メインのアプリケーションハンドルがトレイからのイベントをポーリングするため、ここに移動。
enum AppEventType {
    AppEventType_None,
    AppEventType_TrayClicked,
    AppEventType_TrayDoubleClicked,
    AppEventType_MenuItemClicked
};

// イベントデータを保持する構造体
// メインのアプリケーションハンドルがトレイからのイベントをポーリングするため、ここに移動。
typedef struct {
    AppEventType type;
    const char* menu_id_str; // MenuItemClickedイベントの場合、文字列ID
} AppEvent;

/**
 * @brief Qtアプリケーション（特にトレイ）から次のイベントをポーリングします。
 * @param handle アプリケーションハンドル。
 * @return 次のAppEvent。イベントがない場合はAppEventType_None。
 */
AppEvent poll_event(QtAppHandle* handle);


#ifdef __cplusplus
}
#endif