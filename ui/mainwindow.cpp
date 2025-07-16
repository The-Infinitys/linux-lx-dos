#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QSettings>
#include <QDebug>

// Initialize the static instance pointer
QPointer<MainWindow> MainWindow::s_instance = nullptr;

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent), ui(new Ui::MainWindow), m_vmHandle(nullptr) {
  ui->setupUi(this);
  s_instance = this;

  // Connect the signal for logging to the slot that updates the UI
  connect(this, &MainWindow::logMessage, this, &MainWindow::appendLogMessage, Qt::QueuedConnection);

  connect(ui->startButton, &QPushButton::clicked, this, &MainWindow::on_startButton_clicked);
  connect(ui->stopButton, &QPushButton::clicked, this, &MainWindow::on_stopButton_clicked);

  ui->stopButton->setEnabled(false);
}

MainWindow::~MainWindow() {
  if (m_vmHandle) {
    stop_vm(m_vmHandle);
    m_vmHandle = nullptr;
  }
  delete ui;
}

void MainWindow::on_startButton_clicked() {
  if (m_vmHandle) {
    appendLogMessage("VM is already running.");
    return;
  }

  QSettings settings;
  QString diskImage = settings.value("diskImagePath").toString();

  if (diskImage.isEmpty()) {
    appendLogMessage("ERROR: Disk image path is not set.");
    return;
  }

  appendLogMessage("Starting VM via Rust backend...");
  
  // Convert QString to const char* for Rust FFI
  QByteArray diskImageBytes = diskImage.toUtf8();
  const char *diskImagePathCStr = diskImageBytes.constData();

  // Call the Rust function
  m_vmHandle = start_vm(diskImagePathCStr, &MainWindow::logMessageReceived);

  ui->startButton->setEnabled(false);
  ui->stopButton->setEnabled(true);
}

void MainWindow::on_stopButton_clicked() {
  if (!m_vmHandle) {
    appendLogMessage("VM is not running.");
    return;
  }

  appendLogMessage("Stopping VM via Rust backend...");
  stop_vm(m_vmHandle);
  m_vmHandle = nullptr; // The handle is consumed and freed in Rust

  appendLogMessage("Stop signal sent to VM.");

  ui->startButton->setEnabled(true);
  ui->stopButton->setEnabled(false);
}

// This is the C-style static callback function that Rust will call
void MainWindow::logMessageReceived(const char *message) {
  if (s_instance) {
    // Emit a signal to safely update the UI from the main thread
    emit s_instance->logMessage(QString::fromUtf8(message));
  }
}

// This slot appends the message to the text edit
void MainWindow::appendLogMessage(const QString &message) {
  ui->logOutput->appendPlainText(message);
}
