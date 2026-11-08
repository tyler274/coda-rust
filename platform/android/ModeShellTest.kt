package coda.duet

import android.content.Intent
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.uiautomator.By
import androidx.test.uiautomator.UiDevice
import androidx.test.uiautomator.UiObject2
import androidx.test.uiautomator.Until
import java.io.ByteArrayOutputStream
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

/**
 * Proves the installed mode shell through UI Automator.
 *
 * Client mode must show the Duet account sign-in gate, not the physician
 * shell: without an account the Duet tabs stay gated, as in the browser mode
 * test. Physician mode must show Today and, in its navigation drawer, Find
 * patient, and must not keep
 * the client screen. A missing control fails. A loading or error body on a
 * tab does not.
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

        assertTrue(
            "client mode did not show the Duet sign-in gate:\n${hierarchy()}",
            device.wait(
                Until.hasObject(By.textStartsWith("Sign in to your Duet account")),
                WAIT,
            ) == true,
        )
        control("Sign in with Apple")
        assertNull(
            "client mode showed the Duet tabs without an account:\n${hierarchy()}",
            device.findObject(By.text("Sections")),
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
        // On a phone the sidebar is a drawer; Find patient is inside it.
        click("Open navigation")
        control("Find patient")
        assertNull(
            "physician mode kept the client screen:\n${hierarchy()}",
            device.findObject(By.text("Sign in with Apple")),
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
