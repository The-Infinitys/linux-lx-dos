#ifndef WELCOMEDIALOG_H
#define WELCOMEDIALOG_H

#include <QDialog>

namespace Ui {
class WelcomeDialog;
}

class WelcomeDialog : public QDialog {
  Q_OBJECT

public:
  explicit WelcomeDialog(QWidget *parent = nullptr);
  ~WelcomeDialog();

  QString getDiskImagePath() const;

private slots:
  void on_browseButton_clicked();

private:
  Ui::WelcomeDialog *ui;
};

#endif // WELCOMEDIALOG_H
