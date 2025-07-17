#include "window.hpp"
#include "ui_window.h"
#include <QCloseEvent>
#include <QMessageBox>
#include <QApplication>
#include <QDirIterator>
#include <QCoreApplication>

Widget::Widget(QWidget *parent)
    : QWidget(parent), ui(new Ui::Widget)
{
    ui->setupUi(this);
    createActions();
    createTrayIcon();

    connect(trayIcon, &QSystemTrayIcon::activated, this, &Widget::iconActivated);

    trayIcon->show();
}

Widget::~Widget()
{
    delete ui;
}

void Widget::closeEvent(QCloseEvent *event)
{
    if (trayIcon->isVisible())
    {
        QMessageBox::information(this, tr("LX-DOS"),
                                 tr("The program will continue to run in the system tray. To terminate the program, choose \"Quit\" in the context menu of the system tray icon."));
        hide();
        event->ignore();
    }
    else
    {
        event->accept();
    }
}

void Widget::iconActivated(QSystemTrayIcon::ActivationReason reason)
{
    switch (reason)
    {
    case QSystemTrayIcon::Trigger:
    case QSystemTrayIcon::DoubleClick:
        showNormalWindow();
        break;
    default:
        break;
    }
}

void Widget::showNormalWindow()
{
    showNormal();
    activateWindow();
}

void Widget::quitApplication()
{
    QApplication::quit();
}

void Widget::createActions()
{
    QAction *showNormal = new QAction(tr("&Open"), this);
    connect(showNormal, &QAction::triggered, this, &Widget::showNormalWindow);

    QAction *quitAction = new QAction(tr("&Quit"), this);
    connect(quitAction, &QAction::triggered, this, &Widget::quitApplication);

    trayIconMenu = new QMenu(this);
    trayIconMenu->addAction(showNormal);
    trayIconMenu->addSeparator();
    trayIconMenu->addAction(quitAction);
}

void Widget::createTrayIcon()
{
    QDirIterator it(":", QDirIterator::Subdirectories);
    while (it.hasNext())
    {
        qDebug() << it.next();
    }
    trayIcon = new QSystemTrayIcon(this);
    trayIcon->setIcon(QIcon(QCoreApplication::applicationDirPath() + "/icon.svg"));
    trayIcon->setToolTip(tr("LX-DOS"));
    trayIcon->setContextMenu(trayIconMenu);
}
