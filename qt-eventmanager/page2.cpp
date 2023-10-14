#include "page2.h"
#include <QVBoxLayout>
#include <QListWidget>
#include <QLabel>


typedef struct ExposedStrings {
    const char* summaries;
    const char* descriptions;
    const char* start_dates;
    const char* end_dates;
    const char* uuid;
} ExposedStrings;

extern "C" {
ExposedStrings expose_strings(void);
}


class Task {
public:
    Task(const QString& summary, const QString& description, const QString& startDate, const QString& endDate)
        : summary_(summary), description_(description), startDate_(startDate), endDate_(endDate) {}

    QString getSummary() const { return summary_; }
    QString getDescription() const { return description_; }
    QString getStartDate() const { return startDate_; }
    QString getEndDate() const { return endDate_; }

private:
    QString summary_;
    QString description_;
    QString startDate_;
    QString endDate_;


};


Page2::Page2(QWidget *parrent) : QWidget(parrent) {

    QVBoxLayout *layout = new QVBoxLayout(this);
    layout->setAlignment(Qt::AlignCenter);

    QListWidget *listWidget = new QListWidget(this);
    QLabel *label = new QLabel("List");

    ExposedStrings exposed = expose_strings();

    if (exposed.summaries) {
        QString summariesString = QString::fromUtf8(exposed.summaries);
        QStringList summariesArray = summariesString.split('\n');
        for (const QString& summary : summariesArray) {
            qDebug() << "Summary: " << summary;
        }
    }

    if (exposed.descriptions) {
        QString descriptionsString = QString::fromUtf8(exposed.descriptions);
        QStringList descriptionsArray = descriptionsString.split('\n');
        for (const QString& description : descriptionsArray) {
            qDebug() << "Description: " << description;
        }
    }

    if (exposed.end_dates) {
        QString startDatesString = QString::fromUtf8(exposed.start_dates);
        QStringList startDatesArray = startDatesString.split('\n');
        for (const QString& startDate : startDatesArray) {
            qDebug() << "Start Date: " << startDate;
        }
    }

    if (exposed.end_dates) {
        QString endDatesString = QString::fromUtf8(exposed.end_dates);
        QStringList endDatesArray = endDatesString.split('\n');
        for (const QString& endDate : endDatesArray) {
            qDebug() << "End Date: " << endDate;
        }
    }

    QStringList summaries, descriptions, startDates, endDates;

    if (exposed.summaries) {
        QString summariesString = QString::fromUtf8(exposed.summaries);
        summaries = summariesString.split('\n');
    }

    if (exposed.descriptions) {
        QString descriptionsString = QString::fromUtf8(exposed.descriptions);
        descriptions = descriptionsString.split("\n");
    }

    if (exposed.start_dates) {
        QString startDatesString = QString::fromUtf8(exposed.start_dates);
        startDates = startDatesString.split("\n");
    }

    if (exposed.end_dates) {
        QString endDatesString = QString::fromUtf8(exposed.end_dates);
        endDates = endDatesString.split("\n");
    }

    if (summaries.size() == descriptions.size() && descriptions.size() == startDates.size() && startDates.size() == endDates.size()) {
        for (int i = 0; i < summaries.size(); ++i) {
            QString summary = summaries.at(i);
            QString description = descriptions.at(i);
            QString startDate = startDates.at(i);
            QString endDate = endDates.at(i);

            QString taskText = "Summary: " + summary + "\n" +
                               "Description: " + description + "\n" +
                               "Starting Date: " + startDate + "\n" +
                               "Ending Date: " + endDate;

            listWidget->addItem(taskText);
        }
    } else {
        qDebug("arrays have a different size");
    }





    layout->addWidget(label);
    layout->addWidget(listWidget);
    setLayout(layout);



}
