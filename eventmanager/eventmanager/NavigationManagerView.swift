//
//  NavigationManagerView.swift
//  app
//
//  Created by joel kerbusch on 28.09.23.
//

import SwiftUI

struct mute {
    static var state = 1
}
enum SideBarItem: String, Identifiable, CaseIterable {
    var id: String { rawValue }
    
    case my_tasks
    case create_task
}

struct NavigationManagerView: View {
    @State var sideBarVisibility: NavigationSplitViewVisibility = .doubleColumn
    @State var selectedSideBarItem: SideBarItem = .my_tasks
    @State private var muted = false
    var body: some View {
        NavigationSplitView(columnVisibility: $sideBarVisibility) {
            List(SideBarItem.allCases, selection: $selectedSideBarItem) {
                item in NavigationLink(
                    item.rawValue.localizedCapitalized,
                    value: item
                
                
                )
            }
            
            Toggle(isOn: $muted) {
                Text("Mute/Unmute")
            }
            .padding()
            
            Button(action: {
                mute.state = muted ? 0 : 1
                DispatchQueue.global(qos: .background).async {
                    play_ui_sound("ui-click.mp3", Int32(mute.state))
                }
            }) {
                Text("play sound")
            }
        } detail: {
            switch selectedSideBarItem {
            case .my_tasks:
                SecondView()
                    .onAppear() {
                        DispatchQueue.global(qos: .background).async {
                            play_ui_sound("ui-click.mp3", Int32(mute.state))
                        }
                    }
            case .create_task:
                ContentView()
                    .onAppear() {
                        DispatchQueue.global(qos: .background).async {
                            play_ui_sound("ui-click.mp3", Int32(mute.state))
                        }
                    }
            }
        }
        
    }
}

#Preview {
    NavigationManagerView()
}
