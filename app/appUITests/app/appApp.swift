//
//  appApp.swift
//  app
//
//  Created by joel kerbusch on 21.09.23.
//

import SwiftUI

@main
struct appApp: App {
    init() {
        create_file()
    }
    
    var body: some Scene {
        WindowGroup {
            NavigationManagerView()
        }
    }
}
