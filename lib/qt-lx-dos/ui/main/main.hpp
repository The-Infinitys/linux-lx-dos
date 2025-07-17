#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QWidget>
#include "../task/tray.hpp"

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

private:
    Ui::Widget *ui;
    Tray *tray;
};

#endif // MAINWINDOW_H
