#ifndef TRAY_H
#define TRAY_H

#include <QSystemTrayIcon>
#include <QMenu>
#include <QWidget>

class Tray : public QObject
{
    Q_OBJECT

public:
    explicit Tray(QWidget *mainWindow, QObject *parent = nullptr);
    ~Tray();

    void show();

private slots:
    void iconActivated(QSystemTrayIcon::ActivationReason reason);
    void showMainWindow();
    void quitApplication();

private:
    void createTrayIcon();
    void createActions();

    QWidget *mainWindow;
    QSystemTrayIcon *trayIcon;
    QMenu *trayIconMenu;
};

#endif // TRAY_H
