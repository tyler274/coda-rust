package coda.duet

import android.content.Intent
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.uiautomator.By
import androidx.test.uiautomator.UiDevice
import androidx.test.uiautomator.UiObject2
import androidx.test.uiautomator.Until
import java.io.ByteArrayOutputStream
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

/**
 * Proves the installed mode shell through UI Automator.
 *
 * Client mode must show Duet, Today, You, and Shared, with Duet selected.
 * Physician mode must show Today and Find patient, and must not keep the
 * client tabs. A missing control fails. A loading or error body on a tab does
 * not.
 */
@RunWith(AndroidJUnit4::class)
class ModeShellTest {
    private lateinit var device: UiDevice

    @Before
    fun launchChooser() {
        val instrumentation = InstrumentationRegistry.getInstrumentation()
        device = UiDevice.getInstance(instrumentation)
        // Package data is cleared by the test orchestrator before this process
        // starts. Clearing it here would stop the instrumentation.
        val intent = instrumentation.targetContext.packageManager
            .getLaunchIntentForPackage("com.example.Mobile")
            ?: error("the mode shell is not installed")
        intent.addFlags(Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK)
        instrumentation.targetContext.startActivity(intent)
    }

    @Test
    fun clientThenPhysician() {
        assertTrue(
            "the chooser did not show Coda:\n${hierarchy()}",
            device.wait(Until.hasObject(By.text("Coda")), WAIT) == true,
        )
        click("Client")

        val sections = device.wait(Until.findObject(By.text("Sections")), WAIT)
            ?: error("client mode did not show the tab bar:\n${hierarchy()}")
        val tabs = sections.findObjects(By.clickable(true)).map { it.text }
        assertEquals(
            "client mode did not show Duet, Today, You, Shared in that order:\n${hierarchy()}",
            listOf("Duet", "Today", "You", "Shared"),
            tabs,
        )
        // WebView does not publish aria-current as selected. The open tab is
        // the page heading above the tab bar.
        val heading = device.findObjects(
            By.pkg("com.example.Mobile").clazz("android.widget.TextView"),
        )
            .filter { it.visibleBounds.top < sections.visibleBounds.top }
            .minByOrNull { it.visibleBounds.top }
        assertEquals(
            "Duet was not the open tab:\n${hierarchy()}",
            "Duet",
            heading?.text,
        )
        assertNull(
            "client mode showed the physician shell:\n${hierarchy()}",
            device.findObject(By.text("Find patient")),
        )

        click("Switch mode")
        click("Physician")
        assertTrue(
            "physician mode did not show Today:\n${hierarchy()}",
            device.wait(Until.hasObject(By.text("Today")), WAIT) == true,
        )
        control("Find patient")
        assertNull(
            "physician mode kept the client screen:\n${hierarchy()}",
            device.findObject(By.text("Shared")),
        )
        assertNull(
            "physician mode kept the client tabs:\n${hierarchy()}",
            device.findObject(By.text("Sections")),
        )
    }

    private fun click(label: String) {
        control(label).click()
        device.waitForIdle()
    }

    private fun control(label: String): UiObject2 {
        return device.wait(Until.findObject(By.text(label).clickable(true)), WAIT)
            ?: error("$label was not a control:\n${hierarchy()}")
    }

    private fun hierarchy(): String {
        val stream = ByteArrayOutputStream()
        device.dumpWindowHierarchy(stream)
        return stream.toString(Charsets.UTF_8.name())
    }

    companion object {
        private const val WAIT = 30_000L
    }
}
