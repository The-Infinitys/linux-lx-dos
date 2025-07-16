#include <QApplication>
#include <QSettings>
#include <QMessageBox>
#include "../ui/mainwindow.h"
#include "../ui/welcomedialog.h"

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);
    a.setOrganizationName("YourOrganization"); // Optional: good practice
    a.setApplicationName("Linux-LX-DOS");

    QSettings settings;

    // Check if it's the first launch or if the disk path is not set
    if (!settings.contains("diskImagePath")) {
        WelcomeDialog welcomeDialog;
        if (welcomeDialog.exec() == QDialog::Accepted) {
            QString diskPath = welcomeDialog.getDiskImagePath();
            if (diskPath.isEmpty()) {
                 QMessageBox::critical(nullptr, "Error", "Disk image path cannot be empty.");
                 return 1; // Exit if no path is provided
            }
            settings.setValue("diskImagePath", diskPath);
        } else {
            return 0; // User cancelled the dialog, exit application
        }
    }

    MainWindow w;
    w.show();

    return a.exec();
}