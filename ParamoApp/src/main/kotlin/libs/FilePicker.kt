package libs

import ch.bailu.gtk.gtk.FileChooserAction
import java.io.File
import javax.swing.JFileChooser
import ch.bailu.gtk.gtk.FileChooserDialogExtended

internal fun JFilePicker(): File?{
    var file: File? = null
    val fileChooser = JFileChooser()
    fileChooser.fileSelectionMode = JFileChooser.FILES_ONLY
    val result = fileChooser.showOpenDialog(null)

    if (result == JFileChooser.APPROVE_OPTION) {
       file = fileChooser.selectedFile
    }

    return file
}

internal fun GtkFilePicker(): File {
    val fileChooser = FileChooserDialogExtended("Open File", null, FileChooserAction.OPEN)
    fileChooser.show()

    val path = fileChooser.asFileChooser().file.path.toString()
    return File(path)
}

fun filePicker(): File? {
    val osName = System.getProperty("os.name")

    if (osName.contains("Windows", ignoreCase = true) || osName.contains("Mac", ignoreCase = true)) {
        return JFilePicker()
    } else if (osName.contains("Linux", ignoreCase = true)) {
        return GtkFilePicker()
    }

    return null
}