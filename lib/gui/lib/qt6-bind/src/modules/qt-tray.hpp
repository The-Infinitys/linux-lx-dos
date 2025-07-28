#pragma once

#include <stddef.h> // For size_t

// QApplicationの前方宣言 (QtTrayWrapperが使用するため)
class QApplication;

#ifdef __cplusplus
// C++実装クラスの前方宣言
class QtTrayWrapper; // <--- ここでQtTrayWrapperを前方宣言

extern "C" {
#endif

// C++システムトレイ実装への不透明なポインタ
typedef struct QtTrayHandle QtTrayHandle;

// RustからポーリングできるイベントタイプのEnum
// FFI互換性のためCスタイルEnumを使用
enum QtTrayEventType {
    QtTrayEventType_None,
    QtTrayEventType_TrayClicked,
    QtTrayEventType_TrayDoubleClicked,
    QtTrayEventType_MenuItemClicked
};

// イベントデータを保持する構造体
typedef struct {
    QtTrayEventType type;
    const char* menu_id_str; // MenuItemClickedイベントの場合、文字列ID
} QtTrayEvent;

/**
 * @brief 新しいQtシステムトレイハンドルを作成します。
 * @param app_ptr QApplicationインスタンスへのポインタ (QtAppHandleによって管理されます)。
 * トレイが正しく機能するために必要です。
 * @return 作成されたQtTrayHandleへのポインタ。
 */
QtTrayHandle* create_qt_tray(void* app_ptr);

/**
 * @brief 生のバイナリデータからシステムトレイアイコンを設定します。
 *
 * @param handle トレイハンドル。
 * @param data 生のアイコンデータへのポインタ。
 * @param size データのサイズ（バイト単位）。
 * @param format アイコンデータのフォーマット (例: "PNG", "JPG", "SVG")。
 */
void set_tray_icon_from_data(QtTrayHandle* handle, const unsigned char* data, size_t size, const char* format);

/**
 * @brief システムトレイアイコンをメニューとともに初期化し、表示します。
 * この関数はQApplicationが作成された後に呼び出す必要があります。
 * @param handle トレイハンドル。
 */
void init_tray(QtTrayHandle* handle);

/**
 * @brief システムトレイアイコンのコンテキストメニューにメニューアイテムを追加します。
 *
 * @param handle トレイハンドル。
 * @param text メニューアイテムに表示するテキスト。
 * @param id クリックを識別するためのユニークな文字列ID。
 */
void add_tray_menu_item_to_tray(QtTrayHandle* handle, const char* text, const char* id);

/**
 * @brief Qtシステムトレイから次のイベントをポーリングします。
 * @param handle トレイハンドル。
 * @return 次のQtTrayEvent。イベントがない場合はQtTrayEventType_None。
 */
QtTrayEvent poll_tray_event(QtTrayHandle* handle);

/**
 * @brief トレイハンドルに関連付けられたすべてのリソースをクリーンアップします。
 * @param handle トレイハンドル。
 */
void cleanup_qt_tray(QtTrayHandle* handle);

#ifdef __cplusplus
}
#endif