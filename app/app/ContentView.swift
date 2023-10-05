import SwiftUI
import Foundation

func convertDateToString(date: Date) -> String {
    let formatter = DateFormatter()
    formatter.dateFormat = "yyyy-MM-dd" // Customize the date format as needed
    return formatter.string(from: date)
}


struct SwitcherView: View {
    var body: some View {
        NavigationView {
            VStack {
                NavigationLink(destination: ContentView()) {
                    Text("switch to ContentView()")
                }
            }
            .frame(maxWidth: .infinity, maxHeight: .infinity)
        }
        VStack {
            Text("Hi")
        }
    }
}

struct ContentView: View {
    
    @State private var summaryInput: String = ""
    @State private var descriptionInput: String = ""
    @State private var selectedDate = Date()
    @State private var versionInfo = ""
    

    let customFilePath: String = "textfile.txt"
    
    
    var body: some View {
        
        VStack {
            TextField("Enter your Title", text: $summaryInput, onCommit: performAction)
                .textFieldStyle(RoundedBorderTextFieldStyle())
                .padding()
            TextField("Enter your description", text: $descriptionInput, onCommit: performAction)
                .textFieldStyle(RoundedBorderTextFieldStyle())
                .padding()
            
            
            DatePicker("Select the end date", selection: $selectedDate, displayedComponents: .date)
            
            
            
            Button("Submit") {
                performAction()
                let dateString = convertDateToString(date: selectedDate)
                let inputCString = summaryInput.cString(using: .utf8)
                let inputCString2 = descriptionInput.cString(using: .utf8)
                let inputCString3 = dateString
                print_string_to_console(inputCString, inputCString2, inputCString3)
                currentdate()
            }
            
            Button("request") {
                let exposed = expose_strings()
                
                
                
                if let summaries = exposed.summaries {
                    let summariesString = String(cString: summaries)
                    let summariesArray = summariesString.components(separatedBy: "\n")
                    for summary in summariesArray {
                        print("Summary: \(summary)")
                    }
                }
                
                if let descriptions = exposed.descriptions {
                    let descriptionsString = String(cString: descriptions)
                    let descriptionsArray = descriptionsString.components(separatedBy: "\n")
                    for description in descriptionsArray {
                        print("Description: \(description)")
                    }
                }
                
                if let start_dates = exposed.start_dates {
                    let start_datesString = String(cString: start_dates)
                    let start_datesArray = start_datesString.components(separatedBy: "\n")
                    for start_date in start_datesArray {
                        print("starting date: \(start_date)")
                    }
                }
                
                if let end_dates = exposed.start_dates {
                    let end_datesString = String(cString: end_dates)
                    let end_datesArray = end_datesString.components(separatedBy: "\n")
                    for end_date in end_datesArray {
                        print("ending at: \(end_date)")
                    }
                }
            }
            
            
            Image("Image")
                .renderingMode(.template)
                .resizable()
                .frame(width: 32.0, height: 32.0)
                .frame(width: nil)
                .imageScale(.small)
                .foregroundStyle(.tint)
            
            Text(versionInfo)
                .onAppear() {
                    if let appVersion = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String,
                       let buildNumber = Bundle.main.infoDictionary?["CFBundleVersion"] as? String {
                        versionInfo = "Developer Build\nVer. \(appVersion)\nBuild: \(buildNumber)"
                    }
                }
            
            
        }
        
        // Function to remove a specific line from the text file
        
    }
    
    private func performAction() {
        print("User input: \(summaryInput)")
        print("description: \(descriptionInput)")
        print("date: \(selectedDate)")
    }
    
    
    
    struct ContentView_Previews: PreviewProvider {
        static var previews: some View {
            ContentView()
        }
    }
}


class Task: Identifiable {
    var id: String
    var summary: String
    var description: String
    var startingPoint: String
    var endingDate: String
    
    init(id: String, summary: String, description: String, startingPoint: String, endingDate: String) {
        self.id = id
        self.summary = summary
        self.description = description
        self.startingPoint = startingPoint
        self.endingDate = endingDate
    }
}



struct TaskRow: View {
    @Binding var isDeleted: Bool
    @Binding var isEditing: Bool
    
    @State private var editedSummary = ""
    @State private var editedDescription = ""
    @State private var editedEndingDate = ""
    
    var task: Task
    var deleteAction: (String) -> Void
    var onDelete: () -> Void
    var requestDataAction: () -> Void
    var toggleEditMode: () -> Void
    
    
    
    var body: some View {
        VStack(alignment: .leading) {
            if isEditing {
                
                
                
                TextField("Summary", text: $editedSummary)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .padding()
                TextField("Description", text: $editedDescription)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .padding()
                TextField("Ending Date", text: $editedEndingDate)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .padding()
                
            } else {
                Text("Summary: \(task.summary)")
                Text("Description: \(task.description)")
                Text("Ending date: \(task.endingDate)")
            }
            
            
            Text("Starting point: \(task.startingPoint)")
            Spacer()
            Button("Delete Task") {
                print("Summary: \(task.summary)")
                print("Description: \(task.description)")
                print("Starting point: \(task.startingPoint)")
                print("Ending date: \(task.endingDate)")
                print("ID: \(task.id)")
                onDelete()
                isDeleted = true
                DispatchQueue.main.asyncAfter(deadline: .now() + 2.0) {
                    requestDataAction()
                }
            }
            Button(action: {
                
                if self.isEditing {
                    self.task.summary = self.editedSummary
                    self.task.description = self.editedDescription
                }
                self.isEditing.toggle()
            }) {
                Text(self.isEditing ? "Save" : "Edit")
            }
            .onAppear() {
                editedSummary = task.summary
                editedDescription = task.description
                editedEndingDate = task.endingDate
                
                
                
            }
            
        }
    }
}

struct SecondView: View {
    @State private var tasks: [Task] = []
    @State private var deletedTaskIndex: Int?
    @State private var isEditing = false
    @State private var editedSummary = ""
    @State private var editedDescription = ""


    func toggleEditMode(for task: Task) {
            if isEditing {
                // Save the edited values back to the task
                if let index = tasks.firstIndex(where: { $0.id == task.id }) {
                    tasks[index].summary = editedSummary
                    tasks[index].description = editedDescription
                    let dateFormatter = DateFormatter()
                    dateFormatter.dateFormat = "yyyy-MM-dd"
                   
                    
                    // Update other edited properties similarly
                }
            } else {
                // Set editable properties to current task's values
                editedSummary = task.summary
                editedDescription = task.description
                
                // Set other editable properties similarly
            }
            
            isEditing.toggle()
        }
    
    func deleteTask(withId id: String) {
        delete_block_call(id)
    }
    
    
    func requestData() {
        let exposed = expose_strings()
        
        if let summaries = exposed.summaries, let descriptions = exposed.descriptions, let start_dates = exposed.start_dates, let end_dates = exposed.end_dates, let uuids = exposed.uuid {
            let summariesString = String(cString: summaries)
            let descriptionsString = String(cString: descriptions)
            let startDatesString = String(cString: start_dates)
            let endDatesString = String(cString: end_dates)
            let uuidString = String(cString: uuids)
            
            
            let summaryArray = summariesString.components(separatedBy: "\n")
            let descriptionArray = descriptionsString.components(separatedBy: "\n")
            let startDatesArray = startDatesString.components(separatedBy: "\n")
            let endDatesArray = endDatesString.components(separatedBy: "\n")
            let uuidArray = uuidString.components(separatedBy: "\n");
            
            guard summaryArray.count == descriptionArray.count && descriptionArray.count == startDatesArray.count && startDatesArray.count == endDatesArray.count && endDatesArray.count == uuidArray.count else {
                print("Data arrays have different counts")
                return
            }
            
            tasks.removeAll()
            for i in 0..<summaryArray.count {
                let task = Task(id: uuidArray[i],
                                summary: summaryArray[i],
                                description: descriptionArray[i],
                                startingPoint: startDatesArray[i],
                                endingDate: endDatesArray[i]
                
                )
                                
                                
                tasks.append(task)
            }
            
        } else {
            print("something went wrong");
        }
        
    }
    
    var body: some View {
        VStack {
            List {
                
                ForEach(Array(tasks.enumerated()), id: \.element.id) { index, task in
                    TaskRow(isDeleted: Binding(
                        get: { deletedTaskIndex == index },
                        set: { _ in }
                    ), isEditing: $isEditing, task: task, deleteAction: deleteTask, onDelete: {
                        tasks.removeAll { $0.id == task.id }
                        deletedTaskIndex = index
                        delete_block_call(task.id)
                        // Call the function to remove the line from the text file (deleteBlockCall)
                        deletedTaskIndex = index
                    }, requestDataAction: requestData, toggleEditMode: {
                        toggleEditMode(for: task)
                        
                    })
                    .contentShape(Rectangle()) // Ensure swipe gesture recognition
                }
                
            }
            
            Text("This is the Second View")
            
                
        }
            
            
        
        .onAppear() {
            requestData()
        }
        
        
    }
}


