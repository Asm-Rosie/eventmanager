#include "mainwindow.h"
#include "ui_mainwindow.h"
#include "page1.h"
#include "page2.h"
#include <QTabWidget>

MainWindow::MainWindow(QWidget *parent) : QMainWindow(parent) , ui(new Ui::MainWindow) {
    QTabWidget *tabWidget = new QTabWidget(this);

    Page1 *tab1 = new Page1(this);
    Page2 *tab2 = new Page2(this);

    tabWidget->addTab(tab1, "Create Task");
    tabWidget->addTab(tab2, "My Tasks");

    setCentralWidget(tabWidget);
    setWindowTitle("Eventmanager-QT");
    setGeometry(100, 100, 640, 480);
}

MainWindow::~MainWindow()
{
    delete ui;
}

