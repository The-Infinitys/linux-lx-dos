#ifndef WELCOMEWINDOW_H
#define WELCOMEWINDOW_H

#include <QWidget>
#include <QStackedWidget>
#include <QPushButton>
#include <QProgressBar>
#include <QLabel>
#include <QFile>
#include <QTextStream>
#include <QRegularExpression>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QTextBrowser>

namespace Ui {
class WelcomeWindow;
}

class WelcomeWindow : public QWidget
{
    Q_OBJECT

public:
    explicit WelcomeWindow(QWidget *parent = nullptr);
    ~WelcomeWindow();

private slots:
    void on_nextButton_clicked();
    void on_previousButton_clicked();

private:
    Ui::WelcomeWindow *ui;
    QStackedWidget *stackedWidget;
    QPushButton *nextButton;
    QPushButton *previousButton;
    QProgressBar *progressBar;
    QLabel *pageIndicatorLabel;

    QList<QWidget*> pages;
    int currentPageIndex;

    void loadMarkdownContent(const QString &filePath);
    void updateNavigationButtons();
    void updateProgressBar();
};

#endif // WELCOMEWINDOW_H