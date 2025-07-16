#include "c_api.h"
#include "mainwindow.h"
#include <QApplication>

void run_qt_app() {
    int argc = 0;
    char **argv = nullptr;
    QApplication a(argc, argv);
    Widget w;
    w.show();
    a.exec();
}
