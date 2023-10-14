#include "page1.h"
#include <QVBoxLayout>
#include <QLabel>
#include <QPushButton>
#include <QWidget>
#include <QLineEdit>
#include <QDateEdit>
#include <QDate>
#include <QPushButton>
#include <thread>
#include <iostream>
#include <string>

extern "C" {
void create_new_entry(const char* input,
                      const char* input2,
                      const char* input3);
void play_ui_sound(const char* sound_file, int disabled);

}

void PlayUISound(const char* soundFile, int muteState) {


    std::thread soundThread(play_ui_sound, soundFile, muteState);

    soundThread.detach();

    qDebug() << "played sound successfully!";

}

Page1::Page1(QWidget *parrent) : QWidget(parrent) {


    QVBoxLayout *layout = new QVBoxLayout(this);
    QLineEdit *inputFieldSummary = new QLineEdit(this);
    QLineEdit *inputFieldDescription = new QLineEdit(this);
    QDateEdit *inputDate = new QDateEdit(this);
    inputDate->setDisplayFormat("dd.MM.yyyy");
    inputDate->setDate(QDate::currentDate());


    QLabel *summaryLabel = new QLabel("Summary", this);
    summaryLabel->setAlignment(Qt::AlignCenter);


    QLabel *descriptionLabel = new QLabel("Description", this);
    descriptionLabel->setAlignment(Qt::AlignCenter);

    QLabel *dateLabel = new QLabel("End date", this);
    dateLabel->setAlignment(Qt::AlignCenter);

    QPushButton *button = new QPushButton("Submit");

    QObject::connect(button, &QPushButton::clicked, [inputFieldSummary, inputFieldDescription, inputDate] () {


        PlayUISound("ui-click.mp3", 1);

        qDebug() << "hell";
        QString userInputSumamry = inputFieldSummary->text();
        QString userInputDescription = inputFieldDescription->text();
        QString userInputDate = inputDate->text();
        qDebug() << "User input:" << userInputSumamry;
        qDebug() << "User input:" << userInputDescription;
        qDebug() << "User input:" << userInputDate;
        create_new_entry(userInputSumamry.toUtf8().constData(), userInputDescription.toUtf8().constData(), userInputDate.toUtf8().constData());

    });


    layout->addWidget(summaryLabel);
    layout->addWidget(inputFieldSummary);
    layout->addWidget(descriptionLabel);
    layout->addWidget(inputFieldDescription);
    layout->addWidget(dateLabel);
    layout->addWidget(inputDate);
    layout->addWidget(button);
    setLayout(layout);

}
