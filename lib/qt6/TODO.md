# TODO
qt6について、いくつか機能を追加したり、調整したりしてほしいです。

## QtWindow

QtWindowクラスについて、以下の要素を持つようにしてください。

- new()->Self (初期化) - **(DONE: Now private and created asynchronously via builder)**
- default()->Self (デフォルトの必要最低限の機能を備えたウィンドウ) - **(DONE: Creates a default window asynchronously)**
- start(app_instance: &crate::QtAppInstance)->QtWindowInstance (処理の開始。QtWindowInstanceを返す。以後、QtWindowInstanceを通じて処理を実行するようにする。生成は一度に一つまで。Instanceが破棄された際に、生成権が復活する) - **(DONE: Takes `&crate::QtAppInstance` and registers the window with the app instance)**
- run() スレッドを止めての処理の開始 - **(DONE: Removed: This method is no longer available)**
- builder()->QtWindowBuilder (QtWindowをレンダリングするためのQtWindowBuilderを提供する) - **(DONE: No longer takes `app_handle`)**
- event_handler() QtWindowにイベントが発生した際の処理を書く。引数は関数。
...
## QtWindowBuilder

- new(), default() - **(DONE)**
- build() QtWindowを返す。 - **(DONE)**
- append(QtElement) (QtElementを追加する) - **(DONE: Now takes `QtElement` by value and stores it)**
...

## QtElement

- new()->Self (初期化) - **(DONE: Now created asynchronously)**
- default()->Self (デフォルトの必要最低限の機能を備えた要素) - **(DONE: Creates a default element asynchronously)**
- from(QtElementType) - **(DONE: Creates an element from type asynchronously)**
- property(QtElementProperty::Name(value))
- append(QtElement) (追加できるものとできないものを分けておく。無理やり追加しようとしていた場合は、警告メッセージだけを発するようにする) - **(DONE: Implemented appending, warning for invalid types needs to be added in Rust)**
- event_handler() QtElementにEventが発生した場合の処理を書く。
...

## QtWindowInstance

- new() QtWindowを引数にする - **(DONE: Obtained via `QtWindow::start()` which takes `QtWindow` internally)**
- send_event(QtWindowEvent) イベントを送信する
- add_interval() 定期的に実行する処理を追加する。関数を引数とし、関数にはループを継続するかを返す戻り値を設定させる。また、最短実行間隔も設定しておく
- del_interval() 定期的に実行する処理を削除する
...

## QtTray

- new(app_handle: SafeQtAppHandle)->Self (初期化) - **(DONE)**
- with_icon(data: &'static [u8], format: &str)->Self (アイコンを設定) - **(DONE)**
- init()->Self (トレイアイコンを初期化し、表示) - **(DONE)**
- add_menu_item(&self, text: &str, id: &str)->Result<(), Qt6Error> (メニューアイテムを追加) - **(DONE)**
- poll_event(&self)->Result<QtAppEvent, Qt6Error> (イベントをポーリング) - **(DONE)**

**C++ Side Changes:**
- `create_qt_window_async` and `create_qt_element_async` in `qt-thread-executor.hpp` and `qt-thread-executor.cpp` no longer take an `app_handle` argument. - **(DONE)**
- Added `QtElementType_Widget` to `bindgen_api.hpp` and `qt-element.hpp`. - **(DONE)**
- Modified `QtElementWrapper` to handle `QtElementType_Widget` and added `QVBoxLayout`. - **(DONE)**
- Added `add_child_element_to_element` to `bindgen_api.hpp` and `qt-element.cpp`. - **(DONE)**

ほかは、必要そうな機能を細かく追加したり、src/tray.rsなどにコードを分けたりしてみてください.