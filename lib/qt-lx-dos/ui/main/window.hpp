#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QWidget>
#include <QSystemTrayIcon>
#include <QMenu>

QT_BEGIN_NAMESPACE
namespace Ui { class Widget; }
QT_END_NAMESPACE

class Widget : public QWidget
{
    Q_OBJECT

public:
    explicit Widget(QWidget *parent = nullptr);
    ~Widget();

protected:
    void closeEvent(QCloseEvent *event) override;

private slots:
    void iconActivated(QSystemTrayIcon::ActivationReason reason);
    void showNormalWindow();
    void quitApplication();

private:
    Ui::Widget *ui;
    QSystemTrayIcon *trayIcon;
    QMenu *trayIconMenu;

    void createTrayIcon();
    void createActions();
};

#endif // MAINWINDOW_H
