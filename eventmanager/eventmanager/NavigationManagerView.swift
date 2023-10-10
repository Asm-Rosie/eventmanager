//
//  NavigationManagerView.swift
//  app
//
//  Created by joel kerbusch on 28.09.23.
//

import SwiftUI

enum SideBarItem: String, Identifiable, CaseIterable {
    var id: String { rawValue }
    
    case my_tasks
    case create_task
}

struct NavigationManagerView: View {
    @State var sideBarVisibility: NavigationSplitViewVisibility = .doubleColumn
    @State var selectedSideBarItem: SideBarItem = .my_tasks
    var body: some View {
        NavigationSplitView(columnVisibility: $sideBarVisibility) {
            List(SideBarItem.allCases, selection: $selectedSideBarItem) {
                item in NavigationLink(
                    item.rawValue.localizedCapitalized,
                    value: item
                
                
                )
            }
        } detail: {
            switch selectedSideBarItem {
            case .my_tasks:
                SecondView()
                    .onAppear() {
                        DispatchQueue.global(qos: .background).async {
                            play_ui_sound()
                        }
                    }
            case .create_task:
                ContentView()
                    .onAppear() {
                        DispatchQueue.global(qos: .background).async {
                            play_ui_sound()
                        }
                    }
            }
        }
    }
}

#Preview {
    NavigationManagerView()
}
