#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QPointer>

QT_BEGIN_NAMESPACE
namespace Ui {
class MainWindow;
}
QT_END_NAMESPACE

// Forward declaration for the Rust handle
struct SharedVmHandle;

// C-style function pointers for Rust FFI
extern "C" {
SharedVmHandle *start_vm(const char *disk_image_path, void (*log_callback)(const char *));
void stop_vm(SharedVmHandle *handle);
}

class MainWindow : public QMainWindow {
  Q_OBJECT

public:
  MainWindow(QWidget *parent = nullptr);
  ~MainWindow();

signals:
  void logMessage(const QString &message);

private slots:
  void on_startButton_clicked();
  void on_stopButton_clicked();
  void appendLogMessage(const QString &message);

private:
  static void logMessageReceived(const char *message);

  Ui::MainWindow *ui;
  SharedVmHandle *m_vmHandle;
  static QPointer<MainWindow> s_instance;
};

#endif // MAINWINDOW_H
