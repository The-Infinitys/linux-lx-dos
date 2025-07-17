#include "main.hpp"
#include "ui_main.h"
#include <QCloseEvent>
#include "../../api/qt-lx-dos.hpp"

Widget::Widget(QWidget *parent)
    : QWidget(parent), ui(new Ui::Widget)
{
    ui->setupUi(this);
    tray = new Tray(this, this);
    tray->show();
}

Widget::~Widget()
{
    delete ui;
    delete tray;
}

void Widget::closeEvent(QCloseEvent *event)
{
    hide();
    event->ignore();
    if (global_event_callback) {
        global_event_callback("main_window_closed", "");
    }
}
