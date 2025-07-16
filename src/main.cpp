#include <QApplication>
#include "../ui/mainwindow.h"

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);

    Widget w;
    w.show();

    return a.exec();
}
