#pragma once

#include <stddef.h> // For size_t
#include "qt-window.hpp"
#include "qt-element.hpp"
#include "qt-tray.hpp" // QtTrayEventの定義のために追加

#ifdef __cplusplus
extern "C" {
#endif

// Qtアプリケーションの不透明なポインタ
typedef struct QtAppHandle QtAppHandle;

// Qtトレイハンドルの前方宣言 (QtAppHandleが内部で管理するため)
typedef struct QtTrayHandle QtTrayHandle;

// RustからポーリングできるイベントタイプのEnum
// FFI互換性のためCスタイルEnumを使用
typedef enum AppEventType {
    AppEventType_None,
    AppEventType_TrayClicked,
    AppEventType_TrayDoubleClicked,
    AppEventType_MenuItemClicked
} AppEventType;

// イベントデータを保持する構造体
typedef struct {
    AppEventType type;
    const char* menu_id_str; // MenuItemClickedイベントの場合、文字列ID
} AppEvent;

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
 * @brief Qtアプリケーションから次のイベントをポーリングします。
 * @param handle アプリケーションハンドル。
 * @return 次のAppEvent。イベントがない場合はAppEventType_None。
 */
AppEvent poll_event(QtAppHandle* handle);

/**
 * @brief システムトレイアイコンを初期化します。
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

// Functions to create and manipulate Qt objects on the Qt thread


} // extern "C"