#include "qt-lx-dos.hpp"
#include "../ui/main/window.hpp"
#include <QApplication>

void run_qt_app() {
    int argc = 0;
    char **argv = nullptr;
    QApplication a(argc, argv);
    QApplication::setQuitOnLastWindowClosed(false);
    Widget w;
    // w.show(); // ウィンドウは非表示で起動
    a.exec();
}
