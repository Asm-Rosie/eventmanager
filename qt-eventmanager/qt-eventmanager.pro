QT       += core gui

greaterThan(QT_MAJOR_VERSION, 4): QT += widgets

CONFIG += c++17

# You can make your code fail to compile if it uses deprecated APIs.
# In order to do so, uncomment the following line.
#DEFINES += QT_DISABLE_DEPRECATED_BEFORE=0x060000    # disables all the APIs deprecated before Qt 6.0.0

SOURCES += \
    main.cpp \
    mainwindow.cpp \
    page1.cpp \
    page2.cpp

HEADERS += \
    mainwindow.h \
    page1.h \
    page2.h



FORMS += \
    mainwindow.ui

# Default rules for deployment.
qnx: target.path = /tmp/$${TARGET}/bin
else: unix:!android: target.path = /opt/$${TARGET}/bin
!isEmpty(target.path): INSTALLS += target

win32:CONFIG(release, debug|release): LIBS += -L$$PWD/./ -leventmanager_core
else:unix: LIBS += -L$$PWD/./ -leventmanager_core

INCLUDEPATH += $$PWD/.
DEPENDPATH += $$PWD/.

win32-g++:CONFIG(release, debug|release): PRE_TARGETDEPS += $$PWD/./libeventmanager_core.a
else:win32:!win32-g++:CONFIG(release, debug|release): PRE_TARGETDEPS += $$PWD/./eventmanager_core.lib
else:unix: PRE_TARGETDEPS += $$PWD/./libeventmanager_core.a

unix|win32: LIBS += -L$$PWD/./ -leventmanager_core.dll

INCLUDEPATH += $$PWD/.
DEPENDPATH += $$PWD/.

