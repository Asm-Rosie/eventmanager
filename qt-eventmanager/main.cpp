#include <QApplication>
#include "mainwindow.h"

extern "C" {
void does_file_exist();
}

int main(int argc, char *argv[]) {

    does_file_exist();

    QApplication a(argc, argv);

    MainWindow w;

    w.show();

    return a.exec();
}
