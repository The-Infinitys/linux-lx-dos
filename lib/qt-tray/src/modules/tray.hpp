#ifndef QT_TRAY_HPP
#define QT_TRAY_HPP

#include <QObject>
#include <QSystemTrayIcon>
#include <QMenu>

namespace qt_tray {

class QtTray : public QObject {
    Q_OBJECT

public:
    explicit QtTray(QObject *parent = nullptr);
    ~QtTray();

    void setIcon(const QString &iconPath);
    void setToolTip(const QString &toolTip);
    QMenu* getMenu(); // Returns the menu associated with the tray icon

signals:
    void activated(QSystemTrayIcon::ActivationReason reason);

private slots:
    void onActivated(QSystemTrayIcon::ActivationReason reason);

private:
    QSystemTrayIcon *trayIcon;
    QMenu *contextMenu;
};

} // namespace qt_tray

#endif // QT_TRAY_HPP
