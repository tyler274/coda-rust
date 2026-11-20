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
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TestWatcher
import org.junit.runner.Description
import org.junit.runner.RunWith

/**
 * Proves the installed Duet mobile shell through UI Automator.
 *
 * Duet opens directly to its account sign-in gate with no Physician/Duet
 * mode chooser, keeping the Duet tabs gated and never mounting the physician
 * shell.
 */
@RunWith(AndroidJUnit4::class)
class ModeShellTest {
    private lateinit var device: UiDevice

    /**
     * Saves the failing screen before the runner closes the app. The shell
     * user writes it, so `cargo run -p android -- test` can pull it.
     */
    @get:Rule
    val failureScreen = object : TestWatcher() {
        override fun failed(e: Throwable, description: Description) {
            if (::device.isInitialized) {
                device.executeShellCommand("screencap -p $FAILURE_SCREEN")
            }
        }
    }

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
        val deadline = System.currentTimeMillis() + WAIT
        var opened = false
        while (System.currentTimeMillis() < deadline) {
            device.findObject(By.res("android:id/aerr_wait"))?.click()
            if (device.hasObject(By.textStartsWith("Sign in to your Duet account"))) {
                opened = true
                break
            }
            device.wait(
                Until.hasObject(By.textStartsWith("Sign in to your Duet account")),
                2_000L,
            )
        }
        assertTrue(
            "Duet shell did not open directly to its sign-in gate:\n${hierarchy()}",
            opened,
        )
        control("Continue")
        assertNull(
            "Duet shell exposed a mode chooser:\n${hierarchy()}",
            device.findObject(By.text("Choose a mode")),
        )
        assertNull(
            "Duet shell showed the Duet tabs without an account:\n${hierarchy()}",
            device.findObject(By.text("Sections")),
        )
        assertNull(
            "Duet shell showed the physician shell:\n${hierarchy()}",
            device.findObject(By.text("Find patient")),
        )
        assertNull(
            "Duet shell exposed a Switch mode button:\n${hierarchy()}",
            device.findObject(By.text("Switch mode")),
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

        /** Read by `crates/android` after a failed run. */
        const val FAILURE_SCREEN = "/data/local/tmp/duet-failure.png"
    }
}
