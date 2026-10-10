import WidgetKit
import SwiftUI

/// Home-screen widget. It reads the snapshot `duet-widget` writes and does not
/// own a second copy of the stats.
struct StatEntry: TimelineEntry {
    let date: Date
    let lines: [String]
}

struct StatProvider: TimelineProvider {
    func placeholder(in context: Context) -> StatEntry {
        StatEntry(date: Date(), lines: [])
    }

    func getSnapshot(in context: Context, completion: @escaping (StatEntry) -> Void) {
        completion(load())
    }

    func getTimeline(in context: Context, completion: @escaping (Timeline<StatEntry>) -> Void) {
        completion(Timeline(entries: [load()], policy: .atEnd))
    }

    private func load() -> StatEntry {
        let url = FileManager.default.containerURL(forSecurityApplicationGroupIdentifier: "group.coda.duet")?
            .appendingPathComponent("widget-snapshot.json")
        let lines = url.flatMap { try? Data(contentsOf: $0) }
            .flatMap { try? JSONDecoder().decode(Snapshot.self, from: $0) }
            .map(\.lines) ?? []
        return StatEntry(date: Date(), lines: lines)
    }
}

struct Snapshot: Decodable {
    let lines: [String]
}

struct StatWidgetView: View {
    let entry: StatEntry

    var body: some View {
        VStack(alignment: .leading) {
            if entry.lines.isEmpty {
                Text("Nothing to show")
            } else {
                ForEach(entry.lines, id: \.self) { line in
                    Text(line)
                }
            }
        }
    }
}

struct DuetStatWidget: Widget {
    var body: some WidgetConfiguration {
        StaticConfiguration(kind: "DuetStat", provider: StatProvider()) { entry in
            StatWidgetView(entry: entry)
        }
        .configurationDisplayName("Duet")
        .description("A snapshot from the Duet app.")
    }
}
