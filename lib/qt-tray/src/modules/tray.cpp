#include "tray.hpp"

#include <QIcon>
#include <QCoreApplication>

namespace qt_tray {

QtTray::QtTray(QObject *parent) : QObject(parent) {
    trayIcon = new QSystemTrayIcon(this);
    contextMenu = new QMenu();
    trayIcon->setContextMenu(contextMenu);

    connect(trayIcon, &QSystemTrayIcon::activated, this, &QtTray::onActivated);

    trayIcon->show();
}

QtTray::~QtTray() {
    delete trayIcon;
    delete contextMenu;
}

void QtTray::setIcon(const QString &iconPath) {
    trayIcon->setIcon(QIcon(iconPath));
}

void QtTray::setToolTip(const QString &toolTip) {
    trayIcon->setToolTip(toolTip);
}

QMenu* QtTray::getMenu() {
    return contextMenu;
}

void QtTray::onActivated(QSystemTrayIcon::ActivationReason reason) {
    emit activated(reason);
}

} // namespace qt_tray
