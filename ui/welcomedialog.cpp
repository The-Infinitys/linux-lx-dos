#include "welcomedialog.h"
#include "ui_welcomedialog.h"
#include <QFileDialog>

WelcomeDialog::WelcomeDialog(QWidget *parent)
    : QDialog(parent), ui(new Ui::WelcomeDialog) {
  ui->setupUi(this);
  // The OK button is initially disabled
  ui->buttonBox->button(QDialogButtonBox::Ok)->setEnabled(false);

  // Enable the OK button only when the line edit is not empty
  connect(ui->pathLineEdit, &QLineEdit::textChanged, this, [this](const QString &text) {
    ui->buttonBox->button(QDialogButtonBox::Ok)->setEnabled(!text.isEmpty());
  });
}

WelcomeDialog::~WelcomeDialog() { 
    delete ui;
}

QString WelcomeDialog::getDiskImagePath() const { 
    return ui->pathLineEdit->text();
}

void WelcomeDialog::on_browseButton_clicked() {
  QString filePath = QFileDialog::getOpenFileName(
      this, tr("Select Disk Image"), QDir::homePath(),
      tr("QEMU Disk Images (*.qcow2 *.img *.vdi *.vmdk);;All Files (*)"));

  if (!filePath.isEmpty()) {
    ui->pathLineEdit->setText(filePath);
  }
}
