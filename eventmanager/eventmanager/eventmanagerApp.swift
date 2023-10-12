//
//  eventmanagerApp.swift
//  eventmanager
//
//  Created by joel kerbusch on 05.10.23.
//

import SwiftUI

@main
struct eventmanagerApp: App {
    init() {
        does_file_exist()
    }
    var body: some Scene {
        WindowGroup {
            NavigationManagerView()
        }
    }
}
