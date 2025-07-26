# TODO
qt6について、いくつか機能を追加したり、調整したりしてほしいです。

## QtWindow

QtWindowクラスについて、以下の要素を持つようにしてください。

- new()->Self (初期化)
- default()->Self (デフォルトの必要最低限の機能を備えたウィンドウ)
- start() 処理の開始。QtWindowInstanceを返す。以後、QtWindowInstanceを通じて処理を実行するようにする。生成は一度に一つまで。Instanceが破棄された際に、生成権が復活する
- run() スレッドを止めての処理の開始
- builder() QtWindowをレンダリングするためのQtWindowBuilderを提供する
- event_handler() QtWindowにイベントが発生した際の処理を書く。引数は関数。
...
## QtWindowBuilder

- new(), default()
- build() QtWindowを返す。
- append() QtElementを追加する
...

## QtElement

- new(), default()
- from(QtElementType)
- property(QtElementProperty::Name(value))
- append(QtElement) (追加できるものとできないものを分けておく。無理やり追加しようとしていた場合は、警告メッセージだけを発するようにする)
- event_handler() QtElementにEventが発生した場合の処理を書く。
...

## QtWindowInstance

- new() QtWindowを引数にする
- send_event(QtWindowEvent) イベントを送信する
- add_interval() 定期的に実行する処理を追加する。関数を引数とし、関数にはループを継続するかを返す戻り値を設定させる。また、最短実行間隔も設定しておく
- del_interval() 定期的に実行する処理を削除する
...

ほかは、必要そうな機能を細かく追加したり、src/tray.rsなどにコードを分けたりしてみてください。