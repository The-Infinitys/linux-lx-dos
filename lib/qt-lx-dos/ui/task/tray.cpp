#include "tray.hpp"
#include "../../qrc_tray.cpp"
#include <QApplication>
#include <QAction>
#include <QMenu>
#include <QIcon>
#include <QDebug>
#include <QDirIterator>

Tray::Tray(QWidget *mainWindow, QObject *parent)
    : QObject(parent), mainWindow(mainWindow)
{
    createActions();
    createTrayIcon();

    connect(trayIcon, &QSystemTrayIcon::activated, this, &Tray::iconActivated);
}

Tray::~Tray()
{
    delete trayIcon;
    delete trayIconMenu;
}

void Tray::show()
{
    trayIcon->show();
}

void Tray::iconActivated(QSystemTrayIcon::ActivationReason reason)
{
    switch (reason)
    {
    case QSystemTrayIcon::Trigger:
    case QSystemTrayIcon::DoubleClick:
        showMainWindow();
        break;
    default:
        break;
    }
}

void Tray::showMainWindow()
{
    if (mainWindow) {
        mainWindow->showNormal();
        mainWindow->activateWindow();
    }
}

void Tray::quitApplication()
{
    QApplication::quit();
}

void Tray::createActions()
{
    QAction *showAction = new QAction(tr("&Open"), this);
    connect(showAction, &QAction::triggered, this, &Tray::showMainWindow);

    QAction *quitAction = new QAction(tr("&Quit"), this);
    connect(quitAction, &QAction::triggered, this, &Tray::quitApplication);

    trayIconMenu = new QMenu();
    trayIconMenu->addAction(showAction);
    trayIconMenu->addSeparator();
    trayIconMenu->addAction(quitAction);
}

void Tray::createTrayIcon()
{
    trayIcon = new QSystemTrayIcon(this);
    trayIcon->setIcon(QIcon(":/lx-dos/tray/icon.svg"));
    trayIcon->setToolTip(tr("LX-DOS"));
    trayIcon->setContextMenu(trayIconMenu);
}
