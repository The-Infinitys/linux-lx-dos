#include "welcome.hpp"
#include "ui_welcome.h"

WelcomeWindow::WelcomeWindow(QWidget *parent) :
    QWidget(parent),
    ui(new Ui::WelcomeWindow),
    currentPageIndex(0)
{
    ui->setupUi(this);

    // Main layout
    QVBoxLayout *mainLayout = new QVBoxLayout(this);

    // Top section for markdown content
    stackedWidget = new QStackedWidget(this);
    mainLayout->addWidget(stackedWidget);

    // Progress bar and page indicator
    QHBoxLayout *progressLayout = new QHBoxLayout();
    progressBar = new QProgressBar(this);
    progressBar->setRange(0, 0); // Will be set dynamically
    progressBar->setTextVisible(false);
    progressLayout->addWidget(progressBar);

    pageIndicatorLabel = new QLabel(this);
    pageIndicatorLabel->setAlignment(Qt::AlignCenter);
    progressLayout->addWidget(pageIndicatorLabel);
    mainLayout->addLayout(progressLayout);

    // Navigation buttons
    QHBoxLayout *buttonLayout = new QHBoxLayout();
    previousButton = new QPushButton("Previous", this);
    nextButton = new QPushButton("Next", this);

    buttonLayout->addWidget(previousButton);
    buttonLayout->addStretch(); // Pushes buttons to ends
    buttonLayout->addWidget(nextButton);

    mainLayout->addLayout(buttonLayout);

    // Connect signals and slots
    connect(nextButton, &QPushButton::clicked, this, &WelcomeWindow::on_nextButton_clicked);
    connect(previousButton, &QPushButton::clicked, this, &WelcomeWindow::on_previousButton_clicked);

    // Load markdown content
    loadMarkdownContent("/home/the-infinitys/github/linux-lx-dos/lib/qt-lx-dos/ui/welcome/welcome.md");

    // Initial UI update
    updateNavigationButtons();
    updateProgressBar();
}

WelcomeWindow::~WelcomeWindow()
{
    delete ui;
}

void WelcomeWindow::loadMarkdownContent(const QString &filePath)
{
    QFile file(filePath);
    if (!file.open(QIODevice::ReadOnly | QIODevice::Text)) {
        // Handle error: file not found or cannot be opened
        qWarning("Could not open welcome.md");
        return;
    }

    QTextStream in(&file);
    QString content = in.readAll();
    file.close();

    // Extract H1 for window title
    QRegularExpression h1Regex("^#\s*(.*)", QRegularExpression::MultilineOption);
    QRegularExpressionMatch h1Match = h1Regex.match(content);
    if (h1Match.hasMatch()) {
        setWindowTitle(h1Match.captured(1).trimmed());
        content.remove(h1Match.capturedStart(0), h1Match.capturedLength(0)); // Remove H1 from content
    } else {
        setWindowTitle("Welcome"); // Default title if no H1
    }

    // Split content by H2 headings to create pages
    QRegularExpression h2Regex("^##\s*(.*)", QRegularExpression::MultilineOption);
    QStringList sections = content.split(h2Regex, Qt::SkipEmptyParts);

    // If there's content before the first H2, treat it as the first page
    if (!sections.isEmpty() && !h2Regex.match(sections.first()).hasMatch()) {
        QTextBrowser *textBrowser = new QTextBrowser(this);
        textBrowser->setMarkdown(sections.takeFirst().trimmed());
        stackedWidget->addWidget(textBrowser);
        pages.append(textBrowser);
    }

    // Process remaining sections as pages
    QRegularExpressionMatchIterator i = h2Regex.globalMatch(content);
    int sectionIndex = 0;
    while (i.hasNext()) {
        QRegularExpressionMatch match = i.next();
        if (sectionIndex < sections.size()) {
            QTextBrowser *textBrowser = new QTextBrowser(this);
            textBrowser->setMarkdown("## " + match.captured(1).trimmed() + "\n" + sections.at(sectionIndex).trimmed());
            stackedWidget->addWidget(textBrowser);
            pages.append(textBrowser);
            sectionIndex++;
        }
    }

    if (!pages.isEmpty()) {
        stackedWidget->setCurrentIndex(currentPageIndex);
    } else {
        // Fallback if no content or pages are created
        QLabel *noContentLabel = new QLabel("No content found in welcome.md", this);
        noContentLabel->setAlignment(Qt::AlignCenter);
        stackedWidget->addWidget(noContentLabel);
        pages.append(noContentLabel);
    }

    progressBar->setRange(0, pages.size() - 1);
    updateProgressBar();
    updateNavigationButtons();
}

void WelcomeWindow::on_nextButton_clicked()
{
    if (currentPageIndex < pages.size() - 1) {
        currentPageIndex++;
        stackedWidget->setCurrentIndex(currentPageIndex);
    } else {
        // This is the "Finish" action
        qDebug("Finish button clicked!");
        // You might emit a signal here to close the window or proceed to the next step
        close();
    }
    updateNavigationButtons();
    updateProgressBar();
}

void WelcomeWindow::on_previousButton_clicked()
{
    if (currentPageIndex > 0) {
        currentPageIndex--;
        stackedWidget->setCurrentIndex(currentPageIndex);
    } else {
        // This is the "Exit" action
        qDebug("Exit button clicked!");
        // You might emit a signal here to close the window
        close();
    }
    updateNavigationButtons();
    updateProgressBar();
}

void WelcomeWindow::updateNavigationButtons()
{
    if (pages.isEmpty()) {
        previousButton->setEnabled(false);
        nextButton->setEnabled(false);
        previousButton->setText("Exit");
        nextButton->setText("Finish");
        return;
    }

    // Previous button logic
    if (currentPageIndex == 0) {
        previousButton->setText("Exit");
    } else {
        previousButton->setText("Previous");
    }
    previousButton->setEnabled(true); // Always enabled for now, as Exit is an option

    // Next button logic
    if (currentPageIndex == pages.size() - 1) {
        nextButton->setText("Finish");
    } else {
        nextButton->setText("Next");
    }
    nextButton->setEnabled(true); // Always enabled for now, as Finish is an option
}

void WelcomeWindow::updateProgressBar()
{
    if (pages.isEmpty()) {
        progressBar->setValue(0);
        pageIndicatorLabel->setText("0/0");
        return;
    }
    progressBar->setValue(currentPageIndex);
    pageIndicatorLabel->setText(QString("%1/%2").arg(currentPageIndex + 1).arg(pages.size()));
}