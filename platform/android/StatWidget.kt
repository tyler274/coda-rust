package coda.duet

import android.appwidget.AppWidgetManager
import android.appwidget.AppWidgetProvider
import android.content.Context
import android.widget.RemoteViews
import org.json.JSONObject
import java.io.File

/** Reads the same snapshot JSON the Rust app writes. Health Connect is not implemented here. */
class StatWidget : AppWidgetProvider() {
    override fun onUpdate(context: Context, manager: AppWidgetManager, ids: IntArray) {
        val lines = readLines(context)
        for (id in ids) {
            val views = RemoteViews(context.packageName, android.R.layout.simple_list_item_1)
            views.setTextViewText(
                android.R.id.text1,
                if (lines.isEmpty()) "Nothing to show" else lines.joinToString("\n"),
            )
            manager.updateAppWidget(id, views)
        }
    }

    private fun readLines(context: Context): List<String> {
        val file = File(context.filesDir, "widget-snapshot.json")
        if (!file.exists()) return emptyList()
        val json = JSONObject(file.readText())
        val array = json.optJSONArray("lines") ?: return emptyList()
        return List(array.length()) { array.getString(it) }
    }
}
